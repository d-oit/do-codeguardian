//! # Output Validation Module
//!
//! This module provides validation capabilities for output content,
//! ensuring format compliance, schema validation, and content integrity.

use anyhow::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Validation result containing errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation errors (critical issues)
    pub errors: Vec<ValidationError>,
    /// Validation warnings (non-critical issues)
    pub warnings: Vec<ValidationWarning>,
    /// Validation metadata
    pub metadata: ValidationMetadata,
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error code for categorization
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Location of the error (if applicable)
    pub location: Option<String>,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Warning code for categorization
    pub code: String,
    /// Human-readable warning message
    pub message: String,
    /// Location of the warning (if applicable)
    pub location: Option<String>,
}

/// Validation metadata
#[derive(Debug, Clone)]
pub struct ValidationMetadata {
    /// Validator used
    pub validator: String,
    /// Schema version validated against
    pub schema_version: String,
    /// Validation timestamp
    pub validated_at: chrono::DateTime<chrono::Utc>,
    /// Additional properties
    pub properties: HashMap<String, JsonValue>,
}

/// Validate output content based on format
pub fn validate_output(content: &str, format: &str) -> Result<ValidationResult> {
    let mut result = ValidationResult {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        metadata: ValidationMetadata {
            validator: format!("codeguardian-{}-validator", format),
            schema_version: "1.0.0".to_string(),
            validated_at: chrono::Utc::now(),
            properties: HashMap::new(),
        },
    };

    // Basic content validation
    if content.is_empty() {
        result.errors.push(ValidationError {
            code: "EMPTY_CONTENT".to_string(),
            message: "Output content is empty".to_string(),
            location: None,
            suggestion: Some("Ensure the formatter generates content".to_string()),
        });
        result.is_valid = false;
        return Ok(result);
    }

    // Format-specific validation
    match format.to_lowercase().as_str() {
        "json" => validate_json_format(content, &mut result)?,
        "html" => validate_html_format(content, &mut result)?,
        "markdown" => validate_markdown_format(content, &mut result)?,
        "sarif" => validate_sarif_format(content, &mut result)?,
        "yaml" => validate_yaml_format(content, &mut result)?,
        _ => {
            result.warnings.push(ValidationWarning {
                code: "UNKNOWN_FORMAT".to_string(),
                message: format!(
                    "Unknown format '{}', performing basic validation only",
                    format
                ),
                location: None,
            });
        }
    }

    // Content security validation
    validate_content_security(content, format, &mut result)?;

    // Size validation
    validate_content_size(content, &mut result)?;

    // Encoding validation
    validate_encoding(content, &mut result)?;

    Ok(result)
}

/// Validate JSON format
fn validate_json_format(content: &str, result: &mut ValidationResult) -> Result<()> {
    match serde_json::from_str::<JsonValue>(content) {
        Ok(json) => {
            // Validate structure for CodeGuardian results
            if let Some(obj) = json.as_object() {
                validate_json_structure(obj, result)?;
            }
        }
        Err(e) => {
            result.errors.push(ValidationError {
                code: "INVALID_JSON".to_string(),
                message: format!("Invalid JSON format: {}", e),
                location: Some(format!("Line {}", e.line())),
                suggestion: Some("Check JSON syntax and structure".to_string()),
            });
            result.is_valid = false;
        }
    }
    Ok(())
}

/// Validate JSON structure for CodeGuardian results
fn validate_json_structure(
    obj: &serde_json::Map<String, JsonValue>,
    result: &mut ValidationResult,
) -> Result<()> {
    let required_fields = [
        "schema_version",
        "tool_metadata",
        "findings",
        "summary",
        "timestamp",
    ];

    for field in &required_fields {
        if !obj.contains_key(*field) {
            result.warnings.push(ValidationWarning {
                code: "MISSING_FIELD".to_string(),
                message: format!("Missing recommended field: {}", field),
                location: Some("root".to_string()),
            });
        }
    }

    // Validate schema version if present
    if let Some(schema_version) = obj.get("schema_version") {
        if let Some(version_str) = schema_version.as_str() {
            if !version_str.starts_with("1.") {
                result.warnings.push(ValidationWarning {
                    code: "SCHEMA_VERSION".to_string(),
                    message: format!("Unexpected schema version: {}", version_str),
                    location: Some("schema_version".to_string()),
                });
            }
        }
    }

    Ok(())
}

