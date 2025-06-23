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
- `src-tauri/src/config/` - **NEW: Unified Configuration System**
  - `mod.rs` - Master configuration with environment loading
  - `constants.rs` - Compile-time constants (API endpoints, models, limits)
  - `runtime.rs` - Runtime-configurable settings (timeouts, feature flags)
  - `validation.rs` - Validation limits with type-safe helpers
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
- `ui/js/config.js` - **NEW: Frontend configuration constants**
- `ui/css/styles.css` - Modern styling

**Key Architecture Patterns**:
- **Thread-safe state**: `Arc<Mutex<>>` for conversation and config, `Arc<RwLock<>>` for whitelist
- **Async tool system**: `#[async_trait]` based tools with whitelist validation
- **Tauri v2 communication**: Named parameter objects for all commands
- **Security-first design**: Whitelist-based file access with path validation
- **Real-time updates**: File watcher with debounced UI updates
- **Unified Configuration**: Centralized constants, runtime config, and validation limits

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
    app_config: Arc<AppConfig>,                    // NEW: Unified configuration
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

- Uses `reqwest` for HTTP client with configurable timeout (default 120s)
- Converts internal conversation format to Claude API messages
- Handles tool execution within Claude responses
- Model: `claude-sonnet-4-20250514` (default, fully configurable)
- **Configuration-driven**: All timeouts, limits, and models managed via unified config

## Unified Configuration System

**IMPORTANT**: A comprehensive configuration system has been implemented to eliminate hardcoded values and improve maintainability.

### Configuration Architecture

The system uses **3 strategic patterns**:

1. **Compile-time Constants** (`src-tauri/src/config/constants.rs`):
   - Values that never change at runtime (API endpoints, security patterns, model lists)
   - Examples: `CLAUDE_API_BASE_URL`, `SUSPICIOUS_PATTERNS`, `PROTECTED_FILES`

2. **Runtime Configuration** (`src-tauri/src/config/runtime.rs`):
   - Deployment-configurable values via environment variables and config files
   - Examples: timeouts, feature flags, model selection, API keys
   - Loading hierarchy: Environment variables → Config files → Defaults

3. **Validation Limits** (`src-tauri/src/config/validation.rs`):
   - Security and resource limits with built-in validation
   - Examples: message size limits, file size limits, path length limits
   - Type-safe helpers for validation and formatting

### Key Configuration Features

**Environment Integration**:
```rust
let app_config = AppConfig::load()?; // Loads from multiple sources automatically
```

**Type-Safe Validation**:
```rust
// Before: scattered hardcoded limits
if message.len() > 50000 { return Err(...); }

// After: centralized configuration
state.app_config.validation.validate_message_length(message.len())?;
```

**Frontend/Backend Consistency**:
```javascript
// Frontend mirrors backend configuration
import { VALIDATION, CONFIG_HELPERS } from './config.js';
const warningLevel = CONFIG_HELPERS.getMessageWarningLevel(length);
```

### Configuration Best Practices

- **Single Source of Truth**: All limits and constants defined centrally
- **Environment Flexibility**: Easy deployment configuration via environment variables
- **Type Safety**: Compile-time validation prevents configuration errors
- **Maintainability**: Changes to limits require updates in only one location

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
9. **Unified Configuration System**: Eliminated hardcoded values with centralized config management
10. **Intelligent Security Design**: Layered validation that doesn't break legitimate functionality

## Development Process Learnings

### Core Development Principles for Quality Code

**1. Configuration-First Development**:
- ❌ **Anti-pattern**: Hardcoding limits, timeouts, and magic numbers throughout code
  ```rust
  if message.len() > 50000 { return Err("Too long".into()); }
  if path_str.contains("..") { return Err("Invalid".into()); }
  ```
- ✅ **Best practice**: Define configuration once, use everywhere
  ```rust
  app_config.validation.validate_message_length(message.len())?;
  // Let whitelist system handle path validation intelligently
  ```
- **Principle**: Single source of truth prevents inconsistencies and enables deployment flexibility

**2. Layered Security Validation**:
- ❌ **Anti-pattern**: Overly restrictive early validation that breaks legitimate use cases
- ✅ **Best practice**: Let specialized security systems handle complex validation
  ```rust
  // Don't block all paths with ".." - let whitelist canonicalize and validate
  validate_path(path_str, &whitelist_guard, FileOperation::Read)?
  ```
