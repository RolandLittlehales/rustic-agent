# Security Model

The application implements a comprehensive security architecture based on defense-in-depth principles, with whitelist-based file access, complete API key isolation, automatic error sanitization, and intelligent validation patterns.

## Security Architecture Principles

1. **Whitelist-Based Access**: Default-deny with explicit permission model
2. **API Key Isolation**: Complete separation between frontend and backend secrets
3. **Automatic Sanitization**: Security-by-default error handling
4. **Path Canonicalization**: Robust directory traversal protection
5. **Bounded Resources**: Protection against resource exhaustion attacks
6. **Fail Secure**: Default to denying access when in doubt

## Whitelist-Based File Access System

### **WhitelistConfig Architecture**

```rust
pub struct WhitelistConfig {
    enabled: bool,                           // Global enable/disable
    directories: HashSet<PathBuf>,          // Allowed directory roots
    max_depth: usize,                       // Traversal depth limit
    max_file_size: u64,                     // Individual file size limit
    protected_patterns: Vec<String>,        // File patterns to block
    auto_add_current_dir: bool,            // Convenience for development
}
```

### **Path Validation Process**

```rust
impl WhitelistConfig {
    pub fn validate_access(&self, path: &Path, operation: FileOperation) -> Result<()> {
        // 1. Canonicalize path to resolve symlinks and relative components
        let canonical_path = fs::canonicalize(path)
            .map_err(|_| SecurityError::PathNotFound)?;
        
        // 2. Check if path is within any whitelisted directory
        let is_allowed = self.directories.iter().any(|allowed_dir| {
            canonical_path.starts_with(allowed_dir)
        });
        
        if !is_allowed {
            return Err(SecurityError::PathNotWhitelisted(canonical_path));
        }
        
        // 3. Check protected file patterns
        if let Some(filename) = canonical_path.file_name() {
            let filename_str = filename.to_string_lossy();
            for pattern in &self.protected_patterns {
                if filename_str.contains(pattern) {
                    return Err(SecurityError::ProtectedFile(canonical_path));
                }
            }
        }
        
        // 4. Validate depth limits
        let relative_depth = self.calculate_depth(&canonical_path)?;
        if relative_depth > self.max_depth {
            return Err(SecurityError::DepthLimitExceeded(relative_depth));
        }
        
        // 5. Check file size for read operations
        if operation == FileOperation::Read {
            if let Ok(metadata) = fs::metadata(&canonical_path) {
                if metadata.len() > self.max_file_size {
                    return Err(SecurityError::FileSizeExceeded(metadata.len()));
                }
            }
        }
        
        Ok(())
    }
}
```

### **Directory Traversal Protection**

```rust
// ✅ Secure: Whitelist system handles complex path validation
fn secure_file_access(path: &str, whitelist: &WhitelistConfig) -> Result<String> {
    // Let whitelist system canonicalize and validate
    whitelist.validate_access(Path::new(path), FileOperation::Read)?;
    
    // Safe to proceed with file operation
    fs::read_to_string(path).map_err(Into::into)
}

// ❌ Anti-pattern: Premature path rejection
fn insecure_file_access(path: &str) -> Result<String> {
    // Overly restrictive - breaks legitimate relative paths
    if path.contains("..") {
        return Err("Invalid path".into());
    }
    
    // Still vulnerable to symlink attacks and other traversal methods
    fs::read_to_string(path).map_err(Into::into)
}
```

### **Protected File Patterns**

```rust
pub const PROTECTED_FILES: &[&str] = &[
    // Security credentials
    ".env", ".secret", "id_rsa", "id_ed25519", "*.key", "*.pem",
    
    // Configuration files with secrets
    ".aws/credentials", ".config/gcloud", ".docker/config.json",
    
    // Operating system sensitive files
    "/etc/passwd", "/etc/shadow", "SAM", "SYSTEM", "SECURITY",
    
    // Application secrets
    "appsettings.json", "web.config", "database.yml",
];
```

## API Key Security Architecture

