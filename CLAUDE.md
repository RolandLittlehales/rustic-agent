# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🚨 CRITICAL DEVELOPMENT PROCESS 🚨

**BEFORE making ANY changes to this codebase, you MUST follow this process:**

### 1. UNDERSTAND BEFORE CHANGING
- **Read the code thoroughly** - Don't guess, examine the actual implementation
- **Check file dependencies** - Understand what calls what and how data flows
- **Verify current state** - Use tools to understand existing functionality before modifying
- **Double-check specifications** - This is Tauri v2, not v1. API signatures matter.

### 2. PLAN AND VALIDATE
- **Create a plan** - Use TodoWrite to break down complex tasks into steps
- **Test your understanding** - Read existing code patterns before implementing new ones
- **Verify parameter formats** - Especially for Tauri commands (objects vs primitives)
- **Check for breaking changes** - Ensure existing functionality still works

### 3. IMPLEMENT CAREFULLY
- **Make small, incremental changes** - Don't change multiple things at once
- **Test after each change** - Verify the change works before moving to the next
- **Follow existing patterns** - Match the coding style and architecture already in place
- **Preserve error handling** - Don't remove error logs, only verbose debug logs

### 4. VERIFY AND TEST
- **Build verification**: `cargo build && npm run build` MUST succeed
- **Code quality**: `cargo fmt && cargo clippy` MUST be clean
- **Functional testing**: Test the actual user-facing features
- **Regression testing**: Ensure nothing that worked before is now broken

### 5. LOGGING BEST PRACTICES
- **Keep essential startup info**: Mode detection, API key status, critical errors
- **Remove verbose debug logs**: Detailed trace logging, success confirmations, detailed state dumps
- **Preserve error logging**: All `console.error`, `println!` for errors, `eprintln!`
- **Test debug commands**: Keep debugging functions but make them concise

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

This is a **Tauri v2-based desktop application** that provides a chat interface for Claude AI with secure file system tools.

### Core Components

**Backend (Rust)**:
- `src-tauri/src/main.rs` - Tauri v2 app entry point with command handlers
- `src-tauri/src/claude/` - Claude API integration module
  - `client.rs` - HTTP client for Claude API with tool execution
  - `tools.rs` - Async trait-based tool system with whitelist validation
  - `types.rs` - Type definitions for Claude API
  - `whitelist.rs` - Security whitelist system for file access control
  - `mod.rs` - Configuration and conversation management
- `src-tauri/src/file_watcher.rs` - Real-time file system monitoring
- `src-tauri/src/security.rs` - Security utilities and validation

**Frontend (Vanilla JS)**:
- `ui/index.html` - Main application interface
- `ui/js/app.js` - Chat UI, file explorer, and Tauri v2 command bindings
- `ui/css/styles.css` - Modern styling

**Key Architecture Patterns**:
- **Thread-safe state**: `Arc<Mutex<>>` for conversation and config, `Arc<RwLock<>>` for whitelist
- **Async tool system**: `#[async_trait]` based tools with whitelist validation
- **Tauri v2 communication**: Named parameter objects for all commands
- **Security-first design**: Whitelist-based file access with path validation
- **Real-time updates**: File watcher with debounced UI updates

### Tool System

Tools implement the async `AgentTool` trait in `src-tauri/src/claude/tools.rs`:
- `ReadFileTool` - Read file contents with whitelist validation
- `WriteFileTool` - Write file contents with whitelist validation  
- `ListDirectoryTool` - List directory contents with whitelist validation

**Critical**: All tools use `#[async_trait]` and require whitelist validation via `set_whitelist()`.

### State Management

Current `AppState` structure:
```rust
struct AppState {
    conversation: Arc<Mutex<Conversation>>,
    config: Arc<Mutex<ClaudeConfig>>,
    whitelist: Arc<RwLock<WhitelistConfig>>,
    file_watcher: Arc<FileWatcherService>,
}
```

### Security Whitelist System

**WhitelistConfig** (`src-tauri/src/claude/whitelist.rs`):
- Runtime-configurable directory access control
- Path canonicalization and traversal attack prevention  
- Configurable security settings (max depth, file size limits)
- Persistent storage of whitelist configuration
- Auto-adds current directory on startup

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

- **Tauri v2**: This project uses Tauri v2, NOT v1. API signatures and patterns differ significantly.
- **Command Parameters**: Tauri v2 requires named parameter objects: `{ param: value }` not just `value`
- **Async Everywhere**: All tools and commands use async/await patterns
- **API Key Required**: Application will not function without valid Claude API key
- **Tauri Commands**: All backend functions exposed via `#[tauri::command]`
- **Development Script**: `scripts/dev.js` handles environment variable setup
- **Build Target**: Cross-platform desktop (Windows, macOS, Linux)
- **WebView**: Uses system WebView with CSP security policies
- **Error Handling**: Comprehensive error handling with user-friendly messages

### Critical Tauri v2 Patterns

**JavaScript Command Calls**:
```javascript
// Correct (Tauri v2):
await window.__TAURI__.core.invoke('list_directory', { path: '.' });

// Wrong (Tauri v1 style):
await window.__TAURI__.core.invoke('list_directory', '.');
```

