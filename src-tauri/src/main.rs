// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

mod claude;
mod security;

use claude::{ClaudeClient, ClaudeConfig, Conversation};

#[derive(Debug)]
struct AppState {
    claude_client: Option<Arc<ClaudeClient>>,
    conversation: Arc<Mutex<Conversation>>,
    config: Arc<Mutex<ClaudeConfig>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            claude_client: None,
            conversation: Arc::new(Mutex::new(Conversation::default())),
            config: Arc::new(Mutex::new(ClaudeConfig::default())),
        }
    }
}

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    // Input validation
    if name.is_empty() || name.len() > 100 {
        return Err("Invalid name parameter".to_string());
    }
    
    // Sanitize name to prevent any potential issues
    let sanitized_name = name.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .take(50)
        .collect::<String>();
    
    Ok(format!("Hello, {}! You've been greeted from Rust!", sanitized_name))
}

#[tauri::command]
async fn set_claude_api_key(
    apiKey: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Log without exposing key details
    if apiKey.is_empty() {
        println!("🔑 Rust: set_claude_api_key called with empty key");
    } else {
        println!("🔑 Rust: set_claude_api_key called with key (length: {})", apiKey.len());
    }
    
    // Update the config with the API key
    {
        let mut config = state.config.lock().await;
        config.api_key = apiKey;
    }
    
    // Create a new Claude client with the updated config
    let config = {
        let config_guard = state.config.lock().await;
        config_guard.clone()
    };
    let _client = ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;
    
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
    
    if message.len() > 50000 { // 50KB limit
        return Err("Message too long (max 50KB)".to_string());
    }
    
    // Basic content filtering
    let suspicious_patterns = ["<script", "javascript:", "data:", "vbscript:", "onload=", "onerror="];
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
    let client = ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;
    
    // Send message to Claude
    let response = {
        let mut conversation = state.conversation.lock().await;
        client
            .chat(&mut *conversation, message)
            .await
            .map_err(|e| format!("Claude API error: {}", e))?
    };
    
    Ok(response)
}

#[tauri::command]
async fn get_conversation_history(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<claude::ConversationMessage>, String> {
    let conversation = state.conversation.lock().await;
    Ok(conversation.messages.clone())
}

#[tauri::command]
async fn clear_conversation(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut conversation = state.conversation.lock().await;
    *conversation = Conversation::default();
    Ok("Conversation cleared".to_string())
}

#[tauri::command]
async fn get_available_tools(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let config = {
        let config_guard = state.config.lock().await;
        config_guard.clone()
    };
    
    if config.api_key.is_empty() {
        return Ok(vec!["No tools available - API key not set".to_string()]);
    }
    
    let client = ClaudeClient::new(config).map_err(|e| format!("Failed to create Claude client: {}", e))?;
    Ok(client.get_available_tools())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            app.manage(AppState::default());
            
            // Set up global error handling
            let _handle = app.handle().clone();
            
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            
            println!("Tauri app initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            set_claude_api_key,
            send_message_to_claude,
            get_conversation_history,
            clear_conversation,
            get_available_tools
        ])
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    println!("Window close requested for: {}", window.label());
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .map_err(|e| {
            eprintln!("Error running Tauri application: {}", e);
            e
        })?;

    Ok(())
}