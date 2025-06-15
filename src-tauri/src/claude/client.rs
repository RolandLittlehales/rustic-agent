use crate::claude::{
    types::*,
    tools::{ToolRegistry, AgentTool},
    ClaudeConfig, Conversation,
};
use anyhow::{Result, Context};
use reqwest::Client;
use std::time::{Duration, Instant};
use std::sync::Mutex;

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
        
        let http_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .user_agent("LLMDevAgent/0.1.0")
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

    pub async fn send_message(&self, conversation: &Conversation, message: String) -> Result<String> {
        let mut messages = self.conversation_to_claude_messages(conversation);
        messages.push(ClaudeMessage::user_text(message));

        let tools = self.tool_registry.get_all_tools();

        let request = ClaudeRequest {
            model: self.config.model.clone(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            messages,
            tools: Some(tools),
            system: Some("You are a helpful AI assistant specialized in software development. You have access to various tools to help with file operations, code analysis, and development tasks.".to_string()),
        };

        let response = self.make_api_call(request).await?;
        self.process_response(response).await
    }

    async fn make_api_call(&self, request: ClaudeRequest) -> Result<ClaudeResponse> {
        // Rate limiting: ensure at least 1 second between requests
        let sleep_duration = {
            let mut last_request = self.last_request.lock().unwrap();
            if let Some(last_time) = *last_request {
                let elapsed = last_time.elapsed();
                if elapsed < Duration::from_secs(1) {
                    Some(Duration::from_secs(1) - elapsed)
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
            let mut last_request = self.last_request.lock().unwrap();
            *last_request = Some(Instant::now());
        }

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Claude API request failed with status {}: {}",
                status,
                text
            ));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .context("Failed to parse Claude API response")?;

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

    pub async fn chat(&self, conversation: &mut Conversation, user_message: String) -> Result<String> {
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