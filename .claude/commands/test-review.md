# Test Review

Test optimization and quality analysis following the proven patterns from error handling test refactoring with focus on strategic consolidation and efficiency.

## 🎯 Purpose

This command analyzes **test quality and optimization opportunities** based on the battle-tested patterns that achieved 67% test reduction while maintaining 100% coverage. It identifies duplication, suggests consolidation strategies, and promotes comprehensive testing over fragmented approaches.

## 🧪 Test Analysis Areas

### **📊 Test Organization Analysis**

#### **Test Count and Distribution**
```
✅ Total test count vs coverage efficiency
✅ Test size distribution (small/medium/large)
✅ Setup code duplication across tests
✅ Test execution time analysis
✅ Failure isolation vs comprehensive coverage
```

#### **Test Categories**
```
✅ Unit tests: Isolated component testing
✅ Integration tests: System interaction testing
✅ Property-based tests: Invariant and relationship testing
✅ Performance tests: Critical path benchmarking
✅ Security tests: Vulnerability and boundary testing
```

### **🔍 Duplication Detection**

#### **Setup Code Analysis**
```rust
// ❌ Anti-pattern: Duplicated setup in every test
#[test]
fn test_timeout_error() {
    let error = ClaudeError::TimeoutError { 
        duration: Duration::from_secs(30), 
        context: None 
    };
    // ... test logic
}

#[test]
fn test_rate_limit_error() {
    let error = ClaudeError::RateLimitError { 
        retry_after: Some(60), 
        context: None 
    };
    // ... similar test logic
}
```

#### **Test Fixture Opportunities**
```rust
// ✅ Optimization: Centralized test fixtures
fn create_test_context() -> ErrorContext {
    ErrorContext::new("test_operation")
        .with_message_id("msg_123")
        .with_tool_use_id("tool_456")
        .add_metadata("test_key", "test_value")
}

fn create_test_errors() -> Vec<ClaudeError> {
    vec![
        ClaudeError::TimeoutError { duration: Duration::from_secs(30), context: None },
        ClaudeError::RateLimitError { retry_after: Some(60), context: None },
        // ... more error types
    ]
}
```

### **🎯 Consolidation Opportunities**

#### **Data-Driven Testing Potential**
```rust
// ✅ Optimization: Data-driven approach
#[test]
fn test_security_sanitization_comprehensive() {
    let test_cases = vec![
        ("Error: API key sk-ant-api03-1234 in request", 
         "Error: API key [API_KEY_REDACTED] in request"),
        ("Failed to read /home/username/secret/data.txt", 
         "Failed to read /[USER_DIR_REDACTED]/secret/data.txt"),
        ("Cannot access C:\\Users\\JohnDoe\\Documents\\file.doc", 
         "Cannot access /[USER_DIR_REDACTED]\\Documents\\file.doc"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(ErrorContext::sanitize_error_message(input), expected);
    }
}
```

#### **Comprehensive Testing Patterns**
```rust
// ✅ Optimization: Full component behavior testing
#[test]
fn test_circuit_breaker_comprehensive() {
    let cb = CircuitBreaker::new(3, Duration::from_millis(100));
    
    // Test initial state and failure progression
    assert_eq!(cb.get_state(), CircuitBreakerState::Closed);
    
    // Test failure threshold behavior
    for i in 1..=2 {
        cb.record_failure();
        assert_eq!(cb.get_state(), CircuitBreakerState::Closed);
    }
    
    // Test state transitions: Closed → Open → Half-Open → Closed
    cb.record_failure(); // Third failure opens circuit
    assert_eq!(cb.get_state(), CircuitBreakerState::Open);
    
    std::thread::sleep(Duration::from_millis(150)); // Timeout recovery
    assert!(cb.can_execute()); // Transitions to half-open
    
    cb.record_success(); // Success closes circuit
    assert_eq!(cb.get_state(), CircuitBreakerState::Closed);
}
```

### **🏗️ Property-Based Testing Analysis**

