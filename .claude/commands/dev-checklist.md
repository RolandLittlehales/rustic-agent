# Dev Checklist

Comprehensive development environment validation to ensure all tools, dependencies, and configurations are properly set up before starting development work.

## 🎯 Purpose

This command validates the **complete development environment** to ensure developers have all necessary tools, dependencies, and configurations properly set up for productive development. It provides clear feedback on any missing requirements and guidance on fixing issues.

## ✅ Environment Validation Areas

### **🦀 Rust Development Environment**

#### **Toolchain Validation**
```bash
✅ Rust toolchain version (1.75.0+)
✅ Cargo package manager
✅ rustc compiler functionality
✅ rustfmt code formatter
✅ clippy linter availability
✅ Rust edition compatibility (2021)
```

#### **Target Platform Support**
```bash
✅ Host target compilation
✅ Cross-compilation targets (if needed)
✅ Standard library components
✅ Core crate availability
```

### **🌐 Node.js and Frontend Environment**

#### **Node.js Environment**
```bash
✅ Node.js version (18.0+)
✅ npm package manager
✅ Package.json dependencies resolved
✅ Frontend build tools available
✅ JavaScript/TypeScript support
```

#### **Build System Validation**
```bash
✅ npm run build functionality
✅ Development server capability
✅ Asset processing pipeline
✅ Source map generation
```

### **⚡ Tauri Development Environment**

#### **Tauri CLI and Dependencies**
```bash
✅ Tauri CLI installation (2.0+)
✅ Platform-specific dependencies
  - Linux: webkit2gtk, libayatana-appindicator
  - macOS: Xcode command line tools
  - Windows: WebView2, Build Tools
✅ Native compilation capabilities
```

#### **Cross-Platform Requirements**
```bash
✅ System WebView availability
✅ Native UI framework support
✅ Platform permissions and capabilities
✅ Code signing setup (if needed)
```

### **🔑 API and Configuration**

#### **Claude API Setup**
```bash
✅ CLAUDE_API_KEY environment variable
✅ API key format validation (sk-ant-*)
✅ API connectivity test
✅ Rate limiting awareness
✅ Model access verification
```

#### **Configuration System**
```bash
✅ Environment variable loading
✅ Configuration file parsing
✅ Default value fallbacks
✅ Validation limit enforcement
✅ Three-tier config architecture
```

### **🔧 Development Tools**

#### **Version Control**
```bash
✅ Git installation and configuration
✅ Repository clone and access
✅ Branch management capability
✅ Remote repository connectivity
✅ Commit signing (if required)
```

#### **Code Quality Tools**
```bash
✅ Code formatting tools (cargo fmt)
✅ Linting tools (cargo clippy)
✅ Testing framework (cargo test)
✅ Documentation generation (cargo doc)
```

### **🏗️ Build and Compilation**

#### **Debug Build Validation**
```bash
✅ cargo build (debug mode)
✅ Dependency resolution
✅ Compilation without warnings
✅ Binary generation
✅ Asset bundling
```

#### **Release Build Readiness**
```bash
✅ cargo build --release capability
✅ Optimization settings
✅ Asset optimization
✅ Platform-specific builds
```

## 💡 Usage

### **Basic Usage**
```
/dev-checklist
```

**No arguments required** - performs comprehensive environment validation.

### **Advanced Usage**
```bash
/dev-checklist --quick        # Essential checks only
/dev-checklist --verbose      # Detailed diagnostic output
/dev-checklist --fix          # Attempt automatic fixes
/dev-checklist --setup        # Guide through setup process
```

