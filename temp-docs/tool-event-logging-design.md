# Tool Event Logging System Design

## Overview

This document designs a comprehensive tool event logging system for the LLM Dev Agent's tool execution framework. The system provides structured, meaningful summaries for each tool type while handling large outputs efficiently and maintaining consistent logging across all tool operations.

## Current Tool Ecosystem Analysis

Based on the codebase analysis, the current tools are:

### Existing Tools
1. **ReadFileTool** - Read file contents from filesystem
2. **WriteFileTool** - Write content to filesystem files  
3. **ListDirectoryTool** - List directory contents

### Tool Execution Infrastructure
- **ToolExecutionContext**: Execution metadata, timing, retries
- **ToolExecutionResult**: Structured results with status, metadata, follow-up actions
- **ToolResultData**: Multiple data types (Text, Json, FileReference, DirectoryListing, Binary)
- **FeedbackManager**: Result processing and follow-up action generation
- **Performance tracking**: Execution time, memory usage, retry counts

## Tool-Specific Logging Schemas

### 1. ReadFileTool Logging Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFileLogEntry {
    // Standard execution info
    pub execution_id: String,
    pub tool_name: String,
    pub status: ToolExecutionStatus,
    pub execution_time: Duration,
    
    // Tool-specific parameters
    pub file_path: String,
    pub resolved_path: Option<String>, // After whitelist validation
    
    // Success scenario data
    pub file_size_bytes: Option<u64>,
    pub content_preview: Option<String>, // First 200 chars or truncated
    pub file_type: Option<String>, // Extension/MIME type
    pub encoding_detected: Option<String>,
    
    // Performance metrics
    pub read_throughput_mbps: Option<f64>, // MB/s
    pub cache_hit: Option<bool>, // If caching is implemented
    
    // Security/validation info
    pub whitelist_validation_time: Duration,
    pub security_warnings: Vec<String>,
    
    // Error scenario data
    pub error_type: Option<String>, // "not_found", "permission_denied", "too_large", etc.
    pub error_details: Option<String>,
    pub recovery_suggested: Option<bool>,
}
```

**Summary Strategy for ReadFileTool:**
- **Success**: "Read {file_size} bytes from '{filename}' in {time}ms"
- **Large files**: Show size + preview instead of full content
- **Errors**: "Failed to read '{filename}': {error_type}"
- **Performance**: Include throughput for large files

### 2. WriteFileTool Logging Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteFileLogEntry {
    // Standard execution info  
    pub execution_id: String,
    pub tool_name: String,
    pub status: ToolExecutionStatus,
    pub execution_time: Duration,
    
    // Tool-specific parameters
    pub file_path: String,
    pub resolved_path: Option<String>,
    pub content_size_bytes: u64,
    pub content_preview: Option<String>, // First/last few lines
    
    // File operation details
    pub file_existed: bool,
    pub backup_created: Option<bool>,
    pub bytes_written: Option<u64>,
    pub file_permissions: Option<String>,
    
    // Performance metrics
    pub write_throughput_mbps: Option<f64>,
    pub disk_sync_time: Option<Duration>,
    
    // Security/validation info
    pub whitelist_validation_time: Duration,
    pub protected_file_check: bool,
    pub content_validation_passed: bool,
    pub security_warnings: Vec<String>,
    
    // Error scenario data
    pub error_type: Option<String>, // "permission_denied", "disk_full", "path_invalid", etc.
    pub error_details: Option<String>,
    pub partial_write_bytes: Option<u64>, // If write partially succeeded
}
```

**Summary Strategy for WriteFileTool:**
- **Success**: "Wrote {bytes} bytes to '{filename}' in {time}ms {(overwrite/new)}"
- **Large content**: Show size + content type instead of full content
- **Errors**: "Failed to write '{filename}': {error_type}"
- **Important**: Flag overwrites of existing files

