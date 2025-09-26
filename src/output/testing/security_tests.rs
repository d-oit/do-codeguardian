//! # Security Tests for Output Systems
//!
//! This module provides comprehensive security testing for output generation,
//! focusing on XSS prevention, content validation, and secure output handling.

use super::{measure_test_execution, TestResult};
use crate::output::formats::HtmlFormatter;
use crate::output::formatter::OutputFormatter;
use crate::output::security::{
    generate_csp_header, sanitize_file_path, sanitize_html, validate_content_security,
};
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::path::PathBuf;

/// Security test runner
pub struct SecurityTestRunner {
    test_cases: Vec<SecurityTestCase>,
}

/// Individual security test case
#[derive(Debug, Clone)]
pub struct SecurityTestCase {
    pub name: String,
    pub description: String,
    pub test_type: SecurityTestType,
    pub input_data: SecurityTestInput,
    pub expected_result: SecurityTestResult,
}

/// Types of security tests
#[derive(Debug, Clone)]
pub enum SecurityTestType {
    XSSPrevention,
    ContentValidation,
    CSPGeneration,
    FilePathSanitization,
    HTMLSanitization,
    EventHandlerRemoval,
    ScriptTagRemoval,
    ProtocolSanitization,
}

/// Security test input data
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum SecurityTestInput {
    MaliciousHtml(String),
    DangerousContent(String),
    FilePath(String),
    AnalysisResults(Box<AnalysisResults>),
}

/// Expected security test result
#[derive(Debug, Clone)]
pub enum SecurityTestResult {
    ShouldPass,
    ShouldFail(String),       // Expected error message pattern
    ShouldContain(String),    // Expected content in output
    ShouldNotContain(String), // Content that should be removed/filtered
}

impl Default for SecurityTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityTestRunner {
    /// Create a new security test runner
    pub fn new() -> Self {
        Self {
            test_cases: Self::generate_security_test_cases(),
        }
    }

