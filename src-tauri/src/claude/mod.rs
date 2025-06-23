use crate::claude::error::{ClaudeError, ClaudeResult};
use crate::claude::types::{ContentBlock, MessageRole};
use serde::{Deserialize, Serialize};

pub mod client;
pub mod error;
pub mod message;
pub mod message_processor;
pub mod tools;
pub mod types;
pub mod whitelist;

pub use client::ClaudeClient;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    #[allow(dead_code)]
    pub family: String,
    #[allow(dead_code)]
    pub variant: String,
    #[allow(dead_code)]
    pub max_tokens: u32,
    pub supports_thinking: bool,
    #[allow(dead_code)]
    pub supports_tool_use: bool,
    #[allow(dead_code)]
    pub cost_per_million_input: f64,
    #[allow(dead_code)]
    pub cost_per_million_output: f64,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            family: "claude-3-5".to_string(),
            variant: "sonnet".to_string(),
            max_tokens: 4096,
            supports_thinking: false,
            supports_tool_use: true,
            cost_per_million_input: 3.0,
            cost_per_million_output: 15.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 8192, // Increased for Claude 4
            temperature: 0.7,
        }
    }
}

// Model configuration validation and info
impl ClaudeConfig {
    pub fn validate(&self) -> ClaudeResult<()> {
        if self.api_key.is_empty() {
            return Err(ClaudeError::ConfigError {
                message: "API key cannot be empty".to_string(),
                context: None,
            });
        }

        if !self.is_valid_model() {
            return Err(ClaudeError::ConfigError {
                message: format!("Invalid model: {}", self.model),
                context: None,
            });
        }

        if self.max_tokens == 0 || self.max_tokens > 200000 {
            return Err(ClaudeError::ConfigError {
                message: "Max tokens must be between 1 and 200000".to_string(),
                context: None,
            });
        }

        if !(0.0..=1.0).contains(&self.temperature) {
            return Err(ClaudeError::ConfigError {
                message: "Temperature must be between 0.0 and 1.0".to_string(),
                context: None,
            });
        }

        Ok(())
    }

    fn is_valid_model(&self) -> bool {
        matches!(
            self.model.as_str(),
            "claude-sonnet-4-20250514"
                | "claude-3-5-sonnet-20241022"
                | "claude-3-5-haiku-20241022"
                | "claude-3-opus-20240229"
                | "claude-3-sonnet-20240229"
                | "claude-3-haiku-20240307"
        )
    }

    pub fn get_model_info(&self) -> ModelInfo {
        match self.model.as_str() {
            "claude-sonnet-4-20250514" => ModelInfo {
                family: "claude-4".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: 200000,
                supports_thinking: true,
                supports_tool_use: true,
                cost_per_million_input: 3.0,
                cost_per_million_output: 15.0,
            },
            "claude-3-5-sonnet-20241022" => ModelInfo {
                family: "claude-3-5".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: 200000,
                supports_thinking: false,
                supports_tool_use: true,
                cost_per_million_input: 3.0,
                cost_per_million_output: 15.0,
            },
            "claude-3-5-haiku-20241022" => ModelInfo {
                family: "claude-3-5".to_string(),
                variant: "haiku".to_string(),
                max_tokens: 200000,
                supports_thinking: false,
                supports_tool_use: true,
                cost_per_million_input: 0.25,
                cost_per_million_output: 1.25,
            },
            "claude-3-opus-20240229" => ModelInfo {
                family: "claude-3".to_string(),
                variant: "opus".to_string(),
                max_tokens: 4096,
                supports_thinking: false,
                supports_tool_use: true,
                cost_per_million_input: 15.0,
                cost_per_million_output: 75.0,
            },
            "claude-3-sonnet-20240229" => ModelInfo {
                family: "claude-3".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: 4096,
                supports_thinking: false,
                supports_tool_use: true,
                cost_per_million_input: 3.0,
                cost_per_million_output: 15.0,
            },
            "claude-3-haiku-20240307" => ModelInfo {
                family: "claude-3".to_string(),
                variant: "haiku".to_string(),
                max_tokens: 4096,
                supports_thinking: false,
                supports_tool_use: true,
                cost_per_million_input: 0.25,
                cost_per_million_output: 1.25,
            },
            _ => ModelInfo::default(),
        }
    }

    #[allow(dead_code)]
    pub fn is_claude_4(&self) -> bool {
        self.model.starts_with("claude-4") || self.model.contains("-4-")
    }

    pub fn supports_thinking(&self) -> bool {
        self.get_model_info().supports_thinking
    }

    #[allow(dead_code)]
    pub fn supports_tool_use(&self) -> bool {
        self.get_model_info().supports_tool_use
    }

