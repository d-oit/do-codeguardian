//! # HTML Output Formatter
//!
//! This module provides HTML output formatting for CodeGuardian analysis results.
//! HTML format includes built-in XSS prevention and sanitization for secure web display.

use crate::output::formatter::{FormatMetadata, OutputFormatter, OutputResult};
use crate::output::security::{generate_csp_header, sanitize_html};
use crate::types::{AnalysisResults, Finding, Severity};
use crate::utils::formatting::{format_file_location, severity_css_class};
use anyhow::Result;
use std::time::Instant;

/// HTML output formatter
#[derive(Debug, Clone)]
pub struct HtmlFormatter {
    /// Include CSS styling
    pub include_css: bool,
    /// Include JavaScript (disabled by default for security)
    pub include_js: bool,
    /// Include interactive features
    pub interactive: bool,
    /// Dark mode theme
    pub dark_theme: bool,
    /// Sanitize all user content
    pub sanitize_content: bool,
}

impl HtmlFormatter {
    /// Create a new HTML formatter with default settings
    pub fn new() -> Self {
        Self {
            include_css: true,
            include_js: false, // Disabled by default for security
            interactive: false,
            dark_theme: false,
            sanitize_content: true,
        }
    }

    /// Create a minimal HTML formatter
    pub fn minimal() -> Self {
        Self {
            include_css: false,
            include_js: false,
            interactive: false,
            dark_theme: false,
            sanitize_content: true,
        }
    }

    /// Create an interactive HTML formatter (use with caution)
    pub fn interactive() -> Self {
        Self {
            include_css: true,
            include_js: true,
            interactive: true,
            dark_theme: false,
            sanitize_content: true,
        }
    }
}

impl Default for HtmlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for HtmlFormatter {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult> {
        let start_time = Instant::now();

        let mut content = String::new();

        // HTML document start
        content.push_str("<!DOCTYPE html>\n");
        content.push_str("<html lang=\"en\">\n");
        content.push_str("<head>\n");
        content.push_str("    <meta charset=\"UTF-8\">\n");
        content.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        content.push_str(&format!(
            "    <meta http-equiv=\"Content-Security-Policy\" content=\"{}\">\n",
            generate_csp_header()
        ));
        content.push_str("    <title>CodeGuardian Analysis Report</title>\n");

        // Include CSS if requested
        if self.include_css {
            content.push_str(&self.generate_css());
        }

        content.push_str("</head>\n");
        content.push_str("<body>\n");

        // Main content
        content.push_str(&self.generate_header(results)?);
        content.push_str(&self.generate_summary(results)?);
        content.push_str(&self.generate_findings(results)?);
        content.push_str(&self.generate_footer(results)?);

        // Include JavaScript if requested (and if interactive mode is enabled)
        if self.include_js && self.interactive {
            content.push_str(&self.generate_js());
        }

        content.push_str("</body>\n");
        content.push_str("</html>\n");

        // Sanitize content if enabled (but preserve DOCTYPE and basic structure)
        if self.sanitize_content {
            // Only sanitize the body content, not the entire document
            let body_start = content.find("<body>").unwrap_or(0);
            let body_end = content.find("</body>").unwrap_or(content.len());

            if body_start > 0 && body_end > body_start {
                let before_body = &content[..body_start + 6]; // Include <body>
                let body_content = &content[body_start + 6..body_end];
                let after_body = &content[body_end..];

                let sanitized_body = sanitize_html(body_content, None)?;
                content = format!("{}{}{}", before_body, sanitized_body, after_body);
            }
        }

        // Validate the HTML
        self.validate_output(&content)?;

        // Create output result
        let mut output = OutputResult::new(content, "html", results.config_hash.clone())
            .with_generation_time(start_time.elapsed().as_millis() as u64);

        // Add HTML-specific properties
        output = output.with_property(
            "include_css".to_string(),
            serde_json::Value::Bool(self.include_css),
        );

        output = output.with_property(
            "include_js".to_string(),
            serde_json::Value::Bool(self.include_js),
        );

        output = output.with_property(
            "sanitized".to_string(),
            serde_json::Value::Bool(self.sanitize_content),
        );

        output = output.with_property("csp_enabled".to_string(), serde_json::Value::Bool(true));

        Ok(output)
    }

