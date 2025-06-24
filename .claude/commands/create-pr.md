# Create PR

Professional pull request creation with comprehensive formatting, testing instructions, and quality assurance validation.

## 🎯 Purpose

This command creates **professional-grade pull requests** with comprehensive descriptions, extensive manual testing instructions, proper issue linking, and quality assurance validation. It ensures PRs meet enterprise standards for review and deployment.

## 🔄 PR Creation Process

### **Phase 1: Pre-Creation Validation**
1. **Quality Gate**: Run `/qa-check` and ensure all checks pass
2. **Issue Validation**: Find correct GitHub issue number (not sequence number)
3. **Duplicate Check**: Verify no existing PR for the same issue
4. **Change Analysis**: Analyze scope, complexity, and impact of changes
5. **Documentation Review**: Ensure all documentation is updated

### **Phase 2: Content Generation**
6. **PR Description**: Generate comprehensive description with all required sections
7. **Testing Instructions**: Create 6+ manual test scenarios with clear steps
8. **Quality Checklist**: Include completed quality assurance checklist
9. **Documentation Links**: Reference all created/updated documentation
10. **Implementation Summary**: Provide technical summary of changes

### **Phase 3: PR Creation**
11. **GitHub Command**: Generate ready-to-execute `gh pr create` command
12. **Preview Generation**: Show PR content for review before creation
13. **Execution Guidance**: Provide clear instructions for PR submission
14. **Follow-up Actions**: Outline next steps after PR creation

## 💡 Usage

### **Basic Usage**
```
/create-pr $ARGUMENTS
```

### **Argument Formats**
```bash
/create-pr "#7"                           # GitHub issue number
/create-pr "Fix file watcher spam"        # PR title
/create-pr "#12 console spam fix"         # Issue + title
/create-pr "ContentBlock implementation"  # Feature description
```

### **Example Execution**
```
🚀 Creating Professional Pull Request

📋 Pre-Creation Validation:
✅ Running /qa-check validation...
✅ All quality checks passed
✅ Issue #12 exists and is valid
✅ No existing PR found for issue #12
✅ Changes analyzed: 3 files, 45 additions, 12 deletions

🔍 Change Analysis:
- Type: Bug fix
- Complexity: Low-Medium
- Files modified: src-tauri/src/file_watcher.rs, ui/js/app.js, CLAUDE.md
- Breaking changes: None
- Security impact: None
- Documentation updates: Yes (CLAUDE.md updated)

📝 Generating comprehensive PR description...

🧪 Creating manual testing instructions...
Generated 6 test scenarios covering:
- Basic functionality validation
- Error handling verification  
- Integration testing
- Configuration validation
- Security validation
- Documentation accuracy

✅ PR Content Ready for Review!
```

## 📋 PR Description Template

