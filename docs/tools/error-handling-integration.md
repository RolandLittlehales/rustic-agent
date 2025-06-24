# Error Handling Integration Guide

This guide provides practical examples and best practices for integrating the enhanced error handling framework and model registry system into your application components.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Error Handling Patterns](#error-handling-patterns)
3. [Model Selection Integration](#model-selection-integration)
4. [Circuit Breaker Usage](#circuit-breaker-usage)
5. [Telemetry and Monitoring](#telemetry-and-monitoring)
6. [Testing Strategies](#testing-strategies)
7. [Performance Considerations](#performance-considerations)
8. [Common Pitfalls](#common-pitfalls)

## Quick Start

### Basic Setup

```rust
use claude::error::{ErrorHandler, ErrorHandlerConfig, ErrorContext, ClaudeResult};
use claude::model_registry::{ModelRegistry, ModelSelectionCriteria, TaskComplexity};
use std::time::Duration;

// Create error handler with default settings
let error_handler = ErrorHandler::new();

// Create model registry
let model_registry = ModelRegistry::new();

// Execute operation with retry logic
let result = error_handler.handle_with_retry(|| async {
    // Your operation here
    your_claude_operation().await
}).await?;
```

### Custom Configuration

```rust
// Configure error handling behavior
let config = ErrorHandlerConfig {
    max_retries: 5,
    base_delay: Duration::from_millis(1000),
    max_delay: Duration::from_secs(60),
    backoff_multiplier: 1.5,
    jitter: true,
    circuit_breaker_enabled: true,
    failure_threshold: 3,
    circuit_timeout: Duration::from_secs(30),
};

let error_handler = ErrorHandler::with_config(config);
```

## Error Handling Patterns

### 1. API Client Integration

```rust
use claude::error::{ClaudeError, ClaudeResult, ErrorContext};

pub struct ClaudeClient {
    error_handler: ErrorHandler,
    model_registry: ModelRegistry,
    // ... other fields
}

impl ClaudeClient {
    pub async fn send_message(&self, message: &str) -> ClaudeResult<String> {
        let context = ErrorContext::new("send_message")
            .add_metadata("message_length", &message.len().to_string());

        self.error_handler.handle_with_retry(|| async {
            self.internal_send_message(message, context.clone()).await
        }).await
    }

    async fn internal_send_message(
        &self, 
        message: &str, 
        context: ErrorContext
    ) -> ClaudeResult<String> {
        let response = reqwest::Client::new()
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let body: ApiResponse = response.json().await?;
                Ok(body.content)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok());

                Err(ClaudeError::RateLimitError {
                    retry_after,
                    context: Some(context),
                })
            }
            status => {
                Err(ClaudeError::ApiError {
                    status: status.as_u16(),
                    message: format!("HTTP {}", status),
                    error_type: Some("http_error".to_string()),
                    param: None,
                    context: Some(context),
                })
            }
        }
    }
}
```

### 2. Tool Execution Integration

```rust
use claude::tools::AgentTool;
use async_trait::async_trait;

pub struct FileOperationTool {
    error_handler: ErrorHandler,
}

#[async_trait]
impl AgentTool for FileOperationTool {
    fn name(&self) -> &str {
        "file_operation"
    }

    async fn execute(&self, input: serde_json::Value) -> Result<String, ClaudeError> {
        let context = ErrorContext::new("file_operation_execute")
            .add_metadata("tool_name", self.name());

        self.error_handler.handle_with_retry(|| async {
            self.internal_execute(&input, context.clone()).await
        }).await
    }

    async fn internal_execute(
        &self, 
        input: &serde_json::Value,
        context: ErrorContext
    ) -> ClaudeResult<String> {
        let path = input["path"].as_str()
            .ok_or_else(|| ClaudeError::ValidationError {
                field: "path".to_string(),
                message: "Missing required field 'path'".to_string(),
                context: Some(context.clone()),
            })?;

        match std::fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(ClaudeError::ToolError {
                    tool_name: self.name().to_string(),
                    message: format!("File not found: {}", path),
                    context: Some(context),
                })
            }
            Err(e) => {
                Err(ClaudeError::ToolError {
                    tool_name: self.name().to_string(),
                    message: format!("IO error: {}", e),
                    context: Some(context),
                })
            }
        }
    }
}
```

### 3. Configuration Validation

```rust
use claude::{ClaudeConfig, ModelRegistry};

impl ClaudeConfig {
    pub fn validate_with_enhanced_checks(&self) -> ClaudeResult<()> {
        let context = ErrorContext::new("config_validation");

        // Basic validation
        if self.api_key.is_empty() {
            return Err(ClaudeError::ConfigError {
                message: "API key cannot be empty".to_string(),
                context: Some(context),
            });
        }

        // Model validation using registry
        self.model_registry.validate_model(&self.model)
            .map_err(|mut e| {
                if let ClaudeError::ModelError { context: ref mut ctx, .. } = e {
                    *ctx = Some(context.clone());
                }
                e
            })?;

        // Token limit validation
        if let Some(model_info) = self.model_registry.get_model_info(&self.model) {
            if self.max_tokens > model_info.max_tokens {
                return Err(ClaudeError::ValidationError {
                    field: "max_tokens".to_string(),
                    message: format!(
                        "max_tokens ({}) exceeds model limit ({})",
                        self.max_tokens, model_info.max_tokens
                    ),
                    context: Some(context),
                });
            }
        }

        Ok(())
    }
}
```

## Model Selection Integration

### 1. Dynamic Model Selection

```rust
use claude::model_registry::{ModelSelectionCriteria, TaskComplexity, CostPriority, SpeedPriority};

pub struct SmartClaudeClient {
    model_registry: ModelRegistry,
    error_handler: ErrorHandler,
}

impl SmartClaudeClient {
    pub async fn send_message_optimized(
        &self,
        message: &str,
        complexity: TaskComplexity,
    ) -> ClaudeResult<String> {
        // Select optimal model based on task
        let criteria = ModelSelectionCriteria {
            task_complexity: complexity,
            cost_priority: CostPriority::Medium,
            speed_priority: SpeedPriority::Medium,
            thinking_required: complexity == TaskComplexity::Complex,
            tool_use_required: true,
        };

        let selected_model = self.model_registry.select_optimal_model(&criteria);
        
        // Execute with fallback chain
        self.send_with_fallback(message, &selected_model).await
    }

    async fn send_with_fallback(
        &self,
        message: &str,
        primary_model: &str,
    ) -> ClaudeResult<String> {
        let mut models_to_try = vec![primary_model.to_string()];
        
        // Add fallback chain
        if let Some(fallbacks) = self.model_registry.get_fallback_chain(primary_model) {
            models_to_try.extend(fallbacks.clone());
        }

        let mut last_error = None;

        for model in models_to_try {
            let context = ErrorContext::new("send_with_fallback")
                .add_metadata("model", &model)
                .add_metadata("primary_model", primary_model);

            match self.send_message_with_model(message, &model, context).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    if !error.is_retryable() {
                        return Err(error);
                    }
                    last_error = Some(error);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| ClaudeError::ModelError {
            model: primary_model.to_string(),
            message: "All models in fallback chain failed".to_string(),
            context: None,
        }))
    }
}
```

### 2. Cost-Aware Operations

```rust
pub struct CostOptimizedClient {
    model_registry: ModelRegistry,
    daily_cost_limit: f64,
    current_daily_cost: Arc<Mutex<f64>>,
}

impl CostOptimizedClient {
    pub async fn send_message_with_budget(
        &self,
        message: &str,
        max_tokens: u32,
    ) -> ClaudeResult<String> {
        let input_tokens = self.estimate_input_tokens(message);
        let estimated_output_tokens = max_tokens;

        // Check available models within budget
        let available_models = self.get_budget_friendly_models(
            input_tokens,
            estimated_output_tokens,
        ).await?;

        if available_models.is_empty() {
            return Err(ClaudeError::ConfigError {
                message: "No models available within daily budget".to_string(),
                context: Some(ErrorContext::new("budget_check")),
            });
        }

        // Select most capable model within budget
        let criteria = ModelSelectionCriteria {
            task_complexity: TaskComplexity::Moderate,
            cost_priority: CostPriority::High,
            speed_priority: SpeedPriority::Medium,
            thinking_required: false,
            tool_use_required: true,
        };

        for model in available_models {
            if let Ok(response) = self.try_send_with_model(message, &model).await {
                // Track actual cost
                self.record_usage_cost(&model, input_tokens, estimated_output_tokens).await;
                return Ok(response);
            }
        }

        Err(ClaudeError::ConfigError {
            message: "All budget-friendly models failed".to_string(),
            context: None,
        })
    }

    async fn get_budget_friendly_models(
        &self,
        input_tokens: u32,
        output_tokens: u32,
    ) -> ClaudeResult<Vec<String>> {
        let current_cost = *self.current_daily_cost.lock().await;
        let remaining_budget = self.daily_cost_limit - current_cost;

        let mut affordable_models = Vec::new();

        for model in self.model_registry.get_available_models() {
            if let Some(estimated_cost) = self.model_registry.estimate_cost(
                &model.name,
                input_tokens,
                output_tokens,
            ) {
                if estimated_cost <= remaining_budget {
                    affordable_models.push(model.name.clone());
                }
            }
        }

        // Sort by capability (most capable first)
        affordable_models.sort_by(|a, b| {
            let info_a = self.model_registry.get_model_info(a).unwrap();
            let info_b = self.model_registry.get_model_info(b).unwrap();
            info_b.performance_tier.cmp(&info_a.performance_tier)
        });

        Ok(affordable_models)
    }
}
```

## Circuit Breaker Usage

### 1. Service-Specific Circuit Breakers

```rust
use std::collections::HashMap;

pub struct ServiceManager {
    circuit_breakers: HashMap<String, CircuitBreaker>,
    error_handlers: HashMap<String, ErrorHandler>,
}

impl ServiceManager {
    pub fn new() -> Self {
        let mut circuit_breakers = HashMap::new();
        let mut error_handlers = HashMap::new();

        // Configure different thresholds for different services
        let claude_breaker = CircuitBreaker::new(3, Duration::from_secs(30));
        let file_breaker = CircuitBreaker::new(5, Duration::from_secs(10));

        circuit_breakers.insert("claude_api".to_string(), claude_breaker);
        circuit_breakers.insert("file_system".to_string(), file_breaker);

        // Create corresponding error handlers
        let claude_config = ErrorHandlerConfig {
            max_retries: 3,
            failure_threshold: 3,
            circuit_timeout: Duration::from_secs(30),
            ..Default::default()
        };

        let file_config = ErrorHandlerConfig {
            max_retries: 2,
            failure_threshold: 5,
            circuit_timeout: Duration::from_secs(10),
            ..Default::default()
        };

        error_handlers.insert("claude_api".to_string(), ErrorHandler::with_config(claude_config));
        error_handlers.insert("file_system".to_string(), ErrorHandler::with_config(file_config));

        Self {
            circuit_breakers,
            error_handlers,
        }
    }

    pub async fn execute_claude_operation<F, T, Fut>(
        &self,
        operation: F,
    ) -> ClaudeResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ClaudeResult<T>>,
    {
        let handler = self.error_handlers.get("claude_api").unwrap();
        handler.handle_with_retry(operation).await
    }

    pub async fn execute_file_operation<F, T, Fut>(
        &self,
        operation: F,
    ) -> ClaudeResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ClaudeResult<T>>,
    {
        let handler = self.error_handlers.get("file_system").unwrap();
        handler.handle_with_retry(operation).await
    }
}
```

### 2. Manual Circuit Breaker Control

```rust
pub struct AdaptiveClient {
    error_handler: ErrorHandler,
    health_check_interval: Duration,
}

impl AdaptiveClient {
    pub async fn start_health_monitoring(&self) {
        let handler = self.error_handler.clone();
        let interval = self.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Check if circuit breaker is open
                if let Some(breaker) = handler.circuit_breaker.as_ref() {
                    if breaker.is_open() {
                        // Attempt health check
                        match handler.health_check().await {
                            Ok(_) => {
                                println!("✅ Service recovered, circuit breaker reset");
                                breaker.record_success();
                            }
                            Err(_) => {
                                println!("❌ Service still unhealthy");
                            }
                        }
                    }
                }
            }
        });
    }

    async fn health_check(&self) -> ClaudeResult<()> {
        // Simple health check operation
        self.error_handler.handle_with_retry(|| async {
            // Minimal API call to check service health
            self.ping_service().await
        }).await
    }
}
```

## Telemetry and Monitoring

### 1. Metrics Collection

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TelemetryCollector {
    error_handlers: Vec<Arc<ErrorHandler>>,
    metrics_interval: Duration,
}

impl TelemetryCollector {
    pub fn start_collection(&self) {
        let handlers = self.error_handlers.clone();
        let interval = self.metrics_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                Self::collect_and_report_metrics(&handlers).await;
            }
        });
    }

    async fn collect_and_report_metrics(handlers: &[Arc<ErrorHandler>]) {
        let mut total_errors = 0u64;
        let mut total_successes = 0u64;
        let mut total_retries = 0u64;
        let mut circuit_breaker_triggers = 0u64;

        for handler in handlers {
            let telemetry = handler.telemetry();
            total_errors += telemetry.total_errors.load(Ordering::Relaxed);
            total_successes += telemetry.successful_operations.load(Ordering::Relaxed);
            total_retries += telemetry.total_retries.load(Ordering::Relaxed);
            circuit_breaker_triggers += telemetry.circuit_breaker_triggers.load(Ordering::Relaxed);
        }

        // Report to monitoring system (e.g., Prometheus, CloudWatch, etc.)
        println!("📊 [METRICS] Aggregated Error Handler Metrics:");
        println!("   Total Errors: {}", total_errors);
        println!("   Total Successes: {}", total_successes);
        println!("   Total Retries: {}", total_retries);
        println!("   Circuit Breaker Triggers: {}", circuit_breaker_triggers);

        let total_operations = total_errors + total_successes;
        if total_operations > 0 {
            let success_rate = (total_successes as f64 / total_operations as f64) * 100.0;
            println!("   Overall Success Rate: {:.2}%", success_rate);
        }
    }
}
```

### 2. Alert Integration

```rust
pub struct AlertManager {
    error_handler: ErrorHandler,
    alert_thresholds: AlertThresholds,
}

#[derive(Debug)]
pub struct AlertThresholds {
    pub error_rate_threshold: f64,
    pub circuit_breaker_threshold: u64,
    pub consecutive_failures_threshold: u32,
}

impl AlertManager {
    pub fn monitor_error_rates(&self) {
        let handler = self.error_handler.clone();
        let thresholds = self.alert_thresholds.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                Self::check_alert_conditions(&handler, &thresholds).await;
            }
        });
    }

    async fn check_alert_conditions(
        handler: &ErrorHandler,
        thresholds: &AlertThresholds,
    ) {
        let telemetry = handler.telemetry();
        
        // Check error rate
        let total_errors = telemetry.total_errors.load(Ordering::Relaxed);
        let total_successes = telemetry.successful_operations.load(Ordering::Relaxed);
        let total_operations = total_errors + total_successes;

        if total_operations > 0 {
            let error_rate = (total_errors as f64 / total_operations as f64) * 100.0;
            
            if error_rate > thresholds.error_rate_threshold {
                Self::send_alert(&format!(
                    "🚨 HIGH ERROR RATE: {:.2}% (threshold: {:.2}%)",
                    error_rate, thresholds.error_rate_threshold
                )).await;
            }
        }

        // Check circuit breaker triggers
        let cb_triggers = telemetry.circuit_breaker_triggers.load(Ordering::Relaxed);
        if cb_triggers > thresholds.circuit_breaker_threshold {
            Self::send_alert(&format!(
                "🚨 CIRCUIT BREAKER TRIGGERS: {} (threshold: {})",
                cb_triggers, thresholds.circuit_breaker_threshold
            )).await;
        }
    }

    async fn send_alert(message: &str) {
        eprintln!("{}", message);
        // Integration with alerting services (PagerDuty, Slack, email, etc.)
    }
}
```

## Testing Strategies

### 1. Error Injection Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicU32};

    struct FailingService {
        fail_count: AtomicU32,
        should_fail: AtomicBool,
    }

    impl FailingService {
        fn new() -> Self {
            Self {
                fail_count: AtomicU32::new(0),
                should_fail: AtomicBool::new(true),
            }
        }

        async fn call(&self) -> ClaudeResult<String> {
            let count = self.fail_count.fetch_add(1, Ordering::Relaxed);
            
            if self.should_fail.load(Ordering::Relaxed) && count < 3 {
                Err(ClaudeError::ApiError {
                    status: 500,
                    message: "Simulated failure".to_string(),
                    error_type: Some("test_error".to_string()),
                    param: None,
                    context: Some(ErrorContext::new("test_operation")),
                })
            } else {
                Ok("Success".to_string())
            }
        }

        fn start_succeeding(&self) {
            self.should_fail.store(false, Ordering::Relaxed);
        }
    }

    #[tokio::test]
    async fn test_retry_logic() {
        let service = FailingService::new();
        let handler = ErrorHandler::new();

        let result = handler.handle_with_retry(|| async {
            service.call().await
        }).await;

        assert!(result.is_ok());
        assert_eq!(service.fail_count.load(Ordering::Relaxed), 4); // 3 failures + 1 success
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let service = FailingService::new();
        let config = ErrorHandlerConfig {
            failure_threshold: 2,
            circuit_timeout: Duration::from_millis(100),
            max_retries: 1,
            ..Default::default()
        };
        let handler = ErrorHandler::with_config(config);

        // First few calls should fail and trigger circuit breaker
        for _ in 0..3 {
            let _ = handler.handle_with_retry(|| async {
                service.call().await
            }).await;
        }

        // Circuit should now be open
        let result = handler.handle_with_retry(|| async {
            service.call().await
        }).await;

        assert!(result.is_err());
        if let Err(ClaudeError::ConfigError { message, .. }) = result {
            assert!(message.contains("Circuit breaker"));
        }
    }
}
```

### 2. Telemetry Testing

```rust
#[tokio::test]
async fn test_telemetry_tracking() {
    let handler = ErrorHandler::new();
    
    // Perform some operations
    for i in 0..10 {
        let result = handler.handle_with_retry(|| async {
            if i < 3 {
                Err(ClaudeError::ApiError {
                    status: 500,
                    message: "Test error".to_string(),
                    error_type: Some("test".to_string()),
                    param: None,
                    context: None,
                })
            } else {
                Ok("Success".to_string())
            }
        }).await;
    }

    let telemetry = handler.telemetry();
    
    // Verify metrics
    assert_eq!(telemetry.total_errors.load(Ordering::Relaxed), 3);
    assert_eq!(telemetry.successful_operations.load(Ordering::Relaxed), 7);
    assert!(telemetry.total_retries.load(Ordering::Relaxed) > 0);
}
```

## Performance Considerations

### 1. Resource Management

```rust
// ✅ Good: Reuse error handlers
lazy_static! {
    static ref GLOBAL_ERROR_HANDLER: ErrorHandler = ErrorHandler::new();
}

// ❌ Bad: Create new handler for each operation
async fn bad_example() {
    let handler = ErrorHandler::new(); // Creates new telemetry, circuit breaker, etc.
    // ... use handler
}

// ✅ Good: Reuse shared handler
async fn good_example() {
    GLOBAL_ERROR_HANDLER.handle_with_retry(|| async {
        // ... operation
    }).await
}
```

### 2. Efficient Telemetry

```rust
// Configure telemetry collection interval based on needs
let config = ErrorHandlerConfig {
    // Higher intervals for less critical services
    circuit_timeout: Duration::from_secs(300), // 5 minutes for batch services
    
    // Lower intervals for critical real-time services
    circuit_timeout: Duration::from_secs(30),  // 30 seconds for user-facing APIs
    
    ..Default::default()
};
```

## Common Pitfalls

### 1. Over-Aggressive Retries

```rust
// ❌ Bad: Too many retries for non-retryable errors
let bad_config = ErrorHandlerConfig {
    max_retries: 10, // Too high for validation errors
    ..Default::default()
};

// ✅ Good: Reasonable retry counts
let good_config = ErrorHandlerConfig {
    max_retries: 3, // Appropriate for most scenarios
    ..Default::default()
};
```

### 2. Missing Error Context

```rust
// ❌ Bad: No context information
Err(ClaudeError::ToolError {
    tool_name: "file_reader".to_string(),
    message: "Failed to read".to_string(),
    context: None, // Missing valuable debugging info
})

// ✅ Good: Rich context
let context = ErrorContext::new("file_read_operation")
    .add_metadata("file_path", path)
    .add_metadata("user_id", &user_id)
    .add_metadata("timestamp", &timestamp);

Err(ClaudeError::ToolError {
    tool_name: "file_reader".to_string(),
    message: format!("Failed to read file: {}", path),
    context: Some(context),
})
```

### 3. Ignoring Circuit Breaker State

```rust
// ❌ Bad: Not handling circuit breaker errors
let result = handler.handle_with_retry(operation).await?;

// ✅ Good: Proper circuit breaker error handling
match handler.handle_with_retry(operation).await {
    Ok(result) => Ok(result),
    Err(ClaudeError::ConfigError { message, .. }) if message.contains("Circuit breaker") => {
        // Graceful degradation or alternative path
        fallback_operation().await
    }
    Err(other) => Err(other),
}
```

## Related Documentation

- [Architecture: Error Handling System](../architecture/error-handling-system.md)
- [API Reference: Error Types](../api/error-types.md)
- [API Reference: Model Registry](../api/model-registry-types.md)
- [Configuration Guide](../configuration/error-handling-config.md)