# Pick Next Feature

Automatically select and start the next available major feature for comprehensive development with full professional standards.

## 🎯 Purpose

This command automates feature selection for **comprehensive development work** including new systems, major architectural changes, and complex implementations. It triggers the full professional workflow with extensive documentation and quality standards.

## 🔄 Process

1. **Check Sequencing**: Review implementation-sequence.md or project documentation for feature ordering
2. **Filter for Major Features**: 
   - New system implementations
   - Major architectural changes
   - Complex multi-file features
   - Breaking changes requiring migration
   - Features requiring new documentation
3. **Dependency Validation**: Find first unassigned feature without blocking dependencies
4. **GitHub Assignment**: Assign selected feature to self on GitHub
5. **User Confirmation**: Present selected feature and wait for confirmation
6. **Workflow Trigger**: If confirmed, automatically execute `/start-feature` workflow

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

### **Priority Scoring Factors**
- **Strategic importance**: Core system vs nice-to-have
- **Dependency readiness**: All blockers completed
- **Resource requirements**: Team capacity and expertise
- **Risk assessment**: Technical complexity and unknowns
- **Timeline alignment**: Project milestones and deadlines

### **Complexity Assessment**
- **Code scope**: Lines of code and files affected
- **Integration points**: Number of systems touched
- **Documentation needs**: Required documentation types
- **Testing requirements**: Complexity of test strategy
- **Migration impact**: Breaking changes and compatibility

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

### **Available Features Analysis**
```
🔍 Analyzing available features...

Feature #13: User Authentication System
❌ Blocked by API integration (#12) - In Progress

Feature #15: Advanced ContentBlock System  
✅ Dependencies complete (#14)
✅ High priority (core functionality)
✅ Well-defined scope
✅ Ready for implementation

Feature #18: Advanced Search Interface
⚠️ Lower priority (enhancement)
⚠️ UI/UX design pending

🎯 Recommendation: #15 - Best combination of readiness, priority, and impact
```

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