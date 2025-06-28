/*!
 * Simple Structured Logging
 *
 * A lightweight logging system using the standard `log` crate with structured context.
 */

use log::{error, info, warn, debug};
use std::collections::HashMap;

/// Initialize the logger with environment variable support
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
    Ok(())
}

/// Log with structured context
pub fn log_with_context(level: log::Level, operation: &str, message: &str, context: Option<HashMap<String, String>>) {
    let context_str = if let Some(ctx) = context {
        if ctx.is_empty() {
            String::new()
        } else {
            format!(" [{}]", 
                ctx.iter()
                   .map(|(k, v)| format!("{}={}", k, sanitize_value(v)))
                   .collect::<Vec<_>>()
                   .join(", "))
        }
    } else {
        String::new()
    };

    let formatted_msg = format!("{} | {}{}", operation, message, context_str);
    
    match level {
        log::Level::Error => error!("{}", formatted_msg),
        log::Level::Warn => warn!("{}", formatted_msg),
        log::Level::Info => info!("{}", formatted_msg),
        log::Level::Debug => debug!("{}", formatted_msg),
        log::Level::Trace => debug!("{}", formatted_msg), // Map trace to debug
    }
}

/// Simple sanitization to protect sensitive data
pub fn sanitize_value(value: &str) -> String {
    if value.starts_with("sk-ant-") || 
       value.contains("key") || 
       value.contains("secret") || 
       value.contains("token") || 
       value.contains("password") {
        "[REDACTED]".to_string()
    } else if value.len() > 100 {
        format!("{}...[TRUNCATED]", &value[..97])
    } else {
        value.to_string()
    }
}

// Convenience macros for structured logging
#[macro_export]
macro_rules! log_error {
    ($operation:expr, $message:expr) => {
        $crate::logging::log_with_context(log::Level::Error, $operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::log_with_context(log::Level::Error, $operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($operation:expr, $message:expr) => {
        $crate::logging::log_with_context(log::Level::Warn, $operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::log_with_context(log::Level::Warn, $operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_info {
    ($operation:expr, $message:expr) => {
        $crate::logging::log_with_context(log::Level::Info, $operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::log_with_context(log::Level::Info, $operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($operation:expr, $message:expr) => {
        $crate::logging::log_with_context(log::Level::Debug, $operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::log_with_context(log::Level::Debug, $operation, $message, Some($context))
    };
}

/// Tool execution logging
#[macro_export]
macro_rules! log_tool_execution {
    ($tool:expr, $success:expr, $duration:expr) => {
        let status = if $success { "✓" } else { "✗" };
        $crate::log_info!("tool_execution", &format!("{} {} ({}ms)", status, $tool, $duration.as_millis()));
    };
    ($tool:expr, $success:expr, $duration:expr, $context:expr) => {
        let status = if $success { "✓" } else { "✗" };
        $crate::log_info!("tool_execution", &format!("{} {} ({}) ({}ms)", status, $tool, $context, $duration.as_millis()));
    };
}

/// Claude API call logging  
#[macro_export]
macro_rules! log_claude_api {
    ($model:expr, $tokens:expr, $cost:expr, $duration:expr) => {
        $crate::log_info!("claude_api", &format!("✓ {} ({} tokens, ${:.4}, {}ms)", 
            $model, $tokens, $cost, $duration.as_millis()));
    };
    ($model:expr, $tokens:expr, $cost:expr, $duration:expr, $message:expr) => {
        $crate::log_info!("claude_api", &format!("✓ {} ({} tokens, ${:.4}, {}ms) - {}", 
            $model, $tokens, $cost, $duration.as_millis(), $crate::logging::sanitize_value($message)));
    };
}

// sanitize_value is already public, no need to re-export

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_value() {
        assert_eq!(sanitize_value("sk-ant-12345"), "[REDACTED]");
        assert_eq!(sanitize_value("normal_value"), "normal_value");
        assert_eq!(sanitize_value("secret_key"), "[REDACTED]");
        
        let long_value = "a".repeat(150);
        let sanitized = sanitize_value(&long_value);
        assert!(sanitized.ends_with("...[TRUNCATED]"));
        assert!(sanitized.len() < long_value.len());
    }

    #[test]
    fn test_log_with_context() {
        // This just tests that the function doesn't panic
        let mut context = HashMap::new();
        context.insert("key".to_string(), "value".to_string());
        log_with_context(log::Level::Info, "test", "message", Some(context));
    }
}