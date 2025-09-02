use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Duplicate code analyzer finds security-relevant code duplications
pub struct DuplicateAnalyzer {
    min_duplicate_lines: usize,
    security_function_patterns: Vec<Regex>,
    focus_security: bool,
    ignore_test_files: bool,
    max_files_to_compare: usize,
    // Cache for file contents to avoid re-reading
    file_cache: HashMap<String, Vec<String>>,
}

impl Default for DuplicateAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateAnalyzer {
    pub fn new() -> Self {
        Self {
            min_duplicate_lines: 10,
            security_function_patterns: vec![
                // Authentication patterns
                Regex::new(r"(?i)(authenticate|login|signin|verify|validate)").unwrap(),
                // Authorization patterns
                Regex::new(r"(?i)(authorize|permission|access|role|privilege)").unwrap(),
                // Cryptographic patterns
                Regex::new(r"(?i)(encrypt|decrypt|hash|crypto|cipher|key|token)").unwrap(),
                // Input validation patterns
                Regex::new(r"(?i)(validate|sanitize|escape|filter|clean)").unwrap(),
                // Error handling patterns
                Regex::new(r"(?i)(error|exception|panic|fail|abort)").unwrap(),
                // Security-sensitive operations
                Regex::new(r"(?i)(password|secret|credential|session|cookie)").unwrap(),
            ],
            focus_security: true,
            ignore_test_files: true,
            max_files_to_compare: 1000,
            file_cache: HashMap::new(),
        }
    }

    pub fn with_min_lines(mut self, min_lines: usize) -> Self {
        self.min_duplicate_lines = min_lines;
        self
    }

    pub fn with_security_focus(mut self, focus: bool) -> Self {
        self.focus_security = focus;
        self
    }

    pub fn with_test_files(mut self, include_tests: bool) -> Self {
        self.ignore_test_files = !include_tests;
        self
    }

    pub fn with_max_files(mut self, max_files: usize) -> Self {
        self.max_files_to_compare = max_files;
        self
    }

    /// Normalize a line for comparison (remove whitespace, comments)
    fn normalize_line(&self, line: &str) -> String {
        let mut normalized = line.trim().to_string();

        // Remove single-line comments
        if let Some(pos) = normalized.find("//") {
            normalized = normalized[..pos].trim().to_string();
        }
        if let Some(pos) = normalized.find('#') {
            // Be careful with # in strings
            if !self.is_in_string(&normalized, pos) {
                normalized = normalized[..pos].trim().to_string();
            }
        }

        // Remove extra whitespace
        normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");

        normalized
    }

    /// Check if a position is inside a string literal
    fn is_in_string(&self, line: &str, pos: usize) -> bool {
        let before = &line[..pos];
        let single_quotes = before.matches('\'').count();
        let double_quotes = before.matches('"').count();

        // Simple heuristic: if we have an odd number of quotes before this position,
        // we're likely inside a string
        (single_quotes % 2 == 1) || (double_quotes % 2 == 1)
    }

    /// Extract meaningful code blocks from content
    fn extract_code_blocks(&self, content: &str) -> Vec<CodeBlock> {
        let lines: Vec<String> = content
            .lines()
            .map(|line| self.normalize_line(line))
            .collect();

        let mut blocks = Vec::new();
        let mut current_block = Vec::new();
        let mut start_line = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                if current_block.len() >= self.min_duplicate_lines {
                    blocks.push(CodeBlock {
                        lines: current_block.clone(),
                        start_line: start_line + 1, // 1-indexed
                        end_line: line_num,
                    });
                }
                current_block.clear();
                start_line = line_num + 1;
            } else {
                current_block.push(line.clone());
            }
        }

        // Don't forget the last block
        if current_block.len() >= self.min_duplicate_lines {
            blocks.push(CodeBlock {
                lines: current_block,
                start_line: start_line + 1,
                end_line: lines.len(),
            });
        }

        blocks
    }

    /// Check if a code block contains security-relevant code
    fn is_security_relevant(&self, block: &CodeBlock) -> bool {
        if !self.focus_security {
            return true; // If not focusing on security, all blocks are relevant
        }

        let block_text = block.lines.join(" ").to_lowercase();

        for pattern in &self.security_function_patterns {
            if pattern.is_match(&block_text) {
                return true;
            }
        }

        false
    }

    /// Calculate similarity between two code blocks
    fn calculate_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        if block1.lines.is_empty() || block2.lines.is_empty() {
            return 0.0;
        }

        let mut matching_lines = 0;
        let max_lines = block1.lines.len().max(block2.lines.len());
        let min_lines = block1.lines.len().min(block2.lines.len());

        // Compare line by line
        for i in 0..min_lines {
            if block1.lines[i] == block2.lines[i] {
                matching_lines += 1;
            }
        }

        // Calculate similarity as percentage of matching lines
        matching_lines as f64 / max_lines as f64
    }

    /// Find duplicates within a single file
    fn find_internal_duplicates(&self, file_path: &Path, content: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        let blocks = self.extract_code_blocks(content);

        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                let similarity = self.calculate_similarity(&blocks[i], &blocks[j]);

                if similarity >= 0.8 && self.is_security_relevant(&blocks[i]) {
                    let severity = if similarity >= 0.95 {
                        Severity::High
                    } else if similarity >= 0.9 {
                        Severity::Medium
                    } else {
                        Severity::Low
                    };

                    findings.push(
                        Finding::new(
                            "duplicate",
                            "internal_duplication",
                            severity,
                            file_path.to_path_buf(),
                            blocks[i].start_line as u32,
                            format!("Duplicate code block detected ({}% similar)", (similarity * 100.0) as u32),
                        )
                        .with_description(format!(
                            "Code block at lines {}-{} is {:.1}% similar to block at lines {}-{}",
                            blocks[i].start_line, blocks[i].end_line,
                            similarity * 100.0,
                            blocks[j].start_line, blocks[j].end_line
                        ))
                        .with_suggestion("Consider extracting common code into a shared function to reduce duplication and maintenance burden".to_string()),
                    );
                }
            }
        }

        findings
    }

    /// Check if file should be ignored
    fn should_ignore_file(&self, file_path: &Path) -> bool {
        if self.ignore_test_files && self.is_test_file(file_path) {
            return true;
        }

        // Ignore generated files
        let path_str = file_path.to_string_lossy().to_lowercase();
        if path_str.contains("generated")
            || path_str.contains("target/")
            || path_str.contains("build/")
            || path_str.contains("dist/")
            || path_str.contains("node_modules/")
        {
            return true;
        }

        false
    }

    /// Check if file is a test file
    fn is_test_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_lowercase();

        path_str.contains("/test")
            || path_str.contains("\\test")
            || path_str.contains("/tests")
            || path_str.contains("\\tests")
            || path_str.ends_with("_test.rs")
            || path_str.ends_with(".test.js")
            || path_str.ends_with("_test.py")
            || path_str.ends_with("test.go")
    }

    /// Get security risk level for duplicate code
    fn get_security_risk_level(&self, block: &CodeBlock) -> Severity {
        let block_text = block.lines.join(" ").to_lowercase();

        // High risk patterns
        if block_text.contains("password")
            || block_text.contains("secret")
            || block_text.contains("encrypt")
            || block_text.contains("decrypt")
            || block_text.contains("authenticate")
        {
            return Severity::High;
        }

        // Medium risk patterns
        if block_text.contains("validate")
            || block_text.contains("authorize")
            || block_text.contains("permission")
            || block_text.contains("session")
        {
            return Severity::Medium;
        }

        Severity::Low
    }
}

