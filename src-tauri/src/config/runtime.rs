/*!
 * Runtime Configuration
 *
 * Configuration that can be changed at runtime through:
 * - Environment variables
 * - Configuration files
 * - Runtime settings
 */

use super::constants::{defaults, ENV_CLAUDE_API_KEY, ENV_LOG_LEVEL};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Runtime configuration that can be modified during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    // API Configuration
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,

    // Timeout Configuration (in milliseconds)
    pub http_timeout_ms: u64,
    pub rate_limit_interval_ms: u64,
    pub file_watcher_poll_ms: u64,
    pub file_watcher_debounce_ms: u64,
    pub heartbeat_interval_secs: u64,

    // Retry Configuration
    pub max_retry_attempts: u32,
    pub retry_base_delay_ms: u64,

    // UI Configuration
    pub tauri_init_timeout_ms: u64,
    pub auto_scroll_delay_ms: u64,
    pub message_animation_ms: u64,
    pub typing_indicator_delay_ms: u64,

    // Feature Flags
    pub enable_file_watching: bool,
    pub enable_debug_logging: bool,
    pub enable_thinking_mode: bool,

    // Log Level
    pub log_level: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            // API Configuration
            api_key: None,
            model: super::constants::DEFAULT_MODEL.to_string(),
            max_tokens: defaults::MAX_TOKENS,
            temperature: defaults::TEMPERATURE,

            // Timeout Configuration
            http_timeout_ms: super::constants::HTTP_TIMEOUT_SECS * 1000,
            rate_limit_interval_ms: super::constants::RATE_LIMIT_INTERVAL_MS,
            file_watcher_poll_ms: super::constants::FILE_WATCHER_POLL_MS,
            file_watcher_debounce_ms: super::constants::FILE_WATCHER_DEBOUNCE_MS,
            heartbeat_interval_secs: super::constants::HEARTBEAT_INTERVAL_SECS,

            // Retry Configuration
            max_retry_attempts: super::constants::MAX_RETRY_ATTEMPTS,
            retry_base_delay_ms: super::constants::RETRY_BASE_DELAY_MS,

            // UI Configuration
            tauri_init_timeout_ms: super::constants::TAURI_INIT_TIMEOUT_MS,
            auto_scroll_delay_ms: super::constants::AUTO_SCROLL_DELAY_MS,
            message_animation_ms: super::constants::MESSAGE_ANIMATION_MS,
            typing_indicator_delay_ms: super::constants::TYPING_INDICATOR_DELAY_MS,

            // Feature Flags
            enable_file_watching: true,
            enable_debug_logging: false,
            enable_thinking_mode: true,

            // Logging
            log_level: "info".to_string(),
        }
    }
}

impl RuntimeConfig {
    /// Load configuration from environment variables
    pub fn load_from_env(&mut self) -> Result<()> {
        // API Key
        if let Ok(api_key) = std::env::var(ENV_CLAUDE_API_KEY) {
            if !api_key.is_empty() {
                self.api_key = Some(api_key);
            }
        }

        // Log Level
        if let Ok(log_level) = std::env::var(ENV_LOG_LEVEL) {
            self.log_level = log_level;
        }

        // Model
        if let Ok(model) = std::env::var("CLAUDE_MODEL") {
            if super::constants::SUPPORTED_MODELS.contains(&model.as_str()) {
                self.model = model;
            }
        }

        // Max Tokens
        if let Ok(max_tokens_str) = std::env::var("CLAUDE_MAX_TOKENS") {
            if let Ok(max_tokens) = max_tokens_str.parse::<u32>() {
                if max_tokens > 0 && max_tokens <= 200000 {
                    self.max_tokens = max_tokens;
                }
            }
        }

        // Temperature
        if let Ok(temp_str) = std::env::var("CLAUDE_TEMPERATURE") {
            if let Ok(temperature) = temp_str.parse::<f32>() {
                if (0.0..=1.0).contains(&temperature) {
                    self.temperature = temperature;
                }
            }
        }

        // Feature Flags
        if let Ok(debug) = std::env::var("DEBUG") {
            self.enable_debug_logging = debug.eq_ignore_ascii_case("true") || debug == "1";
        }

        if let Ok(file_watching) = std::env::var("ENABLE_FILE_WATCHING") {
            self.enable_file_watching =
                file_watching.eq_ignore_ascii_case("true") || file_watching == "1";
        }

        Ok(())
    }

