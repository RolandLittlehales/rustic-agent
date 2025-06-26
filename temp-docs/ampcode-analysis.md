# AmpCode Implementation Analysis

## Key Architecture Findings

### Multi-Tier Architecture
- **Client Layer**: VS Code extension/CLI providing UI and local context management
- **Server Layer**: Multi-tenant cloud service (ampcode.com) on Google Cloud Platform  
- **Model Layer**: Claude Sonnet 4 integration with tool orchestration

### Anthropic Model Integration
- Uses Claude Sonnet 4 exclusively ("No model selector, always the best models")
- Designed for "unconstrained token usage" to leverage model capabilities
- Built with anticipation of rapid AI model evolution

### Tool System Architecture
```go
type Agent struct {
    client *anthropic.Client
    tools  []ToolDefinition
}

type ToolDefinition struct {
    Name        string
    Description string
    InputSchema toolInputSchema
    Function    func(input json.RawMessage) (string, error)
}
```

### Developer Experience Features
- **@ File Mentions**: Fuzzy search file references in prompts
- **AGENT.md Support**: Project-specific guidance files
- **Image Upload**: Screenshots and diagrams for visual context
- **Automatic Tool Suggestion**: Context-aware tool recommendations

### Context Management Strategy
- Tracks context usage approaching 100k token limits
- Implements "Compact Thread" and "New Thread with Summary" strategies
- Warns about "doom loops" when context becomes too large

## References
- [AmpCode Architecture Guide](https://ampcode.com/how-to-build-an-agent)
- [AmpCode Documentation](https://ampcode.com/docs)
- Analysis conducted: 2024-12-21