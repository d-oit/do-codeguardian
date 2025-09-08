//! # Output Configuration Module
//!
//! This module contains configuration structures related to output settings,
//! including directory paths, formats, and archiving options.

use serde::{Deserialize, Serialize};

/// Output configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Directory where analysis results will be saved
    pub directory: String,
    /// Output format options: "json", "html", "markdown", "sarif"
    pub format: String,
    /// Enable verbose output
    pub verbose: bool,
    /// Generate summary reports
    pub generate_summary: bool,
    /// Compress large output files
    pub compress_output: bool,
    /// Subdirectory for reports within the output directory
    pub reports_subdirectory: String,
    /// Subdirectory for data files within the output directory
    pub data_subdirectory: String,
    /// Subdirectory for temporary files within the output directory
    pub temp_subdirectory: String,
    /// Subdirectory for historical reports within the output directory
    pub historical_subdirectory: String,
    /// Automatically archive old reports
    pub auto_archive: bool,
    /// Maximum number of reports to keep before archiving
    pub max_reports_kept: u32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            directory: "analysis-results".to_string(),
            format: "sarif".to_string(),
            verbose: false,
            generate_summary: true,
            compress_output: true,
            reports_subdirectory: "reports".to_string(),
            data_subdirectory: "data".to_string(),
            temp_subdirectory: "temp".to_string(),
            historical_subdirectory: "historical".to_string(),
            auto_archive: true,
            max_reports_kept: 10,
        }
    }
}
