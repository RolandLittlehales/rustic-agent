use serde::{Deserialize, Serialize};

pub mod client;
pub mod tools;
pub mod types;

pub use client::ClaudeClient;

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
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message_id: String,
}

impl ConversationMessage {
    pub fn new_user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn new_assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
            timestamp: chrono::Utc::now(),
            message_id: uuid::Uuid::new_v4().to_string(),
        }
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
        self.add_message(ConversationMessage::new_user(content));
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.add_message(ConversationMessage::new_assistant(content));
    }
}
