# CLAUDE.md - Development Hub

## 🚨 IMPORTANT UPDATE 🚨

**This file has been reorganized and streamlined. The comprehensive documentation is now centralized in `.claude/` directory for better organization and maintainability.**

## 🚀 Quick Start

- **Environment Setup**: Use `/dev-checklist` to verify your development environment
- **Simple Work**: Use `/pick-next-ticket` for bug fixes and small improvements  
- **Feature Work**: Use `/pick-next-feature` for major features and comprehensive development
- **All Commands**: See [Command Reference](.claude/commands/README.md)

## 📚 Documentation Hub

**All detailed documentation is now located in `.claude/docs/`:**

- **[`.claude/CLAUDE.md`](.claude/CLAUDE.md)** - Complete development hub with architecture overview
- **[`.claude/docs/`](.claude/docs/)** - Comprehensive documentation with cross-references
- **[`.claude/commands/`](.claude/commands/)** - All available slash commands

## 🎯 Essential Commands

### **Quick Workflow Selection**
```bash
/pick-next-ticket     # Simple work (bugs, small improvements)  
/pick-next-feature    # Major features (comprehensive development)
```

### **Quality Assurance** 
```bash
/qa-check            # Pre-commit validation (build, test, lint)
/review-pr           # Comprehensive code review
```

### **Development Support**
```bash
/dev-checklist       # Environment validation
/create-pr           # Professional PR creation
```

## 🔧 Development Commands

```bash
# Development mode (requires Claude API key)
npm run dev -- --key YOUR_API_KEY
# or with env var
CLAUDE_API_KEY=your_key npm run dev

# Build and quality checks
npm run build
cargo build && cargo fmt && cargo clippy && cargo test
```

## 📋 Critical Principles

1. **Configuration-First**: No hardcoded values, centralized configuration management
2. **Type Safety**: Let the compiler prevent configuration and validation errors  
3. **Layered Security**: Intelligent validation that enables legitimate use cases
4. **Tauri v2 Discipline**: Always use object parameters, verify signatures
5. **Fast Feedback**: Build early and often to catch issues immediately

## ⚡ Quality Gates

**Before Any Commit**:
```bash
cargo build && npm run build  # Must succeed
cargo fmt && cargo clippy     # Must be clean  
cargo test                    # All tests must pass
```

**Use `/qa-check` to automate this validation.**

## 🎯 Getting Started

1. **Environment**: Run `/dev-checklist` to validate setup
2. **Pick Work**: Use `/pick-next-ticket` (simple) or `/pick-next-feature` (comprehensive)  
3. **Quality Check**: Use `/qa-check` before commits
4. **Submit**: Use `/create-pr` for professional PR creation

---

**For comprehensive guidance, architecture details, development standards, and complete command reference, see the centralized documentation in `.claude/`**

- **Main Hub**: [`.claude/CLAUDE.md`](.claude/CLAUDE.md)
- **Documentation**: [`.claude/docs/README.md`](.claude/docs/README.md)  
- **Commands**: [`.claude/commands/README.md`](.claude/commands/README.md)