### **Complete Frontend Isolation**

```rust
// ✅ Backend-only API key handling
impl RuntimeConfig {
    pub fn load_api_key() -> Result<String> {
        env::var("CLAUDE_API_KEY")
            .map_err(|_| "CLAUDE_API_KEY environment variable not set")
            .and_then(|key| {
                if key.starts_with("sk-ant-") {
                    Ok(key)
                } else {
                    Err("Invalid API key format")
                }
            })
    }
    
    /// Safe logging without exposing API key
    pub fn debug_info(&self) -> String {
        format!("RuntimeConfig {{ api_key: {}, timeout: {}ms }}",
            if self.api_key.is_some() { "[PRESENT]" } else { "[MISSING]" },
            self.http_timeout_ms
        )
    }
}

// ❌ Never expose API keys to frontend
// Frontend JavaScript never receives or handles API keys
```

### **Environment Variable Security**

```bash
# ✅ Secure: Environment variable approach
export CLAUDE_API_KEY=sk-ant-your-api-key-here
npm run dev

# ✅ Secure: Command-line flag (for development)
npm run dev -- --key sk-ant-your-api-key-here

# ❌ Never: Hardcoded in configuration files
# config.json: { "api_key": "sk-ant-..." }  // DON'T DO THIS

# ❌ Never: Committed to version control
# .env: CLAUDE_API_KEY=sk-ant-...  // DON'T COMMIT THIS
```

### **Development Script Security**

```javascript
// scripts/dev.js - Secure API key handling
const apiKey = process.argv.includes('--key') 
    ? process.argv[process.argv.indexOf('--key') + 1]
    : process.env.CLAUDE_API_KEY;

if (!apiKey) {
    console.error('Error: CLAUDE_API_KEY not provided');
    process.exit(1);
}

// Pass via environment, never inject into code
const tauriProcess = spawn('cargo', ['tauri', 'dev'], {
    env: { ...process.env, CLAUDE_API_KEY: apiKey },
    stdio: 'inherit'
});
```

## Error Message Sanitization

### **Automatic Security Sanitization**

```rust
impl ErrorContext {
    /// Automatically sanitize all error output
    pub fn sanitize_error_message(message: &str) -> String {
        let mut sanitized = message.to_string();
        
        // API key pattern detection and redaction
        let api_key_patterns = [
            r"sk-ant-[a-zA-Z0-9-_]{20,}",      // Anthropic API keys
            r"sk-[a-zA-Z0-9-_]{20,}",          // Generic OpenAI-style keys
            r"Bearer [a-zA-Z0-9-_]{20,}",      // Bearer tokens
        ];
        
        for pattern in &api_key_patterns {
            let regex = Regex::new(pattern).unwrap();
            sanitized = regex.replace_all(&sanitized, "[API_KEY_REDACTED]").to_string();
        }
        
        // User directory path sanitization
        let sensitive_paths = [
            "/home/", "/Users/", "C:\\Users\\",     // User directories
            "/root/", "C:\\Windows\\",              // System directories
            "/.ssh/", "/.aws/", "/.config/",       // Config directories
        ];
        
        for sensitive_path in &sensitive_paths {
            if let Some(start) = sanitized.find(sensitive_path) {
                // Find end of path (space, quote, or end of string)
                let path_end = sanitized[start..].find(' ')
                    .or_else(|| sanitized[start..].find('"'))
                    .map(|i| start + i)
                    .unwrap_or(sanitized.len());
                
                sanitized.replace_range(start..path_end, "/[USER_DIR_REDACTED]");
            }
        }
        
        // Remove other sensitive patterns
        let sensitive_patterns = [
            (r"password=[^\s]+", "password=[REDACTED]"),
            (r"token=[^\s]+", "token=[REDACTED]"),
            (r"secret=[^\s]+", "secret=[REDACTED]"),
        ];
        
        for (pattern, replacement) in &sensitive_patterns {
            let regex = Regex::new(pattern).unwrap();
            sanitized = regex.replace_all(&sanitized, *replacement).to_string();
        }
        
        sanitized
    }
    
    /// Log error with automatic sanitization
    pub fn log_error(&self, error: &ClaudeError) {
        let safe_message = Self::sanitize_error_message(&error.to_string());
        eprintln!("[ERROR] {}: {}", self.operation, safe_message);
    }
}
```

