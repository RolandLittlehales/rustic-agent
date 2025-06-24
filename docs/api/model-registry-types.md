# Model Registry API Reference

This document provides comprehensive API reference for the Model Registry system implemented in issue [1.2], which manages Claude model configurations, fallback strategies, and cost optimization.

## Core Types

### ModelRegistry

Central registry for managing Claude model configurations and selection logic.

```rust
pub struct ModelRegistry {
    models: HashMap<String, ModelInfo>,
    fallback_chains: HashMap<String, Vec<String>>,
    primary_models: HashMap<PerformanceTier, String>,
}
```

#### Methods

##### `new() -> Self`
Creates a new ModelRegistry with all supported Claude models pre-configured.

```rust
let registry = ModelRegistry::new();
```

##### `get_model_info(&self, model_name: &str) -> Option<&ModelInfo>`
Retrieves model information for a specific model.

```rust
let model_info = registry.get_model_info("claude-4-sonnet-20250522");
```

##### `validate_model(&self, model_name: &str) -> ClaudeResult<&ModelInfo>`
Validates that a model exists and is not deprecated.

```rust
let validated_model = registry.validate_model("claude-4-sonnet-20250522")?;
```

##### `select_optimal_model(&self, criteria: &ModelSelectionCriteria) -> String`
Selects the optimal model based on selection criteria.

```rust
let criteria = ModelSelectionCriteria {
    task_complexity: TaskComplexity::Complex,
    cost_priority: CostPriority::Medium,
    speed_priority: SpeedPriority::High,
    thinking_required: true,
    tool_use_required: true,
};
let selected_model = registry.select_optimal_model(&criteria);
```

##### `get_fallback_chain(&self, model_name: &str) -> Option<&Vec<String>>`
Gets the fallback chain for a specific model.

```rust
let fallbacks = registry.get_fallback_chain("claude-4-opus-20250522");
```

##### `estimate_cost(&self, model_name: &str, input_tokens: u32, output_tokens: u32) -> Option<f64>`
Estimates cost for a specific model and token usage.

```rust
let cost = registry.estimate_cost("claude-4-sonnet-20250522", 1000, 500);
```

### ModelInfo

Comprehensive information about a Claude model.

```rust
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
```

#### Fields

- **`name`** - Full model identifier (e.g., "claude-4-sonnet-20250522")
- **`family`** - Model family (e.g., "claude-4", "claude-3.5")
- **`variant`** - Model variant (e.g., "opus", "sonnet", "haiku")
- **`max_tokens`** - Maximum output tokens per request
- **`context_window`** - Maximum input context window size
- **`supports_thinking`** - Whether model supports thinking mode
- **`supports_tool_use`** - Whether model supports tool execution
- **`supports_streaming`** - Whether model supports streaming responses
- **`cost_per_million_input`** - Cost per million input tokens (USD)
- **`cost_per_million_output`** - Cost per million output tokens (USD)
- **`release_date`** - Model release date (YYYY-MM-DD format)
- **`is_deprecated`** - Whether model is deprecated
- **`performance_tier`** - Performance tier classification

### PerformanceTier

Categorizes models by performance characteristics.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PerformanceTier {
    Fast,     // Haiku models - fastest, lowest cost
    Balanced, // Sonnet models - balanced performance/cost
    Powerful, // Opus models - most capable, highest cost
}
```

### ModelSelectionCriteria

Criteria for optimal model selection.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelectionCriteria {
    pub task_complexity: TaskComplexity,
    pub cost_priority: CostPriority,
    pub speed_priority: SpeedPriority,
    pub thinking_required: bool,
    pub tool_use_required: bool,
}
```

#### Default Values
```rust
ModelSelectionCriteria {
    task_complexity: TaskComplexity::Moderate,
    cost_priority: CostPriority::Medium,
    speed_priority: SpeedPriority::Medium,
    thinking_required: false,
    tool_use_required: true,
}
```

### TaskComplexity

Categorizes task complexity for model selection.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskComplexity {
    Simple,   // File operations, simple queries
    Moderate, // Code analysis, document processing
    Complex,  // Multi-step reasoning, complex tool chains
}
```

### CostPriority

Priority level for cost optimization.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostPriority {
    Low,    // Cost is not a concern
    Medium, // Balanced cost/performance
    High,   // Minimize cost
}
```

### SpeedPriority