### **Generated PR Structure**
```markdown
## Summary

Fixes #12

Brief description of what the issue was and how this PR addresses it.

## Implementation Details

### Changes Made
- **File watcher logging**: Reduced verbose debug output while preserving error logging
- **Console cleanup**: Removed unnecessary success confirmations
- **Documentation**: Updated CLAUDE.md with logging best practices

### Technical Approach
- Replaced `console.log` with conditional logging based on environment
- Preserved all `console.error` and critical error reporting
- Added configuration flag for debug verbosity control

### Files Modified
- `src-tauri/src/file_watcher.rs` - Reduced debug logging verbosity
- `ui/js/app.js` - Cleaned up console output
- `CLAUDE.md` - Updated logging guidelines

## Manual Testing Instructions

### Test 1: Basic File Watching Functionality
1. **Start the application**: Run `npm run dev -- --key YOUR_API_KEY`
2. **Open file explorer**: Verify file tree loads correctly
3. **Test file watching**: Create/modify files in watched directory
4. **Verify updates**: Confirm file changes appear in UI without console spam

**Expected Result**: File watching works correctly with minimal console output

### Test 2: Error Handling Validation
1. **Trigger file access error**: Try to access restricted file
2. **Monitor console**: Verify error is logged appropriately
3. **Check error formatting**: Ensure error messages are user-friendly
4. **Verify recovery**: Confirm application continues working after error

**Expected Result**: Errors properly logged without exposing sensitive information

### Test 3: Development vs Production Logging
1. **Development mode**: Run with dev environment variables
2. **Check console output**: Verify appropriate debug level
3. **Production simulation**: Test with production-like settings
4. **Compare outputs**: Confirm logging levels differ appropriately

**Expected Result**: Logging verbosity appropriate to environment

### Test 4: File Watcher Performance
1. **Large directory test**: Open directory with 100+ files
2. **Rapid changes**: Make multiple quick file modifications
3. **Monitor performance**: Check for lag or excessive processing
4. **Console monitoring**: Verify no performance-related spam

**Expected Result**: Smooth performance without console flooding

### Test 5: Security Validation
1. **Path traversal attempt**: Try accessing files outside whitelist
2. **Monitor logging**: Verify security events properly logged
3. **Error messages**: Confirm no sensitive path information exposed
4. **Access control**: Verify whitelist enforcement working

**Expected Result**: Security events logged safely without information disclosure

### Test 6: Documentation Accuracy
1. **Review updated docs**: Check CLAUDE.md logging section
2. **Follow examples**: Test code patterns from documentation
3. **Verify alignment**: Ensure docs match actual implementation
4. **Cross-references**: Check all links work correctly

**Expected Result**: Documentation accurate and helpful

## Quality Assurance

### Automated Checks
- ✅ cargo build - Success (0 warnings)
- ✅ cargo clippy - No suggestions
- ✅ cargo fmt - No changes needed
- ✅ npm run build - Success
- ✅ cargo test - All tests pass

### Code Quality
- ✅ No hardcoded values introduced
- ✅ Error handling patterns followed
- ✅ Security guidelines maintained
- ✅ Configuration system used appropriately
- ✅ Documentation updated with changes

### Security Review
- ✅ No sensitive information in logs
- ✅ Error messages sanitized appropriately
- ✅ File access patterns secure
- ✅ No new security vulnerabilities

### Performance Impact
- ✅ No performance regressions
- ✅ Reduced console output improves performance
- ✅ File watching efficiency maintained
- ✅ Memory usage unchanged

## Documentation

### Updated Documentation
- [CLAUDE.md](./CLAUDE.md) - Logging best practices section updated
- Code comments added for logging configuration

### Related Documentation
- [Development Standards](./.claude/docs/development/rust-standards.md)
- [Quality Gates](./.claude/docs/development/quality-gates.md)

## Breaking Changes

None. This is a backward-compatible improvement.

## Migration Guide

No migration required. Changes are transparent to users.

---
🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

## 🎯 Manual Testing Strategy

### **Test Scenario Categories**
1. **Basic Functionality** - Core feature works correctly
2. **Error Handling** - Failure scenarios handled gracefully
3. **Integration** - Works with existing systems
4. **Configuration** - Settings and validation work
5. **Security** - No sensitive data exposure
6. **Documentation** - New docs are accurate and linked

### **Test Step Format**
Each test scenario includes:
- **Clear numbered steps** (3-4 steps per test)
- **Specific commands** to execute
- **Expected results** for verification
- **Success criteria** for pass/fail determination

### **Comprehensive Coverage**
- **Happy path testing** - Normal usage scenarios
- **Edge case testing** - Boundary conditions
- **Error scenario testing** - Failure modes
- **Integration testing** - System interactions
- **Performance testing** - Load and stress scenarios
- **Security testing** - Vulnerability verification

## 🔍 Pre-Creation Validation

### **Quality Gate Requirements**
```
🔧 Pre-Creation Checklist:
✅ /qa-check passes completely
✅ All tests passing
✅ No compiler warnings
✅ Documentation updated
✅ Security review completed
```

### **Issue Validation**
```rust
impl IssueValidator {
    fn validate_github_issue(&self, issue_ref: &str) -> Result<IssueInfo> {
        // Handle different issue reference formats
        let issue_number = match issue_ref {
            s if s.starts_with('#') => s[1..].parse()?,
            s if s.parse::<u32>().is_ok() => s.parse()?,
            _ => return Err("Invalid issue reference format".into()),
        };
        
        // Fetch issue from GitHub
        let issue = self.github_client.get_issue(issue_number)?;
        
        if issue.state != "open" {
            return Err("Issue is not open".into());
        }
        
        Ok(IssueInfo {
            number: issue_number,
            title: issue.title,
            labels: issue.labels,
            assignee: issue.assignee,
        })
    }
}
```

### **Duplicate Prevention**
```bash
# Check for existing PRs
gh pr list --search "Fixes #12" --state all
gh pr list --head feature/fix-file-watcher --state all
```

## 🎨 PR Command Generation

### **Generated Command**
```bash
gh pr create --title "fix: reduce file watcher console spam" --body "$(cat <<'EOF'
[Complete PR description as shown above]
EOF
)"
```

### **Command Customization**
```bash
# Draft PR for work-in-progress
gh pr create --draft --title "..." --body "..."

# Target specific branch
gh pr create --base develop --title "..." --body "..."

# Add reviewers
gh pr create --reviewer @user1,@user2 --title "..." --body "..."
```

## 📊 Quality Metrics Integration

### **PR Analytics**
- **Change complexity**: Lines of code, files affected
- **Test coverage**: Manual test scenarios provided
- **Documentation impact**: Docs created/updated
- **Breaking change assessment**: Migration requirements
- **Security impact**: Security-related changes

### **Review Readiness Score**
- **Quality gates** (40%): All automated checks pass
- **Testing instructions** (25%): Comprehensive manual tests
- **Documentation** (20%): Complete and accurate
- **Description quality** (10%): Clear and detailed
- **Issue linking** (5%): Proper GitHub integration

## 🔗 Integration with Workflow

### **Command Sequence**
```bash
# Complete workflow integration
/work-on-ticket "#12"     # Implementation
→ /qa-check               # Quality validation  
→ /create-pr "#12"        # Professional PR creation
```

### **Quality Command Integration**
- **Pre-requisite**: `/qa-check` must pass
- **Optional**: `/review-pr` for self-assessment
- **Recommended**: `/security-check` for security changes
- **Documentation**: `/docs-check` for doc validation

## ⚠️ Important Requirements

### **Before PR Creation**
- [ ] All `/qa-check` validations pass
- [ ] Issue number is correct GitHub issue (not sequence)
- [ ] No existing PR for the same issue
- [ ] All documentation updated
- [ ] Breaking changes documented

### **PR Description Standards**
- **Issue linking**: First line must be `Fixes #[ISSUE_NUMBER]`
- **Implementation details**: Technical approach explained
- **Testing instructions**: 6+ comprehensive test scenarios
- **Quality checklist**: All checks documented
- **Documentation links**: All updates referenced

### **Manual Testing Requirements**
- **Minimum 6 scenarios** covering different aspects
- **Clear step-by-step instructions** (3-4 steps each)
- **Expected results** for each scenario
- **Success criteria** for pass/fail determination
- **Coverage**: functionality, errors, integration, security, performance, docs

This command ensures all pull requests meet enterprise-grade standards for review, testing, and deployment readiness.