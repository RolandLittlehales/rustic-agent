/*!
 * Tests for the logging system
 */

use super::*;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_log_level_parsing() {
    assert_eq!("ERROR".parse::<LogLevel>().unwrap(), LogLevel::Error);
    assert_eq!("WARN".parse::<LogLevel>().unwrap(), LogLevel::Warn);
    assert_eq!("WARNING".parse::<LogLevel>().unwrap(), LogLevel::Warn);
    assert_eq!("INFO".parse::<LogLevel>().unwrap(), LogLevel::Info);
    assert_eq!("DEBUG".parse::<LogLevel>().unwrap(), LogLevel::Debug);
    assert_eq!("TRACE".parse::<LogLevel>().unwrap(), LogLevel::Trace);
    
    assert!("INVALID".parse::<LogLevel>().is_err());
}

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Error > LogLevel::Warn);
    assert!(LogLevel::Warn > LogLevel::Info);
    assert!(LogLevel::Info > LogLevel::Debug);
    assert!(LogLevel::Debug > LogLevel::Trace);
}

#[test]
fn test_log_entry_creation() {
    let entry = LogEntry::new(LogLevel::Info, "test_operation", "test message");
    
    assert_eq!(entry.level, LogLevel::Info);
    assert_eq!(entry.operation, "test_operation");
    assert_eq!(entry.message, "test message");
    assert!(entry.metadata.is_empty());
    assert!(entry.duration.is_none());
}

#[test]
fn test_log_entry_with_metadata() {
    let entry = LogEntry::new(LogLevel::Error, "api_call", "Request failed")
        .with_metadata("status_code", "500")
        .with_metadata("endpoint", "/api/test")
        .with_duration(Duration::from_millis(150));
    
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.metadata.get("status_code").unwrap(), "500");
    assert_eq!(entry.metadata.get("endpoint").unwrap(), "/api/test");
    assert_eq!(entry.duration, Some(Duration::from_millis(150)));
}

#[test]
fn test_logger_level_filtering() {
    let logger = Logger::new(LogLevel::Warn);
    
    // Should log
    assert!(logger.should_log(LogLevel::Error));
    assert!(logger.should_log(LogLevel::Warn));
    
    // Should not log
    assert!(!logger.should_log(LogLevel::Info));
    assert!(!logger.should_log(LogLevel::Debug));
    assert!(!logger.should_log(LogLevel::Trace));
}

#[test]
fn test_security_sanitizer_api_keys() {
    // Test API key redaction with proper formatting
    let sensitive = "Using API key sk-ant-api03_UlOFTawJt5Lj09zH8FwcX7P3jQ2kFRz for authentication";
    let sanitized = logger::SecuritySanitizer::sanitize_message(sensitive);
    assert!(!sanitized.contains("sk-ant-api03_UlOFTawJt5Lj09zH8FwcX7P3jQ2kFRz"));
    assert!(sanitized.contains("sk-...FRz"));
    
    // Test multiple API keys
    let multiple = "Key1: sk-ant-111222333 and Key2: sk-ant-aaabbbccc";
    let sanitized = logger::SecuritySanitizer::sanitize_message(multiple);
    assert!(!sanitized.contains("sk-ant-111222333"));
    assert!(!sanitized.contains("sk-ant-aaabbbccc"));
    assert!(sanitized.contains("sk-...333"));
    assert!(sanitized.contains("sk-...ccc"));
}

#[test]
fn test_security_sanitizer_preserves_file_paths() {
    // Test that file paths are preserved (not redacted)
    let file_paths = vec![
        "/home/user/project/config.json",
        "/Users/admin/app/settings.toml",
        "C:\\Users\\test\\project\\file.dat",
        "target/debug/app.exe",
        "./src/main.rs",
    ];
    
    for path in file_paths {
        let sanitized = logger::SecuritySanitizer::sanitize_message(path);
        assert_eq!(sanitized, path); // Should be unchanged
    }
}

