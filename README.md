# Rustic Agent

A powerful development agent tool built with **Rust + Tauri** that integrates with **Claude AI** to provide intelligent coding assistance.

## 🚀 Features

### Core Capabilities
- **Claude AI Integration**: Direct API connection for intelligent responses
- **Tool System**: Extensible tool registry for file operations, git commands, etc.
- **Modern UI**: Clean, responsive interface with chat and file explorer
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux
- **Developer Tools**: Built-in file operations (read/write/list directories)

### Architecture
- **Backend**: Rust with Tauri 2.0 for native performance and security
- **Frontend**: Modern web UI with vanilla JavaScript
- **API**: Direct Claude API integration with conversation management
- **State Management**: Thread-safe async state with tokio
- **Error Handling**: Robust error handling throughout

## 🛠 Setup & Installation

### Prerequisites for Senior Devs New to Rust

#### 1. Install Rust (5 minutes)
```bash
# Install Rust toolchain (similar to installing Node.js)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell environment (like sourcing .bashrc)
source $HOME/.cargo/env

# Verify installation (should show rustc 1.80+)
rustc --version
cargo --version
```

#### 2. Install Node.js (if not already installed)
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install nodejs npm

# macOS
brew install node

# Windows
# Download from nodejs.org
```

#### 3. System Dependencies (Linux only)
```bash
# Ubuntu/Debian - Required for webkit2gtk (Tauri's webview)
sudo apt update
sudo apt install -y pkg-config libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev

# Fedora/RHEL
sudo dnf install pkg-config openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel webkit2gtk4.1-devel

# Arch Linux
sudo pacman -S pkg-config openssl gtk3 libappindicator-gtk3 librsvg webkit2gtk-4.1
```

### 🚀 Quick Start (3 steps)

#### 1. Install Dependencies
```bash
npm install
```

#### 2. Get Claude API Key
1. Go to [Anthropic Console](https://console.anthropic.com)
2. Create an API key
3. Copy the key (starts with `sk-ant-api03-...`)

#### 3. Run with API Key
```bash
# Option 1: Pass key via command line
npm run dev -- --key YOUR_API_KEY_HERE

# Option 2: Use environment variable
CLAUDE_API_KEY=your_key npm run dev

# Option 3: Create .env file (recommended)
echo "CLAUDE_API_KEY=your_key_here" > .env
npm run dev
```

**That's it!** The first run will:
- Download ~200 Rust dependencies (like node_modules but cached globally)
- Compile the Rust backend (~2-3 minutes first time)
- Launch the desktop application with full Claude integration

**⚠️ Important**: 
- For full functionality, always use `npm run dev` (not opening HTML in browser)
- API key is required - the app will show an error without it
- WSL users: See WSL section below for GUI limitations

### Alternative: Manual Rust Build
```bash
# Build release version (optimized, slower compile)
cd src-tauri
cargo build --release

# Run the compiled binary
./target/release/llm-dev-agent

# Or build + run in one command (debug mode, faster compile)
cargo run
```

### 🔧 Development Commands

```bash
# Development mode (auto-reload on file changes)
npm run dev

# Build production bundle
npm run build

# Run tests (when implemented)
cargo test

# Check code formatting (Rust's prettier equivalent)
cargo fmt

# Run linter (Rust's ESLint equivalent) 
cargo clippy

# Clean build cache (like rm -rf node_modules)
cargo clean
```

### 📦 Understanding the Build Process

**For Senior Devs Coming from Other Languages:**

| Concept | JavaScript/Node.js | Rust/Cargo |
|---------|-------------------|-------------|
| Package Manager | `npm` | `cargo` |
| Dependencies File | `package.json` | `Cargo.toml` |
| Lock File | `package-lock.json` | `Cargo.lock` |
| Install Deps | `npm install` | `cargo build` (auto-installs) |
| Run Script | `npm run start` | `cargo run` |
| Build Production | `npm run build` | `cargo build --release` |
| Dependency Cache | `node_modules/` | `~/.cargo/registry/` |

**Key Differences:**
- **No separate install step**: `cargo build` automatically downloads dependencies
- **Global caching**: Dependencies cached globally, not per-project
- **Compilation required**: Rust compiles to native binary (no interpretation)
- **First build slow**: ~2-3 minutes, subsequent builds ~10-30 seconds

### 🚨 Troubleshooting

#### "cargo: command not found"
```bash
# Reload shell environment
source $HOME/.cargo/env

# Or restart terminal
```

#### Build fails with "webkit2gtk not found"
```bash
# Install system dependencies (see Prerequisites section above)
sudo apt install libwebkit2gtk-4.1-dev  # Ubuntu/Debian
```

#### "failed to run custom build command"
```bash
# Clean cache and rebuild
cargo clean
cargo build
```

#### "Cannot read properties of undefined (reading 'core')"
```bash
# This means Tauri API isn't available - you're probably running in browser mode
# Solution: Always use npm run dev, not opening index.html directly

# Correct way:
npm run dev

