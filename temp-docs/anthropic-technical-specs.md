# Claude 4 Technical Implementation Specifications

## Complete Rust Type Definitions for Claude 4 Agent Integration

### Core Message Types
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub r#type: String, // "message"
    pub role: String,   // "assistant"
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateParams {
    pub model: String,
    pub messages: Vec<MessageParam>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageParam {
    pub role: String, // "user" | "assistant"
    pub content: ContentBlockParam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}
```

### Content Block System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: Option<String>,
        is_error: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentBlockParam {
    String(String),
    Blocks(Vec<ContentBlockItem>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlockItem {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: Option<String>,
        is_error: Option<bool>,
    },
}
```

### Tool Definition System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: ToolInputSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    pub r#type: String, // "object"
    pub properties: HashMap<String, ToolProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProperty {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ToolProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, ToolProperty>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolChoice {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "tool")]
    Tool { name: String },
}
```

### Streaming Implementation Types
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageStreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: Message },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDelta,
        usage: Usage,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: u32,
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: u32,
        delta: ContentDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<StopReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}
```

### Error Handling System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicError {
    pub r#type: String,
    pub error: ErrorDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub r#type: ErrorType,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    InvalidRequestError,
    AuthenticationError,
    PermissionError,
    NotFoundError,
    RateLimitError,
    ApiError,
    OverloadedError,
}
```

### Enums and Constants
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageSource {
    #[serde(rename = "base64")]
    Base64 {
        media_type: String,
        data: String,
    },
    #[serde(rename = "url")]
    Url { url: String },
}

// Claude 4 Model constants
pub const CLAUDE_4_OPUS_20250514: &str = "claude-4-opus-20250514";
pub const CLAUDE_4_SONNET_20250514: &str = "claude-4-sonnet-20250514";
pub const CLAUDE_4_HAIKU_20250514: &str = "claude-4-haiku-20250514";

// Legacy Claude 3 models (deprecated for new integrations)
pub const CLAUDE_3_5_SONNET_20241022: &str = "claude-3-5-sonnet-20241022";
pub const CLAUDE_3_5_HAIKU_20241022: &str = "claude-3-5-haiku-20241022";
pub const CLAUDE_3_OPUS_20240229: &str = "claude-3-opus-20240229";
```

## HTTP Client Implementation Pattern

### Client Configuration
```rust
use reqwest::Client;
use std::time::Duration;

pub struct AnthropicClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;
            
        Ok(Self {
            client,
            api_key,
            base_url: "https://api.anthropic.com".to_string(),
        })
    }
    
    pub async fn create_message(
        &self,
        params: MessageCreateParams,
    ) -> Result<Message, AnthropicError> {
        let url = format!("{}/v1/messages", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2024-10-22")
            .header("content-type", "application/json")
            .json(&params)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(response.json().await?)
        }
    }
}
```

## Key Implementation Requirements

### 1. Configuration File Structure
```toml
# config.toml
[anthropic]
model = "claude-4-opus-20250514"  # Default to most capable Claude 4 model
max_tokens = 8192  # Claude 4 supports higher token limits
temperature = 0.7
base_url = "https://api.anthropic.com"
timeout_seconds = 120

[tools]
enabled = ["read_file", "write_file", "list_directory"]
max_file_size = 10485760  # 10MB
allowed_extensions = [".txt", ".md", ".rs", ".js", ".py"]

[streaming]
enabled = true
buffer_size = 8192
```

### 2. Tool Registration System
```rust
pub trait AnthropicTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> ToolInputSchema;
    async fn execute(&self, input: serde_json::Value) -> Result<String, String>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn AnthropicTool>>,
}

impl ToolRegistry {
    pub fn register_tool(&mut self, tool: Box<dyn AnthropicTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    pub fn get_tool_definitions(&self) -> Vec<Tool> {
        self.tools.values().map(|tool| Tool {
            name: tool.name().to_string(),
            description: Some(tool.description().to_string()),
            input_schema: tool.input_schema(),
        }).collect()
    }
}
```

## Claude 4 Model Specifications

### Model Capabilities and Performance

**Claude 4 Opus (claude-4-opus-20250514)**
- **Context Window**: 200,000 tokens
- **Max Output**: 8,192 tokens  
- **Capabilities**: Premier model with strongest reasoning, complex task handling, and multimodal processing
- **Best For**: Complex analysis, strategic planning, advanced coding, research synthesis
- **Tool Use**: Full support for all tool types with enhanced reliability
- **Multimodal**: Advanced image and document processing
- **Performance**: ~3x faster than Claude 3 Opus for equivalent quality

**Claude 4 Sonnet (claude-4-sonnet-20250514)**
- **Context Window**: 200,000 tokens
- **Max Output**: 8,192 tokens
- **Capabilities**: Balanced performance and speed, excellent for most use cases
- **Best For**: General chat, content creation, coding assistance, document analysis
- **Tool Use**: Full support with optimized execution speed
- **Multimodal**: High-quality image processing and understanding
- **Performance**: ~2x faster than Claude 3.5 Sonnet with improved accuracy

