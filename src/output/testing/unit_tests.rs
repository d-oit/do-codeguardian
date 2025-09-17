//! # Unit Tests for Output Formatters
//!
//! This module provides comprehensive unit testing for all output formatters,
//! ensuring correctness, reliability, and adherence to specifications.

use super::{measure_test_execution, TestResult};
use crate::output::formats::*;
use crate::output::formatter::OutputFormatter;
use crate::output::validation::validate_output;
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;

/// Unit test runner for output formatters
pub struct UnitTestRunner {
    test_cases: Vec<TestCase>,
}

/// Individual test case
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub input_data: TestInputData,
    pub expected_outcome: ExpectedOutcome,
}

/// Test input data variations
#[derive(Debug, Clone)]
pub enum TestInputData {
    EmptyResults,
    SingleFinding,
    MultipleSeverities,
    LargeDataset(usize), // Number of findings
    SpecialCharacters,
    UnicodeContent,
    MalformedData,
}

/// Expected test outcome
#[derive(Debug, Clone)]
pub enum ExpectedOutcome {
    Success,
    ValidationError,
    FormattingError,
    PerformanceThreshold(u64), // Max time in milliseconds
}

impl UnitTestRunner {
    /// Create a new unit test runner
    pub fn new() -> Self {
        Self {
            test_cases: Self::generate_test_cases(),
        }
    }
}

impl Default for UnitTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitTestRunner {
    /// Generate comprehensive test cases
    fn generate_test_cases() -> Vec<TestCase> {
        vec![
            // Basic functionality tests
            TestCase {
                name: "json_empty_results".to_string(),
                description: "JSON formatter handles empty results".to_string(),
                input_data: TestInputData::EmptyResults,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "json_single_finding".to_string(),
                description: "JSON formatter handles single finding".to_string(),
                input_data: TestInputData::SingleFinding,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "json_multiple_severities".to_string(),
                description: "JSON formatter handles multiple severity levels".to_string(),
                input_data: TestInputData::MultipleSeverities,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "html_xss_prevention".to_string(),
                description: "HTML formatter prevents XSS attacks".to_string(),
                input_data: TestInputData::SpecialCharacters,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "markdown_unicode_support".to_string(),
                description: "Markdown formatter supports Unicode content".to_string(),
                input_data: TestInputData::UnicodeContent,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "sarif_compliance".to_string(),
                description: "SARIF formatter produces compliant output".to_string(),
                input_data: TestInputData::MultipleSeverities,
                expected_outcome: ExpectedOutcome::Success,
            },
            TestCase {
                name: "yaml_structure_validation".to_string(),
                description: "YAML formatter produces valid YAML structure".to_string(),
                input_data: TestInputData::SingleFinding,
                expected_outcome: ExpectedOutcome::Success,
            },
            // Performance tests
            TestCase {
                name: "json_large_dataset_performance".to_string(),
                description: "JSON formatter handles large datasets efficiently".to_string(),
                input_data: TestInputData::LargeDataset(10000),
                expected_outcome: ExpectedOutcome::PerformanceThreshold(5000), // 5 seconds
            },
            TestCase {
                name: "html_large_dataset_performance".to_string(),
                description: "HTML formatter handles large datasets efficiently".to_string(),
                input_data: TestInputData::LargeDataset(5000),
                expected_outcome: ExpectedOutcome::PerformanceThreshold(10000), // 10 seconds
            },
            // Edge cases
            TestCase {
                name: "all_formats_special_characters".to_string(),
                description: "All formatters handle special characters safely".to_string(),
                input_data: TestInputData::SpecialCharacters,
                expected_outcome: ExpectedOutcome::Success,
            },
        ]
    }

    /// Run all unit tests
    pub async fn run_all_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        println!("📋 Running {} unit tests...", self.test_cases.len());

        for test_case in &self.test_cases {
            println!("  🧪 Testing: {}", test_case.description);

            let test_result = self.run_single_test(test_case).await?;
            results.push(test_result);
        }

