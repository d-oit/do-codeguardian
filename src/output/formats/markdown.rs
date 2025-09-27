//! # Markdown Output Formatter
//!
//! This module provides Markdown output formatting for CodeGuardian analysis results.
//! Markdown format is optimized for human readability and documentation.

use crate::output::formatter::{FormatMetadata, OutputFormatter, OutputResult};
use crate::types::{AnalysisResults, Finding, Severity};
use crate::utils::formatting::{
    findings_header, format_file_location, format_finding_id, no_issues_message, severity_emoji,
};
use anyhow::Result;
use std::time::Instant;

/// Markdown output formatter
#[derive(Debug, Clone)]
pub struct MarkdownFormatter {
    /// Include table of contents
    pub include_toc: bool,
    /// Include detailed findings
    pub include_details: bool,
    /// Include summary statistics
    pub include_summary: bool,
    /// Group findings by file
    pub group_by_file: bool,
    /// Maximum description length before truncation
    pub max_description_length: usize,
}

impl MarkdownFormatter {
    /// Create a new Markdown formatter with default settings
    pub fn new() -> Self {
        Self {
            include_toc: true,
            include_details: true,
            include_summary: true,
            group_by_file: false,
            max_description_length: 500,
        }
    }

    /// Create a minimal Markdown formatter
    pub fn minimal() -> Self {
        Self {
            include_toc: false,
            include_details: true,
            include_summary: true,
            group_by_file: false,
            max_description_length: 200,
        }
    }

    /// Create a detailed Markdown formatter
    pub fn detailed() -> Self {
        Self {
            include_toc: true,
            include_details: true,
            include_summary: true,
            group_by_file: true,
            max_description_length: 1000,
        }
    }
}

impl Default for MarkdownFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for MarkdownFormatter {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult> {
        let start_time = Instant::now();

        let mut content = String::new();

        // Add title
        content.push_str("# CodeGuardian Analysis Report\n\n");

        // Add metadata
        content.push_str(&format!(
            "**Generated:** {}\n",
            results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        content.push_str(&format!(
            "**Tool Version:** {}\n",
            results.tool_metadata.version
        ));
        content.push_str(&format!(
            "**Schema Version:** {}\n\n",
            results.schema_version
        ));

        // Add table of contents if requested
        if self.include_toc {
            content.push_str(&self.generate_toc(results));
        }

        // Add summary if requested
        if self.include_summary {
            content.push_str(&self.generate_summary(results));
        }

        // Add findings if requested
        if self.include_details {
            content.push_str(&self.generate_findings(results)?);
        }

        // Add footer
        content.push_str(&self.generate_footer(results));

        // Validate the markdown
        self.validate_output(&content)?;

        // Create output result
        let mut output = OutputResult::new(content, "markdown", results.config_hash.clone())
            .with_generation_time(start_time.elapsed().as_millis() as u64);

        // Add Markdown-specific properties
        output = output.with_property(
            "include_toc".to_string(),
            serde_json::Value::Bool(self.include_toc),
        );

        output = output.with_property(
            "group_by_file".to_string(),
            serde_json::Value::Bool(self.group_by_file),
        );

        output = output.with_property(
            "finding_count".to_string(),
            serde_json::Value::Number(results.findings.len().into()),
        );

        Ok(output)
    }

    fn content_type(&self) -> &'static str {
        "text/markdown"
    }

    fn supports_streaming(&self) -> bool {
        false // Markdown needs full content to generate TOC and summary
    }

    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            name: "Markdown",
            version: "1.0.0",
            supports_compression: false,
            max_recommended_size: Some(10 * 1024 * 1024), // 10MB
        }
    }

    fn file_extension(&self) -> &'static str {
        "md"
    }

    fn validate_output(&self, content: &str) -> Result<()> {
        if content.is_empty() {
            return Err(anyhow::anyhow!("Markdown output is empty"));
        }

        // Check for balanced markdown elements
        let code_blocks = content.matches("```").count();
        if code_blocks % 2 != 0 {
            return Err(anyhow::anyhow!("Unbalanced code blocks in Markdown"));
        }

        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "include_toc": {
                    "type": "boolean",
                    "description": "Include table of contents",
                    "default": true
                },
                "include_details": {
                    "type": "boolean",
                    "description": "Include detailed findings",
                    "default": true
                },
                "include_summary": {
                    "type": "boolean",
                    "description": "Include summary statistics",
                    "default": true
                },
                "group_by_file": {
                    "type": "boolean",
                    "description": "Group findings by file",
                    "default": false
                },
                "max_description_length": {
                    "type": "integer",
                    "description": "Maximum description length",
                    "default": 500,
                    "minimum": 50
                }
            }
        }))
    }
}

