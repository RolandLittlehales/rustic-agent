use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, RwLock};
use tokio::time;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileChangeEvent {
    pub path: String,
    pub event_type: String,
    pub affected_paths: Vec<String>,
}

pub struct FileWatcherService {
    watchers: Arc<RwLock<HashMap<PathBuf, RecommendedWatcher>>>,
    app_handle: AppHandle,
    debounce_map: Arc<RwLock<HashMap<PathBuf, Instant>>>,
}

impl FileWatcherService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            watchers: Arc::new(RwLock::new(HashMap::new())),
            app_handle,
            debounce_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_watching(
        &self,
        path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        crate::log_info!(
            "file_watcher",
            &format!("Starting to watch directory: {}", path.display())
        );

        // Check if already watching this path
        {
            let watchers = self.watchers.read().await;
            if watchers.contains_key(&path) {
                crate::log_warn!(
                    "file_watcher",
                    &format!("Already watching path: {}", path.display())
                );
                return Ok(());
            }
        }

        let (tx, mut rx) = mpsc::unbounded_channel();
        let app_handle = self.app_handle.clone();
        let debounce_map = self.debounce_map.clone();
        let watch_path = path.clone();

        // Create file watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if let Err(e) = tx.send(event) {
                        eprintln!("Failed to send file event: {}", e);
                    }
                }
            },
            Config::default().with_poll_interval(Duration::from_millis(500)),
        )?;

        // Start watching the directory
        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        // Store the watcher
        {
            let mut watchers = self.watchers.write().await;
            watchers.insert(path.clone(), watcher);
        }

        // Spawn task to handle events
        tokio::spawn(async move {
            let debounce_duration = Duration::from_millis(300);

            while let Some(event) = rx.recv().await {
                println!("📁 File event received: {:?}", event);

                // Filter out irrelevant events
                if !Self::is_relevant_event(&event) {
                    continue;
                }

                // Debounce events to avoid spam
                let now = Instant::now();
                {
                    let mut debounce = debounce_map.write().await;
                    if let Some(&last_event) = debounce.get(&watch_path) {
                        if now.duration_since(last_event) < debounce_duration {
                            continue;
                        }
                    }
                    debounce.insert(watch_path.clone(), now);
                }

                // Emit event to frontend
                let file_event = FileChangeEvent {
                    path: watch_path.to_string_lossy().to_string(),
                    event_type: Self::event_type_string(&event.kind),
                    affected_paths: event
                        .paths
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect(),
                };

                if let Err(e) = app_handle.emit("file_changed", file_event) {
                    eprintln!("Failed to emit file change event: {}", e);
                }
            }
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn stop_watching(
        &self,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut watchers = self.watchers.write().await;
        if watchers.remove(path).is_some() {
            println!("🛑 Stopped watching: {}", path.display());
        }
        Ok(())
    }

    pub async fn stop_all(&self) {
        let mut watchers = self.watchers.write().await;
        let count = watchers.len();
        watchers.clear();
        println!("🛑 Stopped watching {} directories", count);
    }

    fn is_relevant_event(event: &Event) -> bool {
        match &event.kind {
            EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(_) => {
                // Filter out temporary files and hidden files
                !event.paths.iter().any(|path| {
                    if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy();
                        name_str.starts_with('.')
                            || name_str.ends_with(".tmp")
                            || name_str.ends_with(".swp")
                            || name_str.contains("~")
                    } else {
                        false
                    }
                })
            }
            _ => false,
        }
    }

    fn event_type_string(kind: &EventKind) -> String {
        match kind {
            EventKind::Create(_) => "created".to_string(),
            EventKind::Remove(_) => "removed".to_string(),
            EventKind::Modify(_) => "modified".to_string(),
            _ => "other".to_string(),
        }
    }

    // Start a simple heartbeat to refresh periodically
    pub fn start_heartbeat(&self) {
        let app_handle = self.app_handle.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30)); // Every 30 seconds

            loop {
                interval.tick().await;
                if let Err(e) = app_handle.emit("heartbeat_refresh", ()) {
                    eprintln!("Failed to emit heartbeat: {}", e);
                }
            }
        });
    }
}
