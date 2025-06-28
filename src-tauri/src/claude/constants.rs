/// Constants for the Claude module to eliminate magic numbers
///
/// This file centralizes all numeric constants used throughout the Claude integration
/// to improve code readability and maintainability.
use std::time::Duration;

/// Circuit breaker state values
pub mod circuit_breaker {
    /// Circuit breaker state: Closed (normal operation)
    pub const STATE_CLOSED: u8 = 0;
    /// Circuit breaker state: Open (failing, blocking requests)
    pub const STATE_OPEN: u8 = 1;
    /// Circuit breaker state: Half-open (testing recovery)
    pub const STATE_HALF_OPEN: u8 = 2;
}

/// Error handling and retry configuration constants
pub mod error_handling {
    use super::Duration;

    /// Re-export max retries from global config to avoid duplication
    pub const DEFAULT_MAX_RETRIES: u32 = crate::config::constants::MAX_RETRY_ATTEMPTS;

    /// Default base delay for exponential backoff in milliseconds
    /// Starting point for retry delays: 500ms, 1s, 2s, 4s...
    pub const DEFAULT_BASE_DELAY_MS: u64 = 500;

    /// Default maximum delay between retries in seconds
    /// Caps exponential backoff to prevent excessively long waits
    /// Even with many retries, never wait more than 30 seconds
    pub const DEFAULT_MAX_DELAY_SECS: u64 = 30;

    /// Default backoff multiplier for exponential backoff
    /// Each retry delay = previous_delay * 2.0 (doubles each time)
    /// Standard exponential backoff to quickly space out retries
    pub const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;

    /// Default circuit breaker failure threshold
    /// Circuit opens after 5 consecutive failures to protect downstream systems
    /// Prevents cascading failures when API is unavailable
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;

    /// Default circuit breaker timeout duration in seconds
    /// How long circuit stays open before attempting recovery
    /// After 60s, tries one request to see if service is healthy again
    pub const DEFAULT_CIRCUIT_TIMEOUT_SECS: u64 = 60;

    /// Re-export HTTP timeout from global config to avoid duplication
    pub const DEFAULT_HTTP_TIMEOUT_SECS: u64 = crate::config::constants::HTTP_TIMEOUT_SECS;

    /// Jitter factor for adding randomness to retry delays (10%)
    /// Prevents thundering herd problems when multiple clients retry simultaneously
    pub const JITTER_FACTOR: f64 = 0.1;

    /// Modulus for pseudo-random jitter calculation from system time hash
    /// Used to generate values 0-999 for percentage-based jitter variance
    /// Ensures retry delays are spread out to avoid coordinated retry storms
    pub const JITTER_MODULUS: u64 = 1000;

    /// Maximum character length for error messages before truncation
    /// Prevents log flooding from extremely long error messages
    /// Keeps logs readable while preserving essential error information
    pub const MAX_ERROR_MESSAGE_LENGTH: usize = 500;

    /// Characters to keep when truncating error messages (500 - "...[TRUNCATED]" = 497)
    /// Leaves room for truncation indicator while maximizing preserved content
    pub const ERROR_MESSAGE_TRUNCATE_LENGTH: usize = 497;

    /// Maximum character length for metadata values before truncation
    /// Prevents metadata from overwhelming log entries
    /// Metadata should be concise identifiers, not full content
    pub const MAX_METADATA_VALUE_LENGTH: usize = 100;

    /// Characters to keep when truncating metadata values (100 - "...[TRUNCATED]" = 97)
    /// Preserves most important part of metadata while indicating truncation
    pub const METADATA_VALUE_TRUNCATE_LENGTH: usize = 97;

    /// Percentage multiplier for success rate calculation
    /// Converts decimal success rate (0.85) to percentage (85.0%)
    /// Used in telemetry reporting for human-readable metrics
    #[allow(dead_code)]
    pub const SUCCESS_RATE_PERCENTAGE: f64 = 100.0;

    /// Helper functions for creating Duration constants
    pub const fn base_delay() -> Duration {
        Duration::from_millis(DEFAULT_BASE_DELAY_MS)
    }

    pub const fn max_delay() -> Duration {
        Duration::from_secs(DEFAULT_MAX_DELAY_SECS)
    }

