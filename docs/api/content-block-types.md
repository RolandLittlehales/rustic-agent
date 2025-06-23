# ContentBlock API Reference

## Core Types

### ContentBlock Enum

The fundamental unit of content in the Claude agent system.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { 
        text: String 
    },
    ToolUse { 
        id: String, 
        name: String, 
        input: serde_json::Value 
    },
    ToolResult { 
        tool_use_id: String, 
        content: String, 
        is_error: Option<bool> 
    },
    Thinking { 
        content: String 
    },
}
```

#### Variants

##### Text
Regular text content for display or processing.

**Fields:**
- `text: String` - The text content

**Example:**
```json
{
  "type": "text",
  "text": "Hello, I can help you with that."
}
```

##### ToolUse
Request to execute a tool with specified parameters.

**Fields:**
- `id: String` - Unique identifier for this tool use
- `name: String` - Name of the tool to execute
- `input: serde_json::Value` - JSON parameters for the tool

**Example:**
```json
{
  "type": "tool_use",
  "id": "tool_use_abc123",
  "name": "read_file",
  "input": {
    "path": "/home/user/document.txt"
  }
}
```

##### ToolResult
Result from a tool execution.

**Fields:**
- `tool_use_id: String` - ID of the corresponding ToolUse
- `content: String` - Result content
- `is_error: Option<bool>` - Whether this is an error result

**Example:**
```json
{
  "type": "tool_result",
  "tool_use_id": "tool_use_abc123",
  "content": "File contents: Hello World",
  "is_error": false
}
```

##### Thinking
Claude's internal reasoning (when thinking mode is enabled).

**Fields:**
- `content: String` - The thinking content

**Example:**
```json
{
  "type": "thinking",
  "content": "The user wants to read a file. I should check if it exists first..."
}
```

### ClaudeMessage

A message containing one or more ContentBlocks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: Vec<ContentBlock>,
    pub thinking: Option<Vec<ContentBlock>>,
}
```

**Fields:**
- `role: String` - Either "user" or "assistant"
- `content: Vec<ContentBlock>` - The message content blocks
- `thinking: Option<Vec<ContentBlock>>` - Optional thinking blocks

**Example:**
```json
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll read that file for you."
    },
    {
      "type": "tool_use",
      "id": "tool_123",
      "name": "read_file",
      "input": { "path": "example.txt" }
    }
  ],
  "thinking": null
}
```

### ErrorContext

Rich error information for debugging and monitoring.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message_id: Option<String>,
    pub tool_use_id: Option<String>,
    pub retry_count: u32,
    pub metadata: HashMap<String, String>,
}
```

**Fields:**
- `operation: String` - The operation that failed
- `timestamp: DateTime<Utc>` - When the error occurred
- `message_id: Option<String>` - Related message ID
- `tool_use_id: Option<String>` - Related tool use ID
- `retry_count: u32` - Number of retry attempts
- `metadata: HashMap<String, String>` - Additional context

### ClaudeError

Comprehensive error enumeration with context.

```rust
#[derive(Debug)]
pub enum ClaudeError {
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
        param: Option<String>,
        context: Option<ErrorContext>,
    },
    ModelError {
        model: String,
        message: String,
        context: Option<ErrorContext>,
    },
    ContentBlockError {
        block_type: String,
        message: String,
        context: Option<ErrorContext>,
    },
    ToolError {
        tool_name: String,
        message: String,
        context: Option<ErrorContext>,
    },
    ConfigError {
        message: String,
        context: Option<ErrorContext>,
    },
    ValidationError {
        field: String,
        message: String,
        context: Option<ErrorContext>,
    },
    StreamingError {
        message: String,
        context: Option<ErrorContext>,
    },
    TimeoutError {
        duration: std::time::Duration,
        context: Option<ErrorContext>,
    },
    RateLimitError {
        retry_after: Option<u64>,
        context: Option<ErrorContext>,
    },
}
```

## Traits

### ContentBlock Methods

```rust
impl ContentBlock {
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            ContentBlock::Text { .. } => "text",
            ContentBlock::ToolUse { .. } => "tool_use",
            ContentBlock::ToolResult { .. } => "tool_result",
            ContentBlock::Thinking { .. } => "thinking",
        }
    }

    /// Check if this is a text block
    pub fn is_text(&self) -> bool {
        matches!(self, ContentBlock::Text { .. })
    }

    /// Check if this is a tool use block
    pub fn is_tool_use(&self) -> bool {
        matches!(self, ContentBlock::ToolUse { .. })
    }

    /// Extract text content if this is a text block
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            _ => None,
        }
    }
}
```

### ClaudeMessage Methods

```rust
impl ClaudeMessage {
    /// Create a user message with text content
    pub fn user_text(text: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text: text.into() }],
            thinking: None,
        }
    }

    /// Create an assistant message with text content
    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: vec![ContentBlock::Text { text: text.into() }],
            thinking: None,
        }
    }

    /// Add a content block to this message
    pub fn add_content(&mut self, block: ContentBlock) {
        self.content.push(block);
    }

    /// Get all text content concatenated
    pub fn get_text_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|block| block.as_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

