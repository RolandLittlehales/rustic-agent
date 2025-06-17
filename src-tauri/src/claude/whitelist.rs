use anyhow::{anyhow, Result};
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistConfig {
    /// Set of allowed directories (canonical paths)
    allowed_directories: HashSet<PathBuf>,

    /// Whether to allow access to subdirectories of whitelisted paths
    allow_subdirectories: bool,

    /// Maximum depth for subdirectory access (0 = unlimited)
    max_depth: usize,

    /// Whether to follow symlinks
    follow_symlinks: bool,

    /// File patterns to always block (e.g., "*.env", "id_rsa*")
    blocked_patterns: Vec<String>,

    /// Maximum file size allowed for read operations (in bytes)
    max_file_size: u64,

    /// Whether the whitelist is enabled (false = fallback to current directory only)
    enabled: bool,
}

impl Default for WhitelistConfig {
    fn default() -> Self {
        Self {
            allowed_directories: HashSet::new(),
            allow_subdirectories: true,
            max_depth: 0, // Unlimited by default
            follow_symlinks: false,
            blocked_patterns: vec![
                "*.env".to_string(),
                ".env*".to_string(),
                "id_rsa*".to_string(),
                "id_dsa*".to_string(),
                "id_ecdsa*".to_string(),
                "id_ed25519*".to_string(),
                "*.key".to_string(),
                "*.pem".to_string(),
                "*.p12".to_string(),
                "*.pfx".to_string(),
                ".aws/*".to_string(),
                ".ssh/*".to_string(),
                ".gnupg/*".to_string(),
            ],
            max_file_size: 10 * 1024 * 1024, // 10MB
            enabled: false,
        }
    }
}

impl WhitelistConfig {
    /// Add a directory to the whitelist
    pub fn add_directory(&mut self, path: impl AsRef<Path>) -> Result<PathBuf> {
        let path = path.as_ref();

        // Canonicalize the path to prevent path traversal attacks
        let canonical_path = path
            .canonicalize()
            .map_err(|e| anyhow!("Failed to canonicalize path '{}': {}", path.display(), e))?;

        // Verify the path exists and is a directory
        if !canonical_path.is_dir() {
            return Err(anyhow!(
                "Path '{}' is not a directory",
                canonical_path.display()
            ));
        }

        self.allowed_directories.insert(canonical_path.clone());
        Ok(canonical_path)
    }

    /// Remove a directory from the whitelist
    pub fn remove_directory(&mut self, path: impl AsRef<Path>) -> Result<bool> {
        let path = path.as_ref();
        let canonical_path = path
            .canonicalize()
            .map_err(|e| anyhow!("Failed to canonicalize path '{}': {}", path.display(), e))?;

        Ok(self.allowed_directories.remove(&canonical_path))
    }

    /// List all whitelisted directories
    pub fn list_directories(&self) -> Vec<PathBuf> {
        let mut dirs: Vec<_> = self.allowed_directories.iter().cloned().collect();
        dirs.sort();
        dirs
    }

    /// Clear all whitelisted directories
    #[allow(dead_code)]
    pub fn clear_directories(&mut self) {
        self.allowed_directories.clear();
    }

