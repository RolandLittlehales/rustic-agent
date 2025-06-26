# Configuration Templates for Claude 4 Integration

## Application Configuration (config.toml)

```toml
[anthropic]
# Model configuration - Claude 4 as default
model = "claude-4-sonnet-20250514"
api_version = "2023-06-01"
base_url = "https://api.anthropic.com"
timeout_seconds = 120
max_retries = 3

# Claude 4 enhanced parameters
max_tokens = 8192  # Increased for Claude 4 capabilities
temperature = 0.7
top_p = 0.9
top_k = 40

# Advanced Claude 4 features
enable_reasoning_tokens = true  # New in Claude 4
max_reasoning_tokens = 4096
enable_analysis_mode = true
performance_mode = "balanced"  # balanced, speed, quality

# Streaming configuration
streaming = true
stream_buffer_size = 16384  # Increased for Claude 4

[tools]
# Core file system tools
read_file = { enabled = true, max_file_size = "10MB" }
write_file = { enabled = true, backup_on_write = true }
list_directory = { enabled = true, max_depth = 10 }

# Advanced Anthropic tools
[tools.computer_use]
enabled = false  # Requires explicit opt-in
version = "computer_20250124"
screenshot_format = "png"
max_resolution = "1920x1080"
interaction_delay_ms = 100

[tools.text_editor]
enabled = true
version = "text_editor_20250429"
max_file_size = "1MB"
undo_history_size = 100
auto_format = true

[tools.code_execution]
enabled = false  # Requires explicit opt-in
version = "code_execution_20250522"
sandbox_mode = "strict"
timeout_seconds = 30
max_memory_mb = 256

[security]
# API Key Management (Issue 1.5 - Gap 9)
api_key_source = "environment"  # environment, file, memory
api_key_rotation_enabled = true
api_key_validation_on_startup = true
secure_key_storage = true

# Enhanced Whitelist configuration
whitelist_enabled = true
auto_add_project_root = true
max_path_depth = 20
validate_symlinks = true
strict_path_validation = true
prevent_directory_traversal = true

# File access restrictions
allowed_extensions = [".txt", ".md", ".rs", ".js", ".py", ".json", ".toml", ".yaml", ".yml", ".ts", ".tsx"]
blocked_extensions = [".exe", ".dll", ".so", ".dylib", ".bat", ".cmd", ".sh"]
max_file_size = "10MB"
quarantine_suspicious_files = true

# Network restrictions
allow_localhost = false
allow_network_tools = false
block_sensitive_paths = true
enable_sandbox_mode = true

# Enhanced security features
rate_limiting_enabled = true
request_throttling_ms = 100
audit_logging_detailed = true
security_headers_enforced = true

[ui]
# Interface configuration
theme = "dark"
font_size = 14
show_token_usage = true
auto_scroll = true

# Streaming UI
stream_typing_delay_ms = 50
show_tool_execution = true
highlight_code_blocks = true

[logging]
level = "info"
log_api_requests = true
mask_api_keys = true
log_tool_execution = true
max_log_size = "100MB"

[performance]
# HTTP client settings
connection_pool_size = 10
keep_alive_timeout = 30
tcp_nodelay = true

# Caching
cache_responses = true
cache_ttl_seconds = 300
max_cache_size = "50MB"

# Memory management
max_conversation_length = 1000
auto_compress_history = true
gc_interval_seconds = 600
```

## Environment Variables Template (.env)

