use crate::claude::{
    error::{ClaudeError, ClaudeResult, ErrorContext, ErrorHandler},
    message_processor::MessageProcessor,
    tools::{AgentTool, ToolRegistry, ToolExecutionEngine, ToolExecutionContext, FollowUpAction, StatusLevel},
    types::*,
    whitelist::WhitelistConfig,
    ClaudeConfig, Conversation,
};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct ClaudeClient {
    config: ClaudeConfig,
    http_client: Client,
    tool_registry: ToolRegistry,
    tool_execution_engine: ToolExecutionEngine,
    message_processor: MessageProcessor,
    error_handler: ErrorHandler,
    last_request: Mutex<Option<Instant>>,
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl ClaudeClient {
    pub fn new(config: ClaudeConfig) -> ClaudeResult<Self> {
        // Validate configuration
        config.validate()?;

        let http_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .user_agent("LLMDevAgent/0.1.0")
            .build()
            .map_err(ClaudeError::HttpError)?;

        let mut tool_registry = ToolRegistry::new();

        // Register default tools
        tool_registry.register(crate::claude::tools::ReadFileTool::new());
        tool_registry.register(crate::claude::tools::WriteFileTool::new());
        tool_registry.register(crate::claude::tools::ListDirectoryTool::new());

        // Initialize enhanced tool execution engine
        let mut tool_execution_engine = ToolExecutionEngine::new();
        
        // Register tools with both registry and execution engine
        let read_tool = Arc::new(crate::claude::tools::ReadFileTool::new());
        let write_tool = Arc::new(crate::claude::tools::WriteFileTool::new());
        let list_tool = Arc::new(crate::claude::tools::ListDirectoryTool::new());
        
        tool_execution_engine.register_tool(read_tool.clone());
        tool_execution_engine.register_tool(write_tool.clone());
        tool_execution_engine.register_tool(list_tool.clone());

        Ok(Self {
            config,
            http_client,
            tool_registry,
            tool_execution_engine,
            message_processor: MessageProcessor::new(),
            error_handler: ErrorHandler::new(),
            last_request: Mutex::new(None),
            whitelist: None,
        })
    }

    #[allow(dead_code)]
    pub fn register_tool<T: AgentTool + 'static>(&mut self, tool: T) {
        self.tool_registry.register(tool);
    }

    /// Set the whitelist configuration for all tools
    #[allow(dead_code)]
    pub fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.tool_registry.set_whitelist(whitelist.clone());
        self.message_processor.set_whitelist(whitelist.clone());
        self.whitelist = Some(whitelist);
    }

    pub async fn send_message(
        &self,
        conversation: &Conversation,
        message: String,
    ) -> ClaudeResult<String> {
        // Process the user message using the message processor
        let user_message = self.message_processor.process_user_message(&message)?;

        let mut messages = self.conversation_to_claude_messages(conversation);
        messages.push(user_message);

        let tools = self.tool_registry.get_all_tools();

        let system_message = if self.config.supports_thinking() {
            "You are a helpful AI assistant specialized in software development. You have access to various tools to help with file operations, code analysis, and development tasks. Feel free to use thinking mode to reason through complex problems."
        } else {
            "You are a helpful AI assistant specialized in software development. You have access to various tools to help with file operations, code analysis, and development tasks."
        };

        let request = ClaudeRequest {
            model: self.config.model.clone(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            messages,
            tools: Some(tools),
            system: Some(system_message.to_string()),
        };

        // Use error handler for retry logic
        let response = self
            .error_handler
            .handle_with_retry(|| self.make_api_call(request.clone()))
            .await?;

        self.process_response(response).await
    }

    async fn make_api_call(&self, request: ClaudeRequest) -> ClaudeResult<ClaudeResponse> {
        // Rate limiting: ensure at least 1 second between requests
        let sleep_duration = {
            let last_request = self.last_request.lock().unwrap();
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
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ClaudeError::ApiError {
                status: status.as_u16(),
                message: text,
                error_type: None,
                param: None,
                context: Some(
                    ErrorContext::new("claude_api_call").add_metadata("model", &self.config.model),
                ),
            });
        }

        let claude_response: ClaudeResponse = response.json().await?;

        Ok(claude_response)
    }

    async fn process_response(&self, response: ClaudeResponse) -> ClaudeResult<String> {
        self.process_response_with_tools(response).await
    }

    /// Enhanced response processing with structured tool execution
    async fn process_response_with_tools(&self, response: ClaudeResponse) -> ClaudeResult<String> {
        let mut processed_content = Vec::new();
        let mut tool_results = Vec::new();

        for content_block in &response.content {
            match content_block {
                ContentBlock::Text { text } => {
                    processed_content.push(text.clone());
                }
                ContentBlock::ToolUse { id: _, name, input } => {
                    // Validate tool use first
                    if let Err(e) = self
                        .message_processor
                        .validate_tool_use(content_block)
                        .await
                    {
                        processed_content.push(format!("Tool '{}' validation error: {}", name, e));
                        continue;
                    }

                    // Create execution context
                    let whitelist = self.whitelist.clone().unwrap_or_else(|| {
                        Arc::new(RwLock::new(WhitelistConfig::default()))
                    });

                    let context = ToolExecutionContext::new(
                        name.clone(),
                        input.clone(),
                        whitelist,
                    ).with_timeout(Duration::from_secs(30))
                     .with_max_retries(3);

                    // Execute tool using enhanced execution engine
                    let tool_request = crate::claude::tools::ToolRequest::new(
                        name.clone(),
                        input.clone(),
                    );

                    match self.tool_execution_engine.execute_single_tool(tool_request, context).await {
                        Ok(execution_result) => {
                            // Process the structured result
                            let result_content = execution_result.into_content_block();
                            processed_content.push(format!("Tool '{}' result: {}", name, result_content));
                            
                            // Handle follow-up actions if any
                            if !execution_result.follow_up_actions.is_empty() {
                                for action in &execution_result.follow_up_actions {
                                    match action {
                                        FollowUpAction::ReportStatus { message, level } => {
                                            let level_str = match level {
                                                StatusLevel::Info => "INFO",
                                                StatusLevel::Warning => "WARNING",
                                                StatusLevel::Error => "ERROR",
                                            };
                                            processed_content.push(format!("[{}] {}", level_str, message));
                                        }
                                        FollowUpAction::RequestUserInput { prompt, suggested_actions } => {
                                            processed_content.push(format!("User input requested: {}", prompt));
                                            if !suggested_actions.is_empty() {
                                                processed_content.push(format!("Suggestions: {}", suggested_actions.join(", ")));
                                            }
                                        }
                                        _ => {
                                            // Other follow-up actions would be handled in a more comprehensive implementation
                                            processed_content.push("Additional follow-up actions available".to_string());
                                        }
                                    }
                                }
                            }

                            tool_results.push(execution_result);
                        }
                        Err(e) => {
                            let error_msg = format!("Tool '{}' execution failed: {}", name, e);
                            processed_content.push(error_msg);
                        }
                    }
                }
                ContentBlock::ToolResult { .. } => {
                    // Tool results in response shouldn't happen in this context, but handle gracefully
                }
                ContentBlock::Thinking { content } => {
                    // Thinking blocks can be processed or ignored based on configuration
                    if self.config.supports_thinking() {
                        processed_content.push(format!("Thinking: {}", content));
                    }
                }
            }
        }

        Ok(processed_content.join("\n"))
    }

    fn conversation_to_claude_messages(&self, conversation: &Conversation) -> Vec<ClaudeMessage> {
        conversation
            .messages
            .iter()
            .map(|msg| ClaudeMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
                thinking: msg.thinking.clone(),
            })
            .collect()
    }

    pub async fn chat(
        &self,
        conversation: &mut Conversation,
        user_message: String,
    ) -> ClaudeResult<String> {
        conversation.add_user_message(user_message.clone());

        let response = self.send_message(conversation, user_message).await?;

        conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    #[allow(dead_code)]
    pub fn get_available_tools(&self) -> Vec<String> {
        self.tool_registry
            .get_all_tools()
            .into_iter()
            .map(|tool| tool.name)
            .collect()
    }
}
