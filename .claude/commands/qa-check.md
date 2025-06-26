# QA Check

Comprehensive pre-commit quality assurance validation that runs all essential checks before code submission.

## 🎯 Purpose

This command automates the complete quality assurance checklist to ensure code meets all technical standards before commits and pull requests. It provides immediate feedback and prevents quality issues from reaching the repository.

## ✅ Quality Checks Performed

### **🔨 Build Validation**
```bash
cargo build && npm run build
```
- **Rust compilation** without warnings
- **Frontend build** succeeds  
- **Cross-platform compatibility** verified
- **Dependency resolution** validated

### **🎨 Code Formatting**
```bash
cargo fmt --check
```
- **Rust code formatting** matches project standards
- **Consistent style** across all files
- **No manual formatting** required

### **🔍 Code Quality (Linting)**
```bash
cargo clippy -- -D warnings
```
- **No clippy warnings** or suggestions
- **Idiomatic Rust patterns** enforced
- **Performance optimizations** suggested
- **Common mistakes** prevented

### **🧪 Test Execution**
```bash
cargo test
```
- **All unit tests** pass
- **Integration tests** succeed
- **No flaky tests** or timeouts
- **Test coverage** maintained

### **🌐 Frontend Validation**
- **JavaScript syntax** validation
- **No console errors** in browser
- **Build artifacts** generated correctly
- **Asset loading** verified

### **🔧 Application Functionality**
- **File explorer** loads and shows current directory
- **Tauri commands** use correct parameter format
- **Whitelist functionality** works (Ctrl+T test)
- **File watching** works without console spam

### **⚙️ Configuration Compliance**
- **Configuration system** used instead of hardcoded values
- **Frontend/backend** configuration constants synchronized
- **Environment variables** properly handled
- **Security settings** validated

## 💡 Usage

### **Basic Usage**
```
/qa-check
```

**No arguments required** - runs complete validation suite.

### **Example Output**
```
🔍 Running QA Checks...

🔨 Build Validation
✅ cargo build - Success (0 warnings)
✅ npm run build - Success 
✅ Dependencies resolved correctly

🎨 Code Formatting  
✅ cargo fmt - No changes needed
✅ All files properly formatted

🔍 Code Quality (Linting)
✅ cargo clippy - No warnings or suggestions
✅ Idiomatic Rust patterns confirmed

🧪 Test Execution
✅ cargo test - All 47 tests passed (3.2s)
✅ No flaky or failing tests

🌐 Frontend Validation
✅ JavaScript syntax valid
✅ No console errors detected
✅ Build artifacts generated

🔧 Application Functionality  
✅ File explorer loads current directory
✅ Tauri commands use object parameters
✅ Whitelist test (Ctrl+T) - Working
✅ File watching without console spam

⚙️ Configuration Compliance
✅ No hardcoded values detected
✅ Frontend/backend config synchronized
✅ Environment variables handled securely

🎉 QA Status: ALL CHECKS PASSED ✓

✨ Code is ready for commit and PR submission!
```

### **Failed Check Example**
```
🔍 Running QA Checks...

🔨 Build Validation
❌ cargo build - FAILED
   Error: missing field `api_key` in struct `RuntimeConfig`
   --> src-tauri/src/config/runtime.rs:45:5

🎨 Code Formatting
⚠️  cargo fmt - Changes needed
   Modified: src-tauri/src/claude/client.rs

❌ QA Status: FAILED (2 issues)

🔧 Fixes needed:
1. Fix compilation error in runtime.rs 
2. Run 'cargo fmt' to format code

Please address these issues and run /qa-check again.
```

## 🔄 Integration with Workflows

### **Automatic Integration**
- **[`/work-on-ticket`](./work-on-ticket.md)** - Runs before completion
- **[`/start-feature`](./start-feature.md)** - Multiple validation points
- **[`/create-pr`](./create-pr.md)** - Must pass before PR creation

### **Manual Usage**
- **Before commits** - Validate changes before git commit
- **During development** - Periodic validation while coding
- **After dependencies** - Verify after updating dependencies
- **Pre-deployment** - Final validation before releases

## ⚡ Performance Optimizations

### **Incremental Checks**
- **Cached builds** when possible
- **Parallel execution** of independent checks
- **Fast feedback** on common issues
- **Early exit** on critical failures

### **Selective Validation**
```bash
/qa-check --build-only      # Just build validation
/qa-check --lint-only       # Just formatting and linting  
/qa-check --tests-only      # Just test execution
/qa-check --frontend-only   # Just frontend validation
```

## 📊 Quality Metrics

### **Success Criteria**
- **100% pass rate** on all checks
- **Zero warnings** in build output
- **All tests passing** consistently
- **Clean code formatting** throughout

### **Performance Targets**
- **Build time** < 2 minutes for full validation
- **Test execution** < 5 minutes for full suite
- **Immediate feedback** on formatting issues
- **Clear error messages** for quick fixes

## 🛠️ Troubleshooting

### **Common Issues**

#### **Build Failures**
```bash
# Check for missing dependencies
cargo check

# Clean build cache if corrupted
cargo clean && cargo build

# Verify Rust toolchain version
rustc --version
```

#### **Formatting Issues**
```bash
# Auto-fix formatting
cargo fmt

# Check what would be formatted
cargo fmt -- --check
```

#### **Clippy Warnings**
```bash
# Get detailed clippy output
cargo clippy -- -D warnings

# Fix individual suggestions
cargo clippy --fix
```

#### **Test Failures**
```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Update test snapshots if needed
cargo test -- --update-snapshots
```

## 🔒 Security Validations

### **API Key Security**
- **No API keys** in frontend code
- **Environment variable** usage validated
- **Secure logging** patterns enforced

### **File System Security**
- **Whitelist validation** working correctly
- **Path traversal** protection active
- **File permissions** properly configured

### **Error Handling Security**
- **Error sanitization** active
- **No sensitive data** in error messages
- **Proper logging** levels maintained

## 📋 Configuration

### **Custom Quality Gates**
The QA check can be configured via environment variables:

```bash
# Skip specific checks (not recommended)
SKIP_FRONTEND_VALIDATION=true /qa-check

# Adjust timeout for slow systems
TEST_TIMEOUT_SECONDS=300 /qa-check

# Enable additional debug output
QA_VERBOSE=true /qa-check
```

### **Project-Specific Checks**
- **Custom lint rules** for project standards
- **Additional test categories** as needed
- **Performance benchmarks** for critical paths
- **Documentation validation** for completeness

## 🔗 Related Commands

- **[`/dev-checklist`](./dev-checklist.md)** - Environment setup validation
- **[`/security-check`](./security-check.md)** - Security-specific validation
- **[`/config-check`](./config-check.md)** - Configuration consistency
- **[`/review-pr`](./review-pr.md)** - Comprehensive code review

## ⚠️ Important Notes

- **Never skip QA checks** before commits or PRs
- **Address all issues** before proceeding
- **Use early and often** during development
- **Essential for CI/CD** pipeline integration

This command ensures consistent quality standards across all code contributions and prevents common issues from reaching production.