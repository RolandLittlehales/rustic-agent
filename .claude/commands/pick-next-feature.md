# Pick Next Feature

Automatically select and start the next available major feature for comprehensive development with full professional standards.

## 🎯 Purpose

This command automates feature selection for **comprehensive development work** including new systems, major architectural changes, and complex implementations. It triggers the full professional workflow with extensive documentation and quality standards.

## 🔄 Process

1. **Check GitHub Issues**: Get all open issues and check assignment status (assigned = in progress)
2. **Review Sequencing**: Follow implementation-sequence.md strict dependency order
3. **Find Next Available**: Select lowest-numbered unassigned issue with satisfied dependencies
4. **Filter for Major Features**: 
   - New system implementations (>500 lines)
   - Major architectural changes
   - Complex multi-component features
   - Breaking changes requiring migration
   - Features requiring comprehensive documentation
5. **GitHub Assignment**: Assign selected feature to self on GitHub
6. **User Confirmation**: Present selected feature and wait for confirmation
7. **Workflow Trigger**: If confirmed, automatically execute `/start-feature` workflow

## ✅ Selection Criteria

### **Included Feature Types**
- 🏗️ New system implementations (>500 lines)
- 🔄 Major architectural changes
- 📦 New tool integrations
- 💥 Breaking changes requiring migration
- 🔧 Complex multi-component features
- 📚 Features requiring comprehensive documentation
- 🔒 Security system implementations

### **Excluded Feature Types**
- 🐛 Simple bug fixes
- 🔧 Small improvements (<100 lines)
- 📝 Documentation-only updates
- ⚙️ Configuration tweaks
- 🧹 Code cleanup tasks

## 🎯 Best For

- **Major milestones** - Significant feature development
- **System evolution** - Adding new capabilities
- **Architecture work** - Fundamental system changes
- **Professional development** - Full documentation and testing standards
- **Complex implementations** - Multi-day or multi-week projects

## 💡 Usage

### **Basic Usage**
```
/pick-next-feature
```

**No arguments required** - the command automatically analyzes available major features.

### **Example Output**
```
🎯 Selected Feature: #15 - Implement Advanced ContentBlock System

📋 Details:
- Type: New system implementation
- Estimate: 2-3 weeks
- Complexity: High
- Files affected: 15+ files across backend and frontend
- Dependencies: Basic ContentBlock enum (#14) - ✅ Complete
- Current assignee: Unassigned

📝 Description:
Implement comprehensive ContentBlock system with:
- Advanced type variants (code, text, tool_use, etc.)
- Serialization/deserialization handling
- Frontend integration
- Comprehensive error handling
- Full documentation suite

📚 Documentation Requirements:
- Architecture documentation (system design)
- API reference (all types and methods)
- Integration guide (usage patterns)
- Migration guide (breaking changes)

🧪 Quality Requirements:
- Comprehensive unit tests
- Integration testing
- Property-based testing
- Performance benchmarking
- Security validation

✅ This feature is ready for comprehensive development!

🤔 Proceed with this feature? (y/n)
If yes, I'll assign it to you and start /start-feature workflow.
```

### **Workflow Integration**
If you confirm the selection, the command automatically:

1. **Assigns feature** to you on GitHub
2. **Triggers** `/start-feature "#15"`
3. **Begins** comprehensive development workflow with full professional standards

## 🔄 Workflow Sequence

```
/pick-next-feature
    ↓
🔍 Analyze available major features
    ↓
📋 Filter by complexity and scope
    ↓
🎯 Present best match with full context
    ↓
✅ User confirmation
    ↓
📝 Assign on GitHub
    ↓
🚀 /start-feature execution
```

## ⚙️ Selection Algorithm

