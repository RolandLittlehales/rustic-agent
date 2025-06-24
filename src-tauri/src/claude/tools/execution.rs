use crate::claude::{
    error::{ClaudeError, ErrorContext},
    whitelist::WhitelistConfig,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Execution context for tool operations with comprehensive metadata
#[derive(Debug, Clone)]
pub struct ToolExecutionContext {
    pub execution_id: String,
    pub tool_name: String,
    pub input: serde_json::Value,
    pub whitelist: Arc<RwLock<WhitelistConfig>>,
    pub metadata: ToolExecutionMetadata,
    pub parent_execution: Option<String>,
}

impl ToolExecutionContext {
    pub fn new(
        tool_name: String,
        input: serde_json::Value,
        whitelist: Arc<RwLock<WhitelistConfig>>,
    ) -> Self {
        Self {
            execution_id: Self::generate_id(),
            tool_name,
            input,
            whitelist,
            metadata: ToolExecutionMetadata::default(),
            parent_execution: None,
        }
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_execution = Some(parent_id);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.metadata.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.metadata.max_retries = max_retries;
        self
    }

    fn generate_id() -> String {
        use uuid::Uuid;
        format!("exec_{}", Uuid::new_v4().simple())
    }

    pub fn increment_retry(&mut self) {
        self.metadata.retry_count += 1;
    }

    pub fn can_retry(&self) -> bool {
        self.metadata.retry_count < self.metadata.max_retries
    }

    pub fn elapsed(&self) -> Duration {
        self.metadata.started_at.elapsed()
    }

    pub fn is_timeout(&self) -> bool {
        self.elapsed() > self.metadata.timeout
    }
}

/// Metadata for tool execution tracking
#[derive(Debug, Clone)]
pub struct ToolExecutionMetadata {
    pub started_at: std::time::Instant,
    pub timeout: Duration,
    pub retry_count: u32,
    pub max_retries: u32,
    pub creation_time: DateTime<Utc>,
}

impl Default for ToolExecutionMetadata {
    fn default() -> Self {
        Self {
            started_at: std::time::Instant::now(),
            timeout: Duration::from_secs(30), // Default 30 second timeout
            retry_count: 0,
            max_retries: 3, // Default max 3 retries
            creation_time: Utc::now(),
        }
    }
}

/// Comprehensive tool execution result with structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    pub execution_id: String,
    pub tool_name: String,
    pub status: ToolExecutionStatus,
    pub result: ToolResultData,
    pub metadata: ToolResultMetadata,
    pub follow_up_actions: Vec<FollowUpAction>,
    pub error_context: Option<String>,
}

impl ToolExecutionResult {
    pub fn success(
        execution_id: String,
        tool_name: String,
        result: ToolResultData,
    ) -> Self {
        Self {
            execution_id,
            tool_name,
            status: ToolExecutionStatus::Success,
            result,
            metadata: ToolResultMetadata::default(),
            follow_up_actions: Vec::new(),
            error_context: None,
        }
    }

    pub fn failure(
        execution_id: String,
        tool_name: String,
        error: ToolError,
        recoverable: bool,
    ) -> Self {
        Self {
            execution_id,
            tool_name,
            status: ToolExecutionStatus::Failed { error: error.clone(), recoverable },
            result: ToolResultData::Text(format!("Tool execution failed: {}", error.message)),
            metadata: ToolResultMetadata::default(),
            follow_up_actions: Vec::new(),
            error_context: Some(error.details.unwrap_or_default()),
        }
    }

    pub fn partial_success(
        execution_id: String,
        tool_name: String,
        result: ToolResultData,
        warnings: Vec<String>,
    ) -> Self {
        Self {
            execution_id,
            tool_name,
            status: ToolExecutionStatus::PartialSuccess { warnings },
            result,
            metadata: ToolResultMetadata::default(),
            follow_up_actions: Vec::new(),
            error_context: None,
        }
    }

    pub fn timeout(execution_id: String, tool_name: String, duration: Duration) -> Self {
        Self {
            execution_id,
            tool_name,
            status: ToolExecutionStatus::Timeout,
            result: ToolResultData::Text(format!("Tool execution timed out after {:?}", duration)),
            metadata: ToolResultMetadata::default(),
            follow_up_actions: Vec::new(),
            error_context: Some(format!("Timeout after {:?}", duration)),
        }
    }