Priority level for response speed.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeedPriority {
    Low,    // Speed is not a concern
    Medium, // Balanced speed/quality
    High,   // Prioritize response speed
}
```

## Pre-configured Models

### Claude 4 Models

#### claude-4-opus-20250522
```rust
ModelInfo {
    name: "claude-4-opus-20250522",
    family: "claude-4",
    variant: "opus",
    max_tokens: 8192,
    context_window: 200000,
    supports_thinking: true,
    supports_tool_use: true,
    supports_streaming: true,
    cost_per_million_input: 15.0,
    cost_per_million_output: 75.0,
    performance_tier: PerformanceTier::Powerful,
}
```

#### claude-4-sonnet-20250522
```rust
ModelInfo {
    name: "claude-4-sonnet-20250522",
    family: "claude-4", 
    variant: "sonnet",
    max_tokens: 8192,
    context_window: 200000,
    supports_thinking: true,
    supports_tool_use: true,
    supports_streaming: true,
    cost_per_million_input: 3.0,
    cost_per_million_output: 15.0,
    performance_tier: PerformanceTier::Balanced,
}
```

#### claude-4-haiku-20250522
```rust
ModelInfo {
    name: "claude-4-haiku-20250522",
    family: "claude-4",
    variant: "haiku", 
    max_tokens: 8192,
    context_window: 200000,
    supports_thinking: true,
    supports_tool_use: true,
    supports_streaming: true,
    cost_per_million_input: 0.25,
    cost_per_million_output: 1.25,
    performance_tier: PerformanceTier::Fast,
}
```

### Claude 3.5 Models (Legacy Support)

#### claude-3-5-sonnet-20241022
```rust
ModelInfo {
    name: "claude-3-5-sonnet-20241022",
    family: "claude-3.5",
    variant: "sonnet",
    max_tokens: 8192,
    context_window: 200000,
    supports_thinking: false,
    supports_tool_use: true,
    supports_streaming: true,
    cost_per_million_input: 3.0,
    cost_per_million_output: 15.0,
    performance_tier: PerformanceTier::Balanced,
}
```

#### claude-3-5-haiku-20241022
```rust
ModelInfo {
    name: "claude-3-5-haiku-20241022", 
    family: "claude-3.5",
    variant: "haiku",
    max_tokens: 8192,
    context_window: 200000,
    supports_thinking: false,
    supports_tool_use: true,
    supports_streaming: true,
    cost_per_million_input: 0.8,
    cost_per_million_output: 4.0,
    performance_tier: PerformanceTier::Fast,
}
```

## Fallback Chains

The registry includes pre-configured fallback chains for resilient model selection:

### Claude 4 Opus Fallbacks
1. claude-4-sonnet-20250522
2. claude-3-5-sonnet-20241022
3. claude-4-haiku-20250522

### Claude 4 Sonnet Fallbacks
1. claude-3-5-sonnet-20241022
2. claude-4-haiku-20250522
3. claude-3-5-haiku-20241022

### Claude 4 Haiku Fallbacks
1. claude-3-5-haiku-20241022
2. claude-3-5-sonnet-20241022

## Model Selection Algorithm

The optimal model selection uses a scoring system based on:

1. **Task Complexity Matching**
   - Simple tasks favor Fast tier models
   - Moderate tasks favor Balanced tier models  
   - Complex tasks favor Powerful tier models

2. **Cost Optimization**
   - High cost priority heavily weights lower-cost models
   - Cost score inversely proportional to model cost

3. **Speed Requirements**
   - High speed priority favors Fast tier models
   - Balanced approach considers all tiers equally

4. **Capability Requirements**
   - Thinking requirement filters to thinking-capable models
   - Tool use requirement filters to tool-capable models

## Usage Examples

### Basic Model Information
```rust
let registry = ModelRegistry::new();
let model_info = registry.get_model_info("claude-4-sonnet-20250522").unwrap();
println!("Max tokens: {}", model_info.max_tokens);
println!("Supports thinking: {}", model_info.supports_thinking);
```

### Cost-Optimized Selection
```rust
let criteria = ModelSelectionCriteria {
    task_complexity: TaskComplexity::Simple,
    cost_priority: CostPriority::High,
    speed_priority: SpeedPriority::Medium,
    thinking_required: false,
    tool_use_required: true,
};

let selected_model = registry.select_optimal_model(&criteria);
// Returns: "claude-4-haiku-20250522" (lowest cost)
```

### Complex Task Selection
```rust
let criteria = ModelSelectionCriteria {
    task_complexity: TaskComplexity::Complex,
    cost_priority: CostPriority::Low,
    speed_priority: SpeedPriority::Low,
    thinking_required: true,
    tool_use_required: true,
};

let selected_model = registry.select_optimal_model(&criteria);
// Returns: "claude-4-opus-20250522" (most capable)
```

### Cost Estimation
```rust
let cost = registry.estimate_cost("claude-4-sonnet-20250522", 1000, 500);
// Input: 1000 tokens at $3.0/1M = $0.003
// Output: 500 tokens at $15.0/1M = $0.0075
// Total: $0.0105
```

## Integration with ClaudeConfig

The ModelRegistry integrates seamlessly with the existing ClaudeConfig:

```rust
impl ClaudeConfig {
    pub fn get_model_info(&self) -> Option<&ModelInfo> {
        self.model_registry.get_model_info(&self.model)
    }

    pub fn validate(&self) -> ClaudeResult<()> {
        self.model_registry.validate_model(&self.model)?;
        // Additional validation...
    }
}
```

## Related Documentation

- [Architecture: Model Registry System](../architecture/model-registry-system.md)
- [Integration Guide: Model Selection](../tools/model-selection-integration.md)
- [Error Handling API](./error-types.md)
- [Configuration System](./configuration-types.md)