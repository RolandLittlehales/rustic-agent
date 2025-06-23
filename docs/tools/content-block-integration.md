# ContentBlock Integration Guide for Tools

## Overview

This guide explains how to integrate the ContentBlock system when developing new tools for the Claude agent. Tools must properly handle ContentBlock inputs and generate appropriate outputs.

## Tool Architecture with ContentBlocks

### Tool Trait

All tools implement the `AgentTool` trait:

```rust
#[async_trait]
pub trait AgentTool: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, input: serde_json::Value) -> Result<String>;
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>);
}
```

### ContentBlock Flow in Tools

```
ToolUse ContentBlock → Tool Registry → Tool Execute → Result String → ToolResult ContentBlock
```

## Implementation Guide

### Step 1: Define Your Tool

```rust
use crate::claude::tools::{AgentTool, async_trait};
use crate::claude::whitelist::WhitelistConfig;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyToolInput {
    path: String,
    options: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct MyCustomTool {
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

impl MyCustomTool {
    pub fn new() -> Self {
        Self { whitelist: None }
    }
}
```

### Step 2: Implement AgentTool Trait

```rust
#[async_trait]
impl AgentTool for MyCustomTool {
    fn name(&self) -> &str {
        "my_custom_tool"
    }

    fn description(&self) -> &str {
        "Performs a custom operation on files"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file"
                },
                "options": {
                    "type": "object",
                    "description": "Optional configuration"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        // Parse input
        let params: MyToolInput = serde_json::from_value(input)
            .map_err(|e| anyhow!("Invalid input: {}", e))?;

        // Validate with whitelist if applicable
        if let Some(whitelist) = &self.whitelist {
            let whitelist_guard = whitelist.read().unwrap();
            validate_path(&params.path, &whitelist_guard, FileOperation::Read)?;
        }

        // Perform the operation
        let result = perform_operation(&params.path, params.options).await?;

        // Return string result (will be wrapped in ToolResult ContentBlock)
        Ok(format!("Operation completed: {}", result))
    }

    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>) {
        self.whitelist = Some(whitelist);
    }
}
```

### Step 3: Handle Errors Properly

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    // Use specific error types for better error handling
    let params: MyToolInput = serde_json::from_value(input)
        .map_err(|e| ClaudeError::ValidationError {
            field: "input".to_string(),
            message: format!("Invalid tool input: {}", e),
            context: Some(ErrorContext::new("tool_execution")
                .add_metadata("tool", self.name())),
        })?;

    // Perform operation with proper error context
    match perform_operation(&params.path).await {
        Ok(result) => Ok(result),
        Err(e) => Err(ClaudeError::ToolError {
            tool_name: self.name().to_string(),
            message: e.to_string(),
            context: Some(ErrorContext::new("tool_operation")
                .add_metadata("path", &params.path)),
        }.into())
    }
}
```

## Best Practices

### 1. Input Validation

Always validate inputs thoroughly:

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    // Validate JSON structure
    let params: MyToolInput = serde_json::from_value(input)?;
    
    // Validate field contents
    if params.path.is_empty() {
        return Err(anyhow!("Path cannot be empty"));
    }
    
    // Security validation
    if params.path.contains("..") {
        return Err(anyhow!("Path traversal not allowed"));
    }
    
    // Continue with execution...
}
```

### 2. Async Operations

Use async properly for I/O operations:

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    let params: FileReadInput = serde_json::from_value(input)?;
    
    // Use tokio for async file operations
    let contents = tokio::fs::read_to_string(&params.path)
        .await
        .map_err(|e| anyhow!("Failed to read file: {}", e))?;
    
    Ok(contents)
}
```

### 3. Whitelist Integration

Always respect whitelist configuration:

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    let params: FileOperationInput = serde_json::from_value(input)?;
    
    // Check whitelist before any file operation
    if let Some(whitelist) = &self.whitelist {
        let whitelist_guard = whitelist.read().unwrap();
        
        // Use the validation utilities
        let canonical_path = validate_path(
            &params.path, 
            &whitelist_guard, 
            FileOperation::Read
        )?;
        
        // Use canonical path for operation
        let contents = tokio::fs::read_to_string(canonical_path).await?;
        Ok(contents)
    } else {
        Err(anyhow!("Whitelist not configured"))
    }
}
```

