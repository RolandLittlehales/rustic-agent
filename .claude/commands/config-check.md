# Config Check

Comprehensive configuration consistency validation across frontend and backend with focus on three-tier architecture compliance and synchronization.

## 🎯 Purpose

This command validates the **unified configuration system** to ensure consistency across all layers, proper separation of concerns, elimination of hardcoded values, and synchronization between frontend and backend configuration.

## ⚙️ Configuration Validation Areas

### **🏗️ Three-Tier Architecture Compliance**

#### **Layer 1: Compile-Time Constants**
```rust
// ✅ Validate: Proper compile-time constant usage
✅ API endpoints in constants.rs
✅ Security patterns properly defined
✅ Model definitions centralized
✅ No business logic constants in other layers
✅ Domain-based organization (not type-based)
```

#### **Layer 2: Runtime Configuration**
```rust
// ✅ Validate: Runtime config patterns
✅ Environment variable loading
✅ Config file hierarchy (env → file → defaults)
✅ Proper validation of runtime values
✅ No hardcoded deployment-specific values
✅ Secure API key handling
```

#### **Layer 3: Validation Limits**
```rust
// ✅ Validate: Security and resource boundaries
✅ Type-safe validation helpers
✅ Meaningful error messages
✅ Warning level calculations
✅ Cross-layer validation rules
✅ Bounded resource limits
```

### **🔄 Frontend-Backend Synchronization**

#### **Configuration Drift Detection**
```javascript
// ✅ Validate: Frontend config matches backend
✅ MESSAGE_MAX_CHARS synchronized
✅ FILE_MAX_SIZE_BYTES consistent
✅ Timeout values aligned
✅ Model IDs match across layers
✅ Warning level thresholds identical
```

#### **Build-Time Generation Validation**
```rust
// ✅ Validate: Automatic synchronization
✅ config-generated.js up to date
✅ Build script properly configured
✅ Frontend helpers match backend logic
✅ No manual duplication detected
```

### **🚫 Hardcoded Value Detection**

#### **Magic Number Elimination**
```rust
// ❌ Detect: Hardcoded values that should use config
if message.len() > 50000 { ... }           // Should use config
if timeout > 120000 { ... }                // Should use validation limits
if file_size > 10 * 1024 * 1024 { ... }   // Should use constants
```

#### **String Literal Analysis**
```rust
// ❌ Detect: Hardcoded strings that should be constants
let url = "https://api.anthropic.com/v1"; // Should use CLAUDE_API_BASE_URL
let model = "claude-4-sonnet-20250514";   // Should use model_ids constant
```

### **📝 Configuration Documentation**

#### **Domain-Based Organization**
```rust
// ✅ Validate: Proper constant organization
pub mod circuit_breaker {
    /// Circuit opens after 5 consecutive failures
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
}

// ❌ Anti-pattern: Type-based organization
pub mod numbers {
    pub const FAILURE_THRESHOLD: u32 = 5;
    pub const MAX_TOKENS: u32 = 8192;
}
```

## 💡 Usage

### **Basic Usage**
```
/config-check
```

**No arguments required** - performs comprehensive configuration validation.

### **Advanced Usage**
```bash
/config-check --fix          # Attempt automatic fixes
/config-check --verbose      # Detailed analysis output
/config-check --frontend     # Focus on frontend config only
/config-check --backend      # Focus on backend config only
```

### **Example Output (All Clear)**
```
⚙️ Running Configuration Validation...

🏗️ Three-Tier Architecture: ✅ EXCELLENT
✅ Compile-time constants properly separated
✅ Runtime configuration follows environment hierarchy
✅ Validation limits with type-safe helpers
✅ No layer mixing detected
✅ Domain-based organization maintained

🔄 Frontend-Backend Sync: ✅ SYNCHRONIZED
✅ MESSAGE_MAX_CHARS: 50000 (both layers)
✅ FILE_MAX_SIZE_BYTES: 10485760 (both layers)
✅ REQUEST_TIMEOUT_MS: 120000 (both layers)
✅ Model IDs consistent across layers
✅ Warning thresholds synchronized

🚫 Hardcoded Value Detection: ✅ CLEAN
✅ No magic numbers detected
✅ All string literals use constants
✅ Timeout values use configuration
✅ File size limits use validation system
✅ API endpoints use constants

📝 Configuration Documentation: ✅ EXCELLENT
✅ All constants documented with rationale
✅ Domain-based organization maintained
✅ Cross-references properly maintained
✅ Usage examples provided

🔧 Environment Integration: ✅ WORKING
✅ CLAUDE_API_KEY properly loaded
✅ Environment variable precedence correct
✅ Config file loading functional
✅ Default fallbacks appropriate

🎉 Configuration Status: ALL CHECKS PASSED ✅

⚙️ Configuration system is well-organized and consistent!
```

