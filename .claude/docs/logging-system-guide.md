# Structured Logging System Guide

## Overview

The rustic-agent implements a comprehensive structured logging system designed for production use with security-first principles, telemetry collection, and performance monitoring.

## Quick Start

### Basic Usage

```rust
// Use the logging macros for immediate logging
use crate::{log_error, log_warn, log_info, log_debug};

// Simple logging
log_info!("startup", "Application started successfully");

// With context
log_error!("api_call", "Request failed", {
    let mut context = std::collections::HashMap::new();
    context.insert("status_code".to_string(), "500".to_string());
    context.insert("endpoint".to_string(), "/api/test".to_string());
    context
});
```

### Specialized Logging

```rust
// Tool execution tracking
log_tool_execution!("read_file", true, Duration::from_millis(150));

// Claude API call monitoring
log_claude_api!("claude-4-sonnet", 1500, 0.0045, Duration::from_secs(2));

// Cost monitoring
log_cost!("document_analysis", 0.012, 2400);
```

## Architecture

### Core Components

1. **Logger**: Main logging interface with level filtering
2. **LogEntry**: Structured log data with metadata
3. **SecuritySanitizer**: Automatic sensitive data redaction
4. **TelemetryCollector**: Metrics aggregation and reporting

### Security Features

- **Automatic API Key Redaction**: `sk-ant-*` patterns → `[API_KEY_REDACTED]`
- **Path Sanitization**: `/home/user/` → `/[USER_DIR_REDACTED]`
- **Metadata Filtering**: Keys containing "key", "secret", "token", "password" → `[REDACTED]`
- **Message Truncation**: Long messages automatically truncated with indicator

## Configuration

### Log Levels

```rust
pub enum LogLevel {
    Trace = 0,  // Detailed debugging information
    Debug = 1,  // Development debugging
    Info = 2,   // General information (default)
    Warn = 3,   // Warning conditions
    Error = 4,  // Error conditions
}
```

### Environment Variables

```bash
# Set log level (default: INFO)
export LOG_LEVEL=DEBUG

# Initialize in main.rs
use rustic_agent::logging::{init_logger, LogLevel};

init_logger(LogLevel::from_env())?;
```

## Usage Patterns

### Error Handling Integration

```rust
use crate::claude::error::{ClaudeError, ErrorContext};

impl ErrorContext {
    pub fn log_error(&self, error: &ClaudeError) {
        crate::log_error!(&self.operation, &error.to_string(), {
            let mut context = std::collections::HashMap::new();
            context.insert("error_type".to_string(), "api_error".to_string());
            context.insert("retry_count".to_string(), self.retry_count.to_string());
            context
        });
    }
}
```

### Tool Execution Monitoring

```rust
async fn execute_tool(&self, tool_name: &str, input: Value) -> Result<String> {
    let start_time = Instant::now();
    
    match self.tools.get(tool_name) {
        Some(tool) => {
            match tool.execute(input).await {
                Ok(result) => {
                    let duration = start_time.elapsed();
                    crate::log_tool_execution!(tool_name, true, duration);
                    Ok(result)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    crate::log_tool_execution!(tool_name, false, duration);
                    Err(e)
                }
            }
        }
        None => {
            crate::log_error!("tool_execution", "Tool not found", {
                let mut context = std::collections::HashMap::new();
                context.insert("tool_name".to_string(), tool_name.to_string());
                context
            });
            Err(anyhow::anyhow!("Tool not found: {}", tool_name))
        }
    }
}
```

### File Watcher Integration

```rust
// Replace direct println!/eprintln! calls
// Before:
println!("👁️ Starting to watch directory: {}", path.display());

// After:
crate::log_info!(
    "file_watcher", 
    &format!("Starting to watch directory: {}", path.display())
);
```

## Telemetry and Monitoring

### Automatic Metrics Collection

The logging system automatically collects:

- **Log Metrics**: Total logs, error rates, warnings by level
- **Tool Metrics**: Execution counts, success rates, average duration
- **API Metrics**: Call counts, token usage, costs, response times
- **Cost Tracking**: Total costs, operation costs, token consumption

### Accessing Telemetry Data

```rust
use rustic_agent::logging::logger;

// Get current metrics
let telemetry = logger().get_telemetry();
let log_metrics = telemetry.get_log_metrics();
let tool_metrics = telemetry.get_tool_metrics();

// Generate comprehensive report
let report = telemetry.get_summary_report();
println!("{}", report.format_summary());
```

### Sample Telemetry Output

```
📊 Telemetry Report - Generated at 2025-01-15 14:30:22 UTC

📝 Log Metrics:
  Total logs: 1,247
  Errors: 23 (1.8%)
  Warnings: 45
  Info: 1,179

🛠️ Tool Metrics:
  read_file: 342 executions, 99.4% success, avg 89ms
  write_file: 156 executions, 98.7% success, avg 142ms
  list_directory: 78 executions, 100.0% success, avg 45ms

🤖 API Metrics:
  claude-4-sonnet: 67 calls, 1,847 avg tokens, $0.0089 avg cost, 1,234ms avg

💰 Total Cost: $0.596400
```

## Best Practices

### 1. Use Appropriate Log Levels

```rust
// ✅ Good
log_error!("api_call", "Authentication failed");
log_warn!("rate_limit", "Approaching rate limit threshold");
log_info!("startup", "Service initialized successfully");
log_debug!("validation", "Input validation passed");

// ❌ Avoid
log_error!("debug", "Debugging variable state"); // Wrong level
log_info!("panic", "System crashed"); // Wrong level
```

### 2. Provide Meaningful Context