    /// Generate comprehensive security test cases
    fn generate_security_test_cases() -> Vec<SecurityTestCase> {
        vec![
            // XSS Prevention Tests
            SecurityTestCase {
                name: "xss_script_tag_removal".to_string(),
                description: "Removes script tags from HTML content".to_string(),
                test_type: SecurityTestType::ScriptTagRemoval,
                input_data: SecurityTestInput::MaliciousHtml(
                    r#"<div>Safe content</div><script>alert('xss')</script>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldNotContain("<script".to_string()),
            },
            SecurityTestCase {
                name: "xss_event_handler_removal".to_string(),
                description: "Removes dangerous event handlers".to_string(),
                test_type: SecurityTestType::EventHandlerRemoval,
                input_data: SecurityTestInput::MaliciousHtml(
                    r#"<div onclick="alert('xss')">Click me</div>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldNotContain("onclick".to_string()),
            },
            SecurityTestCase {
                name: "xss_javascript_protocol_removal".to_string(),
                description: "Removes javascript: protocol from links".to_string(),
                test_type: SecurityTestType::ProtocolSanitization,
                input_data: SecurityTestInput::MaliciousHtml(
                    r#"<a href="javascript:alert('xss')">Link</a>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldNotContain("javascript:".to_string()),
            },
            SecurityTestCase {
                name: "xss_dangerous_characters_escape".to_string(),
                description: "Escapes dangerous HTML characters".to_string(),
                test_type: SecurityTestType::HTMLSanitization,
                input_data: SecurityTestInput::MaliciousHtml(
                    r#"<div>& < > " ' </div>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldContain(
                    "&amp; &lt; &gt; &quot; &#x27;".to_string(),
                ),
            },
            // Content Validation Tests
            SecurityTestCase {
                name: "content_validation_script_detection".to_string(),
                description: "Detects script tags in content validation".to_string(),
                test_type: SecurityTestType::ContentValidation,
                input_data: SecurityTestInput::DangerousContent(
                    r#"<html><script>alert('test')</script></html>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldFail("script tags".to_string()),
            },
            SecurityTestCase {
                name: "content_validation_javascript_protocol".to_string(),
                description: "Detects javascript: protocol in validation".to_string(),
                test_type: SecurityTestType::ContentValidation,
                input_data: SecurityTestInput::DangerousContent(
                    r#"<a href="javascript:evil()">Link</a>"#.to_string(),
                ),
                expected_result: SecurityTestResult::ShouldFail("javascript: protocol".to_string()),
            },
            SecurityTestCase {
                name: "content_validation_large_content".to_string(),
                description: "Handles extremely large content safely".to_string(),
                test_type: SecurityTestType::ContentValidation,
                input_data: SecurityTestInput::DangerousContent("x".repeat(60 * 1024 * 1024)), // 60MB
                expected_result: SecurityTestResult::ShouldFail(
                    "Content size is very large".to_string(),
                ),
            },
            // File Path Sanitization Tests
            SecurityTestCase {
                name: "file_path_directory_traversal".to_string(),
                description: "Prevents directory traversal attacks".to_string(),
                test_type: SecurityTestType::FilePathSanitization,
                input_data: SecurityTestInput::FilePath("../../../etc/passwd".to_string()),
                expected_result: SecurityTestResult::ShouldContain("etc/passwd".to_string()),
            },
            SecurityTestCase {
                name: "file_path_double_slash".to_string(),
                description: "Normalizes double slashes in paths".to_string(),
                test_type: SecurityTestType::FilePathSanitization,
                input_data: SecurityTestInput::FilePath("path//to//file".to_string()),
                expected_result: SecurityTestResult::ShouldContain("path/to/file".to_string()),
            },
            SecurityTestCase {
                name: "file_path_empty_after_sanitization".to_string(),
                description: "Handles paths that become empty after sanitization".to_string(),
                test_type: SecurityTestType::FilePathSanitization,
                input_data: SecurityTestInput::FilePath("../../../".to_string()),
                expected_result: SecurityTestResult::ShouldFail(
                    "empty after sanitization".to_string(),
                ),
            },
            // CSP Header Tests
            SecurityTestCase {
                name: "csp_header_generation".to_string(),
                description: "Generates proper CSP header".to_string(),
                test_type: SecurityTestType::CSPGeneration,
                input_data: SecurityTestInput::MaliciousHtml("".to_string()), // Not used for CSP
                expected_result: SecurityTestResult::ShouldContain(
                    "default-src 'self'".to_string(),
                ),
            },
            // HTML Formatter Security Tests
            SecurityTestCase {
                name: "html_formatter_xss_in_finding_message".to_string(),
                description: "HTML formatter sanitizes XSS in finding messages".to_string(),
                test_type: SecurityTestType::XSSPrevention,
                input_data: SecurityTestInput::AnalysisResults(
                    Box::new(create_malicious_results()),
                ),
                expected_result: SecurityTestResult::ShouldNotContain("<script".to_string()),
            },
            SecurityTestCase {
                name: "html_formatter_xss_in_finding_description".to_string(),
                description: "HTML formatter sanitizes XSS in finding descriptions".to_string(),
                test_type: SecurityTestType::XSSPrevention,
                input_data: SecurityTestInput::AnalysisResults(Box::new(
                    create_malicious_description_results(),
                )),
                expected_result: SecurityTestResult::ShouldNotContain("javascript:".to_string()),
            },
        ]
    }

    /// Run all security tests
    pub async fn run_all_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        println!("🔒 Running {} security tests...", self.test_cases.len());

        for test_case in &self.test_cases {
            println!("  🛡️ Testing: {}", test_case.description);

            let test_result = self.run_single_security_test(test_case).await?;
            results.push(test_result);
        }

        println!("✅ Security tests completed: {} tests run", results.len());
        Ok(results)
    }

    /// Run a single security test
    async fn run_single_security_test(&self, test_case: &SecurityTestCase) -> Result<TestResult> {
        let test_name = format!("security_{}", test_case.name);

        measure_test_execution(&test_name, async {
            match &test_case.test_type {
                SecurityTestType::XSSPrevention => self.test_xss_prevention(test_case).await,
                SecurityTestType::ContentValidation => {
                    self.test_content_validation(test_case).await
                }
                SecurityTestType::CSPGeneration => self.test_csp_generation(test_case).await,
                SecurityTestType::FilePathSanitization => {
                    self.test_file_path_sanitization(test_case).await
                }
                SecurityTestType::HTMLSanitization => self.test_html_sanitization(test_case).await,
                SecurityTestType::EventHandlerRemoval => {
                    self.test_event_handler_removal(test_case).await
                }
                SecurityTestType::ScriptTagRemoval => self.test_script_tag_removal(test_case).await,
                SecurityTestType::ProtocolSanitization => {
                    self.test_protocol_sanitization(test_case).await
                }
            }
        })
        .await
    }

    /// Test XSS prevention
    async fn test_xss_prevention(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::AnalysisResults(results) => {
                let formatter: &dyn OutputFormatter = &HtmlFormatter::new();
                let output = formatter.format(results)?;

                match &test_case.expected_result {
                    SecurityTestResult::ShouldNotContain(forbidden) => {
                        if output.content.contains(forbidden) {
                            return Err(anyhow::anyhow!(
                                "XSS prevention failed: found forbidden content '{}'",
                                forbidden
                            ));
                        }
                    }
                    SecurityTestResult::ShouldContain(required) => {
                        if !output.content.contains(required) {
                            return Err(anyhow::anyhow!(
                                "XSS prevention failed: missing required content '{}'",
                                required
                            ));
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for XSS prevention test"
                ))
            }
        }
        Ok(())
    }

    /// Test content validation
    async fn test_content_validation(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::DangerousContent(content) => {
                let warnings = validate_content_security(content, "text/html")?;

                match &test_case.expected_result {
                    SecurityTestResult::ShouldFail(expected_pattern) => {
                        if !warnings.iter().any(|w| w.contains(expected_pattern)) {
                            return Err(anyhow::anyhow!(
                                "Content validation failed: expected warning containing '{}'",
                                expected_pattern
                            ));
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Unexpected result type for content validation test"
                        ))
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for content validation test"
                ))
            }
        }
        Ok(())
    }

    /// Test CSP header generation
    async fn test_csp_generation(&self, _test_case: &SecurityTestCase) -> Result<()> {
        let csp = generate_csp_header();

        if !csp.contains("default-src 'self'") {
            return Err(anyhow::anyhow!("CSP header missing default-src directive"));
        }

        if !csp.contains("script-src 'none'") {
            return Err(anyhow::anyhow!("CSP header missing script-src 'none'"));
        }

        if !csp.contains("object-src 'none'") {
            return Err(anyhow::anyhow!("CSP header missing object-src 'none'"));
        }

        Ok(())
    }

    /// Test file path sanitization
    async fn test_file_path_sanitization(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::FilePath(path) => {
                let result = sanitize_file_path(path);

                match &test_case.expected_result {
                    SecurityTestResult::ShouldContain(expected) => {
                        if let Ok(sanitized) = result {
                            if !sanitized.contains(expected) {
                                return Err(anyhow::anyhow!(
                                    "File path sanitization failed: expected '{}' in '{}'",
                                    expected,
                                    sanitized
                                ));
                            }
                        } else {
                            return Err(anyhow::anyhow!(
                                "File path sanitization failed unexpectedly"
                            ));
                        }
                    }
                    SecurityTestResult::ShouldFail(expected_error) => {
                        if let Ok(sanitized) = result {
                            return Err(anyhow::anyhow!(
                                "File path sanitization should have failed but got: '{}'",
                                sanitized
                            ));
                        } else {
                            let error_msg = result.unwrap_err().to_string();
                            if !error_msg.contains(expected_error) {
                                return Err(anyhow::anyhow!(
                                    "File path sanitization failed with wrong error: '{}'",
                                    error_msg
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for file path sanitization test"
                ))
            }
        }
        Ok(())
    }

    /// Test HTML sanitization
    async fn test_html_sanitization(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::MaliciousHtml(html) => {
                let sanitized = sanitize_html(html, None)?;

                match &test_case.expected_result {
                    SecurityTestResult::ShouldContain(expected) => {
                        if !sanitized.contains(expected) {
                            return Err(anyhow::anyhow!(
                                "HTML sanitization failed: expected '{}' in output",
                                expected
                            ));
                        }
                    }
                    SecurityTestResult::ShouldNotContain(forbidden) => {
                        if sanitized.contains(forbidden) {
                            return Err(anyhow::anyhow!(
                                "HTML sanitization failed: found forbidden content '{}'",
                                forbidden
                            ));
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for HTML sanitization test"
                ))
            }
        }
        Ok(())
    }

    /// Test event handler removal
    async fn test_event_handler_removal(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::MaliciousHtml(html) => {
                let sanitized = sanitize_html(html, None)?;

                if let SecurityTestResult::ShouldNotContain(forbidden) = &test_case.expected_result
                {
                    if sanitized.contains(forbidden) {
                        return Err(anyhow::anyhow!(
                            "Event handler removal failed: found '{}'",
                            forbidden
                        ));
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for event handler removal test"
                ))
            }
        }
        Ok(())
    }

    /// Test script tag removal
    async fn test_script_tag_removal(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::MaliciousHtml(html) => {
                let sanitized = sanitize_html(html, None)?;

                if let SecurityTestResult::ShouldNotContain(forbidden) = &test_case.expected_result
                {
                    if sanitized.contains(forbidden) {
                        return Err(anyhow::anyhow!(
                            "Script tag removal failed: found '{}'",
                            forbidden
                        ));
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for script tag removal test"
                ))
            }
        }
        Ok(())
    }

    /// Test protocol sanitization
    async fn test_protocol_sanitization(&self, test_case: &SecurityTestCase) -> Result<()> {
        match &test_case.input_data {
            SecurityTestInput::MaliciousHtml(html) => {
                let sanitized = sanitize_html(html, None)?;

                if let SecurityTestResult::ShouldNotContain(forbidden) = &test_case.expected_result
                {
                    if sanitized.contains(forbidden) {
                        return Err(anyhow::anyhow!(
                            "Protocol sanitization failed: found '{}'",
                            forbidden
                        ));
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid input data for protocol sanitization test"
                ))
            }
        }
        Ok(())
    }
}

/// Helper function to create test results with malicious content
fn create_malicious_results() -> AnalysisResults {
    let mut results = AnalysisResults::new("test_config".to_string());

    let finding = Finding::new(
        "test_analyzer",
        "test_rule",
        Severity::High,
        PathBuf::from("test.rs"),
        10,
        r#"<script>alert('xss')</script> Test finding message"#.to_string(),
    );

    results.add_finding(finding);
    results
}

/// Helper function to create test results with malicious description
fn create_malicious_description_results() -> AnalysisResults {
    let mut results = AnalysisResults::new("test_config".to_string());

    let mut finding = Finding::new(
        "test_analyzer",
        "test_rule",
        Severity::High,
        PathBuf::from("test.rs"),
        10,
        "Test finding message".to_string(),
    );

    finding.description = Some(r#"<a href="javascript:evil()">Click here</a>"#.to_string());

    results.add_finding(finding);
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::TestStatus;

    #[tokio::test]
    async fn test_security_test_runner_creation() -> Result<(), Box<dyn std::error::Error>> {
        let runner = SecurityTestRunner::new();
        assert!(!runner.test_cases.is_empty());
    }

    #[tokio::test]
    async fn test_security_tests_execution() -> Result<(), Box<dyn std::error::Error>> {
        let runner = SecurityTestRunner::new();
        let results = runner.run_all_tests().await?;
        assert!(!results.is_empty());

        // All tests should pass
        for result in results {
            assert_eq!(
                result.status,
                TestStatus::Passed,
                "Security test '{}' failed: {:?}",
                result.test_name,
                result.error_message
            );
        }
    }

    #[tokio::test]
    async fn test_xss_prevention_in_html_formatter() -> Result<(), Box<dyn std::error::Error>> {
        let _runner = SecurityTestRunner::new();
        let malicious_results = create_malicious_results();

        let formatter = HtmlFormatter::new();
        let output = formatter.format(&malicious_results)?;

        // Should not contain script tags after sanitization
        assert!(!output.content.contains("<script"));
        assert!(!output.content.contains("alert('xss')"));

        // Should still contain safe content
        assert!(output.content.contains("Test finding message"));
    }

    #[tokio::test]
    async fn test_csp_header_security() -> Result<(), Box<dyn std::error::Error>> {
        let csp = generate_csp_header();

        // Should be restrictive
        assert!(csp.contains("script-src 'none'"));
        assert!(csp.contains("object-src 'none'"));
        assert!(csp.contains("base-uri 'self'"));
    }

    #[tokio::test]
    async fn test_file_path_sanitization_security() -> Result<(), Box<dyn std::error::Error>> {
        // Should prevent directory traversal
        let result = sanitize_file_path("../../../etc/passwd")?;
        assert_eq!(result, "etc/passwd");

        // Should handle empty results
        assert!(sanitize_file_path("../../../").is_err());
    }
}