        // Run formatter-specific tests
        results.extend(self.test_json_formatter().await?);
        results.extend(self.test_html_formatter().await?);
        results.extend(self.test_markdown_formatter().await?);
        results.extend(self.test_sarif_formatter().await?);
        results.extend(self.test_yaml_formatter().await?);

        println!("✅ Unit tests completed: {} tests run", results.len());
        Ok(results)
    }

    /// Run a single test case
    async fn run_single_test(&self, test_case: &TestCase) -> Result<TestResult> {
        let test_name = format!("unit_{}", test_case.name);

        measure_test_execution(&test_name, async {
            let input_results = self.generate_test_data(&test_case.input_data).await?;

            // Test all formatters based on test case
            match test_case.name.split('_').next() {
                Some("json") => {
                    self.test_formatter_with_data(
                        &JsonFormatter::new(),
                        &input_results,
                        &test_case.expected_outcome,
                    )
                    .await
                }
                Some("html") => {
                    self.test_formatter_with_data(
                        &HtmlFormatter::new(),
                        &input_results,
                        &test_case.expected_outcome,
                    )
                    .await
                }
                Some("markdown") => {
                    self.test_formatter_with_data(
                        &MarkdownFormatter::new(),
                        &input_results,
                        &test_case.expected_outcome,
                    )
                    .await
                }
                Some("sarif") => {
                    self.test_formatter_with_data(
                        &SarifFormatter::new(),
                        &input_results,
                        &test_case.expected_outcome,
                    )
                    .await
                }
                Some("yaml") => {
                    self.test_formatter_with_data(
                        &YamlFormatter::new(),
                        &input_results,
                        &test_case.expected_outcome,
                    )
                    .await
                }
                _ => {
                    // Test all formatters for generic tests
                    self.test_all_formatters_with_data(&input_results, &test_case.expected_outcome)
                        .await
                }
            }
        })
        .await
    }

    /// Test a specific formatter with data
    async fn test_formatter_with_data(
        &self,
        formatter: &dyn OutputFormatter,
        data: &AnalysisResults,
        expected_outcome: &ExpectedOutcome,
    ) -> Result<()> {
        let start_time = Instant::now();
        let result = formatter.format(data);
        let execution_time = start_time.elapsed();

        match expected_outcome {
            ExpectedOutcome::Success => {
                let output = result?;
                // Validate the output
                validate_output(&output.content, formatter.content_type())?;
                Ok(())
            }
            ExpectedOutcome::ValidationError => {
                // Expect the result to fail validation
                if let Ok(output) = result {
                    if validate_output(&output.content, formatter.content_type()).is_ok() {
                        return Err(anyhow::anyhow!(
                            "Expected validation error but validation passed"
                        ));
                    }
                }
                Ok(())
            }
            ExpectedOutcome::FormattingError => {
                // Expect the formatting to fail
                if result.is_ok() {
                    return Err(anyhow::anyhow!(
                        "Expected formatting error but formatting succeeded"
                    ));
                }
                Ok(())
            }
            ExpectedOutcome::PerformanceThreshold(max_ms) => {
                let output = result?;
                validate_output(&output.content, formatter.content_type())?;

                if execution_time.as_millis() > *max_ms as u128 {
                    return Err(anyhow::anyhow!(
                        "Performance threshold exceeded: {}ms > {}ms",
                        execution_time.as_millis(),
                        max_ms
                    ));
                }
                Ok(())
            }
        }
    }

    /// Test all formatters with the same data
    async fn test_all_formatters_with_data(
        &self,
        data: &AnalysisResults,
        expected_outcome: &ExpectedOutcome,
    ) -> Result<()> {
        let formatters: Vec<Box<dyn OutputFormatter>> = vec![
            Box::new(JsonFormatter::new()),
            Box::new(HtmlFormatter::new()),
            Box::new(MarkdownFormatter::new()),
            Box::new(SarifFormatter::new()),
            Box::new(YamlFormatter::new()),
        ];

        for formatter in formatters {
            self.test_formatter_with_data(formatter.as_ref(), data, expected_outcome)
                .await?;
        }

        Ok(())
    }

    /// Generate test data based on input type
    async fn generate_test_data(&self, input_type: &TestInputData) -> Result<AnalysisResults> {
        match input_type {
            TestInputData::EmptyResults => Ok(AnalysisResults::new("test_config".to_string())),
            TestInputData::SingleFinding => {
                let mut results = AnalysisResults::new("test_config".to_string());
                let finding = Finding::new(
                    "test_analyzer",
                    "test_rule",
                    Severity::Medium,
                    PathBuf::from("test.rs"),
                    42,
                    "Test finding message".to_string(),
                );
                results.add_finding(finding);
                Ok(results)
            }
            TestInputData::MultipleSeverities => {
                let mut results = AnalysisResults::new("test_config".to_string());

                for (i, severity) in [
                    Severity::Critical,
                    Severity::High,
                    Severity::Medium,
                    Severity::Low,
                    Severity::Info,
                ]
                .iter()
                .enumerate()
                {
                    let finding = Finding::new(
                        "test_analyzer",
                        &format!("rule_{}", i),
                        severity.clone(),
                        PathBuf::from(format!("test_{}.rs", i)),
                        (i + 1) as u32 * 10,
                        format!("Test {} severity finding", severity),
                    );
                    results.add_finding(finding);
                }
                Ok(results)
            }
            TestInputData::LargeDataset(count) => {
                let mut results = AnalysisResults::new("test_config".to_string());

                for i in 0..*count {
                    let severity = match i % 5 {
                        0 => Severity::Critical,
                        1 => Severity::High,
                        2 => Severity::Medium,
                        3 => Severity::Low,
                        _ => Severity::Info,
                    };

                    let finding = Finding::new(
                        &format!("analyzer_{}", i % 10),
                        &format!("rule_{}", i % 50),
                        severity,
                        PathBuf::from(format!("file_{}.rs", i % 100)),
                        (i % 1000) as u32 + 1,
                        format!("Generated finding #{}", i),
                    );
                    results.add_finding(finding);
                }
                Ok(results)
            }
            TestInputData::SpecialCharacters => {
                let mut results = AnalysisResults::new("test_config".to_string());
                let finding = Finding::new(
                    "xss_analyzer",
                    "xss_rule",
                    Severity::High,
                    PathBuf::from("dangerous.js"),
                    1,
                    "XSS found: <script>alert('XSS')</script> & \"quoted\" content".to_string(),
                )
                .with_description(
                    "Description with <b>HTML</b> & special chars: '; DROP TABLE users; --"
                        .to_string(),
                );

                results.add_finding(finding);
                Ok(results)
            }
            TestInputData::UnicodeContent => {
                let mut results = AnalysisResults::new("test_config".to_string());
                let finding = Finding::new(
                    "unicode_analyzer",
                    "unicode_rule",
                    Severity::Info,
                    PathBuf::from("测试.rs"),
                    1,
                    "Unicode test: 🔒 Security issue with émojis and spëcial chârs".to_string(),
                )
                .with_description(
                    "Description with 中文, العربية, русский, and 🚀🔥💯 emojis".to_string(),
                );

                results.add_finding(finding);
                Ok(results)
            }
            TestInputData::MalformedData => {
                // This would be used for negative testing
                Ok(AnalysisResults::new("test_config".to_string()))
            }
        }
    }

    /// Test JSON formatter specifically
    async fn test_json_formatter(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test JSON structure validity
        results.push(
            measure_test_execution("unit_json_structure_validity", async {
                let formatter = JsonFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::MultipleSeverities)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Parse JSON to ensure it's valid
                let _parsed: serde_json::Value = serde_json::from_str(&output.content)?;

                Ok(())
            })
            .await?,
        );

        // Test JSON pretty printing
        results.push(
            measure_test_execution("unit_json_pretty_printing", async {
                let pretty_formatter = JsonFormatter::new();
                let compact_formatter = JsonFormatter::compact();
                let test_data = self
                    .generate_test_data(&TestInputData::SingleFinding)
                    .await?;

                let pretty_output = pretty_formatter.format(&test_data)?;
                let compact_output = compact_formatter.format(&test_data)?;

                // Pretty output should be longer (has whitespace)
                if pretty_output.content.len() <= compact_output.content.len() {
                    return Err(anyhow::anyhow!(
                        "Pretty printed JSON should be longer than compact"
                    ));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test HTML formatter specifically
    async fn test_html_formatter(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test HTML XSS prevention
        results.push(
            measure_test_execution("unit_html_xss_prevention", async {
                let formatter = HtmlFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::SpecialCharacters)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Should not contain unescaped script tags
                if output.content.contains("<script>alert('XSS')</script>") {
                    return Err(anyhow::anyhow!("HTML output contains unescaped script tag"));
                }

                // Should contain proper DOCTYPE
                if !output.content.contains("<!DOCTYPE html>") {
                    return Err(anyhow::anyhow!("HTML output missing DOCTYPE"));
                }

                Ok(())
            })
            .await?,
        );

        // Test HTML CSP header inclusion
        results.push(
            measure_test_execution("unit_html_csp_header", async {
                let formatter = HtmlFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::EmptyResults)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Should contain CSP header
                if !output.content.contains("Content-Security-Policy") {
                    return Err(anyhow::anyhow!("HTML output missing CSP header"));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test Markdown formatter specifically
    async fn test_markdown_formatter(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test Markdown structure
        results.push(
            measure_test_execution("unit_markdown_structure", async {
                let formatter = MarkdownFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::MultipleSeverities)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Should contain proper headers
                if !output.content.contains("# CodeGuardian Analysis Report") {
                    return Err(anyhow::anyhow!("Markdown output missing main header"));
                }

                // Should contain table of contents
                if !output.content.contains("## Table of Contents") {
                    return Err(anyhow::anyhow!("Markdown output missing table of contents"));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test SARIF formatter specifically
    async fn test_sarif_formatter(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test SARIF compliance
        results.push(
            measure_test_execution("unit_sarif_compliance", async {
                let formatter = SarifFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::MultipleSeverities)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Parse as JSON first
                let parsed: serde_json::Value = serde_json::from_str(&output.content)?;

                // Check required SARIF fields
                if parsed.get("version").is_none() {
                    return Err(anyhow::anyhow!("SARIF output missing version field"));
                }

                if parsed.get("runs").is_none() {
                    return Err(anyhow::anyhow!("SARIF output missing runs field"));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test YAML formatter specifically
    async fn test_yaml_formatter(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test YAML validity
        results.push(
            measure_test_execution("unit_yaml_validity", async {
                let formatter = YamlFormatter::new();
                let test_data = self
                    .generate_test_data(&TestInputData::SingleFinding)
                    .await?;
                let output = formatter.format(&test_data)?;

                // Parse YAML to ensure it's valid
                let _parsed: serde_yaml::Value = serde_yaml::from_str(&output.content)?;

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unit_test_runner_creation() {
        let runner = UnitTestRunner::new();
        assert!(!runner.test_cases.is_empty());
    }

    #[tokio::test]
    async fn test_generate_test_data() {
        let runner = UnitTestRunner::new();

        let empty_results = runner
            .generate_test_data(&TestInputData::EmptyResults)
            .await
            .unwrap();
        assert_eq!(empty_results.findings.len(), 0);

        let single_finding = runner
            .generate_test_data(&TestInputData::SingleFinding)
            .await
            .unwrap();
        assert_eq!(single_finding.findings.len(), 1);

        let multiple_severities = runner
            .generate_test_data(&TestInputData::MultipleSeverities)
            .await
            .unwrap();
        assert_eq!(multiple_severities.findings.len(), 5);
    }

    #[tokio::test]
    async fn test_large_dataset_generation() {
        let runner = UnitTestRunner::new();
        let large_dataset = runner
            .generate_test_data(&TestInputData::LargeDataset(1000))
            .await
            .unwrap();
        assert_eq!(large_dataset.findings.len(), 1000);
    }
}
