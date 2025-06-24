# Docs Check

Documentation standards verification ensuring completeness, accuracy, cross-references, and integration with the centralized `.claude/docs/` structure.

## 🎯 Purpose

This command validates **documentation quality and standards** across the entire project, ensuring documentation is complete, accurate, properly cross-referenced, and follows established patterns for maintainability and discoverability.

## 📚 Documentation Validation Areas

### **📁 Structure and Organization**

#### **Centralized Documentation**
```
✅ .claude/docs/ structure properly organized
✅ Documentation index files present and current
✅ Proper directory categorization (architecture, development, github, learnings)
✅ No scattered documentation outside .claude/docs/
✅ Consistent naming conventions across all docs
```

#### **File Organization Standards**
```
✅ README.md files in each documentation directory
✅ Proper markdown file extensions (.md)
✅ Descriptive filenames matching content
✅ Logical grouping by domain/topic
✅ No duplicate or redundant documentation
```

### **📝 Content Quality Standards**

#### **Markdown Formatting**
```markdown
✅ Proper heading hierarchy (H1 → H2 → H3)
✅ Consistent formatting patterns
✅ Code blocks with syntax highlighting
✅ Tables properly formatted
✅ Lists and bullets consistent
✅ No broken markdown syntax
```

#### **Content Completeness**
```
✅ All required sections present
✅ Introduction and purpose clearly stated
✅ Examples and usage patterns provided
✅ Technical details sufficiently explained
✅ No TODO or placeholder content
✅ Proper conclusion or summary
```

### **🔗 Cross-Reference Validation**

#### **Internal Links**
```
✅ All internal links functional and accurate
✅ Relative paths correct (./ and ../)
✅ Cross-references between related documents
✅ Proper linking to command documentation
✅ Architecture docs properly interconnected
```

#### **External Resources**
```
✅ External links functional and current
✅ Official documentation links up-to-date
✅ API reference links accurate
✅ Community resource links valid
✅ No dead or outdated external links
```

### **🎯 Command Integration**

#### **Command Documentation**
```
✅ All commands documented in .claude/commands/
✅ Command usage examples accurate
✅ Parameter documentation complete
✅ Integration workflows documented
✅ Command cross-references working
```

#### **Documentation-Command Alignment**
```
✅ Documentation standards match command enforcement
✅ Quality gates align with /review-pr standards
✅ Workflow docs match actual command behavior
✅ Architecture docs reflect current implementation
```

### **📋 Code Examples and Accuracy**

#### **Code Example Validation**
```rust
✅ All code examples syntactically correct
✅ Rust examples follow project conventions
✅ JavaScript examples use proper Tauri v2 patterns
✅ Configuration examples match actual structure
✅ Shell commands tested and working
```

#### **Technical Accuracy**
```
✅ Architecture descriptions match implementation
✅ API signatures current and correct
✅ Configuration patterns accurate
✅ Security patterns properly documented
✅ No outdated technical information
```

## 💡 Usage

### **Basic Usage**
```
/docs-check $ARGUMENTS
```

### **Argument Formats**
```bash
/docs-check                         # Check all documentation
/docs-check "architecture"          # Check architecture docs only
/docs-check ".claude/docs/development" # Check specific directory
/docs-check "commands"              # Check command documentation
/docs-check --links-only            # Only validate links
/docs-check --fix                   # Attempt automatic fixes
```

### **Example Output (All Clear)**
```
📚 Documentation Standards Verification

📁 Structure & Organization: ✅ EXCELLENT
✅ .claude/docs/ structure properly organized
✅ All directory README.md files present
✅ Consistent naming conventions maintained
✅ No scattered documentation detected
✅ Logical categorization verified

📝 Content Quality: ✅ EXCELLENT
✅ Markdown formatting consistent across all files
✅ Proper heading hierarchy maintained
✅ Code blocks with syntax highlighting
✅ All required sections present
✅ No placeholder or TODO content

🔗 Cross-Reference Validation: ✅ EXCELLENT
✅ All 47 internal links functional
✅ External links current and accessible
✅ Command cross-references working
✅ Architecture docs properly interconnected
✅ Navigation paths clear and logical

🎯 Command Integration: ✅ EXCELLENT
✅ All 12 commands documented completely
✅ Usage examples accurate and tested
✅ Integration workflows documented
✅ Command standards align with docs

📋 Code Examples: ✅ EXCELLENT
✅ All Rust examples syntactically correct
✅ Tauri v2 patterns properly used
✅ Configuration examples match structure
✅ Shell commands tested and working

🎉 Documentation Status: EXCELLENT ✅

📚 Documentation meets all quality standards and is ready for use!
```

