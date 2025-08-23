use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;

/// Schema version for results format
///
/// This constant defines the current version of the analysis results schema.
/// It ensures compatibility when deserializing results from different versions.
pub const RESULTS_SCHEMA_VERSION: &str = "1.0.0";

/// Generates a stable, unique finding ID using SHA-256 hashing.
///
/// This function creates deterministic IDs based on the finding's characteristics,
/// ensuring that the same issue will always generate the same ID across runs.
/// The ID is truncated to 16 characters for readability while maintaining uniqueness.
///
/// # Arguments
/// * `analyzer` - Name of the analyzer that detected the issue
/// * `rule` - Specific rule that was violated
/// * `file` - Path to the file containing the issue
/// * `line` - Line number where the issue was found
/// * `key` - Additional key to differentiate similar findings
///
/// # Returns
/// A 16-character hexadecimal string representing the finding ID
pub fn generate_finding_id(analyzer: &str, rule: &str, file: &str, line: u32, key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(analyzer.as_bytes());
    hasher.update(rule.as_bytes());
    hasher.update(file.as_bytes());
    hasher.update(line.to_le_bytes());
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

/// Complete results from a CodeGuardian analysis run.
///
/// This structure contains all findings, metadata, and summary information
/// from a security and code quality analysis. It includes both the raw findings
/// and aggregated statistics for easy consumption and reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    /// Version of the results schema used
    pub schema_version: String,
    /// Metadata about the tool and analysis run
    pub tool_metadata: ToolMetadata,
    /// List of all security and quality findings
    pub findings: Vec<Finding>,
    /// Aggregated statistics and summary information
    pub summary: ResultsSummary,
    /// Hash of the configuration used for this analysis
    pub config_hash: String,
    /// Timestamp when the analysis was completed
    pub timestamp: DateTime<Utc>,
}

/// Metadata about the CodeGuardian tool and analysis run.
///
/// Contains information about which version of the tool was used,
/// when the analysis was performed, and the configuration state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Name of the analysis tool
    pub name: String,
    /// Version of the tool used for analysis
    pub version: String,
    /// Hash of the configuration used
    pub config_hash: String,
    /// Timestamp when the analysis was performed
    pub timestamp: DateTime<Utc>,
}

/// Represents a single security or code quality finding.
///
/// A finding contains all information about a detected issue, including
/// its location, severity, and suggested remediation. Each finding has
/// a unique ID for tracking across analysis runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Unique identifier for this finding (stable across runs)
    pub id: String,
    /// Name of the analyzer that detected this issue
    pub analyzer: String,
    /// Specific rule or check that was violated
    pub rule: String,
    /// Severity level of the finding
    pub severity: Severity,
    /// Path to the file containing the issue
    pub file: PathBuf,
    /// Line number where the issue was found (1-indexed)
    pub line: u32,
    /// Column number where the issue was found (optional, 1-indexed)
    pub column: Option<u32>,
    /// Human-readable message describing the issue
    pub message: String,
    /// Detailed description of the issue and its implications
    pub description: Option<String>,
    /// Suggested remediation or fix for the issue
    pub suggestion: Option<String>,
    /// Additional metadata specific to the analyzer or rule
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Severity levels for analysis findings.
///
/// The severity indicates the potential impact and urgency of addressing
/// a security or code quality issue. Higher severities should be addressed
/// before lower ones.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Critical issues that could lead to immediate security breaches or system compromise
    Critical,
    /// High-priority issues that pose significant security or reliability risks
    High,
    /// Medium-priority issues that should be addressed but are not immediately critical
    Medium,
    /// Low-priority issues that represent best practice violations or minor improvements
    Low,
    /// Informational findings that provide context or suggestions but are not issues
    Info,
}

/// Summary statistics from an analysis run.
///
/// Provides aggregated counts and metrics about the analysis results,
/// useful for reporting and understanding the overall state of the codebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultsSummary {
    /// Total number of files that were scanned during the analysis
    pub total_files_scanned: usize,
    /// Total number of findings across all files
    pub total_findings: usize,
    /// Count of findings grouped by severity level
    pub findings_by_severity: HashMap<Severity, usize>,
    /// Count of findings grouped by analyzer type
    pub findings_by_analyzer: HashMap<String, usize>,
    /// Duration of the scan in milliseconds
    pub scan_duration_ms: u64,
}

impl AnalysisResults {
    /// Creates a new AnalysisResults instance with the given configuration hash.
    ///
    /// Initializes all fields with default values and sets up the tool metadata
    /// with the current timestamp and version information.
    ///
    /// # Arguments
    /// * `config_hash` - Hash of the configuration used for this analysis
    pub fn new(config_hash: String) -> Self {
        Self {
            schema_version: RESULTS_SCHEMA_VERSION.to_string(),
            tool_metadata: ToolMetadata {
                name: "codeguardian".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_hash: config_hash.clone(),
                timestamp: Utc::now(),
            },
            findings: Vec::new(),
            summary: ResultsSummary {
                total_files_scanned: 0,
                total_findings: 0,
                findings_by_severity: HashMap::new(),
                findings_by_analyzer: HashMap::new(),
                scan_duration_ms: 0,
            },
            config_hash,
            timestamp: Utc::now(),
        }
    }

