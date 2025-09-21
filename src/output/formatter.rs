//! # Output Formatter Trait
//!
//! This module defines the unified output formatting interface for CodeGuardian.
//! All output formatters must implement the OutputFormatter trait to ensure
//! consistent behavior and metadata handling across formats.

use crate::types::AnalysisResults;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified output formatter trait
///
/// All output formatters must implement this trait to provide consistent
/// formatting capabilities with proper error handling and metadata inclusion.
pub trait OutputFormatter: Send + Sync {
    /// Format analysis results into the target format
    ///
    /// # Arguments
    /// * `results` - The analysis results to format
    ///
    /// # Returns
    /// Returns an `OutputResult` containing the formatted content and metadata
    ///
    /// # Errors
    /// Returns an error if formatting fails due to invalid data or I/O issues
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult>;

    /// Get the MIME content type for this format
    fn content_type(&self) -> &'static str;

    /// Check if this formatter supports streaming output
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Get format-specific metadata
    fn metadata(&self) -> FormatMetadata;

    /// Get the file extension for this format
    fn file_extension(&self) -> &'static str;

    /// Validate the output before returning
    fn validate_output(&self, content: &str) -> Result<()> {
        if content.is_empty() {
            return Err(anyhow::anyhow!("Output content is empty"));
        }
        Ok(())
    }

    /// Get format-specific configuration options
    fn get_config_schema(&self) -> Option<serde_json::Value> {
        None
    }

    /// Validate input before formatting (security check)
    ///
    /// # Arguments
    /// * `results` - The analysis results to validate
    ///
    /// # Returns
    /// `Ok(())` if validation passes, `Err` if validation fails
    ///
    /// # Security
    /// This method should check for malicious content, oversized inputs,
    /// and other security concerns before processing.
    fn validate_input(&self, results: &AnalysisResults) -> Result<()> {
        let security_config = self.security_config();

        if !security_config.validate_inputs {
            return Ok(());
        }

        // Check findings count limit (using max_files as max findings for now)
        if results.findings.len() > security_config.max_files {
            return Err(anyhow::anyhow!(
                "Too many findings in results (max: {}, found: {})",
                security_config.max_files,
                results.findings.len()
            ));
        }

        // Check total content size (prevent memory exhaustion)
        let total_size: usize = results
            .findings
            .iter()
            .map(|finding| {
                finding.file.as_os_str().len()
                    + finding.message.len()
                    + finding.description.as_ref().map(|d| d.len()).unwrap_or(0)
                    + finding.suggestion.as_ref().map(|s| s.len()).unwrap_or(0)
            })
            .sum();

        if total_size > security_config.max_content_size {
            return Err(anyhow::anyhow!(
                "Total content size too large (max: {} bytes, found: {} bytes)",
                security_config.max_content_size,
                total_size
            ));
        }

        // Validate file paths for directory traversal attacks
        for finding in &results.findings {
            let path_str = finding.file.to_string_lossy();
            if path_str.contains("..") || path_str.starts_with('/') {
                return Err(anyhow::anyhow!(
                    "Potentially malicious file path detected: {}",
                    path_str
                ));
            }
        }

        Ok(())
    }

    /// Get schema version for this formatter's output format
    fn schema_version(&self) -> &'static str {
        "1.0.0"
    }

    /// Get security configuration for this formatter
    fn security_config(&self) -> SecurityConfig {
        SecurityConfig::default()
    }
}

/// Output result containing formatted content and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputResult {
    /// The formatted content
    pub content: String,
    /// Metadata about the output generation
    pub metadata: OutputMetadata,
    /// Format-specific properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Metadata about the output generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetadata {
    /// Schema version for the output format
    pub schema_version: String,
    /// Timestamp when the output was generated
    pub generated_at: DateTime<Utc>,
    /// Hash of the configuration used
    pub config_hash: String,
    /// Size of the output in bytes
    pub content_size_bytes: usize,
    /// Time taken to generate the output in milliseconds
    pub generation_time_ms: u64,
    /// Format used for the output
    pub format: String,
    /// Tool metadata
    pub tool_metadata: ToolMetadata,
    /// Validation status
    pub validation_status: ValidationStatus,
}

/// Tool metadata for output tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Tool name
    pub name: String,
    /// Tool version
    pub version: String,
    /// Build information
    pub build_info: Option<String>,
}