# Incorrect way (will show fallback):
# Opening ui/index.html in browser
```

#### Port 5173 already in use
```bash
# Kill existing dev server
pkill -f "tauri dev"
# Or change port in tauri.conf.json
```

#### Claude API works but no file operations
```bash
# File operations require Tauri backend - ensure you're using:
npm run dev
# Not browser mode. Browser shows "[Direct API]" prefix for responses.
```

### 🐧 WSL (Windows Subsystem for Linux) Issues

**Problem**: Tauri GUI apps can't run properly in WSL

**Solutions**:
1. **Move to Native Windows** (Recommended)
   - Install Rust: https://rustup.rs
   - Install Node.js: https://nodejs.org
   - Copy project to Windows filesystem
   - Run normally

2. **Use X11 Forwarding** (Advanced)
   ```bash
   # Install VcXsrv or similar X server
   export DISPLAY=:0
   npm run dev
   ```

3. **Development Mode** (Backend only)
   ```bash
   # Test backend without GUI
   cd src-tauri
   cargo run --release
   ```

## 🔑 API Configuration

### Getting Your Claude API Key

1. **Visit**: [Anthropic Console](https://console.anthropic.com)
2. **Sign in** with your Anthropic account
3. **Navigate to**: API Keys section
4. **Create** a new API key
5. **Copy** the key (format: `sk-ant-api03-...`)

### Setting the API Key

**Option 1: Environment Variable (Recommended)**
```bash
# Create .env file
echo "CLAUDE_API_KEY=sk-ant-api03-your-key-here" > .env
npm run dev
```

**Option 2: Command Line**
```bash
npm run dev -- --key sk-ant-api03-your-key-here
```

**Option 3: Shell Environment**
```bash
export CLAUDE_API_KEY=sk-ant-api03-your-key-here
npm run dev
```

### 💰 Billing Note
- API usage is **separate** from Claude.ai subscriptions
- Pay-per-use model (very affordable for development)
- Typical cost: ~$0.01-0.10 per conversation

## 📁 Project Structure

```
llm-dev-agent/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── claude/      # Claude API integration
│   │   │   ├── client.rs    # API client
│   │   │   ├── types.rs     # Type definitions
│   │   │   └── tools.rs     # Tool registry
│   │   └── main.rs      # Tauri application entry
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── ui/                 # Frontend interface
│   ├── index.html     # Main application UI
│   ├── css/styles.css # Modern styling
│   └── js/app.js      # Application logic
└── package.json       # Project metadata
```

## 🔧 Development

### Key Components

#### Claude Client (`src-tauri/src/claude/client.rs`)
- Handles API communication with Claude
- Manages conversation state
- Executes tools and processes responses

#### Tool Registry (`src-tauri/src/claude/tools.rs`)
- Extensible system for adding new capabilities
- Built-in tools: file operations, directory listing
- Ready for git operations, code analysis, etc.

#### Frontend Interface (`ui/js/app.js`)
- Chat interface with message history
- File explorer integration
- Real-time API communication via Tauri

### Adding New Tools

```rust
// Example: Adding a new tool
#[derive(Debug)]
pub struct GitStatusTool;

impl AgentTool for GitStatusTool {
    fn name(&self) -> &str { "git_status" }
    fn description(&self) -> &str { "Get git repository status" }
    
    fn execute(&self, input: Value) -> Result<String> {
        // Tool implementation
    }
}

// Register in client.rs
tool_registry.register(GitStatusTool);
```

## 🎯 Usage

1. **Start the Application**: Run with `npm run dev` or `cargo run`
2. **Chat Interface**: Type messages to interact with Claude AI
3. **File Operations**: Claude can read, write, and list files through tools
4. **Development Tasks**: Ask for code help, analysis, debugging assistance

### Example Interactions
- "Read the contents of src/main.rs"
- "List all files in the current directory"  
- "Help me debug this code snippet"
- "Create a new function for user authentication"

### 🎬 Quick Demo

Once running, try these commands in the chat:

```
1. "Hello! Can you help me with my Rust project?"
   → Tests basic Claude AI integration

2. "List the files in the current directory"
   → Demonstrates tool execution (list_directory)

3. "Read the file src-tauri/src/main.rs"
   → Shows file reading capability

4. "Explain what this application does"
   → Tests Claude's understanding and context
```

**Expected Result**: Claude will respond naturally and execute file operations as requested, demonstrating the full agent capabilities.

## 🚧 Roadmap

- [ ] Git integration tools
- [ ] Code search and analysis
- [ ] Multi-agent coordination (inspired by claude-squad)
- [ ] Project-specific context management
- [ ] Plugin system for custom tools
- [ ] Enhanced markdown rendering

## 📊 Performance

- **Build Time**: ~2 minutes for full release build
- **Binary Size**: ~50MB for native executable
- **Memory Usage**: ~50-100MB runtime
- **API Latency**: Depends on Claude API response times

## 🤝 Contributing

This is a demonstration project showcasing modern Rust + Tauri architecture with AI integration. The codebase follows best practices:

- **KISS**: Simple, focused implementations
- **DRY**: Reusable components and patterns
- **Error Handling**: Comprehensive error management
- **Type Safety**: Full Rust type system benefits
- **Performance**: Native compilation and async patterns

## 📄 License

Built for educational and development purposes. Ensure you comply with Anthropic's API terms of service.

---

**Total Development Time**: ~2 hours from conception to working application
**Lines of Code**: ~1,500 lines of well-structured Rust + frontend code
**Architecture**: Production-ready foundation for LLM agent development