### 3. ListDirectoryTool Logging Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDirectoryLogEntry {
    // Standard execution info
    pub execution_id: String,
    pub tool_name: String, 
    pub status: ToolExecutionStatus,
    pub execution_time: Duration,
    
    // Tool-specific parameters
    pub directory_path: String,
    pub resolved_path: Option<String>,
    
    // Directory listing results
    pub total_entries: usize,
    pub file_count: usize,
    pub directory_count: usize,
    pub symlink_count: usize,
    pub hidden_files_skipped: usize,
    
    // Content summary
    pub file_types_found: HashMap<String, usize>, // extension -> count
    pub largest_file_size: Option<u64>,
    pub total_size_bytes: Option<u64>,
    pub entry_preview: Vec<String>, // First few entries for context
    
    // Performance metrics
    pub entries_per_second: f64,
    pub listing_truncated: bool,
    pub truncation_limit: Option<usize>,
    
    // Security/validation info
    pub whitelist_validation_time: Duration,
    pub access_warnings: Vec<String>,
    
    // Error scenario data
    pub error_type: Option<String>, // "not_found", "permission_denied", "not_directory", etc.
    pub error_details: Option<String>,
    pub partial_listing_count: Option<usize>, // If partially succeeded
}
```

**Summary Strategy for ListDirectoryTool:**
- **Success**: "Listed {count} entries in '{dirname}' ({files}F, {dirs}D) in {time}ms"
- **Large directories**: Show counts + preview instead of full listing
- **Errors**: "Failed to list '{dirname}': {error_type}"
- **Performance**: Include entries/second for large directories

## Generic Tool Event Schema

For consistency across all tools and extensibility:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEventLog {
    // Core execution metadata
    pub timestamp: DateTime<Utc>,
    pub execution_id: String,
    pub tool_name: String,
    pub status: ToolExecutionStatus,
    pub execution_time: Duration,
    
    // Input parameters (sanitized)
    pub input_summary: HashMap<String, String>, // Key -> sanitized value summary
    pub input_size_bytes: u64,
    
    // Output results (managed size)
    pub output_type: String, // "text", "json", "file_reference", etc.
    pub output_size_bytes: u64,
    pub output_summary: String, // Tool-specific meaningful summary
    pub output_preview: Option<String>, // Truncated preview if needed
    
    // Performance metrics
    pub cpu_time: Option<Duration>,
    pub memory_peak_bytes: Option<u64>,
    pub io_operations: Option<IoMetrics>,
    
    // Context information
    pub retry_count: u32,
    pub parent_execution_id: Option<String>,
    pub whitelist_validation_time: Duration,
    
    // Error handling
    pub error_context: Option<ErrorLogContext>,
    pub recovery_actions_taken: Vec<String>,
    pub follow_up_actions_generated: usize,
    
    // Tool-specific data
    pub tool_specific_data: Option<serde_json::Value>, // Flexible extension point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoMetrics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u32,
    pub write_operations: u32,
    pub filesystem_sync_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLogContext {
    pub error_type: String,
    pub error_category: String, // "validation", "execution", "permission", "timeout", etc.
    pub is_recoverable: bool,
    pub error_message: String, // Sanitized error message
    pub suggestions: Vec<String>,
    pub system_context: Option<String>, // OS error codes, filesystem state, etc.
}
```

## Output Size Management Strategies

### Large Content Handling

1. **Text Content (ReadFileTool)**
   ```rust
   pub fn create_content_preview(content: &str, max_chars: usize) -> String {
       if content.len() <= max_chars {
           content.to_string()
       } else {
           let preview_size = max_chars / 2;
           format!(
               "{}...\n[{} bytes truncated]\n...{}",
               &content[..preview_size],
               content.len() - max_chars,
               &content[content.len().saturating_sub(preview_size)..]
           )
       }
   }
   ```

2. **Directory Listings (ListDirectoryTool)**
   ```rust
   pub fn create_listing_summary(entries: &[FileItem]) -> String {
       let total = entries.len();
       if total <= 20 {
           // Show all entries
           entries.iter().map(|e| e.display()).collect::<Vec<_>>().join("\n")
       } else {
           // Show first 10, summary, last 5
           let first_10: String = entries[..10].iter()
               .map(|e| e.display()).collect::<Vec<_>>().join("\n");
           let last_5: String = entries[total-5..].iter()
               .map(|e| e.display()).collect::<Vec<_>>().join("\n");
           
           format!("{}\n... [{} entries omitted] ...\n{}", 
                   first_10, total - 15, last_5)
       }
   }
   ```

3. **Binary Content (Future tools)**
   ```rust
   pub fn create_binary_summary(data: &[u8], content_type: &str) -> String {
       format!(
           "{} binary data ({} bytes)\nMagic bytes: {:02X?}\nContent hash: {}",
           content_type,
           data.len(),
           &data[..std::cmp::min(8, data.len())],
           hash_bytes(data)
       )
   }
   ```

## Timing Information Design

