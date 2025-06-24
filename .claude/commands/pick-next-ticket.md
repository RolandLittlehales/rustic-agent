# Pick Next Ticket

Automatically select and start the next available ticket for quick implementation.

## 🎯 Purpose

This command automates ticket selection for **simple development work** like bug fixes, small improvements, and maintenance tasks. It filters for work that can be completed efficiently without heavy process overhead.

## 🔄 Process

1. **Check GitHub Issues**: Get all open issues and check assignment status (assigned = in progress)
2. **Filter for Simple Work**: 
   - Bug fixes and small enhancements (<100 LOC)
   - Maintenance tasks and code cleanup
   - Configuration updates and tweaks
   - Documentation fixes and improvements
   - Quick security patches (non-architectural)
3. **Apply Sequencing Logic**: Prefer lower-numbered issues but allow flexibility for simple work
4. **Dependency Validation**: Skip tickets blocked by major features or complex dependencies
5. **GitHub Assignment**: Assign selected ticket to self on GitHub
6. **User Confirmation**: Present selected ticket and wait for confirmation
7. **Workflow Trigger**: If confirmed, automatically execute `/work-on-ticket` workflow

## ✅ Selection Criteria

### **Included Ticket Types**
- 🐛 Bug fixes
- 🔧 Small improvements (< 100 lines of code)
- 📝 Documentation updates
- ⚙️ Configuration changes
- 🧹 Code cleanup and maintenance
- 🔒 Security patches (non-architectural)

### **Excluded Ticket Types**
- 🏗️ New system implementations
- 🔄 Major architectural changes
- 📦 New tool integrations
- 💥 Breaking changes
- 📚 Comprehensive documentation projects

## 🎯 Best For

- **Quick productivity** - Making progress on high-value, low-complexity work
- **Learning codebase** - Understanding system through focused changes
- **Between features** - Productive work while waiting for feature dependencies
- **Testing workflows** - Validating development processes on smaller scope

## 💡 Usage

### **Basic Usage**
```
/pick-next-ticket
```

**No arguments required** - the command automatically analyzes available work.

### **Example Output**
```
🎯 Selected Ticket: #12 - Fix file watcher console spam

📋 Details:
- Type: Bug fix
- Estimate: 30-60 minutes
- Files affected: src-tauri/src/file_watcher.rs
- Dependencies: None
- Current assignee: Unassigned

📝 Description:
File watcher is logging excessive debug information to console.
Need to reduce verbosity while preserving error logging.

✅ This ticket is ready to start!

🤔 Proceed with this ticket? (y/n)
If yes, I'll assign it to you and start /work-on-ticket workflow.
```

### **Workflow Integration**
If you confirm the selection, the command automatically:

1. **Assigns ticket** to you on GitHub
2. **Triggers** `/work-on-ticket "#12"`
3. **Begins** streamlined implementation workflow

## 🔄 Workflow Sequence

```
/pick-next-ticket
    ↓
📋 Analyze available tickets
    ↓
🎯 Present best match
    ↓
✅ User confirmation
    ↓
📝 Assign on GitHub
    ↓
🚀 /work-on-ticket execution
```

## ⚙️ Selection Algorithm

### **Flexible Sequencing for Simple Work**
1. **GitHub Status Check**: `gh issue list --state open --json number,title,assignees`
2. **Simple Work Filter**: Focus on maintenance, bugs, small improvements
3. **Assignment Filter**: Skip any issues with assignees (they're in progress)
4. **Dependency Check**: Avoid tickets blocked by major incomplete features
5. **Preference Logic**: Lower-numbered tickets preferred but not strict requirement

### **Key Differences from /pick-next-feature**
- **Flexible sequencing**: Can work on simple tickets out of strict order
- **Scope-based filtering**: <100 LOC, maintenance, non-architectural
- **Dependency awareness**: Avoid major feature dependencies but allow simple prerequisites
- **Speed optimization**: Select work that can be completed quickly

### **Example Decision Process**
```
🎯 Simple Work Available:
- Issue #4: Theming (simple CSS/UI work) ✅ Available
- Issue #5: Solid.js integration (architectural) ❌ Too complex
- Issue #25: Fix console logging (bug fix) ✅ Available  
- Issue #26: Update README (documentation) ✅ Available

🎯 Selection: #4 (lowest number) or #25 (bug priority)
```

### **Priority Factors**
- **Simplicity**: <100 LOC, single-file changes preferred
- **Independence**: Minimal dependencies on incomplete major features
- **User Impact**: Bug fixes and UX improvements prioritized
- **Sequence Awareness**: Prefer lower-numbered issues when all else equal

## 🔗 Related Commands

- **[`/work-on-ticket`](./work-on-ticket.md)** - The workflow triggered after selection
- **[`/pick-next-feature`](./pick-next-feature.md)** - For major feature development
- **[`/qa-check`](./qa-check.md)** - Quality validation used in workflow
- **[`/create-pr`](./create-pr.md)** - PR creation at end of workflow

## 📋 Dependencies

### **Required Information**
- GitHub repository with issues
- Implementation sequence documentation (optional)
- Proper issue labeling for filtering

### **GitHub Permissions**
- Read access to issues
- Write access to assign issues
- Repository contributor access

## 🎨 Customization

### **Filtering Preferences**
You can guide selection by mentioning preferences:
```
/pick-next-ticket
"I'd prefer working on frontend issues today"

/pick-next-ticket  
"Looking for Rust backend work"
```

### **Skip Certain Types**
```
/pick-next-ticket
"Skip documentation tasks for now"
```

## ⚠️ Important Notes

- **Automatic assignment** - Selected ticket is assigned to you on GitHub
- **Single selection** - Only picks one ticket at a time for focused work
- **Dependency awareness** - Won't suggest blocked tickets
- **Scope appropriate** - Only suggests work suitable for quick workflow

## 💻 Example Session

```
User: /pick-next-ticket