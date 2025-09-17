//! # Retention Configuration Module
//!
//! This module defines configuration options for the retention policy engine,
//! which manages automatic cleanup of old analysis results and data integrity.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Retention policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfig {
    /// Enable retention policy engine
    pub enabled: bool,
    /// Directory to manage retention for
    pub results_dir: String,
    /// Maximum age of results to keep (in days)
    pub max_age_days: u32,
    /// Maximum total size of results directory (in MB)
    pub max_size_mb: u64,
    /// Minimum number of results to keep (even if old)
    pub min_results_to_keep: u32,
    /// Enable data integrity verification
    pub enable_integrity_check: bool,
    /// Integrity check frequency (in days)
    pub integrity_check_frequency_days: u32,
    /// Enable automatic repair of corrupted data
    pub enable_auto_repair: bool,
    /// Backup corrupted files before repair
    pub backup_corrupted_files: bool,
    /// Backup directory for corrupted files
    pub backup_dir: String,
    /// Enable integrity reporting
    pub enable_integrity_reporting: bool,
    /// Report file path
    pub integrity_report_path: String,
}

impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            results_dir: "analysis-results".to_string(),
            max_age_days: 30,
            max_size_mb: 500,
            min_results_to_keep: 10,
            enable_integrity_check: true,
            integrity_check_frequency_days: 7,
            enable_auto_repair: false,
            backup_corrupted_files: true,
            backup_dir: "analysis-results/backup".to_string(),
            enable_integrity_reporting: true,
            integrity_report_path: "analysis-results/integrity-report.json".to_string(),
        }
    }
}

impl RetentionConfig {
    /// Get maximum age as Duration
    pub fn max_age_duration(&self) -> Duration {
        Duration::from_secs(self.max_age_days as u64 * 24 * 60 * 60)
    }

    /// Get maximum size in bytes
    pub fn max_size_bytes(&self) -> u64 {
        self.max_size_mb * 1024 * 1024
    }

    /// Get integrity check frequency as Duration
    pub fn integrity_check_frequency(&self) -> Duration {
        Duration::from_secs(self.integrity_check_frequency_days as u64 * 24 * 60 * 60)
    }
}