### **Example Output (Issues Found)**
```
⚙️ Running Configuration Validation...

🏗️ Three-Tier Architecture: ⚠️ ISSUES FOUND
✅ Compile-time constants properly separated
❌ Hardcoded value in runtime layer
   📍 src-tauri/src/config/runtime.rs:45
   💡 Timeout value 120000 should use validation limit
   🔧 Fix: Use self.validation.request_timeout_ms

🔄 Frontend-Backend Sync: ❌ DRIFT DETECTED
❌ MESSAGE_MAX_CHARS mismatch
   📍 Backend: 50000, Frontend: 45000
   📍 File: ui/js/config.js:12
   🔧 Fix: Run build script to regenerate frontend config

❌ Model ID inconsistency
   📍 Backend: "claude-4-sonnet-20250514"
   📍 Frontend: "claude-4-sonnet-20240514" (outdated)
   🔧 Fix: Update frontend config generation

🚫 Hardcoded Value Detection: ❌ VIOLATIONS FOUND
❌ Magic number detected
   📍 src-tauri/src/claude/client.rs:89
   💡 Hardcoded timeout: 30000
   🔧 Fix: Use app_config.validation.request_timeout_ms

❌ Hardcoded string literal
   📍 ui/js/app.js:156
   💡 Model name: "claude-4-sonnet-20250514"
   🔧 Fix: Use CONFIG.MODEL_IDS.CLAUDE_4_SONNET

📝 Configuration Documentation: ⚠️ MINOR ISSUES
⚠️ Missing documentation for new constant
   📍 src-tauri/src/config/constants.rs:67
   💡 NEW_FEATURE_TIMEOUT lacks business rationale
   🔧 Fix: Add documentation explaining timeout value

🚨 Configuration Status: ISSUES FOUND ❌

🔧 Priority Fixes Required:
1. HIGH: Fix frontend-backend sync drift
2. MEDIUM: Replace hardcoded values with config
3. LOW: Add missing constant documentation

Run with --fix flag to attempt automatic corrections.
```

## 🔍 Detailed Analysis Areas

### **Configuration Layer Analysis**
```rust
impl ConfigValidator {
    fn validate_layer_separation(&self) -> Vec<ConfigIssue> {
        let mut issues = Vec::new();
        
        // Check for compile-time constants in wrong layers
        if self.runtime_config.contains_compile_time_values() {
            issues.push(ConfigIssue::LayerViolation("Compile-time value in runtime layer"));
        }
        
        // Check for runtime values in constants
        if self.constants.contains_runtime_values() {
            issues.push(ConfigIssue::LayerViolation("Runtime value in constants layer"));
        }
        
        issues
    }
}
```

### **Synchronization Validation**
```rust
impl SyncValidator {
    fn validate_frontend_backend_sync(&self) -> Vec<SyncIssue> {
        let mut issues = Vec::new();
        
        // Compare critical values
        let backend_limits = &self.backend_config.validation;
        let frontend_config = &self.frontend_config;
        
        if backend_limits.message_max_chars != frontend_config.message_max_chars {
            issues.push(SyncIssue::ValueMismatch {
                key: "MESSAGE_MAX_CHARS",
                backend: backend_limits.message_max_chars,
                frontend: frontend_config.message_max_chars,
            });
        }
        
        issues
    }
}
```