- **Principle**: Security should be intelligent, not blunt - enable legitimate use while preventing abuse

**3. Type-Safe Configuration**:
- ❌ **Anti-pattern**: Runtime string comparisons and manual validation
- ✅ **Best practice**: Compile-time constants with type-safe runtime validation
  ```rust
  // Constants defined once, validation built-in
  pub const SUSPICIOUS_PATTERNS: &[&str] = &["<script", "javascript:"];
  state.app_config.validation.validate_message_length(len)?; // Type-safe with helpful errors
  ```
- **Principle**: Let the type system prevent configuration errors

**4. Consistent Frontend/Backend Patterns**:
- ❌ **Anti-pattern**: Different configuration approaches in frontend vs backend
- ✅ **Best practice**: Mirror configuration patterns across layers
  ```javascript
  // Frontend mirrors backend structure
  import { VALIDATION, CONFIG_HELPERS } from './config.js';
  const warningLevel = CONFIG_HELPERS.getMessageWarningLevel(length);
  ```
- **Principle**: Consistency reduces cognitive load and prevents synchronization bugs

**5. Tauri v2 Parameter Discipline**:
- ❌ **Anti-pattern**: Mixing parameter styles, assuming Tauri v1 patterns work
- ✅ **Best practice**: Always use object parameters, verify signatures
  ```javascript
  // Always use object parameters in Tauri v2
  await window.__TAURI__.core.invoke('send_message', { message: text });
  ```
- **Principle**: Framework conventions exist for a reason - follow them consistently

**6. Build-Driven Development**:
- ❌ **Anti-pattern**: Making multiple changes before testing compilation
- ✅ **Best practice**: Build early, build often, fix issues immediately
- **Principle**: Fast feedback loops prevent compound errors and save debugging time

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
- **Path traversal**: Avoid overly restrictive validation - let whitelist system canonicalize and validate paths properly

**5. Configuration Management**:
- Use unified configuration system to eliminate hardcoded values
- Frontend and backend configurations must stay synchronized
- Environment variables take precedence over config files over defaults
- Type-safe validation prevents configuration errors
- **Pattern**: Three-tier config architecture (constants, runtime, validation)

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
- [ ] Configuration system used instead of hardcoded values
- [ ] Frontend and backend configuration constants synchronized
- [ ] Path traversal handled by whitelist system, not rejected outright

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

## GitHub Issues & Project Management

### Issue Creation Process

When creating GitHub issues for this project, follow this standardized format:

**Title Format:** `[Order] Category: Brief Description`
- Example: `[1.1] Foundation: Implement ContentBlock enum system`

**Required Sections:**
1. **Overview** - What needs to be implemented and why
2. **Technical Requirements** - Specific implementation details
3. **Acceptance Criteria** - Definition of done
4. **Quality Control** - Testing requirements (unit tests, integration tests, manual testing)
5. **Documentation Requirements** - What formal documentation needs to be created/updated
6. **Dependencies** - Other issues that must be completed first
7. **Estimated Scope** - Target 500-1000 lines of code per issue

**Labels to Use:**
- `phase-1`, `phase-2`, `phase-3` - Development phases
- `foundation`, `core-features`, `advanced-tools` - Feature categories
- `breaking-change` - Issues that modify existing APIs
- `documentation` - Issues requiring formal docs updates

### Documentation Standards

All new features must include formal documentation in the `docs/` directory:

**Required Documentation:**
- **Architecture docs** - How the feature fits into the overall system
- **API documentation** - For any new types, traits, or functions
- **Integration guides** - How to use the feature
- **Reference links** - Internal cross-references and external resources

**Documentation Format:**
- Use Markdown with clear headings
- Include code examples with proper syntax highlighting
- Add cross-references to related documentation
- Link to relevant external resources (Anthropic docs, Tauri docs, etc.)

**When to Update Documentation:**
- Creating new modules or major functionality
- Modifying existing APIs or behavior
- Adding configuration options
- Implementing new tools or integrations

**Documentation Review:**
- All documentation must be reviewed as part of the issue acceptance criteria
- Documentation should be updated in the same PR as the implementation
- Cross-references should be validated and working

**Documentation Examples from ContentBlock Implementation:**
1. **Architecture Documentation** (`docs/architecture/content-block-system.md`):
   - System overview with design principles
   - Component descriptions and data flow
   - Security considerations
   - Future extensibility plans

