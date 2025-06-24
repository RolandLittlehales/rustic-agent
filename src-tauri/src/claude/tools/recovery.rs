use crate::claude::{
    error::ClaudeResult,
    tools::execution::{ToolError, ToolErrorType, ToolExecutionContext},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Recovery action to be taken when tool execution fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryAction {
    /// Retry the operation with specified delay and max attempts
    Retry { 
        delay: Duration, 
        max_attempts: u32,
        modified_input: Option<serde_json::Value>,
    },
    /// Use a fallback tool with different input
    FallbackTool { 
        tool_name: String, 
        input: serde_json::Value,
        reason: String,
    },
    /// Request user intervention with suggested actions
    UserIntervention { 
        message: String, 
        suggested_actions: Vec<String>,
        blocking: bool,
    },
    /// Skip this operation and continue with the chain
    SkipAndContinue {
        reason: String,
        impact_assessment: String,
    },
    /// Abort the entire operation chain
    AbortChain {
        reason: String,
        cleanup_actions: Vec<String>,
    },
    /// Apply a fix and then retry
    FixAndRetry {
        fix_description: String,
        fix_actions: Vec<FixAction>,
        retry_delay: Duration,
    },
}

impl RecoveryAction {
    /// Check if this recovery action requires user interaction
    pub fn requires_user_interaction(&self) -> bool {
        matches!(self, RecoveryAction::UserIntervention { blocking: true, .. })
    }

    /// Check if this recovery action stops the execution chain
    pub fn stops_execution(&self) -> bool {
        matches!(self, RecoveryAction::AbortChain { .. } | RecoveryAction::UserIntervention { blocking: true, .. })
    }

    /// Get the estimated delay before this action can be executed
    pub fn estimated_delay(&self) -> Duration {
        match self {
            RecoveryAction::Retry { delay, .. } => *delay,
            RecoveryAction::FixAndRetry { retry_delay, .. } => *retry_delay,
            RecoveryAction::FallbackTool { .. } => Duration::from_millis(100),
            RecoveryAction::UserIntervention { .. } => Duration::from_secs(0), // Immediate
            RecoveryAction::SkipAndContinue { .. } => Duration::from_millis(0),
            RecoveryAction::AbortChain { .. } => Duration::from_millis(0),
        }
    }
}

/// Specific fix actions that can be applied automatically
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixAction {
    /// Adjust input parameters
    ModifyInput {
        parameter: String,
        new_value: serde_json::Value,
        reason: String,
    },
    /// Increase timeout
    IncreaseTimeout {
        new_timeout: Duration,
    },
    /// Change file permissions (if applicable)
    ChangePermissions {
        path: String,
        permissions: String,
    },
    /// Create missing directory
    CreateDirectory {
        path: String,
    },
    /// Custom fix action (for extensibility)
    Custom {
        description: String,
        parameters: serde_json::Value,
    },
}

/// Trait for implementing tool-specific recovery strategies
#[async_trait]
pub trait RecoveryStrategy: Send + Sync + std::fmt::Debug {
    /// Analyze an error and suggest recovery actions
    async fn suggest_recovery(
        &self,
        error: &ToolError,
        context: &ToolExecutionContext,
        attempt_count: u32,
    ) -> ClaudeResult<Vec<RecoveryAction>>;

    /// Check if this strategy can handle the given error type
    fn can_handle(&self, error: &ToolError) -> bool;

    /// Get the priority of this strategy (higher = more preferred)
    fn priority(&self) -> u32 {
        50
    }

    /// Get the name of this recovery strategy
    fn name(&self) -> &str;
}

/// Main recovery manager that coordinates different recovery strategies
#[derive(Debug)]
pub struct ToolRecoveryManager {
    strategies: Vec<Arc<dyn RecoveryStrategy>>,
    execution_history: Arc<RwLock<Vec<RecoveryAttempt>>>,
    config: RecoveryConfig,
}

impl ToolRecoveryManager {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
            execution_history: Arc::new(RwLock::new(Vec::new())),
            config: RecoveryConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RecoveryConfig) -> Self {
        self.config = config;
        self
    }

    /// Register a recovery strategy
    pub fn register_strategy(&mut self, strategy: Arc<dyn RecoveryStrategy>) {
        self.strategies.push(strategy);
        // Sort by priority (highest first)
        self.strategies.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Suggest recovery actions for a failed tool execution
    pub async fn suggest_recovery(
        &self,
        error: &ToolError,
        context: &ToolExecutionContext,
    ) -> ClaudeResult<Vec<RecoveryAction>> {
        let attempt_count = self.get_attempt_count(&context.execution_id).await;

        // Check if we've exceeded max recovery attempts
        if attempt_count >= self.config.max_recovery_attempts {
            return Ok(vec![RecoveryAction::AbortChain {
                reason: format!("Exceeded maximum recovery attempts ({})", self.config.max_recovery_attempts),
                cleanup_actions: vec!["Review error logs".to_string(), "Check system configuration".to_string()],
            }]);
        }

        let mut all_actions = Vec::new();

        // Get suggestions from all applicable strategies
        for strategy in &self.strategies {
            if strategy.can_handle(error) {
                match strategy.suggest_recovery(error, context, attempt_count).await {
                    Ok(mut actions) => {
                        all_actions.append(&mut actions);
                    }
                    Err(e) => {
                        eprintln!("Warning: Recovery strategy '{}' failed: {}", strategy.name(), e);
                    }
                }
            }
        }

        // Apply recovery rules and prioritization
        let filtered_actions = self.apply_recovery_rules(all_actions, error, attempt_count);

        // Record this recovery attempt
        self.record_recovery_attempt(&context.execution_id, error, &filtered_actions).await;

        Ok(filtered_actions)
    }

    /// Execute a recovery action
    pub async fn execute_recovery_action(
        &self,
        action: &RecoveryAction,
        context: &ToolExecutionContext,
    ) -> ClaudeResult<RecoveryResult> {
        match action {
            RecoveryAction::Retry { delay, modified_input, .. } => {
                tokio::time::sleep(*delay).await;
                Ok(RecoveryResult::Retry {
                    modified_context: if let Some(input) = modified_input {
                        let mut new_context = context.clone();
                        new_context.input = input.clone();
                        Some(new_context)
                    } else {
                        None
                    },
                })
            }
            RecoveryAction::FallbackTool { tool_name, input, reason } => {
                Ok(RecoveryResult::FallbackTool {
                    tool_name: tool_name.clone(),
                    input: input.clone(),
                    reason: reason.clone(),
                })
            }
            RecoveryAction::FixAndRetry { fix_actions, retry_delay, .. } => {
                // Execute fix actions
                for fix_action in fix_actions {
                    if let Err(e) = self.execute_fix_action(fix_action, context).await {
                        eprintln!("Warning: Fix action failed: {}", e);
                    }
                }
                
                tokio::time::sleep(*retry_delay).await;
                Ok(RecoveryResult::Retry { modified_context: None })
            }
            RecoveryAction::UserIntervention { message, suggested_actions, .. } => {
                Ok(RecoveryResult::UserIntervention {
                    message: message.clone(),
                    suggested_actions: suggested_actions.clone(),
                })
            }
            RecoveryAction::SkipAndContinue { reason, .. } => {
                Ok(RecoveryResult::Skip {
                    reason: reason.clone(),
                })
            }
            RecoveryAction::AbortChain { reason, cleanup_actions } => {
                // Execute cleanup actions
                for cleanup_action in cleanup_actions {
                    eprintln!("Cleanup: {}", cleanup_action);
                }
                
                Ok(RecoveryResult::Abort {
                    reason: reason.clone(),
                })
            }
        }
    }

    async fn execute_fix_action(
        &self,
        fix_action: &FixAction,
        _context: &ToolExecutionContext,
    ) -> ClaudeResult<()> {
        match fix_action {
            FixAction::ModifyInput { parameter, new_value, reason } => {
                eprintln!("Fix: Modifying input parameter '{}' to {:?} ({})", parameter, new_value, reason);
                Ok(())
            }
            FixAction::IncreaseTimeout { new_timeout } => {
                eprintln!("Fix: Increasing timeout to {:?}", new_timeout);
                Ok(())
            }
            FixAction::CreateDirectory { path } => {
                eprintln!("Fix: Creating directory '{}'", path);
                // In a real implementation, this would actually create the directory
                // with proper permission checks
                Ok(())
            }
            FixAction::ChangePermissions { path, permissions } => {
                eprintln!("Fix: Changing permissions of '{}' to '{}'", path, permissions);
                // In a real implementation, this would change file permissions
                // with proper security validation
                Ok(())
            }
            FixAction::Custom { description, .. } => {
                eprintln!("Fix: {}", description);
                Ok(())
            }
        }
    }

    async fn get_attempt_count(&self, execution_id: &str) -> u32 {
        let history = self.execution_history.read().await;
        history.iter()
            .filter(|attempt| attempt.execution_id == execution_id)
            .count() as u32
    }

    async fn record_recovery_attempt(
        &self,
        execution_id: &str,
        error: &ToolError,
        actions: &[RecoveryAction],
    ) {
        let attempt = RecoveryAttempt {
            execution_id: execution_id.to_string(),
            error: error.clone(),
            actions: actions.to_vec(),
            timestamp: chrono::Utc::now(),
        };

        let mut history = self.execution_history.write().await;
        history.push(attempt);

        // Limit history size
        let max_size = self.config.max_history_size;
        if history.len() > max_size {
            let excess = history.len() - max_size;
            history.drain(0..excess);
        }
    }

    fn apply_recovery_rules(
        &self,
        actions: Vec<RecoveryAction>,
        error: &ToolError,
        attempt_count: u32,
    ) -> Vec<RecoveryAction> {
        let mut filtered_actions = actions;

        // Remove duplicate actions
        filtered_actions.dedup_by(|a, b| std::mem::discriminant(a) == std::mem::discriminant(b));

        // Limit total actions
        if filtered_actions.len() > self.config.max_actions_per_error {
            filtered_actions.truncate(self.config.max_actions_per_error);
        }

        // Apply attempt-based filtering
        if attempt_count >= self.config.escalation_threshold {
            // Escalate to more drastic actions
            filtered_actions.retain(|action| {
                matches!(action, 
                    RecoveryAction::UserIntervention { .. } |
                    RecoveryAction::AbortChain { .. } |
                    RecoveryAction::FallbackTool { .. }
                )
            });
        }

        // Apply error-type specific rules
        match error.error_type {
            ToolErrorType::ValidationError => {
                // For validation errors, prefer input modification over retries
                filtered_actions.sort_by_key(|action| match action {
                    RecoveryAction::FixAndRetry { .. } => 0,
                    RecoveryAction::UserIntervention { .. } => 1,
                    RecoveryAction::Retry { .. } => 2,
                    _ => 3,
                });
            }
            ToolErrorType::PermissionError => {
                // For permission errors, prefer user intervention
                filtered_actions.sort_by_key(|action| match action {
                    RecoveryAction::UserIntervention { .. } => 0,
                    RecoveryAction::FixAndRetry { .. } => 1,
                    _ => 2,
                });
            }
            ToolErrorType::TimeoutError => {
                // For timeout errors, prefer retry with increased timeout
                filtered_actions.sort_by_key(|action| match action {
                    RecoveryAction::FixAndRetry { .. } => 0,
                    RecoveryAction::Retry { .. } => 1,
                    _ => 2,
                });
            }
            _ => {
                // Default prioritization
                filtered_actions.sort_by_key(|action| match action {
                    RecoveryAction::Retry { .. } => 0,
                    RecoveryAction::FixAndRetry { .. } => 1,
                    RecoveryAction::FallbackTool { .. } => 2,
                    RecoveryAction::SkipAndContinue { .. } => 3,
                    RecoveryAction::UserIntervention { .. } => 4,
                    RecoveryAction::AbortChain { .. } => 5,
                });
            }
        }

        filtered_actions
    }
}

impl Default for ToolRecoveryManager {
    fn default() -> Self {
        let mut manager = Self::new();
        
        // Register default strategies
        manager.register_strategy(Arc::new(DefaultRecoveryStrategy::new()));
        manager.register_strategy(Arc::new(FileOperationRecoveryStrategy::new()));
        manager.register_strategy(Arc::new(TimeoutRecoveryStrategy::new()));
        
        manager
    }
}

/// Configuration for recovery behavior
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub max_recovery_attempts: u32,
    pub max_actions_per_error: usize,
    pub escalation_threshold: u32,
    pub max_history_size: usize,
    pub enable_automatic_fixes: bool,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_recovery_attempts: 3,
            max_actions_per_error: 3,
            escalation_threshold: 2,
            max_history_size: 100,
            enable_automatic_fixes: true,
        }
    }
}

