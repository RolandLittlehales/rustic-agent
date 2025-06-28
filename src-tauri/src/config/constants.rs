/*!
 * Compile-time Constants
 *
 * These values never change at runtime and are compiled into the binary.
 * Use for: API endpoints, version strings, magic numbers that are truly constant.
 */

use std::time::Duration;

// ============================================================================
// APPLICATION METADATA
// ============================================================================

#[allow(dead_code)]
pub const APP_NAME: &str = "LLM Dev Agent";
#[allow(dead_code)]
pub const APP_VERSION: &str = "0.1.0";
#[allow(dead_code)]
pub const USER_AGENT: &str = "LLMDevAgent/0.1.0";

// ============================================================================
// API CONFIGURATION
// ============================================================================

#[allow(dead_code)]
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com/v1";
#[allow(dead_code)]
pub const CLAUDE_API_MESSAGES_ENDPOINT: &str = "/messages";
#[allow(dead_code)]
pub const CLAUDE_API_VERSION: &str = "2023-06-01";

/// Full Claude API messages URL
#[allow(dead_code)]
pub const fn claude_messages_url() -> &'static str {
    // Note: In const fn, we can't use format!, so we define the full URL
    "https://api.anthropic.com/v1/messages"
}

// ============================================================================
// MODEL CONFIGURATION
// ============================================================================

/// Available Claude models (ordered by preference)
/// Re-exports from claude::constants to maintain single source of truth
pub const SUPPORTED_MODELS: &[&str] = &[
    crate::claude::constants::model_ids::CLAUDE_4_SONNET,
    crate::claude::constants::model_ids::CLAUDE_3_5_SONNET_LATEST,
    crate::claude::constants::model_ids::CLAUDE_3_5_HAIKU,
    crate::claude::constants::model_ids::CLAUDE_3_OPUS,
    "claude-3-sonnet-20240229", // Keep legacy model for compatibility
    crate::claude::constants::model_ids::CLAUDE_3_HAIKU,
];

/// Default model selection - uses latest Claude 4 Sonnet
pub const DEFAULT_MODEL: &str = crate::claude::constants::model_ids::CLAUDE_4_SONNET;

/// Model capabilities mapping
#[allow(dead_code)]
pub struct ModelInfo {
    pub family: &'static str,
    pub variant: &'static str,
    pub max_tokens: u32,
    pub supports_thinking: bool,
    pub supports_tool_use: bool,
    pub cost_per_million_input: f64,
    pub cost_per_million_output: f64,
}

/// Get model information by model name
#[allow(dead_code)]
pub fn get_model_info(model: &str) -> Option<ModelInfo> {
    match model {
        crate::claude::constants::model_ids::CLAUDE_4_SONNET => Some(ModelInfo {
            family: "claude-4",
            variant: "sonnet",
            max_tokens: 200000,
            supports_thinking: true,
            supports_tool_use: true,
            cost_per_million_input: 3.0,
            cost_per_million_output: 15.0,
        }),
        "claude-3-5-sonnet-20241022" => Some(ModelInfo {
            family: "claude-3-5",
            variant: "sonnet",
            max_tokens: 200000,
            supports_thinking: false,
            supports_tool_use: true,
            cost_per_million_input: 3.0,
            cost_per_million_output: 15.0,
        }),
        "claude-3-5-haiku-20241022" => Some(ModelInfo {
            family: "claude-3-5",
            variant: "haiku",
            max_tokens: 200000,
            supports_thinking: false,
            supports_tool_use: true,
            cost_per_million_input: 0.25,
            cost_per_million_output: 1.25,
        }),
        // Add other models as needed
        _ => None,
    }
}

// ============================================================================
// SECURITY PATTERNS
// ============================================================================

/// File patterns to always block for security
#[allow(dead_code)]
pub const SECURITY_BLOCKED_PATTERNS: &[&str] = &[
    "*.env",
    ".env*",
    "id_rsa*",
    "id_dsa*",
    "id_ecdsa*",
    "id_ed25519*",
    "*.key",
    "*.pem",
    "*.p12",
    "*.pfx",
    ".aws/*",
    ".ssh/*",
    ".gnupg/*",
];

/// Files that should never be overwritten
#[allow(dead_code)]
pub const PROTECTED_FILES: &[&str] = &[
    "Cargo.toml",
    "package.json",
    ".env",
    ".gitignore",
    "tauri.conf.json",
    "main.rs",
    "lib.rs",
];

/// Suspicious content patterns for input filtering
pub const SUSPICIOUS_PATTERNS: &[&str] = &[
    "<script",
    "javascript:",
    "data:",
    "vbscript:",
    "onload=",
    "onerror=",
];

/// Allowed file extensions for read operations
#[allow(dead_code)]
pub const ALLOWED_FILE_EXTENSIONS: &[&str] = &[
    "rs", "js", "ts", "json", "toml", "yaml", "yml", "md", "txt", "html", "css", "py", "go",
    "java", "cpp", "c", "h", "hpp", "sh", "bat", "ps1",
];

// ============================================================================
// ENVIRONMENT VARIABLES
// ============================================================================

pub const ENV_CLAUDE_API_KEY: &str = "CLAUDE_API_KEY";
pub const ENV_LOG_LEVEL: &str = "LOG_LEVEL";
#[allow(dead_code)]
pub const ENV_CONFIG_PATH: &str = "CONFIG_PATH";

// ============================================================================
// TIMING CONSTANTS
// ============================================================================

