# Work on Ticket

Quick focused implementation workflow for specific tickets with streamlined process and basic quality checks.

## 🎯 Purpose

This command provides an **efficient development workflow** for implementing specific tickets without heavy process overhead. It focuses on getting quality work done quickly while maintaining essential quality gates.

## 🔄 Streamlined Process

1. **Ticket Validation**: Find and validate the specified ticket exists
2. **Quick Dependency Check**: Verify no blocking dependencies
3. **TodoWrite Planning**: Create focused implementation plan
4. **Incremental Implementation**: Begin coding with periodic testing
5. **Quality Validation**: Run `/qa-check` when implementation complete
6. **PR Preparation**: Ready for `/create-pr` workflow

## ⚡ Focus Areas

### **Efficiency**
- Minimal process overhead
- Direct path to implementation
- Quick feedback loops
- Essential quality gates only

### **Quality**
- TodoWrite for complex tasks
- Incremental testing approach
- Build verification at each step
- Pre-commit quality checks

### **Scope**
- Single ticket focus
- Clear boundaries
- Measurable completion
- Ready for PR submission

## 💡 Usage

### **Basic Usage**
```
/work-on-ticket $ARGUMENTS
```

### **Argument Formats**
```bash
/work-on-ticket "#7"                    # GitHub issue number
/work-on-ticket "Fix file watcher"      # Issue description
/work-on-ticket "#12 console spam"      # Issue number + description
/work-on-ticket "ContentBlock enum"     # Feature name
```

### **Example Execution**
```
🎯 Working on Ticket: #12 - Fix file watcher console spam

📋 Ticket Analysis:
✅ Ticket exists and is accessible
✅ No blocking dependencies found
✅ Ticket currently unassigned

🌿 Creating isolated work branch:
$ git checkout main
$ git pull origin main  
$ git checkout -b ticket/12-fix-file-watcher-spam
✅ Branch created: ticket/12-fix-file-watcher-spam

📝 Assigning to current user...

🔍 Quick Assessment:
- Files likely affected: src-tauri/src/file_watcher.rs
- Estimated complexity: Low-Medium  
- Type: Bug fix
- Expected time: 30-60 minutes

📝 Creating TodoWrite plan:
1. Analyze current file watcher logging
2. Identify verbose debug statements
3. Preserve error logging while reducing debug noise
4. Test file watching functionality still works
5. Run qa-check before completion

🚀 Ready to begin implementation!
```

## 📋 Detailed Workflow

### **Phase 1: Ticket Validation & Analysis**
```
✅ Verify ticket exists in GitHub
✅ Check ticket status and assignability
✅ Analyze dependencies and blockers
✅ Create new branch from main for isolated work:
   • git checkout main
   • git pull origin main
   • git checkout -b ticket/{ticket-number}-{short-description}
✅ Assign ticket to current user
✅ Understand scope and requirements
```

### **Phase 2: Implementation Planning**
```
📝 Create TodoWrite plan with specific steps
🎯 Identify files likely to be modified
⏱️ Estimate implementation time
🔍 Note any potential complications
📚 Review related code patterns
```

### **Phase 3: Incremental Implementation**
```
🔨 Make small, focused changes
🧪 Test after each significant change
📊 Build verification: cargo build && npm run build
🔄 Update TodoWrite progress as you go
🎯 Stay focused on ticket requirements
```

### **Phase 4: Quality & Completion**
```
✅ Run /qa-check for comprehensive validation
🔍 Verify all TodoWrite items completed
📝 Ensure ticket requirements fully met
🧹 Clean up any debugging code
🚀 Ready for /create-pr workflow
```

## 🎯 Best For

### **Ticket Types**
- 🐛 Bug fixes
- 🔧 Small feature enhancements
- 📝 Documentation updates
- ⚙️ Configuration changes
- 🧹 Code cleanup
- 🔒 Security patches

### **Development Scenarios**
- **Quick wins** - High value, low complexity work
- **Learning** - Understanding codebase through focused changes
- **Maintenance** - Keeping system healthy and up-to-date
- **Between features** - Productive work while waiting for dependencies

## ⚙️ Quality Gates

### **Continuous Validation**
- Build after each significant change
- Incremental testing approach
- TodoWrite progress tracking
- Code pattern consistency

### **Pre-Completion Checks**
- `/qa-check` must pass before considering complete
- All TodoWrite items marked complete
- Ticket requirements fully satisfied
- No regression in existing functionality

## 🔗 Command Integration

### **Triggered By**
- [`/pick-next-ticket`](./pick-next-ticket.md) - Automatic workflow trigger
- Manual execution for specific tickets

### **Integrates With**
- [`/qa-check`](./qa-check.md) - Quality validation before completion
- [`/create-pr`](./create-pr.md) - PR creation after implementation
- [`/dev-checklist`](./dev-checklist.md) - Environment validation if needed

### **Followed By**
```bash
/work-on-ticket "#12" → implementation → /qa-check → /create-pr "#12"
```

## 📊 Example TodoWrite Plan

```markdown
## TodoWrite Plan for #12 - Fix file watcher console spam

### High Priority
- [ ] Review current file_watcher.rs logging patterns
- [ ] Identify verbose console.log and println! statements  
- [ ] Preserve error logging (console.error, eprintln!)

### Medium Priority  
- [ ] Replace verbose debug with conditional logging
- [ ] Test file watching still works (Ctrl+T test)
- [ ] Verify build succeeds: cargo build && npm run build

### Completion
- [ ] Run /qa-check and ensure all checks pass
- [ ] Mark GitHub issue ready for PR
- [ ] Prepare for /create-pr workflow
```

## ⚠️ Important Notes

### **Scope Management**
- Stay focused on the specific ticket
- Avoid scope creep or additional "while I'm here" changes
- Create separate tickets for discovered issues

### **Quality Standards**
- All code must pass `/qa-check` before completion
- Follow existing code patterns and conventions
- Maintain or improve test coverage

### **Documentation**
- Update relevant documentation if behavior changes
- Add code comments for complex logic
- Keep CLAUDE.md current if process changes

## 🔄 Workflow Comparison

| Aspect | `/work-on-ticket` | [`/start-feature`](./start-feature.md) |
|--------|-------------------|-------------------------|
| **Process** | Streamlined | Comprehensive |
| **Documentation** | Update existing | Create new docs |
| **Testing** | Basic validation | Comprehensive strategy |
| **Review** | `/qa-check` | Full `/review-pr` |
| **Scope** | Single ticket | Major feature |
| **Time** | 30 minutes - 2 hours | Days to weeks |

## 💻 Example Session

```
User: /work-on-ticket "#12"