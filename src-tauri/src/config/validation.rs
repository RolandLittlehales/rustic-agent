/*!
 * Validation Limits Configuration
 *
 * Centralized validation limits that can be adjusted per deployment.
 * These limits enforce security and prevent resource abuse.
 */

use super::constants::defaults;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Validation limits for various operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationLimits {
    // Message and Content Limits
    pub message_max_chars: usize,
    pub name_max_chars: usize,
    pub path_max_chars: usize,

    // File Size Limits (in bytes)
    pub file_max_size_bytes: u64,
    pub write_content_max_bytes: u64,

    // Directory and Listing Limits
    pub directory_max_entries: usize,
    pub directory_max_depth: usize,

    // Security Limits
    pub max_concurrent_requests: usize,
    pub max_tools_per_request: usize,
    pub max_conversation_messages: usize,

    // Character count warning thresholds (for UI)
    pub message_warning_threshold: usize,
    pub message_danger_threshold: usize,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            // Message and Content Limits
            message_max_chars: defaults::MESSAGE_MAX_CHARS,
            name_max_chars: defaults::NAME_MAX_CHARS,
            path_max_chars: defaults::PATH_MAX_CHARS,

            // File Size Limits
            file_max_size_bytes: defaults::FILE_MAX_SIZE_BYTES,
            write_content_max_bytes: defaults::WRITE_CONTENT_MAX_BYTES,

            // Directory and Listing Limits
            directory_max_entries: defaults::DIRECTORY_MAX_ENTRIES,
            directory_max_depth: 0, // 0 = unlimited

            // Security Limits
            max_concurrent_requests: 5,
            max_tools_per_request: 10,
            max_conversation_messages: 100,

            // UI Warning Thresholds (80% and 95% of max)
            message_warning_threshold: (defaults::MESSAGE_MAX_CHARS as f64 * 0.8) as usize,
            message_danger_threshold: (defaults::MESSAGE_MAX_CHARS as f64 * 0.95) as usize,
        }
    }
}

impl ValidationLimits {
    /// Create validation limits with custom message size
    #[allow(dead_code)]
    pub fn with_message_limit(mut self, limit: usize) -> Self {
        self.message_max_chars = limit;
        self.update_warning_thresholds();
        self
    }

    /// Create validation limits with custom file size
    #[allow(dead_code)]
    pub fn with_file_limit(mut self, limit: u64) -> Self {
        self.file_max_size_bytes = limit;
        self
    }

    /// Update warning thresholds based on message limit
    #[allow(dead_code)]
    fn update_warning_thresholds(&mut self) {
        self.message_warning_threshold = (self.message_max_chars as f64 * 0.8) as usize;
        self.message_danger_threshold = (self.message_max_chars as f64 * 0.95) as usize;
    }

    /// Merge another validation limits config into this one
    pub fn merge(&mut self, other: ValidationLimits) {
        self.message_max_chars = other.message_max_chars;
        self.name_max_chars = other.name_max_chars;
        self.path_max_chars = other.path_max_chars;
        self.file_max_size_bytes = other.file_max_size_bytes;
        self.write_content_max_bytes = other.write_content_max_bytes;
        self.directory_max_entries = other.directory_max_entries;
        self.directory_max_depth = other.directory_max_depth;
        self.max_concurrent_requests = other.max_concurrent_requests;
        self.max_tools_per_request = other.max_tools_per_request;
        self.max_conversation_messages = other.max_conversation_messages;
        self.message_warning_threshold = other.message_warning_threshold;
        self.message_danger_threshold = other.message_danger_threshold;
    }

