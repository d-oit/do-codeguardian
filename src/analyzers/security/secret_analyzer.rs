//! # Secret Detection Analyzer
//!
//! This analyzer detects potential hardcoded secrets and credentials in code files.

use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Secret detection analyzer
pub struct SecretAnalyzer {
    patterns: Vec<Regex>,
}

impl Default for SecretAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretAnalyzer {
    /// Create a new secret analyzer with default patterns
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // Case-insensitive patterns for real-world variable names
                Regex::new(r#"(?i)(api_key|apikey)\s*=\s*["'][^"']{8,}["']"#).unwrap(),
                Regex::new(r#"(?i)(password|passwd|pwd)\s*=\s*["'][^"']{6,}["']"#).unwrap(),
                Regex::new(r#"(?i)(secret|secret_key)\s*=\s*["'][^"']{8,}["']"#).unwrap(),
                Regex::new(r#"(?i)(token|access_token|auth_token)\s*=\s*["'][^"']{10,}["']"#)
                    .unwrap(),
                // Common API key patterns
                Regex::new(r#"["']sk-[a-zA-Z0-9]{20,}["']"#).unwrap(),
                Regex::new(r#"["']pk_[a-zA-Z0-9]{20,}["']"#).unwrap(),
                Regex::new(r#"["']AIza[a-zA-Z0-9]{35}["']"#).unwrap(),
                // AWS patterns
                Regex::new(r#"["']AKIA[A-Z0-9]{16}["']"#).unwrap(),
                // GitHub tokens
                Regex::new(r#"["']ghp_[a-zA-Z0-9]{36}["']"#).unwrap(),
                Regex::new(r#"aws_access_key_id\s*=\s*["'][^"']*["']"#).unwrap(),
                Regex::new(r#"aws_secret_access_key\s*=\s*["'][^"']*["']"#).unwrap(),
            ],
        }
    }

    /// Check if a secret pattern match is likely a false positive
    fn is_likely_false_positive(&self, line: &str, _pattern: &str) -> bool {
        let trimmed = line.trim();

        // Skip only obvious documentation/comments (be more selective)
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            return true;
        }

        // Skip only if it's clearly a pattern definition or documentation
        if trimmed.contains("example") && trimmed.starts_with("//") {
            return true;
        }

        // Skip only obvious test patterns (be more specific)
        if line.contains("#[test]") || line.contains("fn test_") {
            return true;
        }

        // Skip pattern definitions (common in security analyzers)
        if line.contains("Regex::new") || line.contains("Pattern::new") || line.contains("pattern")
        {
            return true;
        }

        // Skip documentation or example code
        if line.contains("///")
            || line.contains("//!")
            || line.contains("example")
            || line.contains("Example")
        {
            return true;
        }

        // Skip if the line contains quotes around the pattern (indicating it's a string literal for pattern matching)
        if line.contains("\"API_KEY\"")
            || line.contains("\"PASSWORD\"")
            || line.contains("\"SECRET\"")
            || line.contains("\"TOKEN\"")
            || line.contains("\"aws_access_key")
        {
            return true;
        }

        false
    }

    /// Analyze content for hardcoded secrets
    fn analyze_content(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                if pattern.is_match(line) {
                    // Skip if this is likely a false positive (pattern definition, test, or documentation)
                    if self.is_likely_false_positive(line, pattern.as_str()) {
                        continue;
                    }

                    findings.push(
                        Finding::new(
                            "security",
                            "hardcoded_secret",
                            Severity::High,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Hardcoded secret detected".to_string(),
                        )
                        .with_description(
                            "Line contains what appears to be a hardcoded secret or credential".to_string()
                        )
                        .with_suggestion(
                            "Use environment variables or secure credential storage instead of hardcoding secrets"
                                .to_string(),
                        ),
                    );
                }
            }
        }

        findings
    }
}

impl Analyzer for SecretAnalyzer {
    fn name(&self) -> &str {
        "secret"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        Ok(self.analyze_content(&content_str, file_path))
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js" | "ts" | "py" | "java" | "php" | "yaml" | "yml" | "json" | "toml"
            )
        } else {
            false
        }
    }
}