### 4. Result Formatting

Format results for clarity:

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    let results = perform_analysis(&input).await?;
    
    // Format results clearly
    let mut output = String::new();
    output.push_str("Analysis Results:\n");
    output.push_str(&format!("- Files analyzed: {}\n", results.file_count));
    output.push_str(&format!("- Issues found: {}\n", results.issue_count));
    
    if !results.issues.is_empty() {
        output.push_str("\nDetailed Issues:\n");
        for issue in &results.issues {
            output.push_str(&format!("  - {}: {}\n", issue.file, issue.description));
        }
    }
    
    Ok(output)
}
```

## Common Tool Patterns

### File Operations Tool

```rust
#[derive(Debug)]
pub struct FileOperationTool {
    whitelist: Option<Arc<RwLock<WhitelistConfig>>>,
}

#[async_trait]
impl AgentTool for FileOperationTool {
    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        let params: FileOpParams = serde_json::from_value(input)?;
        
        // Validate path with whitelist
        let path = self.validate_and_canonicalize_path(&params.path)?;
        
        match params.operation.as_str() {
            "read" => {
                let contents = tokio::fs::read_to_string(&path).await?;
                Ok(contents)
            }
            "write" => {
                tokio::fs::write(&path, &params.content.unwrap_or_default()).await?;
                Ok("File written successfully".to_string())
            }
            "list" => {
                let entries = list_directory(&path).await?;
                Ok(format_directory_listing(entries))
            }
            _ => Err(anyhow!("Unknown operation: {}", params.operation))
        }
    }
}
```

### External API Tool

```rust
#[derive(Debug)]
pub struct ApiTool {
    client: reqwest::Client,
    base_url: String,
}

#[async_trait]
impl AgentTool for ApiTool {
    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        let params: ApiParams = serde_json::from_value(input)?;
        
        // Build request
        let url = format!("{}/{}", self.base_url, params.endpoint);
        let response = self.client
            .request(params.method.parse()?, &url)
            .json(&params.body)
            .send()
            .await?;
        
        // Handle response
        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(anyhow!("API error: {}", response.status()))
        }
    }
}
```

### Data Processing Tool

```rust
#[derive(Debug)]
pub struct DataProcessorTool;

#[async_trait]
impl AgentTool for DataProcessorTool {
    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        let params: ProcessParams = serde_json::from_value(input)?;
        
        // Process data in chunks for large datasets
        let data = load_data(&params.source).await?;
        let chunks = data.chunks(1000);
        
        let mut results = Vec::new();
        for chunk in chunks {
            let processed = process_chunk(chunk, &params.operation).await?;
            results.extend(processed);
        }
        
        // Format results
        Ok(serde_json::to_string_pretty(&results)?)
    }
}
```

## Testing Tools with ContentBlocks

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_execution() {
        let tool = MyCustomTool::new();
        
        let input = json!({
            "path": "/tmp/test.txt",
            "options": {
                "format": "json"
            }
        });
        
        let result = tool.execute(input).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Operation completed"));
    }

    #[tokio::test]
    async fn test_invalid_input() {
        let tool = MyCustomTool::new();
        
        let input = json!({
            "invalid_field": "value"
        });
        
        let result = tool.execute(input).await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid input"));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_tool_with_contentblock_flow() {
    // Create a ToolUse ContentBlock
    let tool_use = ContentBlock::ToolUse {
        id: "test_123".to_string(),
        name: "my_custom_tool".to_string(),
        input: json!({
            "path": "/tmp/test.txt"
        }),
    };
    
    // Execute through tool registry
    let registry = create_test_registry();
    let result = match tool_use {
        ContentBlock::ToolUse { name, input, .. } => {
            registry.execute_tool(&name, input).await
        }
        _ => panic!("Expected ToolUse block"),
    };
    
    // Verify result can be wrapped in ToolResult
    assert!(result.is_ok());
    let tool_result = ContentBlock::ToolResult {
        tool_use_id: "test_123".to_string(),
        content: result.unwrap(),
        is_error: Some(false),
    };
    
    // Verify serialization
    let json = serde_json::to_string(&tool_result).unwrap();
    assert!(json.contains("tool_result"));
}
```