    /// Check if a path matches any blocked pattern
    pub fn is_blocked_by_pattern(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in &self.blocked_patterns {
            if let Ok(glob_pattern) = Pattern::new(pattern) {
                if glob_pattern.matches(&path_str) {
                    return true;
                }
            }
        }

        // Also check individual path components
        for component in path.components() {
            if let Some(name) = component.as_os_str().to_str() {
                for pattern in &self.blocked_patterns {
                    if let Ok(glob_pattern) = Pattern::new(pattern) {
                        if glob_pattern.matches(name) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Calculate the depth of a path relative to a base path
    pub fn calculate_depth(base: &Path, path: &Path) -> Option<usize> {
        path.strip_prefix(base)
            .ok()
            .map(|relative| relative.components().count())
    }

    /// Check if a path is allowed according to whitelist rules
    pub fn is_path_allowed(&self, path: &Path) -> Result<()> {
        // If whitelist is disabled, only allow current directory and subdirectories
        if !self.enabled {
            let current_dir = std::env::current_dir()
                .map_err(|e| anyhow!("Cannot determine current directory: {}", e))?;

            if !path.starts_with(&current_dir) {
                return Err(anyhow!(
                    "Access denied: Path '{}' is outside the current directory",
                    path.display()
                ));
            }

            return Ok(());
        }

        // Check if path is within any whitelisted directory
        let mut is_allowed = false;

        for allowed_dir in &self.allowed_directories {
            if path == allowed_dir {
                is_allowed = true;
                break;
            }

            if self.allow_subdirectories && path.starts_with(allowed_dir) {
                // Check depth restriction
                if self.max_depth > 0 {
                    if let Some(depth) = Self::calculate_depth(allowed_dir, path) {
                        if depth > self.max_depth {
                            continue; // Try next allowed directory
                        }
                    }
                }

                is_allowed = true;
                break;
            }
        }

        if !is_allowed {
            return Err(anyhow!(
                "Access denied: Path '{}' is not in the whitelist",
                path.display()
            ));
        }

        Ok(())
    }

    /// Enable or disable the whitelist
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if the whitelist is enabled
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the maximum file size allowed
    pub fn max_file_size(&self) -> u64 {
        self.max_file_size
    }
}

/// Persistence module for saving/loading whitelist configuration
pub mod persistence {
    use super::*;
    use tauri::{AppHandle, Manager};

    /// Get the path to the whitelist configuration file
    pub fn get_config_path(app: &AppHandle) -> Result<PathBuf> {
        let config_dir = app
            .path()
            .app_config_dir()
            .map_err(|e| anyhow!("Could not determine app config directory: {}", e))?;

        // Ensure the directory exists
        std::fs::create_dir_all(&config_dir)?;

        Ok(config_dir.join("whitelist.json"))
    }

    /// Save whitelist configuration to disk
    pub async fn save(app: &AppHandle, config: &WhitelistConfig) -> Result<()> {
        let path = get_config_path(app)?;
        let json = serde_json::to_string_pretty(config)?;

        // Write atomically by writing to temp file first
        let temp_path = path.with_extension("tmp");
        std::fs::write(&temp_path, json)?;
        std::fs::rename(temp_path, path)?;

        Ok(())
    }

    /// Load whitelist configuration from disk
    pub async fn load(app: &AppHandle) -> Result<WhitelistConfig> {
        let path = get_config_path(app)?;

        if !path.exists() {
            // Return default config if file doesn't exist
            return Ok(WhitelistConfig::default());
        }

        let json = std::fs::read_to_string(path)?;
        let mut config: WhitelistConfig = serde_json::from_str(&json)?;

        // Validate loaded paths still exist
        config.allowed_directories.retain(|path| path.exists());

        Ok(config)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileOperation {
    Read,
    Write,
    List,
}

/// Validate and sanitize a path according to whitelist rules
pub fn validate_path(
    path: &str,
    whitelist: &WhitelistConfig,
    operation: FileOperation,
) -> Result<PathBuf> {
    let path = Path::new(path);

    // First, canonicalize the path to resolve any .. or . components
    let canonical_path = if path.is_absolute() {
        if whitelist.follow_symlinks {
            path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
        } else {
            // Custom canonicalization that doesn't follow symlinks
            canonicalize_no_follow(path)?
        }
    } else {
        // For relative paths, resolve from current directory
        let current_dir = std::env::current_dir()
            .map_err(|e| anyhow!("Cannot determine current directory: {}", e))?;

        let full_path = current_dir.join(path);
        if whitelist.follow_symlinks {
            full_path.canonicalize().unwrap_or(full_path)
        } else {
            canonicalize_no_follow(&full_path)?
        }
    };

    // Check if path is blocked by pattern
    if whitelist.is_blocked_by_pattern(&canonical_path) {
        return Err(anyhow!(
            "Access denied: Path '{}' matches a blocked pattern",
            canonical_path.display()
        ));
    }

    // Check if path is allowed by whitelist
    whitelist.is_path_allowed(&canonical_path)?;

    // Additional checks based on operation type
    match operation {
        FileOperation::Read => {
            // Check file size for read operations
            if let Ok(metadata) = std::fs::metadata(&canonical_path) {
                if metadata.len() > whitelist.max_file_size() {
                    return Err(anyhow!(
                        "File too large: {} bytes (limit: {} bytes)",
                        metadata.len(),
                        whitelist.max_file_size()
                    ));
                }
            }
        }
        FileOperation::Write => {
            // Additional write-specific checks could go here
        }
        FileOperation::List => {
            // Ensure it's a directory for list operations
            if !canonical_path.is_dir() {
                return Err(anyhow!(
                    "Path '{}' is not a directory",
                    canonical_path.display()
                ));
            }
        }
    }

    Ok(canonical_path)
}

/// Canonicalize a path without following symlinks
fn canonicalize_no_follow(path: &Path) -> Result<PathBuf> {
    use std::fs;

    let mut result = PathBuf::new();
    let mut current = PathBuf::new();

    for component in path.components() {
        match component {
            std::path::Component::Prefix(prefix) => {
                result.push(prefix.as_os_str());
            }
            std::path::Component::RootDir => {
                result.push("/");
            }
            std::path::Component::CurDir => {
                // Skip
            }
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::Normal(name) => {
                current.push(&result);
                current.push(name);

                // Check if this component is a symlink
                match fs::symlink_metadata(&current) {
                    Ok(metadata) => {
                        if metadata.file_type().is_symlink() {
                            return Err(anyhow!(
                                "Access denied: Path contains symlink at '{}'",
                                current.display()
                            ));
                        }
                    }
                    Err(_) => {
                        // Path doesn't exist yet, that's okay for write operations
                    }
                }

                result.push(name);
                current.clear();
            }
        }
    }

    Ok(result)
}
