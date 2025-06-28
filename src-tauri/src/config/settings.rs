use crate::config::constants::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Global application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub ui: UiSettings,
    pub performance: PerformanceSettings,
    pub security: SecuritySettings,
    pub logging: LoggingSettings,
    pub storage: StorageSettings,
    pub network: NetworkSettings,
    pub features: FeatureSettings,
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub app_name: String,
    pub version: String,
    pub debug_mode: bool,
    pub auto_save: bool,
    pub auto_save_interval: u64, // seconds
    pub language: String,
    pub theme: String,
}

/// User interface settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_resizable: bool,
    pub window_maximized: bool,
    pub window_fullscreen: bool,
    pub always_on_top: bool,
    pub show_menu_bar: bool,
    pub show_status_bar: bool,
    pub show_toolbar: bool,
    pub font_family: String,
    pub font_size: u32,
    pub theme_mode: ThemeMode,
    pub accent_color: String,
    pub animations_enabled: bool,
    pub transparency_enabled: bool,
    pub transparency_level: f32, // 0.0 to 1.0
}

/// Performance-related settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub max_memory_usage: u64, // MB
    pub max_cpu_usage: f32,    // percentage
    pub enable_hardware_acceleration: bool,
    pub max_concurrent_operations: usize,
    pub cache_size: u64,       // MB
    pub preload_data: bool,
    pub background_processing: bool,
    pub optimization_level: OptimizationLevel,
}

/// Security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub enable_encryption: bool,
    pub encryption_algorithm: String,
    pub require_authentication: bool,
    pub session_timeout: u64, // minutes
    pub max_login_attempts: u32,
    pub enable_audit_log: bool,
    pub secure_storage: bool,
    pub auto_lock_timeout: u64, // minutes
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: LogLevel,
    pub enable_file_logging: bool,
    pub log_file_path: PathBuf,
    pub max_log_file_size: u64, // MB
    pub max_log_files: u32,
    pub enable_console_logging: bool,
    pub enable_remote_logging: bool,
    pub remote_log_endpoint: Option<String>,
    pub log_format: LogFormat,
}

/// Storage settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub data_directory: PathBuf,
    pub backup_directory: PathBuf,
    pub temp_directory: PathBuf,
    pub max_storage_size: u64, // MB
    pub enable_compression: bool,
    pub compression_level: u32,
    pub enable_backup: bool,
    pub backup_interval: u64, // hours
    pub max_backups: u32,
    pub cleanup_old_files: bool,
    pub cleanup_interval: u64, // days
}

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub enable_networking: bool,
    pub api_base_url: String,
    pub api_timeout: u64, // seconds
    pub max_retries: u32,
    pub retry_delay: u64, // milliseconds
    pub enable_proxy: bool,
    pub proxy_host: Option<String>,
    pub proxy_port: Option<u16>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub user_agent: String,
    pub enable_ssl_verification: bool,
}

/// Feature flags and experimental settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSettings {
    pub experimental_features: HashMap<String, bool>,
    pub beta_features: HashMap<String, bool>,
    pub feature_flags: HashMap<String, serde_json::Value>,
}

/// Theme mode enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
    System,
}

/// Optimization level enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Low,
    Medium,
    High,
    Aggressive,
}

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Log format enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Plain,
    Json,
    Structured,
    Custom(String),
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            ui: UiSettings::default(),
            performance: PerformanceSettings::default(),
            security: SecuritySettings::default(),
            logging: LoggingSettings::default(),
            storage: StorageSettings::default(),
            network: NetworkSettings::default(),
            features: FeatureSettings::default(),
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            app_name: DEFAULT_APP_NAME.to_string(),
            version: DEFAULT_VERSION.to_string(),
            debug_mode: false,
            auto_save: true,
            auto_save_interval: 300, // 5 minutes
            language: "en".to_string(),
            theme: "default".to_string(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            window_width: DEFAULT_WINDOW_WIDTH,
            window_height: DEFAULT_WINDOW_HEIGHT,
            window_resizable: true,
            window_maximized: false,
            window_fullscreen: false,
            always_on_top: false,
            show_menu_bar: true,
            show_status_bar: true,
            show_toolbar: true,
            font_family: DEFAULT_FONT_FAMILY.to_string(),
            font_size: DEFAULT_FONT_SIZE,
            theme_mode: ThemeMode::Auto,
            accent_color: "#007ACC".to_string(),
            animations_enabled: true,
            transparency_enabled: false,
            transparency_level: 0.95,
        }
    }
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            max_memory_usage: MAX_MEMORY_USAGE,
            max_cpu_usage: MAX_CPU_USAGE,
            enable_hardware_acceleration: true,
            max_concurrent_operations: MAX_CONCURRENT_OPERATIONS,
            cache_size: DEFAULT_CACHE_SIZE,
            preload_data: true,
            background_processing: true,
            optimization_level: OptimizationLevel::Medium,
        }
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            require_authentication: false,
            session_timeout: SESSION_TIMEOUT,
            max_login_attempts: MAX_LOGIN_ATTEMPTS,
            enable_audit_log: true,
            secure_storage: true,
            auto_lock_timeout: 30, // 30 minutes
        }
    }
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_file_logging: true,
            log_file_path: PathBuf::from("logs/app.log"),
            max_log_file_size: MAX_LOG_FILE_SIZE,
            max_log_files: MAX_LOG_FILES,
            enable_console_logging: true,
            enable_remote_logging: false,
            remote_log_endpoint: None,
            log_format: LogFormat::Structured,
        }
    }
}

