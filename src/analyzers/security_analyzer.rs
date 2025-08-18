use crate::analyzers::Analyzer;
use crate::analyzers::optimized_patterns::{SECURITY_PATTERNS, AnalysisOptimizer, PatternCache};
use crate::analyzers::security_checks::SecurityChecks;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::collections::HashSet;

/// Advanced security analyzer for detecting vulnerabilities and security anti-patterns
pub struct SecurityAnalyzer {
    // Pattern cache for performance
    pattern_cache: PatternCache,
    // Known dangerous functions
    dangerous_functions: HashSet<String>,
    // Language-specific security checks
    security_checks: SecurityChecks,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        let mut dangerous_functions = HashSet::with_capacity(8);
        dangerous_functions.insert("eval".to_string());
        dangerous_functions.insert("exec".to_string());
        dangerous_functions.insert("system".to_string());
        dangerous_functions.insert("shell_exec".to_string());
        dangerous_functions.insert("passthru".to_string());
        dangerous_functions.insert("popen".to_string());
        dangerous_functions.insert("proc_open".to_string());

        Self {
            pattern_cache: PatternCache::new(1000), // Cache up to 1000 pattern matches
            dangerous_functions,
            security_checks: SecurityChecks::new(),
        }
    }

    fn analyze_security_issues(&mut self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        
        // Early exit for non-security files
        let file_type = AnalysisOptimizer::get_file_type(file_path);
        if let Some(ft) = file_type {
            if !ft.supports_security_analysis() {
                return Ok(findings);
            }
        }

        // Analyze each line for security issues
        for (line_num, line) in content_str.lines().enumerate() {
            let line_number = (line_num + 1) as u32;
            findings.extend(self.analyze_line_security(file_path, line, line_number));
        }

        // File-level security checks (optimized)
        findings.extend(self.check_file_level_security_fast(file_path, &content_str)?);

        // Language-specific security checks
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            for (line_num, line) in content_str.lines().enumerate() {
                let line_number = (line_num + 1) as u32;
                
                match ext {
                    "js" | "ts" | "jsx" | "tsx" => {
                        findings.extend(self.security_checks.javascript.check(file_path, line, line_number)?);
                    }
                    "py" => {
                        findings.extend(self.security_checks.python.check(file_path, line, line_number)?);
                    }
                    "php" => {
                        findings.extend(self.security_checks.php.check(file_path, line, line_number)?);
                    }
                    "java" => {
                        findings.extend(self.security_checks.java.check(file_path, line, line_number)?);
                    }
                    "rs" => {
                        findings.extend(self.security_checks.rust.check(file_path, line, line_number)?);
                    }
                    _ => {}
                }
            }
        }

        Ok(findings)
    }

    fn analyze_line_security(&mut self, file_path: &Path, line: &str, line_number: u32) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        // Check for hardcoded secrets first (most critical)
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.secrets_combined, line) {
            findings.push(Finding::new(
                "security",
                "hardcoded_secret",
                Severity::Critical,
                file_path.to_path_buf(),
                line_number,
                "Potential hardcoded secret detected".to_string(),
            )
            .with_description("Hardcoded secrets in source code pose a security risk".to_string())
            .with_suggestion("Move secrets to environment variables or secure configuration".to_string()));
        }
        
        // Check for SQL injection vulnerabilities
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.sql_injection_fast, line) {
            findings.push(Finding::new(
                "security",
                "sql_injection",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Potential SQL injection vulnerability".to_string(),
            )
            .with_description("SQL injection vulnerabilities can allow attackers to access or modify data".to_string())
            .with_suggestion("Use parameterized queries or prepared statements".to_string()));
        }
        
        // Check for XSS vulnerabilities
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.xss_fast, line) {
            findings.push(Finding::new(
                "security",
                "xss_vulnerability",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Potential XSS vulnerability".to_string(),
            )
            .with_description("XSS vulnerabilities can allow attackers to execute malicious scripts".to_string())
            .with_suggestion("Sanitize user input and use safe DOM manipulation methods".to_string()));
        }
        
        // Check for command injection
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.command_injection_fast, line) {
            findings.push(Finding::new(
                "security",
                "command_injection",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Potential command injection vulnerability".to_string(),
            )
            .with_description("Command injection can allow attackers to execute arbitrary system commands".to_string())
            .with_suggestion("Validate and sanitize all user input before using in system commands".to_string()));
        }
        
        // Check for weak cryptography
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.weak_crypto_fast, line) {
            findings.push(Finding::new(
                "security",
                "weak_crypto",
                Severity::Medium,
                file_path.to_path_buf(),
                line_number,
                "Weak cryptographic algorithm detected".to_string(),
            )
            .with_description("Weak cryptographic algorithms are vulnerable to attacks".to_string())
            .with_suggestion("Use strong, modern cryptographic algorithms like AES-256, SHA-256, or better".to_string()));
        }
        
        // Check for dangerous functions
        for func in &self.dangerous_functions {
            if line.contains(func) && !line.trim_start().starts_with("//") && !line.trim_start().starts_with("#") {
                findings.push(Finding::new(
                    "security",
                    "dangerous_function",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    format!("Dangerous function '{}' detected", func),
                )
                .with_description("Dangerous functions can lead to security vulnerabilities".to_string())
                .with_suggestion("Avoid using dangerous functions or ensure proper input validation".to_string()));
            }
        }
        
        findings
    }
    
    fn check_file_level_security_fast(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check for sensitive files
        if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
            let sensitive_files = [".env", "config.json", "secrets.json", "private.key"];
            if sensitive_files.iter().any(|&sf| filename.contains(sf)) {
                findings.push(
                    Finding::new(
                        "security",
                        "sensitive_file",
                        Severity::High,
                        file_path.to_path_buf(),
                        1,
                        "Potentially sensitive file detected".to_string(),
                    )
                );
            }
        }
        
        // Fast high-entropy string detection
        for (line_num, line) in content.lines().enumerate().take(100) { // Limit for performance
            if SECURITY_PATTERNS.high_entropy_check.is_match(line) {
                let entropy = AnalysisOptimizer::calculate_entropy_fast(line);
                if entropy > 4.0 {
                    findings.push(
                        Finding::new(
                            "security",
                            "high_entropy_string",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            (line_num + 1) as u32,
                            "High entropy string detected (possible secret)".to_string(),
                        )
                    );
                }
            }
        }
        
        Ok(findings)
    }

    #[allow(dead_code)]
    fn has_high_entropy(&self, s: &str) -> bool {
        // Simple entropy check - look for strings with good character distribution
        let mut char_counts = std::collections::HashMap::new();
        let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
        
        if chars.len() < 20 {
            return false;
        }

        for c in &chars {
            *char_counts.entry(*c).or_insert(0) += 1;
        }

        // Calculate entropy
        let len = chars.len() as f64;
        let entropy: f64 = char_counts.values()
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum();

        entropy > 3.5 // Threshold for "high" entropy
    }
}