#### **Property vs Value Testing**
```rust
// ✅ Good: Property-based testing
#[test]
fn test_exponential_backoff_properties() {
    let handler = ErrorHandler::new();
    let error = ClaudeError::RateLimitError { retry_after: None, context: None };
    
    // Test property: delays should increase exponentially
    let delays: Vec<_> = (0..3).map(|i| handler.calculate_delay(i, &error)).collect();
    assert!(delays[1] > delays[0] && delays[2] > delays[1]);
    
    // Test property: jitter creates variation but stays within bounds
    let base_delay = Duration::from_secs(1);
    let jittered_delays: Vec<_> = (0..5).map(|_| handler.apply_jitter(base_delay)).collect();
    let unique_count = jittered_delays.iter().collect::<HashSet<_>>().len();
    assert!(unique_count > 1); // Should have variation
}

// ❌ Avoid: Testing specific values instead of properties
#[test]
fn test_delay_calculation_attempt_0() {
    assert_eq!(handler.calculate_delay(0, &error), Duration::from_millis(100));
}
```

### **🔄 Integration vs Unit Balance**

#### **Integration Testing Value**
```rust
// ✅ Good: Integration test covering real usage patterns
#[tokio::test]
async fn test_error_handler_retry_scenarios() {
    let handler = ErrorHandler::new();
    
    // Test successful retry scenario
    let mut attempt = 0;
    let result = handler.handle_with_retry(|| {
        attempt += 1;
        async move {
            if attempt < 3 {
                Err(ClaudeError::TimeoutError { /* ... */ })
            } else {
                Ok("success")
            }
        }
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(attempt, 3);
    
    // Test non-retryable error scenario in same test
    let mut attempt2 = 0;
    let result2 = handler.handle_with_retry(|| {
        attempt2 += 1;
        async move {
            Err::<String, _>(ClaudeError::ValidationError { /* ... */ })
        }
    }).await;
    
    assert!(result2.is_err());
    assert_eq!(attempt2, 1); // Should not retry
}
```

## 💡 Usage

### **Basic Usage**
```
/test-review $ARGUMENTS
```

### **Argument Formats**
```bash
/test-review                           # Analyze all tests
/test-review "error_handling"          # Specific module analysis
/test-review "src/claude/client"       # Specific file/directory
/test-review --suggest-optimizations   # Focus on optimization suggestions
/test-review --fixtures-only           # Analyze test fixtures potential
```

### **Example Output (Optimization Opportunities)**
```
🧪 Test Quality and Optimization Analysis

📊 Current Test Metrics:
- Total tests: 21
- Average test length: 15 lines
- Setup code duplication: 73%
- Execution time: 4.2 seconds
- Test categories: 18 unit, 2 integration, 1 property-based

🔍 Duplication Analysis: ❌ HIGH DUPLICATION
❌ Repeated setup patterns detected:
   📍 5 tests create identical ErrorContext
   📍 4 tests duplicate ClaudeError creation
   📍 3 tests repeat sanitization setup
   
🎯 Consolidation Opportunities: ✅ SIGNIFICANT POTENTIAL
✅ Recommended consolidations:
   1. Merge sanitization tests (5 → 1): Data-driven approach
      - Estimated reduction: 80 lines of code
      - Coverage maintained: 100%
   
   2. Consolidate circuit breaker tests (4 → 1): Comprehensive test
      - Estimated reduction: 60 lines of code
      - Better behavioral coverage
   
   3. Combine error classification tests (3 → 1): Property testing
      - Estimated reduction: 45 lines of code
      - More robust validation

🏗️ Test Infrastructure: ⚠️ NEEDS IMPROVEMENT
⚠️ Missing test fixtures:
   📍 create_test_context() - Used in 8 tests
   📍 create_test_errors() - Used in 6 tests
   📍 create_test_config() - Used in 4 tests

🔄 Testing Strategy: ⚠️ IMBALANCED
⚠️ Testing approach analysis:
   - Unit tests: 86% (too many small tests)
   - Integration tests: 9% (too few real scenarios)
   - Property-based: 5% (missing relationship testing)

📈 Optimization Potential:
Current: 21 tests, 315 lines, 4.2s execution
Optimized: 7 tests, 180 lines, 2.8s execution
Reduction: 67% test count, 43% code reduction, 33% faster

🎯 Quality Score: 6.2/10 (NEEDS OPTIMIZATION)

🔧 Recommended Actions:
1. Create test fixture infrastructure
2. Consolidate related functionality into comprehensive tests
3. Add property-based testing for complex behaviors
4. Balance unit vs integration testing approach

Generate optimization PR? (y/n)
```

