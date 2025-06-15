// Security utilities and configuration
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct SecurityConfig {
    pub allowed_file_extensions: HashSet<String>,
    pub blocked_paths: Vec<String>,
    pub max_file_size: u64,
    pub max_content_size: usize,
    pub rate_limit_seconds: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        let mut allowed_extensions = HashSet::new();
        // Allow common development file types
        for ext in &[
            "rs", "js", "ts", "html", "css", "json", "toml", "md", "txt", 
            "yml", "yaml", "xml", "svg", "png", "jpg", "jpeg", "gif", "ico"
        ] {
            allowed_extensions.insert(ext.to_string());
        }

        Self {
            allowed_file_extensions: allowed_extensions,
            blocked_paths: vec![
                "/etc".to_string(),
                "/root".to_string(),
                "/var".to_string(),
                "/usr".to_string(),
                "/sys".to_string(),
                "/proc".to_string(),
                "\\Windows".to_string(),
                "\\System32".to_string(),
                "\\Users".to_string(),
                ".ssh".to_string(),
                ".aws".to_string(),
                ".env".to_string(),
                "id_rsa".to_string(),
                "id_dsa".to_string(),
                "id_ecdsa".to_string(),
                "id_ed25519".to_string(),
            ],
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_content_size: 50 * 1024 * 1024, // 50MB
            rate_limit_seconds: 1,
        }
    }
}

pub fn validate_file_access(path: &str, config: &SecurityConfig) -> Result<PathBuf> {
    let path = Path::new(path);
    
    // Get current working directory as the allowed base
    let current_dir = std::env::current_dir()
        .map_err(|e| anyhow::anyhow!("Cannot determine current directory: {}", e))?;
    
    // Resolve the path (handles . and .. components)
    let canonical_path = if path.is_absolute() {
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
    } else {
        current_dir.join(path).canonicalize().unwrap_or_else(|_| current_dir.join(path))
    };
    
    // Ensure the path is within the current directory or its subdirectories
    if !canonical_path.starts_with(&current_dir) {
        return Err(anyhow::anyhow!(
            "Access denied: Path '{}' is outside allowed directory", 
            path.display()
        ));
    }
    
    // Check blocked paths
    let path_str = canonical_path.to_string_lossy().to_lowercase();
    for blocked in &config.blocked_paths {
        if path_str.contains(&blocked.to_lowercase()) {
            return Err(anyhow::anyhow!(
                "Access denied: Path contains blocked pattern '{}'", 
                blocked
            ));
        }
    }
    
    // Check file extension if it's a file
    if let Some(extension) = canonical_path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if !config.allowed_file_extensions.contains(&ext) {
            return Err(anyhow::anyhow!(
                "Access denied: File extension '{}' not allowed", 
                ext
            ));
        }
    }
    
    Ok(canonical_path)
}

pub fn sanitize_api_key_for_logging(api_key: &str) -> String {
    if api_key.is_empty() {
        "[EMPTY]".to_string()
    } else if api_key.len() < 10 {
        "[REDACTED]".to_string()
    } else {
        format!("[REDACTED-{}-****]", &api_key[..6])
    }
}

pub fn validate_content_size(content: &str, max_size: usize) -> Result<()> {
    if content.len() > max_size {
        return Err(anyhow::anyhow!(
            "Content too large: {} bytes (limit: {} bytes)", 
            content.len(), 
            max_size
        ));
    }
    Ok(())
}

pub fn sanitize_html_content(content: &str) -> String {
    // Basic HTML entity encoding
    content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_validate_file_access_safe_path() {
        let config = SecurityConfig::default();
        let current_dir = env::current_dir().unwrap();
        let test_file = current_dir.join("test.rs");
        
        let result = validate_file_access("test.rs", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_file);
    }

    #[test]
    fn test_validate_file_access_blocked_path() {
        let config = SecurityConfig::default();
        
        let result = validate_file_access("/etc/passwd", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_sanitize_api_key() {
        assert_eq!(sanitize_api_key_for_logging(""), "[EMPTY]");
        assert_eq!(sanitize_api_key_for_logging("short"), "[REDACTED]");
        assert_eq!(sanitize_api_key_for_logging("sk-ant-1234567890"), "[REDACTED-sk-ant-****]");
    }

    #[test]
    fn test_sanitize_html() {
        let malicious = r#"<script>alert('xss')</script>"#;
        let sanitized = sanitize_html_content(malicious);
        assert!(!sanitized.contains("<script"));
        assert!(sanitized.contains("&lt;script&gt;"));
    }
}