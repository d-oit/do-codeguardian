use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
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

pub struct NonProductionAnalyzer {
    // Patterns for detecting non-production code
    todo_pattern: Regex,
    debug_pattern: Regex,
    console_pattern: Regex,
}

impl NonProductionAnalyzer {
    pub fn new() -> Self {
        Self {
            todo_pattern: Regex::new(r"(?i)(todo|fixme|hack|xxx|bug)").unwrap(),
            debug_pattern: Regex::new(r"(?i)(debug|debugger|console\.log|print\(|println!)").unwrap(),
            console_pattern: Regex::new(r"console\.(log|debug|info|warn|error)").unwrap(),
        }
    }
    
    fn check_non_production_code(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        
        for (line_num, line) in content_str.lines().enumerate() {
            let line_number = (line_num + 1) as u32;
            
            // Check for TODO/FIXME comments
            if let Some(captures) = self.todo_pattern.captures(line) {
                let keyword = captures.get(1).unwrap().as_str();
                let severity = match keyword.to_lowercase().as_str() {
                    "bug" | "xxx" => Severity::High,
                    "fixme" | "hack" => Severity::Medium,
                    "todo" => Severity::Low,
                    _ => Severity::Info,
                };
                
                findings.push(
                    Finding::new(
                        "non_production",
                        "todo_comment",
                        severity,
                        file_path.to_path_buf(),
                        line_number,
                        format!("{} comment found", keyword.to_uppercase()),
                    )
                    .with_description(format!("Line contains a {} comment that should be resolved before production", keyword))
                    .with_suggestion("Resolve the issue or create a proper issue tracker entry".to_string())
                    .with_metadata("comment_type".to_string(), serde_json::Value::String(keyword.to_string()))
                );
            }
            
            // Check for debug statements
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
                    .with_description("Debug statements should not be present in production code".to_string())
                    .with_suggestion("Remove debug statements or replace with proper logging".to_string())
                );
            }
            
            // Check for console statements in JavaScript/TypeScript
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
                    .with_description("Console statements should be replaced with proper logging in production".to_string())
                    .with_suggestion("Use a proper logging library instead of console statements".to_string())
                );
            }
            
            // Check for hardcoded credentials or secrets
            if self.contains_potential_secret(line) {
                // Check if this is in a test or non-production context
                let context = self.analyze_secret_context(file_path, line);
                let (severity, message, description) = match context {
                    SecretContext::Test => (
                        Severity::Info,
                        "Hardcoded secret in test code".to_string(),
                        "Test secrets should use mock values or be clearly marked as test data".to_string()
                    ),
                    SecretContext::NonProduction => (
                        Severity::Low,
                        "Hardcoded secret in non-production code".to_string(),
                        "Non-production secrets should be externalized or clearly documented".to_string()
                    ),
                    SecretContext::Production => (
                        Severity::Critical,
                        "Potential hardcoded secret detected".to_string(),
                        "Line may contain hardcoded credentials or API keys".to_string()
                    ),
                };
                
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
                    .with_suggestion("Move secrets to environment variables or secure configuration".to_string())
                    .with_metadata("context".to_string(), serde_json::Value::String(context.to_string()))
                );
            }
        }
        
        Ok(findings)
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
            "password", "secret", "token", "api_key", "apikey",
            "private_key", "access_key", "auth_token"
        ];
        
        for indicator in &secret_indicators {
            if line_lower.contains(indicator) && line.contains("=") {
                // Check if it looks like an assignment with a non-placeholder value
                if let Some(value_part) = line.split('=').nth(1) {
                    let value = value_part.trim().trim_matches('"').trim_matches('\'');
                    // Skip obvious placeholders
                    if !value.is_empty() && 
                       !value.contains("your_") && 
                       !value.contains("placeholder") &&
                       !value.contains("example") &&
                       value.len() > 8 {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// Analyze the context of a potential secret to determine if it's in test/non-production code
    fn analyze_secret_context(&self, file_path: &Path, line: &str) -> SecretContext {
        // Check file path indicators
        let file_path_str = file_path.to_string_lossy().to_lowercase();
        
        // Test file indicators
        if file_path_str.contains("test") || 
           file_path_str.contains("spec") || 
           file_path_str.contains("__tests__") ||
           file_path_str.contains("tests/") ||
           file_path_str.contains("/test/") ||
           file_path_str.ends_with("_test.rs") ||
           file_path_str.ends_with(".test.js") ||
           file_path_str.ends_with(".test.ts") ||
           file_path_str.ends_with("_spec.rb") {
            return SecretContext::Test;
        }
        
        // Non-production file indicators
        if file_path_str.contains("example") ||
           file_path_str.contains("demo") ||
           file_path_str.contains("sample") ||
           file_path_str.contains("mock") ||
           file_path_str.contains("fixture") ||
           file_path_str.contains("dev") ||
           file_path_str.contains("development") ||
           file_path_str.contains("staging") {
            return SecretContext::NonProduction;
        }
        
        // Check line content for test/non-production indicators
        let line_lower = line.to_lowercase();
        
        // Test function/method indicators
        if line_lower.contains("#[test]") ||
           line_lower.contains("fn test_") ||
           line_lower.contains("function test") ||
           line_lower.contains("it(") ||
           line_lower.contains("describe(") ||
           line_lower.contains("test(") ||
           line_lower.contains("@test") ||
           line_lower.contains("def test_") ||
           line_lower.contains("class test") {
            return SecretContext::Test;
        }
        
        // Non-production indicators in code
        if line_lower.contains("example") ||
           line_lower.contains("demo") ||
           line_lower.contains("sample") ||
           line_lower.contains("mock") ||
           line_lower.contains("fake") ||
           line_lower.contains("dummy") ||
           line_lower.contains("placeholder") ||
           line_lower.contains("test_") ||
           line_lower.contains("_test") ||
           line_lower.contains("dev_") ||
           line_lower.contains("development") {
            return SecretContext::NonProduction;
        }
        
        // Check for obvious test/example values (but not if already in test context)
        if line_lower.contains("your_") ||
           line_lower.contains("replace_") ||
           line_lower.contains("insert_") ||
           line_lower.contains("placeholder") ||
           line_lower.contains("example") ||
           line_lower.contains("demo") ||
           line_lower.contains("mock") ||
           line_lower.contains("fake") ||
           line_lower.contains("dummy") {
            return SecretContext::NonProduction;
        }
        
        // Check for obvious placeholder patterns (but be more specific)
        if line_lower.contains("xxxx") ||
           line_lower.contains("abcd") ||
           (line_lower.contains("1234") && (
               line_lower.contains("example") ||
               line_lower.contains("demo") ||
               line_lower.contains("test") ||
               line_lower.contains("placeholder") ||
               line_lower.contains("your_")
           )) {
            return SecretContext::NonProduction;
        }
        
        // Default to production context for safety
        SecretContext::Production
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
            matches!(ext, 
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | 
                "cpp" | "c" | "h" | "hpp" | "go" | "rb" | "php" |
                "cs" | "swift" | "kt" | "scala" | "clj" | "hs"
            )
        } else {
            false
        }
    }
}