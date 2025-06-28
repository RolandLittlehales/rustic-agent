# CLAUDE.md - Development Hub

This is the central hub for Claude Code development guidance. For detailed documentation, see `.claude/docs/`.

## 🚀 Quick Start

- **Environment Setup**: Use `/dev-checklist` to verify your development environment
- **Simple Work**: Use `/pick-next-ticket` for bug fixes and small improvements
- **Feature Work**: Use `/pick-next-feature` for major features and comprehensive development
- **All Commands**: See [Command Reference](#command-reference) below

## Command Reference

### 🎯 Workflow Commands

| Command | Purpose | Best For |
|---------|---------|----------|
| `/pick-next-ticket` | Auto-select next simple ticket → `/work-on-ticket` | Bug fixes, small improvements, maintenance |
| `/pick-next-feature` | Auto-select next major feature → `/start-feature` | New systems, major features, architectural changes |
| `/work-on-ticket $ARGS` | Quick focused implementation | Efficient development with basic quality checks |
| `/start-feature $ARGS` | Comprehensive feature development | Professional-grade development with full documentation |

### ✅ Quality Assurance Commands

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/qa-check` | Pre-commit validation (build, test, lint) | Before every commit and PR |
| `/review-pr $ARGS` | Code review with standards enforcement | Before merging any PR |
| `/security-check` | Security validation | For any security-related changes |
| `/config-check` | Configuration consistency validation | When modifying configuration system |

### 🛠️ Development Support Commands

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/dev-checklist` | Environment validation | Start of development session |
| `/docs-check $ARGS` | Documentation standards verification | When creating/updating documentation |
| `/create-pr $ARGS` | Professional PR creation | Ready to submit work for review |
| `/test-review $ARGS` | Test optimization analysis | When refactoring or optimizing tests |

## 🏗️ Architecture Overview

This is a **Tauri v2-based desktop application** providing a chat interface for Claude AI with secure file system tools.

### Core Components

**Backend (Rust)**:
- `src-tauri/src/main.rs` - Tauri v2 app entry point with command handlers
- `src-tauri/src/config/` - Unified Configuration System (constants, runtime, validation)
- `src-tauri/src/claude/` - Claude API integration with async tool system
- `src-tauri/src/file_watcher.rs` - Real-time file system monitoring
- `src-tauri/src/security.rs` - Security utilities and validation

**Frontend (Vanilla JS)**:
- `ui/index.html` - Main application interface
- `ui/js/app.js` - Chat UI, file explorer, and Tauri v2 command bindings
- `ui/js/config.js` - Frontend configuration constants
- `ui/css/styles.css` - Modern styling

**Key Patterns**:
- **Thread-safe state**: `Arc<Mutex<>>` for conversation/config, `Arc<RwLock<>>` for whitelist
- **Async tool system**: `#[async_trait]` based tools with whitelist validation
- **Tauri v2 communication**: Named parameter objects for all commands
- **Security-first design**: Whitelist-based file access with path validation
- **Configuration-driven**: Centralized constants, runtime config, and validation limits

> **Detailed Architecture**: See [`.claude/docs/architecture/`](.claude/docs/architecture/) for comprehensive system design documentation.

## 📋 Development Standards Summary

### 🚨 Critical Principles

1. **Configuration-First**: No hardcoded values, centralized configuration management
2. **Type Safety**: Let the compiler prevent configuration and validation errors
3. **Layered Security**: Intelligent validation that enables legitimate use cases
4. **Tauri v2 Discipline**: Always use object parameters, verify signatures
5. **Fast Feedback**: Build early and often to catch issues immediately

### 🎯 Code Quality Standards (YAGNI/DRY/KISS)

**YAGNI (You Aren't Gonna Need It)**:
- No extensive documentation for simple features
- No complex configuration "for future flexibility" 
- No elaborate abstractions without proven need

**DRY (Don't Repeat Yourself)**:
- Centralized constants (no duplication across modules)
- Re-export from single source of truth
- Shared error handling patterns

**KISS (Keep It Simple)**:
- Simple solutions over complex abstractions
- Clear module names and organization
- Functions over complex macro systems

**Safety Patterns**:
- Never use `.unwrap()` in production → Use proper error handling
- Centralize configuration → No hardcoded values
- Sanitize sensitive data → API keys, paths, PII

### ⚡ Quality Gates

**Before Any Commit**:
```bash
cargo build && npm run build  # Must succeed
cargo fmt && cargo clippy     # Must be clean
cargo test                    # All tests must pass
```

**Use `/qa-check` to automate this validation.**

### 🔒 Security Requirements

- **API Key Handling**: Environment variables only, never in frontend
- **File Access**: Whitelist-based validation for all file operations
- **Error Sanitization**: Automatic PII/API key redaction in logs
- **Path Validation**: Canonicalization and traversal attack prevention

> **Detailed Standards**: See [`.claude/docs/development/`](.claude/docs/development/) for comprehensive development guidelines and [PR Review & YAGNI Principles](.claude/learnings/pr-review-and-yagni-principles.md) for code quality learnings.

## ⚙️ Configuration System

**Three-Tier Architecture**:

1. **Compile-time Constants** (`src-tauri/src/config/constants.rs`)
   - Values that never change at runtime (API endpoints, security patterns)
   
2. **Runtime Configuration** (`src-tauri/src/config/runtime.rs`)
   - Deployment-configurable values via environment variables
   
3. **Validation Limits** (`src-tauri/src/config/validation.rs`)
   - Security and resource limits with type-safe validation

**Environment Integration**:
```rust
let app_config = AppConfig::load()?; // Loads from multiple sources automatically
state.app_config.validation.validate_message_length(message.len())?;
```

> **Configuration Details**: See [`.claude/docs/architecture/configuration-system.md`](.claude/docs/architecture/configuration-system.md) for complete configuration guide.

## 🎯 Project Status

- **Framework**: Tauri v2 (NOT v1)
- **Configuration**: Unified three-tier system (constants, runtime, validation)
- **Security**: Intelligent layered validation with whitelist-based file access
- **API Integration**: Claude 4 Sonnet with configurable tool execution
- **Testing**: Comprehensive validation with optimization patterns
- **Documentation**: Centralized in `.claude/docs/` with cross-references

## 📚 Documentation Index

| Directory | Content | Purpose |
|-----------|---------|---------|
| [**Architecture**](.claude/docs/architecture/) | System design, configuration, security, Tauri patterns | Understanding system structure |
| [**Development**](.claude/docs/development/) | Workflows, standards, setup, testing, quality gates | Day-to-day development guidance |
| [**GitHub**](.claude/docs/github/) | Issue templates, PR guidelines, project management | Process and collaboration |
| [**Learnings**](.claude/docs/learnings/) | Implementation insights, patterns, optimization | Battle-tested wisdom and best practices |

### 🔍 Key Learning Documents

- **[PR Review & YAGNI Principles](.claude/learnings/pr-review-and-yagni-principles.md)** - Critical lessons from code review process, over-engineering prevention, and quality standards

## 🔧 Development Commands

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
cargo fmt            # Format code
cargo clippy         # Lint code
cargo test           # Run tests
```

## 🎯 Getting Started

1. **Environment**: Run `/dev-checklist` to validate setup
2. **Pick Work**: Use `/pick-next-ticket` (simple) or `/pick-next-feature` (comprehensive)
3. **Quality Check**: Use `/qa-check` before commits
4. **Submit**: Use `/create-pr` for professional PR creation

For detailed guidance on any topic, explore the documentation in `.claude/docs/` or use the relevant commands.

---

**Command Help**: All commands include detailed usage instructions. For command reference, see [`.claude/commands/README.md`](.claude/commands/README.md).