### **Example Output (Issues Found)**
```
📚 Documentation Standards Verification

📁 Structure & Organization: ⚠️ MINOR ISSUES
✅ .claude/docs/ structure organized
❌ Missing README.md in learnings directory
   📍 .claude/docs/learnings/README.md
   🔧 Fix: Create index file for learnings documentation

📝 Content Quality: ⚠️ ISSUES FOUND
❌ Broken markdown syntax detected
   📍 .claude/docs/architecture/overview.md:45
   💡 Missing closing code block (```)
   🔧 Fix: Add closing backticks

❌ Inconsistent heading hierarchy
   📍 .claude/docs/development/workflow-process.md
   💡 H3 follows H1 directly (missing H2)
   🔧 Fix: Add intermediate H2 heading

🔗 Cross-Reference Validation: ❌ BROKEN LINKS
❌ Internal link broken
   📍 .claude/docs/architecture/security-model.md:123
   💡 Link to ../commands/security-check.md returns 404
   🔧 Fix: Update path to ../../commands/security-check.md

❌ External link outdated
   📍 .claude/docs/development/setup-guide.md:67
   💡 Tauri v1 documentation link (outdated)
   🔧 Fix: Update to current Tauri v2 documentation

🎯 Command Integration: ⚠️ SYNC ISSUES
⚠️ Command documentation incomplete
   📍 .claude/commands/test-review.md
   💡 Missing usage examples section
   🔧 Fix: Add comprehensive usage examples

📋 Code Examples: ❌ ACCURACY ISSUES
❌ Rust example compilation error
   📍 .claude/docs/architecture/configuration-system.md:89
   💡 Missing import: use std::collections::HashMap;
   🔧 Fix: Add required import statement

❌ Outdated Tauri pattern
   📍 .claude/docs/development/tauri-patterns.md:34
   💡 Using Tauri v1 invoke pattern
   🔧 Fix: Update to Tauri v2 object parameter pattern

🚨 Documentation Status: ISSUES FOUND ❌

🔧 Priority Fixes Required:
1. Fix broken internal links (navigation impact)
2. Correct code examples (accuracy critical)
3. Complete missing documentation sections
4. Update outdated external references

Run with --fix flag to attempt automatic corrections.
```

## 🔍 Detailed Validation Process

### **Structure Analysis**
```rust
impl StructureValidator {
    fn validate_documentation_structure(&self) -> Vec<StructureIssue> {
        let mut issues = Vec::new();
        
        // Check required directories exist
        let required_dirs = [
            ".claude/docs/architecture",
            ".claude/docs/development", 
            ".claude/docs/github",
            ".claude/docs/learnings",
            ".claude/commands",
        ];
        
        for dir in &required_dirs {
            if !Path::new(dir).exists() {
                issues.push(StructureIssue::MissingDirectory(dir.to_string()));
            }
        }
        
        // Check for README.md files
        for dir in &required_dirs {
            let readme_path = format!("{}/README.md", dir);
            if !Path::new(&readme_path).exists() {
                issues.push(StructureIssue::MissingIndex(readme_path));
            }
        }
        
        issues
    }
}
```

### **Link Validation**
```rust
impl LinkValidator {
    fn validate_all_links(&self) -> Vec<LinkIssue> {
        let mut issues = Vec::new();
        
        for doc_file in &self.documentation_files {
            let content = fs::read_to_string(&doc_file.path)?;
            
            // Extract markdown links [text](url)
            let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
            
            for link_match in link_regex.captures_iter(&content) {
                let link_text = &link_match[1];
                let link_url = &link_match[2];
                
                if link_url.starts_with("http") {
                    // External link validation
                    if let Err(e) = self.validate_external_link(link_url) {
                        issues.push(LinkIssue::ExternalLinkBroken {
                            file: doc_file.path.clone(),
                            url: link_url.to_string(),
                            error: e.to_string(),
                        });
                    }
                } else {
                    // Internal link validation
                    let resolved_path = self.resolve_relative_path(&doc_file.path, link_url);
                    if !resolved_path.exists() {
                        issues.push(LinkIssue::InternalLinkBroken {
                            file: doc_file.path.clone(),
                            link: link_url.to_string(),
                            resolved_path,
                        });
                    }
                }
            }
        }
        
        issues
    }
}
```

### **Code Example Validation**
```rust
impl CodeExampleValidator {
    fn validate_rust_examples(&self) -> Vec<CodeIssue> {
        let mut issues = Vec::new();
        
        for doc_file in &self.documentation_files {
            let content = fs::read_to_string(&doc_file.path)?;
            
            // Extract Rust code blocks
            let rust_code_regex = Regex::new(r"```rust\n(.*?)\n```").unwrap();
            
            for code_match in rust_code_regex.captures_iter(&content) {
                let code = &code_match[1];
                
                // Basic syntax validation
                if let Err(e) = self.validate_rust_syntax(code) {
                    issues.push(CodeIssue::RustSyntaxError {
                        file: doc_file.path.clone(),
                        code: code.to_string(),
                        error: e.to_string(),
                    });
                }
                
                // Check for common anti-patterns
                if code.contains(".unwrap()") && !code.contains("// Safe unwrap") {
                    issues.push(CodeIssue::AntiPattern {
                        file: doc_file.path.clone(),
                        pattern: "unwrap without justification".to_string(),
                        suggestion: "Use proper error handling with ?".to_string(),
                    });
                }
            }
        }
        
        issues
    }
}
```

## 🔧 Automatic Fix Capabilities

### **Link Repair**
```bash
/docs-check --fix
```

**Automatic fixes include**:
- Update relative path references
- Fix common markdown formatting issues
- Add missing code block closures
- Update outdated external links
- Generate missing README.md files

### **Content Generation**
```markdown
# Auto-generated fixes example