    #[allow(dead_code)]
    pub fn get_max_model_tokens(&self) -> u32 {
        self.get_model_info().max_tokens
    }

    #[allow(dead_code)]
    pub fn estimate_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        let model_info = self.get_model_info();
        let input_cost = (input_tokens as f64 / 1_000_000.0) * model_info.cost_per_million_input;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * model_info.cost_per_million_output;
        input_cost + output_cost
    }

    #[allow(dead_code)]
    pub fn with_model(mut self, model: impl Into<String>) -> ClaudeResult<Self> {
        self.model = model.into();
        if !self.is_valid_model() {
            return Err(ClaudeError::ConfigError {
                message: format!("Invalid model: {}", self.model),
                context: None,
            });
        }
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> ClaudeResult<Self> {
        let model_max = self.get_max_model_tokens();
        if max_tokens > model_max {
            return Err(ClaudeError::ConfigError {
                message: format!(
                    "Max tokens {} exceeds model limit {}",
                    max_tokens, model_max
                ),
                context: None,
            });
        }
        self.max_tokens = max_tokens;
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn with_temperature(mut self, temperature: f32) -> ClaudeResult<Self> {
        if !(0.0..=1.0).contains(&temperature) {
            return Err(ClaudeError::ConfigError {
                message: "Temperature must be between 0.0 and 1.0".to_string(),
                context: None,
            });
        }
        self.temperature = temperature;
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_message_id: Option<String>,
}

impl ConversationMessage {
    pub fn new_user_text(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: vec![ContentBlock::text(content)],
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
            thinking: None,
            tool_use_id: None,
            parent_message_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_user_blocks(content: Vec<ContentBlock>) -> Self {
        Self {
            role: MessageRole::User,
            content,
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
            thinking: None,
            tool_use_id: None,
            parent_message_id: None,
        }
    }

    pub fn new_assistant_text(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: vec![ContentBlock::text(content)],
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
            thinking: None,
            tool_use_id: None,
            parent_message_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_assistant_blocks(content: Vec<ContentBlock>, thinking: Option<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
            thinking,
            tool_use_id: None,
            parent_message_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_tool_result(
        tool_use_id: String,
        content: String,
        is_error: bool,
        parent_id: Option<String>,
    ) -> Self {
        Self {
            role: MessageRole::User,
            content: vec![ContentBlock::tool_result(
                tool_use_id.clone(),
                content,
                Some(is_error),
            )],
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
            thinking: None,
            tool_use_id: Some(tool_use_id),
            parent_message_id: parent_id,
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
    pub fn has_tool_uses(&self) -> bool {
        self.content.iter().any(|block| block.is_tool_use())
    }

    #[allow(dead_code)]
    pub fn get_tool_uses(&self) -> Vec<&ContentBlock> {
        self.content
            .iter()
            .filter(|block| block.is_tool_use())
            .collect()
    }

    // Legacy compatibility methods
    #[allow(dead_code)]
    pub fn new_user(content: String) -> Self {
        Self::new_user_text(content)
    }

    #[allow(dead_code)]
    pub fn new_assistant(content: String) -> Self {
        Self::new_assistant_text(content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub messages: Vec<ConversationMessage>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            messages: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl Conversation {
    pub fn add_message(&mut self, message: ConversationMessage) {
        self.messages.push(message);
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_user_message(&mut self, content: String) {
        self.add_message(ConversationMessage::new_user_text(content));
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.add_message(ConversationMessage::new_assistant_text(content));
    }

    #[allow(dead_code)]
    pub fn add_user_blocks(&mut self, content: Vec<ContentBlock>) {
        self.add_message(ConversationMessage::new_user_blocks(content));
    }

    #[allow(dead_code)]
    pub fn add_assistant_blocks(&mut self, content: Vec<ContentBlock>, thinking: Option<String>) {
        self.add_message(ConversationMessage::new_assistant_blocks(content, thinking));
    }

    #[allow(dead_code)]
    pub fn add_tool_result(
        &mut self,
        tool_use_id: String,
        content: String,
        is_error: bool,
        parent_id: Option<String>,
    ) {
        self.add_message(ConversationMessage::new_tool_result(
            tool_use_id,
            content,
            is_error,
            parent_id,
        ));
    }

    #[allow(dead_code)]
    pub fn get_text_history(&self) -> String {
        self.messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    MessageRole::User => "User",
                    MessageRole::Assistant => "Assistant",
                    MessageRole::System => "System",
                };
                format!("{}: {}", role, msg.get_text_content())
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_config_default() {
        let config = ClaudeConfig::default();
        assert_eq!(config.model, "claude-sonnet-4-20250514");
        assert_eq!(config.max_tokens, 8192);
        assert_eq!(config.temperature, 0.7);
        assert!(config.is_claude_4());
        assert!(config.supports_thinking());
        assert!(config.supports_tool_use());
    }

    #[test]
    fn test_claude_config_validation() {
        let mut config = ClaudeConfig::default();
        config.api_key = "test_key".to_string();
        assert!(config.validate().is_ok());

        // Test empty API key
        config.api_key = String::new();
        assert!(config.validate().is_err());

        // Test invalid model
        config.api_key = "test_key".to_string();
        config.model = "invalid-model".to_string();
        assert!(config.validate().is_err());

        // Test invalid temperature
        config.model = "claude-sonnet-4-20250514".to_string();
        config.temperature = 1.5;
        assert!(config.validate().is_err());

        // Test invalid max_tokens
        config.temperature = 0.7;
        config.max_tokens = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_model_info() {
        let config = ClaudeConfig::default();
        let model_info = config.get_model_info();

        assert_eq!(model_info.family, "claude-4");
        assert_eq!(model_info.variant, "sonnet");
        assert_eq!(model_info.max_tokens, 200000);
        assert!(model_info.supports_thinking);
        assert!(model_info.supports_tool_use);
        assert_eq!(model_info.cost_per_million_input, 3.0);
        assert_eq!(model_info.cost_per_million_output, 15.0);
    }

    #[test]
    fn test_claude_3_5_model_info() {
        let config = ClaudeConfig {
            api_key: "test".to_string(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        };
        let model_info = config.get_model_info();

        assert_eq!(model_info.family, "claude-3-5");
        assert!(!model_info.supports_thinking);
        assert!(model_info.supports_tool_use);
    }

    #[test]
    fn test_cost_estimation() {
        let config = ClaudeConfig::default();
        let cost = config.estimate_cost(1000, 500);

        // 1000 input tokens at 3.0 per million = 0.003
        // 500 output tokens at 15.0 per million = 0.0075
        // Total = 0.0105
        assert!((cost - 0.0105).abs() < 0.0001);
    }

    #[test]
    fn test_config_builder_methods() {
        let config = ClaudeConfig::default()
            .with_model("claude-3-5-sonnet-20241022")
            .unwrap()
            .with_max_tokens(4096)
            .unwrap()
            .with_temperature(0.5)
            .unwrap();

        assert_eq!(config.model, "claude-3-5-sonnet-20241022");
        assert_eq!(config.max_tokens, 4096);
        assert_eq!(config.temperature, 0.5);
    }

    #[test]
    fn test_conversation_message_creation() {
        let msg = ConversationMessage::new_user_text("Hello");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content.len(), 1);
        assert_eq!(msg.get_text_content(), "Hello");
        assert!(!msg.has_tool_uses());
    }

    #[test]
    fn test_conversation_message_with_thinking() {
        let blocks = vec![ContentBlock::text("Response")];
        let msg =
            ConversationMessage::new_assistant_blocks(blocks, Some("Thinking...".to_string()));
        assert_eq!(msg.role, MessageRole::Assistant);
        assert_eq!(msg.thinking, Some("Thinking...".to_string()));
    }

    #[test]
    fn test_conversation_message_tool_result() {
        let msg = ConversationMessage::new_tool_result(
            "tool_123".to_string(),
            "Success".to_string(),
            false,
            Some("parent_456".to_string()),
        );
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.tool_use_id, Some("tool_123".to_string()));
        assert_eq!(msg.parent_message_id, Some("parent_456".to_string()));
    }

    #[test]
    fn test_conversation_management() {
        let mut conversation = Conversation::default();

        conversation.add_user_message("Hello".to_string());
        conversation.add_assistant_message("Hi there!".to_string());

        assert_eq!(conversation.messages.len(), 2);
        assert_eq!(conversation.messages[0].role, MessageRole::User);
        assert_eq!(conversation.messages[1].role, MessageRole::Assistant);

        let history = conversation.get_text_history();
        assert!(history.contains("User: Hello"));
        assert!(history.contains("Assistant: Hi there!"));
    }

    #[test]
    fn test_legacy_compatibility() {
        let user_msg = ConversationMessage::new_user("Legacy message".to_string());
        let assistant_msg = ConversationMessage::new_assistant("Legacy response".to_string());

        assert_eq!(user_msg.role, MessageRole::User);
        assert_eq!(assistant_msg.role, MessageRole::Assistant);
        assert_eq!(user_msg.get_text_content(), "Legacy message");
        assert_eq!(assistant_msg.get_text_content(), "Legacy response");
    }
}