/// Result of executing a recovery action
#[derive(Debug, Clone)]
pub enum RecoveryResult {
    Retry { modified_context: Option<ToolExecutionContext> },
    FallbackTool { tool_name: String, input: serde_json::Value, reason: String },
    UserIntervention { message: String, suggested_actions: Vec<String> },
    Skip { reason: String },
    Abort { reason: String },
}

/// Record of a recovery attempt for analysis and learning
#[derive(Debug, Clone)]
struct RecoveryAttempt {
    execution_id: String,
    error: ToolError,
    actions: Vec<RecoveryAction>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Default recovery strategy with basic error handling
#[derive(Debug)]
pub struct DefaultRecoveryStrategy {
    max_retries: u32,
}

impl DefaultRecoveryStrategy {
    pub fn new() -> Self {
        Self { max_retries: 3 }
    }
}

#[async_trait]
impl RecoveryStrategy for DefaultRecoveryStrategy {
    async fn suggest_recovery(
        &self,
        error: &ToolError,
        _context: &ToolExecutionContext,
        attempt_count: u32,
    ) -> ClaudeResult<Vec<RecoveryAction>> {
        let mut actions = Vec::new();

        if attempt_count < self.max_retries {
            // Suggest retry with exponential backoff
            let delay = Duration::from_secs(2_u64.pow(attempt_count));
            actions.push(RecoveryAction::Retry {
                delay,
                max_attempts: self.max_retries,
                modified_input: None,
            });
        }

        // Always provide user intervention as an option
        actions.push(RecoveryAction::UserIntervention {
            message: format!("Tool execution failed: {}", error.message),
            suggested_actions: error.recovery_suggestions.clone(),
            blocking: false,
        });

        Ok(actions)
    }

