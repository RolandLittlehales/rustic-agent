# Review PR

Comprehensive code review enforcing all quality standards from `.claude/docs/` with focus on performance, maintainability, robustness, and readability.

## 🎯 Purpose

This command performs a **critical code review** that enforces all documented quality standards, architectural patterns, and best practices. It acts as a comprehensive quality gate ensuring code meets professional standards before merging.

## 🔍 Review Categories

### **🦀 Rust-Specific Quality**

#### **Ownership and Borrowing Patterns**
```rust
// ✅ Good: Use borrowing to avoid unnecessary cloning
pub fn validate_message(message: &str, config: &ValidationLimits) -> Result<()>

// ❌ Avoid: Unnecessary cloning
pub fn validate_message(message: String, config: ValidationLimits) -> Result<()>
```

#### **Idiomatic Constructs and Error Handling**
```rust
// ✅ Good: Use ? operator for error propagation
pub fn load_config() -> Result<AppConfig> {
    let mut config = AppConfig::default();
    config.runtime.load_from_env()?;
    Ok(config)
}

// ❌ Avoid: .unwrap() without proper checks
let config = load_config().unwrap(); // Can panic!
```

#### **Descriptive Naming and Constants**
```rust
// ✅ Good: Clear, descriptive names
pub const CLAUDE_API_MESSAGES_ENDPOINT: &str = "/messages";
pub const MESSAGE_MAX_CHARS: usize = 50000; // 50KB for coding helper

// ❌ Avoid: Abbreviated or magic numbers
pub const API_EP: &str = "/messages";
if message.len() > 50000 { ... }
```

### **🏗️ Architecture Compliance**

#### **Configuration-First Development**
```rust
// ✅ Good: Centralized configuration
state.app_config.validation.validate_message_length(message.len())?;

// ❌ Anti-pattern: Hardcoded values
if message.len() > 50000 { return Err("Too long".into()); }
```

#### **Three-Tier Config Architecture**
- **Layer 1**: Compile-time constants (API endpoints, security patterns)
- **Layer 2**: Runtime configuration (environment variables, timeouts)
- **Layer 3**: Validation limits (security boundaries, resource limits)

#### **Security Validation Patterns**
```rust
// ✅ Good: Let whitelist system handle validation
validate_path(path_str, &whitelist_guard, FileOperation::Read)?

// ❌ Anti-pattern: Overly restrictive early validation
if path_str.contains("..") { return Err("Invalid".into()); }
```

#### **Tauri v2 Parameter Discipline**
```javascript
// ✅ Good: Always use object parameters
await window.__TAURI__.core.invoke('send_message', { message: text });

// ❌ Wrong: Tauri v1 style
await window.__TAURI__.core.invoke('send_message', text);
```

### **🔒 Security Standards**

#### **API Key Handling**
- **No frontend exposure**: API keys never in HTML/JavaScript
- **Environment variables only**: Secure loading patterns
- **Automatic sanitization**: Error messages sanitized at logging boundary

#### **File System Security**
- **Whitelist validation**: All file operations validated
- **Path canonicalization**: Traversal attack prevention
- **Security boundaries**: Proper access control

#### **Error Message Sanitization**
```rust
// ✅ Good: Automatic sanitization
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

### **⚡ Performance & Maintainability**

#### **Efficient String Handling**
```rust
// ✅ Good: Use string slices when possible
pub fn validate_path(path: &str, whitelist: &WhitelistConfig) -> Result<PathBuf>

// ✅ Good: Use Cow for conditional cloning
use std::borrow::Cow;
pub fn normalize_path(path: &str) -> Cow<str>
```

#### **Resource Management**
```rust
// ✅ Good: Proper concurrent access patterns
pub struct AppState {
    config: Arc<Mutex<ClaudeConfig>>,          // Exclusive modifications
    app_config: Arc<AppConfig>,                // Immutable shared data
    whitelist: Arc<RwLock<WhitelistConfig>>,   // Concurrent read access
}
```

#### **Bounded Collections**
```rust
// ✅ Good: Bounded by design
pub struct BoundedErrorCounter {
    counts: HashMap<String, u64>,
    max_entries: usize,              // Prevents unbounded growth
    cleanup_interval: Duration,      // Automatic cleanup
}
```

### **🧪 Testing Quality**

#### **Property-Based vs Specific Testing**
```rust
// ✅ Good: Test properties and relationships
#[test]
fn test_exponential_backoff_properties() {
    let delays: Vec<_> = (0..3).map(|i| handler.calculate_delay(i, &error)).collect();
    assert!(delays[1] > delays[0] && delays[2] > delays[1]); // Monotonicity
}