impl MarkdownFormatter {
    fn generate_toc(&self, results: &AnalysisResults) -> String {
        let mut toc = String::from("## Table of Contents\n\n");

        if self.include_summary {
            toc.push_str("- [Summary](#summary)\n");
        }

        if self.include_details && !results.findings.is_empty() {
            toc.push_str("- [Findings](#findings)\n");

            if self.group_by_file {
                // Group by file for TOC
                let mut files: Vec<_> = results.findings.iter().map(|f| &f.file).collect();
                files.sort();
                files.dedup();

                for file in files {
                    let file_anchor = file
                        .to_string_lossy()
                        .replace(['/', '.'], "-")
                        .to_lowercase();
                    toc.push_str(&format!("  - [{}](#{})\n", file.display(), file_anchor));
                }
            }
        }

        toc.push('\n');
        toc
    }

    fn generate_summary(&self, results: &AnalysisResults) -> String {
        let mut summary = String::from("## 📊 Analysis Summary\n\n");

        // Overall statistics
        summary.push_str(&format!(
            "- **Total Files Scanned:** {}\n",
            results.summary.total_files_scanned
        ));
        summary.push_str(&format!(
            "- **Total Findings:** {}\n",
            results.summary.total_findings
        ));
        summary.push_str(&format!(
            "- **Scan Duration:** {}ms\n\n",
            results.summary.scan_duration_ms
        ));

        if results.findings.is_empty() {
            summary.push_str(&no_issues_message());
            return summary;
        }

        // Severity breakdown
        summary.push_str("### Findings by Severity\n\n");
        for severity in [
            Severity::Critical,
            Severity::High,
            Severity::Medium,
            Severity::Low,
            Severity::Info,
        ] {
            if let Some(count) = results.summary.findings_by_severity.get(&severity) {
                if *count > 0 {
                    summary.push_str(&format!(
                        "- {} **{}:** {}\n",
                        severity_emoji(&severity),
                        severity,
                        count
                    ));
                }
            }
        }
        summary.push('\n');

        // Analyzer breakdown
        if !results.summary.findings_by_analyzer.is_empty() {
            summary.push_str("### Findings by Analyzer\n\n");
            let mut analyzers: Vec<_> = results.summary.findings_by_analyzer.iter().collect();
            analyzers.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

            for (analyzer, count) in analyzers {
                summary.push_str(&format!("- **{}:** {}\n", analyzer, count));
            }
            summary.push('\n');
        }

        summary
    }

    fn generate_findings(&self, results: &AnalysisResults) -> Result<String> {
        if results.findings.is_empty() {
            return Ok(String::new());
        }

        let mut findings = findings_header();

        if self.group_by_file {
            findings.push_str(&self.generate_findings_by_file(results)?);
        } else {
            findings.push_str(&self.generate_findings_list(results)?);
        }

        Ok(findings)
    }

    fn generate_findings_by_file(&self, results: &AnalysisResults) -> Result<String> {
        let mut content = String::new();

        // Group findings by file
        let mut file_findings: std::collections::HashMap<_, Vec<_>> =
            std::collections::HashMap::new();
        for finding in &results.findings {
            file_findings
                .entry(&finding.file)
                .or_default()
                .push(finding);
        }

        // Sort files
        let mut files: Vec<_> = file_findings.keys().collect();
        files.sort();

        for file in files {
            let file_anchor = file
                .to_string_lossy()
                .replace(['/', '.'], "-")
                .to_lowercase();

            content.push_str(&format!("### {} {{#{}}}\n\n", file.display(), file_anchor));

            let findings = file_findings.get(file).ok_or_else(|| {
                anyhow::anyhow!("No findings found for file: {}", file.display())
            })?;
            for finding in findings {
                content.push_str(&self.format_finding(finding)?);
                content.push('\n');
            }
        }

        Ok(content)
    }

