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
- **API key injection**: Development script injects API key into frontend

### Tool System

Tools implement the `AgentTool` trait in `src-tauri/src/claude/tools.rs`:
- `ReadFileTool` - Read file contents
- `WriteFileTool` - Write file contents  
- `ListDirectoryTool` - List directory contents

To add new tools, implement `AgentTool` and register in `ClaudeClient::new()`.

### State Management

- **AppState**: Manages Claude client, conversation, and config
- **Conversation**: Thread-safe message history with timestamps
- **ClaudeConfig**: API key, model settings, temperature

### API Integration

- Uses `reqwest` for HTTP client with 120s timeout
- Converts internal conversation format to Claude API messages
- Handles tool execution within Claude responses
- Model: `claude-3-5-sonnet-20241022` (configurable)

## Key Development Notes

- **API Key Required**: Application will not function without valid Claude API key
- **Tauri Commands**: All backend functions exposed via `#[tauri::command]`
- **Development Script**: `scripts/dev.js` handles API key injection and cleanup
- **Build Target**: Cross-platform desktop (Windows, macOS, Linux)
- **WebView**: Uses system WebView with CSP security policies