    /// Validate the limits configuration
    pub fn validate(&self) -> Result<()> {
        // Message limits
        if self.message_max_chars == 0 || self.message_max_chars > 1_000_000 {
            return Err(anyhow::anyhow!(
                "Invalid message_max_chars: {} (must be 1-1000000)",
                self.message_max_chars
            ));
        }

        // File size limits
        if self.file_max_size_bytes == 0 || self.file_max_size_bytes > 1_000_000_000 {
            return Err(anyhow::anyhow!(
                "Invalid file_max_size_bytes: {} (must be 1-1000000000)",
                self.file_max_size_bytes
            ));
        }

        if self.write_content_max_bytes < self.file_max_size_bytes {
            return Err(anyhow::anyhow!(
                "write_content_max_bytes ({}) must be >= file_max_size_bytes ({})",
                self.write_content_max_bytes,
                self.file_max_size_bytes
            ));
        }

        // Directory limits
        if self.directory_max_entries == 0 || self.directory_max_entries > 10000 {
            return Err(anyhow::anyhow!(
                "Invalid directory_max_entries: {} (must be 1-10000)",
                self.directory_max_entries
            ));
        }

        // Security limits
        if self.max_concurrent_requests == 0 || self.max_concurrent_requests > 100 {
            return Err(anyhow::anyhow!(
                "Invalid max_concurrent_requests: {} (must be 1-100)",
                self.max_concurrent_requests
            ));
        }

        // Warning thresholds
        if self.message_warning_threshold >= self.message_max_chars {
            return Err(anyhow::anyhow!(
                "message_warning_threshold ({}) must be < message_max_chars ({})",
                self.message_warning_threshold,
                self.message_max_chars
            ));
        }

        if self.message_danger_threshold >= self.message_max_chars {
            return Err(anyhow::anyhow!(
                "message_danger_threshold ({}) must be < message_max_chars ({})",
                self.message_danger_threshold,
                self.message_max_chars
            ));
        }

        Ok(())
    }

    /// Check if a message length triggers a warning
    #[allow(dead_code)]
    pub fn message_warning_level(&self, length: usize) -> MessageWarningLevel {
        if length >= self.message_danger_threshold {
            MessageWarningLevel::Danger
        } else if length >= self.message_warning_threshold {
            MessageWarningLevel::Warning
        } else {
            MessageWarningLevel::Ok
        }
    }

    /// Validate message length
    pub fn validate_message_length(&self, length: usize) -> Result<()> {
        if length > self.message_max_chars {
            return Err(anyhow::anyhow!(
                "Message too long: {} chars (max: {} chars)",
                length,
                self.message_max_chars
            ));
        }
        Ok(())
    }

    /// Validate file size
    #[allow(dead_code)]
    pub fn validate_file_size(&self, size: u64) -> Result<()> {
        if size > self.file_max_size_bytes {
            return Err(anyhow::anyhow!(
                "File too large: {} bytes (max: {} bytes)",
                size,
                self.file_max_size_bytes
            ));
        }
        Ok(())
    }

    /// Validate path length
    #[allow(dead_code)]
    pub fn validate_path_length(&self, path: &str) -> Result<()> {
        if path.len() > self.path_max_chars {
            return Err(anyhow::anyhow!(
                "Path too long: {} chars (max: {} chars)",
                path.len(),
                self.path_max_chars
            ));
        }
        Ok(())
    }

    /// Validate name length
    pub fn validate_name_length(&self, name: &str) -> Result<()> {
        if name.len() > self.name_max_chars {
            return Err(anyhow::anyhow!(
                "Name too long: {} chars (max: {} chars)",
                name.len(),
                self.name_max_chars
            ));
        }
        Ok(())
    }

    /// Format file size as human readable
    #[allow(dead_code)]
    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if size.fract() == 0.0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Get file size limit as human readable string
    #[allow(dead_code)]
    pub fn file_size_limit_display(&self) -> String {
        Self::format_file_size(self.file_max_size_bytes)
    }

    /// Get write content limit as human readable string
    #[allow(dead_code)]
    pub fn write_content_limit_display(&self) -> String {
        Self::format_file_size(self.write_content_max_bytes)
    }
}

/// Message warning levels for UI feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageWarningLevel {
    Ok,
    Warning,
    Danger,
}

impl MessageWarningLevel {
    /// Get CSS class for the warning level
    #[allow(dead_code)]
    pub fn css_class(&self) -> &'static str {
        match self {
            MessageWarningLevel::Ok => "text-gray-500",
            MessageWarningLevel::Warning => "text-warning-500",
            MessageWarningLevel::Danger => "text-error-500",
        }
    }
}
