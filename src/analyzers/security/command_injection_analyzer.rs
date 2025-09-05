//! # Command Injection Analyzer
//!
//! This analyzer detects potential command injection vulnerabilities in code files.

use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Command injection vulnerability analyzer
pub struct CommandInjectionAnalyzer {
    patterns: Vec<Regex>,
}

impl Default for CommandInjectionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandInjectionAnalyzer {
    /// Create a new command injection analyzer with default patterns
    pub fn new() -> Self {
        Self {
            patterns: vec![
                Regex::new(r#";\s*(rm|del|format|shutdown)"#).unwrap(),
                Regex::new(r#"\|\s*(cat|ls|dir)"#).unwrap(),
                Regex::new(r#"`[^`]*`"#).unwrap(), // More precise backtick pattern
                Regex::new(r#"\$\([^)]*\)"#).unwrap(), // More precise dollar-parentheses
                Regex::new(r#"system\s*\("#).unwrap(),
            ],
        }
    }

    /// Check if the file is a Rust source file
    fn is_rust_file(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|e| e.to_str()) == Some("rs")
    }

    /// Check if a pattern match is likely a false positive
    fn is_likely_false_positive(&self, line: &str, pattern: &str, is_rust: bool) -> bool {
        if is_rust {
            // In Rust, backticks in strings are typically for documentation or raw strings
            if pattern.contains("`")
                && (line.contains("///") || line.contains("//!") || line.contains("r#\""))
            {
                return true;
            }
            // Dollar-parentheses in Rust are typically macro syntax or documentation
            if pattern.contains(r#"\$\("#)
                && (line.contains("macro_rules!") || line.contains("///") || line.contains("//!"))
            {
                return true;
            }
        }

        // Skip patterns in comments
        if line.trim().starts_with("//")
            || line.trim().starts_with("#")
            || line.trim().starts_with("/*")
        {
            return true;
        }

        // Skip patterns in test functions
        if line.contains("#[test]") || line.contains("fn test_") {
            return true;
        }

        false
    }

    /// Check if a command pattern is high risk
    fn is_high_risk_command_pattern(&self, pattern: &str) -> bool {
        // High risk patterns that are more likely to be real vulnerabilities
        pattern.contains("rm")
            || pattern.contains("format")
            || pattern.contains("shutdown")
            || pattern.contains("DROP TABLE")
            || pattern.contains("DELETE FROM")
    }

    /// Analyze content for command injection patterns
    fn analyze_content(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let is_rust = self.is_rust_file(file_path);

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                // Skip backtick pattern for Rust files as backticks are not used for command execution in Rust
                if is_rust && pattern.as_str().contains("`") {
                    continue;
                }
                // Skip dollar-parentheses pattern for Rust files as $(...) is macro syntax
                if is_rust && pattern.as_str().contains(r#"\$\("#) {
                    continue;
                }

                if pattern.is_match(line) {
                    // Additional context checks for false positives
                    if self.is_likely_false_positive(line, pattern.as_str(), is_rust) {
                        continue;
                    }

                    let severity = if self.is_high_risk_command_pattern(pattern.as_str()) {
                        Severity::Critical
                    } else {
                        Severity::High
                    };

                    findings.push(
                        Finding::new(
                            "security",
                            "command_injection",
                            severity,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential command injection vulnerability detected".to_string(),
                        )
                        .with_description(format!(
                            "Line contains pattern that may indicate command injection: {}",
                            pattern.as_str()
                        ))
                        .with_suggestion(
                            "Validate and sanitize user input before passing to system commands"
                                .to_string(),
                        ),
                    );
                }
            }
        }

        findings
    }
}

impl Analyzer for CommandInjectionAnalyzer {
    fn name(&self) -> &str {
        "command_injection"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        Ok(self.analyze_content(&content_str, file_path))
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js" | "ts" | "py" | "java" | "php" | "sh" | "bash"
            )
        } else {
            false
        }
    }
}