    pub fn with_follow_up_actions(mut self, actions: Vec<FollowUpAction>) -> Self {
        self.follow_up_actions = actions;
        self
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, ToolExecutionStatus::Success | ToolExecutionStatus::PartialSuccess { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, ToolExecutionStatus::Failed { .. } | ToolExecutionStatus::Timeout | ToolExecutionStatus::Cancelled)
    }

    pub fn is_recoverable(&self) -> bool {
        match &self.status {
            ToolExecutionStatus::Failed { recoverable, .. } => *recoverable,
            ToolExecutionStatus::Timeout => true,
            _ => false,
        }
    }

    /// Convert to a content block for Claude API integration
    pub fn into_content_block(&self) -> String {
        match &self.result {
            ToolResultData::Text(text) => text.clone(),
            ToolResultData::Json(value) => serde_json::to_string_pretty(value).unwrap_or_default(),
            ToolResultData::FileReference { path, size, hash } => {
                let hash_info = hash.as_ref().map(|h| format!(" (hash: {})", h)).unwrap_or_default();
                format!("File reference: {} ({} bytes){}", path, size, hash_info)
            }
            ToolResultData::DirectoryListing(items) => {
                format!("Directory listing ({} items):\n{}", 
                    items.len(),
                    items.iter().map(|item| format!("  {}", item.display())).collect::<Vec<_>>().join("\n")
                )
            }
            ToolResultData::Binary { content_type, size, preview } => {
                let preview_info = preview.as_ref().map(|p| format!(": {}", p)).unwrap_or_default();
                format!("Binary data: {} ({} bytes){}", content_type, size, preview_info)
            }
        }
    }
}

/// Execution status with detailed error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolExecutionStatus {
    Success,
    PartialSuccess { warnings: Vec<String> },
    Failed { error: ToolError, recoverable: bool },
    Timeout,
    Cancelled,
}

impl ToolExecutionStatus {
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Failed { .. } | Self::Timeout | Self::Cancelled)
    }
}

/// Structured result data supporting multiple data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolResultData {
    Text(String),
    Json(serde_json::Value),
    FileReference { path: String, size: u64, hash: Option<String> },
    DirectoryListing(Vec<FileItem>),
    Binary { content_type: String, size: u64, preview: Option<String> },
}

impl ToolResultData {
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    pub fn json(value: serde_json::Value) -> Self {
        Self::Json(value)
    }

    pub fn file_reference(path: impl Into<String>, size: u64) -> Self {
        Self::FileReference {
            path: path.into(),
            size,
            hash: None,
        }
    }

    pub fn file_reference_with_hash(path: impl Into<String>, size: u64, hash: String) -> Self {
        Self::FileReference {
            path: path.into(),
            size,
            hash: Some(hash),
        }
    }

    pub fn directory_listing(items: Vec<FileItem>) -> Self {
        Self::DirectoryListing(items)
    }

    pub fn binary(content_type: impl Into<String>, size: u64) -> Self {
        Self::Binary {
            content_type: content_type.into(),
            size,
            preview: None,
        }
    }

    pub fn binary_with_preview(content_type: impl Into<String>, size: u64, preview: String) -> Self {
        Self::Binary {
            content_type: content_type.into(),
            size,
            preview: Some(preview),
        }
    }
}

/// File item for directory listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub item_type: FileItemType,
    pub size: Option<u64>,
    pub modified: Option<DateTime<Utc>>,
    pub permissions: Option<String>,
}

