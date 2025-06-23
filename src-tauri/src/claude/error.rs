use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message_id: Option<String>,
    pub tool_use_id: Option<String>,
    pub retry_count: u32,
    pub metadata: HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            timestamp: chrono::Utc::now(),
            message_id: None,
            tool_use_id: None,
            retry_count: 0,
            metadata: HashMap::new(),
        }
    }

    /// Log this error context with structured information (sanitized for security)
    pub fn log_error(&self, error: &ClaudeError) {
        let error_type = match error {
            ClaudeError::ApiError { .. } => "api_error",
            ClaudeError::ModelError { .. } => "model_error",
            ClaudeError::ContentBlockError { .. } => "content_block_error",
            ClaudeError::ToolError { .. } => "tool_error",
            ClaudeError::ConfigError { .. } => "config_error",
            ClaudeError::ValidationError { .. } => "validation_error",
            ClaudeError::StreamingError { .. } => "streaming_error",
            ClaudeError::TimeoutError { .. } => "timeout_error",
            ClaudeError::RateLimitError { .. } => "rate_limit_error",
            ClaudeError::HttpError(_) => "http_error",
            ClaudeError::JsonError(_) => "json_error",
        };

        // Sanitize sensitive error message
        let safe_error_message = Self::sanitize_error_message(&error.to_string());

        // Sanitize metadata to remove sensitive information
        let safe_metadata = self.sanitize_metadata();

        eprintln!(
            "🚨 [ERROR] {} | type={} | operation={} | retry_count={} | timestamp={}{}{}{}",
            safe_error_message,
            error_type,
            self.operation,
            self.retry_count,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.message_id
                .as_ref()
                .map(|id| format!(" | message_id={}", Self::sanitize_id(id)))
                .unwrap_or_default(),
            self.tool_use_id
                .as_ref()
                .map(|id| format!(" | tool_use_id={}", Self::sanitize_id(id)))
                .unwrap_or_default(),
            if !safe_metadata.is_empty() {
                format!(" | metadata={:?}", safe_metadata)
            } else {
                String::new()
            }
        );
    }

    /// Sanitize error messages to prevent sensitive data exposure
    fn sanitize_error_message(message: &str) -> String {
        let mut sanitized = message.to_string();

        // Remove potential API keys (simple string pattern matching)
        if let Some(start) = sanitized.find("sk-ant-") {
            if let Some(end) = sanitized[start..].find(' ').map(|i| start + i) {
                sanitized.replace_range(start..end, "[API_KEY_REDACTED]");
            } else {
                // If no space found, redact to end of string
                sanitized.replace_range(start.., "[API_KEY_REDACTED]");
            }
        }

        // Remove sensitive file paths
        for sensitive_path in &["/home/", "/Users/", "C:\\Users\\"] {
            if let Some(start) = sanitized.find(sensitive_path) {
                if let Some(end) = sanitized[start..].find(' ').map(|i| start + i) {
                    sanitized.replace_range(start..end, "/[USER_DIR_REDACTED]");
                } else {
                    // Find next path separator or end of string
                    let search_from = start + sensitive_path.len();
                    let end = sanitized[search_from..]
                        .find(|c: char| c == '/' || c == '\\' || c.is_whitespace())
                        .map(|i| search_from + i)
                        .unwrap_or(sanitized.len());
                    sanitized.replace_range(start..end, "/[USER_DIR_REDACTED]");
                }
            }
        }

        // Truncate very long messages to prevent log flooding
        if sanitized.len() > 500 {
            format!("{}...[TRUNCATED]", &sanitized[..497])
        } else {
            sanitized
        }
    }

    /// Sanitize ID fields to show only prefix for debugging while protecting privacy
    fn sanitize_id(id: &str) -> String {
        if id.len() > 8 {
            format!("{}...[REDACTED]", &id[..8])
        } else {
            id.to_string()
        }
    }

    /// Sanitize metadata to remove sensitive information
    fn sanitize_metadata(&self) -> HashMap<String, String> {
        let mut safe_metadata = HashMap::new();

        for (key, value) in &self.metadata {
            let safe_key = key.clone();
            let safe_value = if key.to_lowercase().contains("key")
                || key.to_lowercase().contains("secret")
                || key.to_lowercase().contains("token")
                || key.to_lowercase().contains("password")
            {
                "[REDACTED]".to_string()
            } else if key.to_lowercase().contains("path") {
                Self::sanitize_error_message(value)
            } else if value.len() > 100 {
                format!("{}...[TRUNCATED]", &value[..97])
            } else {
                value.clone()
            };
            safe_metadata.insert(safe_key, safe_value);
        }

        safe_metadata
    }

    /// Log successful operation for telemetry
    pub fn log_success(&self, duration: Option<std::time::Duration>) {
        println!(
            "✅ [SUCCESS] operation={} | retry_count={} | timestamp={}{}{}{}",
            self.operation,
            self.retry_count,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            duration
                .map(|d| format!(" | duration={}ms", d.as_millis()))
                .unwrap_or_default(),
            self.message_id
                .as_ref()
                .map(|id| format!(" | message_id={}", id))
                .unwrap_or_default(),
            self.tool_use_id
                .as_ref()
                .map(|id| format!(" | tool_use_id={}", id))
                .unwrap_or_default()
        );
    }

    /// Log retry attempt for monitoring
    pub fn log_retry(&self, attempt: u32, delay: std::time::Duration) {
        println!(
            "🔄 [RETRY] operation={} | attempt={} | delay={}ms | timestamp={}{}{}",
            self.operation,
            attempt,
            delay.as_millis(),
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.message_id
                .as_ref()
                .map(|id| format!(" | message_id={}", id))
                .unwrap_or_default(),
            self.tool_use_id
                .as_ref()
                .map(|id| format!(" | tool_use_id={}", id))
                .unwrap_or_default()
        );
    }

    #[allow(dead_code)]
    pub fn with_message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = Some(tool_use_id.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_retry_count(mut self, retry_count: u32) -> Self {
        self.retry_count = retry_count;
        self
    }

    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::result_large_err)]
