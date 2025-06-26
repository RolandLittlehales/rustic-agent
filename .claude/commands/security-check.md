# Security Check

Comprehensive security validation focusing on API key handling, file system security, error sanitization, and other security-critical aspects.

## 🎯 Purpose

This command performs **focused security validation** to ensure the application maintains robust security posture across all critical areas including API key handling, file system access, error message sanitization, and security boundary enforcement.

## 🔒 Security Validation Areas

### **🔑 API Key Security**

#### **Environment Variable Validation**
```rust
// ✅ Validate: API keys loaded from environment only
✅ CLAUDE_API_KEY environment variable usage
✅ No API keys in frontend code (HTML/JavaScript)  
✅ No API keys in configuration files
✅ No API keys in error messages or logs
```

#### **Secure Storage and Handling**
```rust
// ✅ Validate: Proper API key management
impl RuntimeConfig {
    pub fn load_api_key(&mut self) -> Result<()> {
        match env::var("CLAUDE_API_KEY") {
            Ok(key) if key.starts_with("sk-ant-") => {
                self.api_key = Some(key);
                Ok(())
            }
            _ => Err("Invalid or missing CLAUDE_API_KEY".into())
        }
    }
}
```

#### **API Key Sanitization**
```rust
// ✅ Validate: Automatic sanitization in logs
impl ErrorContext {
    fn sanitize_error_message(message: &str) -> String {
        let mut sanitized = message.to_string();
        if let Some(start) = sanitized.find("sk-ant-") {
            sanitized.replace_range(start.., "[API_KEY_REDACTED]");
        }
        sanitized
    }
}
```

### **📁 File System Security**

#### **Whitelist-Based Access Control**
```rust
// ✅ Validate: All file operations use whitelist validation
✅ Path canonicalization prevents directory traversal
✅ Whitelist configuration properly maintained
✅ No direct file access bypassing security layer
✅ Proper error handling for access denied scenarios
```

#### **Path Validation and Sanitization**
```rust
// ✅ Validate: Secure path handling
pub fn validate_path(path: &str, whitelist: &WhitelistConfig) -> Result<PathBuf> {
    let canonical = fs::canonicalize(path)?;
    whitelist.validate_access(&canonical, FileOperation::Read)
}

// ❌ Anti-pattern: Premature rejection without proper validation
if path.contains("..") { return Err("Invalid path".into()); }
```

#### **File Size and Access Limits**
```rust
// ✅ Validate: Resource protection
✅ File size limits enforced (default 10MB)
✅ Directory depth limits enforced
✅ Maximum files per directory respected
✅ Timeout protection for file operations
```

### **🛡️ Error Message Security**

#### **Automatic Sanitization**
```rust
// ✅ Validate: No sensitive information in error messages
✅ API keys automatically redacted
✅ File paths sanitized (user directories)
✅ System information not exposed
✅ Database connection strings not exposed
✅ Internal implementation details not revealed
```

#### **Security Pattern Detection**
```rust
// ✅ Validate: Dangerous patterns prevented
const SUSPICIOUS_PATTERNS: &[&str] = &[
    "<script",           // XSS prevention
    "javascript:",       // Protocol injection
    "data:",            // Data URI attacks
    "vbscript:",        // VBScript injection
];
```

### **🌐 Network Security**

#### **HTTPS Enforcement**
```rust
// ✅ Validate: Secure communications
✅ All external API calls use HTTPS
✅ Certificate validation enabled
✅ No insecure HTTP endpoints
✅ Proper timeout handling for network requests
```

#### **Request Validation**
```rust
// ✅ Validate: Request security
✅ User-Agent header properly set
✅ Request size limits enforced
✅ Rate limiting considerations
✅ Proper error handling for network failures
```

### **🔐 Input Validation Security**

#### **Message Content Validation**
```rust
// ✅ Validate: Input sanitization
✅ Message length limits enforced
✅ Content type validation
✅ Encoding validation (UTF-8)
✅ No code injection vectors
```

#### **Parameter Validation**
```rust
// ✅ Validate: Tauri command security
✅ All commands use object parameters (Tauri v2)
✅ Parameter validation at command boundaries
✅ Type safety enforced
✅ No direct user input to system calls
```

## 💡 Usage

### **Basic Usage**
```
/security-check
```

**No arguments required** - performs comprehensive security validation.

### **Example Output (All Clear)**
```
🔒 Running Security Validation...

🔑 API Key Security: ✅ SECURE
✅ API key loaded from environment variable only
✅ No API keys detected in frontend code
✅ No API keys in configuration files  
✅ Error message sanitization active
✅ Logging redaction patterns working

📁 File System Security: ✅ SECURE
✅ Whitelist system active and configured
✅ Path canonicalization working correctly
✅ No file access bypassing security layer
✅ File size limits enforced (10MB max)
✅ Directory traversal protection active

🛡️ Error Message Security: ✅ SECURE  
✅ Automatic API key redaction in logs
✅ User directory paths sanitized
✅ No system information exposed
✅ Error messages user-friendly without details
✅ No suspicious patterns detected

🌐 Network Security: ✅ SECURE
✅ All API calls use HTTPS endpoints
✅ Certificate validation enabled
✅ Request timeouts properly configured
✅ User-Agent headers set appropriately

🔐 Input Validation: ✅ SECURE
✅ Message length validation active
✅ Content encoding validation (UTF-8)
✅ Tauri command parameters properly validated
✅ No code injection vectors detected

🎉 Security Status: ALL CHECKS PASSED ✅

🛡️ Security posture is excellent. No vulnerabilities detected.
```

