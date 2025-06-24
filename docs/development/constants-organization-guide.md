# Constants Organization Guide

## Learned Patterns from Issue 1.2 Implementation

This guide documents the specific organizational patterns that emerged from eliminating magic numbers and creating the unified configuration system.

## File Organization Strategy

### Domain-Based Module Structure

```rust
// ✅ Good: Organize by problem domain
src/claude/constants.rs:
├── circuit_breaker       // All circuit breaker related values
├── error_handling        // Retry logic, timeouts, thresholds
├── telemetry            // Monitoring and cleanup intervals
├── model_config         // Model limits and defaults
├── model_costs          // Cost calculation constants
├── model_ids            // Model identifiers and selection
├── file_limits          // File operation boundaries
└── test_data            // Test constants

src/config/constants.rs:
├── defaults             // System-wide default values
├── error_templates      // Standard error messages
└── validation_limits    // Security and resource limits
```

### ❌ Anti-Pattern: Type-Based Organization
```rust
// Bad: Organized by data type
pub mod strings {
    pub const CLAUDE_API_URL: &str = "...";
    pub const ERROR_MESSAGE: &str = "...";
}

pub mod numbers {
    pub const FAILURE_THRESHOLD: u32 = 5;
    pub const MAX_TOKENS: u32 = 8192;
}
```

**Why This Fails**: Developers think in terms of problem domains, not data types.

## Constant Documentation Pattern

### Complete Documentation Template
```rust
pub mod circuit_breaker {
    /// Circuit breaker state: Closed (normal operation)
    /// Used when system is healthy and requests should proceed normally
    /// Transitions to Open after DEFAULT_FAILURE_THRESHOLD consecutive failures
    pub const STATE_CLOSED: u8 = 0;
    
    /// Default circuit breaker failure threshold
    /// Circuit opens after 5 consecutive failures to protect downstream systems
    /// Prevents cascading failures when API is unavailable
    /// Value chosen based on: brief transient issues (1-2 failures) vs persistent problems (3+ failures)
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
    
    /// Default circuit breaker timeout duration in seconds
    /// How long circuit stays open before attempting recovery
    /// After 60s, tries one request to see if service is healthy again
    /// Balance between: quick recovery (30s too aggressive) vs user patience (120s too slow)
    pub const DEFAULT_CIRCUIT_TIMEOUT_SECS: u64 = 60;
}
```

### Documentation Components
1. **What it is**: Brief description of the constant
2. **How it's used**: Context of usage in the system
3. **Business rationale**: Why this specific value was chosen
4. **Trade-offs**: What happens if value is too high/low

## Constants Naming Conventions

### Effective Naming Pattern
```rust
// ✅ Good: Semantic, domain-specific names
pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;        // What it configures
pub const CLAUDE_4_SONNET_INPUT_COST: f64 = 3.0;     // Specific model context
pub const MESSAGE_MAX_CHARS: usize = 50000;          // Clear boundaries
pub const CLEANUP_INTERVAL_SECS: u64 = 300;          // Time unit explicit

// ✅ Good: Module context provides domain
pub mod circuit_breaker {
    pub const STATE_CLOSED: u8 = 0;    // Domain clear from module
    pub const STATE_OPEN: u8 = 1;
}
```

### ❌ Anti-Pattern: Generic or Unclear Names
```rust
pub const FIVE: u32 = 5;                    // No semantic meaning
pub const MAX_SIZE: usize = 50000;          // Max size of what?
pub const TIMEOUT: u64 = 60;                // Timeout for what operation?
pub const THRESHOLD: u32 = 5;               // Threshold for what behavior?
```

## Constants Duplication Resolution

### Problem: Same Values in Multiple Places
```rust
// config/constants.rs
pub const CLAUDE_API_VERSION: &str = "2023-06-01";

// claude/constants.rs  
pub const ANTHROPIC_API_VERSION: &str = "2023-06-01"; // Duplicate!
```

### ✅ Solution: Re-export Pattern
```rust
// claude/constants.rs - Remove duplication
pub mod api {
    /// Re-export API version from unified config to maintain backward compatibility
    pub use crate::config::constants::CLAUDE_API_VERSION as ANTHROPIC_API_VERSION;
}
```

**Key Learning**: Maintain single source of truth with re-exports for backward compatibility.