pub enum ClaudeError {
    // HTTP/Network Errors
    #[error("HTTP request failed: {0}")]
    HttpError(reqwest::Error),

    #[error("JSON processing failed: {0}")]
    JsonError(serde_json::Error),

    // API Errors
    #[error("API error ({status}): {message}{}", format_error_details(.error_type, .param, .context))]
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
        param: Option<String>,
        context: Option<ErrorContext>,
    },

    // Model Errors
    #[error("Model error ({model}): {message}{}", format_context_operation(.context))]
    ModelError {
        model: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Content Block Errors
    #[error("Content block error ({block_type}): {message}{}", format_context_operation(.context))]
    ContentBlockError {
        block_type: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Tool Errors
    #[error("Tool error ({tool_name}): {message}{}", format_context_operation(.context))]
    ToolError {
        tool_name: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Configuration Errors
    #[error("Configuration error: {message}{}", format_context_operation(.context))]
    ConfigError {
        message: String,
        context: Option<ErrorContext>,
    },

    // Validation Errors
    #[error("Validation error ({field}): {message}{}", format_context_operation(.context))]
    ValidationError {
        field: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Streaming Errors (future-ready)
    #[error("Streaming error: {message}{}", format_context_operation(.context))]
    StreamingError {
        message: String,
        context: Option<ErrorContext>,
    },

    // System Errors
    #[error("Request timed out after {duration:?}{}", format_context_operation(.context))]
    TimeoutError {
        duration: std::time::Duration,
        context: Option<ErrorContext>,
    },

    #[error("Rate limit exceeded{}{}", 
        .retry_after.map(|s| format!(" (retry after {} seconds)", s)).unwrap_or_default(),
        format_context_operation(.context)
    )]
    RateLimitError {
        retry_after: Option<u64>,
        context: Option<ErrorContext>,
    },
}

// Helper functions for error formatting
fn format_error_details(
    error_type: &Option<String>,
    param: &Option<String>,
    context: &Option<ErrorContext>,
) -> String {
    let mut details = String::new();

    if let Some(error_type) = error_type {
        details.push_str(&format!(" [type: {}]", error_type));
    }

    if let Some(param) = param {
        details.push_str(&format!(" [param: {}]", param));
    }

    details.push_str(&format_context_operation(context));
    details
}

fn format_context_operation(context: &Option<ErrorContext>) -> String {
    if let Some(ctx) = context {
        format!(" [operation: {}]", ctx.operation)
    } else {
        String::new()
    }
}

impl ClaudeError {
    #[allow(dead_code)]
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        match &mut self {
            ClaudeError::ApiError { context: c, .. }
            | ClaudeError::ModelError { context: c, .. }
            | ClaudeError::ContentBlockError { context: c, .. }
            | ClaudeError::ToolError { context: c, .. }
            | ClaudeError::ConfigError { context: c, .. }
            | ClaudeError::ValidationError { context: c, .. }
            | ClaudeError::StreamingError { context: c, .. }
            | ClaudeError::TimeoutError { context: c, .. }
            | ClaudeError::RateLimitError { context: c, .. } => {
                *c = Some(context);
            }
            _ => {}
        }
        self
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ClaudeError::TimeoutError { .. }
                | ClaudeError::RateLimitError { .. }
                | ClaudeError::HttpError(_)
        )
    }

    pub fn should_retry_after(&self) -> Option<std::time::Duration> {
        match self {
            ClaudeError::RateLimitError {
                retry_after: Some(seconds),
                ..
            } => Some(std::time::Duration::from_secs(*seconds)),
            ClaudeError::TimeoutError { .. } => Some(std::time::Duration::from_secs(5)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn get_context(&self) -> Option<&ErrorContext> {
        match self {
            ClaudeError::ApiError { context, .. }
            | ClaudeError::ModelError { context, .. }
            | ClaudeError::ContentBlockError { context, .. }
            | ClaudeError::ToolError { context, .. }
            | ClaudeError::ConfigError { context, .. }
            | ClaudeError::ValidationError { context, .. }
            | ClaudeError::StreamingError { context, .. }
            | ClaudeError::TimeoutError { context, .. }
            | ClaudeError::RateLimitError { context, .. } => context.as_ref(),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ClaudeError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            ClaudeError::TimeoutError {
                duration: std::time::Duration::from_secs(120),
                context: None,
            }
        } else {
            ClaudeError::HttpError(error)
        }
    }
}

impl From<serde_json::Error> for ClaudeError {
    fn from(error: serde_json::Error) -> Self {
        ClaudeError::JsonError(error)
    }
}

// Error handling configuration
#[derive(Debug, Clone)]
pub struct ErrorHandlerConfig {
    pub max_retries: u32,
    pub base_delay: std::time::Duration,
    pub max_delay: std::time::Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
    pub circuit_breaker_enabled: bool,
    pub failure_threshold: u32,
    pub circuit_timeout: std::time::Duration,
}

impl Default for ErrorHandlerConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: std::time::Duration::from_millis(500),
            max_delay: std::time::Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
            circuit_breaker_enabled: true,
            failure_threshold: 5,
            circuit_timeout: std::time::Duration::from_secs(60),
        }
    }
}