    pub const fn circuit_timeout() -> Duration {
        Duration::from_secs(DEFAULT_CIRCUIT_TIMEOUT_SECS)
    }

    pub const fn http_timeout() -> Duration {
        Duration::from_secs(DEFAULT_HTTP_TIMEOUT_SECS)
    }
}

/// Telemetry and monitoring constants
pub mod telemetry {
    use super::Duration;

    /// Maximum number of different error types to track in telemetry
    /// Prevents unbounded memory growth from tracking too many error categories
    /// 100 types should cover all legitimate error scenarios
    pub const MAX_ERROR_TYPES: usize = 100;

    /// Cleanup interval for bounded error counter in seconds
    /// Periodically removes low-frequency error types to free memory
    /// Every 5 minutes, prunes error types with only 1 occurrence
    pub const CLEANUP_INTERVAL_SECS: u64 = 300;

    /// Helper function for cleanup interval
    pub const fn cleanup_interval() -> Duration {
        Duration::from_secs(CLEANUP_INTERVAL_SECS)
    }
}

/// Model configuration constants
pub mod model_config {
    /// Default model identifier (Claude 4 Sonnet - latest generation)
    /// Uses smart model selection to always get the latest Sonnet model
    pub fn default_model() -> &'static str {
        super::model_ids::latest_claude_4_sonnet()
    }

    /// Default maximum tokens for Claude 4 models
    pub const DEFAULT_MAX_TOKENS: u32 = 8192;

    /// Default temperature for generation
    pub const DEFAULT_TEMPERATURE: f32 = 0.7;

    /// Claude 4 context window size in tokens
    /// Maximum total tokens (input + output) Claude 4 can process
    /// 200K tokens ≈ 150K words or 600 pages of text
    pub const CLAUDE_4_CONTEXT_WINDOW: u32 = 200_000;

    /// Default fallback max tokens for unknown models
    /// Conservative default when model capabilities are unknown
    /// Ensures requests don't exceed limits of older models
    pub const FALLBACK_MAX_TOKENS: u32 = 8192;

    /// Model-specific token limits
    pub const CLAUDE_4_MAX_TOKENS: u32 = 8192;
    pub const CLAUDE_3_5_MAX_TOKENS: u32 = 8192;
}

/// Model cost constants (USD per million tokens)
pub mod model_costs {
    /// Claude 4 Opus input cost per million tokens
    pub const CLAUDE_4_OPUS_INPUT_COST: f64 = 15.0;

    /// Claude 4 Opus output cost per million tokens  
    pub const CLAUDE_4_OPUS_OUTPUT_COST: f64 = 75.0;

    /// Claude 4 Sonnet input cost per million tokens
    pub const CLAUDE_4_SONNET_INPUT_COST: f64 = 3.0;

    /// Claude 4 Sonnet output cost per million tokens
    pub const CLAUDE_4_SONNET_OUTPUT_COST: f64 = 15.0;

    /// Claude 3.7 Sonnet input cost per million tokens
    pub const CLAUDE_3_7_SONNET_INPUT_COST: f64 = 3.0;

    /// Claude 3.7 Sonnet output cost per million tokens
    pub const CLAUDE_3_7_SONNET_OUTPUT_COST: f64 = 15.0;

    /// Claude 3 Opus input cost per million tokens
    pub const CLAUDE_3_OPUS_INPUT_COST: f64 = 15.0;

    /// Claude 3 Opus output cost per million tokens
    pub const CLAUDE_3_OPUS_OUTPUT_COST: f64 = 75.0;

    /// Claude 3 Haiku input cost per million tokens
    pub const CLAUDE_3_HAIKU_INPUT_COST: f64 = 0.25;

    /// Claude 3 Haiku output cost per million tokens
    pub const CLAUDE_3_HAIKU_OUTPUT_COST: f64 = 1.25;

    /// Claude 3.5 Sonnet input cost per million tokens
    pub const CLAUDE_3_5_SONNET_INPUT_COST: f64 = 3.0;

    /// Claude 3.5 Sonnet output cost per million tokens
    pub const CLAUDE_3_5_SONNET_OUTPUT_COST: f64 = 15.0;

    /// Claude 3.5 Haiku input cost per million tokens
    pub const CLAUDE_3_5_HAIKU_INPUT_COST: f64 = 0.8;

