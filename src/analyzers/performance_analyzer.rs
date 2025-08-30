use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Performance analyzer for detecting performance bottlenecks in Rust code.
/// Focuses on common anti-patterns that can cause performance issues.
pub struct PerformanceAnalyzer {
    nested_loop_patterns: Vec<Regex>,
    inefficient_string_patterns: Vec<Regex>,
    blocking_io_patterns: Vec<Regex>,
    algorithmic_inefficiency_patterns: Vec<Regex>,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceAnalyzer {
    /// Creates a new PerformanceAnalyzer with predefined patterns.
    pub fn new() -> Self {
        Self {
            nested_loop_patterns: vec![
                // Detect nested for loops (simple heuristic)
                Regex::new(r"for\s+.*\{\s*for\s+.*\{").unwrap(),
                // Detect nested while loops
                Regex::new(r"while\s+.*\{\s*while\s+.*\{").unwrap(),
                // Detect for inside while or vice versa
                Regex::new(r"(for\s+.*\{|while\s+.*\{).*?(for\s+.*\{|while\s+.*\{)").unwrap(),
            ],
            inefficient_string_patterns: vec![
                // String concatenation in loops using +
                Regex::new(r"(for\s+.*\{|while\s+.*\{).*?(\w+)\s*\+=\s*.*").unwrap(),
                // Repeated string concatenation
                Regex::new(r"(\w+)\s*=\s*(\w+)\s*\+\s*.*").unwrap(),
            ],
            blocking_io_patterns: vec![
                // Synchronous file operations in async functions
                Regex::new(r"async\s+fn.*\{.*std::fs::(read_to_string|write|read|create_dir)")
                    .unwrap(),
                // Blocking I/O in tokio contexts
                Regex::new(r"#\[tokio::main\].*std::fs::(read_to_string|write|read)").unwrap(),
                // Synchronous network operations
                Regex::new(r"std::net::TcpListener::bind").unwrap(),
            ],
            algorithmic_inefficiency_patterns: vec![
                // Potential O(n^2) patterns with collect and iter
                Regex::new(r"\.collect\(\)\s*\.\s*iter\(\)\s*\.\s*map").unwrap(),
                // Inefficient sorting in loops
                Regex::new(r"(for\s+.*\{|while\s+.*\{).*?\.sort\(\)").unwrap(),
            ],
        }
    }

    /// Detects nested loops that may indicate algorithmic inefficiencies.
    fn detect_nested_loops(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.nested_loop_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "performance",
                            "nested_loops",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential nested loop detected".to_string(),
                        )
                        .with_description(
                            "Nested loops can lead to O(n^2) or higher time complexity. Consider optimizing the algorithm.".to_string()
                        )
                        .with_suggestion("Review the algorithm for potential optimizations, such as using hash maps or breaking early.".to_string()),
                    );
                }
            }
        }
        findings
    }

    /// Detects inefficient string operations, particularly concatenation in loops.
    fn detect_inefficient_strings(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.inefficient_string_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "performance",
                            "inefficient_string_ops",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Inefficient string operation detected".to_string(),
                        )
                        .with_description(
                            "String concatenation in loops can be inefficient due to reallocations.".to_string()
                        )
                        .with_suggestion("Use String::with_capacity or collect into a Vec and join for better performance.".to_string()),
                    );
                }
            }
        }
        findings
    }

    /// Detects blocking I/O operations that should be asynchronous.
    fn detect_blocking_io(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.blocking_io_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "performance",
                            "blocking_io",
                            Severity::High,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Blocking I/O operation detected".to_string(),
                        )
                        .with_description(
                            "Blocking I/O can cause performance issues in async contexts."
                                .to_string(),
                        )
                        .with_suggestion(
                            "Use async equivalents like tokio::fs or spawn blocking operations."
                                .to_string(),
                        ),
                    );
                }
            }
        }
        findings
    }

    /// Detects potential algorithmic inefficiencies.
    fn detect_algorithmic_inefficiencies(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.algorithmic_inefficiency_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "performance",
                            "algorithmic_inefficiency",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential algorithmic inefficiency detected".to_string(),
                        )
                        .with_description(
                            "Code pattern suggests potential inefficiency, such as unnecessary collections.".to_string()
                        )
                        .with_suggestion("Review the algorithm for optimizations, e.g., avoid collect().iter() chains.".to_string()),
                    );
                }
            }
        }
        findings
    }

    /// Performs all performance checks on the given content.
    fn perform_performance_checks(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        let mut all_findings = Vec::new();

        all_findings.extend(self.detect_nested_loops(&content_str, file_path));
        all_findings.extend(self.detect_inefficient_strings(&content_str, file_path));
        all_findings.extend(self.detect_blocking_io(&content_str, file_path));
        all_findings.extend(self.detect_algorithmic_inefficiencies(&content_str, file_path));

        Ok(all_findings)
    }
}

impl Analyzer for PerformanceAnalyzer {
    fn name(&self) -> &str {
        "performance"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.perform_performance_checks(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Only analyze Rust files for performance issues
        file_path.extension().and_then(|e| e.to_str()) == Some("rs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_detect_nested_loops() {
        let analyzer = PerformanceAnalyzer::new();
        let content = r#"
        fn example() {
            for i in 0..10 {
                for j in 0..10 {
                    println!("{} {}", i, j);
                }
            }
        }
        "#;
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, content).unwrap();

        let findings = analyzer.detect_nested_loops(content, &file_path);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule, "nested_loops");
    }

    #[test]
    fn test_detect_inefficient_strings() {
        let analyzer = PerformanceAnalyzer::new();
        let content = r#"
        fn example() {
            let mut s = String::new();
            for i in 0..10 {
                s += &format!(" {}", i);
            }
        }
        "#;
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, content).unwrap();

        let findings = analyzer.detect_inefficient_strings(content, &file_path);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule, "inefficient_string_ops");
    }

    #[test]
    fn test_supports_file() {
        let analyzer = PerformanceAnalyzer::new();
        assert!(analyzer.supports_file(&PathBuf::from("test.rs")));
        assert!(!analyzer.supports_file(&PathBuf::from("test.js")));
    }
}
