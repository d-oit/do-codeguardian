//! # YAML Output Formatter
//!
//! This module provides YAML output formatting for CodeGuardian analysis results.
//! YAML format provides human-readable structured data output.

use crate::output::formatter::{FormatMetadata, OutputFormatter, OutputResult};
use crate::types::AnalysisResults;
use anyhow::Result;
use std::time::Instant;

/// YAML output formatter
#[derive(Debug, Clone)]
pub struct YamlFormatter {
    /// Include comments in YAML
    pub include_comments: bool,
    /// Use flow style for compact output
    pub use_flow_style: bool,
}

impl YamlFormatter {
    /// Create a new YAML formatter with default settings
    pub fn new() -> Self {
        Self {
            include_comments: true,
            use_flow_style: false,
        }
    }

    /// Create a compact YAML formatter
    pub fn compact() -> Self {
        Self {
            include_comments: false,
            use_flow_style: true,
        }
    }
}

impl Default for YamlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for YamlFormatter {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult> {
        let start_time = Instant::now();

        let mut content = String::new();

        // Add header comment if requested
        if self.include_comments {
            content.push_str("# CodeGuardian Analysis Report\n");
            content.push_str(&format!(
                "# Generated: {}\n",
                results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            content.push_str(&format!(
                "# Tool Version: {}\n",
                results.tool_metadata.version
            ));
            content.push_str("# Format: YAML\n\n");
        }

        // Convert to YAML
        let yaml_content = serde_yaml::to_string(results)?;
        content.push_str(&yaml_content);

        // Validate the YAML
        self.validate_output(&content)?;

        // Create output result
        let mut output = OutputResult::new(content, "yaml", results.config_hash.clone())
            .with_generation_time(start_time.elapsed().as_millis() as u64);

        // Add YAML-specific properties
        output = output.with_property(
            "include_comments".to_string(),
            serde_json::Value::Bool(self.include_comments),
        );

        output = output.with_property(
            "use_flow_style".to_string(),
            serde_json::Value::Bool(self.use_flow_style),
        );

        Ok(output)
    }

    fn content_type(&self) -> &'static str {
        "application/x-yaml"
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            name: "YAML",
            version: "1.0.0",
            supports_compression: true,
            max_recommended_size: Some(50 * 1024 * 1024), // 50MB
        }
    }

    fn file_extension(&self) -> &'static str {
        "yaml"
    }

    fn validate_output(&self, content: &str) -> Result<()> {
        // Validate that it's valid YAML
        let _: serde_yaml::Value = serde_yaml::from_str(content)?;

        if content.is_empty() {
            return Err(anyhow::anyhow!("YAML output is empty"));
        }

        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "include_comments": {
                    "type": "boolean",
                    "description": "Include comments in YAML output",
                    "default": true
                },
                "use_flow_style": {
                    "type": "boolean",
                    "description": "Use flow style for compact output",
                    "default": false
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
    fn test_yaml_formatter_basic() {
        let formatter = YamlFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results).unwrap();
        assert!(!output.content.is_empty());
        assert_eq!(output.metadata.format, "yaml");

        // Verify it's valid YAML
        let _: serde_yaml::Value = serde_yaml::from_str(&output.content).unwrap();
    }

    #[test]
    fn test_yaml_formatter_with_comments() {
        let formatter = YamlFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results).unwrap();

        // Should contain comments
        assert!(output.content.contains("# CodeGuardian Analysis Report"));
        assert!(output.content.contains("# Generated:"));
    }

    #[test]
    fn test_yaml_formatter_compact() {
        let formatter = YamlFormatter::compact();
        let results = create_test_results();

        let output = formatter.format(&results).unwrap();

        // Should not contain comments
        assert!(!output.content.contains("# CodeGuardian Analysis Report"));
    }

    #[test]
    fn test_yaml_content_type() {
        let formatter = YamlFormatter::new();
        assert_eq!(formatter.content_type(), "application/x-yaml");
    }
}
