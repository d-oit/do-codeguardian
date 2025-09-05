//! # SQL Injection Analyzer
//!
//! This analyzer detects potential SQL injection vulnerabilities in code files.

use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// SQL injection vulnerability analyzer
pub struct SqlInjectionAnalyzer {
    patterns: Vec<Regex>,
}

impl Default for SqlInjectionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SqlInjectionAnalyzer {
    /// Create a new SQL injection analyzer with default patterns
    pub fn new() -> Self {
        Self {
            patterns: vec![
                Regex::new(r#"'?\s*OR\s+\d+\s*=\s*\d+"#).unwrap(),
                Regex::new(r#"'?\s*AND\s+\d+\s*=\s*\d+"#).unwrap(),
                Regex::new(r#"UNION\s+SELECT"#).unwrap(),
                Regex::new(r#"--\s*$"#).unwrap(),
                Regex::new(r#";\s*DROP\s+TABLE"#).unwrap(),
            ],
        }
    }

    /// Analyze content for SQL injection patterns
    fn analyze_content(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "security",
                            "sql_injection",
                            Severity::High,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential SQL injection vulnerability detected".to_string(),
                        )
                        .with_description(format!(
                            "Line contains pattern that may indicate SQL injection: {}",
                            pattern.as_str()
                        ))
                        .with_suggestion(
                            "Use parameterized queries or prepared statements to prevent SQL injection"
                                .to_string(),
                        ),
                    );
                }
            }
        }

        findings
    }
}

impl Analyzer for SqlInjectionAnalyzer {
    fn name(&self) -> &str {
        "sql_injection"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        Ok(self.analyze_content(&content_str, file_path))
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "rs" | "js" | "ts" | "py" | "java" | "php" | "sql")
        } else {
            false
        }
    }
}
