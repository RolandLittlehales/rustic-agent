# Configuration System Architecture

The application uses a unified three-tier configuration architecture designed to eliminate hardcoded values, provide deployment flexibility, and maintain type safety across the entire system.

## Design Principles

1. **Single Source of Truth**: All configuration values defined in one authoritative location
2. **Layer Separation**: Clear separation between compile-time, runtime, and validation concerns
3. **Type Safety**: Leverage Rust's type system to prevent configuration errors
4. **Environment Flexibility**: Support different values per deployment without code changes

## Three-Tier Architecture

### **Layer 1: Compile-Time Constants**
**Location**: `src-tauri/src/config/constants.rs`  
**Purpose**: Values that never change at runtime

```rust
// API endpoints and external service URLs
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com/v1";
pub const CLAUDE_API_MESSAGES_ENDPOINT: &str = "/messages";

// Security patterns and protected resources
pub const SUSPICIOUS_PATTERNS: &[&str] = &["<script", "javascript:", "data:"];
pub const PROTECTED_FILES: &[&str] = &[".env", ".secret", "id_rsa"];

// Model definitions organized by domain
pub mod model_ids {
    pub const CLAUDE_4_SONNET: &str = "claude-4-sonnet-20250514";
    pub const CLAUDE_3_5_SONNET: &str = "claude-3-5-sonnet-20241022";
    pub const CLAUDE_3_HAIKU: &str = "claude-3-haiku-20240307";
}
```

**Characteristics**:
- Compiled into the binary
- Cannot be changed without recompilation
- Used for fundamental system constants
- Organized by domain, not data type

### **Layer 2: Runtime Configuration**
**Location**: `src-tauri/src/config/runtime.rs`  
**Purpose**: Deployment-configurable values via environment variables and config files

```rust
pub struct RuntimeConfig {
    pub api_key: Option<String>,           // From CLAUDE_API_KEY env var
    pub http_timeout_ms: u64,              // Configurable per deployment
    pub model_selection: String,           // Default model to use
    pub feature_flags: HashMap<String, bool>, // Enable/disable features
    pub log_level: String,                 // Logging verbosity
}

impl RuntimeConfig {
    pub fn load_from_env(&mut self) -> Result<()> {
        // Environment variables take precedence
        if let Ok(key) = env::var("CLAUDE_API_KEY") {
            self.api_key = Some(key);
        }
        
        if let Ok(timeout) = env::var("HTTP_TIMEOUT_MS") {
            self.http_timeout_ms = timeout.parse().unwrap_or(self.http_timeout_ms);
        }
        
        // Load from config files if env vars not present
        self.load_from_config_file()
    }
}
```

**Loading Hierarchy**:
1. Environment variables (highest priority)
2. Config files (`.config/app.toml`)
3. Defaults (fallback values)

### **Layer 3: Validation Limits**
**Location**: `src-tauri/src/config/validation.rs`  
**Purpose**: Security and resource limits with built-in validation

```rust
pub struct ValidationLimits {
    pub message_max_chars: usize,          // Security/UX limit
    pub file_max_size_bytes: u64,         // Resource protection
    pub path_max_length: usize,           // Security boundary
    pub request_timeout_ms: u64,          // Performance boundary
}

impl ValidationLimits {
    /// Validate message length with helpful error messages
    pub fn validate_message_length(&self, length: usize) -> Result<()> {
        if length > self.message_max_chars {
            return Err(format!(
                "Message too long: {} chars (max: {})", 
                length, self.message_max_chars
            ));
        }
        Ok(())
    }
    
    /// Get warning level for UI feedback
    pub fn message_warning_level(&self, length: usize) -> MessageWarningLevel {
        let ratio = length as f64 / self.message_max_chars as f64;
        match ratio {
            r if r < 0.7 => MessageWarningLevel::Ok,
            r if r < 0.9 => MessageWarningLevel::Warning,
            _ => MessageWarningLevel::Danger,
        }
    }
}
```

## Configuration Integration

### **Master Configuration**
**Location**: `src-tauri/src/config/mod.rs`

