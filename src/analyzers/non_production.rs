use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

/// Context in which a secret is found
#[derive(Debug, Clone, PartialEq)]
enum SecretContext {
    Test,
    NonProduction,
    Production,
}

impl std::fmt::Display for SecretContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretContext::Test => write!(f, "test"),
            SecretContext::NonProduction => write!(f, "non_production"),
            SecretContext::Production => write!(f, "production"),
        }
    }
}

// Lazy static regex patterns for optimal performance
static TODO_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(todo|fixme|hack|xxx|bug)").unwrap());

static DEBUG_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(debug|debugger|console\.log|print\(|println!)").unwrap());

static CONSOLE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"console\.(log|debug|info|warn|error)").unwrap());

pub struct NonProductionAnalyzer {
    // Use static references to avoid recompilation overhead
    todo_pattern: &'static Regex,
    debug_pattern: &'static Regex,
    console_pattern: &'static Regex,
}

impl Default for NonProductionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl NonProductionAnalyzer {
    pub fn new() -> Self {
        Self {
            todo_pattern: &TODO_PATTERN,
            debug_pattern: &DEBUG_PATTERN,
            console_pattern: &CONSOLE_PATTERN,
        }
    }

    fn check_non_production_code(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);

        for (line_num, line) in content_str.lines().enumerate() {
            let line_number = (line_num + 1) as u32;

            // Check for different types of non-production code patterns
            findings.extend(self.check_todo_comments(file_path, line, line_number));
            findings.extend(self.check_debug_statements(file_path, line, line_number));
            findings.extend(self.check_console_statements(file_path, line, line_number));
            findings.extend(self.check_hardcoded_secrets(file_path, line, line_number));
        }

