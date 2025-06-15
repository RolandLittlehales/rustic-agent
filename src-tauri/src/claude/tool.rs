use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub properties: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ToolInputSchema {
    pub fn new() -> Self {
        Self {
            schema_type: "object".to_string(),
            properties: HashMap::new(),
            required: None,
            description: None,
        }
    }

    pub fn with_property(mut self, name: impl Into<String>, property: Value) -> Self {
        self.properties.insert(name.into(), property);
        self
    }

    pub fn with_required(mut self, required: Vec<String>) -> Self {
        self.required = Some(required);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for ToolInputSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: ToolInputSchema,
}

impl Tool {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: ToolInputSchema::new(),
        }
    }

    pub fn with_input_schema(mut self, input_schema: ToolInputSchema) -> Self {
        self.input_schema = input_schema;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: Value,
}

impl ToolCall {
    pub fn new(id: impl Into<String>, name: impl Into<String>, input: Value) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            input,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub output: String,
    pub is_error: bool,
}

impl ToolResult {
    pub fn success(tool_call_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            output: output.into(),
            is_error: false,
        }
    }

    pub fn error(tool_call_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            output: error.into(),
            is_error: true,
        }
    }
}

/// Trait for implementing tool execution
pub trait ToolExecutor: Send + Sync {
    /// Execute a tool call and return the result
    fn execute(&self, tool_call: &ToolCall) -> Result<String, String>;
    
    /// Get the tool definition
    fn get_tool(&self) -> Tool;
}

/// Tool registry for managing available tools
#[derive(Debug, Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool executor
    pub fn register_tool(&mut self, executor: Box<dyn ToolExecutor>) {
        let tool = executor.get_tool();
        self.tools.insert(tool.name.clone(), executor);
    }

    /// Get all available tools
    pub fn get_tools(&self) -> Vec<Tool> {
        self.tools
            .values()
            .map(|executor| executor.get_tool())
            .collect()
    }

    /// Execute a tool call
    pub fn execute_tool(&self, tool_call: &ToolCall) -> ToolResult {
        match self.tools.get(&tool_call.name) {
            Some(executor) => {
                match executor.execute(tool_call) {
                    Ok(output) => ToolResult::success(&tool_call.id, output),
                    Err(error) => ToolResult::error(&tool_call.id, error),
                }
            }
            None => ToolResult::error(
                &tool_call.id,
                format!("Tool '{}' not found", tool_call.name)
            ),
        }
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get tool names
    pub fn get_tool_names(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

/// Utility functions for creating common tool input schemas
pub mod schema {
    use super::*;
    use serde_json::json;

    pub fn string_property(description: &str) -> Value {
        json!({
            "type": "string",
            "description": description
        })
    }

    pub fn number_property(description: &str) -> Value {
        json!({
            "type": "number",
            "description": description
        })
    }

    pub fn boolean_property(description: &str) -> Value {
        json!({
            "type": "boolean",
            "description": description
        })
    }

    pub fn array_property(description: &str, items: Value) -> Value {
        json!({
            "type": "array",
            "description": description,
            "items": items
        })
    }

    pub fn object_property(description: &str, properties: HashMap<String, Value>) -> Value {
        json!({
            "type": "object",
            "description": description,
            "properties": properties
        })
    }

    pub fn enum_property(description: &str, values: Vec<&str>) -> Value {
        json!({
            "type": "string",
            "description": description,
            "enum": values
        })
    }
}