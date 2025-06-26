# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
# Development mode (requires Claude API key)
npm run dev -- --key YOUR_API_KEY
# or with env var
CLAUDE_API_KEY=your_key npm run dev

# Build production bundle
npm run build

# Rust-specific commands
cd src-tauri
cargo build          # Debug build
cargo build --release # Release build
cargo run            # Run without Tauri
cargo test           # Run tests
cargo fmt            # Format code
cargo clippy         # Lint code
cargo clean          # Clean build cache
```

## Architecture Overview

This is a **Tauri-based desktop application** that provides a chat interface for Claude AI with file system tools.

### Core Components

**Backend (Rust)**:
- `src-tauri/src/main.rs` - Tauri app entry point with command handlers
- `src-tauri/src/claude/` - Claude API integration module
  - `client.rs` - HTTP client for Claude API with tool execution
  - `tools.rs` - Extensible tool registry system (file read/write/list)
  - `types.rs` - Type definitions for Claude API
  - `mod.rs` - Configuration and conversation management

**Frontend (Vanilla JS)**:
- `ui/index.html` - Main application interface
- `ui/js/app.js` - Chat UI, file explorer, and Tauri command bindings
- `ui/css/styles.css` - Modern styling

**Key Architecture Patterns**:
- **Thread-safe state**: `Arc<Mutex<>>` for conversation and config
- **Tool system**: Trait-based extensible tools registered in `ToolRegistry`
- **Async communication**: Tauri commands bridge JS frontend to Rust backend
- **API key injection**: Development script injects API key into frontend

### Tool System

Tools implement the `AgentTool` trait in `src-tauri/src/claude/tools.rs`:
- `ReadFileTool` - Read file contents
- `WriteFileTool` - Write file contents  
- `ListDirectoryTool` - List directory contents

To add new tools, implement `AgentTool` and register in `ClaudeClient::new()`.

### State Management

- **AppState**: Manages Claude client, conversation, and config
- **Conversation**: Thread-safe message history with timestamps
- **ClaudeConfig**: API key, model settings, temperature

### API Integration

- Uses `reqwest` for HTTP client with 120s timeout
- Converts internal conversation format to Claude API messages
- Handles tool execution within Claude responses
- Model: `claude-3-5-sonnet-20241022` (configurable)

## Key Development Notes

- **API Key Required**: Application will not function without valid Claude API key
- **Tauri Commands**: All backend functions exposed via `#[tauri::command]`
- **Development Script**: `scripts/dev.js` handles API key injection and cleanup
- **Build Target**: Cross-platform desktop (Windows, macOS, Linux)
- **WebView**: Uses system WebView with CSP security policies

## Implementation Learnings

This section documents key learnings from implementing features in this codebase, particularly from issue #31 (Quality of Life Improvements). These insights should guide future development work.

### Planning Phase Best Practices

#### Pre-Implementation Analysis Checklist

Before starting any feature implementation:

1. **Comprehensive Codebase Analysis**
   ```bash
   # Read all existing modules thoroughly
   find src -name "*.rs" | xargs grep -l "keyword"
   
   # Run existing tests to understand current functionality
   cargo test
   
   # Check for existing related features
   rg "logging|monitor|track" src/
   ```

2. **Current State Documentation**
   - Map what already exists vs. what the issue requests
   - Identify actual gaps rather than assumed gaps
   - Document existing architecture before proposing changes

3. **Scope Definition**
   - Define "done" criteria before starting
   - List what will NOT be implemented (out of scope)
   - Set quality bars (zero warnings, all tests pass)

#### Common Planning Mistakes to Avoid

- **Assuming features don't exist** - Always verify what's already implemented
- **Vague todo items** - Be specific about what constitutes completion
- **No success criteria** - Define what "comprehensive" or "robust" means
- **Scope creep planning** - Resist adding "nice to have" features upfront

### Implementation Phase Guidelines

#### Code Quality Standards

**Zero Warnings Policy**
```bash
# Before any commit
cargo check  # Must pass with zero warnings
cargo clippy # Address all clippy suggestions
cargo test   # All tests must pass
```

**Dead Code Elimination**
- If Rust marks code as unused, delete it immediately
- Don't keep code "for future use" - implement when needed
- Every constant/function should have a purpose

