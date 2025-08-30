use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub struct SecurityAnalyzer {
    sql_injection_patterns: Vec<Regex>,
    xss_patterns: Vec<Regex>,
    command_injection_patterns: Vec<Regex>,
    secret_patterns: Vec<Regex>,
    vulnerability_patterns: Vec<Regex>,
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            sql_injection_patterns: vec![
                Regex::new(r#"'?\s*OR\s+\d+\s*=\s*\d+"#).unwrap(),
                Regex::new(r#"'?\s*AND\s+\d+\s*=\s*\d+"#).unwrap(),
                Regex::new(r#"UNION\s+SELECT"#).unwrap(),
                Regex::new(r#"--\s*$"#).unwrap(),
                Regex::new(r#";\s*DROP\s+TABLE"#).unwrap(),
            ],
            xss_patterns: vec![
                // More precise patterns to avoid matching legitimate code
                Regex::new(r#"<script[^>]*>.*?</script>"#).unwrap(),
                Regex::new(r#"javascript:\s*["'][^"']*["']"#).unwrap(), // Require quotes around JS URLs
                Regex::new(r#"\bon\w+\s*=\s*["'][^"']*["']"#).unwrap(), // Require quotes and = for event handlers
                Regex::new(r#"<iframe[^>]*src\s*=\s*["'][^"']*["'][^>]*>"#).unwrap(), // More specific iframe
                Regex::new(r#"<object[^>]*data\s*=\s*["'][^"']*["'][^>]*>"#).unwrap(), // More specific object
            ],
            command_injection_patterns: vec![
                Regex::new(r#";\s*(rm|del|format|shutdown)"#).unwrap(),
                Regex::new(r#"\|\s*(cat|ls|dir)"#).unwrap(),
                Regex::new(r#"`[^`]*`"#).unwrap(), // More precise backtick pattern
                Regex::new(r#"\$\([^)]*\)"#).unwrap(), // More precise dollar-parentheses
                Regex::new(r#"system\s*\("#).unwrap(),
            ],
            secret_patterns: vec![
                Regex::new(r#"API_KEY\s*=\s*["']sk-[^"']*["']"#).unwrap(),
                Regex::new(r#"PASSWORD\s*=\s*["'][^"']*["']"#).unwrap(),
                Regex::new(r#"SECRET\s*=\s*["'][^"']*["']"#).unwrap(),
                Regex::new(r#"TOKEN\s*=\s*["'][^"']*["']"#).unwrap(),
                Regex::new(r#"aws_access_key_id\s*=\s*["'][^"']*["']"#).unwrap(),
                Regex::new(r#"aws_secret_access_key\s*=\s*["'][^"']*["']"#).unwrap(),
            ],
            vulnerability_patterns: vec![
                Regex::new(r#"unsafe\s*\{"#).unwrap(),
                Regex::new(r#"std::mem::transmute"#).unwrap(),
                Regex::new(r#"std::ptr::null"#).unwrap(),
                Regex::new(r#"std::ffi::CStr::from_ptr"#).unwrap(),
                Regex::new(r#"eval\s*\("#).unwrap(),
            ],
        }
    }

    /// Check if the file is a Rust source file
    fn is_rust_file(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|e| e.to_str()) == Some("rs")
    }

    fn detect_sql_injection(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.sql_injection_patterns {
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
                        .with_description(
                            format!("Line contains pattern that may indicate SQL injection: {}", pattern.as_str())
                        )
                        .with_suggestion("Use parameterized queries or prepared statements to prevent SQL injection".to_string()),
                    );
                }
            }
        }
        findings
    }

    fn detect_xss(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.xss_patterns {
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
                        .with_description(
                            format!("Line contains pattern that may indicate XSS: {}", pattern.as_str())
                        )
                        .with_suggestion("Sanitize user input and use Content Security Policy (CSP) to prevent XSS".to_string()),
                    );
                }
            }
        }
        findings
    }

    fn detect_command_injection(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let is_rust = self.is_rust_file(file_path);
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.command_injection_patterns {
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

    fn scan_hardcoded_secrets(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.secret_patterns {
                if pattern.is_match(line) {
                    // Skip if this is likely a false positive (pattern definition, test, or documentation)
                    if self.is_likely_secret_false_positive(line, pattern.as_str()) {
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
                        .with_suggestion("Use environment variables or secure credential storage instead of hardcoding secrets".to_string()),
                    );
                }
            }
        }
        findings
    }

    /// Check if a secret pattern match is likely a false positive
    fn is_likely_secret_false_positive(&self, line: &str, _pattern: &str) -> bool {
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

    fn analyze_vulnerabilities(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.vulnerability_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "security",
                            "vulnerability",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "Potential security vulnerability detected".to_string(),
                        )
                        .with_description(format!(
                            "Line contains potentially unsafe code pattern: {}",
                            pattern.as_str()
                        ))
                        .with_suggestion(
                            "Review and ensure proper bounds checking and input validation"
                                .to_string(),
                        ),
                    );
                }
            }
        }
        findings
    }

    fn perform_security_checks(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        let mut all_findings = Vec::new();

        // Skip security analysis for analyzer files as they contain security patterns by design
        if self.should_skip_file(file_path) {
            return Ok(all_findings);
        }

        all_findings.extend(self.detect_sql_injection(&content_str, file_path));
        all_findings.extend(self.detect_xss(&content_str, file_path));
        all_findings.extend(self.detect_command_injection(&content_str, file_path));
        all_findings.extend(self.scan_hardcoded_secrets(&content_str, file_path));
        all_findings.extend(self.analyze_vulnerabilities(&content_str, file_path));

        Ok(all_findings)
    }

    /// Check if a file should be skipped from security analysis
    fn should_skip_file(&self, file_path: &Path) -> bool {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            // Skip analyzer files as they contain security patterns by design
            if file_name.contains("analyzer") || file_name.contains("security") {
                return true;
            }
        }

        // Skip test files
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with("_test.rs")
                || file_name == "tests.rs"
                || file_name.contains("test")
            {
                return true;
            }
        }

        // Skip files in tests directory
        if file_path.to_string_lossy().contains("/tests/") {
            return true;
        }

        false
    }
}

impl Analyzer for SecurityAnalyzer {
    fn name(&self) -> &str {
        "security"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.perform_security_checks(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js" | "ts" | "py" | "java" | "php" | "sql" | "html" | "xml"
            )
        } else {
            false
        }
    }
}