    fn content_type(&self) -> &'static str {
        "text/html"
    }

    fn supports_streaming(&self) -> bool {
        false // HTML needs full document structure
    }

    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            name: "HTML",
            version: "1.0.0",
            supports_compression: false,
            max_recommended_size: Some(5 * 1024 * 1024), // 5MB
        }
    }

    fn file_extension(&self) -> &'static str {
        "html"
    }

    fn validate_output(&self, content: &str) -> Result<()> {
        if content.is_empty() {
            return Err(anyhow::anyhow!("HTML output is empty"));
        }

        // Check for basic HTML structure
        if !content.contains("<!DOCTYPE html>") {
            return Err(anyhow::anyhow!("HTML missing DOCTYPE declaration"));
        }

        if !content.contains("<html") || !content.contains("</html>") {
            return Err(anyhow::anyhow!("HTML missing html tags"));
        }

        // Security validation
        if self.sanitize_content {
            // After sanitization, these should not be present
            let dangerous_patterns = ["<script", "javascript:"];
            for pattern in &dangerous_patterns {
                if content.to_lowercase().contains(pattern) {
                    return Err(anyhow::anyhow!(
                        "HTML contains potentially dangerous pattern: {}",
                        pattern
                    ));
                }
            }

            // Check for event handlers with regex (but allow certain safe patterns like data attributes)
            let event_regex = regex::Regex::new(
                r#"(?i)\s*on(click|load|error|focus|blur|change|submit)\s*=\s*['"][^'"]*['"]"#,
            )?;
            if event_regex.is_match(content) {
                return Err(anyhow::anyhow!(
                    "HTML contains potentially dangerous event handlers"
                ));
            }
        }

        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "include_css": {
                    "type": "boolean",
                    "description": "Include CSS styling",
                    "default": true
                },
                "include_js": {
                    "type": "boolean",
                    "description": "Include JavaScript (security risk)",
                    "default": false
                },
                "interactive": {
                    "type": "boolean",
                    "description": "Enable interactive features",
                    "default": false
                },
                "dark_theme": {
                    "type": "boolean",
                    "description": "Use dark theme",
                    "default": false
                },
                "sanitize_content": {
                    "type": "boolean",
                    "description": "Sanitize all content for XSS prevention",
                    "default": true
                }
            }
        }))
    }
}

impl HtmlFormatter {
    fn generate_css(&self) -> String {
        let theme_vars = if self.dark_theme {
            r#"
    :root {
        --bg-color: #1a1a1a;
        --text-color: #e0e0e0;
        --header-bg: #2d2d2d;
        --border-color: #404040;
        --critical-color: #ff4444;
        --high-color: #ff8800;
        --medium-color: #ffcc00;
        --low-color: #4488ff;
        --info-color: #00aa88;
    }"#
        } else {
            r#"
    :root {
        --bg-color: #ffffff;
        --text-color: #333333;
        --header-bg: #f5f5f5;
        --border-color: #dddddd;
        --critical-color: #dc3545;
        --high-color: #fd7e14;
        --medium-color: #ffc107;
        --low-color: #007bff;
        --info-color: #17a2b8;
    }"#
        };

        format!(
            r#"
    <style>
    {}

    body {{
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        margin: 0;
        padding: 20px;
        background-color: var(--bg-color);
        color: var(--text-color);
        line-height: 1.6;
    }}

    .header {{
        background-color: var(--header-bg);
        padding: 20px;
        border-radius: 8px;
        margin-bottom: 20px;
        border: 1px solid var(--border-color);
    }}

    .summary {{
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 20px;
        margin-bottom: 30px;
    }}

    .summary-card {{
        background-color: var(--header-bg);
        padding: 20px;
        border-radius: 8px;
        border: 1px solid var(--border-color);
    }}

    .finding {{
        background-color: var(--header-bg);
        padding: 20px;
        margin-bottom: 20px;
        border-radius: 8px;
        border-left: 5px solid;
    }}

    .finding.critical {{ border-left-color: var(--critical-color); }}
    .finding.high {{ border-left-color: var(--high-color); }}
    .finding.medium {{ border-left-color: var(--medium-color); }}
    .finding.low {{ border-left-color: var(--low-color); }}
    .finding.info {{ border-left-color: var(--info-color); }}

    .severity {{
        display: inline-block;
        padding: 4px 12px;
        border-radius: 20px;
        font-weight: bold;
        font-size: 0.8em;
        text-transform: uppercase;
    }}

    .severity.critical {{ background-color: var(--critical-color); color: white; }}
    .severity.high {{ background-color: var(--high-color); color: white; }}
    .severity.medium {{ background-color: var(--medium-color); color: black; }}
    .severity.low {{ background-color: var(--low-color); color: white; }}
    .severity.info {{ background-color: var(--info-color); color: white; }}

    .finding-id {{
        font-family: 'Courier New', monospace;
        background-color: var(--border-color);
        padding: 2px 6px;
        border-radius: 4px;
        font-size: 0.9em;
    }}

    .location {{
        font-family: 'Courier New', monospace;
        color: var(--info-color);
    }}

    .no-issues {{
        text-align: center;
        padding: 40px;
        background-color: var(--info-color);
        color: white;
        border-radius: 8px;
        font-size: 1.2em;
    }}

    .footer {{
        margin-top: 40px;
        padding-top: 20px;
        border-top: 1px solid var(--border-color);
        text-align: center;
        color: #666;
        font-size: 0.9em;
    }}

    @media (max-width: 768px) {{
        body {{ padding: 10px; }}
        .summary {{ grid-template-columns: 1fr; }}
    }}
    </style>
"#,
            theme_vars
        )
    }

