/*!
 * Structured Logging System
 *
 * Centralized logging with configurable levels, security sanitization,
 * and consistent formatting for development, monitoring, and debugging.
 */

pub mod logger;
pub mod telemetry;

#[cfg(test)]
mod tests;

pub use logger::{LogEntry, LogLevel, Logger};

use std::sync::OnceLock;

/// Global logger instance
static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initialize the global logger with specified level
pub fn init_logger(level: LogLevel) -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new(level);
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| "Logger already initialized")?;
    Ok(())
}

/// Get the global logger instance
pub fn logger() -> &'static Logger {
    GLOBAL_LOGGER.get().unwrap_or_else(|| {
        // Fallback to default logger if not initialized
        let _ = GLOBAL_LOGGER.set(Logger::new(LogLevel::Info));
        GLOBAL_LOGGER.get().unwrap()
    })
}

/// Convenience macros for structured logging
#[macro_export]
macro_rules! log_error {
    ($operation:expr, $message:expr) => {
        $crate::logging::logger().error($operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::logger().error($operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($operation:expr, $message:expr) => {
        $crate::logging::logger().warn($operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::logger().warn($operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_info {
    ($operation:expr, $message:expr) => {
        $crate::logging::logger().info($operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::logger().info($operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($operation:expr, $message:expr) => {
        $crate::logging::logger().debug($operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::logger().debug($operation, $message, Some($context))
    };
}

#[macro_export]
macro_rules! log_trace {
    ($operation:expr, $message:expr) => {
        $crate::logging::logger().trace($operation, $message, None)
    };
    ($operation:expr, $message:expr, $context:expr) => {
        $crate::logging::logger().trace($operation, $message, Some($context))
    };
}

/// Tool execution logging
#[macro_export]
macro_rules! log_tool_execution {
    ($tool:expr, $result:expr, $duration:expr) => {
        $crate::logging::logger().log_tool_execution($tool, $result, $duration)
    };
}

/// Claude API call logging
#[macro_export]
macro_rules! log_claude_api {
    ($model:expr, $tokens:expr, $cost:expr, $duration:expr) => {
        $crate::logging::logger().log_claude_api($model, $tokens, $cost, $duration)
    };
}

/// Cost monitoring logging
#[macro_export]
macro_rules! log_cost {
    ($operation:expr, $cost:expr, $tokens:expr) => {
        $crate::logging::logger().log_cost($operation, $cost, $tokens)
    };
}