```rust
pub struct AppConfig {
    pub constants: &'static Constants,     // Compile-time constants
    pub runtime: RuntimeConfig,            // Environment-configurable
    pub validation: ValidationLimits,      // Security boundaries
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut runtime = RuntimeConfig::default();
        runtime.load_from_env()?;
        
        let validation = ValidationLimits::default();
        
        let config = AppConfig {
            constants: &CONSTANTS,
            runtime,
            validation,
        };
        
        config.validate()?;
        Ok(config)
    }
    
    fn validate(&self) -> Result<()> {
        // Cross-layer validation
        if self.runtime.http_timeout_ms > self.validation.request_timeout_ms {
            return Err("HTTP timeout exceeds validation limit".into());
        }
        Ok(())
    }
}
```

### **Type-Safe Usage Patterns**

**Before (Anti-pattern)**:
```rust
// ❌ Hardcoded values scattered throughout code
if message.len() > 50000 {
    return Err("Message too long".into());
}

if timeout_ms > 120000 {
    return Err("Timeout too long".into());
}
```

**After (Best practice)**:
```rust
// ✅ Centralized configuration with type safety
app_config.validation.validate_message_length(message.len())?;

if timeout_ms > app_config.validation.request_timeout_ms {
    return Err(format!(
        "Timeout {} exceeds limit {}", 
        timeout_ms, 
        app_config.validation.request_timeout_ms
    ));
}
```

## Frontend-Backend Synchronization

### **Problem: Configuration Drift**
```javascript
// ❌ Problem: Frontend constants manually copied from backend
const MESSAGE_MAX_CHARS = 50000; // Hope this matches backend!
```

### **Solution: Build-Time Generation**
**File**: `src-tauri/build.rs`

```rust
fn generate_frontend_config() -> Result<()> {
    let config = format!(r#"
// Auto-generated from Rust configuration - DO NOT EDIT MANUALLY
export const CONFIG = {{
    MESSAGE_MAX_CHARS: {},
    FILE_MAX_SIZE_BYTES: {},
    REQUEST_TIMEOUT_MS: {},
    
    WARNING_LEVELS: {{
        OK: "text-gray-500",
        WARNING: "text-warning-500", 
        DANGER: "text-error-500",
    }},
    
    MODEL_IDS: {{
        CLAUDE_4_SONNET: "{}",
        CLAUDE_3_5_SONNET: "{}",
        CLAUDE_3_HAIKU: "{}",
    }}
}};

export const CONFIG_HELPERS = {{
    getMessageWarningLevel(length) {{
        const ratio = length / CONFIG.MESSAGE_MAX_CHARS;
        if (ratio < 0.7) return CONFIG.WARNING_LEVELS.OK;
        if (ratio < 0.9) return CONFIG.WARNING_LEVELS.WARNING;
        return CONFIG.WARNING_LEVELS.DANGER;
    }}
}};
"#, 
        defaults::MESSAGE_MAX_CHARS,
        defaults::FILE_MAX_SIZE_BYTES,
        defaults::REQUEST_TIMEOUT_MS,
        model_ids::CLAUDE_4_SONNET,
        model_ids::CLAUDE_3_5_SONNET,
        model_ids::CLAUDE_3_HAIKU,
    );
    
    fs::write("ui/js/config-generated.js", config)?;
    Ok(())
}
```

**Frontend Usage**:
```javascript
import { CONFIG, CONFIG_HELPERS } from './config-generated.js';

// Type-safe usage with automatic synchronization
const warningClass = CONFIG_HELPERS.getMessageWarningLevel(message.length);
const isValidSize = fileSize <= CONFIG.FILE_MAX_SIZE_BYTES;
```

## Configuration Organization Strategy

### **Domain-Based Organization**
```rust
// ✅ Good: Domain-based organization
pub mod circuit_breaker {
    /// Circuit opens after 5 consecutive failures to protect downstream systems
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
    /// How long circuit stays open before attempting recovery (60s)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 60;
}

pub mod model_costs {
    /// Cost per million input tokens (USD)
    pub const CLAUDE_4_OPUS_INPUT_COST: f64 = 15.0;
    /// Cost per million output tokens (USD)
    pub const CLAUDE_4_OPUS_OUTPUT_COST: f64 = 75.0;
}
```

