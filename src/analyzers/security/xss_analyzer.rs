//! # XSS Analyzer
//!
//! This analyzer detects potential Cross-Site Scripting (XSS) vulnerabilities in code files.

use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// XSS vulnerability analyzer
pub struct XssAnalyzer {
    patterns: Vec<Regex>,
}

impl Default for XssAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl XssAnalyzer {
    /// Create a new XSS analyzer with default patterns
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // More precise patterns to avoid matching legitimate code
                Regex::new(r#"<script[^>]*>.*?</script>"#).unwrap(),
                Regex::new(r#"javascript:\s*["'][^"']*["']"#).unwrap(), // Require quotes around JS URLs
                Regex::new(r#"\bon\w+\s*=\s*["'][^"']*["']"#).unwrap(), // Require quotes and = for event handlers
                Regex::new(r#"<iframe[^>]*src\s*=\s*["'][^"']*["'][^>]*>"#).unwrap(), // More specific iframe
                Regex::new(r#"<object[^>]*data\s*=\s*["'][^"']*["'][^>]*>"#).unwrap(), // More specific object
            ],
        }
    }

    /// Analyze content for XSS patterns
    fn analyze_content(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "security",
                            "xss",
                            Severity::High,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential XSS vulnerability detected".to_string(),
                        )
                        .with_description(format!(
                            "Line contains pattern that may indicate XSS: {}",
                            pattern.as_str()
                        ))
                        .with_suggestion(
                            "Sanitize user input and use Content Security Policy (CSP) to prevent XSS"
                                .to_string(),
                        ),
                    );
                }
            }
        }

        findings
    }
}

impl Analyzer for XssAnalyzer {
    fn name(&self) -> &str {
        "xss"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        Ok(self.analyze_content(&content_str, file_path))
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "rs" | "js" | "ts" | "py" | "java" | "php" | "html")
        } else {
            false
        }
    }
}