impl Default for StorageSettings {
    fn default() -> Self {
        Self {
            data_directory: PathBuf::from("data"),
            backup_directory: PathBuf::from("backups"),
            temp_directory: PathBuf::from("temp"),
            max_storage_size: MAX_STORAGE_SIZE,
            enable_compression: true,
            compression_level: 6,
            enable_backup: true,
            backup_interval: BACKUP_INTERVAL,
            max_backups: MAX_BACKUPS,
            cleanup_old_files: true,
            cleanup_interval: CLEANUP_INTERVAL,
        }
    }
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            enable_networking: true,
            api_base_url: DEFAULT_API_BASE_URL.to_string(),
            api_timeout: API_TIMEOUT,
            max_retries: MAX_RETRIES,
            retry_delay: RETRY_DELAY,
            enable_proxy: false,
            proxy_host: None,
            proxy_port: None,
            proxy_username: None,
            proxy_password: None,
            user_agent: format!("{}/{}", DEFAULT_APP_NAME, DEFAULT_VERSION),
            enable_ssl_verification: true,
        }
    }
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            experimental_features: HashMap::new(),
            beta_features: HashMap::new(),
            feature_flags: HashMap::new(),
        }
    }
}

impl AppSettings {
    /// Load settings from a configuration file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let settings: AppSettings = toml::from_str(&content)?;
        Ok(settings)
    }

    /// Save settings to a configuration file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load settings from environment variables
    pub fn load_from_env(&mut self) {
        if let Ok(debug) = std::env::var("DEBUG") {
            self.general.debug_mode = debug.parse().unwrap_or(false);
        }
        
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            self.logging.level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            };
        }

        if let Ok(api_url) = std::env::var("API_BASE_URL") {
            self.network.api_base_url = api_url;
        }

        if let Ok(data_dir) = std::env::var("DATA_DIRECTORY") {
            self.storage.data_directory = PathBuf::from(data_dir);
        }
    }

    /// Validate settings and apply constraints
    pub fn validate(&mut self) -> Result<(), String> {
        // Validate window dimensions
        if self.ui.window_width < MIN_WINDOW_WIDTH {
            self.ui.window_width = MIN_WINDOW_WIDTH;
        }
        if self.ui.window_height < MIN_WINDOW_HEIGHT {
            self.ui.window_height = MIN_WINDOW_HEIGHT;
        }

        // Validate font size
        if self.ui.font_size < MIN_FONT_SIZE {
            self.ui.font_size = MIN_FONT_SIZE;
        }
        if self.ui.font_size > MAX_FONT_SIZE {
            self.ui.font_size = MAX_FONT_SIZE;
        }

        // Validate transparency level
        if self.ui.transparency_level < 0.0 {
            self.ui.transparency_level = 0.0;
        }
        if self.ui.transparency_level > 1.0 {
            self.ui.transparency_level = 1.0;
        }

        // Validate memory usage
        if self.performance.max_memory_usage > MAX_MEMORY_USAGE {
            return Err(format!(
                "Max memory usage cannot exceed {} MB",
                MAX_MEMORY_USAGE
            ));
        }

        // Validate CPU usage
        if self.performance.max_cpu_usage > 100.0 {
            self.performance.max_cpu_usage = 100.0;
        }

        // Validate timeouts
        if self.network.api_timeout == 0 {
            self.network.api_timeout = API_TIMEOUT;
        }

        // Validate session timeout
        if self.security.session_timeout == 0 {
            self.security.session_timeout = SESSION_TIMEOUT;
        }

        Ok(())
    }

    /// Reset settings to default values
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Merge settings from another instance
    pub fn merge(&mut self, other: &AppSettings) {
        // This is a simplified merge - in practice, you might want more sophisticated merging logic
        self.general.debug_mode = other.general.debug_mode;
        self.ui.theme_mode = other.ui.theme_mode.clone();
        self.logging.level = other.logging.level.clone();
        // Add more fields as needed
    }

    /// Get a setting value by path (dot notation)
    pub fn get_setting(&self, path: &str) -> Option<serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "general" => match parts[1] {
                "debug_mode" => Some(serde_json::Value::Bool(self.general.debug_mode)),
                "auto_save" => Some(serde_json::Value::Bool(self.general.auto_save)),
                "language" => Some(serde_json::Value::String(self.general.language.clone())),
                _ => None,
            },
            "ui" => match parts[1] {
                "window_width" => Some(serde_json::Value::Number(self.ui.window_width.into())),
                "window_height" => Some(serde_json::Value::Number(self.ui.window_height.into())),
                "font_size" => Some(serde_json::Value::Number(self.ui.font_size.into())),
                _ => None,
            },
            _ => None,
        }
    }

    /// Set a setting value by path (dot notation)
    pub fn set_setting(&mut self, path: &str, value: serde_json::Value) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return Err("Invalid setting path".to_string());
        }

        match parts[0] {
            "general" => match parts[1] {
                "debug_mode" => {
                    if let Some(val) = value.as_bool() {
                        self.general.debug_mode = val;
                        Ok(())
                    } else {
                        Err("Invalid boolean value".to_string())
                    }
                },
                "language" => {
                    if let Some(val) = value.as_str() {
                        self.general.language = val.to_string();
                        Ok(())
                    } else {
                        Err("Invalid string value".to_string())
                    }
                },
                _ => Err("Unknown setting".to_string()),
            },
            "ui" => match parts[1] {
                "window_width" => {
                    if let Some(val) = value.as_u64() {
                        self.ui.window_width = val as u32;
                        Ok(())
                    } else {
                        Err("Invalid number value".to_string())
                    }
                },
                "font_size" => {
                    if let Some(val) = value.as_u64() {
                        self.ui.font_size = val as u32;
                        Ok(())
                    } else {
                        Err("Invalid number value".to_string())
                    }
                },
                _ => Err("Unknown setting".to_string()),
            },
            _ => Err("Unknown setting category".to_string()),
        }
    }
}