#### Incremental Development Process

1. **One Feature at a Time**
   - Implement smallest possible change first
   - Test immediately after each change
   - Commit frequently with clear messages

2. **Validation at Each Step**
   ```bash
   # After each module/feature
   cargo check       # Fix warnings immediately
   cargo test        # Ensure no regressions
   git add -A        # Stage changes
   git commit -m "feat: specific change description"
   ```

3. **Resist Feature Creep**
   - Ask "Is this explicitly required?" before adding features
   - Focus on current requirements, not future possibilities
   - Build for today's needs, refactor when requirements change

### Technical Best Practices

#### Rust-Specific Guidelines

**Error Handling**
- Use `Result<T>` consistently for fallible operations
- Prefer `.context()` over bare error returns
- Only use `.unwrap_or()` or `.unwrap_or_default()` with safe fallbacks
- Never use bare `.unwrap()` in production code

**Type Safety**
- Use the type system to encode architectural decisions
- Make invalid states unrepresentable
- Leverage traits for extensible architecture (like `AgentTool`)

**Module Organization**
- Keep modules focused on single responsibilities
- Use `mod.rs` for module-level configuration and re-exports
- Separate concerns (constants, types, implementation)

#### Testing Strategy

**Test Categories**
- **Unit Tests**: Individual functions/modules in isolation
- **Integration Tests**: Multiple components with real dependencies
- **End-to-End Tests**: Complete user workflows

**What to Test**
- Business logic and error conditions
- Edge cases and boundary conditions
- Security-critical functionality
- NOT: Configuration values or trivial getters/setters

### Process Templates

#### Feature Implementation Checklist

**Before Starting:**
```
□ Analyzed existing codebase thoroughly
□ Documented current state vs. requirements
□ Defined success criteria and "done" definition
□ Identified minimum viable implementation
□ Zero warnings in current codebase
```

**During Implementation:**
```
□ Working on one feature at a time
□ Testing after each change
□ Fixing warnings immediately
□ Committing frequently with clear messages
□ Staying within defined scope
```

**Before Claiming "Done":**
```
□ All tests pass
□ Zero compiler warnings
□ No dead/unused code
□ Original problem solved completely
□ Code is maintainable by others
□ Documentation updated if needed
```

#### Issue Analysis Template

When working on GitHub issues:

1. **Current State Analysis**
   - What already exists?
   - What's the exact gap?
   - What's working but could be improved?

2. **Requirements Validation**
   - What problem are we solving?
   - How will we measure success?
   - What's explicitly out of scope?

3. **Implementation Strategy**
   - What's the minimum viable solution?
   - How will we validate it works?
   - What's the rollback plan if issues arise?

### Key Principles

**"Good Enough is Perfect"**
- Solve the problem completely, but solve nothing more
- Working code that ships beats perfect code that doesn't
- Refactor when new requirements emerge, not preemptively