// Circuit breaker implementation with atomic state management
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_count: std::sync::atomic::AtomicU32,
    last_failure_time: std::sync::Mutex<Option<std::time::Instant>>,
    failure_threshold: u32,
    timeout: std::time::Duration,
    state: std::sync::atomic::AtomicU8, // 0 = Closed, 1 = Open, 2 = Half-Open
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitBreakerState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout: std::time::Duration) -> Self {
        Self {
            failure_count: std::sync::atomic::AtomicU32::new(0),
            last_failure_time: std::sync::Mutex::new(None),
            failure_threshold,
            timeout,
            state: std::sync::atomic::AtomicU8::new(CircuitBreakerState::Closed as u8),
        }
    }

    pub fn can_execute(&self) -> bool {
        let current_state = self.get_state();

        match current_state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if timeout has passed to transition to half-open
                if self.should_attempt_reset() {
                    self.set_state(CircuitBreakerState::HalfOpen);
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    pub fn is_open(&self) -> bool {
        !self.can_execute()
    }

    pub fn record_success(&self) {
        self.failure_count
            .store(0, std::sync::atomic::Ordering::Relaxed);
        self.set_state(CircuitBreakerState::Closed);

        // Clear failure time
        if let Ok(mut last_failure) = self.last_failure_time.lock() {
            *last_failure = None;
        }
    }

    pub fn record_failure(&self) {
        let current_failures = self
            .failure_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;

        // Update failure time
        if let Ok(mut last_failure) = self.last_failure_time.lock() {
            *last_failure = Some(std::time::Instant::now());
        }

        // Check if we should open the circuit
        if current_failures >= self.failure_threshold {
            self.set_state(CircuitBreakerState::Open);
        }
    }

    fn get_state(&self) -> CircuitBreakerState {
        match self.state.load(std::sync::atomic::Ordering::Relaxed) {
            0 => CircuitBreakerState::Closed,
            1 => CircuitBreakerState::Open,
            2 => CircuitBreakerState::HalfOpen,
            _ => CircuitBreakerState::Closed, // Default fallback
        }
    }

    fn set_state(&self, new_state: CircuitBreakerState) {
        self.state
            .store(new_state as u8, std::sync::atomic::Ordering::Relaxed);
    }

    fn should_attempt_reset(&self) -> bool {
        if let Ok(last_failure) = self.last_failure_time.lock() {
            if let Some(failure_time) = *last_failure {
                return failure_time.elapsed() >= self.timeout;
            }
        }
        false
    }
}

// Error telemetry tracking with bounded memory usage
#[derive(Debug, Clone)]
pub struct ErrorTelemetry {
    pub total_errors: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub errors_by_type: std::sync::Arc<std::sync::Mutex<BoundedErrorCounter>>,
    pub total_retries: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub successful_operations: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub circuit_breaker_triggers: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

// Bounded error counter to prevent memory leaks
#[derive(Debug, Clone)]
pub struct BoundedErrorCounter {
    counts: HashMap<String, u64>,
    max_entries: usize,
    last_cleanup: std::time::Instant,
    cleanup_interval: std::time::Duration,
}

impl BoundedErrorCounter {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            max_entries: 100, // Limit to 100 different error types
            last_cleanup: std::time::Instant::now(),
            cleanup_interval: std::time::Duration::from_secs(300), // Clean up every 5 minutes
        }
    }

    pub fn increment(&mut self, error_type: &str) {
        // Periodic cleanup to prevent unbounded growth
        if self.last_cleanup.elapsed() > self.cleanup_interval {
            self.cleanup_low_frequency_entries();
            self.last_cleanup = std::time::Instant::now();
        }

        // If we're at capacity and this is a new error type, remove the least frequent
        if self.counts.len() >= self.max_entries && !self.counts.contains_key(error_type) {
            self.remove_least_frequent();
        }

        *self.counts.entry(error_type.to_string()).or_insert(0) += 1;
    }

    pub fn get_counts(&self) -> HashMap<String, u64> {
        self.counts.clone()
    }

    fn cleanup_low_frequency_entries(&mut self) {
        // Remove entries with count of 1 that are older (simple heuristic)
        self.counts.retain(|_, count| *count > 1);
    }

    fn remove_least_frequent(&mut self) {
        if let Some((least_frequent_key, _)) = self.counts.iter().min_by_key(|(_, count)| *count) {
            let key_to_remove = least_frequent_key.clone();
            self.counts.remove(&key_to_remove);
        }
    }
}