2. **API Reference** (`docs/api/content-block-types.md`):
   - Complete type definitions with all fields
   - Method signatures and traits
   - Usage examples and serialization
   - Validation rules

3. **Integration Guide** (`docs/tools/content-block-integration.md`):
   - Step-by-step implementation guide
   - Best practices and common patterns
   - Testing strategies
   - Performance considerations

4. **Index Updates** (`docs/architecture/README.md`):
   - Central hub linking all related docs
   - Architecture decision records
   - System diagrams and overviews

### Quality Control Standards

**Automated Testing Requirements:**
- Unit tests for all new functions and methods
- Integration tests for tool execution and API interactions
- Property-based tests for complex data transformations
- Performance tests for critical paths

**Manual Testing Requirements:**
- End-to-end user workflows
- Error handling and edge cases
- Cross-platform compatibility (Windows, macOS, Linux)
- Security validation (whitelist enforcement, API key handling)

**Code Quality Checks:**
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting and best practices
- `cargo test` - All tests must pass
- `cargo build && npm run build` - Compilation must succeed
- No new compiler warnings

## Ticket Implementation Workflow

When working on a new feature or issue, follow this systematic approach:

### 1. Ticket Selection & Assignment

**Initial Steps:**
1. **Check implementation-sequence.md** - Understand the overall ticket ordering and progression
2. **Review GitHub issues** - Find the first ticket without dependencies
3. **Notify user** - Communicate which ticket you plan to work on and wait for confirmation
4. **Assign yourself** - Once confirmed, assign the ticket to yourself on GitHub

### 2. Pre-Implementation Analysis

**Understanding the Task:**
- **Read thoroughly** - Read the ticket multiple times to fully understand requirements
- **Examine from multiple angles** - Consider edge cases, security implications, and integration points
- **Verify understanding** - Ensure you comprehend both the what and the why of the ticket
- **Review related code** - Understand existing patterns and conventions that apply

### 3. Documentation Planning

**Before Implementation, Plan Documentation:**
- **Architecture Documentation** - Determine if this feature needs architecture docs
- **API Documentation** - List all new types, traits, and functions that need docs
- **Integration Guides** - Identify if users/developers need guidance on using the feature
- **Cross-References** - Note which existing docs need updates

**Documentation Requirements by Feature Type:**
- **New Systems** (like ContentBlock): Full architecture doc + API reference + integration guide
- **New Tools**: Tool integration guide + API reference for the tool trait
- **Configuration Changes**: Update configuration guides + migration notes
- **API Changes**: Update API reference + migration guide if breaking
- **Bug Fixes**: Usually just update relevant docs if behavior changes

### 4. Implementation Process

**Development Principles:**
- **Robustness** - Build resilient code that handles errors gracefully
- **Correctness** - Ensure the implementation precisely matches requirements
- **Security** - Follow security best practices, especially for file operations
- **Maintainability** - Write clear, well-structured code that others can understand

**Development Practices:**
- **Create documentation structure first** - Set up doc files before coding
- **Incremental testing** - Run `cargo build && npm run build` periodically
- **Quality checks** - Run `cargo fmt && cargo clippy` regularly
- **Functional verification** - Test the actual functionality as you build
- **Documentation** - Update docs as you implement, not as an afterthought
- **Cross-references** - Add links between related docs as you write

### 5. Documentation Creation

**Documentation Process:**
1. **Create planned documentation files** - Based on your planning phase
2. **Write comprehensive content** - Include:
   - Overview and purpose
   - Technical details
   - Code examples
   - Integration instructions
   - Security considerations
   - Performance notes
3. **Add cross-references** - Link to:
   - Related internal documentation
   - External resources (Anthropic docs, Tauri docs)
   - GitHub issues and PRs
4. **Update existing docs** - Ensure all references to changed functionality are current

**Documentation Quality Standards:**
- **Clear structure** - Use consistent headings and organization
- **Code examples** - Provide working examples with syntax highlighting
- **Completeness** - Cover all aspects of the feature
- **Accuracy** - Ensure docs match actual implementation
- **Accessibility** - Write for both beginners and advanced users

### 6. Code Review Process

