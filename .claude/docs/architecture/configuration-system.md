# Configuration System - Claude Development Guide

This guide covers the configuration system from a Claude Code development perspective, with command integration and quality enforcement.

## 📚 Architecture Reference

**For complete architectural documentation, see: [Configuration System Architecture](../../../docs/architecture/configuration-system.md)**

The configuration system uses a **three-tier architecture** with different lifecycles and purposes. The main documentation covers the technical implementation details.

## 🎯 Claude Development Integration

### **Command Integration**

The configuration system is validated and enforced through Claude commands:

#### **`/config-check`** - Configuration Validation
- **Frontend-backend synchronization** - Ensures consistency across layers
- **Hardcoded value detection** - Identifies magic numbers and string literals
- **Three-tier compliance** - Validates proper layer separation
- **Documentation validation** - Ensures constants are properly documented

#### **`/review-pr`** - Configuration Standards in Code Review
- **Architecture compliance** checking during code review
- **Configuration-first development** pattern enforcement
- **Type safety validation** for configuration usage
- **Cross-layer validation** rule verification

#### **`/qa-check`** - Configuration as Quality Gate
- **Build-time validation** that configuration loads correctly
- **Environment variable** testing
- **Configuration consistency** as part of quality assurance

### **Development Workflow Integration**

#### **When Starting Features (`/start-feature`)**
1. **Configuration Planning**: Identify any new configuration needs
2. **Layer Assignment**: Determine appropriate tier for new values
3. **Documentation**: Plan configuration documentation requirements
4. **Validation**: Design type-safe validation patterns

#### **During Implementation**
```rust
// ✅ Use configuration system consistently
app_config.validation.validate_message_length(message.len())?;

// ❌ Avoid hardcoded values that trigger /config-check
if message.len() > 50000 { // Will be flagged by /config-check
    return Err("Message too long".into());
}
```

#### **Before PR Creation (`/create-pr`)**
- Run `/config-check` to ensure configuration consistency
- Verify frontend-backend synchronization
- Check for hardcoded values in changes
- Validate configuration documentation

## 🔧 Configuration Anti-Patterns Detected by Commands

### **Layer Mixing (Detected by `/config-check`)**
```rust
// ❌ Runtime value in constants layer
pub const TIMEOUT_MS: u64 = env::var("TIMEOUT").parse().unwrap();

// ✅ Proper layer separation
// Constants layer: pub const DEFAULT_TIMEOUT_MS: u64 = 120000;
// Runtime layer: timeout_ms loaded from environment
```

### **Frontend-Backend Drift (Detected by `/config-check`)**
```javascript
// ❌ Manual duplication leads to drift
const MESSAGE_MAX_CHARS = 45000; // Different from backend!

// ✅ Auto-generated synchronization
import { CONFIG } from './config-generated.js';
const maxChars = CONFIG.MESSAGE_MAX_CHARS;
```

### **Hardcoded Values (Detected by `/config-check`)**
```rust
// ❌ Magic numbers flagged by /config-check
if file_size > 10485760 { // Should use configuration

// ✅ Configuration-driven approach
if file_size > app_config.validation.file_max_size_bytes {
```

## 📋 Configuration Quality Checklist

### **Before Adding New Configuration**
- [ ] Determine appropriate tier (constants/runtime/validation)
- [ ] Document business rationale for values
- [ ] Plan frontend synchronization if needed
- [ ] Consider environment-specific requirements

### **During Implementation**
- [ ] Use type-safe configuration access patterns
- [ ] Avoid hardcoded values in implementation
- [ ] Update build scripts for frontend sync if needed
- [ ] Add validation logic where appropriate

### **Quality Validation**
- [ ] Run `/config-check` to validate consistency
- [ ] Verify no hardcoded values detected
- [ ] Check frontend-backend synchronization
- [ ] Ensure configuration documentation complete

## 🎯 Command Usage Examples

### **Configuration Validation Workflow**
```bash
# During development
/config-check                    # Check current state
/config-check --fix             # Attempt automatic fixes

# Before PR
/config-check --verbose         # Detailed analysis
/review-pr "current changes"    # Include config review
```

### **Feature Development with Configuration**
```bash
/start-feature "#15 timeout configuration"
# → Implementation with configuration planning
# → /config-check during development
# → /review-pr including configuration compliance
# → /create-pr with configuration documentation
```

## 🔗 Related Claude Documentation

- **[Development Standards](../development/rust-standards.md)** - Configuration usage patterns
- **[Quality Gates](../development/quality-gates.md)** - Configuration validation requirements
- **[Commands Reference](../../commands/config-check.md)** - Detailed `/config-check` usage

## ⚠️ Configuration Security Notes

### **API Key Handling**
- **Never in frontend**: API keys stay in backend environment variables
- **Automatic sanitization**: Error messages automatically redact API keys
- **Environment only**: No API keys in configuration files

### **Sensitive Configuration**
- Use environment variables for sensitive values
- Validate configuration at startup
- Log configuration status without exposing values
- Implement secure defaults for all settings

This guide ensures configuration is properly managed through Claude development workflows while maintaining the architectural principles documented in the main documentation.