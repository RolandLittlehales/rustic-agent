//! Simple tool execution system
//!
//! Provides a minimal tool execution framework following YAGNI principles.

pub mod execution;

// Re-export main types for convenience
pub use execution::{
    ToolExecutionContext, ToolExecutionEngine,
    ToolRequest, ToolResultMetadata
};

// Re-export existing tool types for backward compatibility
use crate::claude::types::{PropertySchema, Tool, ToolInputSchema};
use crate::claude::whitelist::{validate_path, FileOperation, WhitelistConfig};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait AgentTool: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> ToolInputSchema;
    async fn execute(&self, input: Value) -> Result<String>;
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>);
}

#[derive(Debug)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn AgentTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T: AgentTool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Box::new(tool));
    }


    pub fn get_all_tools(&self) -> Vec<Tool> {
        self.tools
            .values()
            .map(|tool| Tool {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                input_schema: tool.input_schema(),
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        for tool in self.tools.values_mut() {
            tool.set_whitelist(whitelist.clone());
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ReadFileTool {
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl ReadFileTool {
    pub fn new() -> Self {
        Self { whitelist: None }
    }
}

#[async_trait]
impl AgentTool for ReadFileTool {
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.whitelist = Some(whitelist);
    }

    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read the contents of a file from the filesystem"
    }

    fn input_schema(&self) -> ToolInputSchema {
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            PropertySchema {
                r#type: "string".to_string(),
                description: "The path to the file to read".to_string(),
                items: None,
            },
        );

        ToolInputSchema {
            r#type: "object".to_string(),
            properties,
            required: vec!["path".to_string()],
            additional_properties: false,
        }
    }

    async fn execute(&self, input: Value) -> Result<String> {
        // Enhanced input validation
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        // Path validation
        if path_str.is_empty() {
            return Err(anyhow::anyhow!("Path parameter cannot be empty"));
        }

        if path_str.len() > 4096 {
            return Err(anyhow::anyhow!(
                "Path parameter too long (max 4096 characters)"
            ));
        }

        // Prevent null bytes in paths (security issue)
        if path_str.contains('\0') {
            return Err(anyhow::anyhow!("Invalid path: contains null bytes"));
        }

        // Validate and sanitize the path using whitelist
        let safe_path = if let Some(whitelist) = &self.whitelist {
            let whitelist_guard = whitelist.read().await;
            validate_path(path_str, &whitelist_guard, FileOperation::Read)?
        } else {
            // Fallback to basic validation if no whitelist is set
            let current_dir = std::env::current_dir()
                .map_err(|e| anyhow::anyhow!("Cannot determine current directory: {}", e))?;
            let path = Path::new(path_str);
            let canonical_path = if path.is_absolute() {
                path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
            } else {
                current_dir
                    .join(path)
                    .canonicalize()
                    .unwrap_or_else(|_| current_dir.join(path))
            };

            if !canonical_path.starts_with(&current_dir) {
                return Err(anyhow::anyhow!(
                    "Access denied: Path '{}' is outside allowed directory",
                    canonical_path.display()
                ));
            }
            canonical_path
        };

        // Input validation
        if safe_path.to_string_lossy().len() > 4096 {
            return Err(anyhow::anyhow!("File path too long (max 4096 characters)"));
        }

        // Use async file operations
        match tokio::fs::read_to_string(&safe_path).await {
            Ok(content) => {
                // Validate file size
                const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB limit for reading
                if content.len() > MAX_FILE_SIZE {
                    return Err(anyhow::anyhow!(
                        "File too large: {} bytes (limit: {} bytes)",
                        content.len(),
                        MAX_FILE_SIZE
                    ));
                }
                Ok(content)
            }
            Err(e) => Err(anyhow::anyhow!(
                "Failed to read file '{}': {}",
                safe_path.display(),
                e
            )),
        }
    }
}

#[derive(Debug)]
pub struct WriteFileTool {
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl WriteFileTool {
    pub fn new() -> Self {
        Self { whitelist: None }
    }
}

#[async_trait]
impl AgentTool for WriteFileTool {
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.whitelist = Some(whitelist);
    }

    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file on the filesystem"
    }

    fn input_schema(&self) -> ToolInputSchema {
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            PropertySchema {
                r#type: "string".to_string(),
                description: "The path to the file to write".to_string(),
                items: None,
            },
        );
        properties.insert(
            "content".to_string(),
            PropertySchema {
                r#type: "string".to_string(),
                description: "The content to write to the file".to_string(),
                items: None,
            },
        );

        ToolInputSchema {
            r#type: "object".to_string(),
            properties,
            required: vec!["path".to_string(), "content".to_string()],
            additional_properties: false,
        }
    }

    async fn execute(&self, input: Value) -> Result<String> {
        // Enhanced input validation
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'content' parameter"))?;

        // Path validation
        if path_str.is_empty() {
            return Err(anyhow::anyhow!("Path parameter cannot be empty"));
        }

        if path_str.len() > 4096 {
            return Err(anyhow::anyhow!(
                "Path parameter too long (max 4096 characters)"
            ));
        }

        // Prevent null bytes in paths (security issue)
        if path_str.contains('\0') {
            return Err(anyhow::anyhow!("Invalid path: contains null bytes"));
        }

        // Content validation
        if content.len() > 50 * 1024 * 1024 {
            return Err(anyhow::anyhow!(
                "Content too large: {} bytes (limit: 50MB)",
                content.len()
            ));
        }

        // Validate and sanitize the path using whitelist
        let safe_path = if let Some(whitelist) = &self.whitelist {
            let whitelist_guard = whitelist.read().await;
            validate_path(path_str, &whitelist_guard, FileOperation::Write)?
        } else {
            // Fallback to basic validation if no whitelist is set
            let current_dir = std::env::current_dir()
                .map_err(|e| anyhow::anyhow!("Cannot determine current directory: {}", e))?;
            let path = Path::new(path_str);
            let canonical_path = if path.is_absolute() {
                path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
            } else {
                current_dir
                    .join(path)
                    .canonicalize()
                    .unwrap_or_else(|_| current_dir.join(path))
            };

            if !canonical_path.starts_with(&current_dir) {
                return Err(anyhow::anyhow!(
                    "Access denied: Path '{}' is outside allowed directory",
                    canonical_path.display()
                ));
            }
            canonical_path
        };

        // Check if we're trying to overwrite important files
        let file_name = safe_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let protected_files = [
            "Cargo.toml",
            "package.json",
            ".env",
            ".gitignore",
            "tauri.conf.json",
            "main.rs",
            "lib.rs",
        ];

        if protected_files.contains(&file_name) {
            return Err(anyhow::anyhow!(
                "Access denied: Cannot overwrite protected file '{}'",
                file_name
            ));
        }

        // Use async file operations
        match tokio::fs::write(&safe_path, content).await {
            Ok(_) => Ok(format!(
                "Successfully wrote {} bytes to '{}'",
                content.len(),
                safe_path.display()
            )),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to write file '{}': {}",
                safe_path.display(),
                e
            )),
        }
    }
}

