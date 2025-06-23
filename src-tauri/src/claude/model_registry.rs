use super::constants::{model_config, model_costs, model_ids};
use super::error::{ClaudeError, ClaudeResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub family: String,
    pub variant: String,
    pub max_tokens: u32,
    pub context_window: u32,
    pub supports_thinking: bool,
    pub supports_tool_use: bool,
    pub supports_streaming: bool,
    pub cost_per_million_input: f64,
    pub cost_per_million_output: f64,
    pub release_date: String,
    pub is_deprecated: bool,
    pub performance_tier: PerformanceTier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PerformanceTier {
    Fast,     // Haiku models - fastest, lowest cost
    Balanced, // Sonnet models - balanced performance/cost
    Powerful, // Opus models - most capable, highest cost
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelectionCriteria {
    pub task_complexity: TaskComplexity,
    pub cost_priority: CostPriority,
    pub speed_priority: SpeedPriority,
    pub thinking_required: bool,
    pub tool_use_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskComplexity {
    Simple,   // File operations, simple queries
    Moderate, // Code analysis, document processing
    Complex,  // Multi-step reasoning, complex tool chains
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostPriority {
    Low,    // Cost is not a concern
    Medium, // Balanced cost/performance
    High,   // Minimize cost
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeedPriority {
    Low,    // Speed is not a concern
    Medium, // Balanced speed/quality
    High,   // Prioritize response speed
}

impl Default for ModelSelectionCriteria {
    fn default() -> Self {
        Self {
            task_complexity: TaskComplexity::Moderate,
            cost_priority: CostPriority::Medium,
            speed_priority: SpeedPriority::Medium,
            thinking_required: false,
            tool_use_required: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelRegistry {
    models: HashMap<String, ModelInfo>,
    fallback_chains: HashMap<String, Vec<String>>,
    primary_models: HashMap<PerformanceTier, String>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
            fallback_chains: HashMap::new(),
            primary_models: HashMap::new(),
        };
        registry.initialize_models();
        registry.setup_fallback_chains();
        registry
    }

    fn initialize_models(&mut self) {
        // Claude 4 models
        self.models.insert(
            model_ids::CLAUDE_4_OPUS.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_4_OPUS.to_string(),
                family: "claude-4".to_string(),
                variant: "opus".to_string(),
                max_tokens: model_config::CLAUDE_4_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: true,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_4_OPUS_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_4_OPUS_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_4_OPUS).to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Powerful,
            },
        );

        self.models.insert(
            model_ids::CLAUDE_4_SONNET.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_4_SONNET.to_string(),
                family: "claude-4".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: model_config::CLAUDE_4_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: true,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_4_SONNET_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_4_SONNET_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_4_SONNET)
                    .to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Balanced,
            },
        );

        // Claude 3.7 models (enhanced 3.5 series)
        self.models.insert(
            model_ids::CLAUDE_3_7_SONNET.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_3_7_SONNET.to_string(),
                family: "claude-3.7".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: model_config::CLAUDE_4_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: true,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_3_7_SONNET_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_3_7_SONNET_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_3_7_SONNET)
                    .to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Balanced,
            },
        );

        // Claude 3.5 models
        self.models.insert(
            model_ids::CLAUDE_3_5_SONNET_LATEST.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_3_5_SONNET_LATEST.to_string(),
                family: "claude-3.5".to_string(),
                variant: "sonnet".to_string(),
                max_tokens: model_config::CLAUDE_3_5_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: false,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_3_5_SONNET_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_3_5_SONNET_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_3_5_SONNET_LATEST)
                    .to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Balanced,
            },
        );

        self.models.insert(
            model_ids::CLAUDE_3_5_HAIKU.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_3_5_HAIKU.to_string(),
                family: "claude-3.5".to_string(),
                variant: "haiku".to_string(),
                max_tokens: model_config::CLAUDE_3_5_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: false,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_3_5_HAIKU_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_3_5_HAIKU_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_3_5_SONNET_LATEST)
                    .to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Fast,
            },
        );

        // Claude 3 models (legacy but active)
        self.models.insert(
            model_ids::CLAUDE_3_OPUS.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_3_OPUS.to_string(),
                family: "claude-3".to_string(),
                variant: "opus".to_string(),
                max_tokens: model_config::CLAUDE_3_5_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: false,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_3_OPUS_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_3_OPUS_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_3_OPUS).to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Powerful,
            },
        );

        self.models.insert(
            model_ids::CLAUDE_3_HAIKU.to_string(),
            ModelInfo {
                name: model_ids::CLAUDE_3_HAIKU.to_string(),
                family: "claude-3".to_string(),
                variant: "haiku".to_string(),
                max_tokens: model_config::CLAUDE_3_5_MAX_TOKENS,
                context_window: model_config::CLAUDE_4_CONTEXT_WINDOW,
                supports_thinking: false,
                supports_tool_use: true,
                supports_streaming: true,
                cost_per_million_input: model_costs::CLAUDE_3_HAIKU_INPUT_COST,
                cost_per_million_output: model_costs::CLAUDE_3_HAIKU_OUTPUT_COST,
                release_date: model_ids::extract_release_date(model_ids::CLAUDE_3_HAIKU)
                    .to_string(),
                is_deprecated: false,
                performance_tier: PerformanceTier::Fast,
            },
        );

        // Set primary models for each tier using smart selection
        self.primary_models.insert(
            PerformanceTier::Powerful,
            model_ids::latest_claude_4_opus().to_string(),
        );
        self.primary_models.insert(
            PerformanceTier::Balanced,
            model_ids::latest_claude_4_sonnet().to_string(),
        );
        self.primary_models
            .insert(PerformanceTier::Fast, model_ids::latest_haiku().to_string());
    }

    fn setup_fallback_chains(&mut self) {
        // Claude 4 Opus fallback chain: Opus -> Sonnet -> 3.7 Sonnet -> Haiku
        self.fallback_chains.insert(
            model_ids::latest_claude_4_opus().to_string(),
            vec![
                model_ids::latest_claude_4_sonnet().to_string(),
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -1).to_string(), // 3.7 Sonnet
                model_ids::latest_haiku().to_string(),
            ],
        );

        // Claude 4 Sonnet fallback chain: 4 Sonnet -> 3.7 Sonnet -> 3.5 Sonnet -> Haiku
        self.fallback_chains.insert(
            model_ids::latest_claude_4_sonnet().to_string(),
            vec![
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -1).to_string(), // 3.7 Sonnet
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -2).to_string(), // 3.5 Latest
                model_ids::latest_haiku().to_string(),
            ],
        );

        // Claude 3.7 Sonnet fallback chain: 3.7 -> 4 Sonnet -> 3.5 Sonnet -> Haiku
        self.fallback_chains.insert(
            model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -1).to_string(),
            vec![
                model_ids::latest_claude_4_sonnet().to_string(),
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -2).to_string(), // 3.5 Latest
                model_ids::latest_haiku().to_string(),
            ],
        );

        // Claude 3.5 Sonnet (latest) fallback chain: 3.5 Latest -> 4 Sonnet -> 3.7 -> Haiku
        self.fallback_chains.insert(
            model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -2).to_string(),
            vec![
                model_ids::latest_claude_4_sonnet().to_string(),
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -1).to_string(), // 3.7 Sonnet
                model_ids::latest_haiku().to_string(),
            ],
        );

        // Claude 3.5 Haiku fallback chain: 3.5 Haiku -> 3 Haiku -> 3.5 Sonnet
        self.fallback_chains.insert(
            model_ids::latest_haiku().to_string(),
            vec![
                model_ids::get_model_by_variant(model_ids::ModelVariant::Haiku, -1).to_string(), // 3 Haiku
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -2).to_string(), // 3.5 Sonnet
            ],
        );

        // Claude 3 Haiku fallback chain: 3 Haiku -> 3.5 Haiku -> 3.5 Sonnet
        self.fallback_chains.insert(
            model_ids::get_model_by_variant(model_ids::ModelVariant::Haiku, -1).to_string(),
            vec![
                model_ids::latest_haiku().to_string(), // 3.5 Haiku
                model_ids::get_model_by_variant(model_ids::ModelVariant::Sonnet, -2).to_string(), // 3.5 Sonnet
            ],
        );

        // Claude 3 Opus fallback chain: 3 Opus -> 4 Opus -> 4 Sonnet -> Haiku
        self.fallback_chains.insert(
            model_ids::get_model_by_variant(model_ids::ModelVariant::Opus, -1).to_string(),
            vec![
                model_ids::latest_claude_4_opus().to_string(),
                model_ids::latest_claude_4_sonnet().to_string(),
                model_ids::latest_haiku().to_string(),
            ],
        );
    }

    pub fn get_model_info(&self, model_name: &str) -> Option<&ModelInfo> {
        self.models.get(model_name)
    }

    #[allow(dead_code)]
    pub fn get_all_models(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    #[allow(dead_code)]
    pub fn get_available_models(&self) -> Vec<&ModelInfo> {
        self.models
            .values()
            .filter(|model| !model.is_deprecated)
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_models_by_tier(&self, tier: &PerformanceTier) -> Vec<&ModelInfo> {
        self.models
            .values()
            .filter(|model| !model.is_deprecated && &model.performance_tier == tier)
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_fallback_chain(&self, model_name: &str) -> Option<&Vec<String>> {
        self.fallback_chains.get(model_name)
    }

    #[allow(dead_code)]
    pub fn get_next_fallback(&self, current_model: &str) -> Option<String> {
        let chain = self.get_fallback_chain(current_model)?;
        chain.first().cloned()
    }

    pub fn validate_model(&self, model_name: &str) -> ClaudeResult<&ModelInfo> {
        let model_info =
            self.get_model_info(model_name)
                .ok_or_else(|| ClaudeError::ModelError {
                    model: model_name.to_string(),
                    message: format!("Unknown model: {}", model_name),
                    context: None,
                })?;

        if model_info.is_deprecated {
            return Err(ClaudeError::ModelError {
                model: model_name.to_string(),
                message: format!("Model {} is deprecated", model_name),
                context: None,
            });
        }

        Ok(model_info)
    }

    #[allow(dead_code)]
    pub fn select_optimal_model(&self, criteria: &ModelSelectionCriteria) -> String {
        let available_models = self.get_available_models();

        // Filter by requirements first to reduce calculation load
        let suitable_models: Vec<&ModelInfo> = available_models
            .into_iter()
            .filter(|model| {
                (!criteria.thinking_required || model.supports_thinking)
                    && (!criteria.tool_use_required || model.supports_tool_use)
            })
            .collect();

        if suitable_models.is_empty() {
            // Fallback to default
            return self
                .primary_models
                .get(&PerformanceTier::Balanced)
                .unwrap_or(&model_ids::CLAUDE_4_SONNET.to_string())
                .clone();
        }

        // Find best model without full sort - just find maximum
        let mut best_model = suitable_models[0];
        let mut best_score = self.calculate_model_score(best_model, criteria);

        for model in suitable_models.iter().skip(1) {
            let score = self.calculate_model_score(model, criteria);
            if score > best_score {
                best_score = score;
                best_model = model;
            }
        }

        best_model.name.clone()
    }

    #[allow(dead_code)]
    fn calculate_model_score(&self, model: &ModelInfo, criteria: &ModelSelectionCriteria) -> f64 {
        let mut score = 0.0;

        // Task complexity score
        let complexity_score = match (&criteria.task_complexity, &model.performance_tier) {
            (TaskComplexity::Simple, PerformanceTier::Fast) => 3.0,
            (TaskComplexity::Simple, PerformanceTier::Balanced) => 2.0,
            (TaskComplexity::Simple, PerformanceTier::Powerful) => 1.0,
            (TaskComplexity::Moderate, PerformanceTier::Fast) => 1.0,
            (TaskComplexity::Moderate, PerformanceTier::Balanced) => 3.0,
            (TaskComplexity::Moderate, PerformanceTier::Powerful) => 2.0,
            (TaskComplexity::Complex, PerformanceTier::Fast) => 0.5,
            (TaskComplexity::Complex, PerformanceTier::Balanced) => 2.0,
            (TaskComplexity::Complex, PerformanceTier::Powerful) => 3.0,
        };

        score += complexity_score;

        // Cost priority score (lower cost = higher score when cost priority is high)
        let cost_score = match criteria.cost_priority {
            CostPriority::High => {
                let max_cost = model_costs::CLAUDE_4_OPUS_OUTPUT_COST;
                let model_cost = model.cost_per_million_output;
                3.0 * (1.0 - (model_cost / max_cost))
            }
            CostPriority::Medium => 1.0,
            CostPriority::Low => 0.5,
        };

        score += cost_score;

        // Speed priority score (haiku models are faster)
        let speed_score = match (&criteria.speed_priority, &model.performance_tier) {
            (SpeedPriority::High, PerformanceTier::Fast) => 2.0,
            (SpeedPriority::High, PerformanceTier::Balanced) => 1.0,
            (SpeedPriority::High, PerformanceTier::Powerful) => 0.5,
            (SpeedPriority::Medium, _) => 1.0,
            (SpeedPriority::Low, _) => 0.5,
        };

        score += speed_score;

        // Thinking capability bonus
        if criteria.thinking_required && model.supports_thinking {
            score += 1.0;
        }

        score
    }

    #[allow(dead_code)]
    pub fn estimate_cost(
        &self,
        model_name: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Option<f64> {
        // Validate inputs to prevent overflow and unrealistic values
        if input_tokens > model_costs::MAX_REASONABLE_TOKEN_COUNT
            || output_tokens > model_costs::MAX_REASONABLE_TOKEN_COUNT
        {
            // Log warning for unusually large token counts (>10M tokens)
            eprintln!(
                "⚠️ Warning: Very large token count in cost estimation: input={}, output={}",
                input_tokens, output_tokens
            );
        }

        let model_info = self.get_model_info(model_name)?;

        // Use checked arithmetic to prevent overflow
        let input_cost = (input_tokens as f64 / model_costs::TOKENS_PER_MILLION)
            * model_info.cost_per_million_input;
        let output_cost = (output_tokens as f64 / model_costs::TOKENS_PER_MILLION)
            * model_info.cost_per_million_output;

        // Verify the calculation results are valid
        if input_cost.is_finite() && output_cost.is_finite() {
            let total_cost = input_cost + output_cost;
            if total_cost.is_finite() && total_cost >= 0.0 {
                Some(total_cost)
            } else {
                eprintln!("❌ Error: Invalid cost calculation result: {}", total_cost);
                None
            }
        } else {
            eprintln!(
                "❌ Error: Invalid cost calculation: input_cost={}, output_cost={}",
                input_cost, output_cost
            );
            None
        }
    }

    #[allow(dead_code)]
    pub fn compare_costs(
        &self,
        models: &[String],
        input_tokens: u32,
        output_tokens: u32,
    ) -> Vec<(String, f64)> {
        models
            .iter()
            .filter_map(|model| {
                self.estimate_cost(model, input_tokens, output_tokens)
                    .map(|cost| (model.clone(), cost))
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn suggest_alternative(
        &self,
        current_model: &str,
        criteria: &ModelSelectionCriteria,
    ) -> Option<String> {
        // First try the optimal selection
        let optimal = self.select_optimal_model(criteria);
        if optimal != current_model {
            return Some(optimal);
        }

        // If current model is already optimal, suggest from fallback chain
        self.get_next_fallback(current_model)
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::constants::test_data;

    #[test]
    fn test_model_registry_initialization() {
        let registry = ModelRegistry::new();
        assert!(!registry.models.is_empty());
        assert!(registry
            .get_model_info(model_ids::CLAUDE_4_SONNET)
            .is_some());
    }

    #[test]
    fn test_model_validation() {
        let registry = ModelRegistry::new();

        // Valid model
        assert!(registry.validate_model(model_ids::CLAUDE_4_SONNET).is_ok());

        // Invalid model
        assert!(registry.validate_model("invalid-model").is_err());
    }

    #[test]
    fn test_fallback_chains() {
        let registry = ModelRegistry::new();

        let fallback = registry.get_next_fallback(model_ids::CLAUDE_4_OPUS);
        assert_eq!(fallback, Some(model_ids::CLAUDE_4_SONNET.to_string()));
    }

    #[test]
    fn test_model_selection() {
        let registry = ModelRegistry::new();

        // High cost priority should select cheaper models
        let criteria = ModelSelectionCriteria {
            task_complexity: TaskComplexity::Simple,
            cost_priority: CostPriority::High,
            speed_priority: SpeedPriority::Medium,
            thinking_required: false,
            tool_use_required: true,
        };

        let selected = registry.select_optimal_model(&criteria);
        let model_info = registry.get_model_info(&selected).unwrap();
        assert_eq!(model_info.performance_tier, PerformanceTier::Fast);
    }

    #[test]
    fn test_cost_estimation() {
        let registry = ModelRegistry::new();

        let cost = registry.estimate_cost(
            model_ids::CLAUDE_4_SONNET,
            test_data::TEST_INPUT_TOKENS,
            test_data::TEST_OUTPUT_TOKENS,
        );
        assert!(cost.is_some());

        // Expected cost from test constants
        assert!(
            (cost.unwrap() - test_data::EXPECTED_CLAUDE_4_SONNET_COST).abs()
                < test_data::COST_CALCULATION_TOLERANCE
        );
    }

    #[test]
    fn test_performance_tiers() {
        let registry = ModelRegistry::new();

        let fast_models = registry.get_models_by_tier(&PerformanceTier::Fast);
        let balanced_models = registry.get_models_by_tier(&PerformanceTier::Balanced);
        let powerful_models = registry.get_models_by_tier(&PerformanceTier::Powerful);

        assert!(!fast_models.is_empty());
        assert!(!balanced_models.is_empty());
        assert!(!powerful_models.is_empty());
    }
}