### **Logging Security Patterns**

```rust
// ✅ Secure: Always use sanitized logging
impl ClaudeClient {
    async fn send_request(&self, message: &str) -> Result<String> {
        match self.make_api_request(message).await {
            Ok(response) => {
                info!("Request successful, response length: {}", response.len());
                Ok(response)
            }
            Err(e) => {
                // Automatic sanitization prevents API key exposure
                let context = ErrorContext::new("claude_api_request");
                context.log_error(&e);
                Err(e)
            }
        }
    }
}

// ❌ Dangerous: Direct error logging without sanitization
impl ClaudeClient {
    async fn send_request(&self, message: &str) -> Result<String> {
        match self.make_api_request(message).await {
            Err(e) => {
                // Could expose API keys or sensitive paths
                eprintln!("API request failed: {}", e);
                Err(e)
            }
        }
    }
}
```

## Network Security

### **HTTPS Enforcement**

```rust
impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(120_000))
            .user_agent("Claude-Desktop/1.0")
            // Enforce TLS 1.2+ and certificate validation
            .min_tls_version(tls::Version::TLS_1_2)
            .danger_accept_invalid_certs(false)  // Always validate certificates
            .build()
            .expect("Failed to create HTTP client");
            
        Self { client }
    }
}

// ✅ All API endpoints use HTTPS
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com/v1";
pub const CLAUDE_API_MESSAGES_ENDPOINT: &str = "/messages";

// ❌ Never use HTTP for sensitive communications
// const INSECURE_API: &str = "http://api.example.com";  // DON'T DO THIS
```

### **Request Security Headers**

```rust
impl ClaudeClient {
    async fn make_request(&self, payload: &RequestPayload) -> Result<String> {
        let response = self.client
            .post(&format!("{}{}", CLAUDE_API_BASE_URL, CLAUDE_API_MESSAGES_ENDPOINT))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "Claude-Desktop/1.0")
            .header("x-api-key", &self.api_key)  // Secure header transmission
            .json(payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            // Error logging with automatic sanitization
            let context = ErrorContext::new("claude_api_request")
                .with_metadata("status_code", &status.to_string());
            context.log_error(&ClaudeError::ApiError { status, body: error_body });
        }
        
        Ok(response.text().await?)
    }
}
```

## Input Validation Security

### **Tauri Command Security**

```rust
// ✅ Secure: Type-safe command parameters
#[tauri::command]
async fn read_file_secure(
    path: String, 
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    // 1. Validate against whitelist
    let whitelist = state.whitelist.read().await;
    whitelist.validate_access(Path::new(&path), FileOperation::Read)
        .map_err(|e| format!("Access denied: {}", e))?;
    
    // 2. Validate file size
    state.app_config.validation.validate_file_size(&path)
        .map_err(|e| format!("File too large: {}", e))?;
    
    // 3. Safe file reading
    fs::read_to_string(&path)
        .map_err(|e| format!("Read error: {}", e))
}

// ❌ Insecure: Direct file access without validation
#[tauri::command]
async fn read_file_insecure(path: String) -> Result<String, String> {
    // No validation - vulnerable to directory traversal
    fs::read_to_string(&path)
        .map_err(|e| e.to_string())
}
```

### **Message Content Validation**

