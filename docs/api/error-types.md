# Error Handling API Reference

This document provides comprehensive API reference for the enhanced error handling system implemented in issue [1.2], featuring `thiserror` integration, circuit breaker patterns, and structured telemetry.

## Core Types

### ClaudeError

Main error enum using `thiserror` for ergonomic error handling.

```rust
#[derive(Debug, Error)]
pub enum ClaudeError {
    // HTTP/Network Errors
    #[error("HTTP request failed: {0}")]
    HttpError(reqwest::Error),

    #[error("JSON processing failed: {0}")]
    JsonError(serde_json::Error),

    // API Errors
    #[error("API error ({status}): {message}")]
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
        param: Option<String>,
        context: Option<ErrorContext>,
    },

    // Model Errors
    #[error("Model error ({model}): {message}")]
    ModelError {
        model: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Tool Errors
    #[error("Tool error ({tool_name}): {message}")]
    ToolError {
        tool_name: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Configuration Errors
    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
        context: Option<ErrorContext>,
    },

    // Validation Errors
    #[error("Validation error ({field}): {message}")]
    ValidationError {
        field: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // System Errors
    #[error("Request timed out after {duration:?}")]
    TimeoutError {
        duration: std::time::Duration,
        context: Option<ErrorContext>,
    },

    #[error("Rate limit exceeded")]
    RateLimitError {
        retry_after: Option<u64>,
        context: Option<ErrorContext>,
    },

    // Content Block Errors
    #[error("Content block error ({block_type}): {message}")]
    ContentBlockError {
        block_type: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Streaming Errors (future-ready)
    #[error("Streaming error: {message}")]
    StreamingError {
        message: String,
        context: Option<ErrorContext>,
    },
}
```

#### Methods

##### `with_context(mut self, context: ErrorContext) -> Self`
Adds error context to any error variant that supports it.

```rust
let error = ClaudeError::ApiError { /* ... */ }
    .with_context(ErrorContext::new("send_message"));
```

##### `is_retryable(&self) -> bool`
Determines if the error is suitable for retry logic.

```rust
if error.is_retryable() {
    // Retry the operation
}
```

**Retryable errors:**
- `TimeoutError`
- `RateLimitError`
- `HttpError`

##### `should_retry_after(&self) -> Option<Duration>`
Returns suggested retry delay for specific error types.

```rust
if let Some(delay) = error.should_retry_after() {
    tokio::time::sleep(delay).await;
}
```

##### `get_context(&self) -> Option<&ErrorContext>`
Retrieves error context if available.

```rust
if let Some(context) = error.get_context() {
    println!("Operation: {}", context.operation);
}
```

### ErrorContext

Rich contextual information for error debugging and telemetry.

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

#### Methods

##### `new(operation: impl Into<String>) -> Self`
Creates new error context with operation name.

```rust
let context = ErrorContext::new("send_message_to_claude");
```

##### `with_message_id(mut self, message_id: impl Into<String>) -> Self`
Adds message ID to context.

```rust
let context = ErrorContext::new("send_message")
    .with_message_id("msg_123");
```

##### `with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self`
Adds tool use ID to context.

```rust
let context = ErrorContext::new("execute_tool")
    .with_tool_use_id("tool_456");
```

##### `with_retry_count(mut self, retry_count: u32) -> Self`
Sets retry count for context.

```rust
let context = ErrorContext::new("api_call")
    .with_retry_count(2);
```

##### `add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self`
Adds custom metadata to context.

```rust
let context = ErrorContext::new("operation")
    .add_metadata("user_id", "user_123")
    .add_metadata("session_id", "session_456");
```

##### `log_error(&self, error: &ClaudeError)`
Logs error with structured formatting.

```rust
context.log_error(&error);
// Output: 🚨 [ERROR] API error (500): Internal server error | type=api_error | operation=send_message | retry_count=1 | timestamp=2025-01-23 10:30:45 UTC
```

##### `log_success(&self, duration: Option<Duration>)`
Logs successful operation completion.

```rust
context.log_success(Some(duration));
// Output: ✅ [SUCCESS] operation=send_message | retry_count=1 | duration=245ms | timestamp=2025-01-23 10:30:45 UTC
```

