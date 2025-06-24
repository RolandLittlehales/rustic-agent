# Error Handling & Configuration System Implementation Learnings

## Quick Reference Guide

This document captures the specific technical patterns and anti-patterns discovered during the implementation of the unified error handling framework and model configuration system (Issue 1.2).

## Magic Number Elimination Patterns

### ✅ Effective Pattern
```rust
pub mod circuit_breaker {
    /// Circuit opens after 5 consecutive failures to protect downstream systems
    /// Prevents cascading failures when API is unavailable
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
    
    /// How long circuit stays open before attempting recovery
    /// After 60s, tries one request to see if service is healthy again
    pub const DEFAULT_CIRCUIT_TIMEOUT_SECS: u64 = 60;
}
```

### ❌ Anti-Pattern
```rust
const FIVE: u32 = 5;  // No semantic meaning
const SIXTY: u64 = 60; // No business context
```

**Key Insight**: Group constants by domain purpose, include business rationale in comments.

## Configuration Layer Architecture

### Three-Tier System
```rust
// Layer 1: Compile-time (never changes)
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com/v1";

// Layer 2: Runtime (deployment-specific)
pub struct RuntimeConfig {
    pub api_key: Option<String>,     // From environment
    pub http_timeout_ms: u64,        // Configurable per deployment
}

// Layer 3: Validation (security boundaries)
pub struct ValidationLimits {
    pub message_max_chars: usize,    // Security/resource limits
    pub file_max_size_bytes: u64,
}
```

**Critical Rule**: Never mix layers. If a "constant" needs configuration, it belongs in Layer 2.

## Circuit Breaker Implementation

### ✅ Atomic State Management
```rust
pub struct CircuitBreaker {
    state: AtomicU8,                    // Thread-safe state
    failure_count: AtomicU32,           // Atomic counters
    last_failure_time: Mutex<Option<Instant>>, // Only for timestamps
}
```

### ❌ Anti-Pattern
```rust
pub struct CircuitBreaker {
    state: Mutex<CircuitBreakerState>, // Causes contention
}
```

**Key Learning**: Use `AtomicU8` for enum states, not `Mutex<enum>`.

## Error Context Security

### ✅ Automatic Sanitization
```rust
impl ErrorContext {
    pub fn log_error(&self, error: &ClaudeError) {
        let safe_message = Self::sanitize_error_message(&error.to_string());
        // ... safe logging
    }
    
    fn sanitize_error_message(message: &str) -> String {
        let mut sanitized = message.to_string();
        
        // API key detection and redaction
        if let Some(start) = sanitized.find("sk-ant-") {
            sanitized.replace_range(start.., "[API_KEY_REDACTED]");
        }
        
        sanitized
    }
}
```

**Principle**: Sanitize at the logging boundary, not at error creation.

## Bounded Telemetry Design

### ✅ Memory-Safe Collection
```rust
pub struct BoundedErrorCounter {
    counts: HashMap<String, u64>,
    max_entries: usize,              // Prevents unbounded growth
    last_cleanup: Instant,           // Automatic cleanup
    cleanup_interval: Duration,
}

impl BoundedErrorCounter {
    pub fn increment(&mut self, error_type: &str) {
        // Periodic cleanup prevents memory leaks
        if self.last_cleanup.elapsed() > self.cleanup_interval {
            self.cleanup_low_frequency_entries();
        }
        
        // Bounded by design
        if self.counts.len() >= self.max_entries && !self.counts.contains_key(error_type) {
            self.remove_least_frequent();
        }
    }
}
```

**Critical Insight**: Every telemetry system needs explicit bounds to prevent production issues.

## Model Registry Fallback Strategy

### ✅ Smart Selection with Fallbacks
```rust
// Smart selection by variant with offset support
pub fn get_model_by_variant(variant: ModelVariant, offset: i32) -> &'static str {
    let models = get_models_for_variant(variant); // Ordered by preference
    models.get(offset.abs() as usize).unwrap_or(&models[0])
}

// Automatic fallback chains
fn setup_fallback_chains(&mut self) {
    self.fallback_chains.insert(
        model_ids::CLAUDE_4_SONNET,
        vec![
            model_ids::CLAUDE_3_7_SONNET,    // Next best
            model_ids::CLAUDE_3_5_SONNET,    // Fallback
            model_ids::CLAUDE_3_HAIKU,       // Last resort
        ]
    );
}
```

### ❌ Anti-Pattern
```rust
fn get_model() -> &'static str {
    "claude-4-sonnet-20250514" // What happens when deprecated?
}
```

**Key Learning**: Design for model evolution from day one.

## Frontend-Backend Config Sync

### ✅ Build-Time Generation
```rust
// build.rs
fn generate_js_config() -> String {
    format!(r#"
export const CONFIG = {{
    MESSAGE_MAX_CHARS: {},
    FILE_MAX_SIZE_BYTES: {},
}};
"#, 
        crate::config::validation::defaults::MESSAGE_MAX_CHARS,
        crate::config::validation::defaults::FILE_MAX_SIZE_BYTES,
    )
}
```

### ❌ Anti-Pattern
```javascript
// Frontend constants manually copied
const MESSAGE_MAX_CHARS = 50000; // Hope this matches backend!
```

## Testing System Properties

### ✅ Invariant Testing
```rust
#[test]
fn test_circuit_breaker_invariants() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(60));
    
    // Test invariant: circuit opens after threshold failures
    for _ in 0..2 {
        cb.record_failure();
        assert!(cb.can_execute()); // Should still be closed
    }
    
    cb.record_failure(); // Third failure
    assert!(!cb.can_execute()); // Should now be open
}

#[test]
fn test_cost_monotonicity() {
    let registry = ModelRegistry::new();
    
    // Property: more tokens = higher cost
    let cost1 = registry.estimate_cost("model", 1000, 500).unwrap();
    let cost2 = registry.estimate_cost("model", 2000, 1000).unwrap();
    
    assert!(cost2 > cost1); // Monotonicity property
}
```

**Focus**: Test system properties, not just function outputs.

## Implementation Checklist Templates

### Before Adding Constants
- [ ] Is this truly constant or should it be configurable?
- [ ] Which layer? (Compile-time/Runtime/Validation)
- [ ] Business reason documented?
- [ ] Will this differ between deployments?

### Before Implementing Error Handling
- [ ] What context helps debugging?
- [ ] Could this expose sensitive information?
- [ ] Is this actionable by users?
- [ ] What telemetry should be captured?

### Before Creating Collection Systems
- [ ] How does this handle 1000x scale?
- [ ] Memory bounds in worst case?
- [ ] Entry eviction strategy?
- [ ] Concurrent access patterns?

## Abstract Principles Summary

1. **Configuration**: Layer separation (compile-time vs runtime vs validation)
2. **Error Handling**: Security by default, observability first
3. **Concurrency**: Atomic over mutex, bounded by design
4. **Model Management**: Evolution planning, fallback strategies
5. **Testing**: System properties over function behavior
6. **Telemetry**: Bounded by design, automatic cleanup

These patterns emerged from real implementation challenges and should guide future development decisions.