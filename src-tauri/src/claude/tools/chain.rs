use crate::claude::{
    error::{ClaudeError, ClaudeResult, ErrorContext},
    tools::{
        execution::{
            ToolError, ToolErrorType, ToolExecutionContext, ToolExecutionResult, ToolResultData,
        },
        feedback::FeedbackManager,
        recovery::{RecoveryResult, ToolRecoveryManager},
        AgentTool,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Request for tool execution with dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub id: String,
    pub tool_name: String,
    pub input: serde_json::Value,
    pub depends_on: Vec<String>,
    pub timeout: Option<Duration>,
    pub max_retries: Option<u32>,
    pub metadata: HashMap<String, String>,
}

impl ToolRequest {
    pub fn new(tool_name: String, input: serde_json::Value) -> Self {
        Self {
            id: Self::generate_id(),
            tool_name,
            input,
            depends_on: Vec::new(),
            timeout: None,
            max_retries: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_dependency(mut self, dependency_id: String) -> Self {
        self.depends_on.push(dependency_id);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    fn generate_id() -> String {
        use uuid::Uuid;
        format!("req_{}", Uuid::new_v4().simple())
    }
}

/// Status of tool chain execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainExecutionStatus {
    Pending,
    Running {
        current_step: String,
        progress: f32,
    },
    Completed {
        total_time: Duration,
        results_count: usize,
    },
    Failed {
        error: String,
        partial_results: usize,
    },
    Cancelled {
        reason: String,
    },
}

/// Result of executing a tool chain
#[derive(Debug, Clone)]
pub struct ChainExecutionResult {
    pub chain_id: String,
    pub status: ChainExecutionStatus,
    pub results: HashMap<String, ToolExecutionResult>,
    pub execution_order: Vec<String>,
    pub total_time: Duration,
    pub metadata: ChainMetadata,
}

impl ChainExecutionResult {
    pub fn is_success(&self) -> bool {
        matches!(self.status, ChainExecutionStatus::Completed { .. })
    }

    pub fn get_final_result(&self) -> Option<&ToolExecutionResult> {
        self.execution_order
            .last()
            .and_then(|id| self.results.get(id))
    }

    pub fn get_all_results(&self) -> Vec<&ToolExecutionResult> {
        self.execution_order
            .iter()
            .filter_map(|id| self.results.get(id))
            .collect()
    }

    pub fn get_errors(&self) -> Vec<&ToolExecutionResult> {
        self.results
            .values()
            .filter(|result| result.is_error())
            .collect()
    }
}

/// Metadata about chain execution
#[derive(Debug, Clone)]
pub struct ChainMetadata {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub parallel_executions: u32,
    pub total_retries: u32,
    pub recovery_actions_taken: u32,
    pub performance_metrics: PerformanceMetrics,
}

impl Default for ChainMetadata {
    fn default() -> Self {
        Self {
            started_at: chrono::Utc::now(),
            completed_at: None,
            parallel_executions: 0,
            total_retries: 0,
            recovery_actions_taken: 0,
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

/// Performance metrics for chain execution analysis
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub average_tool_execution_time: Duration,
    pub longest_execution_time: Duration,
    pub shortest_execution_time: Duration,
    pub parallel_efficiency: f32,
    pub memory_peak: Option<u64>,
}

/// Main tool execution engine that handles chains and dependencies
#[derive(Debug)]
pub struct ToolExecutionEngine {
    tools: HashMap<String, Arc<dyn AgentTool>>,
    feedback_manager: FeedbackManager,
    recovery_manager: ToolRecoveryManager,
    execution_history: Arc<RwLock<Vec<ChainExecutionResult>>>,
    config: ExecutionConfig,
}

impl ToolExecutionEngine {
    pub fn new() -> Self {
        let mut feedback_manager = FeedbackManager::new();

        // Register default feedback handlers
        feedback_manager.register_handler(Arc::new(
            crate::claude::tools::feedback::DefaultFeedbackHandler::new(),
        ));

        Self {
            tools: HashMap::new(),
            feedback_manager,
            recovery_manager: ToolRecoveryManager::default(),
            execution_history: Arc::new(RwLock::new(Vec::new())),
            config: ExecutionConfig::default(),
        }
    }

    pub fn with_config(mut self, config: ExecutionConfig) -> Self {
        self.config = config;
        self
    }

    /// Register a tool with the execution engine
    pub fn register_tool(&mut self, tool: Arc<dyn AgentTool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    /// Execute a single tool
    pub async fn execute_single_tool(
        &self,
        request: ToolRequest,
        context: ToolExecutionContext,
    ) -> ClaudeResult<ToolExecutionResult> {
        let start_time = Instant::now();

        let tool = self
            .tools
            .get(&request.tool_name)
            .ok_or_else(|| ClaudeError::ToolError {
                tool_name: request.tool_name.clone(),
                message: format!("Tool '{}' not found", request.tool_name),
                context: Some(
                    ErrorContext::new("tool_execution")
                        .add_metadata("tool_name", &request.tool_name),
                ),
            })?;

        let mut attempt_count = 0;
        let max_retries = request
            .max_retries
            .unwrap_or(self.config.default_max_retries);

        loop {
            // Check for timeout
            if context.is_timeout() {
                return Ok(ToolExecutionResult::timeout(
                    context.execution_id.clone(),
                    request.tool_name.clone(),
                    context.elapsed(),
                ));
            }

            match tool.execute(context.input.clone()).await {
                Ok(result_string) => {
                    let _execution_time = start_time.elapsed();
                    let result = ToolExecutionResult::success(
                        context.execution_id.clone(),
                        request.tool_name.clone(),
                        ToolResultData::text(result_string),
                    );

                    // Process result through feedback manager
                    if let Ok(follow_up_actions) =
                        self.feedback_manager.process_result(&result).await
                    {
                        let result_with_actions = result.with_follow_up_actions(follow_up_actions);
                        return Ok(result_with_actions);
                    }

                    return Ok(result);
                }
                Err(e) => {
                    let tool_error = ToolError {
                        error_type: ToolErrorType::ExecutionError,
                        message: e.to_string(),
                        details: Some(format!(
                            "Tool: {}, Attempt: {}",
                            request.tool_name,
                            attempt_count + 1
                        )),
                        recovery_suggestions: vec![
                            "Check input parameters".to_string(),
                            "Verify tool configuration".to_string(),
                        ],
                    };

                    // Try recovery if retries are available
                    if attempt_count < max_retries {
                        if let Ok(recovery_actions) = self
                            .recovery_manager
                            .suggest_recovery(&tool_error, &context)
                            .await
                        {
                            for action in recovery_actions {
                                match self
                                    .recovery_manager
                                    .execute_recovery_action(&action, &context)
                                    .await
                                {
                                    Ok(RecoveryResult::Retry { modified_context }) => {
                                        attempt_count += 1;
                                        if let Some(_new_context) = modified_context {
                                            // Use modified context for retry
                                            // In a full implementation, we'd update the context here
                                        }
                                        break; // Retry the execution
                                    }
                                    Ok(RecoveryResult::Abort { reason: _ }) => {
                                        return Ok(ToolExecutionResult::failure(
                                            context.execution_id.clone(),
                                            request.tool_name.clone(),
                                            tool_error,
                                            false,
                                        ));
                                    }
                                    Ok(RecoveryResult::Skip { reason: _ }) => {
                                        return Ok(ToolExecutionResult::failure(
                                            context.execution_id.clone(),
                                            request.tool_name.clone(),
                                            tool_error,
                                            false,
                                        ));
                                    }
                                    _ => continue, // Try next recovery action
                                }
                            }
                        } else {
                            attempt_count += 1;
                        }
                    } else {
                        // No more retries available
                        return Ok(ToolExecutionResult::failure(
                            context.execution_id.clone(),
                            request.tool_name.clone(),
                            tool_error,
                            false,
                        ));
                    }
                }
            }
        }
    }

    /// Execute a chain of tools with dependency management
    #[allow(dead_code)] // Reserved for advanced tool workflows
    pub async fn execute_tool_chain(
        &self,
        requests: Vec<ToolRequest>,
        whitelist: Arc<RwLock<crate::claude::whitelist::WhitelistConfig>>,
    ) -> ClaudeResult<ChainExecutionResult> {
        let chain_id = self.generate_chain_id();
        let start_time = Instant::now();
        let mut metadata = ChainMetadata::default();

        // Validate and sort requests by dependencies
        let execution_plan = self.create_execution_plan(&requests)?;

        let mut results: HashMap<String, ToolExecutionResult> = HashMap::new();
        let mut execution_order = Vec::new();
        let mut _current_status = ChainExecutionStatus::Running {
            current_step: "Starting".to_string(),
            progress: 0.0,
        };

        for (phase_index, phase) in execution_plan.phases.iter().enumerate() {
            let progress = (phase_index as f32) / (execution_plan.phases.len() as f32);
            _current_status = ChainExecutionStatus::Running {
                current_step: format!(
                    "Phase {} of {}",
                    phase_index + 1,
                    execution_plan.phases.len()
                ),
                progress,
            };

            // Execute phase (potentially in parallel)
            if phase.len() == 1 {
                // Single tool execution
                let request = &phase[0];
                let context = self.create_execution_context(request, whitelist.clone());

                match self.execute_single_tool(request.clone(), context).await {
                    Ok(result) => {
                        execution_order.push(request.id.clone());
                        results.insert(request.id.clone(), result);
                    }
                    Err(e) => {
                        // Handle chain failure
                        let total_time = start_time.elapsed();
                        metadata.completed_at = Some(chrono::Utc::now());

                        return Ok(ChainExecutionResult {
                            chain_id,
                            status: ChainExecutionStatus::Failed {
                                error: e.to_string(),
                                partial_results: results.len(),
                            },
                            results,
                            execution_order,
                            total_time,
                            metadata,
                        });
                    }
                }
            } else {
                // Parallel execution
                metadata.parallel_executions += 1;
                let parallel_results = self.execute_parallel_phase(phase, whitelist.clone()).await;

                for (request_id, result) in parallel_results {
                    match result {
                        Ok(tool_result) => {
                            execution_order.push(request_id.clone());
                            results.insert(request_id, tool_result);
                        }
                        Err(e) => {
                            // Handle parallel execution failure
                            let total_time = start_time.elapsed();
                            metadata.completed_at = Some(chrono::Utc::now());

                            return Ok(ChainExecutionResult {
                                chain_id,
                                status: ChainExecutionStatus::Failed {
                                    error: format!("Parallel execution failed: {}", e),
                                    partial_results: results.len(),
                                },
                                results,
                                execution_order,
                                total_time,
                                metadata,
                            });
                        }
                    }
                }
            }
        }

        // Chain completed successfully
        let total_time = start_time.elapsed();
        metadata.completed_at = Some(chrono::Utc::now());
        metadata.performance_metrics = self.calculate_performance_metrics(&results, total_time);

        let final_result = ChainExecutionResult {
            chain_id: chain_id.clone(),
            status: ChainExecutionStatus::Completed {
                total_time,
                results_count: results.len(),
            },
            results,
            execution_order,
            total_time,
            metadata,
        };

        // Store in execution history
        self.add_to_history(final_result.clone()).await;

        Ok(final_result)
    }

    async fn execute_parallel_phase(
        &self,
        phase: &[ToolRequest],
        whitelist: Arc<RwLock<crate::claude::whitelist::WhitelistConfig>>,
    ) -> Vec<(String, ClaudeResult<ToolExecutionResult>)> {
        // For now, execute sequentially to avoid lifetime issues
        // In a full implementation, we'd need to restructure to support true parallelism
        let mut results = Vec::new();

        for request in phase {
            let context = self.create_execution_context(request, whitelist.clone());
            let result = self.execute_single_tool(request.clone(), context).await;
            results.push((request.id.clone(), result));
        }

        results
    }

    fn create_execution_context(
        &self,
        request: &ToolRequest,
        whitelist: Arc<RwLock<crate::claude::whitelist::WhitelistConfig>>,
    ) -> ToolExecutionContext {
        let mut context =
            ToolExecutionContext::new(request.tool_name.clone(), request.input.clone(), whitelist);

        if let Some(timeout) = request.timeout {
            context = context.with_timeout(timeout);
        }

        if let Some(max_retries) = request.max_retries {
            context = context.with_max_retries(max_retries);
        }

        context
    }

    fn create_execution_plan(&self, requests: &[ToolRequest]) -> ClaudeResult<ExecutionPlan> {
        // Validate dependencies
        self.validate_dependencies(requests)?;

        // Topological sort to determine execution order
        let mut plan = ExecutionPlan { phases: Vec::new() };
        let mut remaining: HashSet<String> = requests.iter().map(|r| r.id.clone()).collect();
        let request_map: HashMap<String, &ToolRequest> =
            requests.iter().map(|r| (r.id.clone(), r)).collect();

        while !remaining.is_empty() {
            let mut current_phase = Vec::new();

            // Find all requests that have no pending dependencies
            let ready_requests: Vec<_> = remaining
                .iter()
                .filter(|&id| {
                    let request = request_map[id];
                    request
                        .depends_on
                        .iter()
                        .all(|dep| !remaining.contains(dep))
                })
                .cloned()
                .collect();

            if ready_requests.is_empty() {
                return Err(ClaudeError::ValidationError {
                    field: "dependencies".to_string(),
                    message: "Circular dependency detected in tool chain".to_string(),
                    context: Some(ErrorContext::new("chain_validation")),
                });
            }

            // Add ready requests to current phase
            for id in ready_requests {
                current_phase.push(request_map[&id].clone());
                remaining.remove(&id);
            }

            plan.phases.push(current_phase);
        }

        Ok(plan)
    }

    fn validate_dependencies(&self, requests: &[ToolRequest]) -> ClaudeResult<()> {
        let request_ids: HashSet<_> = requests.iter().map(|r| &r.id).collect();

        for request in requests {
            for dep in &request.depends_on {
                if !request_ids.contains(dep) {
                    return Err(ClaudeError::ValidationError {
                        field: "dependencies".to_string(),
                        message: format!("Dependency '{}' not found in request list", dep),
                        context: Some(
                            ErrorContext::new("dependency_validation")
                                .add_metadata("request_id", &request.id)
                                .add_metadata("missing_dependency", dep),
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    fn calculate_performance_metrics(
        &self,
        results: &HashMap<String, ToolExecutionResult>,
        total_time: Duration,
    ) -> PerformanceMetrics {
        if results.is_empty() {
            return PerformanceMetrics::default();
        }

        let execution_times: Vec<_> = results
            .values()
            .map(|r| r.metadata.execution_time)
            .collect();

        let total_execution_time: Duration = execution_times.iter().sum();
        let average_time = total_execution_time / execution_times.len() as u32;
        let longest_time = execution_times.iter().max().copied().unwrap_or_default();
        let shortest_time = execution_times.iter().min().copied().unwrap_or_default();

        // Calculate parallel efficiency (how much time we saved by running in parallel)
        let sequential_time = total_execution_time;
        let parallel_efficiency = if sequential_time > Duration::from_millis(0) {
            (sequential_time.as_millis() as f32 - total_time.as_millis() as f32)
                / sequential_time.as_millis() as f32
        } else {
            0.0
        };

        PerformanceMetrics {
            average_tool_execution_time: average_time,
            longest_execution_time: longest_time,
            shortest_execution_time: shortest_time,
            parallel_efficiency: parallel_efficiency.max(0.0),
            memory_peak: None, // Would need system monitoring to implement
        }
    }

    fn generate_chain_id(&self) -> String {
        use uuid::Uuid;
        format!("chain_{}", Uuid::new_v4().simple())
    }

    async fn add_to_history(&self, result: ChainExecutionResult) {
        let mut history = self.execution_history.write().await;
        history.push(result);

        // Limit history size
        let max_size = self.config.max_history_size;
        if history.len() > max_size {
            let excess = history.len() - max_size;
            history.drain(0..excess);
        }
    }

    /// Get execution history for analysis
    pub async fn get_execution_history(&self) -> Vec<ChainExecutionResult> {
        self.execution_history.read().await.clone()
    }

    /// Get execution statistics
    pub async fn get_execution_stats(&self) -> ExecutionStats {
        let history = self.execution_history.read().await;

        let total_chains = history.len();
        let successful_chains = history.iter().filter(|r| r.is_success()).count();
        let failed_chains = total_chains - successful_chains;

        let average_execution_time = if !history.is_empty() {
            let total_time: Duration = history.iter().map(|r| r.total_time).sum();
            total_time / total_chains as u32
        } else {
            Duration::from_millis(0)
        };

        ExecutionStats {
            total_chains,
            successful_chains,
            failed_chains,
            average_execution_time,
            success_rate: if total_chains > 0 {
                successful_chains as f32 / total_chains as f32
            } else {
                0.0
            },
        }
    }
}

impl Default for ToolExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution plan with phases for parallel execution
#[derive(Debug, Clone)]
struct ExecutionPlan {
    phases: Vec<Vec<ToolRequest>>,
}

/// Configuration for the execution engine
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub default_timeout: Duration,
    pub default_max_retries: u32,
    pub max_parallel_executions: usize,
    pub max_history_size: usize,
    pub enable_performance_tracking: bool,
    pub circuit_breaker_enabled: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            default_max_retries: 3,
            max_parallel_executions: 4,
            max_history_size: 100,
            enable_performance_tracking: true,
            circuit_breaker_enabled: true,
        }
    }
}

/// Execution statistics for monitoring
#[derive(Debug, Clone)]
pub struct ExecutionStats {
    pub total_chains: usize,
    pub successful_chains: usize,
    pub failed_chains: usize,
    pub average_execution_time: Duration,
    pub success_rate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::whitelist::WhitelistConfig;

    fn create_test_requests() -> Vec<ToolRequest> {
        vec![
            ToolRequest::new(
                "read_file".to_string(),
                serde_json::json!({"path": "input.txt"}),
            ),
            ToolRequest::new(
                "process_data".to_string(),
                serde_json::json!({"data": "test"}),
            )
            .with_dependency("req_1".to_string()),
            ToolRequest::new(
                "write_file".to_string(),
                serde_json::json!({"path": "output.txt", "content": "result"}),
            )
            .with_dependency("req_2".to_string()),
        ]
    }

    #[test]
    fn test_tool_request_creation() {
        let request =
            ToolRequest::new("test_tool".to_string(), serde_json::json!({"key": "value"}))
                .with_dependency("dep_1".to_string())
                .with_timeout(Duration::from_secs(30))
                .with_max_retries(5)
                .with_metadata("priority".to_string(), "high".to_string());

        assert_eq!(request.tool_name, "test_tool");
        assert_eq!(request.depends_on, vec!["dep_1"]);
        assert_eq!(request.timeout, Some(Duration::from_secs(30)));
        assert_eq!(request.max_retries, Some(5));
        assert_eq!(request.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_dependency_validation() {
        let engine = ToolExecutionEngine::new();

        // Valid dependencies
        let valid_requests = vec![
            ToolRequest {
                id: "1".to_string(),
                tool_name: "tool1".to_string(),
                input: serde_json::json!({}),
                depends_on: vec![],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
            ToolRequest {
                id: "2".to_string(),
                tool_name: "tool2".to_string(),
                input: serde_json::json!({}),
                depends_on: vec!["1".to_string()],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
        ];
        assert!(engine.validate_dependencies(&valid_requests).is_ok());

        // Invalid dependencies (missing dependency)
        let invalid_requests = vec![ToolRequest {
            id: "1".to_string(),
            tool_name: "tool1".to_string(),
            input: serde_json::json!({}),
            depends_on: vec!["missing".to_string()],
            timeout: None,
            max_retries: None,
            metadata: HashMap::new(),
        }];
        assert!(engine.validate_dependencies(&invalid_requests).is_err());
    }

    #[test]
    fn test_execution_plan_creation() {
        let engine = ToolExecutionEngine::new();
        let requests = vec![
            ToolRequest {
                id: "1".to_string(),
                tool_name: "tool1".to_string(),
                input: serde_json::json!({}),
                depends_on: vec![],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
            ToolRequest {
                id: "2".to_string(),
                tool_name: "tool2".to_string(),
                input: serde_json::json!({}),
                depends_on: vec![],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
            ToolRequest {
                id: "3".to_string(),
                tool_name: "tool3".to_string(),
                input: serde_json::json!({}),
                depends_on: vec!["1".to_string(), "2".to_string()],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
        ];

        let plan = engine.create_execution_plan(&requests).unwrap();

        // Should have 2 phases: [1, 2] then [3]
        assert_eq!(plan.phases.len(), 2);
        assert_eq!(plan.phases[0].len(), 2); // Tools 1 and 2 can run in parallel
        assert_eq!(plan.phases[1].len(), 1); // Tool 3 depends on both 1 and 2
    }

    #[test]
    fn test_circular_dependency_detection() {
        let engine = ToolExecutionEngine::new();
        let requests = vec![
            ToolRequest {
                id: "1".to_string(),
                tool_name: "tool1".to_string(),
                input: serde_json::json!({}),
                depends_on: vec!["2".to_string()],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
            ToolRequest {
                id: "2".to_string(),
                tool_name: "tool2".to_string(),
                input: serde_json::json!({}),
                depends_on: vec!["1".to_string()],
                timeout: None,
                max_retries: None,
                metadata: HashMap::new(),
            },
        ];

        let result = engine.create_execution_plan(&requests);
        assert!(result.is_err());

        if let Err(ClaudeError::ValidationError { message, .. }) = result {
            assert!(message.contains("Circular dependency"));
        }
    }

    #[test]
    fn test_chain_execution_result() {
        let result = ChainExecutionResult {
            chain_id: "test_chain".to_string(),
            status: ChainExecutionStatus::Completed {
                total_time: Duration::from_secs(5),
                results_count: 3,
            },
            results: HashMap::new(),
            execution_order: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            total_time: Duration::from_secs(5),
            metadata: ChainMetadata::default(),
        };

        assert!(result.is_success());
        assert_eq!(result.execution_order.len(), 3);
    }

    #[test]
    fn test_performance_metrics_calculation() {
        let engine = ToolExecutionEngine::new();
        let mut results = HashMap::new();

        let result1 = ToolExecutionResult::success(
            "1".to_string(),
            "tool1".to_string(),
            ToolResultData::text("result1"),
        );
        let result2 = ToolExecutionResult::success(
            "2".to_string(),
            "tool2".to_string(),
            ToolResultData::text("result2"),
        );

        results.insert("1".to_string(), result1);
        results.insert("2".to_string(), result2);

        let metrics = engine.calculate_performance_metrics(&results, Duration::from_secs(3));
        assert!(metrics.parallel_efficiency >= 0.0);
        assert!(metrics.parallel_efficiency <= 1.0);
    }

    #[test]
    fn test_execution_config() {
        let config = ExecutionConfig {
            default_timeout: Duration::from_secs(60),
            default_max_retries: 5,
            max_parallel_executions: 8,
            max_history_size: 200,
            enable_performance_tracking: true,
            circuit_breaker_enabled: true,
        };

        let engine = ToolExecutionEngine::new().with_config(config.clone());
        assert_eq!(engine.config.default_timeout, Duration::from_secs(60));
        assert_eq!(engine.config.default_max_retries, 5);
    }
}
