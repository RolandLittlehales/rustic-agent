# Pick Next Ticket

Automatically select and start the next available ticket for quick implementation.

## 🎯 Purpose

This command automates ticket selection for **simple development work** like bug fixes, small improvements, and maintenance tasks. It filters for work that can be completed efficiently without heavy process overhead.

## 🔄 Process

1. **Check Sequencing**: Review implementation-sequence.md or project documentation for ticket ordering
2. **Filter for Simple Work**: 
   - Bug fixes and small enhancements
   - Maintenance tasks and code cleanup
   - Configuration updates
   - Documentation fixes
   - Quick security patches
3. **Dependency Validation**: Find first unassigned ticket without blocking dependencies
4. **GitHub Assignment**: Assign selected ticket to self on GitHub
5. **User Confirmation**: Present selected ticket and wait for confirmation
6. **Workflow Trigger**: If confirmed, automatically execute `/work-on-ticket` workflow

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

## ⚙️ Configuration

The command considers these factors when selecting tickets:

### **Priority Scoring**
- **Age**: Older tickets get higher priority
- **Impact**: User-facing issues prioritized
- **Complexity**: Simpler tickets preferred for this workflow
- **Dependencies**: Tickets without blockers preferred

### **Project Integration**
- Reads from `implementation-sequence.md` if available
- Considers GitHub project boards and milestones
- Respects issue labels for filtering

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