```bash
# .env file - Enhanced API Security (Issue 1.5 - Gap 9)

# Primary API Keys (choose one method)
CLAUDE_API_KEY=your_claude_4_api_key_here
ANTHROPIC_API_KEY=your_claude_4_api_key_here  # Alternative name

# API Key Security Features
CLAUDE_API_KEY_ENCRYPTED=false  # Set to true for encrypted storage
CLAUDE_API_KEY_FILE_PATH=/secure/path/to/key.enc  # Alternative to direct key
API_KEY_ROTATION_INTERVAL_HOURS=168  # Weekly rotation
API_KEY_VALIDATION_STRICT=true

# Claude 4 Model Configuration (Issue 1.2 - Gap 13)
ANTHROPIC_MODEL=claude-4-sonnet-20250514
ANTHROPIC_BASE_URL=https://api.anthropic.com
ANTHROPIC_TIMEOUT=120
ANTHROPIC_MAX_TOKENS=8192
ANTHROPIC_ENABLE_REASONING=true
ANTHROPIC_PERFORMANCE_MODE=balanced

# Development and Security Settings
RUST_LOG=debug
TAURI_ENV=development
SECURITY_MODE=strict
AUDIT_LOGGING=true
SANDBOX_ENABLED=true

# Rate Limiting and Performance
REQUEST_RATE_LIMIT_PER_MINUTE=60
CONNECTION_POOL_SIZE=10
CACHE_ENABLED=true
CACHE_TTL_SECONDS=300

# Phase 4 Enhancement Package
FEATURE_FLAG_ADVANCED_TOOLS=true
FEATURE_FLAG_COMPUTER_USE=false
FEATURE_FLAG_CODE_EXECUTION=false
INTEGRATION_TESTING_MODE=false
```

## Secure .env.example Template

```bash
# .env.example - Template for new installations
# Copy to .env and configure with your actual values

# Required: Claude 4 API Key
CLAUDE_API_KEY=sk-ant-api03-your-key-here
# OR use alternative name
# ANTHROPIC_API_KEY=sk-ant-api03-your-key-here

# Optional: Model Configuration
# ANTHROPIC_MODEL=claude-4-sonnet-20250514
# ANTHROPIC_MAX_TOKENS=8192
# ANTHROPIC_ENABLE_REASONING=true

# Optional: Security Settings
# SECURITY_MODE=strict
# AUDIT_LOGGING=true
# SANDBOX_ENABLED=true

# Optional: Development Settings
# RUST_LOG=info
# TAURI_ENV=production
```

