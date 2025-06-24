# System Architecture Overview

This document provides a comprehensive overview of the system architecture, core components, and key design patterns.

## 🏗️ System Architecture

This is a **Tauri v2-based desktop application** that provides a chat interface for Claude AI with secure file system tools.

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (Vanilla JS)                    │
├─────────────────────────────────────────────────────────────┤
│                    Tauri v2 Bridge                         │
├─────────────────────────────────────────────────────────────┤
│                    Backend (Rust)                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │   Config    │ │   Claude    │ │  Security   │          │
│  │   System    │ │ Integration │ │  & File     │          │
│  │             │ │             │ │  System     │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Core Components

### Backend (Rust)

#### **Main Application (`src-tauri/src/main.rs`)**
- Tauri v2 app entry point with command handlers
- State management with thread-safe patterns
- Command routing and error handling

#### **Configuration System (`src-tauri/src/config/`)**
- **`mod.rs`** - Master configuration with environment loading
- **`constants.rs`** - Compile-time constants (API endpoints, models, limits)
- **`runtime.rs`** - Runtime-configurable settings (timeouts, feature flags)
- **`validation.rs`** - Validation limits with type-safe helpers

See [Configuration System](./configuration-system.md) for detailed architecture.

#### **Claude Integration (`src-tauri/src/claude/`)**
- **`client.rs`** - HTTP client for Claude API with tool execution
- **`tools.rs`** - Async trait-based tool system with whitelist validation
- **`types.rs`** - Type definitions for Claude API
- **`whitelist.rs`** - Security whitelist system for file access control
- **`mod.rs`** - Configuration and conversation management

#### **File System & Security**
- **`file_watcher.rs`** - Real-time file system monitoring with debounced updates
- **`security.rs`** - Security utilities and validation

See [Security Model](./security-model.md) for detailed security architecture.

### Frontend (Vanilla JS)

#### **Core Files**
- **`ui/index.html`** - Main application interface
- **`ui/js/app.js`** - Chat UI, file explorer, and Tauri v2 command bindings
- **`ui/js/config.js`** - Frontend configuration constants (synced with backend)
- **`ui/css/styles.css`** - Modern styling

## 🔑 Key Architecture Patterns

### **Thread-Safe State Management**
```rust
struct AppState {
    conversation: Arc<Mutex<Conversation>>,    // Exclusive access for modifications
    config: Arc<Mutex<ClaudeConfig>>,          // Configuration updates
    app_config: Arc<AppConfig>,                // Immutable shared configuration
    whitelist: Arc<RwLock<WhitelistConfig>>,   // Concurrent read access
    file_watcher: Arc<FileWatcherService>,     // Shared file monitoring
}
```

**Design Decisions**:
- `Arc<Mutex<>>` for data requiring exclusive modification
- `Arc<RwLock<>>` for read-heavy, infrequently-written data
- `Arc<>` for immutable shared data

### **Async Tool System**
```rust
#[async_trait]
pub trait AgentTool: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    async fn execute(&self, input: Value) -> Result<String>;
    fn set_whitelist(&mut self, whitelist: Arc<RwLock<WhitelistConfig>>);
}
```

**Design Principles**:
- All tools implement async trait for non-blocking execution
- Whitelist validation required for all file operations
- Type-safe input/output with comprehensive error handling

### **Tauri v2 Communication**
```javascript
// Frontend: Always use named parameter objects
await window.__TAURI__.core.invoke('send_message_to_claude', { message: 'Hello' });

// Backend: Corresponding Rust signature
#[tauri::command]
async fn send_message_to_claude(message: String, state: tauri::State<'_, AppState>) -> Result<String, String>
```

**Critical Requirements**:
- Named parameter objects required in JavaScript calls
- Rust functions receive named parameters
- Consistent error handling across the bridge

See [Tauri Patterns](./tauri-patterns.md) for comprehensive Tauri v2 implementation details.

### **Security-First Design**
- **Whitelist-based file access** with path canonicalization
- **API key isolation** - never exposed in frontend code
- **Error message sanitization** - automatic PII/API key redaction
- **Path traversal protection** - intelligent validation without breaking legitimate use

See [Security Model](./security-model.md) for detailed security implementation.

### **Configuration-Driven Architecture**
- **No hardcoded values** - all limits and constants centrally managed
- **Three-tier system** - compile-time, runtime, and validation layers
- **Type-safe validation** - compile-time prevention of configuration errors
- **Environment flexibility** - easy deployment configuration

See [Configuration System](./configuration-system.md) for complete configuration architecture.

## 📊 Data Flow

### **Message Processing Flow**
```
User Input → Frontend Validation → Tauri Command → 
Backend Processing → Claude API → Tool Execution → 
Response Processing → Frontend Update
```

### **File System Operations**
```
File Request → Whitelist Validation → Path Canonicalization → 
Security Check → Tool Execution → Response Sanitization → 
Result Return
```

### **Configuration Loading**
```
Environment Variables → Config Files → Defaults → 
Validation → Type-Safe Access → Runtime Usage
```

## 🎯 Design Principles

### **1. Configuration-First**
No hardcoded values; centralized configuration management with environment flexibility.

### **2. Type Safety**
Leverage Rust's type system to prevent configuration and validation errors at compile time.

### **3. Layered Security**
Intelligent validation that enables legitimate use cases while preventing abuse.

### **4. Consistency**
Mirror patterns between frontend and backend; maintain consistent conventions.

### **5. Framework Discipline**
Follow Tauri v2 conventions strictly; use established patterns correctly.

### **6. Fast Feedback**
Build early and often to catch issues immediately; automated quality gates.

## 🔄 System Interactions

### **Startup Sequence**
1. Load configuration from environment and defaults
2. Initialize security whitelist with current directory
3. Start file watcher service
4. Initialize Claude client with API key validation
5. Setup Tauri command handlers
6. Launch frontend interface

### **Tool Execution Sequence**
1. Receive tool request from Claude API
2. Validate tool name and parameters
3. Check whitelist permissions for file operations
4. Execute tool with security constraints
5. Sanitize output for sensitive information
6. Return results to Claude API

### **Error Handling Flow**
1. Capture errors at appropriate boundaries
2. Add structured context (operation, timestamp, metadata)
3. Sanitize error messages for security
4. Log with appropriate severity level
5. Return user-friendly error messages

## 📈 Performance Considerations

### **Memory Management**
- Shared ownership with `Arc<>` for read-only data
- Efficient string handling with borrowing
- Bounded collections to prevent unbounded growth

### **Concurrency**
- Atomic operations for simple state transitions
- Reader-writer locks for read-heavy data
- Proper lock ordering to prevent deadlocks

### **Resource Bounds**
- Configurable timeouts for all external requests
- File size limits for tool operations
- Message length validation
- Connection pooling for HTTP clients

## 🔗 Related Documentation

- [Configuration System](./configuration-system.md) - Detailed configuration architecture
- [Security Model](./security-model.md) - Security design and implementation
- [Tauri Patterns](./tauri-patterns.md) - Tauri v2 specific patterns and requirements
- [Development Standards](../development/rust-standards.md) - Implementation best practices

## 💡 Commands Integration

- Use `/config-check` to validate configuration consistency
- Use `/security-check` to verify security implementation
- Use `/review-pr` to enforce architectural patterns
- Use `/qa-check` to validate build and compilation

This architecture provides a solid foundation for secure, maintainable, and scalable development while leveraging the strengths of both Rust and modern web technologies.