/// Settings manager for handling configuration persistence and updates
pub struct SettingsManager {
    settings: AppSettings,
    config_path: PathBuf,
    auto_save: bool,
}

impl SettingsManager {
    /// Create a new settings manager
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: AppSettings::default(),
            config_path,
            auto_save: true,
        }
    }

    /// Load settings from file or create default if file doesn't exist
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            self.settings = AppSettings::load_from_file(&self.config_path)?;
        } else {
            self.settings = AppSettings::default();
            self.save()?;
        }
        
        // Load environment variable overrides
        self.settings.load_from_env();
        
        // Validate settings
        self.settings.validate()?;
        
        Ok(())
    }

    /// Save settings to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        self.settings.save_to_file(&self.config_path)
    }

    /// Get immutable reference to settings
    pub fn get(&self) -> &AppSettings {
        &self.settings
    }

    /// Get mutable reference to settings
    pub fn get_mut(&mut self) -> &mut AppSettings {
        &mut self.settings
    }

    /// Update a specific setting
    pub fn update_setting(&mut self, path: &str, value: serde_json::Value) -> Result<(), String> {
        self.settings.set_setting(path, value)?;
        
        if self.auto_save {
            if let Err(e) = self.save() {
                return Err(format!("Failed to save settings: {}", e));
            }
        }
        
        Ok(())
    }

    /// Enable or disable auto-save
    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
    }

    /// Reset settings to defaults
    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.settings.reset_to_defaults();
        self.save()
    }

    /// Export settings to a different file
    pub fn export(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.settings.save_to_file(path)
    }

    /// Import settings from a file
    pub fn import(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let imported_settings = AppSettings::load_from_file(path)?;
        self.settings.merge(&imported_settings);
        
        if self.auto_save {
            self.save()?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.general.app_name, DEFAULT_APP_NAME);
        assert_eq!(settings.ui.window_width, DEFAULT_WINDOW_WIDTH);
        assert!(settings.security.enable_encryption);
    }

    #[test]
    fn test_settings_validation() {
        let mut settings = AppSettings::default();
        settings.ui.window_width = 50; // Below minimum
        settings.ui.transparency_level = 1.5; // Above maximum
        
        settings.validate().unwrap();
        
        assert_eq!(settings.ui.window_width, MIN_WINDOW_WIDTH);
        assert_eq!(settings.ui.transparency_level, 1.0);
    }

    #[test]
    fn test_settings_manager() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let mut manager = SettingsManager::new(config_path.clone());
        manager.load().unwrap();
        
        // Test setting update
        manager.update_setting("general.debug_mode", serde_json::Value::Bool(true)).unwrap();
        assert!(manager.get().general.debug_mode);
        
        // Test persistence
        let mut manager2 = SettingsManager::new(config_path);
        manager2.load().unwrap();
        assert!(manager2.get().general.debug_mode);
    }

    #[test]
    fn test_get_set_setting() {
        let mut settings = AppSettings::default();
        
        // Test getting a setting
        let value = settings.get_setting("general.debug_mode").unwrap();
        assert_eq!(value, serde_json::Value::Bool(false));
        
        // Test setting a value
        settings.set_setting("general.debug_mode", serde_json::Value::Bool(true)).unwrap();
        assert!(settings.general.debug_mode);
    }
}