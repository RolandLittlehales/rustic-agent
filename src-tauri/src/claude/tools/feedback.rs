use crate::claude::{
    error::ClaudeResult,
    tools::execution::{FollowUpAction, StatusLevel, ToolExecutionResult, ToolError, ToolErrorType},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for handling tool execution results and generating follow-up actions
#[async_trait]
pub trait ToolFeedbackHandler: Send + Sync + std::fmt::Debug {
    /// Process a tool execution result and generate follow-up actions
    async fn handle_result(&self, result: &ToolExecutionResult) -> ClaudeResult<Vec<FollowUpAction>>;
    
    /// Handle tool execution errors and suggest recovery actions
    async fn handle_error(&self, error: &ToolError, execution_id: &str) -> ClaudeResult<Vec<FollowUpAction>>;
    
    /// Get the priority of this feedback handler (higher numbers = higher priority)
    fn priority(&self) -> u32 {
        0
    }
    
    /// Check if this handler can process the given tool result
    fn can_handle(&self, result: &ToolExecutionResult) -> bool;
}

/// Main feedback manager that orchestrates multiple feedback handlers
#[derive(Debug)]
pub struct FeedbackManager {
    handlers: Vec<Arc<dyn ToolFeedbackHandler>>,
    execution_history: Arc<RwLock<Vec<ToolExecutionResult>>>,
    feedback_rules: FeedbackRules,
}

impl FeedbackManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            execution_history: Arc::new(RwLock::new(Vec::new())),
            feedback_rules: FeedbackRules::default(),
        }
    }

    pub fn with_rules(mut self, rules: FeedbackRules) -> Self {
        self.feedback_rules = rules;
        self
    }

    /// Register a feedback handler
    pub fn register_handler(&mut self, handler: Arc<dyn ToolFeedbackHandler>) {
        self.handlers.push(handler);
        // Sort by priority (highest first)
        self.handlers.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Process a tool result through all applicable handlers
    pub async fn process_result(&self, result: &ToolExecutionResult) -> ClaudeResult<Vec<FollowUpAction>> {
        // Store result in history
        {
            let mut history = self.execution_history.write().await;
            history.push(result.clone());
            
            // Limit history size to prevent memory issues
            let max_size = self.feedback_rules.max_history_size;
            if history.len() > max_size {
                let excess = history.len() - max_size;
                history.drain(0..excess);
            }
        }

        let mut all_actions = Vec::new();

        // Process through all handlers that can handle this result
        for handler in &self.handlers {
            if handler.can_handle(result) {
                match handler.handle_result(result).await {
                    Ok(mut actions) => {
                        all_actions.append(&mut actions);
                    }
                    Err(e) => {
                        eprintln!("Warning: Feedback handler failed: {}", e);
                        // Continue with other handlers even if one fails
                    }
                }
            }
        }

        // Apply feedback rules to filter and prioritize actions
        Ok(self.apply_feedback_rules(all_actions, result))
    }

    /// Process an error through all handlers
    pub async fn process_error(&self, error: &ToolError, execution_id: &str) -> ClaudeResult<Vec<FollowUpAction>> {
        let mut all_actions = Vec::new();

        for handler in &self.handlers {
            match handler.handle_error(error, execution_id).await {
                Ok(mut actions) => {
                    all_actions.append(&mut actions);
                }
                Err(e) => {
                    eprintln!("Warning: Error feedback handler failed: {}", e);
                }
            }
        }

        Ok(all_actions)
    }

    /// Get execution history for analysis
    pub async fn get_execution_history(&self) -> Vec<ToolExecutionResult> {
        self.execution_history.read().await.clone()
    }

    /// Clear execution history
    pub async fn clear_history(&self) {
        self.execution_history.write().await.clear();
    }

    /// Apply feedback rules to filter and organize actions
    fn apply_feedback_rules(&self, actions: Vec<FollowUpAction>, result: &ToolExecutionResult) -> Vec<FollowUpAction> {
        let mut filtered_actions = actions;

        // Limit total actions
        if filtered_actions.len() > self.feedback_rules.max_actions_per_result {
            filtered_actions.truncate(self.feedback_rules.max_actions_per_result);
        }

        // Filter based on result type
        if result.is_error() && !self.feedback_rules.allow_actions_on_error {
            filtered_actions.retain(|action| {
                matches!(action, FollowUpAction::ReportStatus { .. })
            });
        }

        // Sort actions by priority (highest priority first)
        filtered_actions.sort_by(|a, b| self.action_priority(b).cmp(&self.action_priority(a)));

        filtered_actions
    }

    fn action_priority(&self, action: &FollowUpAction) -> u32 {
        match action {
            FollowUpAction::ReportStatus { level, .. } => match level {
                StatusLevel::Error => 100,
                StatusLevel::Warning => 50,
                StatusLevel::Info => 10,
            },
            FollowUpAction::RequestUserInput { .. } => 80,
            FollowUpAction::RetryWithModifiedInput { .. } => 60,
            FollowUpAction::ExecuteTool { .. } => 40,
        }
    }
}