**Self-Review via Sub-Agent:**
1. **Launch sub-agent** - Start a critical code review sub-agent
2. **Critical analysis** - Have the sub-agent examine code for:
   - Security vulnerabilities
   - Performance issues
   - Code clarity and maintainability
   - Adherence to project patterns
   - Test coverage adequacy
   - **Documentation completeness and accuracy**
3. **Address feedback** - Fix all issues identified by the sub-agent
4. **Re-review if needed** - For significant changes, run another review cycle

### 7. Pull Request Creation

**PR Requirements:**
- **Link to issue** - Include "Fixes #[issue-number]" in the PR body
- **Clear summary** - Provide a concise summary of what the issue was
- **Implementation TLDR** - Brief explanation of how you addressed the issue
- **Documentation summary** - List all documentation created/updated
- **Detailed body** - Include:
  - Key implementation decisions
  - Any trade-offs made
  - Breaking changes (if any)
  - Performance considerations
  - Links to new documentation

**Testing Instructions:**
- **Determine testing needs** - Not all PRs require manual testing:
  - ✅ Manual testing needed: New features, UI changes, integrations
  - ❌ Manual testing optional: Pure refactors, type refinements, documentation
- **Clear test steps** - If manual testing is needed, provide step-by-step instructions
- **Expected outcomes** - Describe what the tester should see/experience
- **Edge cases** - Note any specific scenarios that should be tested

### 8. Post-PR Documentation Review

**After PR Approval:**
- **Verify all docs are linked** - Ensure documentation is discoverable
- **Check cross-references** - Verify all links work correctly
- **Update indexes** - Add new docs to relevant index files
- **Notify of docs** - Mention new documentation in release notes

## Formal Documentation Structure

The `docs/` directory contains comprehensive technical documentation:

- `docs/architecture/` - System design and architectural decisions
- `docs/api/` - API reference documentation
- `docs/tools/` - Tool system documentation
- `docs/configuration/` - Configuration guides and references
- `docs/security/` - Security implementation details
- `docs/development/` - Development guides and workflows
- `docs/integration/` - Third-party integration guides

**Cross-Reference Guidelines:**
- Always link to relevant sections in other docs
- Include links to external resources (Anthropic API docs, Tauri guides, etc.)
- Keep an updated index of all documentation
- Use consistent linking patterns for maintainability

## Project Status Summary

- **Framework**: Tauri v2 (NOT v1)
- **Configuration**: Unified three-tier system (constants, runtime, validation)
- **Security**: Intelligent layered validation with whitelist-based file access
- **File System**: Real-time monitoring with debounced updates  
- **API Integration**: Claude 4 Sonnet with configurable tool execution
- **State**: Thread-safe with proper async patterns
- **Architecture**: Configuration-first development with type safety
- **Validation**: Centralized limits with helpful error messages
- **Testing**: Comprehensive validation scripts available
- **Documentation**: Formal docs in `docs/` directory with cross-references

## Core Architecture Principles

1. **Configuration-First**: No hardcoded values, centralized configuration management
2. **Type Safety**: Let the compiler prevent configuration and validation errors
3. **Layered Security**: Intelligent validation that enables legitimate use cases
4. **Consistency**: Mirror patterns between frontend and backend
5. **Framework Discipline**: Follow Tauri v2 conventions strictly
6. **Fast Feedback**: Build early and often to catch issues immediately

## Rust-Specific Development Standards

### 1. Embrace Rust's Strengths

**Ownership and Borrowing**:
```rust
// ✅ Good: Use borrowing to avoid unnecessary cloning
pub fn validate_message(message: &str, config: &ValidationLimits) -> Result<()> {
    config.validate_message_length(message.len())
}

// ❌ Avoid: Unnecessary cloning
pub fn validate_message(message: String, config: ValidationLimits) -> Result<()> { ... }
```

**Idiomatic Constructs**:
```rust
// ✅ Good: Use pattern matching and enums
match app_config.validation.message_warning_level(length) {
    MessageWarningLevel::Ok => "text-gray-500",
    MessageWarningLevel::Warning => "text-warning-500", 
    MessageWarningLevel::Danger => "text-error-500",
}

// ✅ Good: Use traits for shared behavior
#[async_trait]
pub trait AgentTool: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    async fn execute(&self, input: Value) -> Result<String>;
}
```