    fn generate_header(&self, results: &AnalysisResults) -> Result<String> {
        Ok(format!(
            r#"
    <div class="header">
        <h1>🛡️ CodeGuardian Analysis Report</h1>
        <div class="metadata">
            <p><strong>Generated:</strong> {}</p>
            <p><strong>Tool Version:</strong> {}</p>
            <p><strong>Schema Version:</strong> {}</p>
        </div>
    </div>
"#,
            html_escape::encode_text(
                &results
                    .timestamp
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
            ),
            html_escape::encode_text(&results.tool_metadata.version),
            html_escape::encode_text(&results.schema_version)
        ))
    }

    fn generate_summary(&self, results: &AnalysisResults) -> Result<String> {
        let mut summary = String::from(
            r#"
    <div class="summary">
        <div class="summary-card">
            <h3>📊 Overview</h3>
"#,
        );

        summary.push_str(&format!(
            r#"            <p><strong>Files Scanned:</strong> {}</p>
            <p><strong>Total Findings:</strong> {}</p>
            <p><strong>Scan Duration:</strong> {}ms</p>
"#,
            results.summary.total_files_scanned,
            results.summary.total_findings,
            results.summary.scan_duration_ms
        ));

        summary.push_str("        </div>\n");

        // Severity breakdown
        summary.push_str(
            r#"
        <div class="summary-card">
            <h3>🚨 Findings by Severity</h3>
"#,
        );

        if results.findings.is_empty() {
            summary.push_str("            <p class=\"no-issues\">✅ No issues found!</p>\n");
        } else {
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
                            r#"            <p><span class="severity {}">{}</span> {}</p>
"#,
                            severity_css_class(&severity),
                            severity,
                            count
                        ));
                    }
                }
            }
        }

        summary.push_str("        </div>\n    </div>\n");
        Ok(summary)
    }

    fn generate_findings(&self, results: &AnalysisResults) -> Result<String> {
        if results.findings.is_empty() {
            return Ok(String::new());
        }

        let mut findings = String::from("\n    <h2>🔍 Detailed Findings</h2>\n\n");

        for finding in &results.findings {
            findings.push_str(&self.format_finding_html(finding)?);
        }

        Ok(findings)
    }

    fn format_finding_html(&self, finding: &Finding) -> Result<String> {
        let severity_class = severity_css_class(&finding.severity);

        let mut content = format!(
            r#"
    <div class="finding {}" data-severity="{}" data-finding-id="{}">
        <div class="finding-header">
            <h3><span class="severity {}">{}</span> {}</h3>
            <p><strong>ID:</strong> <span class="finding-id">{}</span></p>
            <p><strong>Location:</strong> <span class="location">{}</span></p>
            <p><strong>Analyzer:</strong> {}</p>
        </div>

        <div class="finding-content">
            <p><strong>Message:</strong> {}</p>
"#,
            severity_class,
            finding.severity,
            html_escape::encode_text(&finding.id),
            severity_class,
            finding.severity,
            html_escape::encode_text(&finding.rule),
            html_escape::encode_text(&finding.id),
            html_escape::encode_text(&format_file_location(
                &finding.file,
                finding.line as usize,
                finding.column.map(|c| c as usize)
            )),
            html_escape::encode_text(&finding.analyzer),
            html_escape::encode_text(&finding.message)
        );

        // Add description if available
        if let Some(description) = &finding.description {
            content.push_str(&format!(
                "            <p><strong>Description:</strong> {}</p>\n",
                html_escape::encode_text(description)
            ));
        }

        // Add suggestion if available
        if let Some(suggestion) = &finding.suggestion {
            content.push_str(&format!(
                "            <p><strong>Suggestion:</strong> {}</p>\n",
                html_escape::encode_text(suggestion)
            ));
        }

        // Add metadata if present
        if !finding.metadata.is_empty() {
            content.push_str("            <details>\n                <summary><strong>Additional Information</strong></summary>\n                <ul>\n");
            for (key, value) in &finding.metadata {
                content.push_str(&format!(
                    "                    <li><strong>{}:</strong> {}</li>\n",
                    html_escape::encode_text(key),
                    html_escape::encode_text(&value.to_string())
                ));
            }
            content.push_str("                </ul>\n            </details>\n");
        }

        content.push_str("        </div>\n    </div>\n");
        Ok(content)
    }

    fn generate_footer(&self, results: &AnalysisResults) -> Result<String> {
        Ok(format!(
            r#"
    <div class="footer">
        <p>Report generated by CodeGuardian {} at {}</p>
    </div>
"#,
            html_escape::encode_text(&results.tool_metadata.version),
            html_escape::encode_text(
                &results
                    .timestamp
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
            )
        ))
    }

    fn generate_js(&self) -> String {
        // Minimal, safe JavaScript for interactive features
        // Note: This should be used with caution and proper CSP
        r#"
    <script>
    // Safe, minimal JavaScript for interactive features
    document.addEventListener('DOMContentLoaded', function() {
        // Add click handlers for collapsible sections
        const details = document.querySelectorAll('details');
        details.forEach(detail => {
            detail.addEventListener('toggle', function() {
                if (this.open) {
                    this.setAttribute('data-open', 'true');
                } else {
                    this.removeAttribute('data-open');
                }
            });
        });

        // Add keyboard navigation
        document.addEventListener('keydown', function(e) {
            if (e.key === 'Escape') {
                const openDetails = document.querySelectorAll('details[open]');
                openDetails.forEach(detail => detail.open = false);
            }
        });
    });
    </script>
"#
        .to_string()
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
    fn test_html_formatter_basic() -> Result<(), Box<dyn std::error::Error>> {
        let mut formatter = HtmlFormatter::new();
        formatter.sanitize_content = false; // Disable sanitization for basic test
        let results = create_test_results();

        let output = formatter.format(&results)?;
        assert!(!output.content.is_empty());
        assert_eq!(output.metadata.format, "html");

        // Should contain expected HTML structure
        assert!(output.content.contains("<!DOCTYPE html>"));
        assert!(output.content.contains("<html"));
        assert!(output.content.contains("</html>"));
        assert!(output.content.contains("CodeGuardian Analysis Report"));
    }

    #[test]
    fn test_html_formatter_security() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = HtmlFormatter::new(); // sanitize_content is true by default
        let results = create_test_results();

        let output = formatter.format(&results)?;

        // Should include CSP header
        assert!(output.content.contains("Content-Security-Policy"));

        // Should not contain dangerous patterns after sanitization
        assert!(!output.content.contains("<script"));
        assert!(!output.content.contains("javascript:"));

        // Verify it's properly structured HTML
        assert!(output.content.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_html_formatter_content_type() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = HtmlFormatter::new();
        assert_eq!(formatter.content_type(), "text/html");
    }

    #[test]
    fn test_html_formatter_validation() -> Result<(), Box<dyn std::error::Error>> {
        let formatter = HtmlFormatter::new();

        // Valid HTML should pass
        let valid_html = "<!DOCTYPE html><html><head></head><body></body></html>";
        assert!(formatter.validate_output(valid_html).is_ok());

        // Empty content should fail
        assert!(formatter.validate_output("").is_err());

        // Missing DOCTYPE should fail
        assert!(formatter.validate_output("<html></html>").is_err());
    }
}