/// HTTP client timeout in seconds
pub const HTTP_TIMEOUT_SECS: u64 = 120;

/// Rate limiting interval between API calls
pub const RATE_LIMIT_INTERVAL_MS: u64 = 1000;

/// File watcher poll interval
pub const FILE_WATCHER_POLL_MS: u64 = 500;

/// File watcher debounce duration
pub const FILE_WATCHER_DEBOUNCE_MS: u64 = 300;

/// Heartbeat interval for file watching
pub const HEARTBEAT_INTERVAL_SECS: u64 = 30;

/// Maximum retry attempts for failed operations
pub const MAX_RETRY_ATTEMPTS: u32 = 3;

/// Base delay for exponential backoff (milliseconds)
pub const RETRY_BASE_DELAY_MS: u64 = 500;

// ============================================================================
// UI CONSTANTS
// ============================================================================

/// Maximum time to wait for Tauri initialization
pub const TAURI_INIT_TIMEOUT_MS: u64 = 5000;

/// Interval for checking Tauri availability
#[allow(dead_code)]
pub const TAURI_CHECK_INTERVAL_MS: u64 = 100;

/// Auto-scroll delay for chat messages
pub const AUTO_SCROLL_DELAY_MS: u64 = 100;

/// Message animation duration
pub const MESSAGE_ANIMATION_MS: u64 = 300;

/// Typing indicator delay
pub const TYPING_INDICATOR_DELAY_MS: u64 = 500;

// ============================================================================
// VALIDATION CONSTANTS
// ============================================================================

/// Safety buffer ratio for input validation (take half for safety)
pub const SAFETY_BUFFER_RATIO: f32 = 0.5;

/// File type icon mappings for UI display
pub const FILE_TYPE_ICONS: &[(&str, &str)] = &[
    ("rs", "🦀"),
    ("js", "📄"),
    ("ts", "📄"),
    ("json", "⚙️"),
    ("md", "📝"),
    ("toml", "⚙️"),
    ("html", "🌐"),
    ("css", "🎨"),
    ("py", "🐍"),
    ("go", "🔷"),
    ("java", "☕"),
    ("cpp", "⚡"),
    ("c", "⚡"),
    ("h", "⚡"),
    ("hpp", "⚡"),
    ("sh", "📜"),
    ("bat", "📜"),
    ("ps1", "📜"),
];

/// Default file icon for unknown extensions
pub const DEFAULT_FILE_ICON: &str = "📄";

/// Directory icon
pub const DIRECTORY_ICON: &str = "📁";

/// Helper function to get file icon by extension
pub fn get_file_icon(extension: &str) -> &'static str {
    FILE_TYPE_ICONS
        .iter()
        .find(|(ext, _)| *ext == extension)
        .map(|(_, icon)| *icon)
        .unwrap_or(DEFAULT_FILE_ICON)
}

// ============================================================================
// ERROR MESSAGE TEMPLATES
// ============================================================================

/// Standard error message templates for consistency
#[allow(dead_code)]
pub mod error_templates {
    pub const EMPTY_INPUT: &str = "Input cannot be empty";
    #[allow(dead_code)]
    pub const INVALID_INPUT: &str = "Invalid input provided";
    pub const API_KEY_NOT_SET: &str = "Claude API key not set. Please set the API key first";
    pub const API_KEY_NOT_FOUND: &str = "No API key found in environment variables";
    pub const CLIENT_CREATION_FAILED: &str = "Failed to create Claude client";
    pub const API_ERROR: &str = "Claude API error";
    #[allow(dead_code)]
    pub const TOOL_EXECUTION_FAILED: &str = "Tool execution failed";
    pub const UNSAFE_CONTENT: &str = "Message contains potentially unsafe content";
    pub const WHITELIST_SAVE_FAILED: &str = "Failed to save whitelist";
    pub const DIRECTORY_NOT_FOUND: &str = "Directory not found in whitelist";

    /// Format an error with context
    pub fn with_context(template: &str, context: &str) -> String {
        format!("{}: {}", template, context)
    }

    /// Format a generic operation failed error
    pub fn operation_failed(operation: &str, error: &str) -> String {
        format!("Failed to {}: {}", operation, error)
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert milliseconds to Duration
#[allow(dead_code)]
pub const fn ms_to_duration(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

/// Convert seconds to Duration  
#[allow(dead_code)]
pub const fn secs_to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

/// Check if a model supports thinking
#[allow(dead_code)]
pub fn model_supports_thinking(model: &str) -> bool {
    matches!(model, crate::claude::constants::model_ids::CLAUDE_4_SONNET)
}

/// Check if a model is Claude 4
#[allow(dead_code)]
pub fn is_claude_4_model(model: &str) -> bool {
    model.starts_with("claude-4") || model.contains("-4-")
}

/// Get default configuration values as constants
pub mod defaults {
    pub const MAX_TOKENS: u32 = 8192;
    pub const TEMPERATURE: f32 = 0.7;
    pub const MESSAGE_MAX_CHARS: usize = 50000; // 50KB for coding helper
    pub const FILE_MAX_SIZE_BYTES: u64 = 10 * 1024 * 1024; // 10MB
    pub const PATH_MAX_CHARS: usize = 4096;
    pub const NAME_MAX_CHARS: usize = 100;
    pub const DIRECTORY_MAX_ENTRIES: usize = 1000;
    pub const WRITE_CONTENT_MAX_BYTES: u64 = 50 * 1024 * 1024; // 50MB
}
