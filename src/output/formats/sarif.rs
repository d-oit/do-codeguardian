//! # SARIF Output Formatter
//!
//! This module provides SARIF (Static Analysis Results Interchange Format) output formatting.
//! SARIF is the industry standard for static analysis tool output.

use crate::output::formatter::{FormatMetadata, OutputFormatter, OutputResult};
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

/// SARIF output formatter
#[derive(Debug, Clone)]
pub struct SarifFormatter {
    /// SARIF schema version
    pub schema_version: String,
    /// Include rule metadata
    pub include_rules: bool,
    /// Include driver information
    pub include_driver_info: bool,
}

impl SarifFormatter {
    /// Create a new SARIF formatter with default settings
    pub fn new() -> Self {
        Self {
            schema_version: "2.1.0".to_string(),
            include_rules: true,
            include_driver_info: true,
        }
    }

    /// Create a minimal SARIF formatter
    pub fn minimal() -> Self {
        Self {
            schema_version: "2.1.0".to_string(),
            include_rules: false,
            include_driver_info: false,
        }
    }
}

impl Default for SarifFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for SarifFormatter {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult> {
        let start_time = Instant::now();

        let sarif = self.create_sarif_document(results)?;
        let content = serde_json::to_string_pretty(&sarif)?;

        // Validate the SARIF
        self.validate_output(&content)?;

        // Create output result
        let mut output = OutputResult::new(content, "sarif", results.config_hash.clone())
            .with_generation_time(start_time.elapsed().as_millis() as u64);

        // Add SARIF-specific properties
        output = output.with_property(
            "sarif_version".to_string(),
            Value::String(self.schema_version.clone()),
        );

        output = output.with_property(
            "rule_count".to_string(),
            Value::Number(self.get_unique_rules(results).len().into()),
        );

        Ok(output)
    }

    fn content_type(&self) -> &'static str {
        "application/sarif+json"
    }

    fn supports_streaming(&self) -> bool {
        false // SARIF needs complete document structure
    }

    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            name: "SARIF",
            version: "2.1.0",
            supports_compression: true,
            max_recommended_size: Some(100 * 1024 * 1024), // 100MB
        }
    }

    fn file_extension(&self) -> &'static str {
        "sarif"
    }

    fn validate_output(&self, content: &str) -> Result<()> {
        // Validate that it's valid JSON
        let sarif: Value = serde_json::from_str(content)?;

        // Validate SARIF structure
        if !sarif.is_object() {
            return Err(anyhow::anyhow!("SARIF must be a JSON object"));
        }

        let obj = sarif
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("SARIF is not an object"))?;

        // Check required fields
        if !obj.contains_key("version") {
            return Err(anyhow::anyhow!("SARIF missing required 'version' field"));
        }

        if !obj.contains_key("runs") {
            return Err(anyhow::anyhow!("SARIF missing required 'runs' field"));
        }

        Ok(())
    }

    fn get_config_schema(&self) -> Option<Value> {
        Some(json!({
            "type": "object",
            "properties": {
                "schema_version": {
                    "type": "string",
                    "description": "SARIF schema version",
                    "default": "2.1.0"
                },
                "include_rules": {
                    "type": "boolean",
                    "description": "Include rule definitions",
                    "default": true
                },
                "include_driver_info": {
                    "type": "boolean",
                    "description": "Include driver information",
                    "default": true
                }
            }
        }))
    }
}

impl SarifFormatter {
    fn create_sarif_document(&self, results: &AnalysisResults) -> Result<Value> {
        let mut sarif = json!({
            "$schema": format!("https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-{}.json", self.schema_version),
            "version": self.schema_version,
            "runs": []
        });

        let run = self.create_run(results)?;
        sarif["runs"]
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("runs is not an array"))?
            .push(run);