/// Validation status for output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatus {
    /// Whether the output passed validation
    pub is_valid: bool,
    /// Validation errors if any
    pub errors: Vec<String>,
    /// Validation warnings if any
    pub warnings: Vec<String>,
}

/// Format-specific metadata
#[derive(Debug, Clone)]
pub struct FormatMetadata {
    /// Name of the format
    pub name: &'static str,
    /// Version of the format specification
    pub version: &'static str,
    /// Whether the format supports compression
    pub supports_compression: bool,
    /// Maximum recommended size for output in bytes
    pub max_recommended_size: Option<u64>,
}

/// Security configuration for output formatters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable HTML sanitization (for HTML/Markdown formats)
    pub sanitize_html: bool,
    /// Maximum content size in bytes (prevents memory exhaustion)
    pub max_content_size: usize,
    /// Maximum number of files to process
    pub max_files: usize,
    /// Enable Content Security Policy headers (for web output)
    pub enable_csp: bool,
    /// Allow external resources (links, images)
    pub allow_external_resources: bool,
    /// Validate all user inputs
    pub validate_inputs: bool,
    /// Enable output compression
    pub enable_compression: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sanitize_html: true,
            max_content_size: 100_000_000, // 100MB
            max_files: 10_000,
            enable_csp: true,
            allow_external_resources: false,
            validate_inputs: true,
            enable_compression: true,
        }
    }
}

/// Base configuration for all formatters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Security configuration
    pub security: SecurityConfig,
    /// Whether to include metadata in output
    pub include_metadata: bool,
    /// Whether to enable pretty printing
    pub pretty_print: bool,
    /// Custom schema version override
    pub schema_version: Option<String>,
    /// Custom properties to include in output
    pub custom_properties: HashMap<String, serde_json::Value>,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            include_metadata: true,
            pretty_print: true,
            schema_version: None,
            custom_properties: HashMap::new(),
        }
    }
}

/// Base formatter implementation providing common functionality
///
/// This struct provides default implementations for common formatter operations
/// and can be used as a foundation for specific format implementations.
#[derive(Debug, Clone)]
pub struct BaseFormatter {
    /// Configuration for this formatter
    pub config: FormatterConfig,
    /// Format metadata
    pub metadata: FormatMetadata,
}

impl BaseFormatter {
    /// Create a new base formatter with the given metadata
    pub fn new(metadata: FormatMetadata) -> Self {
        Self {
            config: FormatterConfig::default(),
            metadata,
        }
    }

    /// Create a new base formatter with custom configuration
    pub fn with_config(metadata: FormatMetadata, config: FormatterConfig) -> Self {
        Self { config, metadata }
    }

    /// Generate standard output metadata
    pub fn create_output_metadata(
        &self,
        content: &str,
        format: &str,
        generation_time_ms: u64,
    ) -> OutputMetadata {
        OutputMetadata {
            schema_version: self
                .config
                .schema_version
                .clone()
                .unwrap_or_else(|| "1.0.0".to_string()),
            generated_at: Utc::now(),
            config_hash: self.calculate_config_hash(),
            content_size_bytes: content.len(),
            generation_time_ms,
            format: format.to_string(),
            tool_metadata: ToolMetadata {
                name: "CodeGuardian".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                build_info: Some(format!(
                    "{}+{}",
                    env!("CARGO_PKG_VERSION"),
                    option_env!("BUILD_ID").unwrap_or("dev")
                )),
            },
            validation_status: ValidationStatus::default(),
        }
    }

    /// Calculate a hash of the current configuration
    fn calculate_config_hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        // Hash the serialized config for a stable hash
        if let Ok(config_json) = serde_json::to_string(&self.config) {
            config_json.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }

    /// Perform common input validation
    pub fn validate_common_input(&self, results: &AnalysisResults) -> Result<()> {
        let security_config = &self.config.security;

        if !security_config.validate_inputs {
            return Ok(());
        }

        // Check findings count limit
        if results.findings.len() > security_config.max_files {
            return Err(anyhow::anyhow!(
                "Too many findings in results (max: {}, found: {})",
                security_config.max_files,
                results.findings.len()
            ));
        }

        // Check total content size
        let total_size: usize = results
            .findings
            .iter()
            .map(|finding| {
                finding.file.as_os_str().len()
                    + finding.message.len()
                    + finding.description.as_ref().map(|d| d.len()).unwrap_or(0)
                    + finding.suggestion.as_ref().map(|s| s.len()).unwrap_or(0)
            })
            .sum();

        if total_size > security_config.max_content_size {
            return Err(anyhow::anyhow!(
                "Total content size too large (max: {} bytes, found: {} bytes)",
                security_config.max_content_size,
                total_size
            ));
        }

        // Validate file paths for directory traversal attacks
        for finding in &results.findings {
            let path_str = finding.file.to_string_lossy();
            if path_str.contains("..") || path_str.starts_with('/') {
                return Err(anyhow::anyhow!(
                    "Potentially malicious file path detected: {}",
                    path_str
                ));
            }
        }

        Ok(())
    }
}