/// Validate HTML format
fn validate_html_format(content: &str, result: &mut ValidationResult) -> Result<()> {
    // Check for basic HTML structure
    if !content.trim_start().starts_with("<!DOCTYPE") && !content.trim_start().starts_with("<html")
    {
        result.warnings.push(ValidationWarning {
            code: "NO_DOCTYPE".to_string(),
            message: "HTML content missing DOCTYPE declaration".to_string(),
            location: Some("document start".to_string()),
        });
    }

    // Check for potential XSS issues
    let xss_patterns = ["<script", "javascript:", "data:"];
    for pattern in &xss_patterns {
        if content.to_lowercase().contains(pattern) {
            result.errors.push(ValidationError {
                code: "POTENTIAL_XSS".to_string(),
                message: format!("Potential XSS pattern detected: {}", pattern),
                location: None,
                suggestion: Some("Use HTML sanitization".to_string()),
            });
            result.is_valid = false;
        }
    }

    // Check for event handlers with regex
    if content.to_lowercase().contains("on") && (content.contains("=") || content.contains("(")) {
        result.warnings.push(ValidationWarning {
            code: "POTENTIAL_EVENT_HANDLER".to_string(),
            message: "HTML content may contain event handlers".to_string(),
            location: None,
        });
    }

    // Check for unclosed tags (basic check)
    let open_tags = content.matches('<').count();
    let close_tags = content.matches('>').count();
    if open_tags != close_tags {
        result.warnings.push(ValidationWarning {
            code: "UNBALANCED_TAGS".to_string(),
            message: "HTML may have unbalanced tags".to_string(),
            location: None,
        });
    }

    Ok(())
}

/// Validate Markdown format
fn validate_markdown_format(content: &str, result: &mut ValidationResult) -> Result<()> {
    // Check for common Markdown issues

    // Check for unescaped HTML in Markdown
    if content.contains('<') && content.contains('>') {
        result.warnings.push(ValidationWarning {
            code: "HTML_IN_MARKDOWN".to_string(),
            message: "Markdown contains HTML tags".to_string(),
            location: None,
        });
    }

    // Check for proper header structure
    let lines: Vec<&str> = content.lines().collect();
    let mut previous_header_level = 0;

    for (line_num, line) in lines.iter().enumerate() {
        if line.starts_with('#') {
            let header_level = line.chars().take_while(|&c| c == '#').count();

            if header_level > previous_header_level + 1 && previous_header_level != 0 {
                result.warnings.push(ValidationWarning {
                    code: "HEADER_SKIP".to_string(),
                    message: "Header level skipped (bad for accessibility)".to_string(),
                    location: Some(format!("Line {}", line_num + 1)),
                });
            }

            previous_header_level = header_level;
        }
    }

    Ok(())
}

/// Validate SARIF format
fn validate_sarif_format(content: &str, result: &mut ValidationResult) -> Result<()> {
    match serde_json::from_str::<JsonValue>(content) {
        Ok(json) => {
            if let Some(obj) = json.as_object() {
                // Check for SARIF-specific fields
                if !obj.contains_key("$schema") {
                    result.warnings.push(ValidationWarning {
                        code: "NO_SARIF_SCHEMA".to_string(),
                        message: "SARIF missing $schema field".to_string(),
                        location: Some("root".to_string()),
                    });
                }

                if !obj.contains_key("version") {
                    result.errors.push(ValidationError {
                        code: "NO_SARIF_VERSION".to_string(),
                        message: "SARIF missing required version field".to_string(),
                        location: Some("root".to_string()),
                        suggestion: Some("Add SARIF version field".to_string()),
                    });
                    result.is_valid = false;
                }

                if !obj.contains_key("runs") {
                    result.errors.push(ValidationError {
                        code: "NO_SARIF_RUNS".to_string(),
                        message: "SARIF missing required runs field".to_string(),
                        location: Some("root".to_string()),
                        suggestion: Some("Add SARIF runs array".to_string()),
                    });
                    result.is_valid = false;
                }
            }
        }
        Err(e) => {
            result.errors.push(ValidationError {
                code: "INVALID_SARIF_JSON".to_string(),
                message: format!("Invalid SARIF JSON: {}", e),
                location: Some(format!("Line {}", e.line())),
                suggestion: Some("Check SARIF JSON syntax".to_string()),
            });
            result.is_valid = false;
        }
    }
    Ok(())
}