        Ok(sarif)
    }

    fn create_run(&self, results: &AnalysisResults) -> Result<Value> {
        let mut run = json!({
            "tool": self.create_tool(results)?,
            "results": []
        });

        // Add rules if requested
        if self.include_rules {
            run["tool"]["driver"]["rules"] = self.create_rules(results)?;
        }

        // Add results
        let sarif_results = self.create_results(results)?;
        run["results"] = Value::Array(sarif_results);

        Ok(run)
    }

    fn create_tool(&self, results: &AnalysisResults) -> Result<Value> {
        let mut driver = json!({
            "name": "codeguardian",
            "version": results.tool_metadata.version,
            "informationUri": "https://github.com/your-org/codeguardian"
        });

        if self.include_driver_info {
            driver["fullName"] = Value::String("CodeGuardian Security Analyzer".to_string());
            driver["shortDescription"] = json!({
                "text": "A comprehensive security analysis tool for code repositories"
            });
        }

        Ok(json!({
            "driver": driver
        }))
    }

    fn create_rules(&self, results: &AnalysisResults) -> Result<Value> {
        let unique_rules = self.get_unique_rules(results);
        let mut rules = Vec::new();

        for (rule_id, findings) in unique_rules {
            let rule = json!({
                "id": rule_id,
                "name": rule_id,
                "shortDescription": {
                    "text": findings[0].rule.clone()
                },
                "fullDescription": {
                    "text": findings[0].description.as_ref().unwrap_or(&findings[0].message).clone()
                },
                "defaultConfiguration": {
                    "level": self.severity_to_sarif_level(&findings[0].severity)
                },
                "properties": {
                    "category": findings[0].category.as_ref().unwrap_or(&"security".to_string()).clone(),
                    "analyzer": findings[0].analyzer.clone()
                }
            });
            rules.push(rule);
        }

        Ok(Value::Array(rules))
    }

    fn create_results(&self, results: &AnalysisResults) -> Result<Vec<Value>> {
        let mut sarif_results = Vec::new();

        for finding in &results.findings {
            let result = json!({
                "ruleId": format!("{}_{}", finding.analyzer, finding.rule),
                "level": self.severity_to_sarif_level(&finding.severity),
                "message": {
                    "text": finding.message.clone()
                },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {
                            "uri": finding.file.to_string_lossy()
                        },
                        "region": {
                            "startLine": finding.line,
                            "startColumn": finding.column.unwrap_or(1)
                        }
                    }
                }],
                "properties": {
                    "findingId": finding.id.clone(),
                    "analyzer": finding.analyzer.clone(),
                    "category": finding.category.as_ref().unwrap_or(&"security".to_string()).clone()
                }
            });
            sarif_results.push(result);
        }

        Ok(sarif_results)
    }

    fn get_unique_rules<'a>(
        &self,
        results: &'a AnalysisResults,
    ) -> HashMap<String, Vec<&'a Finding>> {
        let mut rules = HashMap::new();

        for finding in &results.findings {
            let rule_id = format!("{}_{}", finding.analyzer, finding.rule);
            rules.entry(rule_id).or_insert_with(Vec::new).push(finding);
        }

        rules
    }

    fn severity_to_sarif_level(&self, severity: &Severity) -> &'static str {
        match severity {
            Severity::Critical => "error",
            Severity::High => "error",
            Severity::Medium => "warning",
            Severity::Low => "note",
            Severity::Info => "note",
        }
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
    fn test_sarif_formatter_basic() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = SarifFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results)?;
        assert!(!output.content.is_empty());
        assert_eq!(output.metadata.format, "sarif");

        // Verify it's valid JSON
        let sarif: Value = serde_json::from_str(&output.content)?;
        assert!(sarif.is_object());
    }

    #[test]
    fn test_sarif_structure() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = SarifFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results)?;
        let sarif: Value = serde_json::from_str(&output.content)?;

        // Check required SARIF fields
        assert!(sarif.get("$schema").is_some());
        assert!(sarif.get("version").is_some());
        assert!(sarif.get("runs").is_some());

        let runs = sarif.get("runs")?.as_array()?;
        assert_eq!(runs.len(), 1);

        let run = &runs[0];
        assert!(run.get("tool").is_some());
        assert!(run.get("results").is_some());
    }

    #[test]
    fn test_sarif_content_type() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = SarifFormatter::new();
        assert_eq!(formatter.content_type(), "application/sarif+json");
    }
}