**Rust Command Signatures**:
```rust
#[tauri::command]
async fn list_directory(path: String, state: tauri::State<'_, AppState>) -> Result<Vec<FileItem>, String>
```

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

### Critical Mistakes to Avoid

**1. Parameter Format Errors (Tauri v2)**:
- ❌ Using string parameters directly: `invoke('cmd', 'value')`
- ✅ Using object parameters: `invoke('cmd', { param: 'value' })`
- **Impact**: Complete function failure, cryptic error messages

**2. Missing Function Definitions**:
- ❌ Calling functions that aren't defined (e.g., `setupTextareaAutoResize`)
- ✅ Always verify function exists before calling
- **Impact**: Runtime TypeError, app crash

**3. Ignoring Compilation Errors**:
- ❌ Making changes without running `cargo build`
- ✅ Build and test after every significant change
- **Impact**: Broken functionality, wasted time

**4. Excessive Debug Logging**:
- ❌ Leaving verbose console.log statements in production
- ✅ Clean up debug logs, keep essential startup info and errors
- **Impact**: Poor user experience, console spam

### Proven Development Process

**Phase 1: Understand**
```bash
# Read the actual code, don't assume
cargo check                    # Verify Rust compiles
rg "function_name" .          # Find all usages
rg "console\.log" ui/js/      # Check current logging
```

**Phase 2: Plan**
```bash
# Use TodoWrite to break down complex tasks
# Identify all files that need changes
# Verify parameter formats for Tauri commands
```

**Phase 3: Implement**
```bash
# Make one change at a time
cargo build && npm run build  # Test after each change
cargo fmt && cargo clippy     # Keep code clean
```

**Phase 4: Verify**
```bash
# Test the actual user workflow
# Check both desktop and browser modes
# Verify no regressions in existing functionality
```

### Specific Technical Learnings

**1. Tauri v2 Command Patterns**:
- All commands require object parameters in JavaScript
- Rust functions receive named parameters
- Error messages are often unclear when format is wrong

**2. Frontend/Backend Integration**:
- Backend tools need explicit Tauri command exposure
- Whitelist validation happens at tool level, not command level
- File watcher requires event system setup

**3. State Management**:
- AppState uses different lock types for different data
- Whitelist uses RwLock for concurrent read access
- Conversation uses Mutex for exclusive access

**4. Security Implementation**:
- Path validation prevents directory traversal
- Whitelist system provides runtime-configurable access control
- API keys never exposed in frontend code

### Quality Assurance Checklist

Before considering any change complete:

- [ ] `cargo build` succeeds without warnings
- [ ] `cargo clippy` passes without suggestions  
- [ ] `cargo fmt` has been run
- [ ] `npm run build` succeeds
- [ ] File explorer loads and shows current directory
- [ ] Tauri commands use correct parameter format
- [ ] No missing function errors in browser console
- [ ] Essential logging preserved, verbose logging removed
- [ ] Whitelist functionality works (Ctrl+T test)
- [ ] File watching works without console spam

## Available Tauri Commands

**Core Application Commands**:
```javascript
// API Key Management
await window.__TAURI__.core.invoke('get_api_key_status');
await window.__TAURI__.core.invoke('initialize_with_env_key');
await window.__TAURI__.core.invoke('set_claude_api_key', { api_key: 'sk-...' });

// Claude AI Integration  
await window.__TAURI__.core.invoke('send_message_to_claude', { message: 'Hello' });
await window.__TAURI__.core.invoke('get_conversation_history');
await window.__TAURI__.core.invoke('clear_conversation');

// File System Operations
await window.__TAURI__.core.invoke('list_directory', { path: '.' });
```

**Whitelist Management Commands**:
```javascript
// Configuration
await window.__TAURI__.core.invoke('whitelist_get_config');
await window.__TAURI__.core.invoke('whitelist_set_enabled', { enabled: true });

// Directory Management
await window.__TAURI__.core.invoke('whitelist_list_directories');
await window.__TAURI__.core.invoke('whitelist_add_directory', { path: '/path/to/dir' });
await window.__TAURI__.core.invoke('whitelist_remove_directory', { path: '/path/to/dir' });
```

**File Watching Commands**:
```javascript
// File Monitoring
await window.__TAURI__.core.invoke('start_file_watching');
await window.__TAURI__.core.invoke('stop_file_watching');
```

**Debug Shortcuts (Browser Console)**:
- `Ctrl+R` - Refresh file tree
- `Ctrl+T` - Test whitelist functionality  
- `Ctrl+D` - Debug app state

## Project Status Summary

- **Framework**: Tauri v2 (NOT v1)
- **Security**: Whitelist-based file access with runtime configuration
- **File System**: Real-time monitoring with debounced updates
- **API Integration**: Claude 3.5 Sonnet with tool execution
- **State**: Thread-safe with proper async patterns
- **Logging**: Essential startup info + errors only, verbose logging cleaned
- **Testing**: Comprehensive validation scripts available