impl OutputResult {
    /// Create a new output result
    pub fn new(content: String, format: &str, config_hash: String) -> Self {
        let content_size = content.len();

        Self {
            metadata: OutputMetadata {
                schema_version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                config_hash,
                content_size_bytes: content_size,
                generation_time_ms: 0, // Set by caller
                format: format.to_string(),
                tool_metadata: ToolMetadata {
                    name: "codeguardian".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    build_info: option_env!("BUILD_INFO").map(|s| s.to_string()),
                },
                validation_status: ValidationStatus {
                    is_valid: true,
                    errors: Vec::new(),
                    warnings: Vec::new(),
                },
            },
            content,
            properties: HashMap::new(),
        }
    }

    /// Set the generation time
    pub fn with_generation_time(mut self, time_ms: u64) -> Self {
        self.metadata.generation_time_ms = time_ms;
        self
    }

    /// Add a property to the output result
    pub fn with_property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }

    /// Set validation status
    pub fn with_validation_status(mut self, status: ValidationStatus) -> Self {
        self.metadata.validation_status = status;
        self
    }

    /// Check if the output is valid
    pub fn is_valid(&self) -> bool {
        self.metadata.validation_status.is_valid
    }

    /// Get validation errors
    pub fn validation_errors(&self) -> &[String] {
        &self.metadata.validation_status.errors
    }

    /// Get validation warnings
    pub fn validation_warnings(&self) -> &[String] {
        &self.metadata.validation_status.warnings
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
    fn test_base_formatter_metadata() {
        let metadata = FormatMetadata {
            name: "test",
            version: "1.0.0",
            supports_compression: false,
            max_recommended_size: None,
        };
        let formatter = BaseFormatter::new(metadata);

        let output_metadata = formatter.create_output_metadata("test content", "test", 100);
        assert_eq!(output_metadata.schema_version, "1.0.0");
        assert_eq!(output_metadata.format, "test");
        assert_eq!(output_metadata.generation_time_ms, 100);
    }

    #[test]
    fn test_base_formatter_validation() {
        let metadata = FormatMetadata {
            name: "test",
            version: "1.0.0",
            supports_compression: false,
            max_recommended_size: None,
        };
        let formatter = BaseFormatter::new(metadata);
        let results = create_test_results();

        let validation_result = formatter.validate_common_input(&results);
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_base_formatter_config_hash() {
        let metadata = FormatMetadata {
            name: "test",
            version: "1.0.0",
            supports_compression: false,
            max_recommended_size: None,
        };
        let formatter = BaseFormatter::new(metadata);

        let hash1 = formatter.calculate_config_hash();
        let hash2 = formatter.calculate_config_hash();
        assert_eq!(hash1, hash2); // Same config should produce same hash
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_security_config_defaults() {
        let security_config = SecurityConfig::default();
        assert!(security_config.sanitize_html);
        assert_eq!(security_config.max_content_size, 100_000_000);
        assert_eq!(security_config.max_files, 10_000);
        assert!(security_config.validate_inputs);
    }

    #[test]
    fn test_formatter_config_defaults() {
        let config = FormatterConfig::default();
        assert!(config.include_metadata);
        assert!(config.pretty_print);
        assert!(config.security.validate_inputs);
    }
}

/// Base formatter with common functionality for all output formatters
///
/// Provides shared implementations for metadata formatting, finding formatting,
/// and summary formatting that can be used by specific format implementations.
///
/// # Examples
///
/// ```rust
/// use codeguardian::output::{BaseFormatter, FormatterConfig};
/// use codeguardian::types::AnalysisResults;
///
impl Default for ToolMetadata {
    fn default() -> Self {
        Self {
            name: "codeguardian".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_info: option_env!("BUILD_INFO").map(|s| s.to_string()),
        }
    }
}

impl Default for ValidationStatus {
    fn default() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}