### **Hardcoded Value Detection**
```rust
impl HardcodedDetector {
    fn scan_for_magic_numbers(&self) -> Vec<HardcodedIssue> {
        let mut issues = Vec::new();
        
        // Common magic numbers to detect
        let magic_patterns = &[
            ("50000", "MESSAGE_MAX_CHARS"),
            ("10485760", "FILE_MAX_SIZE_BYTES"),
            ("120000", "REQUEST_TIMEOUT_MS"),
        ];
        
        for (pattern, config_name) in magic_patterns {
            if let Some(location) = self.find_hardcoded_value(pattern) {
                issues.push(HardcodedIssue::MagicNumber {
                    location,
                    value: pattern.to_string(),
                    suggested_config: config_name.to_string(),
                });
            }
        }
        
        issues
    }
}
```

## 🔧 Automatic Fix Capabilities

### **Configuration Sync Fixes**
```bash
/config-check --fix
```

**Automatic fixes include**:
- Regenerate frontend configuration from backend constants
- Update outdated model IDs and endpoints
- Synchronize timeout and limit values
- Fix documentation formatting

### **Manual Fix Suggestions**
```
🔧 Suggested Fixes:

1. Replace hardcoded timeout:
   📍 src-tauri/src/claude/client.rs:89
   
   Replace:
   let timeout = Duration::from_millis(30000);
   
   With:
   let timeout = Duration::from_millis(app_config.validation.request_timeout_ms);

2. Use configuration constant:
   📍 ui/js/app.js:156
   
   Replace:
   const model = "claude-4-sonnet-20250514";
   
   With:
   const model = CONFIG.MODEL_IDS.CLAUDE_4_SONNET;
```

## 📊 Configuration Health Metrics

### **Compliance Scoring**
- **Layer Separation** (30%): Proper three-tier architecture
- **Synchronization** (25%): Frontend-backend consistency
- **Hardcoded Elimination** (25%): No magic numbers or literals
- **Documentation** (15%): Proper constant documentation
- **Environment Integration** (5%): Environment variable handling

### **Health Levels**
- **EXCELLENT** (9.0+): Best practices followed consistently
- **GOOD** (7.0-8.9): Minor issues or improvements needed
- **NEEDS WORK** (5.0-6.9): Several configuration issues
- **POOR** (<5.0): Major configuration problems

## 🔗 Integration with Development Workflow

### **Command Integration**
- **[`/start-feature`](./start-feature.md)** - Config validation during development
- **[`/review-pr`](./review-pr.md)** - Configuration compliance in review
- **[`/qa-check`](./qa-check.md)** - Basic config validation in QA

### **Development Integration**
```rust
// ✅ Config-check validates these patterns are followed
pub fn validate_message(message: &str, app_config: &AppConfig) -> Result<()> {
    app_config.validation.validate_message_length(message.len())
}

// ❌ Config-check detects these anti-patterns
pub fn validate_message(message: &str) -> Result<()> {
    if message.len() > 50000 { // Magic number!
        return Err("Message too long".into());
    }
    Ok(())
}
```

## 📋 Configuration Best Practices

### **Layer Guidelines**
1. **Constants**: Never change at runtime (API URLs, security patterns)
2. **Runtime**: Environment-configurable (timeouts, feature flags)  
3. **Validation**: Security boundaries (limits, resource protection)

### **Organization Principles**
- **Domain-based grouping** - Group by business domain, not data type
- **Documentation required** - Every constant needs business rationale
- **Single source of truth** - One authoritative definition per value
- **Type safety** - Leverage Rust's type system for validation

### **Synchronization Strategy**
- **Build-time generation** - Frontend config generated from Rust
- **Automated validation** - CI/CD integration with config-check
- **Regular audits** - Periodic configuration reviews

## ⚠️ Important Notes

### **Zero Tolerance for**
- **Configuration drift** between frontend and backend
- **Hardcoded values** that should use configuration system
- **Layer violations** mixing configuration concerns
- **Undocumented constants** without business rationale

### **Maintenance Requirements**
- **Run before commits** when modifying configuration
- **Include in CI/CD** pipeline for continuous validation
- **Regular reviews** of configuration organization
- **Update documentation** when adding new configuration

This command ensures the configuration system remains well-organized, consistent, and maintainable as the project evolves.