impl Default for FeedbackManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for feedback processing
#[derive(Debug, Clone)]
pub struct FeedbackRules {
    pub max_actions_per_result: usize,
    pub max_history_size: usize,
    pub allow_actions_on_error: bool,
    pub enable_chain_optimization: bool,
    pub max_chain_depth: u32,
}

impl Default for FeedbackRules {
    fn default() -> Self {
        Self {
            max_actions_per_result: 5,
            max_history_size: 100,
            allow_actions_on_error: true,
            enable_chain_optimization: true,
            max_chain_depth: 10,
        }
    }
}

/// Default feedback handler for common tool result patterns
#[derive(Debug)]
pub struct DefaultFeedbackHandler {
    tool_patterns: HashMap<String, Vec<FeedbackPattern>>,
}

impl DefaultFeedbackHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            tool_patterns: HashMap::new(),
        };
        handler.initialize_default_patterns();
        handler
    }

    fn initialize_default_patterns(&mut self) {
        // File operation patterns
        let file_patterns = vec![
            FeedbackPattern {
                condition: PatternCondition::SuccessWithWarnings,
                action_template: FollowUpAction::ReportStatus {
                    message: "File operation completed with warnings".to_string(),
                    level: StatusLevel::Warning,
                },
            },
            FeedbackPattern {
                condition: PatternCondition::Error(ToolErrorType::PermissionError),
                action_template: FollowUpAction::RequestUserInput {
                    prompt: "File permission error occurred. Please check file permissions or update whitelist.".to_string(),
                    suggested_actions: vec![
                        "Check file permissions".to_string(),
                        "Update whitelist configuration".to_string(),
                        "Try with different file path".to_string(),
                    ],
                },
            },
        ];

        self.tool_patterns.insert("read_file".to_string(), file_patterns.clone());
        self.tool_patterns.insert("write_file".to_string(), file_patterns.clone());
        self.tool_patterns.insert("list_directory".to_string(), file_patterns);

        // Add more patterns for other tools as needed
    }

    fn find_matching_patterns(&self, result: &ToolExecutionResult) -> Vec<&FeedbackPattern> {
        let patterns = self.tool_patterns.get(&result.tool_name);
        if let Some(patterns) = patterns {
            patterns.iter()
                .filter(|pattern| pattern.matches(result))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for DefaultFeedbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolFeedbackHandler for DefaultFeedbackHandler {
    async fn handle_result(&self, result: &ToolExecutionResult) -> ClaudeResult<Vec<FollowUpAction>> {
        let matching_patterns = self.find_matching_patterns(result);
        let mut actions = Vec::new();

        for pattern in matching_patterns {
            let action = pattern.generate_action(result);
            actions.push(action);
        }

        // Add default success reporting if no other actions
        if actions.is_empty() && result.is_success() {
            actions.push(FollowUpAction::ReportStatus {
                message: format!("Tool '{}' executed successfully", result.tool_name),
                level: StatusLevel::Info,
            });
        }

        Ok(actions)
    }

    async fn handle_error(&self, error: &ToolError, _execution_id: &str) -> ClaudeResult<Vec<FollowUpAction>> {
        let mut actions = Vec::new();

        // Generate recovery actions based on error type
        match error.error_type {
            ToolErrorType::ValidationError => {
                actions.push(FollowUpAction::ReportStatus {
                    message: format!("Validation error: {}", error.message),
                    level: StatusLevel::Error,
                });
            }
            ToolErrorType::PermissionError => {
                actions.push(FollowUpAction::RequestUserInput {
                    prompt: format!("Permission error: {}", error.message),
                    suggested_actions: error.recovery_suggestions.clone(),
                });
            }
            ToolErrorType::TimeoutError => {
                actions.push(FollowUpAction::RetryWithModifiedInput {
                    modified_input: serde_json::json!({"timeout": 60}),
                    delay: Some(std::time::Duration::from_secs(5)),
                });
            }
            _ => {
                actions.push(FollowUpAction::ReportStatus {
                    message: format!("Tool error: {}", error.message),
                    level: StatusLevel::Error,
                });
            }
        }

        Ok(actions)
    }

    fn can_handle(&self, result: &ToolExecutionResult) -> bool {
        // This default handler can handle any result
        self.tool_patterns.contains_key(&result.tool_name) || true
    }

    fn priority(&self) -> u32 {
        1 // Low priority as this is the default handler
    }
}

/// Pattern-based feedback configuration
#[derive(Debug, Clone)]
pub struct FeedbackPattern {
    condition: PatternCondition,
    action_template: FollowUpAction,
}

impl FeedbackPattern {
    pub fn new(condition: PatternCondition, action_template: FollowUpAction) -> Self {
        Self {
            condition,
            action_template,
        }
    }

    pub fn matches(&self, result: &ToolExecutionResult) -> bool {
        self.condition.matches(result)
    }

    pub fn generate_action(&self, _result: &ToolExecutionResult) -> FollowUpAction {
        // For now, just clone the template. In a more sophisticated implementation,
        // this could customize the action based on the specific result
        self.action_template.clone()
    }
}

#[derive(Debug, Clone)]
pub enum PatternCondition {
    Success,
    SuccessWithWarnings,
    Error(ToolErrorType),
    AnyError,
    Timeout,
    Custom(String), // For future extensibility
}

impl PatternCondition {
    pub fn matches(&self, result: &ToolExecutionResult) -> bool {
        match self {
            PatternCondition::Success => result.is_success() && !self.has_warnings(result),
            PatternCondition::SuccessWithWarnings => result.is_success() && self.has_warnings(result),
            PatternCondition::Error(error_type) => {
                if let Some(error_context) = &result.error_context {
                    // Simple pattern matching - in practice this would be more sophisticated
                    let error_type_str = format!("{:?}", error_type);
                    error_context.contains(&error_type_str)
                } else {
                    false
                }
            }
            PatternCondition::AnyError => result.is_error(),
            PatternCondition::Timeout => matches!(result.status, crate::claude::tools::execution::ToolExecutionStatus::Timeout),
            PatternCondition::Custom(_) => false, // Not implemented yet
        }
    }

    fn has_warnings(&self, result: &ToolExecutionResult) -> bool {
        match &result.status {
            crate::claude::tools::execution::ToolExecutionStatus::PartialSuccess { warnings } => !warnings.is_empty(),
            _ => !result.metadata.warnings.is_empty(),
        }
    }
}

/// Smart feedback handler that learns from execution patterns
#[derive(Debug)]  
pub struct SmartFeedbackHandler {
    execution_patterns: Arc<RwLock<HashMap<String, ExecutionPattern>>>,
    learning_enabled: bool,
}

impl SmartFeedbackHandler {
    pub fn new() -> Self {
        Self {
            execution_patterns: Arc::new(RwLock::new(HashMap::new())),
            learning_enabled: true,
        }
    }

    pub fn with_learning(mut self, enabled: bool) -> Self {
        self.learning_enabled = enabled;
        self
    }

    async fn learn_from_result(&self, result: &ToolExecutionResult) {
        if !self.learning_enabled {
            return;
        }

        let mut patterns = self.execution_patterns.write().await;
        let pattern = patterns.entry(result.tool_name.clone())
            .or_insert_with(|| ExecutionPattern::new(result.tool_name.clone()));
        
        pattern.add_execution(result);
    }

    async fn suggest_optimizations(&self, result: &ToolExecutionResult) -> Vec<FollowUpAction> {
        let patterns = self.execution_patterns.read().await;
        if let Some(pattern) = patterns.get(&result.tool_name) {
            pattern.suggest_optimizations(result)
        } else {
            Vec::new()
        }
    }
}

#[async_trait]
impl ToolFeedbackHandler for SmartFeedbackHandler {
    async fn handle_result(&self, result: &ToolExecutionResult) -> ClaudeResult<Vec<FollowUpAction>> {
        // Learn from this execution
        self.learn_from_result(result).await;

        // Generate optimization suggestions
        let optimizations = self.suggest_optimizations(result).await;

        Ok(optimizations)
    }

    async fn handle_error(&self, error: &ToolError, _execution_id: &str) -> ClaudeResult<Vec<FollowUpAction>> {
        // For now, provide basic error handling
        Ok(vec![FollowUpAction::ReportStatus {
            message: format!("Smart handler detected error: {}", error.message),
            level: StatusLevel::Warning,
        }])
    }

    fn can_handle(&self, _result: &ToolExecutionResult) -> bool {
        true // Can handle any result for learning purposes
    }

    fn priority(&self) -> u32 {
        10 // Higher priority than default handler
    }
}

impl Default for SmartFeedbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution pattern for learning and optimization
#[derive(Debug, Clone)]
struct ExecutionPattern {
    tool_name: String,
    execution_count: u32,
    average_execution_time: std::time::Duration,
    common_errors: HashMap<String, u32>,
    success_rate: f64,
}

impl ExecutionPattern {
    fn new(tool_name: String) -> Self {
        Self {
            tool_name,
            execution_count: 0,
            average_execution_time: std::time::Duration::from_millis(0),
            common_errors: HashMap::new(),
            success_rate: 0.0,
        }
    }

    fn add_execution(&mut self, result: &ToolExecutionResult) {
        self.execution_count += 1;
        
        // Update average execution time
        let new_time = result.metadata.execution_time;
        let total_time = self.average_execution_time * (self.execution_count - 1) + new_time;
        self.average_execution_time = total_time / self.execution_count;

        // Track errors
        if result.is_error() {
            if let Some(error_context) = &result.error_context {
                *self.common_errors.entry(error_context.clone()).or_insert(0) += 1;
            }
        }

        // Update success rate
        let success_count = if result.is_success() { 1.0 } else { 0.0 };
        self.success_rate = (self.success_rate * (self.execution_count - 1) as f64 + success_count) / self.execution_count as f64;
    }

    fn suggest_optimizations(&self, result: &ToolExecutionResult) -> Vec<FollowUpAction> {
        let mut suggestions = Vec::new();

        // Suggest timeout adjustments if execution time is consistently high
        if result.metadata.execution_time > self.average_execution_time * 2 {
            suggestions.push(FollowUpAction::ReportStatus {
                message: format!("Tool '{}' took longer than average. Consider optimizing.", self.tool_name),
                level: StatusLevel::Info,
            });
        }

        // Suggest retry if success rate is low
        if self.success_rate < 0.8 && result.is_error() {
            suggestions.push(FollowUpAction::RetryWithModifiedInput {
                modified_input: serde_json::json!({"retry_reason": "low_success_rate"}),
                delay: Some(std::time::Duration::from_secs(2)),
            });
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::tools::execution::{ToolExecutionResult, ToolResultData};

    #[tokio::test]
    async fn test_feedback_manager_basic_flow() {
        let mut manager = FeedbackManager::new();
        let handler = Arc::new(DefaultFeedbackHandler::new());
        manager.register_handler(handler);

        let result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "read_file".to_string(),
            ToolResultData::text("File contents"),
        );

        let actions = manager.process_result(&result).await.unwrap();
        assert!(!actions.is_empty());
        
        // Should have at least a status report
        assert!(actions.iter().any(|action| matches!(action, FollowUpAction::ReportStatus { .. })));
    }

    #[tokio::test]
    async fn test_pattern_matching() {
        let success_pattern = FeedbackPattern::new(
            PatternCondition::Success,
            FollowUpAction::ReportStatus {
                message: "Success!".to_string(),
                level: StatusLevel::Info,
            },
        );

        let success_result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "test_tool".to_string(),
            ToolResultData::text("result"),
        );

        assert!(success_pattern.matches(&success_result));

        let error_result = ToolExecutionResult::failure(
            "exec_456".to_string(),
            "test_tool".to_string(),
            ToolError::validation_error("Test error"),
            true,
        );

        assert!(!success_pattern.matches(&error_result));
    }

    #[tokio::test]
    async fn test_smart_feedback_handler() {
        let handler = SmartFeedbackHandler::new();
        
        let result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "test_tool".to_string(),
            ToolResultData::text("result"),
        );

        let actions = handler.handle_result(&result).await.unwrap();
        // Smart handler may or may not generate actions initially
        assert!(actions.len() <= 5); // Should be within reasonable bounds
    }

    #[test]
    fn test_feedback_rules_application() {
        let manager = FeedbackManager::new();
        let result = ToolExecutionResult::success(
            "exec_123".to_string(),
            "test_tool".to_string(),
            ToolResultData::text("result"),
        );

        let actions = vec![
            FollowUpAction::ReportStatus { message: "Status 1".to_string(), level: StatusLevel::Info },
            FollowUpAction::ReportStatus { message: "Status 2".to_string(), level: StatusLevel::Warning },
            FollowUpAction::ReportStatus { message: "Status 3".to_string(), level: StatusLevel::Error },
        ];

        let filtered = manager.apply_feedback_rules(actions, &result);
        assert_eq!(filtered.len(), 3);
        
        // Should be sorted by priority (Error first)
        if let FollowUpAction::ReportStatus { level, .. } = &filtered[0] {
            assert!(matches!(level, StatusLevel::Error));
        }
    }

    #[test]
    fn test_execution_pattern_learning() {
        let mut pattern = ExecutionPattern::new("test_tool".to_string());
        
        let result1 = ToolExecutionResult::success(
            "exec_1".to_string(),
            "test_tool".to_string(),
            ToolResultData::text("result"),
        );
        
        pattern.add_execution(&result1);
        assert_eq!(pattern.execution_count, 1);
        assert!(pattern.success_rate > 0.9);

        let result2 = ToolExecutionResult::failure(
            "exec_2".to_string(),
            "test_tool".to_string(),
            ToolError::validation_error("Test error"),
            true,
        );
        
        pattern.add_execution(&result2);
        assert_eq!(pattern.execution_count, 2);
        assert!(pattern.success_rate < 0.9); // Should decrease due to failure
    }
}