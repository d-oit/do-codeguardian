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
                Regex::new(r#"<script[^>]*>.*?</script>"#).unwrap(),
                Regex::new(r#"javascript:""#).unwrap(),
                Regex::new(r#"on\w+\s*="#).unwrap(),
                Regex::new(r#"<iframe[^>]*>"#).unwrap(),
                Regex::new(r#"<object[^>]*>"#).unwrap(),
            ],
            command_injection_patterns: vec![
                Regex::new(r#";\s*(rm|del|format|shutdown)"#).unwrap(),
                Regex::new(r#"\|\s*(cat|ls|dir)"#).unwrap(),
                Regex::new(r#"`.*`"#).unwrap(),
                Regex::new(r#"\$\(.*\)"#).unwrap(),
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
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.command_injection_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "security",
                            "command_injection",
                            Severity::Critical,
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

    fn scan_hardcoded_secrets(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.secret_patterns {
                if pattern.is_match(line) {
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

        all_findings.extend(self.detect_sql_injection(&content_str, file_path));
        all_findings.extend(self.detect_xss(&content_str, file_path));
        all_findings.extend(self.detect_command_injection(&content_str, file_path));
        all_findings.extend(self.scan_hardcoded_secrets(&content_str, file_path));
        all_findings.extend(self.analyze_vulnerabilities(&content_str, file_path));

        Ok(all_findings)
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
