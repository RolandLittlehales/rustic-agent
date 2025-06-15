use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    /// Claude API key
    pub api_key: String,
    /// Base URL for Claude API (defaults to Anthropic's API)
    pub base_url: String,
    /// Model to use (e.g., "claude-3-sonnet-20240229")
    pub model: String,
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Temperature for response generation (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    pub top_k: Option<u32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Request timeout duration
    pub timeout: Duration,
    /// Maximum number of retry attempts
    pub max_retries: u32,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.anthropic.com".to_string(),
            model: "claude-3-sonnet-20240229".to_string(),
            max_tokens: 4096,
            temperature: None,
            top_p: None,
            top_k: None,
            stop_sequences: None,
            timeout: Duration::from_secs(60),
            max_retries: 3,
        }
    }
}

impl ClaudeConfig {
    /// Create a new config with the provided API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    /// Set the model to use
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set the maximum tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set the temperature for response generation
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the top-p sampling parameter
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the top-k sampling parameter
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set stop sequences
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set maximum retry attempts
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), crate::claude::error::ClaudeError> {
        if self.api_key.is_empty() {
            return Err(crate::claude::error::ClaudeError::ConfigError(
                "API key is required".to_string()
            ));
        }

        if self.model.is_empty() {
            return Err(crate::claude::error::ClaudeError::ConfigError(
                "Model is required".to_string()
            ));
        }

        if self.max_tokens == 0 {
            return Err(crate::claude::error::ClaudeError::ConfigError(
                "Max tokens must be greater than 0".to_string()
            ));
        }

        if let Some(temp) = self.temperature {
            if !(0.0..=1.0).contains(&temp) {
                return Err(crate::claude::error::ClaudeError::ConfigError(
                    "Temperature must be between 0.0 and 1.0".to_string()
                ));
            }
        }

        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err(crate::claude::error::ClaudeError::ConfigError(
                    "Top-p must be between 0.0 and 1.0".to_string()
                ));
            }
        }

        Ok(())
    }
}