impl Default for ErrorTelemetry {
    fn default() -> Self {
        Self {
            total_errors: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            errors_by_type: std::sync::Arc::new(std::sync::Mutex::new(BoundedErrorCounter::new())),
            total_retries: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            successful_operations: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            circuit_breaker_triggers: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }
}

impl ErrorTelemetry {
    pub fn record_error(&self, error_type: &str) {
        self.total_errors
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if let Ok(mut errors_by_type) = self.errors_by_type.lock() {
            errors_by_type.increment(error_type);
        }
    }

    pub fn record_retry(&self) {
        self.total_retries
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn record_success(&self) {
        self.successful_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn record_circuit_breaker_trigger(&self) {
        self.circuit_breaker_triggers
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Print telemetry summary for monitoring
    pub fn print_summary(&self) {
        let total_errors = self.total_errors.load(std::sync::atomic::Ordering::Relaxed);
        let total_retries = self
            .total_retries
            .load(std::sync::atomic::Ordering::Relaxed);
        let successful_ops = self
            .successful_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let circuit_breaker_triggers = self
            .circuit_breaker_triggers
            .load(std::sync::atomic::Ordering::Relaxed);

        println!("📊 [TELEMETRY] Error Handler Summary:");
        println!("   • Total Errors: {}", total_errors);
        println!("   • Total Retries: {}", total_retries);
        println!("   • Successful Operations: {}", successful_ops);
        println!(
            "   • Circuit Breaker Triggers: {}",
            circuit_breaker_triggers
        );

        if let Ok(errors_by_type) = self.errors_by_type.lock() {
            let error_counts = errors_by_type.get_counts();
            if !error_counts.is_empty() {
                println!("   • Errors by Type:");
                for (error_type, count) in error_counts.iter() {
                    println!("     - {}: {}", error_type, count);
                }
            }
        }

        let total_operations = successful_ops + total_errors;
        if total_operations > 0 {
            let success_rate = (successful_ops as f64 / total_operations as f64) * 100.0;
            println!("   • Success Rate: {:.2}%", success_rate);
        }
    }
}

// Enhanced error handling utilities
#[derive(Debug)]
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    circuit_breaker: Option<CircuitBreaker>,
    telemetry: ErrorTelemetry,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self::with_config(ErrorHandlerConfig::default())
    }

    pub fn with_config(config: ErrorHandlerConfig) -> Self {
        let circuit_breaker = if config.circuit_breaker_enabled {
            Some(CircuitBreaker::new(
                config.failure_threshold,
                config.circuit_timeout,
            ))
        } else {
            None
        };

        Self {
            config,
            circuit_breaker,
            telemetry: ErrorTelemetry::default(),
        }
    }

    /// Get a reference to the telemetry data
    pub fn telemetry(&self) -> &ErrorTelemetry {
        &self.telemetry
    }

    /// Print telemetry summary for monitoring
    pub fn print_telemetry_summary(&self) {
        self.telemetry.print_summary();
    }

    #[allow(dead_code)]
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    #[allow(dead_code)]
    pub fn with_base_delay(mut self, base_delay: std::time::Duration) -> Self {
        self.config.base_delay = base_delay;
        self
    }

    pub async fn handle_with_retry<F, T, Fut>(&self, mut operation: F) -> ClaudeResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ClaudeResult<T>>,
    {
        let start_time = std::time::Instant::now();
        let mut context = ErrorContext::new("retry_operation");

        // Check circuit breaker
        if let Some(circuit_breaker) = &self.circuit_breaker {
            if !circuit_breaker.can_execute() {
                self.telemetry.record_circuit_breaker_trigger();
                let error = ClaudeError::ConfigError {
                    message: "Circuit breaker is open - operation temporarily unavailable"
                        .to_string(),
                    context: Some(context.clone()),
                };
                context.log_error(&error);
                return Err(error);
            }
        }

        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            context.retry_count = attempt;

            match operation().await {
                Ok(result) => {
                    // Record success for circuit breaker and telemetry
                    if let Some(circuit_breaker) = &self.circuit_breaker {
                        circuit_breaker.record_success();
                    }
                    self.telemetry.record_success();

                    let duration = start_time.elapsed();
                    context.log_success(Some(duration));
                    return Ok(result);
                }
                Err(error) => {
                    // Determine error type for telemetry
                    let error_type = match &error {
                        ClaudeError::ApiError { .. } => "api_error",
                        ClaudeError::ModelError { .. } => "model_error",
                        ClaudeError::ContentBlockError { .. } => "content_block_error",
                        ClaudeError::ToolError { .. } => "tool_error",
                        ClaudeError::ConfigError { .. } => "config_error",
                        ClaudeError::ValidationError { .. } => "validation_error",
                        ClaudeError::StreamingError { .. } => "streaming_error",
                        ClaudeError::TimeoutError { .. } => "timeout_error",
                        ClaudeError::RateLimitError { .. } => "rate_limit_error",
                        ClaudeError::HttpError(_) => "http_error",
                        ClaudeError::JsonError(_) => "json_error",
                    };

                    // Record failure for circuit breaker and telemetry
                    if let Some(circuit_breaker) = &self.circuit_breaker {
                        circuit_breaker.record_failure();
                    }
                    self.telemetry.record_error(error_type);

                    if !error.is_retryable() || attempt == self.config.max_retries {
                        context.log_error(&error);
                        return Err(error);
                    }

                    // Log retry attempt
                    if attempt < self.config.max_retries {
                        self.telemetry.record_retry();
                        let delay = self.calculate_delay(attempt, &error);
                        context.log_retry(attempt + 1, delay);
                        tokio::time::sleep(delay).await;
                    }

                    last_error = Some(error);
                }
            }
        }

        // Final error logging
        let final_error = last_error.unwrap();
        context.log_error(&final_error);
        Err(final_error)
    }