    fn can_handle(&self, _error: &ToolError) -> bool {
        true // Can handle any error as a fallback
    }

    fn name(&self) -> &str {
        "default"
    }

    fn priority(&self) -> u32 {
        1 // Low priority as this is the fallback
    }
}

/// Specialized recovery strategy for file operations
#[derive(Debug)]
pub struct FileOperationRecoveryStrategy;

impl FileOperationRecoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RecoveryStrategy for FileOperationRecoveryStrategy {
    async fn suggest_recovery(
        &self,
        error: &ToolError,
        context: &ToolExecutionContext,
        attempt_count: u32,
    ) -> ClaudeResult<Vec<RecoveryAction>> {
        let mut actions = Vec::new();

        match error.error_type {
            ToolErrorType::PermissionError => {
                actions.push(RecoveryAction::UserIntervention {
                    message: "File permission error detected. Please check file permissions or update whitelist configuration.".to_string(),
                    suggested_actions: vec![
                        "Check file permissions".to_string(),
                        "Update whitelist configuration".to_string(),
                        "Try with a different file path".to_string(),
                        "Run with appropriate privileges".to_string(),
                    ],
                    blocking: false,
                });

                // Suggest automatic fix if path is in input
                if let Some(path) = context.input.get("path").and_then(|p| p.as_str()) {
                    actions.push(RecoveryAction::FixAndRetry {
                        fix_description: "Attempt to adjust file permissions".to_string(),
                        fix_actions: vec![FixAction::ChangePermissions {
                            path: path.to_string(),
                            permissions: "644".to_string(),
                        }],
                        retry_delay: Duration::from_secs(1),
                    });
                }
            }
            ToolErrorType::ValidationError => {
                // Check if it's a missing directory issue
                if error.message.contains("No such file or directory") || error.message.contains("not found") {
                    if let Some(path) = context.input.get("path").and_then(|p| p.as_str()) {
                        if let Some(parent) = std::path::Path::new(path).parent() {
                            actions.push(RecoveryAction::FixAndRetry {
                                fix_description: "Create missing parent directory".to_string(),
                                fix_actions: vec![FixAction::CreateDirectory {
                                    path: parent.to_string_lossy().to_string(),
                                }],
                                retry_delay: Duration::from_millis(500),
                            });
                        }
                    }
                }
            }
            _ => {
                // For other file operation errors, suggest retry with delay
                if attempt_count < 2 {
                    actions.push(RecoveryAction::Retry {
                        delay: Duration::from_secs(2),
                        max_attempts: 3,
                        modified_input: None,
                    });
                }
            }
        }

        Ok(actions)
    }

