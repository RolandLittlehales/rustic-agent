/*!
 * Telemetry Collection and Metrics
 *
 * Collects and aggregates metrics about tool execution, API usage,
 * costs, and system health for monitoring and optimization.
 */

use super::{LogEntry, LogLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;

/// Types of metrics we can collect
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    ToolExecution,
    ApiCall,
    Cost,
    Error,
    General,
}

/// Aggregated metrics for a specific type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummary {
    pub count: u64,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub success_rate: f64,
    pub total_cost: f64,
    pub last_updated: DateTime<Utc>,
}

impl Default for MetricSummary {
    fn default() -> Self {
        Self {
            count: 0,
            total_duration: Duration::ZERO,
            avg_duration: Duration::ZERO,
            success_rate: 1.0,
            total_cost: 0.0,
            last_updated: Utc::now(),
        }
    }
}

/// Detailed log metrics with counts per level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetrics {
    pub total_logs: u64,
    pub error_count: u64,
    pub warn_count: u64,
    pub info_count: u64,
    pub debug_count: u64,
    pub trace_count: u64,
    pub error_rate: f64,
    pub last_updated: DateTime<Utc>,
}

impl Default for LogMetrics {
    fn default() -> Self {
        Self {
            total_logs: 0,
            error_count: 0,
            warn_count: 0,
            info_count: 0,
            debug_count: 0,
            trace_count: 0,
            error_rate: 0.0,
            last_updated: Utc::now(),
        }
    }
}

/// Tool execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetrics {
    pub tool_name: String,
    pub executions: u64,
    pub successes: u64,
    pub failures: u64,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub success_rate: f64,
    pub last_execution: DateTime<Utc>,
}

impl ToolMetrics {
    pub fn new(tool_name: String) -> Self {
        Self {
            tool_name,
            executions: 0,
            successes: 0,
            failures: 0,
            total_duration: Duration::ZERO,
            avg_duration: Duration::ZERO,
            success_rate: 1.0,
            last_execution: Utc::now(),
        }
    }

    pub fn record_execution(&mut self, success: bool, duration: Duration) {
        self.executions += 1;
        if success {
            self.successes += 1;
        } else {
            self.failures += 1;
        }
        self.total_duration += duration;
        self.avg_duration = self.total_duration / self.executions as u32;
        self.success_rate = self.successes as f64 / self.executions as f64;
        self.last_execution = Utc::now();
    }
}

/// API call metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub model: String,
    pub calls: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub avg_tokens_per_call: f64,
    pub avg_cost_per_call: f64,
    pub last_call: DateTime<Utc>,
}

impl ApiMetrics {
    pub fn new(model: String) -> Self {
        Self {
            model,
            calls: 0,
            total_tokens: 0,
            total_cost: 0.0,
            total_duration: Duration::ZERO,
            avg_duration: Duration::ZERO,
            avg_tokens_per_call: 0.0,
            avg_cost_per_call: 0.0,
            last_call: Utc::now(),
        }
    }

    pub fn record_call(&mut self, tokens: u64, cost: f64, duration: Duration) {
        self.calls += 1;
        self.total_tokens += tokens;
        self.total_cost += cost;
        self.total_duration += duration;
        self.avg_duration = self.total_duration / self.calls as u32;
        self.avg_tokens_per_call = self.total_tokens as f64 / self.calls as f64;
        self.avg_cost_per_call = self.total_cost / self.calls as f64;
        self.last_call = Utc::now();
    }
}

