use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl MessageContent {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: Vec<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<crate::claude::tool::ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: vec![MessageContent::text(content)],
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: vec![MessageContent::text(content)],
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: vec![MessageContent::text(content)],
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message with tool calls
    pub fn assistant_with_tools(content: impl Into<String>, tool_calls: Vec<crate::claude::tool::ToolCall>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: vec![MessageContent::text(content)],
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
    }

    /// Create a tool result message
    pub fn tool_result(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: vec![MessageContent::text(content)],
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    /// Get the text content of the message
    pub fn get_text(&self) -> String {
        self.content
            .iter()
            .filter(|c| c.content_type == "text")
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Check if this message has tool calls
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls.as_ref().map_or(false, |calls| !calls.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationHistory {
    messages: Vec<Message>,
    system_message: Option<String>,
}

impl ConversationHistory {
    /// Create a new conversation history
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            system_message: None,
        }
    }

    /// Create a conversation history with a system message
    pub fn with_system(system_message: impl Into<String>) -> Self {
        Self {
            messages: Vec::new(),
            system_message: Some(system_message.into()),
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Add a user message
    pub fn add_user_message(&mut self, content: impl Into<String>) {
        self.add_message(Message::user(content));
    }

    /// Add an assistant message
    pub fn add_assistant_message(&mut self, content: impl Into<String>) {
        self.add_message(Message::assistant(content));
    }

    /// Add an assistant message with tool calls
    pub fn add_assistant_message_with_tools(&mut self, content: impl Into<String>, tool_calls: Vec<crate::claude::tool::ToolCall>) {
        self.add_message(Message::assistant_with_tools(content, tool_calls));
    }

    /// Add a tool result message
    pub fn add_tool_result(&mut self, tool_call_id: impl Into<String>, content: impl Into<String>) {
        self.add_message(Message::tool_result(tool_call_id, content));
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Get the system message
    pub fn system_message(&self) -> Option<&str> {
        self.system_message.as_deref()
    }

    /// Set the system message
    pub fn set_system_message(&mut self, system_message: impl Into<String>) {
        self.system_message = Some(system_message.into());
    }

    /// Clear the conversation history
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get the number of messages
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if the conversation is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get the last message
    pub fn last_message(&self) -> Option<&Message> {
        self.messages.last()
    }

    /// Get the last assistant message
    pub fn last_assistant_message(&self) -> Option<&Message> {
        self.messages
            .iter()
            .rev()
            .find(|msg| msg.role == MessageRole::Assistant)
    }

    /// Remove messages beyond a certain count to manage context length
    pub fn truncate_to_last_n(&mut self, n: usize) {
        if self.messages.len() > n {
            let start_index = self.messages.len() - n;
            self.messages.drain(0..start_index);
        }
    }
}

impl Default for ConversationHistory {
    fn default() -> Self {
        Self::new()
    }
}