    fn can_handle(&self, _error: &ToolError) -> bool {
        // This strategy can handle any error, but it's specifically designed for file operations
        true
    }

    fn name(&self) -> &str {
        "file_operations"
    }

    fn priority(&self) -> u32 {
        75 // High priority for file operation errors
    }
}

/// Specialized recovery strategy for timeout errors
#[derive(Debug)]
pub struct TimeoutRecoveryStrategy;

impl TimeoutRecoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RecoveryStrategy for TimeoutRecoveryStrategy {
    async fn suggest_recovery(
        &self,
        _error: &ToolError,
        _context: &ToolExecutionContext,
        attempt_count: u32,
    ) -> ClaudeResult<Vec<RecoveryAction>> {
        let mut actions = Vec::new();

        if attempt_count < 2 {
            // Suggest retry with increased timeout
            actions.push(RecoveryAction::FixAndRetry {
                fix_description: "Increase timeout and retry".to_string(),
                fix_actions: vec![FixAction::IncreaseTimeout {
                    new_timeout: Duration::from_secs(60 * (attempt_count + 2) as u64),
                }],
                retry_delay: Duration::from_secs(5),
            });
        } else {
            // After multiple timeout failures, suggest user intervention
            actions.push(RecoveryAction::UserIntervention {
                message: "Operation consistently timing out. The task may be too complex or the system may be overloaded.".to_string(),
                suggested_actions: vec![
                    "Break down the task into smaller parts".to_string(),
                    "Check system resources".to_string(),
                    "Try again later".to_string(),
                    "Use a different approach".to_string(),
                ],
                blocking: false,
            });
        }

        Ok(actions)
    }

