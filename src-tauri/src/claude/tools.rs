use crate::claude::types::{Tool, ToolInputSchema, PropertySchema};
use serde_json::Value;
use std::collections::HashMap;
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
        }
    }

    fn execute(&self, input: Value) -> Result<String> {
        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        match std::fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(anyhow::anyhow!("Failed to read file '{}': {}", path, e)),
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
        }
    }

    fn execute(&self, input: Value) -> Result<String> {
        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'content' parameter"))?;

        match std::fs::write(path, content) {
            Ok(_) => Ok(format!("Successfully wrote {} bytes to '{}'", content.len(), path)),
            Err(e) => Err(anyhow::anyhow!("Failed to write file '{}': {}", path, e)),
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
        }
    }

    fn execute(&self, input: Value) -> Result<String> {
        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'path' parameter"))?;

        match std::fs::read_dir(path) {
            Ok(entries) => {
                let mut result = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let file_type = if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                "directory"
                            } else {
                                "file"
                            };
                            result.push(format!("{} ({})", name, file_type));
                        }
                        Err(e) => result.push(format!("Error reading entry: {}", e)),
                    }
                }
                Ok(result.join("\n"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to read directory '{}': {}", path, e)),
        }
    }
}