**YAGNI (You Aren't Gonna Need It)**
- Only build what's explicitly required
- Resist the urge to future-proof
- Delete unused code immediately

**Maintenance Mindset**
- Write code you'd want to debug at 2 AM
- Ensure new team members can understand the code
- Consider what happens when things break

### Code Review & Cleanup Learnings

Based on the cleanup of issue #31, here are additional critical learnings about managing technical debt and conducting effective code reviews.

#### The Power of Incremental Cleanup

**What We Achieved:**
- Reduced compiler warnings from **24 to 5** (79% reduction)
- Removed **15+ unused constants and functions**
- Maintained **100% test coverage** (29 passing tests)
- Preserved **all valuable functionality** (API cost tracking, logging, etc.)

**Key Insight:** Small, focused cleanup efforts yield massive improvements in code quality without breaking functionality.

#### Warning Management Policy

**Zero Tolerance for New Warnings**
```bash
# Before any commit
cargo check  # Must show ≤5 warnings (baseline)
cargo test   # All tests must pass
```

**Warning Categories:**
1. **Fix Immediately**: Unused imports, obvious dead code
2. **Evaluate**: Unused functions that might have future value  
3. **Document**: Intentionally unused code (like extensibility methods)

**The "24 Warning Rule"**: If warnings exceed 10-15, you have a code quality crisis that needs immediate attention.

#### Technical Debt Identification Framework

**High-Priority Debt (Fix Now):**
- Compiler warnings about unused code
- Functions/constants with zero references
- Broken or commented-out tests
- Hardcoded unwrap() calls

**Medium-Priority Debt (Plan to Fix):**
- Over-complex abstractions with single use
- Large modules doing multiple things
- Missing error handling in edge cases

**Low-Priority Debt (Monitor):**
- Extensibility methods not yet used
- Security functions for future features
- Helper utilities for planned work

#### Effective Code Review Process

**Pre-Review Checklist:**
```bash
# Automated quality gates
cargo check      # No new warnings
cargo test       # All tests pass  
cargo clippy     # Address clippy suggestions
cargo fmt        # Consistent formatting

# Manual review gates
□ Every new function/constant is used
□ No dead code added
□ Tests cover new functionality
□ Documentation updated if needed
```

**Review Focus Areas:**
1. **Necessity**: "Is this code actually needed for the current requirements?"
2. **Simplicity**: "What's the simplest way to solve this problem?"
3. **Testability**: "How will we know if this breaks?"
4. **Maintainability**: "Will future developers understand this?"

#### Refactoring Strategy: The "Valuable Core" Approach

**Step 1: Identify the Valuable Core**
- What functionality provides real business value?
- What code is actively used and tested?
- What infrastructure enables future development?

**Step 2: Aggressive Dead Code Removal**
- Delete anything with zero references
- Remove failed experiments and over-engineering
- Eliminate speculative "future-proofing"

**Step 3: Clean Around the Edges**
- Fix imports and warnings
- Update tests to match reality
- Simplify module structure

**Critical Rule**: Never remove code that provides current business value, even if it seems over-engineered.

#### Testing During Cleanup

**Test-Driven Cleanup:**
1. Run tests before any changes (establish baseline)
2. Clean up one category at a time (constants, then functions, then imports)
3. Run tests after each category
4. Fix broken tests immediately
5. Never let test count decrease

**What to Test During Cleanup:**
- ✅ Keep testing business logic and edge cases
- ✅ Keep testing security-critical functionality  
- ❌ Remove tests for deleted functions
- ❌ Remove tests that only verify configuration values

#### Communication During Cleanup

**What to Preserve (User-Visible Value):**
- API cost tracking and monitoring
- Performance improvements
- Security features
- Operational visibility

**What to Remove (Internal Complexity):**
- Unused abstractions
- Speculative infrastructure  
- Over-engineered solutions
- Failed experiments

**Team Communication:**
- Be explicit about what's being removed and why
- Highlight what valuable functionality is preserved
- Document the improvement metrics (warnings reduced, etc.)

#### Metrics for Cleanup Success

**Quantitative Measures:**
- Compiler warnings (target: <10)
- Test coverage percentage
- Lines of code removed vs. added
- Build time improvements

**Qualitative Measures:**
- Code is easier to understand
- New developers can contribute faster
- Debugging is simpler
- Less cognitive overhead

#### Anti-Patterns to Avoid During Cleanup

**The "Preserve Everything" Trap**
- Keeping unused code "just in case"
- Afraid to delete abstractions that took time to build
- Treating all code as equally valuable

**The "Clean Slate" Trap**  
- Deleting working functionality
- Removing code that has business value
- Breaking tests in the name of "simplification"

**The "Perfect Refactor" Trap**
- Trying to fix everything at once
- Over-engineering the cleanup itself
- Spending more time refactoring than implementing features

#### The 80/20 Rule for Code Quality

**80% of quality improvements come from:**
- Removing obvious dead code
- Fixing compiler warnings
- Maintaining comprehensive tests
- Clear, focused modules

**20% comes from:**
- Perfect abstractions
- Theoretical future-proofing
- Complex architectural patterns
- Premature optimizations

**Focus on the 80% first.** It provides the highest return on investment and creates a solid foundation for future work.

#### Success Pattern: The "Cleanup Commit"

After major feature work, always do a focused cleanup:

```bash
git add feature-files
git commit -m "feat: implement user-requested functionality"

# Then cleanup
git add cleanup-files  
git commit -m "refactor: remove dead code and fix warnings"
```

This separates feature delivery from code quality maintenance, making both easier to review and understand.