impl FileItem {
    pub fn new(name: String, item_type: FileItemType) -> Self {
        Self {
            name,
            item_type,
            size: None,
            modified: None,
            permissions: None,
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_modified(mut self, modified: DateTime<Utc>) -> Self {
        self.modified = Some(modified);
        self
    }

    pub fn with_permissions(mut self, permissions: String) -> Self {
        self.permissions = Some(permissions);
        self
    }

    pub fn display(&self) -> String {
        let type_indicator = match self.item_type {
            FileItemType::File => "",
            FileItemType::Directory => "/",
            FileItemType::Symlink => "@",
            FileItemType::Unknown => "?",
        };
        
        let size_info = self.size
            .map(|s| format!(" ({} bytes)", s))
            .unwrap_or_default();
            
        format!("{}{}{}", self.name, type_indicator, size_info)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileItemType {
    File,
    Directory,
    Symlink,
    Unknown,
}

/// Result metadata for performance tracking and analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResultMetadata {
    pub execution_time: Duration,
    pub memory_used: Option<u64>,
    pub warnings: Vec<String>,
    pub performance_hints: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Default for ToolResultMetadata {
    fn default() -> Self {
        Self {
            execution_time: Duration::from_millis(0),
            memory_used: None,
            warnings: Vec::new(),
            performance_hints: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

impl ToolResultMetadata {
    pub fn with_execution_time(mut self, duration: Duration) -> Self {
        self.execution_time = duration;
        self
    }

    pub fn with_memory_used(mut self, bytes: u64) -> Self {
        self.memory_used = Some(bytes);
        self
    }

    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn add_performance_hint(mut self, hint: String) -> Self {
        self.performance_hints.push(hint);
        self
    }
}

/// Actions to be taken after tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FollowUpAction {
    ExecuteTool {
        tool_name: String,
        input: serde_json::Value,
        depends_on: Option<String>,
    },
    RetryWithModifiedInput {
        modified_input: serde_json::Value,
        delay: Option<Duration>,
    },
    RequestUserInput {
        prompt: String,
        suggested_actions: Vec<String>,
    },
    ReportStatus {
        message: String,
        level: StatusLevel,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusLevel {
    Info,
    Warning,  
    Error,
}

/// Tool-specific error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolError {
    pub error_type: ToolErrorType,
    pub message: String,
    pub details: Option<String>,
    pub recovery_suggestions: Vec<String>,
}

impl ToolError {
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ToolErrorType::ValidationError,
            message: message.into(),
            details: None,
            recovery_suggestions: vec!["Check input parameters".to_string()],
        }
    }

    pub fn execution_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ToolErrorType::ExecutionError,
            message: message.into(),
            details: None,
            recovery_suggestions: vec!["Retry operation".to_string()],
        }
    }

    pub fn permission_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ToolErrorType::PermissionError,
            message: message.into(),
            details: None,
            recovery_suggestions: vec!["Check file permissions or whitelist configuration".to_string()],
        }
    }

    pub fn timeout_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ToolErrorType::TimeoutError,
            message: message.into(),
            details: None,
            recovery_suggestions: vec!["Increase timeout or optimize operation".to_string()],
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_recovery_suggestion(mut self, suggestion: String) -> Self {
        self.recovery_suggestions.push(suggestion);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolErrorType {
    ValidationError,
    ExecutionError,
    PermissionError,
    TimeoutError,
    NetworkError,
    ResourceError,
}

/// Convert ToolError to ClaudeError for consistency
impl From<ToolError> for ClaudeError {
    fn from(tool_error: ToolError) -> Self {
        ClaudeError::ToolError {
            tool_name: "unknown".to_string(),
            message: tool_error.message,
            context: tool_error.details.map(|_details| {
                ErrorContext::new("tool_execution").add_metadata("error_type", &format!("{:?}", tool_error.error_type))
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn create_test_whitelist() -> Arc<RwLock<WhitelistConfig>> {
        Arc::new(RwLock::new(WhitelistConfig::default()))
    }

    #[test]
    fn test_execution_context_creation() {
        let whitelist = create_test_whitelist();
        let input = serde_json::json!({"path": "test.txt"});
        let context = ToolExecutionContext::new("read_file".to_string(), input.clone(), whitelist);
        
        assert_eq!(context.tool_name, "read_file");
        assert_eq!(context.input, input);
        assert!(context.execution_id.starts_with("exec_"));
        assert!(context.can_retry());
        assert!(!context.is_timeout());
    }

    #[test]
    fn test_execution_context_retry_logic() {
        let whitelist = create_test_whitelist();
        let input = serde_json::json!({});
        let mut context = ToolExecutionContext::new("test_tool".to_string(), input, whitelist)
            .with_max_retries(2);
        
        assert!(context.can_retry());
        context.increment_retry();
        assert!(context.can_retry());
        context.increment_retry();
        assert!(!context.can_retry());
    }

    #[test]
    fn test_tool_execution_result_creation() {
        let result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "read_file".to_string(),
            ToolResultData::text("File contents"),
        );
        
        assert!(result.is_success());
        assert!(!result.is_error());
        assert_eq!(result.tool_name, "read_file");
        assert_eq!(result.execution_id, "exec_123");
    }

    #[test]
    fn test_tool_result_data_variants() {
        let text_result = ToolResultData::text("Hello world");
        let json_result = ToolResultData::json(serde_json::json!({"key": "value"}));
        let file_result = ToolResultData::file_reference("/path/to/file", 1024);
        
        match text_result {
            ToolResultData::Text(content) => assert_eq!(content, "Hello world"),
            _ => panic!("Expected text result"),
        }
        
        match json_result {
            ToolResultData::Json(value) => assert_eq!(value["key"], "value"),
            _ => panic!("Expected JSON result"),
        }
        
        match file_result {
            ToolResultData::FileReference { path, size, .. } => {
                assert_eq!(path, "/path/to/file");
                assert_eq!(size, 1024);
            }
            _ => panic!("Expected file reference result"),
        }
    }

    #[test]
    fn test_file_item_creation() {
        let file_item = FileItem::new("test.txt".to_string(), FileItemType::File)
            .with_size(1024)
            .with_permissions("rw-r--r--".to_string());
        
        assert_eq!(file_item.name, "test.txt");
        assert_eq!(file_item.size, Some(1024));
        assert!(file_item.permissions.is_some());
        
        let display = file_item.display();
        assert!(display.contains("test.txt"));
        assert!(display.contains("1024 bytes"));
    }

    #[test]
    fn test_tool_error_creation() {
        let error = ToolError::validation_error("Invalid input parameter")
            .with_details("Parameter 'path' cannot be empty".to_string())
            .with_recovery_suggestion("Provide a valid file path".to_string());
        
        assert_eq!(error.message, "Invalid input parameter");
        assert!(error.details.is_some());
        assert_eq!(error.recovery_suggestions.len(), 2); // Default + added
        
        match error.error_type {
            ToolErrorType::ValidationError => (),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_follow_up_actions() {
        let execute_action = FollowUpAction::ExecuteTool {
            tool_name: "write_file".to_string(),
            input: serde_json::json!({"path": "output.txt", "content": "data"}),
            depends_on: Some("exec_123".to_string()),
        };
        
        let retry_action = FollowUpAction::RetryWithModifiedInput {
            modified_input: serde_json::json!({"timeout": 60}),
            delay: Some(Duration::from_secs(5)),
        };
        
        match execute_action {
            FollowUpAction::ExecuteTool { tool_name, depends_on, .. } => {
                assert_eq!(tool_name, "write_file");
                assert_eq!(depends_on, Some("exec_123".to_string()));
            }
            _ => panic!("Expected execute tool action"),
        }
        
        match retry_action {
            FollowUpAction::RetryWithModifiedInput { delay, .. } => {
                assert_eq!(delay, Some(Duration::from_secs(5)));
            }
            _ => panic!("Expected retry action"),
        }
    }

    #[test]
    fn test_result_content_block_conversion() {
        let text_result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "test_tool".to_string(),
            ToolResultData::text("Simple text result"),
        );
        assert_eq!(text_result.into_content_block(), "Simple text result");
        
        let json_result = ToolExecutionResult::success(
            "exec_456".to_string(),
            "test_tool".to_string(),
            ToolResultData::json(serde_json::json!({"status": "ok"})),
        );
        let content = json_result.into_content_block();
        assert!(content.contains("\"status\": \"ok\""));
        
        let file_result = ToolExecutionResult::success(
            "exec_789".to_string(),
            "test_tool".to_string(),
            ToolResultData::file_reference_with_hash("/path/file.txt", 2048, "abc123".to_string()),
        );
        let content = file_result.into_content_block();
        assert!(content.contains("File reference: /path/file.txt"));
        assert!(content.contains("2048 bytes"));
        assert!(content.contains("hash: abc123"));
    }
}