```rust
// ✅ Good - Rich context
log_error!("file_operation", "Failed to read configuration file", {
    let mut context = std::collections::HashMap::new();
    context.insert("file_path".to_string(), "/etc/config.toml".to_string());
    context.insert("permission".to_string(), "denied".to_string());
    context.insert("operation".to_string(), "read".to_string());
    context
});

// ❌ Poor - No context
log_error!("error", "Something failed");
```

### 3. Use Operation Categories

Consistent operation naming helps with log analysis:

- `startup` / `shutdown` - Application lifecycle
- `api_call` - External API interactions
- `tool_execution` - Tool operations
- `file_watcher` - File system monitoring
- `validation` - Input/data validation
- `authentication` - Auth operations
- `rate_limiting` - Rate limiting events

### 4. Security Considerations

```rust
// ✅ Automatic sanitization
log_info!("auth", "User authenticated with key sk-ant-12345"); 
// Logs: "User authenticated with key [API_KEY_REDACTED]"

// ✅ Safe metadata
let mut context = std::collections::HashMap::new();
context.insert("user_id".to_string(), "user123".to_string()); // Safe
context.insert("api_key".to_string(), "secret"); // Auto-redacted to [REDACTED]
```

## Performance Considerations

### 1. Level-Based Filtering

The logger checks levels before formatting expensive strings:

```rust
// ✅ Efficient - only formats if debug enabled
log_debug!("processing", &format!("Processing {} items", expensive_count()));

// ❌ Inefficient - always formats
let msg = format!("Processing {} items", expensive_count());
log_debug!("processing", &msg);
```

### 2. Lazy Evaluation

```rust
// ✅ Use closures for expensive operations in debug
if logger().should_log(LogLevel::Debug) {
    let debug_info = expensive_debug_calculation();
    log_debug!("analysis", &format!("Debug info: {:?}", debug_info));
}
```

### 3. Batch Operations

For high-frequency operations, consider batching:

```rust
// For frequent tool executions, telemetry batches automatically
// Individual log calls are optimized with RwLock
```

## Migration Guide

### From Direct Console Output

```rust
// Before
println!("✅ [SUCCESS] operation={} | duration={}ms", op, duration);
eprintln!("🚨 [ERROR] {}", error_msg);

// After
log_info!(op, "Operation completed successfully", {
    let mut context = std::collections::HashMap::new();
    context.insert("duration_ms".to_string(), duration.to_string());
    context
});

log_error!(op, &error_msg);
```

### From Custom Logging

```rust
// Before
struct CustomLogger {
    level: String,
}

impl CustomLogger {
    fn log(&self, message: &str) {
        println!("[{}] {}", self.level, message);
    }
}

// After
use crate::logging::{init_logger, LogLevel};

// Initialize once in main
init_logger(LogLevel::Info)?;

// Use throughout application
log_info!("operation", message);
```

## Troubleshooting

### Common Issues

1. **Logger Not Initialized**
   ```rust
   // Error: Logger not initialized
   // Solution: Call init_logger() in main.rs
   init_logger(LogLevel::Info)?;
   ```

2. **Sensitive Data in Logs**
   ```rust
   // Automatic sanitization should handle this
   // Check: SecuritySanitizer patterns may need updates
   ```

3. **Performance Issues**
   ```rust
   // Reduce log level in production
   init_logger(LogLevel::Warn)?; // Only warnings and errors
   ```

4. **Missing Context**
   ```rust
   // Always provide operation context
   log_error!("specific_operation", "Error details", context);
   ```

### Debug Mode

```rust
// Enable detailed logging for development
export LOG_LEVEL=DEBUG

// In code
log_debug!("detailed_operation", "Step-by-step progress", {
    let mut context = std::collections::HashMap::new();
    context.insert("step".to_string(), "1".to_string());
    context.insert("state".to_string(), format!("{:?}", current_state));
    context
});
```

## Integration Examples

### With Error Handling

```rust
use crate::claude::error::{ClaudeError, ErrorContext};

fn process_request() -> Result<(), ClaudeError> {
    let context = ErrorContext::new("process_request");
    
    match risky_operation() {
        Ok(result) => {
            context.log_success(Some(Duration::from_millis(100)));
            Ok(result)
        }
        Err(e) => {
            let error = ClaudeError::ApiError {
                status: 500,
                message: e.to_string(),
                error_type: None,
                param: None,
                context: Some(context),
            };
            
            context.log_error(&error);
            Err(error)
        }
    }
}
```

### With Retry Logic

```rust
async fn retry_operation() -> Result<String> {
    let mut attempt = 0;
    let max_retries = 3;
    
    loop {
        attempt += 1;
        
        log_debug!("retry_operation", "Attempting operation", {
            let mut context = std::collections::HashMap::new();
            context.insert("attempt".to_string(), attempt.to_string());
            context.insert("max_retries".to_string(), max_retries.to_string());
            context
        });
        
        match perform_operation().await {
            Ok(result) => {
                log_info!("retry_operation", "Operation succeeded", {
                    let mut context = std::collections::HashMap::new();
                    context.insert("attempts_used".to_string(), attempt.to_string());
                    context
                });
                return Ok(result);
            }
            Err(e) if attempt >= max_retries => {
                log_error!("retry_operation", "Operation failed after all retries", {
                    let mut context = std::collections::HashMap::new();
                    context.insert("total_attempts".to_string(), attempt.to_string());
                    context.insert("final_error".to_string(), e.to_string());
                    context
                });
                return Err(e);
            }
            Err(e) => {
                log_warn!("retry_operation", "Operation failed, retrying", {
                    let mut context = std::collections::HashMap::new();
                    context.insert("attempt".to_string(), attempt.to_string());
                    context.insert("error".to_string(), e.to_string());
                    context
                });
                
                tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
            }
        }
    }
}
```

This logging system provides comprehensive observability while maintaining security and performance, making it suitable for production deployment of the rustic-agent.