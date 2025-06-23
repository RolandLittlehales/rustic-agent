// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::result_large_err)]

use std::sync::Arc;
use tauri::{async_runtime::Mutex, Manager};

mod claude;
mod config;
mod file_watcher;
mod security;
use claude::whitelist::{persistence, WhitelistConfig};
use claude::{ClaudeClient, ClaudeConfig, Conversation, ConversationMessage};
use config::{
    constants::{self, error_templates, get_file_icon, DIRECTORY_ICON, SAFETY_BUFFER_RATIO},
    AppConfig,
};
use file_watcher::FileWatcherService;
use serde_json::Value;
use tokio::sync::RwLock;

// Shared application state
struct AppState {
    conversation: Arc<Mutex<Conversation>>,
    config: Arc<Mutex<ClaudeConfig>>,
    app_config: Arc<AppConfig>,
    whitelist: Arc<RwLock<WhitelistConfig>>,
    file_watcher: Arc<FileWatcherService>,
}

impl AppState {
    /// Helper to get a clone of the Claude config without repeating the locking pattern
    async fn get_claude_config(&self) -> ClaudeConfig {
        let config_guard = self.config.lock().await;
        config_guard.clone()
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, state: tauri::State<'_, AppState>) -> Result<String, String> {
    if name.is_empty() {
        return Err(error_templates::EMPTY_INPUT.to_string());
    }

    // Use validation limits from configuration
    state
        .app_config
        .validation
        .validate_name_length(name)
        .map_err(|e| e.to_string())?;

    // Basic input sanitization
    let safety_limit =
        (state.app_config.validation.name_max_chars as f32 * SAFETY_BUFFER_RATIO) as usize;
    let sanitized_name = name
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .take(safety_limit)
        .collect::<String>();

    Ok(format!(
        "Hello, {}! You've been greeted from Rust!",
        sanitized_name
    ))
}

#[tauri::command]
async fn get_api_key_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let config = state.config.lock().await;
    Ok(!config.api_key.is_empty())
}

#[tauri::command]
async fn initialize_with_env_key(state: tauri::State<'_, AppState>) -> Result<String, String> {
    // Try to get API key from environment variable
    let api_key = std::env::var("CLAUDE_API_KEY").unwrap_or_default();

    if api_key.is_empty() {
        return Err(error_templates::API_KEY_NOT_FOUND.to_string());
    }

    // Update the config with the API key
    {
        let mut config = state.config.lock().await;
        config.api_key = api_key;
    }

    // Create a new Claude client with the updated config
    let config = state.get_claude_config().await;
    let _client = ClaudeClient::new(config).map_err(|e| {
        error_templates::with_context(error_templates::CLIENT_CREATION_FAILED, &e.to_string())
    })?;

    Ok("Claude API key initialized from environment".to_string())
}

#[tauri::command]
async fn set_claude_api_key(
    api_key: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Log without exposing key details
    if !api_key.is_empty() {
        // API key provided (length hidden for security)
    }

    // Update the config with the API key
    {
        let mut config = state.config.lock().await;
        config.api_key = api_key;
    }

    // Create a new Claude client with the updated config
    let config = state.get_claude_config().await;
    let _client = ClaudeClient::new(config).map_err(|e| {
        error_templates::with_context(error_templates::CLIENT_CREATION_FAILED, &e.to_string())
    })?;

    // Note: In a real app, you'd want to properly manage this state with Arc/Mutex
    // For now, we'll just return success
    Ok("Claude API key set successfully".to_string())
}

#[tauri::command]
async fn send_message_to_claude(
    message: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Input validation
    if message.is_empty() {
        return Err(error_templates::EMPTY_INPUT.to_string());
    }

    // Use validation limits from configuration
    state
        .app_config
        .validation
        .validate_message_length(message.len())
        .map_err(|e| e.to_string())?;

    // Basic content filtering using constants
    let message_lower = message.to_lowercase();
    for pattern in constants::SUSPICIOUS_PATTERNS {
        if message_lower.contains(pattern) {
            return Err(error_templates::UNSAFE_CONTENT.to_string());
        }
    }

    // Check if we have a valid configuration
    let config = state.get_claude_config().await;

    if config.api_key.is_empty() {
        return Err(error_templates::API_KEY_NOT_SET.to_string());
    }

    // Create Claude client
    let client = ClaudeClient::new(config).map_err(|e| {
        error_templates::with_context(error_templates::CLIENT_CREATION_FAILED, &e.to_string())
    })?;

    // Send message to Claude
    let response = {
        let mut conversation = state.conversation.lock().await;
        client.chat(&mut conversation, message).await.map_err(|e| {
            error_templates::with_context(error_templates::API_ERROR, &e.to_string())
        })?
    };

    Ok(response)
}