    /// Claude 3.5 Haiku output cost per million tokens
    pub const CLAUDE_3_5_HAIKU_OUTPUT_COST: f64 = 4.0;

    /// Tokens per million constant for cost calculations
    /// Anthropic prices are quoted per million tokens
    /// Used to convert actual token counts to pricing units
    #[allow(dead_code)]
    pub const TOKENS_PER_MILLION: f64 = 1_000_000.0;

    /// Maximum reasonable token count for validation (10 million)
    /// Sanity check to detect programming errors or unrealistic inputs
    /// 10M tokens would cost ~$150 and take hours to process
    #[allow(dead_code)]
    pub const MAX_REASONABLE_TOKEN_COUNT: u32 = 10_000_000;
}

/// File and path handling constants
#[allow(dead_code)]
pub mod file_limits {
    /// Maximum path length for validation in characters
    /// Most filesystems support paths up to 4096 characters
    /// Prevents buffer overflows and filesystem compatibility issues
    pub const MAX_PATH_LENGTH: usize = 4096;

    /// Maximum file size for reading operations in bytes (10MB)
    /// Prevents memory exhaustion from accidentally reading huge files
    /// 10MB should handle most code files while protecting system resources
    pub const MAX_FILE_SIZE_BYTES: usize = 10 * 1024 * 1024;

    /// Maximum content size for writing operations in bytes (50MB)
    /// Larger than read limit since AI can generate substantial content
    /// Prevents runaway generation from consuming all disk space
    pub const MAX_WRITE_CONTENT_SIZE: usize = 50 * 1024 * 1024;

    /// Maximum directory entries to list in one operation
    /// Prevents UI freezing when scanning directories with thousands of files
    /// 1000 entries provides good coverage while maintaining responsiveness
    pub const MAX_DIRECTORY_ENTRIES: usize = 1000;

    /// Default whitelist max file size in bytes (10MB)
    /// Security limit for files accessible through the whitelist system
    /// Prevents reading of large files that could contain sensitive data
    pub const DEFAULT_WHITELIST_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
}

/// API version and protocol constants
/// NOTE: API version constants moved to config/constants.rs to avoid duplication
#[allow(dead_code)]
pub mod api {
    /// Re-export API version from unified config to maintain backward compatibility
    #[allow(unused_imports)]
    pub use crate::config::constants::CLAUDE_API_VERSION as ANTHROPIC_API_VERSION;
}

/// Model identifiers and release dates
/// Source: https://docs.anthropic.com/en/docs/about-claude/model-deprecations#model-status
/// NOTE: These IDs must be validated against Anthropic's documentation when changed
/// Only includes currently active (non-deprecated) models
pub mod model_ids {
    /// Claude 4 Opus model identifier (latest generation)
    pub const CLAUDE_4_OPUS: &str = "claude-opus-4-20250514";

    /// Claude 4 Sonnet model identifier (latest generation)
    pub const CLAUDE_4_SONNET: &str = "claude-sonnet-4-20250514";

    /// Claude 3.7 Sonnet model identifier (enhanced 3.5 series)
    pub const CLAUDE_3_7_SONNET: &str = "claude-3-7-sonnet-20250219";

    /// Claude 3.5 Sonnet model identifier (most recent 3.5)
    pub const CLAUDE_3_5_SONNET_LATEST: &str = "claude-3-5-sonnet-20241022";

    /// Claude 3.5 Sonnet model identifier (previous version)
    pub const CLAUDE_3_5_SONNET_PREVIOUS: &str = "claude-3-5-sonnet-20240620";

    /// Claude 3.5 Haiku model identifier
    pub const CLAUDE_3_5_HAIKU: &str = "claude-3-5-haiku-20241022";

    /// Claude 3 Opus model identifier (legacy but active)
    pub const CLAUDE_3_OPUS: &str = "claude-3-opus-20240229";

    /// Claude 3 Haiku model identifier (legacy but active)
    pub const CLAUDE_3_HAIKU: &str = "claude-3-haiku-20240307";

    /// Extract release date from model ID (last 8 characters: YYYYMMDD)
    /// Example: "claude-3-5-sonnet-20240620" -> "20240620"
    pub fn extract_release_date(model_id: &str) -> &str {
        if model_id.len() >= 8 {
            &model_id[model_id.len() - 8..]
        } else {
            "unknown"
        }
    }