// ❌ Avoid: Testing specific values only
#[test]
fn test_delay_calculation_attempt_0() {
    assert_eq!(handler.calculate_delay(0, &error), Duration::from_millis(100));
}
```

#### **Integration vs Unit Testing**
- **Integration tests** for real usage patterns
- **Comprehensive tests** covering full component behavior
- **Test fixtures** to reduce duplication
- **Data-driven approaches** for multiple scenarios

### **📁 Code Organization**

#### **Domain-Based Organization**
```rust
// ✅ Good: Domain-based constant organization
pub mod circuit_breaker {
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
    pub const DEFAULT_TIMEOUT_SECS: u64 = 60;
}

// ❌ Avoid: Type-based organization
pub mod numbers {
    pub const FAILURE_THRESHOLD: u32 = 5;
    pub const MAX_TOKENS: u32 = 8192;
}
```

#### **Function and Module Size**
- **Single responsibility** functions
- **Small, focused modules** (< 300 lines per file)
- **Clear module boundaries**
- **Proper error propagation**

## 💡 Usage

### **Basic Usage**
```
/review-pr $ARGUMENTS
```

### **Argument Formats**
```bash
/review-pr "#25"                    # PR number
/review-pr "feature/content-block"  # Branch name  
/review-pr "current changes"        # Current working changes
/review-pr                          # Review current uncommitted changes
```

### **Example Review Output**
```
🔍 Comprehensive Code Review: PR #25 - Implement error handling framework

📊 Review Summary:
- Files analyzed: 12
- Lines of code: 847 additions, 123 deletions
- Complexity score: Medium-High
- Security impact: High (API handling changes)

🦀 Rust-Specific Quality: ✅ EXCELLENT
✅ Ownership patterns - Proper borrowing throughout
✅ Error handling - Consistent ? operator usage  
✅ Naming conventions - Clear, descriptive names
✅ Constants - All magic numbers eliminated
✅ Idiomatic patterns - Follows Rust best practices

🏗️ Architecture Compliance: ⚠️ MINOR ISSUES
✅ Configuration-first - No hardcoded values detected
✅ Three-tier config - Proper layer separation
⚠️ Security validation - One instance of premature path validation
   📍 src-tauri/src/claude/tools.rs:45 - Let whitelist handle validation
✅ Tauri v2 patterns - Object parameters used correctly

🔒 Security Standards: ✅ EXCELLENT  
✅ API key handling - No frontend exposure
✅ File system security - Whitelist validation active
✅ Error sanitization - Automatic PII redaction
✅ Path validation - Proper canonicalization

⚡ Performance & Maintainability: ✅ EXCELLENT
✅ String handling - Efficient borrowing patterns
✅ Resource management - Proper Arc/RwLock usage
✅ Bounded collections - No unbounded growth risks
✅ Memory safety - No unsafe code, proper lifetimes

🧪 Testing Quality: ✅ EXCELLENT
✅ Property-based testing - Tests invariants and relationships
✅ Integration coverage - Real usage patterns tested
✅ Test organization - Comprehensive, well-structured
✅ Fixtures usage - Reduced duplication, reusable setup

📁 Code Organization: ✅ EXCELLENT
✅ Domain organization - Constants grouped by purpose
✅ Module boundaries - Clear responsibilities
✅ Function size - All functions < 50 lines
✅ Documentation - Comprehensive inline docs

🔄 Process Compliance: ✅ EXCELLENT
✅ TodoWrite usage - Complex tasks properly planned
✅ Incremental development - Clean commit history
✅ Quality gates - All checks passed
✅ Documentation - Updated with implementation