##### `log_retry(&self, attempt: u32, delay: Duration)`
Logs retry attempt with delay information.

```rust
context.log_retry(2, Duration::from_millis(1000));
// Output: 🔄 [RETRY] operation=send_message | attempt=2 | delay=1000ms | timestamp=2025-01-23 10:30:45 UTC
```

### ErrorHandler

Main component for handling errors with retry logic and circuit breaker.

```rust
#[derive(Debug)]
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    circuit_breaker: Option<CircuitBreaker>,
    telemetry: ErrorTelemetry,
}
```

#### Methods

##### `new() -> Self`
Creates error handler with default configuration.

```rust
let handler = ErrorHandler::new();
```

##### `with_config(config: ErrorHandlerConfig) -> Self`
Creates error handler with custom configuration.

```rust
let config = ErrorHandlerConfig {
    max_retries: 5,
    base_delay: Duration::from_millis(1000),
    ..Default::default()
};
let handler = ErrorHandler::with_config(config);
```

##### `handle_with_retry<F, T, Fut>(&self, operation: F) -> ClaudeResult<T>`
Executes operation with retry logic and circuit breaker protection.

```rust
let result = handler.handle_with_retry(|| async {
    claude_client.send_message("Hello").await
}).await?;
```

**Features:**
- Exponential backoff with jitter
- Circuit breaker protection
- Structured logging
- Telemetry tracking
- Error-specific retry policies

##### `telemetry(&self) -> &ErrorTelemetry`
Returns reference to telemetry data.

```rust
let telemetry = handler.telemetry();
let total_errors = telemetry.total_errors.load(Ordering::Relaxed);
```

##### `print_telemetry_summary(&self)`
Prints comprehensive telemetry summary.

```rust
handler.print_telemetry_summary();
// Output:
// 📊 [TELEMETRY] Error Handler Summary:
//    • Total Errors: 15
//    • Total Retries: 8
//    • Successful Operations: 142
//    • Circuit Breaker Triggers: 2
//    • Success Rate: 90.45%
```

### ErrorHandlerConfig

Configuration for error handling behavior.

```rust
#[derive(Debug, Clone)]
pub struct ErrorHandlerConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
    pub circuit_breaker_enabled: bool,
    pub failure_threshold: u32,
    pub circuit_timeout: Duration,
}
```

#### Default Values
```rust
ErrorHandlerConfig {
    max_retries: 3,
    base_delay: Duration::from_millis(500),
    max_delay: Duration::from_secs(30),
    backoff_multiplier: 2.0,
    jitter: true,
    circuit_breaker_enabled: true,
    failure_threshold: 5,
    circuit_timeout: Duration::from_secs(60),
}
```

### CircuitBreaker

Implements circuit breaker pattern for failure protection.

```rust
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    last_failure_time: Mutex<Option<Instant>>,
    failure_threshold: u32,
    timeout: Duration,
}
```

#### Methods

##### `new(failure_threshold: u32, timeout: Duration) -> Self`
Creates new circuit breaker with specified parameters.

```rust
let breaker = CircuitBreaker::new(5, Duration::from_secs(60));
```

##### `is_open(&self) -> bool`
Checks if circuit breaker is currently open.

```rust
if breaker.is_open() {
    return Err(ClaudeError::ConfigError { /* ... */ });
}
```

##### `record_success(&self)`
Records successful operation, potentially closing the circuit.

```rust
breaker.record_success();
```

##### `record_failure(&self)`
Records failed operation, potentially opening the circuit.

```rust
breaker.record_failure();
```

### ErrorTelemetry

Tracks comprehensive error metrics for monitoring.

```rust
#[derive(Debug, Clone)]
pub struct ErrorTelemetry {
    pub total_errors: Arc<AtomicU64>,
    pub errors_by_type: Arc<Mutex<HashMap<String, u64>>>,
    pub total_retries: Arc<AtomicU64>,
    pub successful_operations: Arc<AtomicU64>,
    pub circuit_breaker_triggers: Arc<AtomicU64>,
}
```

#### Methods

##### `record_error(&self, error_type: &str)`
Records error occurrence by type.

```rust
telemetry.record_error("api_error");
```

##### `record_retry(&self)`
Records retry attempt.