### **Example Output (All Clear)**
```
🔧 Development Environment Checklist

🦀 Rust Development Environment: ✅ READY
✅ Rust toolchain: 1.75.0 (stable)
✅ Cargo: 1.75.0
✅ rustfmt: Available and functional
✅ clippy: Available and functional
✅ Rust edition: 2021 (compatible)

🌐 Node.js & Frontend Environment: ✅ READY
✅ Node.js: v20.11.0 (LTS)
✅ npm: 10.2.4
✅ package.json: All dependencies resolved
✅ Build tools: Working correctly
✅ Frontend pipeline: Functional

⚡ Tauri Development Environment: ✅ READY
✅ Tauri CLI: 2.0.5
✅ Platform dependencies: All present
✅ WebView2: Available (Windows)
✅ Native compilation: Functional
✅ Cross-platform support: Ready

🔑 API & Configuration: ✅ READY
✅ CLAUDE_API_KEY: Present and valid format
✅ API connectivity: Successful test connection
✅ Model access: claude-4-sonnet-20250514 available
✅ Configuration loading: Working correctly
✅ Environment variables: Properly configured

🔧 Development Tools: ✅ READY
✅ Git: 2.42.1 (configured)
✅ Repository access: Clone and push working
✅ Branch management: Functional
✅ Code quality tools: All available

🏗️ Build & Compilation: ✅ READY
✅ Debug build: cargo build successful (2.3s)
✅ Release build: cargo build --release capable
✅ Frontend build: npm run build successful (4.1s)
✅ Asset bundling: Working correctly
✅ No compilation warnings

🎉 Development Environment: FULLY READY ✅

🚀 Environment is optimally configured for development!
   Ready to use /pick-next-ticket or /pick-next-feature
```

### **Example Output (Issues Found)**
```
🔧 Development Environment Checklist

🦀 Rust Development Environment: ⚠️ ISSUES
✅ Rust toolchain: 1.75.0 (stable)
❌ clippy: Not installed
   🔧 Fix: rustup component add clippy
✅ rustfmt: Available
⚠️ Outdated cargo: 1.74.0 (recommend 1.75.0+)
   🔧 Fix: rustup update

🌐 Node.js & Frontend Environment: ❌ ISSUES
❌ Node.js: v16.14.0 (outdated, need 18.0+)
   🔧 Fix: Update Node.js to LTS version (20.11.0)
❌ Dependencies: 3 packages need updates
   🔧 Fix: npm audit fix && npm update

⚡ Tauri Development Environment: ❌ CRITICAL
❌ Tauri CLI: Not installed
   🔧 Fix: cargo install tauri-cli --version "^2.0"
❌ WebView2: Missing (Windows)
   🔧 Fix: Download from Microsoft WebView2 page

🔑 API & Configuration: ❌ BLOCKED
❌ CLAUDE_API_KEY: Not set
   🔧 Fix: Set environment variable or use --key flag
   💡 export CLAUDE_API_KEY=sk-ant-your-key-here

🚨 Development Status: SETUP REQUIRED ❌

🔧 Critical Setup Steps:
1. Install missing Tauri CLI and dependencies
2. Update Node.js to supported version
3. Configure Claude API key
4. Update Rust components

Run /dev-checklist --setup for guided setup process.
```

### **Guided Setup Mode**
```bash
/dev-checklist --setup
```

```
🛠️ Development Environment Setup Guide

Step 1/5: Rust Environment
Current: Rust 1.75.0 ✅
Action: Update clippy component
Command: rustup component add clippy

Step 2/5: Node.js Environment  
Current: Node.js 16.14.0 ❌
Action: Update to Node.js LTS (20.11.0)
Instructions:
  1. Visit https://nodejs.org/
  2. Download LTS version
  3. Run installer
  4. Verify: node --version

Step 3/5: Tauri CLI
Current: Not installed ❌
Action: Install Tauri CLI v2
Command: cargo install tauri-cli --version "^2.0"

Step 4/5: API Configuration
Current: No API key ❌
Action: Configure Claude API key
Options:
  - Environment: export CLAUDE_API_KEY=sk-ant-your-key
  - Runtime: npm run dev -- --key sk-ant-your-key

Step 5/5: Validation
Command: /dev-checklist
Expected: All checks pass ✅

🔧 Run each step and then execute /dev-checklist to verify setup.
```

## 🔍 Detailed Validation Process

### **Rust Environment Analysis**
```rust
impl RustEnvironmentValidator {
    fn validate_toolchain(&self) -> ValidationResult {
        // Check Rust version
        let rust_version = self.get_rust_version()?;
        if rust_version < Version::new(1, 75, 0) {
            return Err("Rust version too old, need 1.75.0+".into());
        }
        
        // Check essential components
        let components = ["rustfmt", "clippy"];
        for component in components {
            if !self.is_component_installed(component) {
                return Err(format!("{} component missing", component));
            }
        }
        
        Ok(ValidationSuccess::RustReady)
    }
}
```