📈 Quality Score: 9.2/10

🎯 Readiness Assessment: READY FOR MERGE ✅

🔧 Minor Improvements Recommended:
1. src-tauri/src/claude/tools.rs:45 - Remove premature path validation
   Let the whitelist system handle path canonicalization and validation

💡 Excellent Work! This implementation demonstrates:
- Solid understanding of Rust best practices
- Proper architecture compliance
- Security-first design approach
- Comprehensive testing strategy
- Clean, maintainable code organization

✨ This PR maintains high code quality standards and is ready for merge.
```

### **Issues Found Example**
```
🔍 Comprehensive Code Review: PR #23 - Quick bug fix

📊 Review Summary:
- Files analyzed: 3
- Lines of code: 45 additions, 12 deletions
- Complexity score: Low
- Security impact: Low

🦀 Rust-Specific Quality: ❌ NEEDS WORK
❌ Error handling - Using .unwrap() without checks
   📍 src-tauri/src/config/runtime.rs:67
   🔧 Fix: Use ? operator or proper error handling
❌ Magic numbers - Hardcoded timeout value
   📍 src-tauri/src/client.rs:123
   🔧 Fix: Use constants from configuration system

🏗️ Architecture Compliance: ❌ MAJOR ISSUES
❌ Configuration-first - Hardcoded values detected
   📍 Multiple locations using magic numbers
❌ Security validation - Bypassing whitelist system
   📍 src-tauri/src/claude/tools.rs:34

📈 Quality Score: 4.2/10

🎯 Readiness Assessment: NEEDS MAJOR WORK ❌

🚨 Critical Issues (Must Fix):
1. Replace .unwrap() calls with proper error handling
2. Eliminate all magic numbers using configuration system
3. Remove whitelist bypass - use proper validation

🔧 Immediate Actions Required:
- Run /qa-check to catch compilation issues
- Review configuration system documentation
- Follow security patterns from existing code

❌ This PR cannot be merged until these issues are resolved.
```

## 🔄 Review Process

### **Automated Analysis**
1. **Static code analysis** of changes
2. **Pattern recognition** for anti-patterns
3. **Security scanning** for vulnerabilities
4. **Performance analysis** for bottlenecks
5. **Architecture compliance** checking

### **Quality Scoring**
- **Rust Quality** (25%): Idiomatic patterns, error handling, naming
- **Architecture** (25%): Configuration compliance, security patterns
- **Security** (20%): API handling, file system, sanitization
- **Performance** (15%): Memory management, resource usage
- **Testing** (10%): Coverage, quality, organization
- **Organization** (5%): Code structure, documentation

### **Readiness Levels**
- **READY FOR MERGE** (8.0+): Minor or no issues
- **NEEDS WORK** (6.0-7.9): Several issues to address
- **MAJOR ISSUES** (4.0-5.9): Significant problems
- **NEEDS COMPLETE REWORK** (<4.0): Fundamental issues

## 🔗 Integration with Standards

### **Documentation References**
- [**Rust Standards**](../.claude/docs/development/rust-standards.md) - Idiomatic patterns
- [**Architecture**](../.claude/docs/architecture/) - System design compliance
- [**Security Model**](../.claude/docs/architecture/security-model.md) - Security requirements
- [**Configuration**](../.claude/docs/architecture/configuration-system.md) - Config patterns
- [**Learnings**](../.claude/docs/learnings/) - Battle-tested insights

### **Command Integration**
- Runs after `/qa-check` passes
- Integrates with `/create-pr` workflow
- References `/security-check` and `/config-check`
- Informs `/test-review` recommendations

## ⚠️ Important Notes

- **Comprehensive analysis** - Reviews all aspects of code quality
- **Security focus** - Special attention to security implications
- **Educational feedback** - Explains issues and provides fixes
- **Standard enforcement** - Ensures consistency across codebase
- **Readiness assessment** - Clear go/no-go decision for merging

This command ensures all code contributions meet the high quality standards documented throughout the project and maintains consistency across the entire codebase.