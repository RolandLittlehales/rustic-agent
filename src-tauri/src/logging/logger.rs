/*!
 * Core Logger Implementation
 *
 * Provides structured logging with security sanitization, configurable levels,
 * and consistent formatting for all application components.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::time::Duration;

/// Log levels in order of severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl LogLevel {
    /// Parse log level from environment variable or string
    pub fn from_env() -> Self {
        env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".to_string())
            .parse()
            .unwrap_or(LogLevel::Info)
    }

    /// Get emoji for log level
    pub fn emoji(&self) -> &'static str {
        match self {
            LogLevel::Trace => "🔍",
            LogLevel::Debug => "🐛",
            LogLevel::Info => "ℹ️",
            LogLevel::Warn => "⚠️",
            LogLevel::Error => "🚨",
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(LogLevel::Trace),
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" | "WARNING" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

/// Structured log entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub operation: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub duration: Option<Duration>,
}

impl LogEntry {
    pub fn new(level: LogLevel, operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            operation: operation.into(),
            message: message.into(),
            metadata: HashMap::new(),
            duration: None,
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Format the log entry for output
    pub fn format(&self) -> String {
        let mut parts = vec![
            format!("{}", self.level.emoji()),
            format!("[{}]", self.level),
            format!("operation={}", self.operation),
            format!(
                "message=\"{}\"",
                SecuritySanitizer::sanitize_message(&self.message)
            ),
            format!(
                "timestamp={}",
                self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ),
        ];

        // Add duration if present
        if let Some(duration) = self.duration {
            parts.push(format!("duration={}ms", duration.as_millis()));
        }

        // Add sanitized metadata
        for (key, value) in &self.metadata {
            let safe_value = SecuritySanitizer::sanitize_metadata_value(key, value);
            parts.push(format!("{}={}", key, safe_value));
        }

        parts.join(" | ")
    }
}

/// Main logger implementation
#[derive(Debug)]
pub struct Logger {
    level: LogLevel,
    telemetry: super::telemetry::TelemetryCollector,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            telemetry: super::telemetry::TelemetryCollector::new(),
        }
    }

    /// Check if a level should be logged
    pub fn should_log(&self, level: LogLevel) -> bool {
        level >= self.level
    }

    /// Set the minimum log level
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    /// Log an error message
    pub fn error(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        context: Option<HashMap<String, String>>,
    ) {
        if !self.should_log(LogLevel::Error) {
            return;
        }

        let mut entry = LogEntry::new(LogLevel::Error, operation, message);
        if let Some(ctx) = context {
            for (k, v) in ctx {
                entry = entry.with_metadata(k, v);
            }
        }

        self.output(&entry);
        self.telemetry.record_log(&entry);
    }

    /// Log a warning message
    pub fn warn(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        context: Option<HashMap<String, String>>,
    ) {
        if !self.should_log(LogLevel::Warn) {
            return;
        }

        let mut entry = LogEntry::new(LogLevel::Warn, operation, message);
        if let Some(ctx) = context {
            for (k, v) in ctx {
                entry = entry.with_metadata(k, v);
            }
        }

        self.output(&entry);
        self.telemetry.record_log(&entry);
    }

    /// Log an info message
    pub fn info(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        context: Option<HashMap<String, String>>,
    ) {
        if !self.should_log(LogLevel::Info) {
            return;
        }

        let mut entry = LogEntry::new(LogLevel::Info, operation, message);
        if let Some(ctx) = context {
            for (k, v) in ctx {
                entry = entry.with_metadata(k, v);
            }
        }

        self.output(&entry);
        self.telemetry.record_log(&entry);
    }

    /// Log a debug message
    pub fn debug(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        context: Option<HashMap<String, String>>,
    ) {
        if !self.should_log(LogLevel::Debug) {
            return;
        }

        let mut entry = LogEntry::new(LogLevel::Debug, operation, message);
        if let Some(ctx) = context {
            for (k, v) in ctx {
                entry = entry.with_metadata(k, v);
            }
        }

        self.output(&entry);
        self.telemetry.record_log(&entry);
    }

    /// Log a trace message
    pub fn trace(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        context: Option<HashMap<String, String>>,
    ) {
        if !self.should_log(LogLevel::Trace) {
            return;
        }

        let mut entry = LogEntry::new(LogLevel::Trace, operation, message);
        if let Some(ctx) = context {
            for (k, v) in ctx {
                entry = entry.with_metadata(k, v);
            }
        }

        self.output(&entry);
        self.telemetry.record_log(&entry);
    }

    /// Log tool execution results
    pub fn log_tool_execution(
        &self,
        tool_name: impl Into<String>,
        success: bool,
        duration: Duration,
    ) {
        let tool_name_str = tool_name.into();
        let result = if success { "success" } else { "failure" };
        let entry = LogEntry::new(
            LogLevel::Info,
            "tool_execution",
            format!("Tool {} executed", tool_name_str),
        )
        .with_metadata("tool", tool_name_str)
        .with_metadata("result", result)
        .with_duration(duration);

        if self.should_log(LogLevel::Info) {
            self.output(&entry);
        }
        self.telemetry.record_tool_execution(&entry);
    }

    /// Log Claude API calls with cost tracking
    pub fn log_claude_api(
        &self,
        model: impl Into<String>,
        tokens: u32,
        cost: f64,
        duration: Duration,
    ) {
        let entry = LogEntry::new(
            LogLevel::Info,
            "claude_api_call",
            "Claude API request completed",
        )
        .with_metadata("model", model.into())
        .with_metadata("tokens", tokens.to_string())
        .with_metadata("cost_usd", format!("{:.6}", cost))
        .with_duration(duration);

        if self.should_log(LogLevel::Info) {
            self.output(&entry);
        }
        self.telemetry.record_api_call(&entry);
    }

    /// Log cost information for monitoring
    pub fn log_cost(&self, operation: impl Into<String>, cost: f64, tokens: u32) {
        let operation_str = operation.into();
        let entry = LogEntry::new(
            LogLevel::Info,
            "cost_tracking",
            format!("Operation {} cost", operation_str),
        )
        .with_metadata("operation", operation_str)
        .with_metadata("cost_usd", format!("{:.6}", cost))
        .with_metadata("tokens", tokens.to_string());

        if self.should_log(LogLevel::Info) {
            self.output(&entry);
        }
        self.telemetry.record_cost(&entry);
    }

    /// Output the log entry (to stderr for errors, stdout for others)
    fn output(&self, entry: &LogEntry) {
        let formatted = entry.format();
        match entry.level {
            LogLevel::Error => eprintln!("{}", formatted),
            _ => println!("{}", formatted),
        }
    }

    /// Get telemetry data
    pub fn get_telemetry(&self) -> &super::telemetry::TelemetryCollector {
        &self.telemetry
    }
}

/// Security sanitization utilities
pub struct SecuritySanitizer;

impl SecuritySanitizer {
    /// Sanitize log messages to prevent sensitive data exposure
    pub fn sanitize_message(message: &str) -> String {
        let mut sanitized = message.to_string();

        // Remove API keys
        Self::redact_pattern(&mut sanitized, "sk-ant-", "[API_KEY_REDACTED]");
        Self::redact_pattern(&mut sanitized, "sk-", "[API_KEY_REDACTED]");

        // Remove file paths with sensitive locations
        for sensitive_path in &["/home/", "/Users/", "C:\\Users\\"] {
            Self::redact_file_path(&mut sanitized, sensitive_path);
        }

        // Truncate very long messages
        if sanitized.len() > crate::claude::constants::error_handling::MAX_ERROR_MESSAGE_LENGTH {
            format!(
                "{}...[TRUNCATED]",
                &sanitized
                    [..crate::claude::constants::error_handling::ERROR_MESSAGE_TRUNCATE_LENGTH]
            )
        } else {
            sanitized
        }
    }

    /// Sanitize metadata values based on key name
    pub fn sanitize_metadata_value(key: &str, value: &str) -> String {
        let key_lower = key.to_lowercase();

        // Redact sensitive keys
        if key_lower.contains("key")
            || key_lower.contains("secret")
            || key_lower.contains("token")
            || key_lower.contains("password")
        {
            return "[REDACTED]".to_string();
        }

        // Sanitize paths
        if key_lower.contains("path") || key_lower.contains("file") {
            return Self::sanitize_message(value);
        }

        // Truncate long values
        if value.len() > crate::claude::constants::error_handling::MAX_METADATA_VALUE_LENGTH {
            format!(
                "{}...[TRUNCATED]",
                &value[..crate::claude::constants::error_handling::METADATA_VALUE_TRUNCATE_LENGTH]
            )
        } else {
            value.to_string()
        }
    }

    /// Redact a specific pattern from the string
    fn redact_pattern(text: &mut String, pattern: &str, replacement: &str) {
        while let Some(start) = text.find(pattern) {
            let end = text[start..]
                .find(|c: char| {
                    c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == '}' || c == ']'
                })
                .map(|i| start + i)
                .unwrap_or(text.len());
            text.replace_range(start..end, replacement);
        }
    }

    /// Redact file paths starting with sensitive directories
    fn redact_file_path(text: &mut String, sensitive_path: &str) {
        while let Some(start) = text.find(sensitive_path) {
            // Find the end of the path (next whitespace or path separator)
            let search_from = start + sensitive_path.len();
            let end = text[search_from..]
                .find(|c: char| {
                    c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == '}' || c == ']'
                })
                .map(|i| search_from + i)
                .unwrap_or(text.len());
            text.replace_range(start..end, "/[USER_DIR_REDACTED]");
        }
    }
}