    /// Adds a finding to the results and updates summary statistics.
    ///
    /// This method adds the finding to the findings list and automatically
    /// updates the summary counters for total findings, severity counts,
    /// and analyzer counts.
    ///
    /// # Arguments
    /// * `finding` - The finding to add to the results
    pub fn add_finding(&mut self, finding: Finding) {
        *self
            .summary
            .findings_by_severity
            .entry(finding.severity.clone())
            .or_insert(0) += 1;
        *self
            .summary
            .findings_by_analyzer
            .entry(finding.analyzer.clone())
            .or_insert(0) += 1;
        self.summary.total_findings += 1;
        self.findings.push(finding);
    }

    /// Sorts findings by severity, file path, and line number.
    ///
    /// Findings are sorted in descending order of severity (Critical first),
    /// then alphabetically by file path, then by line number. This provides
    /// a consistent ordering for reporting and display purposes.
    pub fn sort_findings(&mut self) {
        // Deterministic ordering: severity → file → line
        self.findings.sort_by(|a, b| {
            a.severity
                .cmp(&b.severity)
                .then_with(|| a.file.cmp(&b.file))
                .then_with(|| a.line.cmp(&b.line))
        });
    }

    /// Checks if the analysis found any issues.
    ///
    /// Returns true if there are any findings in the results,
    /// regardless of their severity level.
    ///
    /// # Returns
    /// `true` if any findings were detected, `false` otherwise
    pub fn has_issues(&self) -> bool {
        !self.findings.is_empty()
    }

    /// Checks if the analysis found any high or critical severity issues.
    ///
    /// Returns true if there are any findings with Critical or High severity,
    /// which typically require immediate attention.
    ///
    /// # Returns
    /// `true` if any Critical or High severity findings were detected, `false` otherwise
    #[allow(dead_code)]
    pub fn has_high_severity_issues(&self) -> bool {
        self.findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Critical | Severity::High))
    }
}

