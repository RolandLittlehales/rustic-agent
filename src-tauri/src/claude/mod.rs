use crate::claude::constants::model_config;
use crate::claude::error::{ClaudeError, ClaudeResult};
use crate::claude::types::{ContentBlock, MessageRole};
use serde::{Deserialize, Serialize};

pub mod client;
pub mod connection_pool;
pub mod constants;
pub mod error;
pub mod message;
pub mod message_processor;
pub mod model_registry;
pub mod tools;
pub mod types;
pub mod whitelist;

pub use client::ClaudeClient;
pub use model_registry::{ModelInfo, ModelRegistry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    #[serde(skip)]
    pub model_registry: ModelRegistry,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: model_config::default_model().to_string(),
            max_tokens: model_config::DEFAULT_MAX_TOKENS,
            temperature: model_config::DEFAULT_TEMPERATURE,
            model_registry: ModelRegistry::new(),
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

        // Validate model using registry
        self.model_registry.validate_model(&self.model)?;

        // Validate max_tokens against model limits
        if let Some(model_info) = self.model_registry.get_model_info(&self.model) {
            if self.max_tokens == 0 {
                return Err(ClaudeError::ValidationError {
                    field: "max_tokens".to_string(),
                    message: "Max tokens must be greater than 0".to_string(),
                    context: None,
                });
            }

            if self.max_tokens > model_info.max_tokens {
                return Err(ClaudeError::ValidationError {
                    field: "max_tokens".to_string(),
                    message: format!(
                        "Max tokens ({}) exceeds model limit ({})",
                        self.max_tokens, model_info.max_tokens
                    ),
                    context: None,
                });
            }
        }

        if !(0.0..=1.0).contains(&self.temperature) {
            return Err(ClaudeError::ValidationError {
                field: "temperature".to_string(),
                message: "Temperature must be between 0.0 and 1.0".to_string(),
                context: None,
            });
        }

        Ok(())
    }

    pub fn get_model_info(&self) -> Option<&ModelInfo> {
        self.model_registry.get_model_info(&self.model)
    }

    #[allow(dead_code)]
    pub fn is_claude_4(&self) -> bool {
        self.model.starts_with("claude-4") || self.model.contains("-4-")
    }

    pub fn supports_thinking(&self) -> bool {
        self.get_model_info()
            .map(|info| info.supports_thinking)
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn supports_tool_use(&self) -> bool {
        self.get_model_info()
            .map(|info| info.supports_tool_use)
            .unwrap_or(true)
    }

    #[allow(dead_code)]
    pub fn get_max_model_tokens(&self) -> u32 {
        self.get_model_info()
            .map(|info| info.max_tokens)
            .unwrap_or(model_config::FALLBACK_MAX_TOKENS)
    }

    #[allow(dead_code)]
    pub fn estimate_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        if let Some(model_info) = self.get_model_info() {
            let input_cost =
                (input_tokens as f64 / 1_000_000.0) * model_info.cost_per_million_input;
            let output_cost =
                (output_tokens as f64 / 1_000_000.0) * model_info.cost_per_million_output;
            input_cost + output_cost
        } else {
            0.0 // Default fallback
        }
    }

    #[allow(dead_code)]
    pub fn with_model(mut self, model: impl Into<String>) -> ClaudeResult<Self> {
        self.model = model.into();
        // Use ModelRegistry for validation instead of hardcoded list
        self.model_registry.validate_model(&self.model)?;
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
    use crate::claude::constants::test_data;

    #[test]
    fn test_claude_config_default() {
        let config = ClaudeConfig::default();
        assert_eq!(config.model, model_config::default_model());
        assert_eq!(config.max_tokens, model_config::DEFAULT_MAX_TOKENS);
        assert_eq!(config.temperature, model_config::DEFAULT_TEMPERATURE);
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
        let model_info = config.get_model_info().unwrap();

        assert_eq!(model_info.family, "claude-4");
        assert_eq!(model_info.variant, "sonnet");
        assert_eq!(model_info.max_tokens, model_config::DEFAULT_MAX_TOKENS);
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
            max_tokens: test_data::TEST_MAX_TOKENS,
            temperature: model_config::DEFAULT_TEMPERATURE,
            model_registry: ModelRegistry::new(),
        };
        let model_info = config.get_model_info().unwrap();

        assert_eq!(model_info.family, "claude-3.5");
        assert!(!model_info.supports_thinking);
        assert!(model_info.supports_tool_use);
    }

    #[test]
    fn test_cost_estimation() {
        let config = ClaudeConfig::default();
        let cost =
            config.estimate_cost(test_data::TEST_INPUT_TOKENS, test_data::TEST_OUTPUT_TOKENS);

        // Expected cost from test constants
        assert!(
            (cost - test_data::EXPECTED_CLAUDE_4_SONNET_COST).abs()
                < test_data::COST_CALCULATION_TOLERANCE
        );
    }

    #[test]
    fn test_config_builder_methods() {
        let config = ClaudeConfig::default()
            .with_model("claude-3-5-sonnet-20241022")
            .unwrap()
            .with_max_tokens(test_data::TEST_MAX_TOKENS)
            .unwrap()
            .with_temperature(test_data::TEST_TEMPERATURE)
            .unwrap();

        assert_eq!(config.model, "claude-3-5-sonnet-20241022");
        assert_eq!(config.max_tokens, test_data::TEST_MAX_TOKENS);
        assert_eq!(config.temperature, test_data::TEST_TEMPERATURE);
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