**Claude 4 Haiku (claude-4-haiku-20250514)**
- **Context Window**: 200,000 tokens  
- **Max Output**: 8,192 tokens
- **Capabilities**: Ultra-fast responses while maintaining high quality
- **Best For**: Quick queries, real-time chat, simple coding tasks, rapid document processing
- **Tool Use**: Full support optimized for speed
- **Multimodal**: Fast image processing with good accuracy
- **Performance**: ~5x faster than Claude 3 Haiku with equivalent or better quality

### Pricing (as of 2025-05-14)

**Claude 4 Opus**
- Input: $15.00 per million tokens
- Output: $75.00 per million tokens
- Tool Use: Standard rates apply to tool execution tokens

**Claude 4 Sonnet**
- Input: $3.00 per million tokens
- Output: $15.00 per million tokens
- Tool Use: Standard rates apply to tool execution tokens

**Claude 4 Haiku**
- Input: $0.25 per million tokens
- Output: $1.25 per million tokens
- Tool Use: Standard rates apply to tool execution tokens

### Enhanced Tool Use Capabilities

Claude 4 models include significant improvements to tool use:

**Reliability Improvements**
- 40% reduction in tool hallucination
- Better parameter validation and error handling
- Improved tool selection accuracy

**Performance Enhancements**  
- Parallel tool execution support
- Reduced latency for tool calls
- Better handling of complex tool chains

**New Tool Features**
- Enhanced file processing tools (supports larger files)
- Improved code analysis and generation tools
- Better integration with external APIs

### API Compatibility

Claude 4 maintains full backward compatibility with Claude 3 API patterns:
- Same HTTP endpoints and request/response formats
- Identical tool use specification format
- Compatible authentication and headers
- Same streaming implementation

**Recommended Migration Path**:
1. Update model constants to Claude 4 variants
2. Increase max_tokens limits to take advantage of higher output capacity
3. Optionally adjust timeout values for faster models (Haiku/Sonnet)
4. Test tool use workflows for improved reliability

### Integration Considerations for 1500 LOC Approach

**Model Selection Strategy**:
```rust
// Recommended model selection based on use case
pub fn select_optimal_model(task_complexity: TaskComplexity) -> &'static str {
    match task_complexity {
        TaskComplexity::Simple => CLAUDE_4_HAIKU_20250514,
        TaskComplexity::Moderate => CLAUDE_4_SONNET_20250514,
        TaskComplexity::Complex => CLAUDE_4_OPUS_20250514,
    }
}

pub enum TaskComplexity {
    Simple,   // File operations, simple queries
    Moderate, // Code analysis, document processing  
    Complex,  // Multi-step reasoning, complex tool chains
}
```

**Configuration for Different Use Cases**:
```toml
# Development/Testing Configuration (cost-optimized)
[anthropic.development]
model = "claude-4-haiku-20250514"
max_tokens = 4096

# Production Configuration (balanced)
[anthropic.production] 
model = "claude-4-sonnet-20250514"
max_tokens = 6144

# Complex Analysis Configuration (performance-optimized)
[anthropic.analysis]
model = "claude-4-opus-20250514"
max_tokens = 8192
```

**Error Handling for Model-Specific Features**:
```rust
impl AnthropicClient {
    pub async fn create_message_with_fallback(
        &self,
        mut params: MessageCreateParams,
    ) -> Result<Message, AnthropicError> {
        // Try primary model first
        match self.create_message(params.clone()).await {
            Ok(response) => Ok(response),
            Err(e) if e.is_model_overloaded() => {
                // Fallback to faster model if primary is overloaded
                params.model = self.get_fallback_model(&params.model);
                self.create_message(params).await
            }
            Err(e) => Err(e),
        }
    }
    
    fn get_fallback_model(&self, primary: &str) -> String {
        match primary {
            CLAUDE_4_OPUS_20250514 => CLAUDE_4_SONNET_20250514.to_string(),
            CLAUDE_4_SONNET_20250514 => CLAUDE_4_HAIKU_20250514.to_string(),
            _ => primary.to_string(),
        }
    }
}
```

## References
- [Anthropic API Documentation](https://docs.anthropic.com/en/api)
- [Claude 4 Release Notes](https://docs.anthropic.com/en/docs/claude-4-overview)
- [Claude 4 Migration Guide](https://docs.anthropic.com/en/docs/claude-4-migration)
- [TypeScript SDK Types](https://github.com/anthropics/anthropic-sdk-typescript/blob/main/api.md)
- [Tool Use Implementation](https://docs.anthropic.com/en/docs/agents-and-tools/tool-use/implement-tool-use)
- [Claude 4 Tool Use Enhancements](https://docs.anthropic.com/en/docs/claude-4-tools)
- Analysis updated for Claude 4: 2025-06-21