### Granular Timing Breakdown

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedTiming {
    pub total_execution_time: Duration,
    pub validation_time: Duration, // Whitelist, security checks
    pub io_time: Duration, // Actual file system operations
    pub processing_time: Duration, // Content processing, parsing
    pub serialization_time: Duration, // Result formatting
    
    // Context switching and overhead
    pub overhead_time: Duration,
    pub waiting_time: Duration, // Time spent waiting for locks, etc.
}
```

### Performance Baselines

```rust
pub mod performance_thresholds {
    use std::time::Duration;
    
    // ReadFileTool thresholds
    pub const READ_FAST_THRESHOLD: Duration = Duration::from_millis(10);
    pub const READ_SLOW_THRESHOLD: Duration = Duration::from_millis(100);
    
    // WriteFileTool thresholds  
    pub const WRITE_FAST_THRESHOLD: Duration = Duration::from_millis(20);
    pub const WRITE_SLOW_THRESHOLD: Duration = Duration::from_millis(200);
    
    // ListDirectoryTool thresholds
    pub const LIST_FAST_THRESHOLD: Duration = Duration::from_millis(5);
    pub const LIST_SLOW_THRESHOLD: Duration = Duration::from_millis(50);
    
    pub fn classify_performance(duration: Duration, tool_name: &str) -> &'static str {
        let (fast, slow) = match tool_name {
            "read_file" => (READ_FAST_THRESHOLD, READ_SLOW_THRESHOLD),
            "write_file" => (WRITE_FAST_THRESHOLD, WRITE_SLOW_THRESHOLD),
            "list_directory" => (LIST_FAST_THRESHOLD, LIST_SLOW_THRESHOLD),
            _ => (Duration::from_millis(10), Duration::from_millis(100)),
        };
        
        if duration <= fast {
            "fast"
        } else if duration <= slow {
            "normal"
        } else {
            "slow"
        }
    }
}
```

## Success vs Failure Logging

### Success Scenarios

**ReadFileTool Success:**
```json
{
  "status": "success",
  "summary": "Read 2.3KB from 'src/main.rs' in 8ms",
  "details": {
    "file_size": 2347,
    "file_type": "rust",
    "encoding": "utf-8",
    "performance": "fast"
  }
}
```

**WriteFileTool Success:**
```json
{
  "status": "success", 
  "summary": "Wrote 1.5KB to 'output.json' (new file) in 12ms",
  "details": {
    "bytes_written": 1534,
    "file_operation": "create",
    "sync_required": true,
    "performance": "normal"
  }
}
```

**ListDirectoryTool Success:**
```json
{
  "status": "success",
  "summary": "Listed 47 entries in 'src/' (32F, 15D) in 6ms", 
  "details": {
    "total_entries": 47,
    "file_types": {"rs": 28, "toml": 2, "md": 2},
    "total_size": "156KB",
    "performance": "fast"
  }
}
```

### Failure Scenarios

**ReadFileTool Failures:**
```json
{
  "status": "failed",
  "summary": "Failed to read 'missing.txt': file not found",
  "error": {
    "type": "not_found",
    "category": "execution", 
    "recoverable": false,
    "suggestions": ["Verify file path", "Check file permissions"]
  }
}
```

**WriteFileTool Failures:**
```json
{
  "status": "failed",
  "summary": "Failed to write 'protected.rs': access denied (protected file)",
  "error": {
    "type": "permission_denied",
    "category": "validation",
    "recoverable": false,
    "suggestions": ["File is in protected list", "Choose different filename"]
  }
}
```

## Implementation Architecture

### Logging Pipeline

```rust
pub struct ToolEventLogger {
    storage: Box<dyn EventStorage>,
    formatters: HashMap<String, Box<dyn LogFormatter>>,
    filters: Vec<Box<dyn LogFilter>>,
    config: LoggingConfig,
}