#[test]
fn test_security_sanitizer_metadata() {
    // Test actual sensitive metadata keys
    let api_key_value = "sk-ant-12345abcdefg";
    let sanitized = logger::SecuritySanitizer::sanitize_metadata_value("api_key", api_key_value);
    assert!(sanitized.contains("sk-...efg"));
    assert!(!sanitized.contains("12345abcdefg"));
    
    // Test that common logging words are NOT redacted
    let safe_keys = vec![
        ("tokens", "1500"),
        ("cost", "$0.0045"), 
        ("model", "claude-4-sonnet"),
        ("operation", "read_file"),
        ("tool", "list_directory"),
        ("result", "success"),
        ("duration_ms", "123"),
    ];
    
    for (key, value) in safe_keys {
        let sanitized = logger::SecuritySanitizer::sanitize_metadata_value(key, value);
        assert_eq!(sanitized, value); // Should be unchanged
    }
}

#[test]
fn test_security_sanitizer_long_messages() {
    // Test message truncation
    let long_message = "x".repeat(1000);
    let sanitized = logger::SecuritySanitizer::sanitize_message(&long_message);
    
    assert!(sanitized.len() < long_message.len());
    assert!(sanitized.contains("...[TRUNCATED]"));
}

#[test]
fn test_telemetry_collector() {
    let collector = telemetry::TelemetryCollector::new();
    
    // Test log recording
    let entry = LogEntry::new(LogLevel::Error, "test", "test message");
    collector.record_log(&entry);
    
    let metrics = collector.get_log_metrics();
    assert_eq!(metrics.total_logs, 1);
    assert_eq!(metrics.error_count, 1);
    assert_eq!(metrics.warn_count, 0);
    assert!(metrics.error_rate > 0.0);
    
    // Test multiple log levels
    let warn_entry = LogEntry::new(LogLevel::Warn, "test", "warning");
    collector.record_log(&warn_entry);
    
    let updated_metrics = collector.get_log_metrics();
    assert_eq!(updated_metrics.total_logs, 2);
    assert_eq!(updated_metrics.error_count, 1);
    assert_eq!(updated_metrics.warn_count, 1);
    assert_eq!(updated_metrics.error_rate, 0.5);
}

#[test]
fn test_telemetry_tool_execution() {
    let collector = telemetry::TelemetryCollector::new();
    
    let entry = LogEntry::new(LogLevel::Info, "tool_execution", "Tool executed")
        .with_metadata("tool", "test_tool")
        .with_metadata("result", "success")
        .with_duration(Duration::from_millis(100));
    
    collector.record_tool_execution(&entry);
    
    let tool_metrics = collector.get_tool_metrics();
    assert!(tool_metrics.contains_key("test_tool"));
    
    let metric = tool_metrics.get("test_tool").unwrap();
    assert_eq!(metric.executions, 1);
    assert_eq!(metric.successes, 1);
    assert_eq!(metric.failures, 0);
    assert_eq!(metric.success_rate, 1.0);
}

#[test]
fn test_global_logger_singleton() {
    // Test that we can get a logger instance
    let logger = logger();
    assert!(logger.should_log(LogLevel::Info)); // Default level
    
    // Test initialization
    let result = init_logger(LogLevel::Debug);
    // Should fail because logger is already initialized
    assert!(result.is_err());
}

#[test]
fn test_log_entry_formatting() {
    let entry = LogEntry::new(LogLevel::Info, "test_op", "Test message")
        .with_metadata("key1", "value1")
        .with_metadata("key2", "value2")
        .with_duration(Duration::from_millis(42));
    
    let formatted = entry.format();
    
    // Should contain all components in new format
    assert!(formatted.contains("✓"));
    assert!(formatted.contains("Test message"));
    assert!(formatted.contains("[42ms]"));
}

#[test]
fn test_telemetry_report_generation() {
    let collector = telemetry::TelemetryCollector::new();
    
    // Add some test data
    let log_entry = LogEntry::new(LogLevel::Info, "test", "message");
    collector.record_log(&log_entry);
    
    let tool_entry = LogEntry::new(LogLevel::Info, "tool_execution", "executed")
        .with_metadata("tool", "test_tool")
        .with_metadata("result", "success")
        .with_duration(Duration::from_millis(100));
    collector.record_tool_execution(&tool_entry);
    
    let report = collector.get_summary_report();
    
    assert_eq!(report.log_metrics.total_logs, 1);
    assert!(report.tool_metrics.contains_key("test_tool"));
    
    let summary = report.format_summary();
    assert!(summary.contains("📊 Telemetry Report"));
    assert!(summary.contains("Total logs: 1"));
    assert!(summary.contains("test_tool"));
}