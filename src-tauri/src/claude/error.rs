use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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

#[derive(Debug)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::result_large_err)]
pub enum ClaudeError {
    // HTTP/Network Errors
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),

    // API Errors
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
        param: Option<String>,
        context: Option<ErrorContext>,
    },

    // Model Errors
    ModelError {
        model: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Content Block Errors
    ContentBlockError {
        block_type: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Tool Errors
    ToolError {
        tool_name: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Configuration Errors
    ConfigError {
        message: String,
        context: Option<ErrorContext>,
    },

    // Validation Errors
    ValidationError {
        field: String,
        message: String,
        context: Option<ErrorContext>,
    },

    // Streaming Errors (future-ready)
    StreamingError {
        message: String,
        context: Option<ErrorContext>,
    },

    // System Errors
    TimeoutError {
        duration: std::time::Duration,
        context: Option<ErrorContext>,
    },
    RateLimitError {
        retry_after: Option<u64>,
        context: Option<ErrorContext>,
    },
}

impl fmt::Display for ClaudeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClaudeError::HttpError(e) => write!(f, "HTTP request failed: {}", e),
            ClaudeError::JsonError(e) => write!(f, "JSON processing failed: {}", e),
            ClaudeError::ApiError {
                status,
                message,
                error_type,
                param,
                context,
            } => {
                write!(f, "API error ({}): {}", status, message)?;
                if let Some(error_type) = error_type {
                    write!(f, " [type: {}]", error_type)?;
                }
                if let Some(param) = param {
                    write!(f, " [param: {}]", param)?;
                }
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::ModelError {
                model,
                message,
                context,
            } => {
                write!(f, "Model error ({}): {}", model, message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::ContentBlockError {
                block_type,
                message,
                context,
            } => {
                write!(f, "Content block error ({}): {}", block_type, message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::ToolError {
                tool_name,
                message,
                context,
            } => {
                write!(f, "Tool error ({}): {}", tool_name, message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::ConfigError { message, context } => {
                write!(f, "Configuration error: {}", message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::ValidationError {
                field,
                message,
                context,
            } => {
                write!(f, "Validation error ({}): {}", field, message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::StreamingError { message, context } => {
                write!(f, "Streaming error: {}", message)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::TimeoutError { duration, context } => {
                write!(f, "Request timed out after {:?}", duration)?;
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
            ClaudeError::RateLimitError {
                retry_after,
                context,
            } => {
                write!(f, "Rate limit exceeded")?;
                if let Some(seconds) = retry_after {
                    write!(f, " (retry after {} seconds)", seconds)?;
                }
                if let Some(ctx) = context {
                    write!(f, " [operation: {}]", ctx.operation)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ClaudeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClaudeError::HttpError(e) => Some(e),
            ClaudeError::JsonError(e) => Some(e),
            _ => None,
        }
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

// Error handling utilities
#[derive(Debug)]
pub struct ErrorHandler {
    max_retries: u32,
    base_delay: std::time::Duration,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            base_delay: std::time::Duration::from_millis(500),
        }
    }

    #[allow(dead_code)]
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    #[allow(dead_code)]
    pub fn with_base_delay(mut self, base_delay: std::time::Duration) -> Self {
        self.base_delay = base_delay;
        self
    }

    pub async fn handle_with_retry<F, T, Fut>(&self, mut operation: F) -> ClaudeResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ClaudeResult<T>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if !error.is_retryable() || attempt == self.max_retries {
                        return Err(error);
                    }

                    let delay = if let Some(retry_after) = error.should_retry_after() {
                        retry_after
                    } else {
                        self.base_delay * (2_u32.pow(attempt))
                    };

                    tokio::time::sleep(delay).await;
                    last_error = Some(error);
                }
            }
        }

        Err(last_error.unwrap())
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub type ClaudeResult<T> = Result<T, ClaudeError>;