## Missing Code Block Closure
```rust
// Before: Unclosed code block
let config = AppConfig::load()?;

// After: Properly closed
```rust
let config = AppConfig::load()?;
```

## Updated External Link
```markdown
<!-- Before: Outdated link -->
[Tauri Documentation](https://tauri.studio/docs)

<!-- After: Current link -->
[Tauri Documentation](https://tauri.app/v1/guides/)
```
```

## 📊 Documentation Quality Metrics

### **Quality Scoring**
- **Structure & Organization** (25%): Proper file organization and naming
- **Content Quality** (25%): Complete, accurate, well-formatted content
- **Cross-References** (20%): Working links and navigation
- **Command Integration** (15%): Alignment with command documentation
- **Code Examples** (10%): Accurate, tested code examples
- **External Resources** (5%): Current external references

### **Quality Levels**
- **EXCELLENT** (9.0+): Comprehensive, accurate, well-organized
- **GOOD** (7.0-8.9): Minor issues, mostly complete
- **NEEDS WORK** (5.0-6.9): Several issues affecting usability
- **POOR** (<5.0): Major problems, significant gaps

## 🔗 Integration with Development Workflow

### **Command Integration**
- **[`/start-feature`](./start-feature.md)** - Documentation planning and creation
- **[`/review-pr`](./review-pr.md)** - Documentation review in code review
- **[`/create-pr`](./create-pr.md)** - Documentation links in PR descriptions

### **Quality Workflow**
```bash
# Documentation validation workflow
/start-feature "#15"          # Includes documentation planning
→ Implementation with docs    # Create docs as you implement
→ /docs-check                # Validate documentation quality
→ /review-pr                 # Include docs review
→ /create-pr                 # Reference docs in PR
```

## 📋 Documentation Standards Checklist

### **Before Creating Documentation**
- [ ] Determine appropriate location in `.claude/docs/`
- [ ] Plan cross-references to related documentation
- [ ] Prepare code examples and test them
- [ ] Identify external resources to reference

### **During Documentation Creation**
- [ ] Follow markdown formatting standards
- [ ] Include clear introduction and purpose
- [ ] Provide comprehensive examples
- [ ] Add cross-references as you write
- [ ] Test all code examples

### **After Documentation Creation**
- [ ] Run `/docs-check` to validate quality
- [ ] Verify all links work correctly
- [ ] Update related documentation with cross-references
- [ ] Include documentation in PR descriptions

## ⚠️ Important Standards

### **Documentation Requirements**
- **Centralized location** - All docs in `.claude/docs/`
- **Consistent formatting** - Follow established markdown patterns
- **Working cross-references** - All links must be functional
- **Accurate examples** - All code must be tested and correct
- **Current information** - No outdated technical details

### **Maintenance Guidelines**
- **Update with changes** - Documentation must stay current with implementation
- **Regular validation** - Run `/docs-check` periodically
- **Link maintenance** - Check external links for currency
- **Cross-reference updates** - Maintain navigation between related docs

This command ensures documentation remains high-quality, discoverable, and valuable for both current development and future team members.