    fn can_handle(&self, error: &ToolError) -> bool {
        matches!(error.error_type, ToolErrorType::TimeoutError)
    }

    fn name(&self) -> &str {
        "timeout"
    }

    fn priority(&self) -> u32 {
        90 // Very high priority for timeout errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::whitelist::WhitelistConfig;

    fn create_test_context() -> ToolExecutionContext {
        ToolExecutionContext::new(
            "test_tool".to_string(),
            serde_json::json!({"path": "/test/file.txt"}),
            Arc::new(RwLock::new(WhitelistConfig::default())),
        )
    }

    #[tokio::test]
    async fn test_recovery_manager_basic_flow() {
        let manager = ToolRecoveryManager::default();
        let error = ToolError::validation_error("Test error");
        let context = create_test_context();

        let actions = manager.suggest_recovery(&error, &context).await.unwrap();
        assert!(!actions.is_empty());
        
        // Should include at least a retry or user intervention
        assert!(actions.iter().any(|action| 
            matches!(action, RecoveryAction::Retry { .. } | RecoveryAction::UserIntervention { .. })
        ));
    }

    #[tokio::test]
    async fn test_file_operation_recovery_strategy() {
        let strategy = FileOperationRecoveryStrategy::new();
        let permission_error = ToolError {
            error_type: ToolErrorType::PermissionError,
            message: "Permission denied".to_string(),
            details: None,
            recovery_suggestions: vec![],
        };
        let context = create_test_context();

        let actions = strategy.suggest_recovery(&permission_error, &context, 0).await.unwrap();
        assert!(!actions.is_empty());
        
        // Should suggest user intervention for permission errors
        assert!(actions.iter().any(|action| 
            matches!(action, RecoveryAction::UserIntervention { .. })
        ));
    }

    #[tokio::test]
    async fn test_timeout_recovery_strategy() {
        let strategy = TimeoutRecoveryStrategy::new();
        let timeout_error = ToolError {
            error_type: ToolErrorType::TimeoutError,
            message: "Operation timed out".to_string(),
            details: None,
            recovery_suggestions: vec![],
        };
        let context = create_test_context();

        let actions = strategy.suggest_recovery(&timeout_error, &context, 0).await.unwrap();
        assert!(!actions.is_empty());
        
        // Should suggest fix and retry for timeout errors
        assert!(actions.iter().any(|action| 
            matches!(action, RecoveryAction::FixAndRetry { .. })
        ));
    }

    #[tokio::test]
    async fn test_recovery_action_execution() {
        let manager = ToolRecoveryManager::new();
        let context = create_test_context();

        let retry_action = RecoveryAction::Retry {
            delay: Duration::from_millis(10),
            max_attempts: 3,
            modified_input: None,
        };

        let result = manager.execute_recovery_action(&retry_action, &context).await.unwrap();
        
        match result {
            RecoveryResult::Retry { .. } => (),
            _ => panic!("Expected retry result"),
        }
    }

    #[test]
    fn test_recovery_action_properties() {
        let user_intervention = RecoveryAction::UserIntervention {
            message: "Test".to_string(),
            suggested_actions: vec![],
            blocking: true,
        };
        assert!(user_intervention.requires_user_interaction());
        assert!(user_intervention.stops_execution());

        let retry_action = RecoveryAction::Retry {
            delay: Duration::from_secs(5),
            max_attempts: 3,
            modified_input: None,
        };
        assert!(!retry_action.requires_user_interaction());
        assert!(!retry_action.stops_execution());
        assert_eq!(retry_action.estimated_delay(), Duration::from_secs(5));
    }

    #[test]
    fn test_fix_actions() {
        let modify_input = FixAction::ModifyInput {
            parameter: "timeout".to_string(),
            new_value: serde_json::json!(60),
            reason: "Increase timeout".to_string(),
        };

        let increase_timeout = FixAction::IncreaseTimeout {
            new_timeout: Duration::from_secs(120),
        };

        let create_dir = FixAction::CreateDirectory {
            path: "/tmp/test".to_string(),
        };

        // Test that fix actions can be created and serialized
        let json = serde_json::to_string(&modify_input).unwrap();
        assert!(json.contains("timeout"));

        let json = serde_json::to_string(&increase_timeout).unwrap();
        assert!(json.contains("IncreaseTimeout"));

        let json = serde_json::to_string(&create_dir).unwrap();
        assert!(json.contains("/tmp/test"));
    }

    #[tokio::test]
    async fn test_recovery_attempt_limiting() {
        let mut manager = ToolRecoveryManager::new();
        manager.config.max_recovery_attempts = 2;

        let error = ToolError::validation_error("Test error");
        let context = create_test_context();

        // Record multiple attempts
        for _ in 0..3 {
            manager.record_recovery_attempt(&context.execution_id, &error, &[]).await;
        }

        let actions = manager.suggest_recovery(&error, &context).await.unwrap();
        
        // Should suggest abort due to too many attempts
        assert!(actions.iter().any(|action| 
            matches!(action, RecoveryAction::AbortChain { .. })
        ));
    }
}