### **Strict Sequencing Logic**
1. **GitHub Status Check**: `gh issue list --state open --json number,title,assignees`
2. **Dependency Validation**: Check implementation-sequence.md for prerequisites
3. **Assignment Filter**: Skip any issues with assignees (they're in progress)
4. **Sequential Selection**: Choose lowest-numbered unassigned issue with satisfied dependencies
5. **Complexity Verification**: Ensure issue meets major feature criteria (>500 LOC)

### **Key Validation Steps**
- **Prerequisites Complete**: All dependency issues must be CLOSED (not just assigned)
- **Unassigned Status**: No assignees means available for work
- **Sequence Order**: Follow numbered sequence (1.1 → 1.2 → 1.3 → 1.4 → 1.5 → 2.1...)
- **Major Feature Scope**: Architectural changes, new systems, complex integrations

### **Example Decision Process**
```
✅ Issue #6 (1.1): CLOSED - ContentBlock System complete
✅ Issue #7 (1.2): CLOSED - Error Handling complete  
🎯 Issue #8 (1.3): OPEN, unassigned, dependencies satisfied → SELECT
❌ Issue #9 (1.4): OPEN, but requires #8 completion
❌ Issue #10 (1.5): OPEN, but requires #7 AND #9
```

## 🎯 Feature Categories

### **New System Implementations**
```
Example: ContentBlock System, Error Handling Framework, Model Registry
- Complete new subsystems
- Multiple interacting components
- Extensive API surface
- Comprehensive documentation
```

### **Architectural Changes**
```
Example: Configuration system overhaul, Security model redesign
- Fundamental system modifications
- Cross-cutting concerns
- Migration strategies required
- Backward compatibility considerations
```

### **Integration Features**
```
Example: New Claude API features, External tool integrations
- External system interactions
- Protocol implementations
- Security considerations
- Robust error handling
```

## 📋 Project Integration

### **Documentation Dependencies**
The command considers:
- **Implementation sequence** from project documentation
- **Milestone alignment** with project roadmap
- **Resource allocation** and team capacity
- **Risk management** for complex features

### **GitHub Integration**
- **Issue analysis** for scope and requirements
- **Label-based filtering** for feature identification
- **Milestone tracking** for timeline management
- **Dependency mapping** via linked issues

## 🔗 Related Commands

- **[`/start-feature`](./start-feature.md)** - The comprehensive workflow triggered
- **[`/pick-next-ticket`](./pick-next-ticket.md)** - For simple work instead
- **[`/review-pr`](./review-pr.md)** - Quality validation used in workflow
- **[`/docs-check`](./docs-check.md)** - Documentation validation

## 📊 Example Selection Logic

### **Real Selection Process**
```
🔍 Checking GitHub issues and implementation sequence...

✅ Issue #6 [1.1]: CLOSED - Enhanced ContentBlock System
✅ Issue #7 [1.2]: CLOSED - Error Handling Framework  
🎯 Issue #8 [1.3]: OPEN, unassigned - Tool Result Handling
   Dependencies: 1.1 ✅ + 1.2 ✅ = READY
   Scope: Major system (~1,500 LOC)
   Impact: Foundation for all Phase 2 features

❌ Issue #9 [1.4]: OPEN, unassigned - Streaming Foundation
   Dependencies: 1.3 (not complete) = BLOCKED

❌ Issue #11 [2.1]: OPEN, unassigned - Parallel Tool Execution  
   Dependencies: 1.3 + 1.4 (both incomplete) = BLOCKED

🎯 Selection: #8 - Next in sequence with satisfied dependencies
```

### **Key Learnings Applied**
- **Follow numbered sequence**: Don't skip ahead to higher-numbered issues
- **Check actual GitHub status**: Closed = complete, assigned = in progress, unassigned = available
- **Validate dependencies strictly**: All prerequisites must be CLOSED before starting
- **Sequence over complexity**: Implementation order matters more than individual feature appeal

## ⚠️ Important Considerations

### **Commitment Level**
- **Significant time investment** - Features typically take days/weeks
- **Full professional standards** - Comprehensive documentation and testing
- **Architecture responsibility** - Decisions impact entire system
- **Quality expectations** - High standards for maintainability

### **Resource Requirements**
- **Deep focus time** - Complex problems requiring sustained attention
- **Documentation writing** - Substantial documentation creation
- **Testing strategy** - Comprehensive test planning and implementation
- **Review cycles** - Multiple rounds of feedback and refinement

### **Scope Management**
- **Clear boundaries** - Well-defined feature scope
- **MVP approach** - Core functionality first, enhancements later
- **Migration planning** - Breaking changes handled systematically
- **Rollback strategy** - Safe deployment and rollback procedures

## 💻 Example Session

```
User: /pick-next-feature