/// Main telemetry collector
#[derive(Debug)]
pub struct TelemetryCollector {
    log_metrics: RwLock<LogMetrics>,
    tool_metrics: RwLock<HashMap<String, ToolMetrics>>,
    api_metrics: RwLock<HashMap<String, ApiMetrics>>,
    general_metrics: RwLock<HashMap<String, MetricSummary>>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self {
            log_metrics: RwLock::new(LogMetrics::default()),
            tool_metrics: RwLock::new(HashMap::new()),
            api_metrics: RwLock::new(HashMap::new()),
            general_metrics: RwLock::new(HashMap::new()),
        }
    }

    /// Record a log entry for metrics
    pub fn record_log(&self, entry: &LogEntry) {
        if let Ok(mut metrics) = self.log_metrics.write() {
            metrics.total_logs += 1;

            match entry.level {
                LogLevel::Error => metrics.error_count += 1,
                LogLevel::Warn => metrics.warn_count += 1,
                LogLevel::Info => metrics.info_count += 1,
                LogLevel::Debug => metrics.debug_count += 1,
                LogLevel::Trace => metrics.trace_count += 1,
            }

            // Calculate error rate
            metrics.error_rate = if metrics.total_logs > 0 {
                metrics.error_count as f64 / metrics.total_logs as f64
            } else {
                0.0
            };

            metrics.last_updated = Utc::now();
        }
    }

    /// Record tool execution metrics
    pub fn record_tool_execution(&self, entry: &LogEntry) {
        if let (Some(tool_name), Some(result)) =
            (entry.metadata.get("tool"), entry.metadata.get("result"))
        {
            let success = result == "success";
            let duration = entry.duration.unwrap_or(Duration::ZERO);

            if let Ok(mut metrics) = self.tool_metrics.write() {
                let tool_metric = metrics
                    .entry(tool_name.clone())
                    .or_insert_with(|| ToolMetrics::new(tool_name.clone()));

                tool_metric.record_execution(success, duration);
            }
        }
    }

    /// Record API call metrics
    pub fn record_api_call(&self, entry: &LogEntry) {
        if let (Some(model), Some(tokens_str), Some(cost_str)) = (
            entry.metadata.get("model"),
            entry.metadata.get("tokens"),
            entry.metadata.get("cost_usd"),
        ) {
            if let (Ok(tokens), Ok(cost)) = (tokens_str.parse::<u64>(), cost_str.parse::<f64>()) {
                let duration = entry.duration.unwrap_or(Duration::ZERO);

                if let Ok(mut metrics) = self.api_metrics.write() {
                    let api_metric = metrics
                        .entry(model.clone())
                        .or_insert_with(|| ApiMetrics::new(model.clone()));

                    api_metric.record_call(tokens, cost, duration);
                }
            }
        }
    }

    /// Record cost metrics
    pub fn record_cost(&self, entry: &LogEntry) {
        if let Some(cost_str) = entry.metadata.get("cost_usd") {
            if let Ok(cost) = cost_str.parse::<f64>() {
                if let Ok(mut metrics) = self.general_metrics.write() {
                    let cost_metric = metrics
                        .entry("total_cost".to_string())
                        .or_insert_with(MetricSummary::default);

                    cost_metric.count += 1;
                    cost_metric.total_cost += cost;
                    cost_metric.last_updated = Utc::now();
                }
            }
        }
    }

    /// Get current log metrics
    pub fn get_log_metrics(&self) -> LogMetrics {
        self.log_metrics.read().unwrap().clone()
    }

    /// Get tool metrics
    pub fn get_tool_metrics(&self) -> HashMap<String, ToolMetrics> {
        self.tool_metrics.read().unwrap().clone()
    }

    /// Get API metrics
    pub fn get_api_metrics(&self) -> HashMap<String, ApiMetrics> {
        self.api_metrics.read().unwrap().clone()
    }

    /// Get general metrics
    pub fn get_general_metrics(&self) -> HashMap<String, MetricSummary> {
        self.general_metrics.read().unwrap().clone()
    }

    /// Get a summary report of all metrics
    pub fn get_summary_report(&self) -> TelemetryReport {
        TelemetryReport {
            log_metrics: self.get_log_metrics(),
            tool_metrics: self.get_tool_metrics(),
            api_metrics: self.get_api_metrics(),
            general_metrics: self.get_general_metrics(),
            generated_at: Utc::now(),
        }
    }

    /// Reset all metrics (useful for testing or periodic cleanup)
    pub fn reset(&self) {
        if let Ok(mut log_metrics) = self.log_metrics.write() {
            *log_metrics = LogMetrics::default();
        }
        if let Ok(mut tool_metrics) = self.tool_metrics.write() {
            tool_metrics.clear();
        }
        if let Ok(mut api_metrics) = self.api_metrics.write() {
            api_metrics.clear();
        }
        if let Ok(mut general_metrics) = self.general_metrics.write() {
            general_metrics.clear();
        }
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete telemetry report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryReport {
    pub log_metrics: LogMetrics,
    pub tool_metrics: HashMap<String, ToolMetrics>,
    pub api_metrics: HashMap<String, ApiMetrics>,
    pub general_metrics: HashMap<String, MetricSummary>,
    pub generated_at: DateTime<Utc>,
}

impl TelemetryReport {
    /// Format the report as a human-readable string
    pub fn format_summary(&self) -> String {
        let mut report = vec![
            format!(
                "📊 Telemetry Report - Generated at {}",
                self.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
            ),
            String::new(),
            "📝 Log Metrics:".to_string(),
            format!("  Total logs: {}", self.log_metrics.total_logs),
            format!(
                "  Errors: {} ({:.1}%)",
                self.log_metrics.error_count,
                self.log_metrics.error_rate * 100.0
            ),
            format!("  Warnings: {}", self.log_metrics.warn_count),
            format!("  Info: {}", self.log_metrics.info_count),
            format!("  Debug: {}", self.log_metrics.debug_count),
            String::new(),
        ];

        if !self.tool_metrics.is_empty() {
            report.push("🛠️ Tool Metrics:".to_string());
            for (tool, metrics) in &self.tool_metrics {
                report.push(format!(
                    "  {}: {} executions, {:.1}% success, avg {}ms",
                    tool,
                    metrics.executions,
                    metrics.success_rate * 100.0,
                    metrics.avg_duration.as_millis()
                ));
            }
            report.push(String::new());
        }

        if !self.api_metrics.is_empty() {
            report.push("🤖 API Metrics:".to_string());
            for (model, metrics) in &self.api_metrics {
                report.push(format!(
                    "  {}: {} calls, {:.0} avg tokens, ${:.4} avg cost, {}ms avg",
                    model,
                    metrics.calls,
                    metrics.avg_tokens_per_call,
                    metrics.avg_cost_per_call,
                    metrics.avg_duration.as_millis()
                ));
            }
            report.push(String::new());
        }

        if let Some(total_cost) = self.general_metrics.get("total_cost") {
            report.push(format!("💰 Total Cost: ${:.6}", total_cost.total_cost));
        }

        report.join("\n")
    }
}
