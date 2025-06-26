use crate::claude::{
    tools::{AgentTool, ToolRegistry},
    types::*,
    ClaudeConfig, Conversation,
};
use crate::constants;
use crate::logging::ApiCallLog;
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Instant;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct ClaudeClient {
    config: ClaudeConfig,
    http_client: Client,
    tool_registry: ToolRegistry,
    last_request: Mutex<Option<Instant>>,
}

impl ClaudeClient {
    pub fn new(config: ClaudeConfig) -> Result<Self> {
        // Basic API key validation (just check it's not empty)
        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("API key cannot be empty"));
        }

        // Create HTTP client with connection pooling for improved performance
        let http_client = Client::builder()
            .timeout(constants::http_client_timeout())
            .user_agent(constants::USER_AGENT)
            .pool_idle_timeout(std::time::Duration::from_secs(constants::POOL_IDLE_TIMEOUT_SECS))
            .pool_max_idle_per_host(constants::POOL_MAX_IDLE_PER_HOST)
            .build()
            .context("Failed to create HTTP client")?;

        let mut tool_registry = ToolRegistry::new();

        // Register default tools
        tool_registry.register(crate::claude::tools::ReadFileTool);
        tool_registry.register(crate::claude::tools::WriteFileTool);
        tool_registry.register(crate::claude::tools::ListDirectoryTool);

        Ok(Self {
            config,
            http_client,
            tool_registry,
            last_request: Mutex::new(None),
        })
    }

    pub fn register_tool<T: AgentTool + 'static>(&mut self, tool: T) {
        self.tool_registry.register(tool);
    }

    pub async fn send_message(
        &self,
        conversation: &Conversation,
        message: String,
    ) -> Result<String> {
        let mut messages = self.conversation_to_claude_messages(conversation);
        messages.push(ClaudeMessage::user_text(message));

        let tools = self.tool_registry.get_all_tools();

        let request = ClaudeRequest {
            model: self.config.model.clone(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            messages,
            tools: Some(tools),
            system: Some(constants::CLAUDE_SYSTEM_PROMPT.to_string()),
        };

        let response = self.make_api_call(request).await?;
        self.process_response(response).await
    }

    async fn make_api_call(&self, request: ClaudeRequest) -> Result<ClaudeResponse> {
        let api_start_time = Instant::now();
        let request_id = Uuid::new_v4().to_string();

        // Create log entry with request snippet
        let request_content = request
            .messages
            .last()
            .and_then(|msg| msg.content.first())
            .and_then(|content| match content {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .unwrap_or("No content");

        let mut api_log = ApiCallLog::new(&request_id, &self.config.model, request_content);

        // Rate limiting: ensure at least 1 second between requests
        let sleep_duration = {
            let last_request = self.last_request.lock().await;
            if let Some(last_time) = *last_request {
                let elapsed = last_time.elapsed();
                if elapsed < constants::rate_limit_duration() {
                    Some(constants::rate_limit_duration() - elapsed)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(duration) = sleep_duration {
            tokio::time::sleep(duration).await;
        }

        // Update last request time
        {
            let mut last_request = self.last_request.lock().await;
            *last_request = Some(Instant::now());
        }

        let response = match self
            .http_client
            .post(constants::CLAUDE_API_MESSAGES_ENDPOINT)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", constants::CLAUDE_API_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                api_log = api_log.with_error(format!("Request failed: {}", e));
                api_log.duration = api_start_time.elapsed();
                api_log.log();
                return Err(e).context("Failed to send request to Claude API");
            }
        };

        let status = response.status();
        api_log = api_log.with_response(status.as_u16(), api_start_time.elapsed());

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            let error = format!("Claude API request failed with status {}: {}", status, text);
            api_log = api_log.with_error(&error);
            api_log.log();
            return Err(anyhow::anyhow!(error));
        }

        let claude_response: ClaudeResponse = match response.json::<ClaudeResponse>().await {
            Ok(resp) => {
                // Extract token usage if available
                if let Some(usage) = &resp.usage {
                    api_log = api_log.with_tokens(
                        usage.input_tokens.unwrap_or(0),
                        usage.output_tokens.unwrap_or(0),
                    );
                }
                api_log.log();
                resp
            }
            Err(e) => {
                api_log = api_log.with_error(format!("Failed to parse response: {}", e));
                api_log.log();
                return Err(e).context("Failed to parse Claude API response");
            }
        };

        Ok(claude_response)
    }

    async fn process_response(&self, response: ClaudeResponse) -> Result<String> {
        let mut result_parts = Vec::new();

        for content_block in &response.content {
            match content_block {
                ContentBlock::Text { text } => {
                    result_parts.push(text.clone());
                }
                ContentBlock::ToolUse { id: _, name, input } => {
                    match self.tool_registry.execute_tool(name, input.clone()) {
                        Ok(tool_result) => {
                            result_parts.push(format!("Tool '{}' result: {}", name, tool_result));
                        }
                        Err(e) => {
                            result_parts.push(format!("Tool '{}' error: {}", name, e));
                        }
                    }
                }
                ContentBlock::ToolResult { .. } => {
                    // Tool results in response shouldn't happen, but handle gracefully
                }
            }
        }

        Ok(result_parts.join("\n"))
    }

    fn conversation_to_claude_messages(&self, conversation: &Conversation) -> Vec<ClaudeMessage> {
        conversation
            .messages
            .iter()
            .map(|msg| match msg.role.as_str() {
                "user" => ClaudeMessage::user_text(msg.content.clone()),
                "assistant" => ClaudeMessage::assistant_text(msg.content.clone()),
                _ => ClaudeMessage::user_text(msg.content.clone()),
            })
            .collect()
    }

    pub async fn chat(
        &self,
        conversation: &mut Conversation,
        user_message: String,
    ) -> Result<String> {
        conversation.add_user_message(user_message.clone());

        let response = self.send_message(conversation, user_message).await?;

        conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    pub fn get_available_tools(&self) -> Vec<String> {
        self.tool_registry
            .get_all_tools()
            .into_iter()
            .map(|tool| tool.name)
            .collect()
    }
}