impl ToolEventLogger {
    pub async fn log_tool_execution(
        &self,
        context: &ToolExecutionContext,
        result: &ToolExecutionResult,
        detailed_metrics: Option<DetailedMetrics>,
    ) -> Result<(), LoggingError> {
        // Create base log entry
        let mut log_entry = ToolEventLog::from_execution(context, result);
        
        // Add tool-specific data
        if let Some(formatter) = self.formatters.get(&context.tool_name) {
            log_entry.tool_specific_data = Some(
                formatter.format_tool_data(context, result)?
            );
            log_entry.output_summary = formatter.create_summary(result);
        }
        
        // Add detailed metrics if available
        if let Some(metrics) = detailed_metrics {
            log_entry.cpu_time = Some(metrics.cpu_time);
            log_entry.memory_peak_bytes = Some(metrics.memory_peak);
            log_entry.io_operations = Some(metrics.io_metrics);
        }
        
        // Apply filters
        for filter in &self.filters {
            if !filter.should_log(&log_entry) {
                return Ok(()); // Skip logging
            }
            filter.sanitize(&mut log_entry);
        }
        
        // Store the log entry
        self.storage.store(log_entry).await
    }
}
```

### Configuration

```rust
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    // Output size limits
    pub max_output_preview_chars: usize, // Default: 500
    pub max_input_summary_chars: usize,  // Default: 200
    pub max_error_message_chars: usize,  // Default: 1000
    
    // Performance tracking
    pub enable_detailed_timing: bool,    // Default: true
    pub enable_memory_tracking: bool,    // Default: false (expensive)
    pub enable_io_metrics: bool,         // Default: true
    
    // Storage settings
    pub retention_days: u32,             // Default: 30
    pub max_entries_per_day: usize,      // Default: 10000
    pub compress_old_logs: bool,         // Default: true
    
    // Privacy and security
    pub sanitize_file_paths: bool,       // Default: true
    pub log_content_previews: bool,      // Default: true 
    pub redact_sensitive_patterns: bool, // Default: true
}
```

## Integration Points

### 1. Tool Execution Pipeline Integration

```rust
// In ToolExecutionEngine::execute_single_tool
let logger = self.event_logger.as_ref();

// Start timing
let detailed_metrics = if self.config.enable_detailed_metrics {
    Some(DetailedMetrics::start())
} else {
    None
};

// Execute tool (existing code)
let result = tool.execute(context.input.clone()).await;

// Finalize metrics
if let Some(mut metrics) = detailed_metrics {
    metrics.finish();
    
    // Log the execution
    if let Some(logger) = logger {
        logger.log_tool_execution(&context, &result, Some(metrics)).await?;
    }
}
```

### 2. Feedback Manager Integration

```rust
// In FeedbackManager::process_result
impl FeedbackManager {
    pub async fn process_result(&self, result: &ToolExecutionResult) -> ClaudeResult<Vec<FollowUpAction>> {
        // Existing processing logic...
        
        // Generate logging follow-up action if needed
        if self.should_create_log_summary(result) {
            actions.push(FollowUpAction::ReportStatus {
                message: self.create_execution_summary(result),
                level: if result.is_success() { 
                    StatusLevel::Info 
                } else { 
                    StatusLevel::Warning 
                },
            });
        }
        
        Ok(actions)
    }
}
```

### 3. Constants Integration

Add to `src-tauri/src/config/constants.rs`:

```rust
// ============================================================================
// TOOL LOGGING CONSTANTS
// ============================================================================

/// Maximum characters for tool output preview in logs
pub const TOOL_LOG_OUTPUT_PREVIEW_CHARS: usize = 500;

/// Maximum characters for tool input summary in logs  
pub const TOOL_LOG_INPUT_SUMMARY_CHARS: usize = 200;

/// Maximum characters for error messages in logs
pub const TOOL_LOG_ERROR_MESSAGE_CHARS: usize = 1000;

/// Performance classification thresholds (milliseconds)
pub const TOOL_PERFORMANCE_FAST_MS: u64 = 10;
pub const TOOL_PERFORMANCE_SLOW_MS: u64 = 100;

/// Maximum log entries to keep per day
pub const TOOL_LOG_MAX_ENTRIES_PER_DAY: usize = 10000;

/// Log retention period in days
pub const TOOL_LOG_RETENTION_DAYS: u32 = 30;
```

## Benefits of This Design

1. **Consistent Structure**: All tools use the same base logging schema
2. **Tool-Specific Insights**: Each tool type captures relevant operational details
3. **Performance Awareness**: Built-in performance classification and metrics
4. **Size Management**: Intelligent truncation prevents log bloat
5. **Security Conscious**: Sanitization and redaction of sensitive information
6. **Extensible**: Easy to add new tools with their specific logging needs
7. **Integration Ready**: Hooks into existing execution and feedback systems
8. **Operational Value**: Provides actionable insights for debugging and optimization

This logging system provides the right level of detail for each tool type while maintaining consistency and preventing overwhelming output sizes.