```rust
telemetry.record_retry();
```

##### `record_success(&self)`
Records successful operation.

```rust
telemetry.record_success();
```

##### `record_circuit_breaker_trigger(&self)`
Records circuit breaker activation.

```rust
telemetry.record_circuit_breaker_trigger();
```

##### `print_summary(&self)`
Prints detailed telemetry summary.

```rust
telemetry.print_summary();
```

## Type Aliases

### ClaudeResult

Standard result type for Claude operations.

```rust
pub type ClaudeResult<T> = Result<T, ClaudeError>;
```

## Error Conversion Traits

The system provides automatic conversions from common error types:

### From reqwest::Error
```rust
impl From<reqwest::Error> for ClaudeError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            ClaudeError::TimeoutError {
                duration: Duration::from_secs(120),
                context: None,
            }
        } else {
            ClaudeError::HttpError(error)
        }
    }
}
```

### From serde_json::Error
```rust
impl From<serde_json::Error> for ClaudeError {
    fn from(error: serde_json::Error) -> Self {
        ClaudeError::JsonError(error)
    }
}
```

## Usage Examples

### Basic Error Handling
```rust
use claude::error::{ClaudeError, ClaudeResult, ErrorContext};

fn example_operation() -> ClaudeResult<String> {
    let context = ErrorContext::new("example_operation")
        .add_metadata("user_id", "123");

    // Your operation here
    Err(ClaudeError::ApiError {
        status: 500,
        message: "Internal server error".to_string(),
        error_type: Some("server_error".to_string()),
        param: None,
        context: Some(context),
    })
}
```

### Error Handler Usage
```rust
use claude::error::{ErrorHandler, ErrorHandlerConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> ClaudeResult<()> {
    let config = ErrorHandlerConfig {
        max_retries: 3,
        base_delay: Duration::from_millis(500),
        circuit_breaker_enabled: true,
        ..Default::default()
    };

    let handler = ErrorHandler::with_config(config);

    let result = handler.handle_with_retry(|| async {
        // Your async operation here
        claude_client.send_message("Hello").await
    }).await?;

    // Print telemetry after operations
    handler.print_telemetry_summary();

    Ok(())
}
```

### Custom Error Context
```rust
let context = ErrorContext::new("complex_operation")
    .with_message_id("msg_abc123")
    .with_tool_use_id("tool_def456")
    .add_metadata("operation_type", "batch_processing")
    .add_metadata("batch_size", "100");

let error = ClaudeError::ToolError {
    tool_name: "file_processor".to_string(),
    message: "Processing failed".to_string(),
    context: Some(context),
};

// Log the error with rich context
if let Some(ctx) = error.get_context() {
    ctx.log_error(&error);
}
```

### Circuit Breaker Integration
```rust
let handler = ErrorHandler::new();

// This will automatically use circuit breaker
let result = handler.handle_with_retry(|| async {
    // If this fails repeatedly, circuit breaker will open
    unreliable_service.call().await
}).await;

match result {
    Ok(response) => println!("Success: {}", response),
    Err(ClaudeError::ConfigError { message, .. }) if message.contains("Circuit breaker") => {
        println!("Service temporarily unavailable due to repeated failures");
    }
    Err(other) => println!("Other error: {}", other),
}
```

## Integration Points

### ClaudeClient Integration
```rust
impl ClaudeClient {
    pub async fn send_message_with_retry(&self, message: &str) -> ClaudeResult<String> {
        let handler = ErrorHandler::new();
        
        handler.handle_with_retry(|| async {
            self.send_message(message).await
        }).await
    }
}
```

### Tool Execution Integration
```rust
#[async_trait]
impl AgentTool for SomeTool {
    async fn execute(&self, input: Value) -> Result<String, ClaudeError> {
        let handler = ErrorHandler::new();
        
        handler.handle_with_retry(|| async {
            // Tool execution logic
            self.internal_execute(&input).await
        }).await
    }
}
```

## Related Documentation

- [Architecture: Error Handling System](../architecture/error-handling-system.md)
- [Integration Guide: Error Handling](../tools/error-handling-integration.md)
- [Model Registry API](./model-registry-types.md)
- [Configuration System](./configuration-types.md)