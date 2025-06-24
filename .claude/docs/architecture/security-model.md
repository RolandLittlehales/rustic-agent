# Security Model - Claude Development Guide

This guide covers the security model from a Claude Code development perspective, with command integration and security quality enforcement.

## 📚 Architecture Reference

**For complete security architecture documentation, see: [Security Model Architecture](../../../docs/architecture/security-model.md)**

The security model implements defense-in-depth with whitelist-based file access, API key isolation, automatic sanitization, and intelligent validation. The main documentation covers the detailed technical implementation.

## 🎯 Claude Security Integration

### **Security Command Integration**

The security model is validated and enforced through Claude commands:

#### **`/security-check`** - Comprehensive Security Validation
- **API key security** - Validates no frontend exposure or logging
- **File system security** - Ensures whitelist validation usage
- **Error sanitization** - Confirms automatic PII/API key redaction
- **Network security** - Validates HTTPS usage and certificate validation
- **Input validation** - Checks security boundary enforcement

#### **`/review-pr`** - Security Standards in Code Review
- **Security pattern compliance** - Enforces established security patterns
- **Whitelist validation usage** - Ensures no security bypass
- **Error sanitization** - Validates automatic sanitization usage
- **API key isolation** - Confirms backend-only API key handling

#### **`/qa-check`** - Security as Quality Gate
- **Security boundary testing** - Validates security controls work
- **Configuration security** - Tests secure configuration loading
- **Error handling security** - Ensures no sensitive data exposure

### **Development Workflow Integration**

#### **When Starting Security Features (`/start-feature`)**
1. **Security Impact Assessment**: Analyze security implications
2. **Threat Modeling**: Identify potential security risks
3. **Security Pattern Selection**: Choose appropriate security controls
4. **Validation Strategy**: Plan security testing approach

#### **During Implementation**
```rust
// ✅ Always use whitelist validation
whitelist.validate_access(Path::new(path), FileOperation::Read)?;

// ❌ Never bypass security (detected by /security-check)
if path.contains("..") { // Overly simplistic, will be flagged
    return Err("Invalid path".into());
}

// ✅ Use automatic sanitization
let context = ErrorContext::new("operation");
context.log_error(&error); // Automatically sanitizes

// ❌ Direct logging (detected by /security-check)
eprintln!("Error: {}", error); // Could expose sensitive data
```

#### **Before PR Creation (`/create-pr`)**
- Run `/security-check` to validate security implementation
- Verify no security bypasses introduced
- Check error sanitization patterns
- Validate API key isolation maintained

## 🔧 Security Anti-Patterns Detected by Commands

### **API Key Exposure (Detected by `/security-check`)**
```rust
// ❌ API key in error message
return Err(format!("API request failed with key {}", api_key));

// ✅ Automatic sanitization
let context = ErrorContext::new("api_request");
context.log_error(&error); // API key automatically redacted
```

### **Whitelist Bypass (Detected by `/security-check`)**
```rust
// ❌ Direct file access without validation
let content = fs::read_to_string(path)?; // Security bypass!

// ✅ Whitelist validation required
let whitelist = state.whitelist.read().await;
whitelist.validate_access(Path::new(path), FileOperation::Read)?;
let content = fs::read_to_string(path)?;
```

### **Frontend API Key Exposure (Detected by `/security-check`)**
```javascript
// ❌ API key in frontend code
const apiKey = "sk-ant-..."; // Never do this!

// ✅ Backend-only API key handling
// Frontend never receives or handles API keys
const response = await window.__TAURI__.core.invoke('claude_request', { message });
```

## 📋 Security Quality Checklist

### **Before Implementing Security Features**
- [ ] Review security architecture documentation
- [ ] Identify security boundaries and threat model
- [ ] Plan whitelist validation approach
- [ ] Design error sanitization strategy

### **During Security Implementation**
- [ ] Use whitelist validation for all file operations
- [ ] Implement automatic error sanitization
- [ ] Maintain API key isolation (backend only)
- [ ] Follow established security patterns

### **Security Validation**
- [ ] Run `/security-check` to validate security implementation
- [ ] Verify no security bypasses detected
- [ ] Check error sanitization working
- [ ] Confirm API key isolation maintained

## 🎯 Command Usage Examples

### **Security Validation Workflow**
```bash
# During development
/security-check                 # Validate security implementation
/security-check --verbose       # Detailed security analysis

# Before PR
/security-check                 # Final security validation
/review-pr "current changes"    # Include security review
```

### **Security Feature Development**
```bash
/start-feature "#20 enhanced file security"
# → Security impact assessment
# → Implementation with security patterns
# → /security-check validation
# → /review-pr including security compliance
# → /create-pr with security documentation
```

## 🛡️ Security Patterns Enforced by Commands

### **Whitelist Validation Pattern**
```rust
// ✅ Standard pattern enforced by /security-check
#[tauri::command]
async fn secure_file_operation(path: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    // 1. Whitelist validation (required)
    let whitelist = state.whitelist.read().await;
    whitelist.validate_access(Path::new(&path), FileOperation::Read)
        .map_err(|e| format!("Access denied: {}", e))?;
    
    // 2. Additional validation
    state.app_config.validation.validate_file_size(&path)?;
    
    // 3. Safe operation
    fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))
}
```

### **Error Sanitization Pattern**
```rust
// ✅ Standard pattern enforced by /security-check
impl ErrorHandler {
    pub fn handle_error(&self, error: &ClaudeError) -> String {
        // Automatic sanitization prevents sensitive data exposure
        let context = ErrorContext::new(self.operation_name);
        context.log_error(error);
        
        // Return user-friendly error without sensitive details
        "Operation failed. Check logs for details.".to_string()
    }
}
```

### **API Key Isolation Pattern**
```rust
// ✅ Backend-only pattern enforced by /security-check
impl ClaudeClient {
    pub fn new() -> Result<Self> {
        // Load from environment only
        let api_key = env::var("CLAUDE_API_KEY")
            .map_err(|_| "API key not configured")?;
        
        // Validate format
        if !api_key.starts_with("sk-ant-") {
            return Err("Invalid API key format".into());
        }
        
        // Never expose to frontend
        Ok(Self { api_key })
    }
}
```

## 🔗 Related Claude Documentation

- **[Development Standards](../development/rust-standards.md)** - Security coding patterns
- **[Quality Gates](../development/quality-gates.md)** - Security validation requirements
- **[Commands Reference](../../commands/security-check.md)** - Detailed `/security-check` usage

## ⚠️ Security Development Notes

### **Zero Tolerance Security Issues**
- **API key exposure** - Immediate fix required, flagged by `/security-check`
- **Whitelist bypass** - Critical security issue, blocked by `/review-pr`
- **Information disclosure** - Must be addressed, detected by error analysis
- **Insecure communications** - No HTTP allowed, enforced by patterns

### **Security-First Development**
1. **Design security boundaries** before implementation
2. **Use established patterns** validated by commands
3. **Test security controls** with `/security-check`
4. **Review security implications** with `/review-pr`
5. **Document security decisions** for future reference

### **Emergency Security Procedures**
- **Security issue discovered**: Run `/security-check` immediately
- **False positive**: Document rationale and get security review
- **New threat identified**: Update security patterns and validation
- **Compliance requirement**: Integrate into command validation

This guide ensures security is properly implemented and maintained through Claude development workflows while following the comprehensive security architecture documented in the main documentation.