        Ok(findings)
    }

    /// Check for TODO/FIXME comments
    fn check_todo_comments(&self, file_path: &Path, line: &str, line_number: u32) -> Vec<Finding> {
        let mut findings = Vec::new();

        if let Some(captures) = self.todo_pattern.captures(line) {
            let keyword = captures.get(1).unwrap().as_str();
            let severity = self.get_todo_severity(keyword);

            findings.push(
                Finding::new(
                    "non_production",
                    "todo_comment",
                    severity,
                    file_path.to_path_buf(),
                    line_number,
                    format!("{} comment found", keyword.to_uppercase()),
                )
                .with_description(format!(
                    "Line contains a {} comment that should be resolved before production",
                    keyword
                ))
                .with_suggestion(
                    "Resolve the issue or create a proper issue tracker entry".to_string(),
                )
                .with_metadata(
                    "comment_type".to_string(),
                    serde_json::Value::String(keyword.to_string()),
                ),
            );
        }

        findings
    }

    /// Determine severity for TODO comment types
    fn get_todo_severity(&self, keyword: &str) -> Severity {
        match keyword.to_lowercase().as_str() {
            "bug" | "xxx" => Severity::High,
            "fixme" | "hack" => Severity::Medium,
            "todo" => Severity::Low,
            _ => Severity::Info,
        }
    }

    /// Check for debug statements
    fn check_debug_statements(&self, file_path: &Path, line: &str, line_number: u32) -> Vec<Finding> {
        let mut findings = Vec::new();

        if self.debug_pattern.is_match(line) {
            let severity = if line.contains("debugger") {
                Severity::High
            } else {
                Severity::Medium
            };

            findings.push(
                Finding::new(
                    "non_production",
                    "debug_statement",
                    severity,
                    file_path.to_path_buf(),
                    line_number,
                    "Debug statement found".to_string(),
                )
                .with_description(
                    "Debug statements should not be present in production code".to_string(),
                )
                .with_suggestion(
                    "Remove debug statements or replace with proper logging".to_string(),
                ),
            );
        }

        findings
    }

    /// Check for console statements in JavaScript/TypeScript
    fn check_console_statements(&self, file_path: &Path, line: &str, line_number: u32) -> Vec<Finding> {
        let mut findings = Vec::new();

        if self.is_js_ts_file(file_path) && self.console_pattern.is_match(line) {
            findings.push(
                Finding::new(
                    "non_production",
                    "console_statement",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Console statement found".to_string(),
                )
                .with_description(
                    "Console statements should be replaced with proper logging in production"
                        .to_string(),
                )
                .with_suggestion(
                    "Use a proper logging library instead of console statements".to_string(),
                ),
            );
        }

        findings
    }

    /// Check for hardcoded credentials or secrets
    fn check_hardcoded_secrets(&self, file_path: &Path, line: &str, line_number: u32) -> Vec<Finding> {
        let mut findings = Vec::new();

        if self.contains_potential_secret(line) {
            let context = self.analyze_secret_context(file_path, line);
            let (severity, message, description) = self.get_secret_finding_details(&context);

            findings.push(
                Finding::new(
                    "non_production",
                    "potential_secret",
                    severity,
                    file_path.to_path_buf(),
                    line_number,
                    message,
                )
                .with_description(description)
                .with_suggestion(
                    "Move secrets to environment variables or secure configuration".to_string(),
                )
                .with_metadata(
                    "context".to_string(),
                    serde_json::Value::String(context.to_string()),
                ),
            );
        }

        findings
    }

    /// Get finding details based on secret context
    fn get_secret_finding_details(&self, context: &SecretContext) -> (Severity, String, String) {
        match context {
            SecretContext::Test => (
                Severity::Info,
                "Hardcoded secret in test code".to_string(),
                "Test secrets should use mock values or be clearly marked as test data"
                    .to_string(),
            ),
            SecretContext::NonProduction => (
                Severity::Low,
                "Hardcoded secret in non-production code".to_string(),
                "Non-production secrets should be externalized or clearly documented"
                    .to_string(),
            ),
            SecretContext::Production => (
                Severity::Critical,
                "Potential hardcoded secret detected".to_string(),
                "Line may contain hardcoded credentials or API keys".to_string(),
            ),
        }
    }

    fn is_js_ts_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "js" | "ts" | "jsx" | "tsx")
        } else {
            false
        }
    }

    fn contains_potential_secret(&self, line: &str) -> bool {
        let line_lower = line.to_lowercase();

        // Look for patterns that might indicate secrets
        let secret_indicators = [
            "password",
            "secret",
            "token",
            "api_key",
            "apikey",
            "private_key",
            "access_key",
            "auth_token",
        ];

        for indicator in &secret_indicators {
            if line_lower.contains(indicator) && line.contains("=") {
                // Skip enum display implementations and obvious non-secret contexts
                if line.contains("write!(f,") || line.contains("SecretContext::") {
                    continue;
                }

                // Check if it looks like an assignment with a non-placeholder value
                if let Some(value_part) = line.split('=').nth(1) {
                    let value = value_part.trim().trim_matches('"').trim_matches('\'');
                    // Skip obvious placeholders and enum values
                    if !value.is_empty()
                        && !value.contains("your_")
                        && !value.contains("placeholder")
                        && !value.contains("example")
                        && !value.contains("test")
                        && !value.contains("non_production")
                        && !value.contains("production")
                        && value.len() > 8
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Analyze the context of a potential secret to determine if it's in test/non-production code
    fn analyze_secret_context(&self, file_path: &Path, line: &str) -> SecretContext {
        // Check file path indicators first (most reliable)
        if self.is_test_file(file_path) {
            return SecretContext::Test;
        }

        if self.is_non_production_file(file_path) {
            return SecretContext::NonProduction;
        }

        // Check line content for context indicators
        let line_lower = line.to_lowercase();

        if self.is_test_code_line(&line_lower) {
            return SecretContext::Test;
        }

        if self.is_non_production_code_line(&line_lower) {
            return SecretContext::NonProduction;
        }

        if self.is_placeholder_value(&line_lower) {
            return SecretContext::NonProduction;
        }

        // Default to production context for safety
        SecretContext::Production
    }

    /// Check if file path indicates a test file
    fn is_test_file(&self, file_path: &Path) -> bool {
        let file_path_str = file_path.to_string_lossy().to_lowercase();
        let test_indicators = [
            "test", "spec", "__tests__", "tests/", "/test/",
            "_test.rs", ".test.js", ".test.ts", "_spec.rb"
        ];

        test_indicators.iter().any(|&indicator| file_path_str.contains(indicator))
    }

    /// Check if file path indicates a non-production file
    fn is_non_production_file(&self, file_path: &Path) -> bool {
        let file_path_str = file_path.to_string_lossy().to_lowercase();
        let non_prod_indicators = [
            "example", "demo", "sample", "mock", "fixture",
            "dev", "development", "staging"
        ];

        non_prod_indicators.iter().any(|&indicator| file_path_str.contains(indicator))
    }

    /// Check if line content indicates test code
    fn is_test_code_line(&self, line_lower: &str) -> bool {
        let test_code_indicators = [
            "#[test]", "fn test_", "function test", "it(", "describe(",
            "test(", "@test", "def test_", "class test"
        ];

        test_code_indicators.iter().any(|&indicator| line_lower.contains(indicator))
    }

    /// Check if line content indicates non-production code
    fn is_non_production_code_line(&self, line_lower: &str) -> bool {
        let non_prod_indicators = [
            "example", "demo", "sample", "mock", "fake", "dummy",
            "placeholder", "test_", "_test", "dev_", "development"
        ];

        non_prod_indicators.iter().any(|&indicator| line_lower.contains(indicator))
    }

    /// Check if line contains obvious placeholder values
    fn is_placeholder_value(&self, line_lower: &str) -> bool {
        // Check for obvious placeholder prefixes
        if line_lower.contains("your_") || line_lower.contains("replace_") ||
           line_lower.contains("insert_") || line_lower.contains("placeholder") {
            return true;
        }

        // Check for common placeholder patterns
        if line_lower.contains("xxxx") || line_lower.contains("abcd") {
            return true;
        }

        // Check for 1234 with context words
        if line_lower.contains("1234") {
            let context_words = ["example", "demo", "test", "placeholder", "your_"];
            return context_words.iter().any(|&word| line_lower.contains(word));
        }

        false
    }
}

impl Analyzer for NonProductionAnalyzer {
    fn name(&self) -> &str {
        "non_production"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.check_non_production_code(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Support common source code file types
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js"
                    | "ts"
                    | "jsx"
                    | "tsx"
                    | "py"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
                    | "go"
                    | "rb"
                    | "php"
                    | "cs"
                    | "swift"
                    | "kt"
                    | "scala"
                    | "clj"
                    | "hs"
            )
        } else {
            false
        }
    }
}
