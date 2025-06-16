# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
# Development mode (requires Claude API key)
npm run dev -- --key YOUR_API_KEY
# or with env var
CLAUDE_API_KEY=your_key npm run dev

# Build production bundle
npm run build

# Rust-specific commands
cd src-tauri
cargo build          # Debug build
cargo build --release # Release build
cargo run            # Run without Tauri
cargo test           # Run tests
cargo fmt            # Format code
cargo clippy         # Lint code
cargo clean          # Clean build cache
```

## Architecture Overview

This is a **Tauri-based desktop application** that provides a chat interface for Claude AI with file system tools.

### Core Components

**Backend (Rust)**:
- `src-tauri/src/main.rs` - Tauri app entry point with command handlers
- `src-tauri/src/claude/` - Claude API integration module
  - `client.rs` - HTTP client for Claude API with tool execution
  - `tools.rs` - Extensible tool registry system (file read/write/list)
  - `types.rs` - Type definitions for Claude API
  - `mod.rs` - Configuration and conversation management

**Frontend (Vanilla JS)**:
- `ui/index.html` - Main application interface
- `ui/js/app.js` - Chat UI, file explorer, and Tauri command bindings
- `ui/css/styles.css` - Modern styling

**Key Architecture Patterns**:
- **Thread-safe state**: `Arc<Mutex<>>` for conversation and config
- **Tool system**: Trait-based extensible tools registered in `ToolRegistry`
- **Async communication**: Tauri commands bridge JS frontend to Rust backend
- **Secure API key handling**: Environment variables only, no code injection

### Tool System

Tools implement the `AgentTool` trait in `src-tauri/src/claude/tools.rs`:
- `ReadFileTool` - Read file contents
- `WriteFileTool` - Write file contents  
- `ListDirectoryTool` - List directory contents

To add new tools, implement `AgentTool` and register in `ClaudeClient::new()`.

### State Management

- **AppState**: Manages conversation and config (simplified from previous version)
- **Conversation**: Thread-safe message history with timestamps
- **ClaudeConfig**: API key, model settings, temperature

### API Integration

- Uses `reqwest` for HTTP client with 120s timeout
- Converts internal conversation format to Claude API messages
- Handles tool execution within Claude responses
- Model: `claude-3-5-sonnet-20241022` (configurable)

## Security & API Key Handling

**IMPORTANT**: As of the latest update, API key handling has been significantly improved:

- **Environment Variables Only**: API keys are passed through `CLAUDE_API_KEY` environment variable
- **No Frontend Exposure**: API keys are never injected into HTML/JavaScript code
- **Runtime Initialization**: Backend reads API key from environment on startup
- **No File Modification**: Development script no longer modifies source files
- **Secure Logging**: API keys are masked in all log output

### Development Script Security

The `scripts/dev.js` file now:
- Passes API key as environment variable to Tauri process
- Never modifies HTML or JavaScript files
- Provides clear error messages when API key is missing
- Supports both `--key` flag and `CLAUDE_API_KEY` env var

## Quality Assurance

**Code Quality Checks** (run these before committing):
```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Build and verify compilation
cargo build
npm run build

# Run tests
cargo test
```

**Best Practices**:
- Always run `cargo fmt` and `cargo clippy` before committing
- Use `#[allow(dead_code)]` for future-use code rather than deleting
- Follow Rust naming conventions (snake_case for variables/functions)
- Minimize compiler warnings and fix all clippy suggestions
- Test both environment variable and command-line API key approaches

## Key Development Notes

- **API Key Required**: Application will not function without valid Claude API key
- **Tauri Commands**: All backend functions exposed via `#[tauri::command]`
- **Development Script**: `scripts/dev.js` handles environment variable setup
- **Build Target**: Cross-platform desktop (Windows, macOS, Linux)
- **WebView**: Uses system WebView with CSP security policies
- **Error Handling**: Comprehensive error handling with user-friendly messages

## Recent Improvements (Quality of Life)

1. **Secure API Key Handling**: Moved from code injection to environment variables
2. **Compiler Warning Fixes**: Eliminated all unused code and naming warnings
3. **Code Quality**: Applied cargo fmt, clippy, and best practices
4. **Simplified Architecture**: Removed unused AppState fields
5. **Better Security**: No API keys ever exposed in frontend code
6. **Development Experience**: Clear error messages and improved dev workflow
7. **File Explorer Functionality**: Fixed missing file explorer by adding `list_directory` Tauri command
8. **Frontend Security**: Removed dead code and potential API key exposure paths

## Development Process Learnings

### Critical Review Process
When making significant changes to a codebase:

1. **Security-First Approach**: Always audit API key handling and sensitive data exposure
2. **Comprehensive Testing**: Test all user-facing functionality after changes
3. **Multi-Agent Reviews**: Use specialized agents for different aspects (security, performance, frontend)
4. **Incremental Fixes**: Address issues immediately when found during review
5. **Functionality Preservation**: Ensure no features are lost during refactoring

### Key Technical Findings

1. **Tauri Command Exposure**: Backend tools need explicit Tauri commands to be accessible from frontend
2. **Mock Data Trap**: Always replace mock data with real implementations before production
3. **Dead Code Security Risk**: Unreachable code can still pose security risks if it contains sensitive patterns
4. **Environment Variable Security**: Proper environment variable handling prevents accidental commits
5. **Compiler Warnings Matter**: Address all warnings for production-ready code

### Best Practices Established

1. **API Key Security**: Never expose API keys in frontend, use environment variables only
2. **Code Reviews**: Use automated tools (clippy, fmt) AND manual review processes
3. **Documentation Updates**: Keep documentation current with code changes
4. **Testing Strategy**: Test both happy path and error scenarios
5. **Build Verification**: Always verify builds succeed after significant changes