## Rust Configuration Structures

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub anthropic: AnthropicConfig,
    pub tools: ToolsConfig,
    pub security: SecurityConfig,
    pub ui: UiConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub model: String,
    pub api_version: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u8,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub streaming: bool,
    pub stream_buffer_size: usize,
    // Claude 4 specific features
    pub enable_reasoning_tokens: bool,
    pub max_reasoning_tokens: u32,
    pub enable_analysis_mode: bool,
    pub performance_mode: PerformanceMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceMode {
    Speed,
    Balanced,
    Quality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub read_file: FileToolConfig,
    pub write_file: WriteFileConfig,
    pub list_directory: DirectoryToolConfig,
    pub computer_use: Option<ComputerUseConfig>,
    pub text_editor: Option<TextEditorConfig>,
    pub code_execution: Option<CodeExecutionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileToolConfig {
    pub enabled: bool,
    pub max_file_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteFileConfig {
    pub enabled: bool,
    pub backup_on_write: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryToolConfig {
    pub enabled: bool,
    pub max_depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputerUseConfig {
    pub enabled: bool,
    pub version: String,
    pub screenshot_format: String,
    pub max_resolution: String,
    pub interaction_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEditorConfig {
    pub enabled: bool,
    pub version: String,
    pub max_file_size: String,
    pub undo_history_size: usize,
    pub auto_format: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionConfig {
    pub enabled: bool,
    pub version: String,
    pub sandbox_mode: String,
    pub timeout_seconds: u64,
    pub max_memory_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    // API Key Security (Issue 1.5 - Gap 9)
    pub api_key_source: ApiKeySource,
    pub api_key_rotation_enabled: bool,
    pub api_key_validation_on_startup: bool,
    pub secure_key_storage: bool,
    
    // Enhanced Whitelist configuration
    pub whitelist_enabled: bool,
    pub auto_add_project_root: bool,
    pub max_path_depth: u8,
    pub validate_symlinks: bool,
    pub strict_path_validation: bool,
    pub prevent_directory_traversal: bool,
    
    // File access restrictions
    pub allowed_extensions: Vec<String>,
    pub blocked_extensions: Vec<String>,
    pub max_file_size: String,
    pub quarantine_suspicious_files: bool,
    
    // Network restrictions
    pub allow_localhost: bool,
    pub allow_network_tools: bool,
    pub block_sensitive_paths: bool,
    pub enable_sandbox_mode: bool,
    
    // Enhanced security features
    pub rate_limiting_enabled: bool,
    pub request_throttling_ms: u64,
    pub audit_logging_detailed: bool,
    pub security_headers_enforced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiKeySource {
    Environment,
    File,
    Memory,
    Encrypted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub font_size: u8,
    pub show_token_usage: bool,
    pub auto_scroll: bool,
    pub stream_typing_delay_ms: u64,
    pub show_tool_execution: bool,
    pub highlight_code_blocks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_api_requests: bool,
    pub mask_api_keys: bool,
    pub log_tool_execution: bool,
    pub max_log_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub connection_pool_size: u8,
    pub keep_alive_timeout: u64,
    pub tcp_nodelay: bool,
    pub cache_responses: bool,
    pub cache_ttl_seconds: u64,
    pub max_cache_size: String,
    pub max_conversation_length: usize,
    pub auto_compress_history: bool,
    pub gc_interval_seconds: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            anthropic: AnthropicConfig::default(),
            tools: ToolsConfig::default(),
            security: SecurityConfig::default(),
            ui: UiConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            model: "claude-4-sonnet-20250514".to_string(),  // Claude 4 as default
            api_version: "2023-06-01".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            timeout_seconds: 120,
            max_retries: 3,
            max_tokens: 8192,  // Increased for Claude 4
            temperature: 0.7,
            top_p: None,
            top_k: None,
            streaming: true,
            stream_buffer_size: 16384,  // Increased for Claude 4
            enable_reasoning_tokens: true,
            max_reasoning_tokens: 4096,
            enable_analysis_mode: true,
            performance_mode: PerformanceMode::Balanced,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            // API Key Security defaults
            api_key_source: ApiKeySource::Environment,
            api_key_rotation_enabled: false,
            api_key_validation_on_startup: true,
            secure_key_storage: true,
            
            // Whitelist defaults
            whitelist_enabled: true,
            auto_add_project_root: true,
            max_path_depth: 20,
            validate_symlinks: true,
            strict_path_validation: true,
            prevent_directory_traversal: true,
            
            // File access defaults
            allowed_extensions: vec![
                ".txt".to_string(), ".md".to_string(), ".rs".to_string(),
                ".js".to_string(), ".py".to_string(), ".json".to_string(),
                ".toml".to_string(), ".yaml".to_string(), ".yml".to_string(),
                ".ts".to_string(), ".tsx".to_string()
            ],
            blocked_extensions: vec![
                ".exe".to_string(), ".dll".to_string(), ".so".to_string(),
                ".dylib".to_string(), ".bat".to_string(), ".cmd".to_string(),
                ".sh".to_string()
            ],
            max_file_size: "10MB".to_string(),
            quarantine_suspicious_files: true,
            
            // Network defaults
            allow_localhost: false,
            allow_network_tools: false,
            block_sensitive_paths: true,
            enable_sandbox_mode: true,
            
            // Security features defaults
            rate_limiting_enabled: true,
            request_throttling_ms: 100,
            audit_logging_detailed: true,
            security_headers_enforced: true,
        }
    }
}
```

## Configuration Loading Implementation

```rust
use std::path::Path;
use anyhow::Result;

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    pub fn load_from_env() -> Result<Self> {
        let mut config = AppConfig::default();
        
        // Claude 4 Model Configuration (Issue 1.2 - Gap 13)
        if let Ok(model) = std::env::var("ANTHROPIC_MODEL") {
            config.anthropic.model = model;
        }
        
        if let Ok(base_url) = std::env::var("ANTHROPIC_BASE_URL") {
            config.anthropic.base_url = base_url;
        }
        
        if let Ok(timeout) = std::env::var("ANTHROPIC_TIMEOUT") {
            config.anthropic.timeout_seconds = timeout.parse()?;
        }
        
        if let Ok(max_tokens) = std::env::var("ANTHROPIC_MAX_TOKENS") {
            config.anthropic.max_tokens = max_tokens.parse()?;
        }
        
        if let Ok(enable_reasoning) = std::env::var("ANTHROPIC_ENABLE_REASONING") {
            config.anthropic.enable_reasoning_tokens = enable_reasoning.parse().unwrap_or(true);
        }
        
        if let Ok(performance_mode) = std::env::var("ANTHROPIC_PERFORMANCE_MODE") {
            config.anthropic.performance_mode = match performance_mode.as_str() {
                "speed" => PerformanceMode::Speed,
                "quality" => PerformanceMode::Quality,
                _ => PerformanceMode::Balanced,
            };
        }
        
        // API Security Configuration (Issue 1.5 - Gap 9)
        if let Ok(security_mode) = std::env::var("SECURITY_MODE") {
            if security_mode == "strict" {
                config.security.strict_path_validation = true;
                config.security.prevent_directory_traversal = true;
                config.security.enable_sandbox_mode = true;
                config.security.rate_limiting_enabled = true;
            }
        }
        
        if let Ok(audit_logging) = std::env::var("AUDIT_LOGGING") {
            config.security.audit_logging_detailed = audit_logging.parse().unwrap_or(true);
        }
        
        if let Ok(sandbox_enabled) = std::env::var("SANDBOX_ENABLED") {
            config.security.enable_sandbox_mode = sandbox_enabled.parse().unwrap_or(true);
        }
        
        Ok(config)
    }
    
    pub fn merge_with_env(mut self) -> Result<Self> {
        let env_config = Self::load_from_env()?;
        
        // Merge configurations (env takes precedence)
        if env_config.anthropic.model != self.anthropic.model {
            self.anthropic.model = env_config.anthropic.model;
        }
        
        if env_config.anthropic.max_tokens != self.anthropic.max_tokens {
            self.anthropic.max_tokens = env_config.anthropic.max_tokens;
        }
        
        if env_config.anthropic.enable_reasoning_tokens != self.anthropic.enable_reasoning_tokens {
            self.anthropic.enable_reasoning_tokens = env_config.anthropic.enable_reasoning_tokens;
        }
        
        // Merge security settings
        if env_config.security.strict_path_validation != self.security.strict_path_validation {
            self.security.strict_path_validation = env_config.security.strict_path_validation;
        }
        
        Ok(self)
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate Claude 4 model name
        if !self.anthropic.model.starts_with("claude-") {
            return Err(anyhow::anyhow!("Invalid model name: {}", self.anthropic.model));
        }
        
        // Validate Claude 4 specific constraints
        if self.anthropic.model.starts_with("claude-4") {
            if self.anthropic.max_tokens > 200000 {
                return Err(anyhow::anyhow!("Claude 4 max_tokens cannot exceed 200,000"));
            }
            
            if self.anthropic.enable_reasoning_tokens && self.anthropic.max_reasoning_tokens > 20000 {
                return Err(anyhow::anyhow!("Claude 4 max_reasoning_tokens cannot exceed 20,000"));
            }
        }
        
        // Validate timeout
        if self.anthropic.timeout_seconds < 10 || self.anthropic.timeout_seconds > 600 {
            return Err(anyhow::anyhow!("Timeout must be between 10 and 600 seconds"));
        }
        
        // Validate temperature
        if self.anthropic.temperature < 0.0 || self.anthropic.temperature > 1.0 {
            return Err(anyhow::anyhow!("Temperature must be between 0.0 and 1.0"));
        }
        
        // Validate security settings
        if self.security.max_path_depth > 50 {
            return Err(anyhow::anyhow!("Path depth cannot exceed 50 levels"));
        }
        
        if self.security.request_throttling_ms > 10000 {
            return Err(anyhow::anyhow!("Request throttling cannot exceed 10 seconds"));
        }
        
        Ok(())
    }
    
    pub fn get_api_key(&self) -> Result<String> {
        match self.security.api_key_source {
            ApiKeySource::Environment => {
                std::env::var("CLAUDE_API_KEY")
                    .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
                    .map_err(|_| anyhow::anyhow!("No API key found in environment"))
            },
            ApiKeySource::File => {
                if let Ok(key_file) = std::env::var("CLAUDE_API_KEY_FILE_PATH") {
                    std::fs::read_to_string(key_file)
                        .map_err(|e| anyhow::anyhow!("Failed to read API key file: {}", e))
                } else {
                    Err(anyhow::anyhow!("API key file path not specified"))
                }
            },
            ApiKeySource::Memory => {
                Err(anyhow::anyhow!("Memory-based API keys not yet implemented"))
            },
            ApiKeySource::Encrypted => {
                Err(anyhow::anyhow!("Encrypted API keys not yet implemented"))
            }
        }
    }
    
    pub fn validate_api_key(&self, api_key: &str) -> Result<()> {
        if !api_key.starts_with("sk-ant-") {
            return Err(anyhow::anyhow!("Invalid API key format"));
        }
        
        if api_key.len() < 20 {
            return Err(anyhow::anyhow!("API key too short"));
        }
        
        if self.security.api_key_validation_on_startup {
            // Additional validation logic here
            if api_key.contains(' ') || api_key.contains('\n') {
                return Err(anyhow::anyhow!("API key contains invalid characters"));
            }
        }
        
        Ok(())
    }
}
```

## Project-Specific Configuration (CLAUDE.md)

```markdown
# Claude 4 Configuration for [Project Name]

## Project Context
This is a [description of project] built with [technology stack], configured for Claude 4 integration.

## Claude 4 Model Configuration (Issue 1.2 - Gap 13)
- Default model: claude-4-sonnet-20250514
- Max tokens: 8192 (increased from 4096)
- Reasoning tokens: enabled with 4096 limit
- Performance mode: balanced (speed/quality trade-off)
- Analysis mode: enabled for complex tasks

## Tool Configuration
- File access: Limited to src/, docs/, tests/ directories
- Max file size: 10MB (increased for Claude 4)
- Preferred languages: Rust, JavaScript, TypeScript
- Build command: `cargo build && npm run build`
- Test command: `cargo test && npm test`

## API Security Configuration (Issue 1.5 - Gap 9)
- API key source: environment variables only
- Key validation: strict on startup
- Rate limiting: 60 requests per minute
- Audit logging: comprehensive tracking
- Sandbox mode: enabled by default

## Coding Standards for Claude 4
- Use existing error handling patterns
- Follow Rust naming conventions
- Maintain existing architecture patterns
- Add comprehensive documentation
- Leverage Claude 4's enhanced reasoning capabilities
- Implement proper error boundaries for new features

## Security Constraints
- No network access during tool execution
- Validate all file paths with strict checking
- Respect existing whitelist configuration
- Never expose API keys in code
- Enable directory traversal prevention
- Quarantine suspicious files automatically

## Development Workflow
- Always run tests after changes
- Use cargo fmt and clippy
- Commit with descriptive messages
- Update documentation for public APIs
- Validate Claude 4 specific features
- Test reasoning token usage

## Phase 4 Enhancement Package
- Advanced tool integration capabilities
- Computer use tools (disabled by default)
- Code execution sandbox (strict mode)
- Enhanced streaming with larger buffers
- Feature flag system for gradual rollout
```

## Advanced Security Templates

### API Key Management (Issue 1.5 - Gap 9)

```rust
// Enhanced API key management implementation
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct SecureApiKeyManager {
    current_key: Arc<RwLock<Option<String>>>,
    rotation_interval: std::time::Duration,
    validation_enabled: bool,
    audit_logger: Arc<dyn AuditLogger>,
}

impl SecureApiKeyManager {
    pub fn new(config: &SecurityConfig) -> Self {
        Self {
            current_key: Arc::new(RwLock::new(None)),
            rotation_interval: std::time::Duration::from_secs(
                config.api_key_rotation_interval_hours.unwrap_or(168) * 3600
            ),
            validation_enabled: config.api_key_validation_on_startup,
            audit_logger: Arc::new(DefaultAuditLogger::new()),
        }
    }
    
    pub async fn initialize(&self, api_key: String) -> Result<()> {
        if self.validation_enabled {
            self.validate_key(&api_key).await?;
        }
        
        let mut key_guard = self.current_key.write().await;
        *key_guard = Some(api_key);
        
        self.audit_logger.log_key_initialization().await;
        Ok(())
    }
    
    pub async fn get_key(&self) -> Result<String> {
        let key_guard = self.current_key.read().await;
        key_guard.clone().ok_or_else(|| anyhow::anyhow!("No API key available"))
    }
    
    async fn validate_key(&self, key: &str) -> Result<()> {
        // Validate format
        if !key.starts_with("sk-ant-") {
            return Err(anyhow::anyhow!("Invalid API key format"));
        }
        
        // Test API connection
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": "claude-4-sonnet-20250514",
                "max_tokens": 1,
                "messages": [{"role": "user", "content": "test"}]
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("API key validation failed"));
        }
        
        Ok(())
    }
}

pub trait AuditLogger: Send + Sync {
    async fn log_key_initialization(&self);
    async fn log_key_rotation(&self);
    async fn log_security_violation(&self, violation: &str);
}
```

### Phase 4 Feature Flags

```rust
// Feature flag system for Phase 4 enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    // Core Claude 4 features
    pub claude_4_reasoning: bool,
    pub claude_4_analysis_mode: bool,
    pub enhanced_streaming: bool,
    
    // Advanced tool integration
    pub computer_use_tools: bool,
    pub code_execution_sandbox: bool,
    pub advanced_file_operations: bool,
    
    // Security enhancements
    pub strict_security_mode: bool,
    pub api_key_rotation: bool,
    pub comprehensive_audit_logging: bool,
    
    // Performance optimizations
    pub request_batching: bool,
    pub response_caching: bool,
    pub connection_pooling: bool,
    
    // Integration testing
    pub integration_test_mode: bool,
    pub debug_mode_enhanced: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            // Claude 4 features enabled by default
            claude_4_reasoning: true,
            claude_4_analysis_mode: true,
            enhanced_streaming: true,
            
            // Advanced tools disabled by default for security
            computer_use_tools: false,
            code_execution_sandbox: false,
            advanced_file_operations: false,
            
            // Security features enabled by default
            strict_security_mode: true,
            api_key_rotation: false,  // Requires manual setup
            comprehensive_audit_logging: true,
            
            // Performance features enabled
            request_batching: true,
            response_caching: true,
            connection_pooling: true,
            
            // Testing features disabled by default
            integration_test_mode: false,
            debug_mode_enhanced: false,
        }
    }
}
```

## References
- [Claude 4 API Documentation](https://docs.anthropic.com/en/api/claude-4)
- [Anthropic Security Best Practices](https://docs.anthropic.com/en/api/security)
- [Tauri v2 Configuration Guide](https://tauri.app/v2/guides/getting-started/configuration)
- [Serde Configuration Patterns](https://serde.rs/examples.html)
- [Rust Async Security Patterns](https://rust-lang.github.io/async-book/)
- [Issue 1.2 - Claude 4 Model Integration](../issues/issue-1-2-claude-4-model-integration.md)
- [Issue 1.5 - API Security Enhancement](../issues/issue-1-5-api-security-enhancement.md)
- Analysis conducted: 2024-12-21
- Updated for Claude 4 integration: 2024-12-21