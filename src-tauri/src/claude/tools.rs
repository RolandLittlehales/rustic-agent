use crate::claude::types::{Tool, ToolInputSchema, PropertySchema};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub trait AgentTool: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> ToolInputSchema;
    fn execute(&self, input: Value) -> Result<String>;
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

    pub fn get_tool(&self, name: &str) -> Option<&dyn AgentTool> {
        self.tools.get(name).map(|tool| tool.as_ref())
    }

    pub fn execute_tool(&self, name: &str, input: Value) -> Result<String> {
        match self.get_tool(name) {
            Some(tool) => tool.execute(input),
            None => Err(anyhow::anyhow!("Tool '{}' not found", name)),
        }
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
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Security utilities for path validation
fn validate_and_sanitize_path(path: &str) -> Result<PathBuf> {
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
    
    // Additional security checks
    let path_str = canonical_path.to_string_lossy();
    
    // Block access to sensitive directories
    let forbidden_patterns = [
        "/etc/", "/root/", "/home/", "/var/", "/usr/", "/sys/", "/proc/",
        "\\Windows\\", "\\System32\\", "\\Users\\", "\\Program Files\\",
        ".ssh", ".aws", ".config", ".env", "id_rsa", "id_dsa", "id_ecdsa", "id_ed25519"
    ];
    
    for pattern in &forbidden_patterns {
        if path_str.contains(pattern) {
            return Err(anyhow::anyhow!(
                "Access denied: Path contains forbidden pattern '{}'", 
                pattern
            ));
        }
    }
    
    Ok(canonical_path)
}

#[derive(Debug)]
pub struct ReadFileTool;

impl AgentTool for ReadFileTool {
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

    fn execute(&self, input: Value) -> Result<String> {
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        // Validate and sanitize the path
        let safe_path = validate_and_sanitize_path(path_str)?;

        // Additional size check to prevent reading huge files
        if let Ok(metadata) = std::fs::metadata(&safe_path) {
            const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit
            if metadata.len() > MAX_FILE_SIZE {
                return Err(anyhow::anyhow!(
                    "File too large: {} bytes (limit: {} bytes)", 
                    metadata.len(), 
                    MAX_FILE_SIZE
                ));
            }
        }

        match std::fs::read_to_string(&safe_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(anyhow::anyhow!("Failed to read file '{}': {}", safe_path.display(), e)),
        }
    }
}

#[derive(Debug)]
pub struct WriteFileTool;

impl AgentTool for WriteFileTool {
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

    fn execute(&self, input: Value) -> Result<String> {
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'content' parameter"))?;

        // Validate and sanitize the path
        let safe_path = validate_and_sanitize_path(path_str)?;

        // Content size validation
        const MAX_CONTENT_SIZE: usize = 50 * 1024 * 1024; // 50MB limit
        if content.len() > MAX_CONTENT_SIZE {
            return Err(anyhow::anyhow!(
                "Content too large: {} bytes (limit: {} bytes)", 
                content.len(), 
                MAX_CONTENT_SIZE
            ));
        }

        // Check if we're trying to overwrite important files
        let file_name = safe_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let protected_files = [
            "Cargo.toml", "package.json", ".env", ".gitignore", 
            "tauri.conf.json", "main.rs", "lib.rs"
        ];
        
        if protected_files.contains(&file_name) {
            return Err(anyhow::anyhow!(
                "Access denied: Cannot overwrite protected file '{}'", 
                file_name
            ));
        }

        match std::fs::write(&safe_path, content) {
            Ok(_) => Ok(format!("Successfully wrote {} bytes to '{}'", content.len(), safe_path.display())),
            Err(e) => Err(anyhow::anyhow!("Failed to write file '{}': {}", safe_path.display(), e)),
        }
    }
}

#[derive(Debug)]
pub struct ListDirectoryTool;

impl AgentTool for ListDirectoryTool {
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

    fn execute(&self, input: Value) -> Result<String> {
        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        // Validate and sanitize the path
        let safe_path = validate_and_sanitize_path(path_str)?;

        match std::fs::read_dir(&safe_path) {
            Ok(entries) => {
                let mut result = Vec::new();
                let mut count = 0;
                const MAX_ENTRIES: usize = 1000; // Limit directory listing
                
                for entry in entries {
                    if count >= MAX_ENTRIES {
                        result.push(format!("... (truncated, showing first {} entries)", MAX_ENTRIES));
                        break;
                    }
                    
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();
                            
                            // Skip hidden files and sensitive directories
                            if name.starts_with('.') && !name.eq(".") && !name.eq("..") {
                                continue;
                            }
                            
                            let file_type = if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                "directory"
                            } else {
                                "file"
                            };
                            result.push(format!("{} ({})", name, file_type));
                            count += 1;
                        }
                        Err(e) => result.push(format!("Error reading entry: {}", e)),
                    }
                }
                Ok(result.join("\n"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to read directory '{}': {}", safe_path.display(), e)),
        }
    }
}