    /// Model variant types for smart selection
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ModelVariant {
        Opus,
        Sonnet,
        Haiku,
    }

    /// Get the latest (or offset) model ID for a given variant
    /// offset: 0 = latest, -1 = previous, -2 = one before previous, etc.
    /// Returns the most recent model by default
    ///
    /// Examples:
    /// - get_model_by_variant(ModelVariant::Sonnet, 0) -> "claude-sonnet-4-20250514"
    /// - get_model_by_variant(ModelVariant::Sonnet, -1) -> "claude-3-7-sonnet-20250219"
    /// - get_model_by_variant(ModelVariant::Haiku, 0) -> "claude-3-5-haiku-20241022"
    pub fn get_model_by_variant(variant: ModelVariant, offset: i32) -> &'static str {
        match variant {
            ModelVariant::Opus => {
                // Opus models in reverse chronological order (newest first)
                let opus_models = [CLAUDE_4_OPUS, CLAUDE_3_OPUS];
                get_model_at_offset(&opus_models, offset)
            }
            ModelVariant::Sonnet => {
                // Sonnet models in reverse chronological order (newest first)
                let sonnet_models = [
                    CLAUDE_4_SONNET,
                    CLAUDE_3_7_SONNET,
                    CLAUDE_3_5_SONNET_LATEST,
                    CLAUDE_3_5_SONNET_PREVIOUS,
                ];
                get_model_at_offset(&sonnet_models, offset)
            }
            ModelVariant::Haiku => {
                // Haiku models in reverse chronological order (newest first)
                let haiku_models = [CLAUDE_3_5_HAIKU, CLAUDE_3_HAIKU];
                get_model_at_offset(&haiku_models, offset)
            }
        }
    }

    /// Helper function to get model at specific offset from array
    /// offset: 0 = first (latest), -1 = second, -2 = third, etc.
    /// Falls back to latest if offset is out of bounds
    fn get_model_at_offset(models: &[&'static str], offset: i32) -> &'static str {
        if models.is_empty() {
            return "claude-sonnet-4-20250514"; // Safe fallback
        }

        let index = if offset <= 0 {
            (-offset) as usize
        } else {
            0 // Positive offsets default to latest
        };

        models.get(index).unwrap_or(&models[0])
    }

    // Convenience functions for common use cases
    /// Get the latest Claude 4 Sonnet model
    pub fn latest_claude_4_sonnet() -> &'static str {
        get_model_by_variant(ModelVariant::Sonnet, 0)
    }

    /// Get the latest Claude 4 Opus model
    pub fn latest_claude_4_opus() -> &'static str {
        get_model_by_variant(ModelVariant::Opus, 0)
    }

    /// Get the latest Haiku model (currently Claude 3.5)
    pub fn latest_haiku() -> &'static str {
        get_model_by_variant(ModelVariant::Haiku, 0)
    }

    /// Get the previous generation Sonnet model
    #[allow(dead_code)]
    pub fn previous_sonnet() -> &'static str {
        get_model_by_variant(ModelVariant::Sonnet, -1)
    }
}

/// Test data constants for consistent testing
#[allow(dead_code)]
pub mod test_data {
    /// Test token counts for cost estimation validation
    /// Realistic values for testing cost calculation logic
    /// 1000 input + 500 output = typical conversation exchange
    pub const TEST_INPUT_TOKENS: u32 = 1000;
    pub const TEST_OUTPUT_TOKENS: u32 = 500;

    /// Expected cost calculation result for test inputs
    /// 1000 input tokens * $3/1M + 500 output tokens * $15/1M = $0.0105
    /// Used to validate cost calculation accuracy in tests
    pub const EXPECTED_CLAUDE_4_SONNET_COST: f64 = 0.0105;

    /// Test tolerance for floating point comparisons
    /// Accounts for floating point precision errors in cost calculations
    /// Allows for tiny rounding differences while catching real errors
    pub const COST_CALCULATION_TOLERANCE: f64 = 0.0001;

    /// Test model configuration values for validation
    /// Realistic settings different from defaults to test configuration logic
    /// 4096 tokens = half of default, 0.5 temperature = balanced creativity
    pub const TEST_MAX_TOKENS: u32 = 4096;
    pub const TEST_TEMPERATURE: f32 = 0.5;
}
