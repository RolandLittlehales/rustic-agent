/*!
 * Unified Configuration System for LLM Dev Agent
 *
 * This module provides a holistic configuration approach using three patterns:
 * 1. Compile-time constants for immutable values
 * 2. Runtime configuration for deployment-specific settings
 * 3. Builder pattern for complex configurations with validation
 */

pub mod constants;
pub mod runtime;
pub mod validation;

pub use constants::*;
pub use runtime::RuntimeConfig;
pub use validation::ValidationLimits;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Master configuration that combines all configuration sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub runtime: RuntimeConfig,
    pub validation: ValidationLimits,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig::default(),
            validation: ValidationLimits::default(),
        }
    }
}

impl AppConfig {
    /// Load configuration from multiple sources with fallbacks
    pub fn load() -> Result<Self> {
        // Start with defaults
        let mut config = Self::default();

        // Try to load from environment variables
        config.runtime.load_from_env()?;

        // Try to load from config file if it exists
        if let Ok(file_config) = Self::load_from_file() {
            config.merge(file_config);
        }

        // Validate the final configuration
        config.validate()?;

        Ok(config)
    }

    /// Load configuration from TOML file
    fn load_from_file() -> Result<Self> {
        let config_path = std::env::current_dir()?.join("config").join("app.toml");
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            let config: Self = toml::from_str(&content)?;
            Ok(config)
        } else {
            Err(anyhow::anyhow!("Config file not found"))
        }
    }

    /// Merge another configuration into this one
    fn merge(&mut self, other: Self) {
        // Merge runtime config
        self.runtime.merge(other.runtime);

        // Merge validation limits
        self.validation.merge(other.validation);
    }

    /// Validate the configuration
    fn validate(&self) -> Result<()> {
        self.runtime.validate()?;
        self.validation.validate()?;
        Ok(())
    }

    /// Get timeout as Duration from milliseconds
    pub fn get_timeout(&self, timeout_ms: u64) -> Duration {
        Duration::from_millis(timeout_ms)
    }
}

/// Configuration builder for complex setups
pub struct AppConfigBuilder {
    config: AppConfig,
}

impl AppConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.config.runtime.api_key = Some(api_key);
        self
    }

    pub fn with_message_limit(mut self, limit: usize) -> Self {
        self.config.validation.message_max_chars = limit;
        self
    }

    pub fn with_file_size_limit(mut self, limit: u64) -> Self {
        self.config.validation.file_max_size_bytes = limit;
        self
    }

    pub fn build(self) -> Result<AppConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for AppConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