/// Validate YAML format
fn validate_yaml_format(content: &str, result: &mut ValidationResult) -> Result<()> {
    match serde_yaml::from_str::<JsonValue>(content) {
        Ok(_) => {
            // YAML is valid
        }
        Err(e) => {
            result.errors.push(ValidationError {
                code: "INVALID_YAML".to_string(),
                message: format!("Invalid YAML format: {}", e),
                location: e.location().map(|loc| format!("Line {}", loc.line())),
                suggestion: Some("Check YAML syntax and indentation".to_string()),
            });
            result.is_valid = false;
        }
    }
    Ok(())
}

/// Validate content security
fn validate_content_security(
    content: &str,
    format: &str,
    result: &mut ValidationResult,
) -> Result<()> {
    // Use the security module for validation
    let security_warnings = crate::output::security::validate_content_security(content, format)?;

    for warning in security_warnings {
        result.warnings.push(ValidationWarning {
            code: "SECURITY_WARNING".to_string(),
            message: warning,
            location: None,
        });
    }

    Ok(())
}

/// Validate content size
fn validate_content_size(content: &str, result: &mut ValidationResult) -> Result<()> {
    let size_bytes = content.len();

    // Warn for large files that might impact performance
    if size_bytes > 10 * 1024 * 1024 {
        // 10MB
        result.warnings.push(ValidationWarning {
            code: "LARGE_CONTENT".to_string(),
            message: format!(
                "Content is large ({} bytes), may impact performance",
                size_bytes
            ),
            location: None,
        });
    }

    // Error for extremely large files
    if size_bytes > 100 * 1024 * 1024 {
        // 100MB
        result.errors.push(ValidationError {
            code: "EXCESSIVE_SIZE".to_string(),
            message: format!("Content exceeds size limit ({} bytes)", size_bytes),
            location: None,
            suggestion: Some("Consider using streaming output or pagination".to_string()),
        });
        result.is_valid = false;
    }

    Ok(())
}

/// Validate encoding
fn validate_encoding(content: &str, result: &mut ValidationResult) -> Result<()> {
    // Check for valid UTF-8 (Rust strings are already UTF-8, but check for issues)
    for (idx, ch) in content.char_indices() {
        if ch == '\u{FFFD}' {
            // Replacement character indicates encoding issues
            result.warnings.push(ValidationWarning {
                code: "ENCODING_ISSUE".to_string(),
                message: "Content contains replacement characters (possible encoding issue)"
                    .to_string(),
                location: Some(format!("Character position {}", idx)),
            });
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_content() -> Result<(), Box<dyn std::error::Error>> {
        let result = validate_output("", "json")?;
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        assert_eq!(result.errors[0].code, "EMPTY_CONTENT");
    }

    #[test]
    fn test_validate_valid_json() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{"schema_version": "1.0.0", "tool_metadata": {}, "findings": [], "summary": {}, "timestamp": "2023-01-01T00:00:00Z"}"#;
        let result = validate_output(json, "json")?;
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_invalid_json() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_json = r#"{"invalid": json}"#;
        let result = validate_output(invalid_json, "json")?;
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_html_xss() -> Result<(), Box<dyn std::error::Error>> {
        let html = r#"<html><script>alert('xss')</script></html>"#;
        let result = validate_output(html, "html")?;
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.code == "POTENTIAL_XSS"));
    }
}