#[tauri::command]
async fn get_conversation_history(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ConversationMessage>, String> {
    let conversation = state.conversation.lock().await;
    Ok(conversation.messages.clone())
}

#[tauri::command]
async fn clear_conversation(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut conversation = state.conversation.lock().await;
    *conversation = Conversation::default();
    Ok("Conversation cleared".to_string())
}

/// Execute the list directory tool with the given path
async fn execute_list_directory_tool(
    path: String,
    whitelist: Arc<RwLock<WhitelistConfig>>,
) -> Result<String, String> {
    use claude::tools::{AgentTool, ListDirectoryTool};

    let mut tool = ListDirectoryTool::new();
    tool.set_whitelist(whitelist);

    let mut input = serde_json::Map::new();
    input.insert("path".to_string(), Value::String(path));

    tool.execute(Value::Object(input)).await.map_err(|e| {
        println!("❌ Tool execution failed: {}", e);
        error_templates::operation_failed("list directory", &e.to_string())
    })
}

/// Determine the appropriate icon for a file based on its type and extension
fn determine_file_icon(file_type: &str, file_name: &str) -> String {
    match file_type {
        "directory" => DIRECTORY_ICON.to_string(),
        _ => {
            let extension = file_name.split('.').next_back().unwrap_or("");
            get_file_icon(extension).to_string()
        }
    }
}

/// Parse directory listing results into structured FileItem objects
fn parse_directory_results(result: &str) -> Result<Vec<FileItem>, String> {
    let mut items = Vec::new();
    let current_dir = std::env::current_dir().unwrap_or_default();

    for line in result.lines() {
        if line.is_empty() || line.starts_with("...") {
            continue;
        }

        if let Some((name, type_info)) = line.rsplit_once(" (") {
            let file_type = type_info.trim_end_matches(')');
            let icon = determine_file_icon(file_type, name);

            items.push(FileItem {
                name: name.to_string(),
                file_type: file_type.to_string(),
                icon,
                path: format!("{}/{}", current_dir.display(), name),
            });
        }
    }

    Ok(items)
}

#[tauri::command]
async fn list_directory(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileItem>, String> {
    let result = execute_list_directory_tool(path, state.whitelist.clone()).await?;
    parse_directory_results(&result)
}

#[derive(serde::Serialize)]
struct FileItem {
    name: String,
    file_type: String,
    icon: String,
    path: String,
}

// Whitelist management commands
#[tauri::command]
async fn whitelist_add_directory(
    path: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut whitelist = state.whitelist.write().await;

    match whitelist.add_directory(&path) {
        Ok(canonical_path) => {
            // Save to disk
            if let Err(e) = persistence::save(&app, &whitelist).await {
                return Err(error_templates::with_context(
                    error_templates::WHITELIST_SAVE_FAILED,
                    &e.to_string(),
                ));
            }
            Ok(format!("Added directory: {}", canonical_path.display()))
        }
        Err(e) => Err(error_templates::operation_failed(
            "add directory",
            &e.to_string(),
        )),
    }
}

#[tauri::command]
async fn whitelist_remove_directory(
    path: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut whitelist = state.whitelist.write().await;

    match whitelist.remove_directory(&path) {
        Ok(removed) => {
            if removed {
                // Save to disk
                if let Err(e) = persistence::save(&app, &whitelist).await {
                    return Err(error_templates::with_context(
                        error_templates::WHITELIST_SAVE_FAILED,
                        &e.to_string(),
                    ));
                }
                Ok(format!("Removed directory: {}", path))
            } else {
                Err(error_templates::DIRECTORY_NOT_FOUND.to_string())
            }
        }
        Err(e) => Err(error_templates::operation_failed(
            "remove directory",
            &e.to_string(),
        )),
    }
}

#[tauri::command]
async fn whitelist_list_directories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let whitelist = state.whitelist.read().await;
    let directories = whitelist
        .list_directories()
        .into_iter()
        .map(|path| path.display().to_string())
        .collect();
    Ok(directories)
}

#[tauri::command]
async fn whitelist_set_enabled(
    enabled: bool,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut whitelist = state.whitelist.write().await;
    whitelist.set_enabled(enabled);

    // Save to disk
    if let Err(e) = persistence::save(&app, &whitelist).await {
        return Err(error_templates::with_context(
            error_templates::WHITELIST_SAVE_FAILED,
            &e.to_string(),
        ));
    }

    Ok(format!(
        "Whitelist {}",
        if enabled { "enabled" } else { "disabled" }
    ))
}

#[tauri::command]
async fn whitelist_get_config(
    state: tauri::State<'_, AppState>,
) -> Result<WhitelistConfig, String> {
    let whitelist = state.whitelist.read().await;
    Ok(whitelist.clone())
}

// File watching commands
#[tauri::command]
async fn start_file_watching(state: tauri::State<'_, AppState>) -> Result<String, String> {
    // Get whitelisted directories and start watching them
    let whitelist = state.whitelist.read().await;
    let directories = whitelist.list_directories();

    for dir in directories {
        if let Err(e) = state.file_watcher.start_watching(dir.clone()).await {
            eprintln!("Failed to watch directory {}: {}", dir.display(), e);
        }
    }

    // Start heartbeat
    state.file_watcher.start_heartbeat();

    Ok("File watching started".to_string())
}

#[tauri::command]
async fn stop_file_watching(state: tauri::State<'_, AppState>) -> Result<String, String> {
    state.file_watcher.stop_all().await;
    Ok("File watching stopped".to_string())
}

fn main() {
    // Load application configuration
    let app_config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Failed to load app config: {}, using defaults", e);
        AppConfig::default()
    });

    // Check for API key in environment on startup
    let initial_api_key = app_config.runtime.api_key.clone().unwrap_or_default();
    if !initial_api_key.is_empty() {
        println!(
            "🔑 Found {} API key in configuration",
            constants::ENV_CLAUDE_API_KEY
        );
    }

    // Initialize Claude config with values from app config
    let initial_config = ClaudeConfig {
        api_key: initial_api_key,
        model: app_config.runtime.model.clone(),
        max_tokens: app_config.runtime.max_tokens,
        temperature: app_config.runtime.temperature,
    };

    tauri::Builder::default()
        .setup(move |app| {
            // Load whitelist configuration from disk or create default
            let mut whitelist_config = tauri::async_runtime::block_on(async {
                persistence::load(app.handle()).await.unwrap_or_else(|e| {
                    eprintln!("Failed to load whitelist config: {}", e);
                    WhitelistConfig::default()
                })
            });

            // Ensure current directory is always accessible by default
            if let Ok(current_dir) = std::env::current_dir() {
                if whitelist_config.list_directories().is_empty() {
                    println!(
                        "📁 Adding current directory to whitelist: {}",
                        current_dir.display()
                    );
                    if let Err(e) = whitelist_config.add_directory(&current_dir) {
                        eprintln!("Failed to add current directory to whitelist: {}", e);
                    }
                }
            }

            // Create file watcher service
            let file_watcher = Arc::new(FileWatcherService::new(app.handle().clone()));

            let app_state = AppState {
                conversation: Arc::new(Mutex::new(Conversation::default())),
                config: Arc::new(Mutex::new(initial_config)),
                app_config: Arc::new(app_config),
                whitelist: Arc::new(RwLock::new(whitelist_config)),
                file_watcher,
            };

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_api_key_status,
            initialize_with_env_key,
            set_claude_api_key,
            send_message_to_claude,
            get_conversation_history,
            clear_conversation,
            list_directory,
            whitelist_add_directory,
            whitelist_remove_directory,
            whitelist_list_directories,
            whitelist_set_enabled,
            whitelist_get_config,
            start_file_watching,
            stop_file_watching,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
