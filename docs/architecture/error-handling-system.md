# Enhanced Error Handling Framework

This document describes the enhanced error handling framework implemented in issue [1.2], which provides comprehensive error taxonomy, circuit breaker patterns, retry logic, and structured telemetry.

## System Overview

The enhanced error handling system provides a robust foundation for handling failures across all components of the application. It includes:

- **Comprehensive Error Taxonomy** - Using `thiserror` for ergonomic error definitions
- **Circuit Breaker Pattern** - Preventing cascading failures
- **Exponential Backoff with Jitter** - Intelligent retry mechanisms  
- **Structured Logging** - Rich context for debugging and monitoring
- **Error Telemetry** - Metrics for operational monitoring

## Architecture Components

### 1. Error Types (`ClaudeError`)

The system uses a comprehensive error taxonomy with `thiserror` derive macros:

```rust
#[derive(Debug, Error)]
pub enum ClaudeError {
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String, /* ... */ },
    
    #[error("Model error ({model}): {message}")]
    ModelError { model: String, message: String, /* ... */ },
    
    // ... other variants
}
```

**Key Features:**
- Automatic `Display` implementation via `thiserror`
- Rich context information in `ErrorContext`
- Structured error categorization
- Error-specific retry policies

### 2. Circuit Breaker Pattern

Prevents cascading failures by temporarily disabling operations when failure rate exceeds threshold:

```rust
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    last_failure_time: Mutex<Option<Instant>>,
    failure_threshold: u32,
    timeout: Duration,
}
```

**States:**
- **Closed** - Normal operation, requests pass through
- **Open** - Failure threshold exceeded, requests rejected immediately
- **Half-Open** - Timeout elapsed, testing if service recovered

### 3. Error Handler with Retry Logic

Provides intelligent retry mechanisms with exponential backoff:

```rust
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    circuit_breaker: Option<CircuitBreaker>,
    telemetry: ErrorTelemetry,
}
```

**Features:**
- Configurable retry counts and delays
- Exponential backoff with jitter
- Error-specific retry policies
- Circuit breaker integration
- Comprehensive telemetry tracking

### 4. Structured Logging System

Rich contextual logging for debugging and monitoring:

```rust
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: DateTime<Utc>,
    pub message_id: Option<String>,
    pub tool_use_id: Option<String>,
    pub retry_count: u32,
    pub metadata: HashMap<String, String>,
}
```

**Log Formats:**
- **Errors**: `🚨 [ERROR] {error} | type={type} | operation={op} | retry_count={count} | timestamp={time}`
- **Success**: `✅ [SUCCESS] operation={op} | duration={duration} | timestamp={time}`
- **Retries**: `🔄 [RETRY] operation={op} | attempt={num} | delay={delay} | timestamp={time}`

### 5. Error Telemetry System

Comprehensive metrics for operational monitoring:

```rust
pub struct ErrorTelemetry {
    pub total_errors: Arc<AtomicU64>,
    pub errors_by_type: Arc<Mutex<HashMap<String, u64>>>,
    pub total_retries: Arc<AtomicU64>,
    pub successful_operations: Arc<AtomicU64>,
    pub circuit_breaker_triggers: Arc<AtomicU64>,
}
```

**Metrics Tracked:**
- Total error count by type
- Retry attempt counts
- Success/failure ratios
- Circuit breaker activation frequency
- Operation duration statistics

## Configuration Options

### ErrorHandlerConfig

```rust
pub struct ErrorHandlerConfig {
    pub max_retries: u32,                    // Default: 3
    pub base_delay: Duration,                // Default: 500ms
    pub max_delay: Duration,                 // Default: 30s
    pub backoff_multiplier: f64,             // Default: 2.0
    pub jitter: bool,                        // Default: true
    pub circuit_breaker_enabled: bool,       // Default: true
    pub failure_threshold: u32,              // Default: 5
    pub circuit_timeout: Duration,           // Default: 60s
}
```

### Retry Policies

Different error types have different retry behaviors:

- **Retryable**: `TimeoutError`, `RateLimitError`, `HttpError`
- **Non-retryable**: `ValidationError`, `ConfigError`, `ContentBlockError`
- **Error-specific delays**: Rate limit errors respect `Retry-After` headers

## Integration Patterns

### Basic Error Handling

```rust
use claude::error::{ErrorHandler, ErrorContext};

let handler = ErrorHandler::new();
let result = handler.handle_with_retry(|| async {
    // Your operation here
    claude_client.send_message(message).await
}).await?;
```

### Custom Configuration

```rust
let config = ErrorHandlerConfig {
    max_retries: 5,
    base_delay: Duration::from_millis(1000),
    circuit_breaker_enabled: true,
    ..Default::default()
};

let handler = ErrorHandler::with_config(config);
```

### Telemetry Monitoring

```rust
// Print telemetry summary
handler.print_telemetry_summary();

// Access specific metrics
let telemetry = handler.telemetry();
let total_errors = telemetry.total_errors.load(Ordering::Relaxed);
```

## Security Considerations

1. **Error Information Exposure** - Error messages are sanitized to prevent sensitive data leakage
2. **Resource Protection** - Circuit breaker prevents resource exhaustion during outages
3. **Rate Limiting** - Respects upstream rate limits to prevent service degradation
4. **Timeout Protection** - Prevents hanging operations that could exhaust connections

## Performance Characteristics

- **Memory Usage** - Minimal overhead with atomic counters and efficient HashMap storage
- **CPU Impact** - Jitter calculation uses fast hash-based pseudo-random generation
- **Thread Safety** - All components are thread-safe with minimal lock contention
- **Latency** - Retry delays are optimized for fast recovery without overwhelming upstream services

## Future Extensibility

The architecture supports future enhancements:

- **Custom Retry Policies** - Per-operation retry strategies
- **Distributed Circuit Breakers** - Shared state across multiple instances
- **External Telemetry** - Integration with Prometheus, OpenTelemetry, etc.
- **Advanced Jitter** - Different jitter algorithms for specific scenarios
- **Error Aggregation** - Batch error reporting for reduced logging overhead

## Related Documentation

- [API Reference: Error Types](../api/error-types.md)
- [Integration Guide: Error Handling](../tools/error-handling-integration.md)
- [Model Registry System](./model-registry-system.md)
- [Configuration System](./configuration-system.md)

## Implementation Details

See the source code in:
- `src-tauri/src/claude/error.rs` - Core error handling implementation
- `src-tauri/src/claude/model_registry.rs` - Model-specific error handling
- `src-tauri/src/config/` - Configuration integration