**Anti-pattern to Avoid**:
```rust
// ❌ Bad: Type-based organization
pub mod strings {
    pub const CLAUDE_API_URL: &str = "...";
    pub const ERROR_MESSAGE: &str = "...";
}

pub mod numbers {
    pub const FAILURE_THRESHOLD: u32 = 5;
    pub const MAX_TOKENS: u32 = 8192;
}
```

### **Documentation Standards**
Each constant should include:

```rust
pub mod error_handling {
    /// Circuit breaker failure threshold
    /// 
    /// After this many consecutive failures, the circuit opens to protect
    /// downstream services. Based on industry standard of 5 failures.
    /// 
    /// **Impact**: Higher values increase tolerance but risk cascading failures.
    /// **Rationale**: Balances resilience with fault tolerance.
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
}
```

**Required Documentation**:
- **Business rationale** - Why this specific value
- **Usage context** - How and where it's used
- **Impact explanation** - What happens if changed
- **Cross-references** - Related constants and dependencies

## Security Considerations

### **Sensitive Configuration**
```rust
impl RuntimeConfig {
    /// Load API key securely from environment
    pub fn load_api_key(&mut self) -> Result<()> {
        match env::var("CLAUDE_API_KEY") {
            Ok(key) => {
                // Validate key format
                if !key.starts_with("sk-ant-") {
                    return Err("Invalid API key format".into());
                }
                self.api_key = Some(key);
                Ok(())
            }
            Err(_) => Err("CLAUDE_API_KEY environment variable not set".into())
        }
    }
    
    /// Never log API keys
    pub fn safe_debug(&self) -> String {
        format!("RuntimeConfig {{ api_key: {}, timeout: {}ms }}",
            if self.api_key.is_some() { "[PRESENT]" } else { "[MISSING]" },
            self.http_timeout_ms
        )
    }
}
```

### **Configuration Validation**
```rust
impl ValidationLimits {
    pub fn validate_all(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Security validations
        if self.message_max_chars > 100_000 {
            warnings.push("Message limit very high - security risk".to_string());
        }
        
        if self.file_max_size_bytes > 100 * 1024 * 1024 {
            warnings.push("File size limit very high - DoS risk".to_string());
        }
        
        // Performance validations
        if self.request_timeout_ms > 300_000 {
            warnings.push("Request timeout very high - UX impact".to_string());
        }
        
        Ok(warnings)
    }
}
```

## Best Practices

### **Layer Separation**
- **Never mix layers** - if a "constant" needs to be configurable, it belongs in Runtime layer
- **Clear boundaries** - each layer has distinct lifecycle and purpose
- **Validation relationships** - Runtime values must respect Validation limits

### **Environment Flexibility**
```bash
# Development
CLAUDE_API_KEY=sk-ant-dev-key npm run dev

# Production
CLAUDE_API_KEY=sk-ant-prod-key
HTTP_TIMEOUT_MS=30000
LOG_LEVEL=warn
npm run build
```

### **Type Safety**
```rust
// ✅ Good: Type-safe access with validation
pub fn get_timeout(&self) -> Duration {
    Duration::from_millis(self.runtime.http_timeout_ms)
}

// ❌ Avoid: Raw access without validation
pub fn get_timeout_raw(&self) -> u64 {
    self.runtime.http_timeout_ms
}
```

## Development Integration

### **Configuration Validation**
The configuration system integrates with the development workflow through:
- Automated configuration consistency checks
- Build-time frontend/backend synchronization
- Runtime validation and error reporting
- Environment-specific configuration validation

### **Usage in Application Code**
```rust
// Typical usage pattern throughout application
pub fn send_message(message: &str, app_config: &AppConfig) -> Result<()> {
    // Use validation layer for input checking
    app_config.validation.validate_message_length(message.len())?;
    
    // Use runtime config for operational parameters
    let timeout = Duration::from_millis(app_config.runtime.http_timeout_ms);
    
    // Use constants for API endpoints
    let url = format!("{}{}", 
        app_config.constants.CLAUDE_API_BASE_URL,
        app_config.constants.CLAUDE_API_MESSAGES_ENDPOINT
    );
    
    // Implementation continues...
    Ok(())
}
```

This configuration architecture ensures maintainable, secure, and flexible configuration management across the entire application lifecycle.