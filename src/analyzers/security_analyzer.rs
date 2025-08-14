use crate::analyzers::Analyzer;
use crate::analyzers::optimized_patterns::{SECURITY_PATTERNS, AnalysisOptimizer, FileType, PatternCache};
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::collections::HashSet;

/// Advanced security analyzer for detecting vulnerabilities and security anti-patterns
pub struct SecurityAnalyzer {
    // Pattern cache for performance
    pattern_cache: PatternCache,
    // Known dangerous functions
    dangerous_functions: HashSet<String>,
    // Optimized patterns are accessed via lazy_static SECURITY_PATTERNS
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

        // Use optimized line analysis
        let line_findings = AnalysisOptimizer::analyze_lines_optimized(&content_str, |line_num, line| {
            self.analyze_line_security(file_path, line, (line_num + 1) as u32)
        });

        for (line_num, finding_msg) in line_findings {
            // Convert to Finding objects (simplified for performance)
            findings.push(Finding::new(
                "security",
                "security_issue",
                Severity::Medium,
                file_path.to_path_buf(),
                (line_num + 1) as u32,
                finding_msg,
            ));
        }

        // File-level security checks (optimized)
        findings.extend(self.check_file_level_security_fast(file_path, &content_str)?);

        Ok(findings)
    }

    fn analyze_line_security(&mut self, file_path: &Path, line: &str, line_number: u32) -> Option<String> {
        // Fast checks using optimized patterns
        
        // Check for hardcoded secrets first (most critical)
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.secrets_combined, line) {
            return Some("Potential hardcoded secret detected".to_string());
        }
        
        // Check for SQL injection vulnerabilities
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.sql_injection_fast, line) {
            return Some("Potential SQL injection vulnerability".to_string());
        }
        
        // Check for XSS vulnerabilities
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.xss_fast, line) {
            return Some("Potential XSS vulnerability".to_string());
        }
        
        // Check for command injection
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.command_injection_fast, line) {
            return Some("Potential command injection vulnerability".to_string());
        }
        
        // Check for weak cryptography
        if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.weak_crypto_fast, line) {
            return Some("Weak cryptographic algorithm detected".to_string());
        }
        
        // Check for dangerous functions
        for func in &self.dangerous_functions {
            if line.contains(func) && !line.trim_start().starts_with("//") && !line.trim_start().starts_with("#") {
                return Some(format!("Dangerous function '{}' detected", func));
            }
        }
        
        None
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
                        line_number,
                        "Potential SQL injection vulnerability".to_string(),
                    )
                    .with_description("Dynamic SQL query construction can lead to SQL injection attacks")
                    .with_suggestion("Use parameterized queries or prepared statements instead of string concatenation")
                );
            }

            // Check for XSS vulnerabilities
            if self.xss_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "xss_vulnerability",
                        Severity::High,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential XSS vulnerability".to_string(),
                    )
                    .with_description("Dynamic HTML content insertion can lead to cross-site scripting attacks")
                    .with_suggestion("Sanitize user input and use safe DOM manipulation methods")
                );
            }

            // Check for command injection
            if self.command_injection_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "command_injection",
                        Severity::Critical,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential command injection vulnerability".to_string(),
                    )
                    .with_description("Dynamic command execution can lead to arbitrary code execution")
                    .with_suggestion("Avoid dynamic command construction; use safe APIs or validate input strictly")
                );
            }

            // Check for weak cryptography
            if self.weak_crypto_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "weak_cryptography",
                        Severity::High,
                        file_path.to_path_buf(),
                        line_number,
                        "Weak cryptographic algorithm detected".to_string(),
                    )
                    .with_description("MD5, SHA1, and DES are cryptographically broken and should not be used")
                    .with_suggestion("Use SHA-256, SHA-3, or other modern cryptographic algorithms")
                );
            }

            // Check for authentication bypass patterns
            if self.auth_bypass_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "auth_bypass",
                        Severity::Critical,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential authentication bypass".to_string(),
                    )
                    .with_description("Hardcoded authentication logic or always-true conditions detected")
                    .with_suggestion("Implement proper authentication checks and avoid hardcoded credentials")
                );
            }

            // Check for path traversal vulnerabilities
            if self.path_traversal_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "path_traversal",
                        Severity::High,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential path traversal vulnerability".to_string(),
                    )
                    .with_description("Unsanitized file path construction can lead to directory traversal attacks")
                    .with_suggestion("Validate and sanitize file paths; use safe path joining methods")
                );
            }

            // Check for hardcoded secrets
            for pattern in &self.secret_patterns {
                if pattern.is_match(line) {
                    findings.push(
                        Finding::new(
                            "security",
                            "hardcoded_secret",
                            Severity::Critical,
                            file_path.to_path_buf(),
                            line_number,
                            "Hardcoded secret or credential detected".to_string(),
                        )
                        .with_description("Hardcoded secrets in source code pose a significant security risk")
                        .with_suggestion("Move secrets to environment variables or secure configuration management")
                    );
                    break; // Only report once per line
                }
            }

            // Check for weak random number generation
            if self.weak_random_pattern.is_match(line) && self.is_security_context(line) {
                findings.push(
                    Finding::new(
                        "security",
                        "weak_random",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "Weak random number generation for security purposes".to_string(),
                    )
                    .with_description("Predictable random numbers should not be used for security-critical operations")
                    .with_suggestion("Use cryptographically secure random number generators (CSPRNG)")
                );
            }

            // Check for dangerous functions
            for func in &self.dangerous_functions {
                if line.contains(func) && !line.trim_start().starts_with("//") && !line.trim_start().starts_with("#") {
                    findings.push(
                        Finding::new(
                            "security",
                            "dangerous_function",
                            Severity::High,
                            file_path.to_path_buf(),
                            line_number,
                            format!("Dangerous function '{}' detected", func),
                        )
                        .with_description("This function can execute arbitrary code and poses security risks")
                        .with_suggestion("Avoid using dangerous functions; use safer alternatives")
                    );
                }
            }

            // Language-specific security checks
            findings.extend(self.check_language_specific_security(file_path, line, line_number)?);
        }

        // File-level security checks
        findings.extend(self.check_file_level_security(file_path, &content_str)?);

        Ok(findings)
    }

    fn is_security_context(&self, line: &str) -> bool {
        let security_keywords = ["token", "password", "key", "auth", "session", "crypto", "random", "nonce", "salt"];
        let line_lower = line.to_lowercase();
        security_keywords.iter().any(|keyword| line_lower.contains(keyword))
    }

    fn check_language_specific_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "js" | "ts" | "jsx" | "tsx" => {
                    findings.extend(self.check_javascript_security(file_path, line, line_number)?);
                }
                "py" => {
                    findings.extend(self.check_python_security(file_path, line, line_number)?);
                }
                "php" => {
                    findings.extend(self.check_php_security(file_path, line, line_number)?);
                }
                "java" => {
                    findings.extend(self.check_java_security(file_path, line, line_number)?);
                }
                "rs" => {
                    findings.extend(self.check_rust_security(file_path, line, line_number)?);
                }
                _ => {}
            }
        }

        Ok(findings)
    }

    fn check_javascript_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe eval usage
        if line.contains("eval(") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_eval",
                    Severity::Critical,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe eval() usage detected".to_string(),
                )
                .with_description("eval() can execute arbitrary code and is a major security risk")
                .with_suggestion("Avoid eval(); use JSON.parse() for data or safer alternatives")
            );
        }

        // Check for unsafe innerHTML with user data
        if line.contains("innerHTML") && (line.contains("req.") || line.contains("input") || line.contains("params")) {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_innerhtml",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe innerHTML with user data".to_string(),
                )
                .with_description("Setting innerHTML with user data can lead to XSS attacks")
                .with_suggestion("Use textContent or sanitize user input before setting innerHTML")
            );
        }

        // Check for insecure HTTP requests
        if line.contains("http://") && !line.contains("localhost") && !line.contains("127.0.0.1") {
            findings.push(
                Finding::new(
                    "security",
                    "insecure_http",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "Insecure HTTP URL detected".to_string(),
                )
                .with_description("HTTP connections are not encrypted and can be intercepted")
                .with_suggestion("Use HTTPS for all external communications")
            );
        }

        Ok(findings)
    }

    fn check_python_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for pickle usage (deserialization vulnerability)
        if line.contains("pickle.load") || line.contains("pickle.loads") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_deserialization",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe pickle deserialization".to_string(),
                )
                .with_description("Pickle can execute arbitrary code during deserialization")
                .with_suggestion("Use safer serialization formats like JSON or validate pickle sources")
            );
        }

        // Check for shell=True in subprocess
        if line.contains("subprocess") && line.contains("shell=True") {
            findings.push(
                Finding::new(
                    "security",
                    "shell_injection",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Subprocess with shell=True".to_string(),
                )
                .with_description("shell=True can lead to shell injection vulnerabilities")
                .with_suggestion("Use shell=False and pass commands as lists")
            );
        }

        // Check for unsafe YAML loading
        if line.contains("yaml.load(") && !line.contains("Loader=") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_yaml",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe YAML loading".to_string(),
                )
                .with_description("yaml.load() without a Loader can execute arbitrary code")
                .with_suggestion("Use yaml.safe_load() or specify a safe Loader")
            );
        }

        Ok(findings)
    }

    fn check_php_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe include/require
        if (line.contains("include") || line.contains("require")) && line.contains("$_") {
            findings.push(
                Finding::new(
                    "security",
                    "file_inclusion",
                    Severity::Critical,
                    file_path.to_path_buf(),
                    line_number,
                    "Potential file inclusion vulnerability".to_string(),
                )
                .with_description("Including files based on user input can lead to remote file inclusion")
                .with_suggestion("Validate and whitelist file paths before inclusion")
            );
        }

        // Check for unsafe deserialization
        if line.contains("unserialize(") && line.contains("$_") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_unserialize",
                    Severity::Critical,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe unserialize() with user data".to_string(),
                )
                .with_description("Unserializing user data can lead to object injection attacks")
                .with_suggestion("Validate data before unserialization or use safer formats like JSON")
            );
        }

        Ok(findings)
    }

    fn check_java_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe deserialization
        if line.contains("ObjectInputStream") && line.contains("readObject") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_deserialization",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe object deserialization".to_string(),
                )
                .with_description("Deserializing untrusted data can lead to remote code execution")
                .with_suggestion("Validate serialized data or use safer serialization methods")
            );
        }

        // Check for SQL injection in JDBC
        if line.contains("Statement") && line.contains("executeQuery") && line.contains("+") {
            findings.push(
                Finding::new(
                    "security",
                    "jdbc_sql_injection",
                    Severity::Critical,
                    file_path.to_path_buf(),
                    line_number,
                    "Potential SQL injection in JDBC".to_string(),
                )
                .with_description("String concatenation in SQL queries can lead to SQL injection")
                .with_suggestion("Use PreparedStatement with parameterized queries")
            );
        }

        Ok(findings)
    }

    fn check_rust_security(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe blocks
        if line.contains("unsafe") && !line.trim_start().starts_with("//") {
            findings.push(
                Finding::new(
                    "security",
                    "unsafe_block",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "Unsafe block detected".to_string(),
                )
                .with_description("Unsafe blocks bypass Rust's safety guarantees")
                .with_suggestion("Ensure unsafe code is necessary and properly documented")
            );
        }

        // Check for potential integer overflow
        if line.contains("as u") || line.contains("as i") {
            if line.contains("*") || line.contains("+") {
                findings.push(
                    Finding::new(
                        "security",
                        "potential_overflow",
                        Severity::Low,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential integer overflow in cast".to_string(),
                    )
                    .with_description("Arithmetic operations before casting can overflow")
                    .with_suggestion("Use checked arithmetic or validate ranges before casting")
                );
            }
        }

        Ok(findings)
    }

    fn check_file_level_security(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for files that might contain sensitive information
        if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
            let sensitive_files = [
                ".env", "config.json", "secrets.json", "credentials.json",
                "private.key", "id_rsa", "id_dsa", "database.yml"
            ];

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
                    .with_description("This file may contain sensitive information")
                    .with_suggestion("Ensure this file is not committed to version control and is properly secured")
                );
            }
        }

        // Check for high entropy strings (potential secrets)
        let lines: Vec<&str> = content.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            if self.has_high_entropy(line) && line.len() > 20 {
                findings.push(
                    Finding::new(
                        "security",
                        "high_entropy_string",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        (line_num + 1) as u32,
                        "High entropy string detected (possible secret)".to_string(),
                    )
                    .with_description("High entropy strings may indicate encoded secrets or keys")
                    .with_suggestion("Review if this string contains sensitive information")
                );
            }
        }

        Ok(findings)
    }

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