    fn generate_findings_list(&self, results: &AnalysisResults) -> Result<String> {
        let mut content = String::new();

        for finding in &results.findings {
            content.push_str(&self.format_finding(finding)?);
            content.push('\n');
        }

        Ok(content)
    }

    fn format_finding(&self, finding: &Finding) -> Result<String> {
        let mut content = String::new();

        // Finding header with severity emoji
        content.push_str(&format!(
            "#### {} {} - {}\n\n",
            severity_emoji(&finding.severity),
            finding.severity,
            finding.rule
        ));

        // Basic information
        content.push_str(&format!("- **ID:** {}\n", format_finding_id(&finding.id)));
        content.push_str(&format!(
            "- **Location:** {}\n",
            format_file_location(
                &finding.file,
                finding.line as usize,
                finding.column.map(|c| c as usize)
            )
        ));
        content.push_str(&format!("- **Analyzer:** {}\n", finding.analyzer));

        // Message
        content.push_str(&format!("\n**Message:** {}\n\n", finding.message));

        // Description if available
        if let Some(description) = &finding.description {
            let truncated_desc = if description.len() > self.max_description_length {
                format!("{}...", &description[..self.max_description_length])
            } else {
                description.clone()
            };
            content.push_str(&format!("**Description:** {}\n\n", truncated_desc));
        }

        // Suggestion if available
        if let Some(suggestion) = &finding.suggestion {
            content.push_str(&format!("**Suggestion:** {}\n\n", suggestion));
        }

        // Metadata if present
        if !finding.metadata.is_empty() {
            content.push_str("**Additional Information:**\n\n");
            for (key, value) in &finding.metadata {
                content.push_str(&format!("- **{}:** {}\n", key, value));
            }
            content.push('\n');
        }

        content.push_str("---\n\n");
        Ok(content)
    }

    fn generate_footer(&self, results: &AnalysisResults) -> String {
        format!(
            "\n---\n\n*Report generated by CodeGuardian {} at {}*\n",
            results.tool_metadata.version,
            results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
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
        )
        .with_description("Test description".to_string())
        .with_suggestion("Test suggestion".to_string());

        results.add_finding(finding);
        results
    }

    #[test]
    fn test_markdown_formatter_basic() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = MarkdownFormatter::new();
        let results = create_test_results();

        let output = formatter.format(&results)?;
        assert!(!output.content.is_empty());
        assert_eq!(output.metadata.format, "markdown");

        // Should contain expected sections
        assert!(output.content.contains("# CodeGuardian Analysis Report"));
        assert!(output.content.contains("## Table of Contents"));
        assert!(output.content.contains("## 📊 Analysis Summary"));
        Ok(())
    }

    #[test]
    fn test_markdown_formatter_minimal() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = MarkdownFormatter::minimal();
        let results = create_test_results();

        let output = formatter.format(&results)?;

        // Minimal format should not contain TOC
        assert!(!output.content.contains("## Table of Contents"));
        Ok(())
    }

    #[test]
    fn test_markdown_formatter_empty_results() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = MarkdownFormatter::new();
        let results = AnalysisResults::new("test_config".to_string());

        let output = formatter.format(&results)?;
        assert!(output.content.contains("No Issues Found"));
        Ok(())
    }

    #[test]
    fn test_markdown_formatter_content_type() {
        let formatter = MarkdownFormatter::new();
        assert_eq!(formatter.content_type(), "text/markdown");
    }

    #[test]
    fn test_markdown_formatter_validation() {
        let formatter = MarkdownFormatter::new();

        // Valid markdown should pass
        assert!(formatter
            .validate_output("# Valid Markdown\n\nContent here")
            .is_ok());

        // Empty content should fail
        assert!(formatter.validate_output("").is_err());

        // Unbalanced code blocks should fail
        assert!(formatter
            .validate_output("```\nCode block without end")
            .is_err());
    }
}
