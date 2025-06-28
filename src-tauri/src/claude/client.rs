use crate::claude::{
    connection_pool::ConnectionPool,
    error::{ClaudeError, ClaudeResult, ErrorContext, ErrorHandler},
    message_processor::MessageProcessor,
    tools::{AgentTool, ToolExecutionContext, ToolExecutionEngine, ToolRegistry},
    types::*,
    whitelist::WhitelistConfig,
    ClaudeConfig, Conversation,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct ClaudeClient {
    config: ClaudeConfig,
    connection_pool: ConnectionPool,
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

        // Create connection pool with optimized settings
        let connection_pool = ConnectionPool::new();

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
            connection_pool,
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
        // Rate limiting: ensure minimum interval between requests
        let sleep_duration = {
            let last_request = self.last_request.lock().unwrap();
            if let Some(last_time) = *last_request {
                let elapsed = last_time.elapsed();
                let min_interval =
                    Duration::from_millis(crate::config::constants::RATE_LIMIT_INTERVAL_MS);
                if elapsed < min_interval {
                    Some(min_interval - elapsed)
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

        let http_client = self
            .connection_pool
            .get_client()
            .await
            .map_err(ClaudeError::HttpError)?;

        let response = http_client
            .post(crate::config::constants::claude_messages_url())
            .header("x-api-key", &self.config.api_key)
            .header(
                "anthropic-version",
                crate::config::constants::CLAUDE_API_VERSION,
            )
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
        // Check if the response contains tool uses that need to be executed
        let mut has_tool_uses = false;
        let mut tool_result_blocks = Vec::new();

        for content_block in &response.content {
            if let ContentBlock::ToolUse { id, name, input } = content_block {
                has_tool_uses = true;

                // Validate tool use first
                if let Err(e) = self
                    .message_processor
                    .validate_tool_use(content_block)
                    .await
                {
                    // Create error tool result
                    tool_result_blocks.push(ContentBlock::ToolResult {
                        tool_use_id: id.clone(),
                        content: format!("Tool validation error: {}", e),
                        is_error: Some(true),
                        metadata: None,
                    });
                    continue;
                }

                // Create execution context
                let whitelist = self
                    .whitelist
                    .clone()
                    .unwrap_or_else(|| Arc::new(RwLock::new(WhitelistConfig::default())));

                let context = ToolExecutionContext::new(name.clone(), input.clone(), whitelist)
                    .with_timeout(Duration::from_secs(
                        crate::claude::constants::error_handling::DEFAULT_HTTP_TIMEOUT_SECS,
                    ))
                    .with_max_retries(
                        crate::claude::constants::error_handling::DEFAULT_MAX_RETRIES,
                    );

                // Execute tool using enhanced execution engine
                let tool_request =
                    crate::claude::tools::ToolRequest::new(name.clone(), input.clone());

                match self
                    .tool_execution_engine
                    .execute_single_tool(tool_request, context)
                    .await
                {
                    Ok(execution_result) => {
                        // Create ToolResult content block with the tool execution result
                        let tool_result_content = execution_result.into_content_block();

                        tool_result_blocks.push(ContentBlock::ToolResult {
                            tool_use_id: id.clone(),
                            content: tool_result_content,
                            is_error: Some(execution_result.is_error()),
                            metadata: None,
                        });
                    }
                    Err(e) => {
                        // Create error tool result
                        tool_result_blocks.push(ContentBlock::ToolResult {
                            tool_use_id: id.clone(),
                            content: format!("Tool execution failed: {}", e),
                            is_error: Some(true),
                            metadata: None,
                        });
                    }
                }
            }
        }

        // If there were tool uses, we need to send the results back to Claude for interpretation
        if has_tool_uses {
            // Create a new message with the original assistant response and tool results
            let mut conversation_messages = Vec::new();

            // Add the assistant's response with tool uses
            conversation_messages.push(ClaudeMessage {
                role: MessageRole::Assistant,
                content: response.content.clone(),
                thinking: None,
            });

            // Add the tool results as a user message
            conversation_messages.push(ClaudeMessage {
                role: MessageRole::User,
                content: tool_result_blocks,
                thinking: None,
            });

            // Get tools for the second request
            let tools = self.tool_registry.get_all_tools();

            let system_message = if self.config.supports_thinking() {
                "You are a helpful AI assistant specialized in software development. You have access to various tools to help with file operations, code analysis, and development tasks. Feel free to use thinking mode to reason through complex problems."
            } else {
                "You are a helpful AI assistant specialized in software development. You have access to various tools to help with file operations, code analysis, and development tasks."
            };

            // Create second request to Claude with tool results
            let second_request = ClaudeRequest {
                model: self.config.model.clone(),
                max_tokens: self.config.max_tokens,
                temperature: self.config.temperature,
                messages: conversation_messages,
                tools: Some(tools),
                system: Some(system_message.to_string()),
            };

            // Make the second API call to get Claude's interpretation
            let second_response = self
                .error_handler
                .handle_with_retry(|| self.make_api_call(second_request.clone()))
                .await?;

            // Process the second response (should not have tool uses, but handle recursively if needed)
            return Box::pin(self.process_response_with_tools(second_response)).await;
        }

        // No tool uses - just return the text content from Claude's response
        let mut processed_content = Vec::new();
        for content_block in &response.content {
            match content_block {
                ContentBlock::Text { text } => {
                    processed_content.push(text.clone());
                }
                ContentBlock::Thinking { content } => {
                    // Thinking blocks can be processed or ignored based on configuration
                    if self.config.supports_thinking() {
                        processed_content.push(format!("Thinking: {}", content));
                    }
                }
                ContentBlock::ToolUse { .. } => {
                    // This shouldn't happen as we handle tool uses above, but be defensive
                    processed_content.push("Tool use encountered but not processed".to_string());
                }
                ContentBlock::ToolResult { .. } => {
                    // Tool results shouldn't be in the final response to user
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