    fn calculate_delay(&self, attempt: u32, error: &ClaudeError) -> std::time::Duration {
        // Use error-specific delay if available
        if let Some(delay) = error.should_retry_after() {
            return delay;
        }

        // Calculate exponential backoff
        let base_delay_ms = self.config.base_delay.as_millis() as f64;
        let delay_ms = base_delay_ms * self.config.backoff_multiplier.powi(attempt as i32);
        let delay = std::time::Duration::from_millis(delay_ms as u64);

        // Apply maximum delay cap
        let capped_delay = std::cmp::min(delay, self.config.max_delay);

        // Apply jitter if enabled
        if self.config.jitter {
            self.apply_jitter(capped_delay)
        } else {
            capped_delay
        }
    }

    fn apply_jitter(&self, delay: std::time::Duration) -> std::time::Duration {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create a simple pseudo-random jitter based on current time
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .hash(&mut hasher);
        let hash = hasher.finish();

        let jitter_factor = 0.1; // 10% jitter
        let jitter = ((hash % 1000) as f64 / 1000.0 - 0.5) * 2.0 * jitter_factor;
        let jittered_ms = (delay.as_millis() as f64) * (1.0 + jitter);
        std::time::Duration::from_millis(jittered_ms.max(0.0) as u64)
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub type ClaudeResult<T> = Result<T, ClaudeError>;