```rust
impl ValidationLimits {
    pub fn validate_message_content(&self, message: &str) -> Result<()> {
        // Length validation
        self.validate_message_length(message.len())?;
        
        // Encoding validation
        if !message.is_ascii() && !self.is_valid_utf8(message) {
            return Err("Invalid character encoding".into());
        }
        
        // Content pattern validation
        for suspicious_pattern in SUSPICIOUS_PATTERNS {
            if message.contains(suspicious_pattern) {
                return Err(format!("Suspicious pattern detected: {}", suspicious_pattern));
            }
        }
        
        Ok(())
    }
}

pub const SUSPICIOUS_PATTERNS: &[&str] = &[
    "<script",           // XSS prevention
    "javascript:",       // Protocol injection
    "data:",            // Data URI attacks
    "vbscript:",        // VBScript injection
    "\x00",             // Null byte injection
];
```

## Security Monitoring and Event Logging

### **Security Event Types**

```rust
pub enum SecurityEvent {
    WhitelistViolation { path: PathBuf, operation: FileOperation },
    ApiKeyExposure { location: String },
    SuspiciousPattern { pattern: String, content_hash: String },
    PathTraversalAttempt { attempted_path: String },
    FileSizeViolation { path: PathBuf, size: u64 },
}

impl SecurityLogger {
    pub fn log_security_event(&self, event: SecurityEvent) {
        let sanitized_event = self.sanitize_security_event(event);
        
        match sanitized_event.severity {
            Severity::Critical => {
                eprintln!("[SECURITY CRITICAL] {}", sanitized_event.message);
                // Could trigger alerts to security team
            }
            Severity::Warning => {
                eprintln!("[SECURITY WARNING] {}", sanitized_event.message);
            }
            Severity::Info => {
                println!("[SECURITY INFO] {}", sanitized_event.message);
            }
        }
    }
}
```

### **Rate Limiting and Resource Protection**

```rust
impl SecurityLimits {
    pub fn check_rate_limits(&mut self, operation: &str) -> Result<()> {
        let now = Instant::now();
        let counter = self.rate_counters.entry(operation.to_string())
            .or_insert_with(|| RateCounter::new(Duration::from_secs(60)));
        
        if counter.check_rate(now) {
            Ok(())
        } else {
            Err(SecurityError::RateLimitExceeded(operation.to_string()))
        }
    }
}
```

## Security Best Practices

### **Development Guidelines**
1. **Never bypass security controls** - Always use whitelist validation
2. **Fail secure** - Default to denying access when in doubt
3. **Log security events** - But with automatic sanitization
4. **Validate all inputs** - Especially file paths and user content
5. **Minimize attack surface** - Use principle of least privilege

### **Deployment Security**
1. **Environment variables** for all secrets
2. **HTTPS everywhere** for network communications
3. **Certificate validation** enabled for all TLS connections
4. **Resource limits** configured appropriately
5. **Monitoring and alerting** for security events

### **Code Patterns**
```rust
// ✅ Always validate before accessing
pub fn secure_operation(path: &str, whitelist: &WhitelistConfig) -> Result<()> {
    whitelist.validate_access(Path::new(path), FileOperation::Read)?;
    // Safe to proceed
    Ok(())
}

// ✅ Always sanitize before logging
pub fn safe_error_log(error: &Error) {
    let sanitized = ErrorContext::sanitize_error_message(&error.to_string());
    eprintln!("Error: {}", sanitized);
}

// ✅ Use configuration for security boundaries
pub fn validate_input(data: &str, config: &AppConfig) -> Result<()> {
    config.validation.validate_message_length(data.len())?;
    // Additional validation...
    Ok(())
}
```

## Threat Model

### **Threats Mitigated**
- **Directory traversal attacks**: Path canonicalization and whitelist validation
- **API key exposure**: Complete frontend isolation and automatic sanitization
- **Information disclosure**: Comprehensive error message sanitization
- **Resource exhaustion**: File size limits and rate limiting
- **Configuration tampering**: Read-only configuration with validation

### **Security Boundaries**
- **File system access**: Whitelist-controlled with explicit permissions
- **Network communications**: HTTPS-only with certificate validation
- **Error reporting**: Automatic sanitization of sensitive information
- **Resource consumption**: Bounded by configuration limits

This security model provides enterprise-grade protection while maintaining usability and enabling legitimate development workflows.