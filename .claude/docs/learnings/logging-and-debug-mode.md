# Logging and Debug Mode Implementation

## Overview

This document captures the implementation of enhanced logging and debug mode functionality to address excessive verbose logging during development. The solution provides better control over log verbosity and clearer operation context.

## Problem Identified

During development with the two-step Claude API pattern, users experienced excessive logging messages like:
```
✅ [SUCCESS] operation=retry_operation...
Cleanup: Review error logs... 
```

These messages appeared for every successful API call, making it difficult to understand what was happening and cluttering the development experience.

## Root Cause

The two-step Claude API pattern introduced in the tool response architecture fix resulted in multiple API calls for each user request:

1. **First API Call**: User message → Claude (triggers tool uses)
2. **Tool Execution**: Execute requested tools
3. **Second API Call**: Tool results → Claude (for interpretation)

Each API call went through the error handler's retry mechanism, which logged success messages even for normal operations.

## Solution Implementation

### 1. Debug Mode Infrastructure

Added debug mode support throughout the application stack:

#### ErrorHandler Updates (`src/claude/error.rs`)
```rust
#[derive(Debug)]
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    circuit_breaker: Option<CircuitBreaker>,
    telemetry: ErrorTelemetry,
    debug_mode: bool,  // NEW: Debug mode flag
}

impl ErrorHandler {
    pub fn with_debug(debug_mode: bool) -> Self {
        // Constructor accepting debug mode
    }
}
```

#### Conditional Logging
```rust
pub fn log_success(&self, duration: Option<std::time::Duration>, debug_mode: bool) {
    // Only log if:
    // - Debug mode is enabled, OR
    // - Operation involved retries, OR  
    // - Operation took longer than 5 seconds
    let should_log = debug_mode || 
        self.retry_count > 0 || 
        duration.map_or(false, |d| d.as_millis() > 5000);
    
    if should_log {
        // Enhanced logging with operation context
    }
}
```

### 2. Request Context Enhancement

Added request snippet extraction for better operation visibility:

#### Request Snippet Support
```rust
impl ErrorContext {
    /// Add a request snippet for better logging context (truncated to 20 chars)
    pub fn add_request_snippet(mut self, request_content: impl Into<String>) -> Self {
        let content = request_content.into();
        let snippet = if content.len() > 20 {
            format!("{}...", &content[..17])
        } else {
            content
        };
        self.metadata.insert("request_snippet".to_string(), snippet);
        self
    }
}
```

#### Enhanced Operation Descriptions
```rust
fn get_operation_description(&self) -> String {
    match self.operation.as_str() {
        "claude_api_call" => {
            if let Some(snippet) = self.metadata.get("request_snippet") {
                format!("Claude API call: \"{}\"", snippet)
            } else {
                "Claude API call".to_string()
            }
        },
        "tool_execution" => {
            if let Some(tool_name) = self.metadata.get("tool_name") {
                format!("Tool execution: {}", tool_name)
            } else {
                "Tool execution".to_string()
            }
        },
        // ... other operation types
    }
}
```

### 3. Claude Client Integration

Updated Claude client to pass debug configuration:

#### Constructor Enhancement
```rust
impl ClaudeClient {
    pub fn new(config: ClaudeConfig) -> ClaudeResult<Self> {
        Self::new_with_debug(config, false)
    }

    pub fn new_with_debug(config: ClaudeConfig, debug_mode: bool) -> ClaudeResult<Self> {
        // ... existing code ...
        Self {
            // ... other fields ...
            error_handler: ErrorHandler::with_debug(debug_mode),
        }
    }
}
```

#### API Error Context
```rust
// Enhanced error context with request snippets
let request_snippet = if let Some(last_message) = request.messages.last() {
    match &last_message.content.first() {
        Some(ContentBlock::Text { text }) => text.clone(),
        _ => "Non-text content".to_string(),
    }
} else {
    "No messages".to_string()
};

return Err(ClaudeError::ApiError {
    // ... error fields ...
    context: Some(
        ErrorContext::new("claude_api_call")
            .add_metadata("model", &self.config.model)
            .add_request_snippet(request_snippet),
    ),
});
```

### 4. Development Script Integration

Enhanced `scripts/dev.js` to support debug mode flag:

#### Flag Parsing
```javascript
// Support multiple debug flag formats
if (args[i] === '--debug' && i + 1 < args.length) {
    debugMode = args[i + 1] === 'true' || args[i + 1] === '1';
}
if (args[i].startsWith('--debug=')) {
    const debugValue = args[i].split('=')[1];
    debugMode = debugValue === 'true' || debugValue === '1';
}
if (args[i] === '--debug') {
    debugMode = true;
}
```

#### Environment Variable Support
```javascript
// Pass debug mode to Tauri
const env = { 
    ...process.env, 
    CLAUDE_API_KEY: apiKey,
    DEBUG: debugMode ? 'true' : 'false'
};
```

### 5. Runtime Configuration Integration

Connected debug mode to the existing runtime configuration:

#### Main Application Integration
```rust
// Create Claude client with debug mode from runtime config
let debug_mode = state.app_config.runtime.enable_debug_logging;
let client = ClaudeClient::new_with_debug(config, debug_mode)?;
```

## Usage

### Command Line Usage
```bash
# Normal mode (minimal logging)
npm run dev -- --key YOUR_API_KEY

# Debug mode (verbose logging)
npm run dev -- --key YOUR_API_KEY --debug
npm run dev -- --key YOUR_API_KEY --debug=true

# Environment variable approach
DEBUG=true npm run dev -- --key YOUR_API_KEY
```

### Configuration File
Debug mode can also be controlled via the runtime configuration in `config/runtime.rs`:
```rust
pub struct RuntimeConfig {
    // ... other fields ...
    pub enable_debug_logging: bool,
}
```

## Benefits

### 1. Cleaner Development Experience
- **Normal mode**: Only logs retries, errors, and slow operations (>5s)
- **Debug mode**: Logs all operations for detailed troubleshooting

### 2. Better Operation Context
- Request snippets provide context: `Claude API call: "Please read main.rs..."`
- Tool execution context: `Tool execution: read_file`
- Clear operation descriptions instead of generic `retry_operation`

### 3. Flexible Control
- Multiple injection methods: CLI flags, environment variables, config files
- Granular control over what gets logged
- Maintains performance in production mode

### 4. Enhanced Troubleshooting
- Request snippets help identify which user requests triggered issues
- Clear correlation between user actions and system operations
- Detailed error context for debugging complex flows

## Technical Implementation Details

### Message Flow with Debug Context
1. **User Request**: "Please read main.rs and explain the code"
2. **First API Call**: Creates context with request snippet "Please read main.rs..."
3. **Tool Execution**: Read file tool executes
4. **Second API Call**: Tool results sent back to Claude
5. **Logging**: Only logs if debug mode enabled or operation had issues

### Error Context Hierarchy
```
ErrorContext
├── operation: "claude_api_call"
├── metadata
│   ├── model: "claude-sonnet-4-20250514"
│   └── request_snippet: "Please read main.rs..."
├── retry_count: 0
└── timestamp: 2024-12-24T10:30:00Z
```

### Log Output Examples

#### Normal Mode (Minimal)
```
🔑 Claude API key detected
🚀 Starting Tauri development server...
```

#### Debug Mode (Verbose)
```
🔑 Claude API key detected  
🐛 Debug mode enabled - verbose logging active
🚀 Starting Tauri development server...
✅ [SUCCESS] Claude API call: "Please read main.rs..." | retry_count=0 | duration=1250ms
✅ [SUCCESS] Tool execution: read_file | retry_count=0 | duration=45ms
✅ [SUCCESS] Claude API call: "Here are the tool..." | retry_count=0 | duration=890ms
```

## Future Enhancements

1. **Log Levels**: Could extend to support multiple log levels (TRACE, DEBUG, INFO, WARN, ERROR)
2. **Request Filtering**: Could add filters to only log certain types of requests
3. **Performance Metrics**: Could add detailed timing breakdowns for complex operations
4. **Log Persistence**: Could add option to write logs to files for later analysis

## Key Learnings

1. **Two-Step API Pattern Impact**: The architectural fix for tool response interpretation doubled the number of API calls, requiring smarter logging
2. **User Experience Focus**: Excessive logging significantly impacts developer experience and needs careful control
3. **Context is King**: Request snippets and operation descriptions make logs much more useful for debugging
4. **Multiple Injection Methods**: Supporting CLI flags, env vars, and config files provides flexibility for different deployment scenarios

This implementation successfully addresses the excessive logging issue while providing enhanced debugging capabilities when needed.