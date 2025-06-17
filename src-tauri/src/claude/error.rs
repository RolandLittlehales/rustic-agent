use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
pub enum ClaudeError {
    /// HTTP request failed
    HttpError(reqwest::Error),
    /// JSON serialization/deserialization failed
    JsonError(serde_json::Error),
    /// API returned an error response
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
        param: Option<String>,
    },
    /// Configuration error (missing API key, etc.)
    ConfigError(String),
    /// Tool execution error
    ToolError(String),
    /// Invalid input provided
    ValidationError(String),
    /// Network timeout
    TimeoutError,
    /// Rate limit exceeded
    RateLimitError { retry_after: Option<u64> },
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
            } => {
                write!(f, "API error ({}): {}", status, message)?;
                if let Some(error_type) = error_type {
                    write!(f, " [type: {}]", error_type)?;
                }
                if let Some(param) = param {
                    write!(f, " [param: {}]", param)?;
                }
                Ok(())
            }
            ClaudeError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ClaudeError::ToolError(msg) => write!(f, "Tool execution error: {}", msg),
            ClaudeError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ClaudeError::TimeoutError => write!(f, "Request timed out"),
            ClaudeError::RateLimitError { retry_after } => {
                write!(f, "Rate limit exceeded")?;
                if let Some(seconds) = retry_after {
                    write!(f, " (retry after {} seconds)", seconds)?;
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

impl From<reqwest::Error> for ClaudeError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            ClaudeError::TimeoutError
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

#[allow(dead_code)]
pub type ClaudeResult<T> = Result<T, ClaudeError>;
