# Architecture Documentation

This directory contains architectural documentation for the Claude AI Agent system.

## Overview

The Claude AI Agent is a Tauri v2-based desktop application that provides a secure interface for interacting with Claude AI, complete with file system tools and advanced content processing capabilities.

## Architecture Documents

### Core Systems

- **[ContentBlock System](./content-block-system.md)** - Foundation for structured content handling, including text, tool use, tool results, and thinking blocks. This system enables type-safe message processing and tool interaction.

- **[Error Handling System](./error-handling-system.md)** - Comprehensive error handling framework with `thiserror` integration, circuit breaker patterns, retry logic, and structured telemetry for robust failure management.

### API References

- **[ContentBlock Types API](../api/content-block-types.md)** - Complete API reference for ContentBlock enum, ClaudeMessage structure, and error handling types.

- **[Error Types API](../api/error-types.md)** - Comprehensive API reference for ClaudeError enum, ErrorContext, ErrorHandler, and telemetry systems with `thiserror` integration.

- **[Model Registry API](../api/model-registry-types.md)** - API reference for ModelRegistry, ModelInfo, ModelSelectionCriteria, and intelligent model selection with fallback strategies.

### Integration Guides

- **[Tool ContentBlock Integration](../tools/content-block-integration.md)** - Guide for implementing tools that work with the ContentBlock system.

- **[Error Handling Integration](../tools/error-handling-integration.md)** - Comprehensive guide for integrating error handling, circuit breakers, model selection, and telemetry into application components.

## System Architecture Overview

### Technology Stack

- **Backend**: Rust with Tauri v2 framework
- **Frontend**: Vanilla JavaScript with modern CSS
- **API Integration**: Claude API via HTTP/REST
- **Security**: Whitelist-based file access control
- **State Management**: Thread-safe Arc<Mutex<>> patterns

### Key Components

1. **Tauri Application Layer**
   - Desktop application framework
   - Secure bridge between web view and system
   - Command handlers for all operations

2. **Claude Integration Layer**
   - HTTP client for Claude API
   - Message processing and validation
   - Tool execution coordination

3. **ContentBlock System**
   - Type-safe content representation
   - Extensible message format
   - Tool integration framework

4. **Error Handling System**
   - Comprehensive error taxonomy with `thiserror`
   - Circuit breaker patterns for failure protection
   - Exponential backoff with jitter
   - Structured logging and telemetry

5. **Model Registry System**
   - Intelligent model selection and optimization
   - Fallback chain management
   - Cost estimation and budget controls
   - Performance tier classification

6. **Security Layer**
   - Whitelist configuration
   - Path validation
   - API key management

7. **Tool System**
   - Async trait-based tools
   - File operations (read, write, list)
   - Extensible tool registry

### Data Flow

```
User Input → Frontend → Tauri Commands → Backend Processing → Claude API
     ↑                                                            ↓
     ←─────────── Tool Execution ←─── Response Processing ←──────
```

### State Management

The application maintains several stateful components:

- **Conversation State**: Thread-safe conversation history
- **Configuration**: Runtime and compile-time settings
- **Whitelist**: Dynamic file access permissions
- **File Watcher**: Real-time file system monitoring

### Security Model

- **API Key Security**: Environment variable only, never exposed to frontend
- **File Access**: Whitelist-based validation with path canonicalization
- **Input Validation**: Multi-layer validation at entry points
- **Error Handling**: Sensitive data excluded from error messages

## Development Guidelines

### Adding New Features

1. **Understand the ContentBlock System** - All content flows through ContentBlocks
2. **Follow Async Patterns** - Use async/await for all I/O operations
3. **Implement Proper Error Handling** - Use ClaudeError with context
4. **Respect Security Boundaries** - Validate all inputs, use whitelist for files
5. **Write Tests** - Unit and integration tests required

### Code Organization

```
src-tauri/
├── src/
│   ├── main.rs              # Tauri entry point and commands
│   ├── claude/              # Claude integration module
│   │   ├── mod.rs          # Module configuration
│   │   ├── types.rs        # ContentBlock and message types
│   │   ├── client.rs       # API client implementation
│   │   ├── tools.rs        # Tool system implementation
│   │   ├── error.rs        # Error types and handling
│   │   └── whitelist.rs    # Security whitelist
│   ├── config/             # Configuration system
│   │   ├── constants.rs    # Compile-time constants
│   │   ├── runtime.rs      # Runtime configuration
│   │   └── validation.rs   # Validation limits
│   └── file_watcher.rs     # File system monitoring
```

### Best Practices

1. **Type Safety First** - Leverage Rust's type system
2. **Configuration Over Code** - Use the config system, avoid hardcoding
3. **Consistent Error Handling** - Always provide context
4. **Security by Default** - Validate inputs, use whitelists
5. **Performance Awareness** - Profile and optimize critical paths

## Future Architecture Plans

### Planned Enhancements

1. **Streaming Support** - Real-time content streaming from Claude
2. **Advanced Tools** - Computer use, code execution, text editing
3. **Parallel Execution** - Concurrent tool execution
4. **Enhanced Monitoring** - Metrics and observability
5. **Enterprise Features** - SSO, audit logging, compliance

### Extension Points

- **Custom Tools** - Implement AgentTool trait
- **Content Types** - Extend ContentBlock enum
- **Configuration** - Add new config sections
- **UI Components** - Frontend extensions

## Related Documentation

- [Implementation Sequencing](../implementation-sequencing.md) - Development roadmap
- [Issue Tracking](../issues/) - Detailed feature specifications
- [CLAUDE.md](../../CLAUDE.md) - AI assistant instructions

## Architecture Decision Records (ADRs)

Key architectural decisions are documented here:

### ADR-001: ContentBlock System
- **Decision**: Use enum-based content blocks instead of string messages
- **Rationale**: Type safety, extensibility, clear semantics
- **Consequences**: Breaking change from legacy format, migration required

### ADR-002: Async Tool System
- **Decision**: All tools use async trait pattern
- **Rationale**: I/O operations benefit from async, consistency
- **Consequences**: Requires tokio runtime, async complexity

### ADR-003: Whitelist Security
- **Decision**: Whitelist-based file access control
- **Rationale**: Explicit permission model, security by default
- **Consequences**: User must approve directories, some friction

## Diagrams

### System Overview
```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│                 │     │                  │     │                 │
│   Frontend UI   │────▶│  Tauri Commands  │────▶│  Claude Client  │
│   (JavaScript)  │     │    (Rust)        │     │    (Rust)       │
│                 │     │                  │     │                 │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │                          │
                               ▼                          ▼
                        ┌──────────────────┐     ┌─────────────────┐
                        │                  │     │                 │
                        │   Tool System    │     │   Claude API    │
                        │   (Async Rust)   │     │   (HTTPS)       │
                        │                  │     │                 │
                        └──────────────────┘     └─────────────────┘
```

### ContentBlock Flow
```
User Message
    │
    ▼
ContentBlock::Text
    │
    ▼
Claude Processing
    │
    ▼
ContentBlock::ToolUse
    │
    ▼
Tool Execution
    │
    ▼
ContentBlock::ToolResult
    │
    ▼
Response Assembly
    │
    ▼
UI Display
```