// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{async_runtime::Mutex, Manager};

mod claude;
mod file_watcher;
mod security;
use claude::whitelist::{persistence, WhitelistConfig};
use claude::{ClaudeClient, ClaudeConfig, Conversation, ConversationMessage};
use file_watcher::FileWatcherService;
use serde_json::Value;
use tokio::sync::RwLock;

// Shared application state
struct AppState {
    conversation: Arc<Mutex<Conversation>>,
    config: Arc<Mutex<ClaudeConfig>>,
    whitelist: Arc<RwLock<WhitelistConfig>>,
    file_watcher: Arc<FileWatcherService>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if name.len() > 100 {
        return Err("Name is too long (max 100 characters)".to_string());
    }

    // Basic input sanitization
    let sanitized_name = name
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .take(50)
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
        return Err("No API key found in environment variables".to_string());
    }

    // Update the config with the API key
    {
        let mut config = state.config.lock().await;
        config.api_key = api_key;
    }

    // Create a new Claude client with the updated config
    let config = {
        let config_guard = state.config.lock().await;
        config_guard.clone()
    };
    let _client =
        ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;

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
    let config = {
        let config_guard = state.config.lock().await;
        config_guard.clone()
    };
    let _client =
        ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;

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
        return Err("Message cannot be empty".to_string());
    }

    if message.len() > 50000 {
        // 50KB limit
        return Err("Message too long (max 50KB)".to_string());
    }

    // Basic content filtering
    let suspicious_patterns = [
        "<script",
        "javascript:",
        "data:",
        "vbscript:",
        "onload=",
        "onerror=",
    ];
    let message_lower = message.to_lowercase();
    for pattern in &suspicious_patterns {
        if message_lower.contains(pattern) {
            return Err("Message contains potentially unsafe content".to_string());
        }
    }

    // Check if we have a valid configuration
    let config = {
        let config_guard = state.config.lock().await;
        config_guard.clone()
    };

    if config.api_key.is_empty() {
        return Err("Claude API key not set. Please set the API key first.".to_string());
    }

    // Create Claude client
    let client =
        ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;

    // Send message to Claude
    let response = {
        let mut conversation = state.conversation.lock().await;
        client
            .chat(&mut conversation, message)
            .await
            .map_err(|e| format!("Claude API error: {}", e))?
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

#[tauri::command]
async fn list_directory(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileItem>, String> {
    use claude::tools::{AgentTool, ListDirectoryTool};

    let mut tool = ListDirectoryTool::new();

    // Set the whitelist for the tool
    tool.set_whitelist(state.whitelist.clone());

    let mut input = serde_json::Map::new();
    input.insert("path".to_string(), Value::String(path.clone()));

    match tool.execute(Value::Object(input)).await {
        Ok(result) => {
            // Parse the result string into structured data
            let mut items = Vec::new();
            let current_dir = std::env::current_dir().unwrap_or_default();

            for line in result.lines() {
                if line.is_empty() || line.starts_with("...") {
                    continue;
                }

                if let Some((name, type_info)) = line.rsplit_once(" (") {
                    let file_type = type_info.trim_end_matches(')');
                    let icon = match file_type {
                        "directory" => "📁",
                        _ => {
                            // Determine icon based on file extension
                            match name.split('.').next_back().unwrap_or("") {
                                "rs" => "🦀",
                                "js" | "ts" => "📄",
                                "json" => "⚙️",
                                "md" => "📝",
                                "toml" => "⚙️",
                                "html" => "🌐",
                                "css" => "🎨",
                                _ => "📄",
                            }
                        }
                    };

                    items.push(FileItem {
                        name: name.to_string(),
                        file_type: file_type.to_string(),
                        icon: icon.to_string(),
                        path: format!("{}/{}", current_dir.display(), name),
                    });
                }
            }

            Ok(items)
        }
        Err(e) => {
            println!("❌ Tool execution failed: {}", e);
            Err(format!("Failed to list directory: {}", e))
        }
    }
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
                return Err(format!("Failed to save whitelist: {}", e));
            }
            Ok(format!("Added directory: {}", canonical_path.display()))
        }
        Err(e) => Err(format!("Failed to add directory: {}", e)),
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
                    return Err(format!("Failed to save whitelist: {}", e));
                }
                Ok(format!("Removed directory: {}", path))
            } else {
                Err("Directory not found in whitelist".to_string())
            }
        }
        Err(e) => Err(format!("Failed to remove directory: {}", e)),
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
        return Err(format!("Failed to save whitelist: {}", e));
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
    // Check for API key in environment on startup
    let initial_api_key = std::env::var("CLAUDE_API_KEY").unwrap_or_default();
    if !initial_api_key.is_empty() {
        println!("🔑 Found CLAUDE_API_KEY in environment");
    }

    // Initialize app state with default values and environment API key if available
    let initial_config = ClaudeConfig {
        api_key: initial_api_key,
        ..Default::default()
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