### **Tauri Environment Validation**
```rust
impl TauriEnvironmentValidator {
    fn validate_platform_dependencies(&self) -> ValidationResult {
        match std::env::consts::OS {
            "linux" => self.validate_linux_deps(),
            "macos" => self.validate_macos_deps(), 
            "windows" => self.validate_windows_deps(),
            _ => Err("Unsupported platform".into()),
        }
    }
    
    fn validate_linux_deps(&self) -> ValidationResult {
        let required_packages = [
            "libwebkit2gtk-4.0-dev",
            "libayatana-appindicator3-dev",
        ];
        
        for package in required_packages {
            if !self.is_package_installed(package) {
                return Err(format!("Missing package: {}", package));
            }
        }
        
        Ok(ValidationSuccess::PlatformReady)
    }
}
```

### **API Connectivity Testing**
```rust
impl ApiValidator {
    async fn test_api_connectivity(&self) -> ValidationResult {
        let api_key = env::var("CLAUDE_API_KEY")
            .map_err(|_| "CLAUDE_API_KEY not set")?;
            
        if !api_key.starts_with("sk-ant-") {
            return Err("Invalid API key format".into());
        }
        
        // Test basic connectivity
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.anthropic.com/v1/models")
            .header("x-api-key", &api_key)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(ValidationSuccess::ApiReady)
        } else {
            Err(format!("API test failed: {}", response.status()))
        }
    }
}
```

## 🔧 Automatic Fix Capabilities

### **Component Installation**
```bash
/dev-checklist --fix
```

**Automatic fixes include**:
- Install missing Rust components (clippy, rustfmt)
- Update outdated packages with npm update
- Install compatible dependency versions
- Configure git settings if missing

### **Platform-Specific Setup**
```bash
# Linux package installation
sudo apt install libwebkit2gtk-4.0-dev libayatana-appindicator3-dev

# macOS developer tools
xcode-select --install

# Windows WebView2 download
# Provides download link and instructions
```

## 📊 Environment Health Scoring

### **Readiness Levels**
- **FULLY READY** (100%): All checks pass, optimal development environment
- **MOSTLY READY** (80-99%): Minor issues, can develop with workarounds
- **SETUP NEEDED** (50-79%): Several missing components, setup required
- **BLOCKED** (0-49%): Critical components missing, cannot develop

### **Component Weighting**
- **API Configuration** (30%): Essential for application functionality
- **Rust Environment** (25%): Core development platform
- **Tauri Environment** (20%): Desktop application framework
- **Build System** (15%): Compilation and asset processing
- **Development Tools** (10%): Code quality and version control

## 🔗 Integration with Development Workflow

### **Command Integration**
- **[`/start-feature`](./start-feature.md)** - Environment check before major work
- **[`/work-on-ticket`](./work-on-ticket.md)** - Quick environment validation
- **[`/qa-check`](./qa-check.md)** - Build environment verification

### **Onboarding Integration**
```bash
# New developer onboarding flow
/dev-checklist --setup    # Guided setup process
/dev-checklist           # Verify setup complete
/pick-next-ticket        # Start with simple work
```

## 📋 Platform-Specific Requirements

### **Linux Development**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libayatana-appindicator3-dev

# Fedora/RHEL
sudo dnf install webkit2gtk3-devel libappindicator-gtk3-devel
```

### **macOS Development**
```bash
# Xcode command line tools
xcode-select --install

# Homebrew (optional but recommended)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### **Windows Development**
```powershell
# WebView2 Runtime
# Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

# Visual Studio Build Tools (if needed)
# Download from: https://visualstudio.microsoft.com/downloads/
```

## ⚠️ Important Notes

### **First-Time Setup**
- **Run before any development** - Validates environment readiness
- **Platform-specific requirements** - Different setup for each OS
- **API key security** - Never commit API keys to repository
- **Tool compatibility** - Ensure version compatibility across tools

### **Regular Validation**
- **Run after system updates** - OS or tool updates may affect environment
- **Run after dependency changes** - New dependencies may require setup
- **Include in CI/CD** - Validate environment in automated pipelines
- **Team synchronization** - Ensure all team members have compatible environments

This command ensures developers have a consistent, fully-functional development environment before starting any coding work.