## Configuration Layer Mapping

### Layer 1: Compile-Time Constants
```rust
// src/config/constants.rs
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com/v1";
pub const CLAUDE_API_VERSION: &str = "2023-06-01";
```
**Characteristics**: Never change at runtime, compiled into binary

### Layer 2: Runtime Configuration  
```rust
// src/config/runtime.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub api_key: Option<String>,        // From environment
    pub http_timeout_ms: u64,           // Deployment-specific
    pub model: String,                  // Can be overridden
}
```
**Characteristics**: Loaded from environment/config files

### Layer 3: Validation Limits
```rust
// src/config/validation.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationLimits {
    pub message_max_chars: usize,       // Security boundaries
    pub file_max_size_bytes: u64,       // Resource limits
}
```
**Characteristics**: Security and resource boundaries with validation

## Smart Constants Pattern

### Model Selection Constants
```rust
pub mod model_ids {
    /// Model variant types for smart selection
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ModelVariant {
        Opus,
        Sonnet, 
        Haiku,
    }
    
    /// Get model by variant with offset support
    /// offset: 0 = latest, -1 = previous, -2 = one before previous
    pub fn get_model_by_variant(variant: ModelVariant, offset: i32) -> &'static str {
        match variant {
            ModelVariant::Sonnet => {
                let sonnet_models = [
                    CLAUDE_4_SONNET,         // Latest
                    CLAUDE_3_7_SONNET,       // Previous  
                    CLAUDE_3_5_SONNET_LATEST, // Fallback
                ];
                get_model_at_offset(&sonnet_models, offset)
            }
            // ... other variants
        }
    }
}
```

**Pattern**: Constants that support intelligent selection and fallback behavior.

## Migration Strategy for Existing Magic Numbers

### 1. Identification Phase
```bash
# Find magic numbers in code
rg '\b\d+\b' src/ --type rust | grep -v 'const\|let.*=\|#\[' | head -10
```

### 2. Categorization
- **Circuit breaker values** → `circuit_breaker` module
- **Retry/timeout values** → `error_handling` module  
- **Model-related numbers** → `model_config` or `model_costs` modules
- **File size limits** → `file_limits` module

### 3. Replacement Pattern
```rust
// Before
if retry_count > 3 {
    return Err("Too many retries");
}

// After  
if retry_count > error_handling::DEFAULT_MAX_RETRIES {
    return Err("Too many retries");
}
```

### 4. Documentation Requirements
Each new constant must include:
- [ ] Business rationale comment
- [ ] Usage context explanation
- [ ] Value choice justification
- [ ] Related constants grouped together

## Constants Testing Strategy

### Test Constants Separately
```rust
pub mod test_data {
    /// Test input token count for cost estimation validation
    /// Realistic value representing typical conversation exchange
    pub const TEST_INPUT_TOKENS: u32 = 1000;
    
    /// Test output token count for cost estimation validation
    /// Realistic value for assistant response length
    pub const TEST_OUTPUT_TOKENS: u32 = 500;
    
    /// Expected cost calculation result for test inputs
    /// 1000 input * $3/1M + 500 output * $15/1M = $0.0105
    pub const EXPECTED_CLAUDE_4_SONNET_COST: f64 = 0.0105;
}
```

**Pattern**: Test-specific constants in dedicated module, with expected results documented.

## Maintenance Guidelines

### When Adding New Constants
1. **Check for existing similar constants** - avoid duplication
2. **Choose appropriate module** - by domain, not type
3. **Document completely** - business context required
4. **Consider configurability** - might this need to vary by deployment?
5. **Add to tests** - ensure constant behavior is tested

### When Modifying Constants  
1. **Update all documentation** - ensure comments reflect new values
2. **Check dependent tests** - may need adjustment
3. **Consider backward compatibility** - especially for public APIs
4. **Review fallback chains** - model constants may affect selection logic

### Red Flags (Anti-Patterns)
- Constants with names like `VALUE_1`, `SETTING_A`, `MAX_SIZE`
- Multiple constants with same value but different names
- Constants scattered across multiple files without organization
- Magic numbers still present in code
- Constants without any documentation

This organization strategy emerged from the practical experience of eliminating over 50 magic numbers and creating a maintainable constant system.