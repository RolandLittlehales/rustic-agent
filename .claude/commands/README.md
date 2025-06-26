# Command Reference Guide

This directory contains all available slash commands for Claude Code development workflows.

## 🎯 Quick Command Overview

### Workflow Selection
- **Simple Work**: `/pick-next-ticket` → `/work-on-ticket` (bug fixes, small improvements)
- **Feature Work**: `/pick-next-feature` → `/start-feature` (new systems, major features)

### Quality Assurance
- **Pre-commit**: `/qa-check` (build, test, lint validation)
- **Code Review**: `/review-pr` (comprehensive standards enforcement)

### Development Support
- **Environment**: `/dev-checklist` (setup validation)
- **PR Creation**: `/create-pr` (professional PR formatting)

## 📋 Command Categories

### 🎯 Workflow Commands

| Command | Arguments | Purpose |
|---------|-----------|---------|
| [`/pick-next-ticket`](./pick-next-ticket.md) | None | Auto-select next simple ticket |
| [`/pick-next-feature`](./pick-next-feature.md) | None | Auto-select next major feature |
| [`/work-on-ticket`](./work-on-ticket.md) | `$ARGS` | Quick focused implementation |
| [`/start-feature`](./start-feature.md) | `$ARGS` | Comprehensive feature development |

### ✅ Quality Assurance Commands

| Command | Arguments | Purpose |
|---------|-----------|---------|
| [`/qa-check`](./qa-check.md) | None | Pre-commit validation |
| [`/review-pr`](./review-pr.md) | `$ARGS` | Code review with standards |
| [`/security-check`](./security-check.md) | None | Security validation |
| [`/config-check`](./config-check.md) | None | Configuration consistency |

### 🛠️ Development Support Commands

| Command | Arguments | Purpose |
|---------|-----------|---------|
| [`/dev-checklist`](./dev-checklist.md) | None | Environment validation |
| [`/docs-check`](./docs-check.md) | `$ARGS` | Documentation standards |
| [`/create-pr`](./create-pr.md) | `$ARGS` | Professional PR creation |
| [`/test-review`](./test-review.md) | `$ARGS` | Test optimization analysis |

## 🔄 Command Workflows

### Simple Ticket Development
```bash
/pick-next-ticket
# Auto-selects simple ticket → confirms with user → triggers:
/work-on-ticket "#12"
# Quick implementation → /qa-check → /create-pr
```

### Feature Development
```bash
/pick-next-feature
# Auto-selects major feature → confirms with user → triggers:
/start-feature "#15" 
# Full workflow: docs planning → implementation → comprehensive review → /create-pr
```

### Quality Gates
```bash
/qa-check && /security-check && /config-check
# Comprehensive validation before PR submission
```

## 📖 Command Usage Patterns

### **Ticket Selection**
- Use `/pick-next-ticket` when you want to work on the next priority item quickly
- Use `/pick-next-feature` when you're ready for comprehensive feature development
- Manual selection: `/work-on-ticket "#X"` or `/start-feature "#Y"`

### **Quality Validation**
- Always run `/qa-check` before commits
- Use `/review-pr` for thorough code quality assessment
- Run `/security-check` for any security-related changes

### **Environment Management**
- Start each session with `/dev-checklist`
- Use `/config-check` when modifying configuration
- Use `/docs-check` when creating/updating documentation

## 🎨 Command Arguments

### **Ticket/Issue References**
```bash
/work-on-ticket "#7"                    # GitHub issue number
/work-on-ticket "ContentBlock system"   # Feature description
/start-feature "#15 error handling"     # Issue + description
```

### **PR References**
```bash
/review-pr "#25"                        # PR number
/review-pr "feature/content-block"      # Branch name
/review-pr "current changes"            # Current working changes
```

### **Documentation Paths**
```bash
/docs-check "architecture"              # Specific documentation area
/docs-check ".claude/docs/development"  # Specific directory
/test-review "error_handling"           # Specific module
```

## 🔗 Integration with Documentation

Each command references relevant documentation:

- **Workflow commands** integrate with [GitHub workflows](../.claude/docs/github/)
- **Quality commands** enforce [development standards](../.claude/docs/development/)
- **Architecture commands** validate [system design](../.claude/docs/architecture/)
- **All commands** leverage [learnings and patterns](../.claude/docs/learnings/)

## 💡 Tips for Effective Usage

### **Command Chaining**
Commands can be used together in the same request:
```bash
/qa-check
# If it passes, then:
/create-pr "Fix file watcher console spam"
```

### **Workflow Selection**
- **Use simple workflow** for: bug fixes, small enhancements, maintenance
- **Use feature workflow** for: new systems, breaking changes, architectural work

### **Quality First**
- Never skip `/qa-check` before commits
- Use `/review-pr` to catch issues before they reach production
- Regular `/dev-checklist` ensures consistent environment

For detailed usage of any command, click the command name in the tables above or browse the individual command files in this directory.