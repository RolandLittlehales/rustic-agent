# Documentation Index

This directory contains comprehensive technical documentation for the project.

## 📂 Directory Structure

### [Architecture](./architecture/)
System design and architectural decisions
- [System Overview](./architecture/overview.md) - Core components and patterns
- [Configuration System](./architecture/configuration-system.md) - Three-tier config architecture
- [Security Model](./architecture/security-model.md) - Whitelist-based security design
- [Tauri Patterns](./architecture/tauri-patterns.md) - Tauri v2 specific implementations

### [Development](./development/)
Workflows, standards, and development guidelines
- [Setup Guide](./development/setup-guide.md) - Development environment setup
- [Workflow Process](./development/workflow-process.md) - 4-phase development process
- [Rust Standards](./development/rust-standards.md) - Rust-specific best practices
- [Testing Guidelines](./development/testing-guidelines.md) - Testing strategies and patterns
- [Quality Gates](./development/quality-gates.md) - Quality assurance requirements

### [GitHub](./github/)
Process documentation for collaboration
- [Issue Templates](./github/issue-templates.md) - Standardized issue creation
- [PR Guidelines](./github/pr-guidelines.md) - Pull request standards
- [Project Management](./github/project-management.md) - Ticket workflows and sequencing

### [Learnings](./learnings/)
Implementation insights and battle-tested patterns
- [Error Handling Insights](./learnings/error-handling-insights.md) - Error handling patterns and lessons
- [Configuration Patterns](./learnings/configuration-patterns.md) - Config system design principles
- [Test Optimization](./learnings/test-optimization.md) - Test quality and optimization strategies
- [Development Principles](./learnings/development-principles.md) - Abstract principles and checklists

## 🔗 Cross-References

### Command Integration
Each documentation section references relevant commands:
- Architecture docs mention `/config-check` and `/security-check`
- Development docs reference `/qa-check` and `/dev-checklist`
- GitHub docs integrate with `/create-pr` and `/pick-next-ticket`
- Learnings inform `/review-pr` and `/test-review`

### External Resources
- [Anthropic API Documentation](https://docs.anthropic.com/)
- [Tauri v2 Documentation](https://tauri.app/v1/guides/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Claude Code Documentation](https://docs.anthropic.com/en/docs/claude-code)

## 📋 Documentation Standards

- **Format**: Markdown with consistent headings
- **Code Examples**: Syntax highlighting with working examples
- **Cross-References**: Internal links between related docs
- **External Links**: References to official documentation
- **Maintainability**: Clear ownership and update patterns

For quick reference, see the main [CLAUDE.md](../.claude/CLAUDE.md) hub file.