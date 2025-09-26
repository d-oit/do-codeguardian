//! # JSON Output Formatter
//!
//! This module provides JSON output formatting for CodeGuardian analysis results.
//! JSON is the "source of truth" format with complete data preservation.

use crate::output::formatter::{FormatMetadata, OutputFormatter, OutputResult};
use crate::performance::GlobalMemoryPools;
use crate::types::AnalysisResults;
use anyhow::Result;
use std::time::Instant;

/// JSON output formatter
#[derive(Debug, Clone)]
pub struct JsonFormatter {
    /// Optional memory manager for efficient buffer reuse
    pub memory_manager: Option<std::sync::Arc<GlobalMemoryPools>>,
    /// Pretty print the JSON output
    pub pretty: bool,
    /// Include metadata in output
    pub include_metadata: bool,
}

impl JsonFormatter {
    /// Create a new JSON formatter with default settings
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self {
            memory_manager: None,
            pretty: true,
            include_metadata: true,
        }
    }
}

impl JsonFormatter {
    /// Create a new JSON formatter with memory manager
    pub fn with_memory_manager(memory_manager: std::sync::Arc<GlobalMemoryPools>) -> Self {
        Self {
            memory_manager: Some(memory_manager),
            pretty: true,
            include_metadata: true,
        }
    }

    /// Create a compact JSON formatter
    pub fn compact() -> Self {
        Self {
            memory_manager: None,
            pretty: false,
            include_metadata: true,
        }
    }

    /// Create a minimal JSON formatter without metadata
    pub fn minimal() -> Self {
        Self {
            memory_manager: None,
            pretty: false,
            include_metadata: false,
        }
    }
}

impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult> {
        let start_time = Instant::now();

        // Create the JSON content
        let content = if self.pretty {
            serde_json::to_string_pretty(results)?
        } else {
            serde_json::to_string(results)?
        };

        // Validate the JSON is valid
        self.validate_output(&content)?;

        // Create output result
        let mut output = OutputResult::new(content, "json", results.config_hash.clone())
            .with_generation_time(start_time.elapsed().as_millis() as u64);

        // Add JSON-specific properties
        output = output.with_property(
            "pretty_printed".to_string(),
            serde_json::Value::Bool(self.pretty),
        );

        output = output.with_property(
            "include_metadata".to_string(),
            serde_json::Value::Bool(self.include_metadata),
        );

        output = output.with_property(
            "total_findings".to_string(),
            serde_json::Value::Number(results.findings.len().into()),
        );

        output = output.with_property(
            "schema_version".to_string(),
            serde_json::Value::String(results.schema_version.clone()),
        );

        Ok(output)
    }

    fn content_type(&self) -> &'static str {
        "application/json"
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            name: "JSON",
            version: "1.0.0",
            supports_compression: true,
            max_recommended_size: Some(100 * 1024 * 1024), // 100MB
        }
    }

    fn file_extension(&self) -> &'static str {
        "json"
    }

    fn validate_output(&self, content: &str) -> Result<()> {
        // Validate that the content is valid JSON
        let _: serde_json::Value = serde_json::from_str(content)?;

        // Additional validation
        if content.is_empty() {
            return Err(anyhow::anyhow!("JSON output is empty"));
        }

        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "pretty": {
                    "type": "boolean",
                    "description": "Enable pretty printing",
                    "default": true
                },
                "include_metadata": {
                    "type": "boolean",
                    "description": "Include metadata in output",
                    "default": true
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnalysisResults, Finding, Severity};
    use std::path::PathBuf;

    fn create_test_results() -> AnalysisResults {
        let mut results = AnalysisResults::new("test_config".to_string());

        let finding = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test finding message".to_string(),
        );

        results.add_finding(finding);
        results
    }

    #[test]
    fn test_json_formatter_basic() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = JsonFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results)?;
        assert!(!output.content.is_empty());
        assert_eq!(output.metadata.format, "json");

        // Verify it's valid JSON
        let _: serde_json::Value = serde_json::from_str(&output.content)?;
    }

    #[test]
    fn test_json_formatter_compact() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = JsonFormatter::compact();
        let results = create_test_results();

        let output = formatter.format(&results)?;

        // Compact format should not contain newlines
        assert!(!output.content.contains('\n'));
    }

    #[test]
    fn test_json_formatter_content_type() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = JsonFormatter::new();
        assert_eq!(formatter.content_type(), "application/json");
    }

    #[test]
    fn test_json_formatter_supports_streaming() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = JsonFormatter::new();
        assert!(formatter.supports_streaming());
    }

    #[test]
    fn test_json_formatter_validation() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = JsonFormatter::new();

        // Valid JSON should pass
        assert!(formatter.validate_output(r#"{"valid": "json"}"#).is_ok());

        // Invalid JSON should fail
        assert!(formatter.validate_output(r#"{"invalid": json}"#).is_err());

        // Empty content should fail
        assert!(formatter.validate_output("").is_err());
    }
}