### **Example Output (Issues Found)**
```
🔒 Running Security Validation...

🔑 API Key Security: ⚠️ ISSUES FOUND
✅ API key loaded from environment variable
❌ API key detected in error message
   📍 src-tauri/src/claude/client.rs:123
   💡 Error: "Failed to authenticate with key sk-ant-..."
   🔧 Fix: Use sanitization before logging

📁 File System Security: ❌ CRITICAL ISSUE
✅ Whitelist system configured
❌ File access bypassing whitelist detected
   📍 src-tauri/src/claude/tools.rs:67
   💡 Direct fs::read() call without validation
   🔧 Fix: Use whitelist.validate_access() first

🛡️ Error Message Security: ⚠️ MINOR ISSUE
❌ User path exposed in error message
   📍 ui/js/app.js:234
   💡 Error shows full path: "/home/username/secret/file.txt"
   🔧 Fix: Sanitize paths in frontend error handling

🚨 Security Status: CRITICAL ISSUES FOUND ❌

🔧 Immediate Action Required:
1. CRITICAL: Fix file access bypass in tools.rs
2. HIGH: Remove API key from error message
3. MEDIUM: Sanitize user paths in frontend

❌ Security validation FAILED. Address these issues before proceeding.
```

## 🔍 Detailed Security Analysis

### **API Key Exposure Prevention**
```rust
// ✅ Security check validates these patterns
impl SecurityValidator {
    fn check_api_key_exposure(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        
        // Check frontend files for API key patterns
        for file in &self.frontend_files {
            if file.content.contains("sk-ant-") {
                issues.push(SecurityIssue::ApiKeyInFrontend(file.path.clone()));
            }
        }
        
        // Check log messages for API key exposure
        for log_call in &self.log_calls {
            if !log_call.uses_sanitization {
                issues.push(SecurityIssue::UnsanitizedLogging(log_call.location.clone()));
            }
        }
        
        issues
    }
}
```

### **File System Access Validation**
```rust
// ✅ Security check validates whitelist usage
impl SecurityValidator {
    fn check_file_access_patterns(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        
        // Find direct file system calls
        for fs_call in &self.fs_calls {
            if !fs_call.uses_whitelist_validation {
                issues.push(SecurityIssue::BypassingWhitelist(fs_call.location.clone()));
            }
        }
        
        // Check for path traversal vulnerabilities
        for path_handling in &self.path_handlers {
            if !path_handling.uses_canonicalization {
                issues.push(SecurityIssue::PathTraversalRisk(path_handling.location.clone()));
            }
        }
        
        issues
    }
}
```

### **Error Message Sanitization Validation**
```rust
// ✅ Security check validates sanitization patterns
impl SecurityValidator {
    fn check_error_sanitization(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        
        // Check for unsanitized error logging
        for error_log in &self.error_logs {
            if !error_log.uses_sanitization {
                issues.push(SecurityIssue::UnsanitizedErrorLogging(error_log.location.clone()));
            }
        }
        
        // Check for information disclosure
        for error_msg in &self.error_messages {
            if error_msg.contains_sensitive_info() {
                issues.push(SecurityIssue::InformationDisclosure(error_msg.location.clone()));
            }
        }
        
        issues
    }
}
```

## 🔧 Security Configuration

### **Whitelist Configuration Validation**
```toml
# ✅ Security check validates these settings
[whitelist]
enabled = true
auto_add_current_dir = true
max_depth = 10
max_file_size = 10485760  # 10MB
protected_patterns = [".env", ".secret", "id_rsa", "*.key"]
```

### **Security Headers and Settings**
```rust
// ✅ Security check validates these configurations
const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("User-Agent", "Claude-Desktop/1.0"),
    ("Accept", "application/json"),
    ("Content-Type", "application/json"),
];

const PROTECTED_DIRECTORIES: &[&str] = &[
    ".ssh",
    ".aws", 
    ".config/gcloud",
    "AppData/Roaming",
];
```

## 🔗 Integration with Other Commands

### **Workflow Integration**
- **[`/start-feature`](./start-feature.md)** - Security validation during development
- **[`/review-pr`](./review-pr.md)** - Security aspects in code review
- **[`/qa-check`](./qa-check.md)** - Basic security as part of QA

### **Security-Specific Validation**
- **File operations** - Validates whitelist usage
- **API handling** - Ensures secure patterns
- **Error handling** - Checks sanitization
- **Network requests** - Validates HTTPS usage

## 📋 Security Checklist

### **Before Any Security-Related Changes**
- [ ] API key handling patterns secure
- [ ] File system access properly validated  
- [ ] Error messages sanitized
- [ ] Network requests use HTTPS
- [ ] Input validation comprehensive

### **Regular Security Validation**
- [ ] Run `/security-check` after any security-related changes
- [ ] Include security validation in `/review-pr`
- [ ] Regular audits of whitelist configuration
- [ ] Monitor for new security patterns

## ⚠️ Important Security Notes

### **Zero Tolerance Policy**
- **API key exposure** - Immediate fix required
- **File system bypass** - Critical security issue
- **Information disclosure** - Must be addressed
- **Insecure communications** - No HTTP allowed

### **Defense in Depth**
- **Multiple validation layers** - Whitelist + path validation
- **Automatic sanitization** - Default secure behavior
- **Fail secure** - Deny access by default
- **Audit trail** - Security events logged

This command ensures the application maintains enterprise-grade security standards and quickly identifies potential vulnerabilities before they reach production.