### ClaudeError Methods

```rust
impl ClaudeError {
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ClaudeError::TimeoutError { .. }
                | ClaudeError::RateLimitError { .. }
                | ClaudeError::HttpError(_)
        )
    }

    /// Get retry delay if applicable
    pub fn should_retry_after(&self) -> Option<std::time::Duration> {
        match self {
            ClaudeError::RateLimitError {
                retry_after: Some(seconds),
                ..
            } => Some(std::time::Duration::from_secs(*seconds)),
            ClaudeError::TimeoutError { .. } => {
                Some(std::time::Duration::from_secs(5))
            }
            _ => None,
        }
    }

    /// Add context to this error
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        // Implementation sets context field
        self
    }

    /// Get the error context if available
    pub fn get_context(&self) -> Option<&ErrorContext> {
        // Implementation returns context field
    }
}
```

## Type Conversions

### From String to ContentBlock

```rust
impl From<String> for ContentBlock {
    fn from(text: String) -> Self {
        ContentBlock::Text { text }
    }
}

impl From<&str> for ContentBlock {
    fn from(text: &str) -> Self {
        ContentBlock::Text { text: text.to_string() }
    }
}
```

### From Legacy Format

```rust
/// Convert legacy string message to ClaudeMessage
pub fn from_legacy_message(role: &str, content: String) -> ClaudeMessage {
    ClaudeMessage {
        role: role.to_string(),
        content: vec![ContentBlock::Text { text: content }],
        thinking: None,
    }
}
```

## Serialization Examples

### JSON Serialization

```rust
let block = ContentBlock::Text { 
    text: "Hello".to_string() 
};
let json = serde_json::to_string(&block)?;
// {"type":"text","text":"Hello"}

let tool_use = ContentBlock::ToolUse {
    id: "123".to_string(),
    name: "read_file".to_string(),
    input: json!({"path": "test.txt"}),
};
let json = serde_json::to_string(&tool_use)?;
// {"type":"tool_use","id":"123","name":"read_file","input":{"path":"test.txt"}}
```

### Deserialization

```rust
let json = r#"{"type":"text","text":"Hello"}"#;
let block: ContentBlock = serde_json::from_str(json)?;

let json = r#"{
    "role": "user",
    "content": [{"type":"text","text":"Hello"}]
}"#;
let message: ClaudeMessage = serde_json::from_str(json)?;
```

## Usage Examples

### Creating Messages

```rust
// Simple text message
let message = ClaudeMessage::user_text("What is the weather?");

// Message with multiple content blocks
let mut message = ClaudeMessage {
    role: "assistant".to_string(),
    content: vec![],
    thinking: None,
};
message.add_content(ContentBlock::Text { 
    text: "I'll check the weather for you.".to_string() 
});
message.add_content(ContentBlock::ToolUse {
    id: "weather_check_1".to_string(),
    name: "get_weather".to_string(),
    input: json!({"location": "San Francisco"}),
});
```

### Processing Responses

```rust
for content_block in &response.content {
    match content_block {
        ContentBlock::Text { text } => {
            println!("Text: {}", text);
        }
        ContentBlock::ToolUse { id, name, input } => {
            println!("Tool request: {} with id {}", name, id);
            let result = execute_tool(name, input).await?;
            // Create tool result block
        }
        ContentBlock::ToolResult { content, is_error, .. } => {
            if is_error.unwrap_or(false) {
                eprintln!("Tool error: {}", content);
            } else {
                println!("Tool result: {}", content);
            }
        }
        ContentBlock::Thinking { content } => {
            // Handle thinking content based on configuration
            if config.show_thinking {
                println!("Thinking: {}", content);
            }
        }
    }
}
```

### Error Handling

```rust
match client.send_message(&conversation, message).await {
    Ok(response) => process_response(response),
    Err(e) => {
        if e.is_retryable() {
            if let Some(duration) = e.should_retry_after() {
                tokio::time::sleep(duration).await;
                // Retry the request
            }
        } else {
            // Handle non-retryable error
            eprintln!("Error: {}", e);
            if let Some(context) = e.get_context() {
                eprintln!("Operation: {}", context.operation);
                eprintln!("Timestamp: {}", context.timestamp);
            }
        }
    }
}
```

## Validation Rules

### Text Blocks
- Maximum length: Configured via `MESSAGE_MAX_CHARS`
- No empty text allowed
- UTF-8 encoding required

### ToolUse Blocks
- ID must be non-empty and unique
- Name must match a registered tool
- Input must be valid JSON

### ToolResult Blocks
- Must reference a valid tool_use_id
- Content length limits apply
- is_error defaults to false if not specified

### Thinking Blocks
- Only included when thinking mode is enabled
- Subject to same length limits as text
- May be filtered in production mode

## See Also

- [Architecture: ContentBlock System](../architecture/content-block-system.md)
- [Tool Integration Guide](../tools/content-block-integration.md)
- [Error Handling Best Practices](../development/error-handling.md)
- [Claude API Documentation](https://docs.anthropic.com/claude/reference/messages)