impl Analyzer for SecurityAnalyzer {
    fn name(&self) -> &str {
        "security"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        // Note: We need to make this mutable for pattern cache, but Analyzer trait doesn't allow it
        // For now, we'll create a new analyzer instance to maintain the interface
        let mut analyzer = SecurityAnalyzer::new();
        analyzer.analyze_security_issues(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, 
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | 
                "cpp" | "c" | "h" | "hpp" | "go" | "rb" | "php" |
                "cs" | "swift" | "kt" | "scala" | "clj" | "hs" |
                "json" | "xml" | "yml" | "yaml" | "env"
            )
        } else {
            // Check for sensitive filenames without extensions
            if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
                filename.contains("secret") || filename.contains("key") || filename.contains("password")
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sql_injection_detection() {
        let analyzer = SecurityAnalyzer::new();
        let code = r#"query("SELECT * FROM users WHERE id = " + userId)"#;
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "sql_injection"));
    }

    #[test]
    fn test_hardcoded_secret_detection() {
        let analyzer = SecurityAnalyzer::new();
        let code = r#"const apiKey = "sk-1234567890abcdef1234567890abcdef";"#;
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "hardcoded_secret"));
    }

    #[test]
    fn test_xss_detection() {
        let analyzer = SecurityAnalyzer::new();
        let code = r#"element.innerHTML = userInput + "<div>content</div>";"#;
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "xss_vulnerability"));
    }
}