## Debugging Tools

### Logging Best Practices

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    // Log tool invocation
    log::debug!("Tool {} invoked with input: {:?}", self.name(), input);
    
    let params: MyToolInput = serde_json::from_value(input)?;
    log::info!("Processing request for path: {}", params.path);
    
    match perform_operation(&params).await {
        Ok(result) => {
            log::info!("Tool {} completed successfully", self.name());
            Ok(result)
        }
        Err(e) => {
            log::error!("Tool {} failed: {}", self.name(), e);
            Err(e)
        }
    }
}
```

### Error Context

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    let context = ErrorContext::new("tool_execution")
        .add_metadata("tool_name", self.name())
        .add_metadata("input_size", &input.to_string().len().to_string());
    
    match self.perform_operation(input).await {
        Ok(result) => Ok(result),
        Err(e) => Err(ClaudeError::ToolError {
            tool_name: self.name().to_string(),
            message: e.to_string(),
            context: Some(context.with_retry_count(0)),
        }.into())
    }
}
```

## Performance Considerations

### Streaming Large Results

For tools that generate large outputs:

```rust
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    let params: LargeFileParams = serde_json::from_value(input)?;
    
    // For very large files, consider chunking
    let file_size = tokio::fs::metadata(&params.path).await?.len();
    
    if file_size > 10_000_000 { // 10MB
        // Return summary instead of full content
        Ok(format!(
            "File is {} bytes. Use specialized tools for large file processing.",
            file_size
        ))
    } else {
        // Normal processing for smaller files
        let contents = tokio::fs::read_to_string(&params.path).await?;
        Ok(contents)
    }
}
```

### Caching Results

For expensive operations:

```rust
#[derive(Debug)]
pub struct CachedTool {
    cache: Arc<Mutex<HashMap<String, (Instant, String)>>>,
    cache_duration: Duration,
}

#[async_trait]
impl AgentTool for CachedTool {
    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        let cache_key = serde_json::to_string(&input)?;
        
        // Check cache
        {
            let cache = self.cache.lock().unwrap();
            if let Some((timestamp, result)) = cache.get(&cache_key) {
                if timestamp.elapsed() < self.cache_duration {
                    return Ok(result.clone());
                }
            }
        }
        
        // Perform expensive operation
        let result = expensive_operation(input).await?;
        
        // Update cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(cache_key, (Instant::now(), result.clone()));
        }
        
        Ok(result)
    }
}
```

## Migration Guide

### From String-Based Tools

If you have existing tools that work with strings:

```rust
// Old tool interface
fn execute(&self, input: &str) -> Result<String> {
    // Parse input manually
    let parts: Vec<&str> = input.split(',').collect();
    // ...
}

// New ContentBlock-aware interface
async fn execute(&self, input: serde_json::Value) -> Result<String> {
    // Structured input parsing
    let params: ToolParams = serde_json::from_value(input)?;
    // ...
}
```

### Handling Legacy Tools

Wrapper for legacy tools:

```rust
pub struct LegacyToolWrapper<T: LegacyTool> {
    inner: T,
}

#[async_trait]
impl<T: LegacyTool> AgentTool for LegacyToolWrapper<T> {
    async fn execute(&self, input: serde_json::Value) -> Result<String> {
        // Convert structured input to legacy format
        let legacy_input = convert_to_legacy_format(input)?;
        
        // Call legacy tool
        let result = self.inner.execute_legacy(&legacy_input)?;
        
        // Return result (will be wrapped in ContentBlock)
        Ok(result)
    }
}
```

## See Also

- [ContentBlock Architecture](../architecture/content-block-system.md)
- [API Reference: ContentBlock Types](../api/content-block-types.md)
- [Tool Development Guide](../development/creating-tools.md)
- [Security: Whitelist System](../security/whitelist-system.md)