**Error Handling**:
```rust
// ✅ Good: Use ? operator for error propagation
pub fn load_config() -> Result<AppConfig> {
    let mut config = AppConfig::default();
    config.runtime.load_from_env()?;
    config.validate()?;
    Ok(config)
}

// ❌ Avoid: .unwrap() without proper checks
let config = load_config().unwrap(); // Can panic!
```

### 2. Write Small, Focused Functions and Modules

**Single Responsibility**:
```rust
// ✅ Good: Each function has one clear purpose
impl ValidationLimits {
    pub fn validate_message_length(&self, length: usize) -> Result<()> { ... }
    pub fn validate_file_size(&self, size: u64) -> Result<()> { ... }
    pub fn validate_path_length(&self, path: &str) -> Result<()> { ... }
}
```

**Modularity**:
```
src/config/
├── mod.rs          # Master configuration coordination
├── constants.rs    # Compile-time constants (< 200 lines)
├── runtime.rs      # Runtime configuration (< 300 lines)
└── validation.rs   # Validation limits (< 300 lines)
```

### 3. Maintain Clarity and Readability

**Descriptive Names**:
```rust
// ✅ Good: Clear, descriptive names
pub const CLAUDE_API_MESSAGES_ENDPOINT: &str = "/messages";
pub fn validate_message_length(&self, length: usize) -> Result<()>

// ❌ Avoid: Abbreviated or unclear names
pub const API_EP: &str = "/messages";
pub fn val_msg_len(&self, len: usize) -> Result<()>
```

**Named Constants**:
```rust
// ✅ Good: Named constants instead of magic numbers
pub mod defaults {
    pub const MESSAGE_MAX_CHARS: usize = 50000; // 50KB for coding helper
    pub const FILE_MAX_SIZE_BYTES: u64 = 10 * 1024 * 1024; // 10MB
}

// ❌ Avoid: Magic numbers scattered throughout code
if message.len() > 50000 { ... }
```

### 4. Leverage Rust's Tooling

**Development Workflow**:
```bash
# Essential commands for clean code
cargo check          # Fast compilation check
cargo clippy          # Linting and best practices
cargo fmt            # Consistent formatting
cargo test           # Run all tests

# Quality gates before committing
cargo clippy -- -D warnings    # Treat warnings as errors
cargo test                      # All tests must pass
```

**Clippy Integration**:
```rust
// Follow Clippy suggestions for idiomatic Rust
#[allow(dead_code)]  // Only when code is intentionally unused
pub fn future_feature() { ... }

// Prefer Clippy-suggested patterns
if let Some(value) = optional_value {  // Instead of match
    // Handle value
}
```

### 5. Testing and Code Quality

**Test Organization**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test] 
    fn test_message_limits() {
        let limits = ValidationLimits::default();
        assert!(limits.validate_message_length(1000).is_ok());
        assert!(limits.validate_message_length(100000).is_err());
    }
}
```

**Documentation Standards**:
```rust
/// Validate message length against configured limits
/// 
/// Returns `Ok(())` if the message is within limits, or an error
/// with a descriptive message if validation fails.
/// 
/// # Arguments
/// * `length` - The message length in characters
/// 
/// # Examples
/// ```
/// let limits = ValidationLimits::default();
/// assert!(limits.validate_message_length(1000).is_ok());
/// ```
pub fn validate_message_length(&self, length: usize) -> Result<()> { ... }
```

### 6. Dependency Management

**Cargo.toml Best Practices**:
```toml
[dependencies]
# Group by purpose with comments
# Tauri framework
tauri = { version = "2.0", features = ["tray-icon", "devtools"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# HTTP client - specify features to avoid unused dependencies
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
```

### 7. Performance and Memory Safety

**Efficient String Handling**:
```rust
// ✅ Good: Use string slices when possible
pub fn validate_path(path: &str, whitelist: &WhitelistConfig) -> Result<PathBuf>

// ✅ Good: Use Cow for conditional cloning
use std::borrow::Cow;
pub fn normalize_path(path: &str) -> Cow<str> { ... }
```

**Resource Management**:
```rust
// ✅ Good: Use Arc for shared ownership, RwLock for concurrent access
pub struct AppState {
    config: Arc<Mutex<ClaudeConfig>>,
    app_config: Arc<AppConfig>,           // Immutable shared data
    whitelist: Arc<RwLock<WhitelistConfig>>,  // Concurrent read access
}
```

These patterns ensure we write idiomatic, maintainable Rust code that leverages the language's strengths while following established best practices.