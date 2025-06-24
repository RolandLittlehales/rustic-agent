use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClaudeMessage {
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
        is_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<crate::claude::tools::ToolResultMetadata>,
    },
    // Future-ready for streaming
    Thinking {
        content: String,
    },
}

impl ContentBlock {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    #[allow(dead_code)]
    pub fn tool_use(
        id: impl Into<String>,
        name: impl Into<String>,
        input: serde_json::Value,
    ) -> Self {
        Self::ToolUse {
            id: id.into(),
            name: name.into(),
            input,
        }
    }

    #[allow(dead_code)]
    pub fn tool_result(
        tool_use_id: impl Into<String>,
        content: impl Into<String>,
        is_error: Option<bool>,
    ) -> Self {
        Self::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error,
            metadata: None,
        }
    }

    pub fn tool_result_with_metadata(
        tool_use_id: impl Into<String>,
        content: impl Into<String>,
        is_error: Option<bool>,
        metadata: crate::claude::tools::ToolResultMetadata,
    ) -> Self {
        Self::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error,
            metadata: Some(metadata),
        }
    }

    #[allow(dead_code)]
    pub fn thinking(content: impl Into<String>) -> Self {
        Self::Thinking {
            content: content.into(),
        }
    }

    #[allow(dead_code)]
    pub fn get_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            ContentBlock::ToolResult { content, .. } => Some(content),
            ContentBlock::Thinking { content } => Some(content),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn is_tool_use(&self) -> bool {
        matches!(self, ContentBlock::ToolUse { .. })
    }

    #[allow(dead_code)]
    pub fn is_tool_result(&self) -> bool {
        matches!(self, ContentBlock::ToolResult { .. })
    }

    #[allow(dead_code)]
    pub fn is_thinking(&self) -> bool {
        matches!(self, ContentBlock::Thinking { .. })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeRequest {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub messages: Vec<ClaudeMessage>,
    pub tools: Option<Vec<Tool>>,
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub r#type: String,
    pub role: String,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: ToolInputSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    pub r#type: String,
    pub properties: HashMap<String, PropertySchema>,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    pub r#type: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<PropertySchema>>,
}

impl ClaudeMessage {
    pub fn user_text(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: vec![ContentBlock::text(content)],
            thinking: None,
        }
    }

    #[allow(dead_code)]
    pub fn assistant_text(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: vec![ContentBlock::text(content)],
            thinking: None,
        }
    }

    #[allow(dead_code)]
    pub fn user_blocks(content: Vec<ContentBlock>) -> Self {
        Self {
            role: MessageRole::User,
            content,
            thinking: None,
        }
    }

    #[allow(dead_code)]
    pub fn assistant_blocks(content: Vec<ContentBlock>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            thinking: None,
        }
    }

    #[allow(dead_code)]
    pub fn assistant_with_thinking(content: Vec<ContentBlock>, thinking: Option<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            thinking,
        }
    }

    #[allow(dead_code)]
    pub fn get_text_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|block| block.get_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[allow(dead_code)]
    pub fn get_tool_uses(&self) -> Vec<&ContentBlock> {
        self.content
            .iter()
            .filter(|block| block.is_tool_use())
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_tool_results(&self) -> Vec<&ContentBlock> {
        self.content
            .iter()
            .filter(|block| block.is_tool_result())
            .collect()
    }

    #[allow(dead_code)]
    pub fn has_tool_uses(&self) -> bool {
        self.content.iter().any(|block| block.is_tool_use())
    }

    #[allow(dead_code)]
    pub fn has_thinking(&self) -> bool {
        self.thinking.is_some() || self.content.iter().any(|block| block.is_thinking())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_content_block_creation() {
        let text_block = ContentBlock::text("Hello world");
        assert_eq!(text_block.get_text(), Some("Hello world"));
        assert!(!text_block.is_tool_use());
        assert!(!text_block.is_tool_result());
        assert!(!text_block.is_thinking());
    }

    #[test]
    fn test_tool_use_block() {
        let tool_block = ContentBlock::tool_use("test_id", "test_tool", json!({"arg": "value"}));
        assert!(tool_block.is_tool_use());
        assert!(!tool_block.is_tool_result());
        assert!(!tool_block.is_thinking());
        assert_eq!(tool_block.get_text(), None);
    }

    #[test]
    fn test_tool_result_block() {
        let result_block =
            ContentBlock::tool_result("test_id", "Tool executed successfully", Some(false));
        assert!(!result_block.is_tool_use());
        assert!(result_block.is_tool_result());
        assert!(!result_block.is_thinking());
        assert_eq!(result_block.get_text(), Some("Tool executed successfully"));
    }

    #[test]
    fn test_thinking_block() {
        let thinking_block = ContentBlock::thinking("Let me think about this...");
        assert!(!thinking_block.is_tool_use());
        assert!(!thinking_block.is_tool_result());
        assert!(thinking_block.is_thinking());
        assert_eq!(
            thinking_block.get_text(),
            Some("Let me think about this...")
        );
    }

    #[test]
    fn test_content_block_serialization() {
        let block = ContentBlock::text("test");
        let serialized = serde_json::to_string(&block).unwrap();
        let deserialized: ContentBlock = serde_json::from_str(&serialized).unwrap();
        assert_eq!(block, deserialized);
    }

    #[test]
    fn test_complex_content_block_serialization() {
        let tool_block =
            ContentBlock::tool_use("test_123", "read_file", json!({"path": "test.txt"}));
        let serialized = serde_json::to_string(&tool_block).unwrap();
        let deserialized: ContentBlock = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tool_block, deserialized);
    }

    #[test]
    fn test_claude_message_creation() {
        let user_msg = ClaudeMessage::user_text("Hello Claude 4!");
        assert_eq!(user_msg.role, MessageRole::User);
        assert_eq!(user_msg.content.len(), 1);
        assert_eq!(user_msg.get_text_content(), "Hello Claude 4!");
        assert!(!user_msg.has_tool_uses());
        assert!(!user_msg.has_thinking());
    }

    #[test]
    fn test_claude_message_with_thinking() {
        let assistant_msg = ClaudeMessage::assistant_with_thinking(
            vec![ContentBlock::text("Let me help you with that.")],
            Some("The user is asking for help.".to_string()),
        );
        assert_eq!(assistant_msg.role, MessageRole::Assistant);
        assert!(assistant_msg.has_thinking());
        assert_eq!(
            assistant_msg.thinking,
            Some("The user is asking for help.".to_string())
        );
    }

    #[test]
    fn test_claude_message_with_tools() {
        let blocks = vec![
            ContentBlock::text("I'll read that file for you."),
            ContentBlock::tool_use("tool_123", "read_file", json!({"path": "test.txt"})),
        ];
        let message = ClaudeMessage::assistant_blocks(blocks);

        assert!(message.has_tool_uses());
        assert_eq!(message.get_tool_uses().len(), 1);
        assert_eq!(message.get_tool_results().len(), 0);

        let text_content = message.get_text_content();
        assert!(text_content.contains("I'll read that file for you."));
    }

    #[test]
    fn test_claude_message_mixed_content() {
        let blocks = vec![
            ContentBlock::text("Here's the result:"),
            ContentBlock::tool_result("tool_123", "File contents: Hello World", Some(false)),
            ContentBlock::thinking("The file was read successfully."),
        ];
        let message = ClaudeMessage::assistant_blocks(blocks);

        assert!(!message.has_tool_uses());
        assert_eq!(message.get_tool_results().len(), 1);
        assert!(message.has_thinking());

        let text_content = message.get_text_content();
        assert!(text_content.contains("Here's the result:"));
        assert!(text_content.contains("File contents: Hello World"));
        assert!(text_content.contains("The file was read successfully."));
    }

    #[test]
    fn test_message_role_serialization() {
        let user_role = MessageRole::User;
        let serialized = serde_json::to_string(&user_role).unwrap();
        assert_eq!(serialized, "\"user\"");

        let assistant_role = MessageRole::Assistant;
        let serialized = serde_json::to_string(&assistant_role).unwrap();
        assert_eq!(serialized, "\"assistant\"");
    }

    #[test]
    fn test_claude_message_serialization() {
        let message = ClaudeMessage::user_text("Test message");
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: ClaudeMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[test]
    fn test_claude_request_creation() {
        let messages = vec![ClaudeMessage::user_text("Hello")];
        let request = ClaudeRequest {
            model: "claude-4-sonnet-20250522".to_string(),
            max_tokens: 8192,
            temperature: 0.7,
            messages,
            tools: None,
            system: Some("Test system message".to_string()),
        };

        assert_eq!(request.model, "claude-4-sonnet-20250522");
        assert_eq!(request.max_tokens, 8192);
        assert_eq!(request.messages.len(), 1);
    }
}
