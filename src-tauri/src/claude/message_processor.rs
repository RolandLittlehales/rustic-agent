use crate::claude::{
    error::{ClaudeError, ClaudeResult},
    types::{ClaudeMessage, ContentBlock, MessageRole},
    whitelist::WhitelistConfig,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct MessageProcessor {
    #[allow(dead_code)]
    tool_use_counter: u32,
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl MessageProcessor {
    pub fn new() -> Self {
        Self {
            tool_use_counter: 0,
            whitelist: None,
        }
    }

    /// Set the whitelist configuration for tool validation
    pub fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.whitelist = Some(whitelist);
    }

    /// Process a raw message string into structured ContentBlocks
    pub fn process_user_message(&self, content: &str) -> ClaudeResult<ClaudeMessage> {
        // Handle simple text messages
        if !content.contains("tool:") {
            return Ok(ClaudeMessage::user_text(content));
        }

        // Parse structured content (future enhancement)
        self.parse_structured_content(content)
    }

    /// Process Claude API response into structured message
    #[allow(dead_code)]
    pub fn process_claude_response(
        &self,
        response: &serde_json::Value,
    ) -> ClaudeResult<ClaudeMessage> {
        let content =
            response["content"]
                .as_array()
                .ok_or_else(|| ClaudeError::ValidationError {
                    field: "content".to_string(),
                    message: "Invalid response format - content must be array".to_string(),
                    context: None,
                })?;

        let content_blocks: Result<Vec<ContentBlock>, _> = content
            .iter()
            .map(|block| serde_json::from_value(block.clone()))
            .collect();

        let content_blocks = content_blocks.map_err(ClaudeError::JsonError)?;

        // Validate all content blocks
        for block in &content_blocks {
            self.validate_content_block(block)?;
        }

        let thinking = response["thinking"].as_str().map(|s| s.to_string());

        Ok(ClaudeMessage {
            role: MessageRole::Assistant,
            content: content_blocks,
            thinking,
        })
    }

    /// Generate tool use ID
    #[allow(dead_code)]
    pub fn generate_tool_use_id(&mut self) -> String {
        self.tool_use_counter += 1;
        let uuid_str = Uuid::new_v4().to_string();
        format!("toolu_{}", &uuid_str[..8])
    }

    /// Validate ContentBlock integrity
    #[allow(dead_code)]
    pub fn validate_content_block(&self, block: &ContentBlock) -> ClaudeResult<()> {
        match block {
            ContentBlock::Text { text } => {
                if text.is_empty() {
                    return Err(ClaudeError::ValidationError {
                        field: "text".to_string(),
                        message: "Text content cannot be empty".to_string(),
                        context: None,
                    });
                }
            }
            ContentBlock::ToolUse { id, name, .. } => {
                if id.is_empty() || name.is_empty() {
                    return Err(ClaudeError::ValidationError {
                        field: "tool_use".to_string(),
                        message: "Tool use must have valid id and name".to_string(),
                        context: None,
                    });
                }
            }
            ContentBlock::ToolResult {
                tool_use_id,
                content,
                ..
            } => {
                if tool_use_id.is_empty() || content.is_empty() {
                    return Err(ClaudeError::ValidationError {
                        field: "tool_result".to_string(),
                        message: "Tool result must have valid tool_use_id and content".to_string(),
                        context: None,
                    });
                }
            }
            ContentBlock::Thinking { content } => {
                if content.is_empty() {
                    return Err(ClaudeError::ValidationError {
                        field: "thinking".to_string(),
                        message: "Thinking content cannot be empty".to_string(),
                        context: None,
                    });
                }
            }
        }
        Ok(())
    }

    /// Validate tool use against whitelist
    pub async fn validate_tool_use(&self, tool_use: &ContentBlock) -> ClaudeResult<()> {
        if let ContentBlock::ToolUse { name, .. } = tool_use {
            if let Some(whitelist) = &self.whitelist {
                let whitelist_guard = whitelist.read().await;
                if !whitelist_guard.is_enabled() {
                    return Ok(());
                }

                // Basic tool name validation - in a real implementation this would
                // check against allowed tools and parameters
                if name.is_empty() {
                    return Err(ClaudeError::ToolError {
                        tool_name: name.clone(),
                        message: "Empty tool name not allowed".to_string(),
                        context: None,
                    });
                }
            }
        }
        Ok(())
    }

    /// Convert legacy string message to ContentBlock
    #[allow(dead_code)]
    pub fn migrate_legacy_message(&self, role: &str, content: &str) -> ClaudeMessage {
        let message_role = match role {
            "user" => MessageRole::User,
            "assistant" => MessageRole::Assistant,
            "system" => MessageRole::System,
            _ => MessageRole::User,
        };

        ClaudeMessage {
            role: message_role,
            content: vec![ContentBlock::text(content)],
            thinking: None,
        }
    }

    /// Extract text from content blocks
    #[allow(dead_code)]
    pub fn extract_text_content(&self, blocks: &[ContentBlock]) -> String {
        blocks
            .iter()
            .filter_map(|block| block.get_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Create tool result content block
    #[allow(dead_code)]
    pub fn create_tool_result(
        &mut self,
        tool_use_id: String,
        content: String,
        is_error: bool,
    ) -> ContentBlock {
        ContentBlock::tool_result(tool_use_id, content, Some(is_error))
    }

    /// Process multiple content blocks and ensure consistency
    #[allow(dead_code)]
    pub fn process_content_blocks(
        &self,
        blocks: Vec<ContentBlock>,
    ) -> ClaudeResult<Vec<ContentBlock>> {
        let mut processed_blocks = Vec::new();

        for block in blocks {
            self.validate_content_block(&block)?;
            processed_blocks.push(block);
        }

        // Ensure at least one content block exists
        if processed_blocks.is_empty() {
            return Err(ClaudeError::ValidationError {
                field: "content_blocks".to_string(),
                message: "At least one content block is required".to_string(),
                context: None,
            });
        }

        Ok(processed_blocks)
    }

    fn parse_structured_content(&self, content: &str) -> ClaudeResult<ClaudeMessage> {
        // Implementation for parsing structured content
        // This would handle tool calls embedded in text
        // For now, just treat as text
        Ok(ClaudeMessage::user_text(content))
    }
}

impl Default for MessageProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_process_simple_message() {
        let processor = MessageProcessor::new();
        let result = processor.process_user_message("Hello").unwrap();
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.get_text_content(), "Hello");
        assert_eq!(result.role, MessageRole::User);
    }

    #[test]
    fn test_process_claude_response() {
        let processor = MessageProcessor::new();
        let response = json!({
            "content": [
                {"type": "text", "text": "Hello"},
                {"type": "tool_use", "id": "test", "name": "test_tool", "input": {}}
            ]
        });
        let result = processor.process_claude_response(&response).unwrap();
        assert_eq!(result.content.len(), 2);
        assert_eq!(result.get_tool_uses().len(), 1);
        assert_eq!(result.role, MessageRole::Assistant);
    }

    #[test]
    fn test_validate_content_block() {
        let processor = MessageProcessor::new();

        // Valid text block
        let text_block = ContentBlock::text("Hello");
        assert!(processor.validate_content_block(&text_block).is_ok());

        // Invalid empty text block
        let empty_text_block = ContentBlock::Text {
            text: String::new(),
        };
        assert!(processor.validate_content_block(&empty_text_block).is_err());
    }

    #[test]
    fn test_migrate_legacy_message() {
        let processor = MessageProcessor::new();
        let result = processor.migrate_legacy_message("user", "Hello");
        assert_eq!(result.role, MessageRole::User);
        assert_eq!(result.get_text_content(), "Hello");
    }

    #[test]
    fn test_generate_tool_use_id() {
        let mut processor = MessageProcessor::new();
        let id1 = processor.generate_tool_use_id();
        let id2 = processor.generate_tool_use_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with("toolu_"));
    }
}