### **Example Output (Well-Optimized Tests)**
```
🧪 Test Quality and Optimization Analysis

📊 Current Test Metrics:
- Total tests: 7
- Average test length: 25 lines
- Setup code duplication: 12%
- Execution time: 2.1 seconds
- Test categories: 4 comprehensive, 2 integration, 1 property-based

🔍 Duplication Analysis: ✅ MINIMAL DUPLICATION
✅ Well-organized test fixtures:
   - Centralized test data creation
   - Reusable setup functions
   - Clean test organization

🎯 Test Organization: ✅ EXCELLENT
✅ Comprehensive test coverage:
   1. Full component behavior testing
   2. Data-driven validation testing
   3. Property-based relationship testing
   4. Integration scenario testing

🏗️ Test Infrastructure: ✅ MATURE
✅ Effective test fixtures:
   - create_test_context() - Centralized setup
   - create_test_errors() - Standardized error creation
   - Comprehensive test utilities

🔄 Testing Strategy: ✅ WELL-BALANCED
✅ Optimal testing approach:
   - Comprehensive tests: 57% (covers full behavior)
   - Integration tests: 29% (real usage patterns)
   - Property-based: 14% (relationship validation)

📈 Performance Metrics:
- Fast execution: 2.1 seconds total
- Clear test intent: 95% readability score
- Maintainability: Excellent (low duplication)
- Coverage efficiency: 100% with minimal tests

🎯 Quality Score: 9.4/10 (EXCELLENT)

✨ This test suite demonstrates best practices:
- Strategic consolidation over fragmentation
- Comprehensive behavioral testing
- Efficient test fixtures
- Property-based validation
- Integration over isolation

No optimizations needed - this is a model test suite!
```

## 🔍 Detailed Analysis Algorithms

### **Duplication Detection**
```rust
impl DuplicationAnalyzer {
    fn analyze_setup_duplication(&self) -> DuplicationReport {
        let mut patterns = HashMap::new();
        
        for test in &self.tests {
            // Extract setup patterns
            let setup_lines = test.lines.iter()
                .take_while(|line| !line.contains("assert"))
                .collect::<Vec<_>>();
            
            let setup_signature = self.normalize_setup(&setup_lines);
            patterns.entry(setup_signature)
                .or_insert_with(Vec::new)
                .push(test.name.clone());
        }
        
        // Find patterns used multiple times
        let duplicated_patterns: Vec<_> = patterns.into_iter()
            .filter(|(_, tests)| tests.len() > 1)
            .collect();
        
        DuplicationReport {
            total_duplicated_lines: self.calculate_duplication(&duplicated_patterns),
            fixture_opportunities: self.suggest_fixtures(&duplicated_patterns),
            consolidation_potential: self.analyze_consolidation(&duplicated_patterns),
        }
    }
}
```

### **Consolidation Strategy**
```rust
impl ConsolidationAnalyzer {
    fn suggest_test_consolidations(&self) -> Vec<ConsolidationSuggestion> {
        let mut suggestions = Vec::new();
        
        // Group tests by functionality
        let functional_groups = self.group_by_functionality();
        
        for (functionality, tests) in functional_groups {
            if tests.len() > 2 && self.can_consolidate(&tests) {
                let estimated_reduction = self.calculate_reduction(&tests);
                suggestions.push(ConsolidationSuggestion {
                    functionality: functionality.clone(),
                    current_tests: tests.len(),
                    suggested_tests: 1,
                    reduction_percentage: estimated_reduction,
                    consolidation_type: self.determine_consolidation_type(&tests),
                });
            }
        }
        
        suggestions.sort_by(|a, b| b.reduction_percentage.cmp(&a.reduction_percentage));
        suggestions
    }
}
```

### **Property-Based Testing Analysis**
```rust
impl PropertyTestAnalyzer {
    fn identify_property_opportunities(&self) -> Vec<PropertyOpportunity> {
        let mut opportunities = Vec::new();
        
        for test in &self.tests {
            // Look for specific value assertions that could be properties
            let specific_value_assertions = test.extract_value_assertions();
            
            if specific_value_assertions.len() > 2 {
                let property = self.infer_property_from_assertions(&specific_value_assertions);
                opportunities.push(PropertyOpportunity {
                    current_tests: specific_value_assertions,
                    suggested_property: property,
                    benefits: vec![
                        "Tests broader range of inputs".to_string(),
                        "More robust validation".to_string(),
                        "Fewer brittle tests".to_string(),
                    ],
                });
            }
        }
        
        opportunities
    }
}
```