impl Finding {
    /// Creates a new Finding with the required fields.
    ///
    /// Automatically generates a unique ID for the finding based on its
    /// characteristics. Optional fields like description and suggestion
    /// can be added using the builder pattern methods.
    ///
    /// # Arguments
    /// * `analyzer` - Name of the analyzer that detected the issue
    /// * `rule` - Specific rule that was violated
    /// * `severity` - Severity level of the finding
    /// * `file` - Path to the file containing the issue
    /// * `line` - Line number where the issue was found
    /// * `message` - Human-readable message describing the issue
    pub fn new(
        analyzer: &str,
        rule: &str,
        severity: Severity,
        file: PathBuf,
        line: u32,
        message: String,
    ) -> Self {
        let id = generate_finding_id(analyzer, rule, &file.to_string_lossy(), line, &message);

        Self {
            id,
            analyzer: analyzer.to_string(),
            rule: rule.to_string(),
            severity,
            file,
            line,
            column: None,
            message,
            description: None,
            suggestion: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the column number for this finding.
    ///
    /// # Arguments
    /// * `column` - Column number where the issue was found (1-indexed)
    #[allow(dead_code)]
    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets a detailed description for this finding.
    ///
    /// # Arguments
    /// * `description` - Detailed explanation of the issue and its implications
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets a remediation suggestion for this finding.
    ///
    /// # Arguments
    /// * `suggestion` - Suggested fix or remediation for the issue
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Adds metadata to this finding.
    ///
    /// # Arguments
    /// * `key` - Metadata key
    /// * `value` - Metadata value (any JSON-serializable value)
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::High => write!(f, "high"),
            Severity::Medium => write!(f, "medium"),
            Severity::Low => write!(f, "low"),
            Severity::Info => write!(f, "info"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_finding_id_generation() {
        let id1 = generate_finding_id("security", "hardcoded_secret", "src/main.rs", 42, "test message");
        let id2 = generate_finding_id("security", "hardcoded_secret", "src/main.rs", 42, "test message");
        let id3 = generate_finding_id("security", "hardcoded_secret", "src/main.rs", 43, "test message");
        
        // Same inputs should generate same ID
        assert_eq!(id1, id2);
        // Different inputs should generate different IDs
        assert_ne!(id1, id3);
        // ID should be 16 characters (truncated SHA-256)
        assert_eq!(id1.len(), 16);
    }

    #[test]
    fn test_finding_creation() {
        let finding = Finding::new(
            "security",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/test.rs"),
            10,
            "Test message".to_string(),
        );

        assert_eq!(finding.analyzer, "security");
        assert_eq!(finding.rule, "hardcoded_secret");
        assert_eq!(finding.severity, Severity::High);
        assert_eq!(finding.file, PathBuf::from("src/test.rs"));
        assert_eq!(finding.line, 10);
        assert_eq!(finding.message, "Test message");
        assert!(finding.description.is_none());
        assert!(finding.suggestion.is_none());
        assert!(finding.metadata.is_empty());
        assert!(!finding.id.is_empty());
    }

    #[test]
    fn test_finding_builder_pattern() {
        let finding = Finding::new(
            "security",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/test.rs"),
            10,
            "Test message".to_string(),
        )
        .with_description("Test description".to_string())
        .with_suggestion("Test suggestion".to_string())
        .with_metadata("key".to_string(), serde_json::Value::String("value".to_string()));

        assert_eq!(finding.description, Some("Test description".to_string()));
        assert_eq!(finding.suggestion, Some("Test suggestion".to_string()));
        assert_eq!(finding.metadata.len(), 1);
        assert_eq!(finding.metadata.get("key"), Some(&serde_json::Value::String("value".to_string())));
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Info);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Critical.to_string(), "critical");
        assert_eq!(Severity::High.to_string(), "high");
        assert_eq!(Severity::Medium.to_string(), "medium");
        assert_eq!(Severity::Low.to_string(), "low");
        assert_eq!(Severity::Info.to_string(), "info");
    }

    #[test]
    fn test_analysis_results_creation() {
        let config_hash = "test_hash".to_string();
        let results = AnalysisResults::new(config_hash.clone());

        assert_eq!(results.schema_version, RESULTS_SCHEMA_VERSION);
        assert_eq!(results.tool_metadata.name, "codeguardian");
        assert_eq!(results.tool_metadata.config_hash, config_hash);
        assert_eq!(results.config_hash, config_hash);
        assert!(results.findings.is_empty());
        assert_eq!(results.summary.total_findings, 0);
        assert_eq!(results.summary.total_files_scanned, 0);
    }

    #[test]
    fn test_analysis_results_add_finding() {
        let mut results = AnalysisResults::new("test_hash".to_string());
        
        let finding = Finding::new(
            "security",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/test.rs"),
            10,
            "Test message".to_string(),
        );

        results.add_finding(finding);

        assert_eq!(results.summary.total_findings, 1);
        assert_eq!(results.findings.len(), 1);
        assert_eq!(results.summary.findings_by_severity.get(&Severity::High), Some(&1));
        assert_eq!(results.summary.findings_by_analyzer.get("security"), Some(&1));
    }

    #[test]
    fn test_analysis_results_sorting() {
        let mut results = AnalysisResults::new("test_hash".to_string());
        
        // Add findings in non-sorted order
        results.add_finding(Finding::new(
            "security", "rule1", Severity::Low, PathBuf::from("z.rs"), 20, "msg".to_string()
        ));
        results.add_finding(Finding::new(
            "security", "rule2", Severity::Critical, PathBuf::from("a.rs"), 10, "msg".to_string()
        ));
        results.add_finding(Finding::new(
            "security", "rule3", Severity::High, PathBuf::from("b.rs"), 5, "msg".to_string()
        ));

        results.sort_findings();

        // Should be sorted by severity (Critical first), then file, then line
        assert_eq!(results.findings[0].severity, Severity::Critical);
        assert_eq!(results.findings[1].severity, Severity::High);
        assert_eq!(results.findings[2].severity, Severity::Low);
    }

    #[test]
    fn test_analysis_results_has_issues() {
        let mut results = AnalysisResults::new("test_hash".to_string());
        assert!(!results.has_issues());

        results.add_finding(Finding::new(
            "security", "rule1", Severity::Info, PathBuf::from("test.rs"), 1, "msg".to_string()
        ));
        assert!(results.has_issues());
    }

    #[test]
    fn test_analysis_results_has_high_severity_issues() {
        let mut results = AnalysisResults::new("test_hash".to_string());
        assert!(!results.has_high_severity_issues());

        // Add low severity finding
        results.add_finding(Finding::new(
            "security", "rule1", Severity::Info, PathBuf::from("test.rs"), 1, "msg".to_string()
        ));
        assert!(!results.has_high_severity_issues());

        // Add high severity finding
        results.add_finding(Finding::new(
            "security", "rule2", Severity::High, PathBuf::from("test.rs"), 2, "msg".to_string()
        ));
        assert!(results.has_high_severity_issues());
    }

    #[test]
    fn test_results_summary_aggregation() {
        let mut results = AnalysisResults::new("test_hash".to_string());
        
        // Add multiple findings with different severities and analyzers
        results.add_finding(Finding::new(
            "security", "rule1", Severity::High, PathBuf::from("test.rs"), 1, "msg".to_string()
        ));
        results.add_finding(Finding::new(
            "security", "rule2", Severity::High, PathBuf::from("test.rs"), 2, "msg".to_string()
        ));
        results.add_finding(Finding::new(
            "quality", "rule3", Severity::Medium, PathBuf::from("test.rs"), 3, "msg".to_string()
        ));

        assert_eq!(results.summary.total_findings, 3);
        assert_eq!(results.summary.findings_by_severity.get(&Severity::High), Some(&2));
        assert_eq!(results.summary.findings_by_severity.get(&Severity::Medium), Some(&1));
        assert_eq!(results.summary.findings_by_analyzer.get("security"), Some(&2));
        assert_eq!(results.summary.findings_by_analyzer.get("quality"), Some(&1));
    }
}