#[derive(Debug)]
pub struct ListDirectoryTool {
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl ListDirectoryTool {
    pub fn new() -> Self {
        Self { whitelist: None }
    }
}

#[async_trait]
impl AgentTool for ListDirectoryTool {
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.whitelist = Some(whitelist);
    }

    fn name(&self) -> &str {
        "list_directory"
    }

    fn description(&self) -> &str {
        "List the contents of a directory"
    }

    fn input_schema(&self) -> ToolInputSchema {
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            PropertySchema {
                r#type: "string".to_string(),
                description: "The path to the directory to list".to_string(),
                items: None,
            },
        );

        ToolInputSchema {
            r#type: "object".to_string(),
            properties,
            required: vec!["path".to_string()],
            additional_properties: false,
        }
    }

    async fn execute(&self, input: Value) -> Result<String> {
        // Input validation and sanitization
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        // Additional path validation
        if path_str.is_empty() {
            return Err(anyhow::anyhow!("Path parameter cannot be empty"));
        }

        if path_str.len() > 4096 {
            return Err(anyhow::anyhow!(
                "Path parameter too long (max 4096 characters)"
            ));
        }

        // Validate and sanitize the path using whitelist
        let safe_path = if let Some(whitelist) = &self.whitelist {
            let whitelist_guard = whitelist.read().await;
            validate_path(path_str, &whitelist_guard, FileOperation::List)?
        } else {
            // Fallback to basic validation if no whitelist is set
            let current_dir = std::env::current_dir()
                .map_err(|e| anyhow::anyhow!("Cannot determine current directory: {}", e))?;
            let path = Path::new(path_str);
            let canonical_path = if path.is_absolute() {
                path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
            } else {
                current_dir
                    .join(path)
                    .canonicalize()
                    .unwrap_or_else(|_| current_dir.join(path))
            };

            if !canonical_path.starts_with(&current_dir) {
                return Err(anyhow::anyhow!(
                    "Access denied: Path '{}' is outside allowed directory",
                    canonical_path.display()
                ));
            }
            canonical_path
        };

        // Use async file operations
        match tokio::fs::read_dir(&safe_path).await {
            Ok(mut entries) => {
                let mut result = Vec::new();
                let mut count = 0;
                const MAX_ENTRIES: usize = 1000; // Limit directory listing

                while let Some(entry) = entries.next_entry().await.transpose() {
                    if count >= MAX_ENTRIES {
                        result.push(format!(
                            "... (truncated, showing first {} entries)",
                            MAX_ENTRIES
                        ));
                        break;
                    }

                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();

                            // Skip hidden files and sensitive directories
                            if name.starts_with('.') && !name.eq(".") && !name.eq("..") {
                                continue;
                            }

                            let file_type = match entry.file_type().await {
                                Ok(ft) if ft.is_dir() => "directory",
                                Ok(_) => "file",
                                Err(_) => "unknown",
                            };
                            result.push(format!("{} ({})", name, file_type));
                            count += 1;
                        }
                        Err(e) => result.push(format!("Error reading entry: {}", e)),
                    }
                }
                Ok(result.join("\n"))
            }
            Err(e) => Err(anyhow::anyhow!(
                "Failed to read directory '{}': {}",
                safe_path.display(),
                e
            )),
        }
    }
}