    /// Merge another runtime config into this one (other takes precedence)
    pub fn merge(&mut self, other: RuntimeConfig) {
        if other.api_key.is_some() {
            self.api_key = other.api_key;
        }

        if other.model != super::constants::DEFAULT_MODEL {
            self.model = other.model;
        }

        if other.max_tokens != defaults::MAX_TOKENS {
            self.max_tokens = other.max_tokens;
        }

        if other.temperature != defaults::TEMPERATURE {
            self.temperature = other.temperature;
        }

        // Merge timeouts
        self.http_timeout_ms = other.http_timeout_ms;
        self.rate_limit_interval_ms = other.rate_limit_interval_ms;
        self.file_watcher_poll_ms = other.file_watcher_poll_ms;
        self.file_watcher_debounce_ms = other.file_watcher_debounce_ms;
        self.heartbeat_interval_secs = other.heartbeat_interval_secs;

        // Merge retry config
        self.max_retry_attempts = other.max_retry_attempts;
        self.retry_base_delay_ms = other.retry_base_delay_ms;

        // Merge UI config
        self.tauri_init_timeout_ms = other.tauri_init_timeout_ms;
        self.auto_scroll_delay_ms = other.auto_scroll_delay_ms;
        self.message_animation_ms = other.message_animation_ms;
        self.typing_indicator_delay_ms = other.typing_indicator_delay_ms;

        // Merge feature flags
        self.enable_file_watching = other.enable_file_watching;
        self.enable_debug_logging = other.enable_debug_logging;
        self.enable_thinking_mode = other.enable_thinking_mode;

        self.log_level = other.log_level;
    }

    /// Validate the runtime configuration
    pub fn validate(&self) -> Result<()> {
        // Validate model
        if !super::constants::SUPPORTED_MODELS.contains(&self.model.as_str()) {
            return Err(anyhow::anyhow!("Unsupported model: {}", self.model));
        }

        // Validate max tokens
        if self.max_tokens == 0 || self.max_tokens > 200000 {
            return Err(anyhow::anyhow!(
                "Invalid max_tokens: {} (must be 1-200000)",
                self.max_tokens
            ));
        }

        // Validate temperature
        if !(0.0..=1.0).contains(&self.temperature) {
            return Err(anyhow::anyhow!(
                "Invalid temperature: {} (must be 0.0-1.0)",
                self.temperature
            ));
        }

        // Validate timeouts are reasonable
        if self.http_timeout_ms < 1000 || self.http_timeout_ms > 300000 {
            return Err(anyhow::anyhow!(
                "Invalid http_timeout_ms: {} (must be 1000-300000)",
                self.http_timeout_ms
            ));
        }

        Ok(())
    }

    /// Check if the model supports thinking
    #[allow(dead_code)]
    pub fn supports_thinking(&self) -> bool {
        super::constants::model_supports_thinking(&self.model)
    }

    /// Check if this is a Claude 4 model
    #[allow(dead_code)]
    pub fn is_claude_4(&self) -> bool {
        super::constants::is_claude_4_model(&self.model)
    }

    /// Get HTTP timeout as Duration
    #[allow(dead_code)]
    pub fn http_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.http_timeout_ms)
    }

    /// Get rate limit interval as Duration
    #[allow(dead_code)]
    pub fn rate_limit_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.rate_limit_interval_ms)
    }

    /// Get file watcher poll interval as Duration
    #[allow(dead_code)]
    pub fn file_watcher_poll_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.file_watcher_poll_ms)
    }

    /// Get file watcher debounce duration
    #[allow(dead_code)]
    pub fn file_watcher_debounce(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.file_watcher_debounce_ms)
    }

    /// Get heartbeat interval as Duration
    #[allow(dead_code)]
    pub fn heartbeat_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.heartbeat_interval_secs)
    }

    /// Get retry base delay as Duration
    #[allow(dead_code)]
    pub fn retry_base_delay(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.retry_base_delay_ms)
    }
}