## 🔧 Optimization Generation

### **Fixture Creation**
```rust
// Generated test fixture recommendations
fn generate_test_fixtures(&self) -> String {
    format!(r#"
// Suggested test fixtures for {module}

fn create_test_context() -> ErrorContext {{
    ErrorContext::new("test_operation")
        .with_message_id("msg_123")
        .with_tool_use_id("tool_456")
        .add_metadata("test_key", "test_value")
}}

fn create_test_errors() -> Vec<ClaudeError> {{
    vec![
        ClaudeError::TimeoutError {{ duration: Duration::from_secs(30), context: None }},
        ClaudeError::RateLimitError {{ retry_after: Some(60), context: None }},
        ClaudeError::ValidationError {{ message: "test error".to_string(), context: None }},
    ]
}}

fn create_test_config() -> AppConfig {{
    AppConfig::default()
        .with_validation_limits(ValidationLimits::test_defaults())
}}
"#, module = self.module_name)
}
```

### **Consolidation Templates**
```rust
// Generated comprehensive test template
fn generate_comprehensive_test(&self, tests: &[Test]) -> String {
    format!(r#"
#[test]
fn test_{functionality}_comprehensive() {{
    let fixtures = create_test_fixtures();
    
    // Test initial state and basic functionality
    {initial_tests}
    
    // Test error scenarios and edge cases
    {error_tests}
    
    // Test state transitions and recovery
    {state_tests}
    
    // Test integration and real usage patterns
    {integration_tests}
}}
"#, 
        functionality = self.functionality_name,
        initial_tests = self.generate_initial_tests(tests),
        error_tests = self.generate_error_tests(tests),
        state_tests = self.generate_state_tests(tests),
        integration_tests = self.generate_integration_tests(tests),
    )
}
```

## 📊 Test Quality Metrics

### **Optimization Scoring**
- **Duplication Reduction** (30%): Amount of duplicated setup eliminated
- **Test Consolidation** (25%): Fragmented tests combined effectively
- **Property Coverage** (20%): Relationship and invariant testing
- **Integration Balance** (15%): Real usage vs isolated testing
- **Execution Efficiency** (10%): Test speed and resource usage

### **Quality Levels**
- **EXCELLENT** (9.0+): Optimal test organization, minimal duplication
- **GOOD** (7.0-8.9): Well-organized with minor optimization opportunities
- **NEEDS OPTIMIZATION** (5.0-6.9): Significant duplication and fragmentation
- **POOR** (<5.0): Extensive duplication, poor organization

## 🔗 Integration with Development

### **Command Integration**
- **[`/start-feature`](./start-feature.md)** - Test strategy planning
- **[`/review-pr`](./review-pr.md)** - Test quality in code review
- **[`/qa-check`](./qa-check.md)** - Test execution validation

### **Optimization Workflow**
```bash
# Test optimization workflow
/test-review "error_handling"     # Analyze current tests
→ Review suggestions              # Understand optimization opportunities
→ Implement consolidations        # Apply strategic consolidation
→ /qa-check                      # Verify all tests still pass
→ /test-review                   # Confirm improvements
```

## ⚠️ Optimization Guidelines

### **When to Consolidate**
- [ ] Tests share similar setup code
- [ ] Tests are testing related functionality on same component
- [ ] Tests would be clearer if grouped to show complete behavior
- [ ] Individual tests are very short and don't justify separate functions

### **When to Keep Separate**
- [ ] Tests cover completely different components or concerns
- [ ] Tests have significantly different setup requirements
- [ ] Failure of one test should not mask failures in others
- [ ] Tests are already comprehensive and well-organized

### **Quality Targets**
- **Test count reduction**: 50-70% through strategic consolidation
- **Code duplication**: <15% through effective fixtures
- **Execution speed**: 30-50% improvement through efficiency
- **Maintainability**: Improved through comprehensive behavioral testing

This command applies the proven optimization patterns that achieved 67% test reduction while maintaining 100% coverage, ensuring test suites are efficient, maintainable, and comprehensive.