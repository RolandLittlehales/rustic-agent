/*!
 * Simplified Tool Execution Engine
 *
 * Minimal implementation that removes over-engineered abstractions
 * while maintaining the interface needed by ClaudeClient.
 */

use crate::claude::{
    error::{ClaudeError, ClaudeResult, ErrorContext},
    tools::{execution::ToolExecutionResult, AgentTool},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Extract file/directory path from tool input for logging context
fn extract_path_from_input(tool_name: &str, input: &Value) -> Option<String> {
    if let Value::Object(obj) = input {
        match tool_name {
            "read_file" | "write_file" => {
                // These tools use "path" parameter
                obj.get("path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            }
            "list_directory" => {
                // List directory tool uses "path" parameter
                obj.get("path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            }
            _ => None,
        }
    } else {
        None
    }
}

/// Minimal tool request for single tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub tool_name: String,
    pub input: serde_json::Value,
}

impl ToolRequest {
    pub fn new(tool_name: String, input: serde_json::Value) -> Self {
        Self { tool_name, input }
    }
}

/// Simplified tool execution engine focused on single tool execution
pub struct ToolExecutionEngine {
    tools: HashMap<String, Arc<dyn AgentTool>>,
}

impl ToolExecutionEngine {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool for execution
    pub fn register_tool(&mut self, tool: Arc<dyn AgentTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Execute a single tool with the given context
    pub async fn execute_single_tool(
        &self,
        request: ToolRequest,
        context: crate::claude::tools::ToolExecutionContext,
    ) -> ClaudeResult<ToolExecutionResult> {
        let start_time = Instant::now();

        let tool = self
            .tools
            .get(&request.tool_name)
            .ok_or_else(|| ClaudeError::ToolError {
                tool_name: request.tool_name.clone(),
                message: format!("Tool '{}' not found", request.tool_name),
                context: Some(
                    ErrorContext::new("tool_execution")
                        .add_metadata("tool_name", &request.tool_name),
                ),
            })?;

        // Execute the tool with retry logic
        let mut attempt_count = 0;
        let max_retries = context.metadata.max_retries;

        loop {
            attempt_count += 1;

            crate::log_debug!(
                "tool_execution",
                &format!(
                    "Executing tool '{}' (attempt {}/{})",
                    request.tool_name, attempt_count, max_retries
                ),
                {
                    let mut ctx = std::collections::HashMap::new();
                    ctx.insert("tool_name".to_string(), request.tool_name.clone());
                    ctx.insert("attempt".to_string(), attempt_count.to_string());
                    ctx.insert("max_retries".to_string(), max_retries.to_string());
                    ctx
                }
            );

            match tool.execute(request.input.clone()).await {
                Ok(result) => {
                    let execution_time = start_time.elapsed();
                    
                    // Extract file/directory path from input for better logging
                    let context_path = extract_path_from_input(&request.tool_name, &request.input);
                    if let Some(path) = context_path {
                        crate::log_tool_execution!(&request.tool_name, true, execution_time, &path);
                    } else {
                        crate::log_tool_execution!(&request.tool_name, true, execution_time);
                    }
                    
                    return Ok(ToolExecutionResult::success(
                        context.execution_id,
                        request.tool_name,
                        crate::claude::tools::execution::ToolResultData::Text(result),
                    ));
                }
                Err(e) => {
                    if attempt_count >= max_retries {
                        let execution_time = start_time.elapsed();
                        
                        // Extract file/directory path from input for better logging
                        let context_path = extract_path_from_input(&request.tool_name, &request.input);
                        if let Some(path) = context_path {
                            crate::log_tool_execution!(&request.tool_name, false, execution_time, &path);
                        } else {
                            crate::log_tool_execution!(&request.tool_name, false, execution_time);
                        }
                        
                        let tool_error = crate::claude::tools::execution::ToolError::validation_error(e.to_string());
                        return Ok(ToolExecutionResult::failure(
                            context.execution_id,
                            request.tool_name,
                            tool_error,
                            false, // not recoverable
                        ));
                    }

                    // Wait before retry with exponential backoff
                    let delay = Duration::from_millis(500 * (attempt_count as u64).pow(2));
                    
                    crate::log_warn!(
                        "tool_execution",
                        &format!(
                            "Tool '{}' failed, retrying in {}ms",
                            request.tool_name,
                            delay.as_millis()
                        ),
                        {
                            let mut ctx = std::collections::HashMap::new();
                            ctx.insert("tool_name".to_string(), request.tool_name.clone());
                            ctx.insert("error".to_string(), e.to_string());
                            ctx.insert("retry_delay_ms".to_string(), delay.as_millis().to_string());
                            ctx
                        }
                    );
                    
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

impl Default for ToolExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ToolExecutionEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolExecutionEngine")
            .field("tools_count", &self.tools.len())
            .field("registered_tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::tools::execution::ToolExecutionContext;
    use crate::claude::whitelist::WhitelistConfig;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_tool_execution_engine() {
        let mut engine = ToolExecutionEngine::new();
        
        // Test with empty engine
        let request = ToolRequest::new("nonexistent".to_string(), serde_json::json!({}));
        let context = ToolExecutionContext::new(
            "test".to_string(),
            serde_json::json!({}),
            Arc::new(RwLock::new(WhitelistConfig::default())),
        );
        
        let result = engine.execute_single_tool(request, context).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_tool_request_creation() {
        let request = ToolRequest::new(
            "test_tool".to_string(),
            serde_json::json!({"key": "value"}),
        );
        
        assert_eq!(request.tool_name, "test_tool");
        assert_eq!(request.input["key"], "value");
    }
}