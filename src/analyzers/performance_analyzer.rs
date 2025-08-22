use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Analyzer for detecting potential performance issues and anti-patterns
pub struct PerformanceAnalyzer {
    // Patterns for detecting performance issues
    nested_loop_pattern: Regex,
    string_concat_pattern: Regex,
    inefficient_collection_pattern: Regex,
    blocking_io_pattern: Regex,
    memory_leak_pattern: Regex,
    inefficient_regex_pattern: Regex,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            nested_loop_pattern: Regex::new(r#"for\s*\([^{]*\{[^}]*for\s*\(|while\s*\([^{]*\{[^}]*while\s*\(|\.forEach\([^}]*\.forEach\("#).unwrap(),
            string_concat_pattern: Regex::new(r#"\+\s*=\s*[\"']|String\s*\+|str\s*\+\s*str"#).unwrap(),
            inefficient_collection_pattern: Regex::new(r#"\.contains\(.*\).*for|\.indexOf\(.*\).*for|in\s+list.*for"#).unwrap(),
            blocking_io_pattern: Regex::new(r#"\.read\(\)|\.write\(\)|\.sleep\(|Thread\.sleep|time\.sleep|fs\.readFileSync|fs\.writeFileSync"#).unwrap(),
            memory_leak_pattern: Regex::new(r#"setInterval\(|addEventListener\(.*,.*\)|on\(.*,.*function|\.on\(.*,.*=>"#).unwrap(),
            inefficient_regex_pattern: Regex::new(r#"new\s+RegExp\(|Regex::new\(.*\).*for|Pattern\.compile\(.*\).*for"#).unwrap(),
        }
    }

    fn analyze_performance_issues(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        let lines: Vec<&str> = content_str.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_number = (line_num + 1) as u32;

            // Check for nested loops (O(n²) complexity warning)
            if self.nested_loop_pattern.is_match(line) || self.detect_nested_loops(&lines, line_num) {
                findings.push(
                    Finding::new(
                        "performance",
                        "nested_loops",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "Nested loops detected - potential O(n²) complexity".to_string(),
                    )
                    .with_description("Nested loops can lead to quadratic time complexity and poor performance with large datasets".to_string())
                    .with_suggestion("Consider using more efficient algorithms, hash maps, or breaking early when possible".to_string())
                );
            }

            // Check for inefficient string concatenation
            if self.string_concat_pattern.is_match(line) && self.is_in_loop_context(&lines, line_num) {
                findings.push(
                    Finding::new(
                        "performance",
                        "string_concat_in_loop",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "String concatenation in loop detected".to_string(),
                    )
                    .with_description("String concatenation in loops can be inefficient due to repeated memory allocations".to_string())
                    .with_suggestion("Use StringBuilder, Vec<String>, or similar efficient string building methods".to_string())
                );
            }

            // Check for inefficient collection operations
            if self.inefficient_collection_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "performance",
                        "inefficient_collection_ops",
                        Severity::Low,
                        file_path.to_path_buf(),
                        line_number,
                        "Potentially inefficient collection operation".to_string(),
                    )
                    .with_description("Linear search operations in loops can lead to O(n²) complexity".to_string())
                    .with_suggestion("Consider using HashSet, HashMap, or other O(1) lookup data structures".to_string())
                );
            }

            // Check for blocking I/O operations
            if self.blocking_io_pattern.is_match(line) && !self.is_in_async_context(&lines, line_num) {
                let severity = if line.contains("sleep") { Severity::High } else { Severity::Medium };
                findings.push(
                    Finding::new(
                        "performance",
                        "blocking_io",
                        severity,
                        file_path.to_path_buf(),
                        line_number,
                        "Blocking I/O operation detected".to_string(),
                    )
                    .with_description("Blocking I/O operations can freeze the application and reduce responsiveness".to_string())
                    .with_suggestion("Use async/await patterns or non-blocking I/O operations".to_string())
                );
            }

            // Check for potential memory leaks
            if self.memory_leak_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "performance",
                        "potential_memory_leak",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential memory leak pattern detected".to_string(),
                    )
                    .with_description("Event listeners and intervals without proper cleanup can cause memory leaks".to_string())
                    .with_suggestion("Ensure proper cleanup with removeEventListener, clearInterval, or similar cleanup methods".to_string())
                );
            }

            // Check for regex compilation in loops
            if self.inefficient_regex_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "performance",
                        "regex_in_loop",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "Regex compilation in loop detected".to_string(),
                    )
                    .with_description("Compiling regex patterns repeatedly is expensive".to_string())
                    .with_suggestion("Compile regex patterns once outside the loop and reuse them".to_string())
                );
            }

            // Language-specific performance checks
            findings.extend(self.check_language_specific_issues(file_path, line, line_number)?);
        }

        // Check for large file issues
        findings.extend(self.check_file_size_issues(file_path, &content_str)?);

        Ok(findings)
    }

    fn detect_nested_loops(&self, lines: &[&str], current_line: usize) -> bool {
        // Check if current line is a loop and if we're already inside another loop
        let current = lines[current_line].trim();
        if current.contains("for ") || current.contains("while ") || current.contains(".forEach") {
            // Look backwards to see if we're inside another loop
            let start = current_line.saturating_sub(20);
            let mut brace_count = 0;
            for i in (start..current_line).rev() {
                let line = lines[i].trim();
                // Count braces to track nesting
                brace_count += line.matches('}').count() as i32;
                brace_count -= line.matches('{').count() as i32;
                
                if brace_count <= 0 && (line.contains("for ") || line.contains("while ") || line.contains(".forEach")) {
                    return true; // Found an outer loop
                }
            }
        }
        false
    }

    fn is_in_loop_context(&self, lines: &[&str], current_line: usize) -> bool {
        // Look backwards up to 10 lines to see if we're in a loop
        let start = current_line.saturating_sub(10);
        for line in lines.iter().take(current_line).skip(start) {
            let line = line.trim();
            if line.contains("for ") || line.contains("while ") || line.contains(".forEach") || 
               line.contains("for(") || line.contains("while(") {
                return true;
            }
        }
        false
    }

    fn is_in_async_context(&self, lines: &[&str], current_line: usize) -> bool {
        // Look backwards to see if we're in an async function
        let start = current_line.saturating_sub(20);
        for line in lines.iter().take(current_line).skip(start) {
            let line = line.trim();
            if line.contains("async fn") || line.contains("async function") || 
               line.contains("await ") || line.contains(".await") {
                return true;
            }
        }
        false
    }

    fn check_language_specific_issues(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => {
                    findings.extend(self.check_rust_performance(file_path, line, line_number)?);
                }
                "js" | "ts" | "jsx" | "tsx" => {
                    findings.extend(self.check_javascript_performance(file_path, line, line_number)?);
                }
                "py" => {
                    findings.extend(self.check_python_performance(file_path, line, line_number)?);
                }
                "java" => {
                    findings.extend(self.check_java_performance(file_path, line, line_number)?);
                }
                _ => {}
            }
        }

        Ok(findings)
    }

    fn check_rust_performance(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unnecessary cloning
        if line.contains(".clone()") && !line.contains("// clone needed") {
            findings.push(
                Finding::new(
                    "performance",
                    "unnecessary_clone",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Potentially unnecessary clone() call".to_string(),
                )
                .with_description("Cloning can be expensive; consider using references or borrowing".to_string())
                .with_suggestion("Use references (&) or borrowing instead of cloning when possible".to_string())
            );
        }

        // Check for inefficient vector operations
        if line.contains("vec![]") && line.contains("push") {
            findings.push(
                Finding::new(
                    "performance",
                    "inefficient_vec_growth",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Vector without capacity hint".to_string(),
                )
                .with_description("Growing vectors without capacity hints can cause multiple reallocations".to_string())
                .with_suggestion("Use Vec::with_capacity() if you know the approximate size".to_string())
            );
        }

        // Check for String vs &str inefficiency
        if line.contains("String::new()") && line.contains("push_str") {
            findings.push(
                Finding::new(
                    "performance",
                    "string_building",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Inefficient string building pattern".to_string(),
                )
                .with_description("Building strings character by character can be inefficient".to_string())
                .with_suggestion("Consider using format! macro or collecting into String".to_string())
            );
        }

        Ok(findings)
    }

    fn check_javascript_performance(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for inefficient DOM queries
        if (line.contains("document.getElementById") || line.contains("document.querySelector")) &&
           (line.contains("for") || line.contains("while")) {
            findings.push(
                Finding::new(
                    "performance",
                    "dom_query_in_loop",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "DOM query in loop detected".to_string(),
                )
                .with_description("DOM queries are expensive and should be cached when used repeatedly".to_string())
                .with_suggestion("Cache DOM elements outside the loop".to_string())
            );
        }

        // Check for inefficient array operations
        if line.contains(".filter(") && line.contains(".map(") {
            findings.push(
                Finding::new(
                    "performance",
                    "chained_array_ops",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Chained array operations detected".to_string(),
                )
                .with_description("Chaining filter and map creates intermediate arrays".to_string())
                .with_suggestion("Consider using reduce() or a single loop for better performance".to_string())
            );
        }

        // Check for == vs === usage
        if line.contains(" == ") && !line.contains(" === ") {
            findings.push(
                Finding::new(
                    "performance",
                    "loose_equality",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Loose equality operator used".to_string(),
                )
                .with_description("Loose equality (==) performs type coercion which can be slower".to_string())
                .with_suggestion("Use strict equality (===) for better performance and clarity".to_string())
            );
        }

        Ok(findings)
    }

    fn check_python_performance(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for list comprehension vs loops
        if line.contains("for ") && line.contains("append(") {
            findings.push(
                Finding::new(
                    "performance",
                    "list_append_in_loop",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "List append in loop - consider list comprehension".to_string(),
                )
                .with_description("List comprehensions are generally faster than append() in loops".to_string())
                .with_suggestion("Use list comprehension: [expr for item in iterable]".to_string())
            );
        }

        // Check for inefficient string operations
        if line.contains("str(") && line.contains(" + ") {
            findings.push(
                Finding::new(
                    "performance",
                    "string_concatenation",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "String concatenation with + operator".to_string(),
                )
                .with_description("String concatenation with + can be inefficient for multiple strings".to_string())
                .with_suggestion("Use f-strings, .join(), or .format() for better performance".to_string())
            );
        }

        Ok(findings)
    }

    fn check_java_performance(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for String concatenation in loops
        if line.contains("String ") && line.contains(" + ") {
            findings.push(
                Finding::new(
                    "performance",
                    "string_concat_java",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "String concatenation with + operator".to_string(),
                )
                .with_description("String concatenation creates new objects and can be inefficient".to_string())
                .with_suggestion("Use StringBuilder for multiple concatenations".to_string())
            );
        }

        // Check for ArrayList without capacity
        if line.contains("new ArrayList()") {
            findings.push(
                Finding::new(
                    "performance",
                    "arraylist_no_capacity",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "ArrayList without initial capacity".to_string(),
                )
                .with_description("ArrayList without capacity hint may cause multiple resizing operations".to_string())
                .with_suggestion("Use ArrayList(int initialCapacity) if size is known".to_string())
            );
        }

        Ok(findings)
    }

    fn check_file_size_issues(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        let line_count = content.lines().count();
        let _char_count = content.chars().count();

        // Check for very large files
        if line_count > 1000 {
            findings.push(
                Finding::new(
                    "performance",
                    "large_file",
                    Severity::Low,
                    file_path.to_path_buf(),
                    1,
                    format!("Large file detected ({} lines)", line_count),
                )
                .with_description("Large files can be difficult to maintain and may indicate need for refactoring".to_string())
                .with_suggestion("Consider breaking this file into smaller, more focused modules".to_string())
            );
        }

        // Check for very long lines
        for (line_num, line) in content.lines().enumerate() {
            if line.len() > 120 {
                findings.push(
                    Finding::new(
                        "performance",
                        "long_line",
                        Severity::Info,
                        file_path.to_path_buf(),
                        (line_num + 1) as u32,
                        format!("Long line detected ({} characters)", line.len()),
                    )
                    .with_description("Very long lines can impact readability and code review efficiency".to_string())
                    .with_suggestion("Consider breaking long lines for better readability".to_string())
                );
                break; // Only report first occurrence to avoid spam
            }
        }

        Ok(findings)
    }
}

impl Analyzer for PerformanceAnalyzer {
    fn name(&self) -> &str {
        "performance"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.analyze_performance_issues(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, 
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | 
                "cpp" | "c" | "h" | "hpp" | "go" | "rb" | "php" |
                "cs" | "swift" | "kt" | "scala"
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_nested_loop_detection() {
        let analyzer = PerformanceAnalyzer::new();
        let code = "for (let i = 0; i < n; i++) {\n    for (let j = 0; j < m; j++) {\n        // nested loop\n    }\n}";
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "nested_loops"));
    }

    #[test]
    fn test_rust_clone_detection() {
        let analyzer = PerformanceAnalyzer::new();
        let code = "let x = some_value.clone();";
        
        let findings = analyzer.analyze(&PathBuf::from("test.rs"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "unnecessary_clone"));
    }

    #[test]
    fn test_blocking_io_detection() {
        let analyzer = PerformanceAnalyzer::new();
        let code = "let content = fs.readFileSync('file.txt');";
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "blocking_io"));
    }
}