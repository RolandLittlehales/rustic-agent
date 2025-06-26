use crate::claude::types::{PropertySchema, Tool, ToolInputSchema};
use crate::constants;
use crate::logging::{ListDirectoryLogEntry, ReadFileLogEntry, WriteFileLogEntry};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use uuid::Uuid;

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
        current_dir
            .join(path)
            .canonicalize()
            .unwrap_or_else(|_| current_dir.join(path))
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
    for pattern in constants::FORBIDDEN_PATH_PATTERNS {
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
        let start_time = Instant::now();
        let execution_id = Uuid::new_v4().to_string();

        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let mut log_entry = ReadFileLogEntry::new(&execution_id, path_str);

        // Validate and sanitize the path
        let safe_path = match validate_and_sanitize_path(path_str) {
            Ok(path) => path,
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("path_validation", e.to_string(), false);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                return Err(e);
            }
        };

        // Additional size check to prevent reading huge files
        let file_size = match std::fs::metadata(&safe_path) {
            Ok(metadata) => {
                let size = metadata.len();
                if size > constants::MAX_FILE_SIZE_BYTES {
                    let error = anyhow::anyhow!(
                        "File too large: {} bytes (limit: {} bytes)",
                        size,
                        constants::MAX_FILE_SIZE_BYTES
                    );
                    log_entry.base =
                        log_entry
                            .base
                            .with_error("file_too_large", error.to_string(), false);
                    log_entry.base.execution_time = start_time.elapsed();
                    log_entry.log();
                    return Err(error);
                }
                size
            }
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("metadata_error", e.to_string(), true);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                return Err(anyhow::anyhow!("Failed to read file metadata: {}", e));
            }
        };

        match std::fs::read_to_string(&safe_path) {
            Ok(content) => {
                let duration = start_time.elapsed();
                log_entry = log_entry.with_success(file_size, &content, duration);
                log_entry.log();
                Ok(content)
            }
            Err(e) => {
                log_entry.base = log_entry.base.with_error("read_error", e.to_string(), true);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                Err(anyhow::anyhow!(
                    "Failed to read file '{}': {}",
                    safe_path.display(),
                    e
                ))
            }
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
        let start_time = Instant::now();
        let execution_id = Uuid::new_v4().to_string();

        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'content' parameter"))?;

        let content_size = content.len() as u64;
        let mut log_entry = WriteFileLogEntry::new(&execution_id, path_str, content_size);

        // Validate and sanitize the path
        let safe_path = match validate_and_sanitize_path(path_str) {
            Ok(path) => path,
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("path_validation", e.to_string(), false);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                return Err(e);
            }
        };

        // Check if file exists
        let file_existed = safe_path.exists();
        log_entry.file_existed = file_existed;

        // Content size validation
        if content.len() > constants::MAX_CONTENT_SIZE_BYTES {
            let error = anyhow::anyhow!(
                "Content too large: {} bytes (limit: {} bytes)",
                content.len(),
                constants::MAX_CONTENT_SIZE_BYTES
            );
            log_entry.base =
                log_entry
                    .base
                    .with_error("content_too_large", error.to_string(), false);
            log_entry.base.execution_time = start_time.elapsed();
            log_entry.log();
            return Err(error);
        }

        // Check if we're trying to overwrite important files
        let file_name = safe_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if constants::PROTECTED_FILES.contains(&file_name) {
            let error = anyhow::anyhow!(
                "Access denied: Cannot overwrite protected file '{}'",
                file_name
            );
            log_entry.base = log_entry
                .base
                .with_error("protected_file", error.to_string(), false);
            log_entry.base.execution_time = start_time.elapsed();
            log_entry.log();
            return Err(error);
        }

        match std::fs::write(&safe_path, content) {
            Ok(_) => {
                let duration = start_time.elapsed();
                log_entry = log_entry.with_success(file_existed, duration);
                log_entry.log();
                Ok(format!(
                    "Successfully wrote {} bytes to '{}'",
                    content.len(),
                    safe_path.display()
                ))
            }
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("write_error", e.to_string(), true);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                Err(anyhow::anyhow!(
                    "Failed to write file '{}': {}",
                    safe_path.display(),
                    e
                ))
            }
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
        let start_time = Instant::now();
        let execution_id = Uuid::new_v4().to_string();

        let path_str = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let mut log_entry = ListDirectoryLogEntry::new(&execution_id, path_str);

        // Validate and sanitize the path
        let safe_path = match validate_and_sanitize_path(path_str) {
            Ok(path) => path,
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("path_validation", e.to_string(), false);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                return Err(e);
            }
        };

        match std::fs::read_dir(&safe_path) {
            Ok(entries) => {
                let mut result = Vec::new();
                let mut count = 0;
                let mut file_count = 0;
                let mut dir_count = 0;
                const MAX_ENTRIES: usize = constants::MAX_DIRECTORY_ENTRIES;

                for entry in entries {
                    if count >= MAX_ENTRIES {
                        result.push(constants::format_directory_truncation_message(MAX_ENTRIES));
                        break;
                    }

                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();

                            // Skip hidden files and sensitive directories
                            if name.starts_with('.') && !name.eq(".") && !name.eq("..") {
                                continue;
                            }

                            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                            if is_dir {
                                dir_count += 1;
                                result.push(format!("{} (directory)", name));
                            } else {
                                file_count += 1;
                                result.push(format!("{} (file)", name));
                            }
                            count += 1;
                        }
                        Err(e) => result.push(format!("Error reading entry: {}", e)),
                    }
                }

                let duration = start_time.elapsed();
                log_entry = log_entry.with_success(file_count, dir_count, duration);
                log_entry.log();

                Ok(result.join("\n"))
            }
            Err(e) => {
                log_entry.base = log_entry
                    .base
                    .with_error("read_dir_error", e.to_string(), true);
                log_entry.base.execution_time = start_time.elapsed();
                log_entry.log();
                Err(anyhow::anyhow!(
                    "Failed to read directory '{}': {}",
                    safe_path.display(),
                    e
                ))
            }
        }
    }
}