#[derive(Debug, Clone)]
struct CodeBlock {
    lines: Vec<String>,
    start_line: usize,
    end_line: usize,
}

impl Analyzer for DuplicateAnalyzer {
    fn name(&self) -> &str {
        "duplicate"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        if self.should_ignore_file(file_path) {
            return Ok(Vec::new());
        }

        let content_str = String::from_utf8_lossy(content);

        // For now, only analyze internal duplicates within the same file
        // Cross-file analysis would require coordination between analyzer calls
        let findings = self.find_internal_duplicates(file_path, &content_str);

        Ok(findings)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext.to_lowercase().as_str(),
                "rs" | "js"
                    | "ts"
                    | "py"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
                    | "go"
                    | "php"
                    | "rb"
                    | "cs"
                    | "swift"
                    | "kt"
                    | "scala"
                    | "dart"
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
    fn test_detect_internal_duplicates() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(3);
        let content = r#"
fn authenticate_user(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}

fn authenticate_admin(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}
"#;
        let findings = analyzer
            .analyze(Path::new("auth.rs"), content.as_bytes())
            .unwrap();
        assert!(findings.iter().any(|f| f.rule_id == "internal_duplication"));
    }

    #[test]
    fn test_normalize_line() {
        let analyzer = DuplicateAnalyzer::new();
        assert_eq!(
            analyzer.normalize_line("  let x = 5;  // comment"),
            "let x = 5;"
        );
        assert_eq!(
            analyzer.normalize_line("    if condition {"),
            "if condition {"
        );
    }

    #[test]
    fn test_security_relevance() {
        let analyzer = DuplicateAnalyzer::new();
        let security_block = CodeBlock {
            lines: vec!["authenticate_user".to_string(), "hash_password".to_string()],
            start_line: 1,
            end_line: 2,
        };
        let normal_block = CodeBlock {
            lines: vec!["println!".to_string(), "format!".to_string()],
            start_line: 1,
            end_line: 2,
        };

        assert!(analyzer.is_security_relevant(&security_block));
        assert!(!analyzer.is_security_relevant(&normal_block));
    }

    #[test]
    fn test_ignore_test_files() {
        let analyzer = DuplicateAnalyzer::new();
        assert!(analyzer.should_ignore_file(Path::new("tests/test_auth.rs")));
        assert!(analyzer.should_ignore_file(Path::new("src/auth_test.rs")));
        assert!(!analyzer.should_ignore_file(Path::new("src/auth.rs")));
    }

    #[test]
    fn test_calculate_similarity() {
        let analyzer = DuplicateAnalyzer::new();
        let block1 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            start_line: 1,
            end_line: 3,
        };
        let block2 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            start_line: 5,
            end_line: 7,
        };
        let block3 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "different".to_string(),
                "line3".to_string(),
            ],
            start_line: 9,
            end_line: 11,
        };

        assert_eq!(analyzer.calculate_similarity(&block1, &block2), 1.0);
        assert!((analyzer.calculate_similarity(&block1, &block3) - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_supports_file() {
        let analyzer = DuplicateAnalyzer::new();
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("test.js")));
        assert!(analyzer.supports_file(Path::new("test.py")));
        assert!(!analyzer.supports_file(Path::new("test.txt")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
    }
}
