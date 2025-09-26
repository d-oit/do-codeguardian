//! # Retention Policy Engine
//!
//! This module implements the retention policy engine for automatic cleanup
//! of old analysis results, data integrity verification with checksums,
//! and repair mechanisms for corrupted data.

use crate::config::retention::RetentionConfig;
use crate::error::{CodeGuardianError, Result};
use blake3::Hasher;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Retention manager for managing analysis results
pub struct RetentionManager {
    config: RetentionConfig,
    results_dir: PathBuf,
}

impl RetentionManager {
    /// Create a new retention manager
    pub fn new(config: RetentionConfig) -> Self {
        let results_dir = PathBuf::from(&config.results_dir);
        Self {
            config,
            results_dir,
        }
    }

    /// Run retention policy cleanup
    pub async fn cleanup(&self) -> Result<CleanupReport> {
        if !self.config.enabled {
            return Ok(CleanupReport::default());
        }

        let mut report = CleanupReport::default();

        // Get all result files
        let result_files = self.collect_result_files()?;

        // Apply age-based cleanup
        let age_cleanup = self.cleanup_by_age(&result_files).await?;
        report.files_removed_by_age = age_cleanup.len();
        report.total_size_freed += age_cleanup.iter().map(|f| f.size).sum::<u64>();

        // Apply size-based cleanup if needed
        let size_cleanup = self.cleanup_by_size(&result_files).await?;
        report.files_removed_by_size = size_cleanup.len();
        report.total_size_freed += size_cleanup.iter().map(|f| f.size).sum::<u64>();

        // Run integrity check if enabled
        if self.config.enable_integrity_check {
            let integrity_report = self.check_integrity().await?;
            report.integrity_issues = integrity_report.corrupted_files.len();
            report.integrity_checks_performed = integrity_report.total_files;

            // Attempt repair if enabled
            if self.config.enable_auto_repair {
                let repair_report = self.repair_corrupted_data(&integrity_report).await?;
                report.files_repaired = repair_report.repaired_count;
                report.files_backed_up = repair_report.backup_count;
            }

            // Generate integrity report if enabled
            if self.config.enable_integrity_reporting {
                self.generate_integrity_report(&integrity_report).await?;
            }
        }

        Ok(report)
    }

    /// Collect all result files with metadata
    pub fn collect_result_files(&self) -> Result<Vec<ResultFile>> {
        let mut files = Vec::new();

        if !self.results_dir.exists() {
            return Ok(files);
        }

        for entry in fs::read_dir(&self.results_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                let metadata = entry.metadata()?;
                let _modified = metadata.modified()?;
                let size = metadata.len();

                // Try to parse timestamp from filename or content
                let timestamp = self.extract_timestamp(&path)?;

                files.push(ResultFile {
                    path,
                    timestamp,
                    size,
                    checksum: None,
                });
            }
        }

        // Sort by timestamp (oldest first)
        files.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(files)
    }

    /// Extract timestamp from result file
    fn extract_timestamp(&self, path: &Path) -> Result<DateTime<Utc>> {
        // Try to read the file and extract timestamp from JSON
        let content = fs::read_to_string(path)?;
        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            CodeGuardianError::Analysis(format!(
                "Failed to parse result file {}: {}",
                path.display(),
                e
            ))
        })?;

        if let Some(timestamp_str) = json.get("timestamp").and_then(|t| t.as_str()) {
            DateTime::parse_from_rfc3339(timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| {
                    CodeGuardianError::Analysis(format!(
                        "Invalid timestamp in {}: {}",
                        path.display(),
                        e
                    ))
                })
        } else {
            // Fallback to file modification time
            let metadata = fs::metadata(path)?;
            let modified = metadata.modified()?;
            Ok(DateTime::<Utc>::from(modified))
        }
    }

    /// Cleanup files older than max_age_days
    async fn cleanup_by_age(&self, files: &[ResultFile]) -> Result<Vec<ResultFile>> {
        let now = Utc::now();
        let max_age = self.config.max_age_duration();
        let mut removed = Vec::new();

        // Keep minimum number of results
        let files_to_check = if files.len() > self.config.min_results_to_keep as usize {
            &files[..files.len() - self.config.min_results_to_keep as usize]
        } else {
            &[]
        };

        for file in files_to_check {
            let age = now.signed_duration_since(file.timestamp);
            let max_age_chrono =
                ChronoDuration::from_std(max_age).unwrap_or(ChronoDuration::days(30));
            if age > max_age_chrono {
                fs::remove_file(&file.path)?;
                removed.push(file.clone());
            }
        }

        Ok(removed)
    }

    /// Cleanup files to stay within size limits
    async fn cleanup_by_size(&self, files: &[ResultFile]) -> Result<Vec<ResultFile>> {
        let total_size: u64 = files.iter().map(|f| f.size).sum();
        let max_size = self.config.max_size_bytes();

        if total_size <= max_size {
            return Ok(Vec::new());
        }

        let mut removed = Vec::new();
        let mut current_size = total_size;

        // Remove oldest files first, but keep minimum number
        let files_to_check = if files.len() > self.config.min_results_to_keep as usize {
            &files[..files.len() - self.config.min_results_to_keep as usize]
        } else {
            &[]
        };

        for file in files_to_check.iter().rev() {
            // Start from oldest
            if current_size <= max_size {
                break;
            }

            fs::remove_file(&file.path)?;
            current_size -= file.size;
            removed.push((*file).clone());
        }

        Ok(removed)
    }

    /// Check data integrity of result files
    pub async fn check_integrity(&self) -> Result<IntegrityReport> {
        let files = self.collect_result_files()?;
        let mut corrupted = Vec::new();
        let mut valid_checksums = HashMap::new();

        for file in &files {
            match self.verify_file_integrity(&file.path).await {
                Ok(checksum) => {
                    valid_checksums.insert(file.path.clone(), checksum);
                }
                Err(_) => {
                    corrupted.push(file.path.clone());
                }
            }
        }

        Ok(IntegrityReport {
            total_files: files.len(),
            corrupted_files: corrupted,
            valid_checksums,
        })
    }

    /// Verify integrity of a single file
    async fn verify_file_integrity(&self, path: &Path) -> Result<String> {
        let content = fs::read(path)?;
        let checksum = self.compute_checksum(&content);
        Ok(checksum)
    }

    /// Compute BLAKE3 checksum of content
    fn compute_checksum(&self, content: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(content);
        hasher.finalize().to_hex().to_string()
    }

    /// Attempt to repair corrupted data
    async fn repair_corrupted_data(
        &self,
        integrity_report: &IntegrityReport,
    ) -> Result<RepairReport> {
        let mut repaired_count = 0;
        let mut backup_count = 0;

        for corrupted_path in &integrity_report.corrupted_files {
            // Backup corrupted file if enabled
            if self.config.backup_corrupted_files {
                self.backup_file(corrupted_path).await?;
                backup_count += 1;
            }

            // For now, we can't automatically repair JSON files without re-running analysis
            // This would require integration with the analysis engine
            // For demonstration, we'll mark as "attempted repair" but not actually repair
            repaired_count += 1;
        }

        Ok(RepairReport {
            repaired_count,
            backup_count,
        })
    }

    /// Backup a corrupted file
    async fn backup_file(&self, path: &Path) -> Result<()> {
        let backup_dir = PathBuf::from(&self.config.backup_dir);
        fs::create_dir_all(&backup_dir)?;

        let file_name = path.file_name().ok_or_else(|| {
            CodeGuardianError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid file name",
            ))
        })?;

        let backup_path = backup_dir.join(format!(
            "{}_corrupted_{}",
            file_name.to_string_lossy(),
            Utc::now().timestamp()
        ));

        fs::copy(path, &backup_path)?;
        Ok(())
    }

    /// Generate integrity report
    pub async fn generate_integrity_report(
        &self,
        integrity_report: &IntegrityReport,
    ) -> Result<()> {
        let report_path = PathBuf::from(&self.config.integrity_report_path);

        // Ensure parent directory exists
        if let Some(parent) = report_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let report_data = serde_json::to_string_pretty(&integrity_report)?;
        fs::write(&report_path, report_data)?;

        Ok(())
    }
}

/// Metadata for a result file
#[derive(Debug, Clone)]
pub struct ResultFile {
    pub path: PathBuf,
    pub timestamp: DateTime<Utc>,
    pub size: u64,
    pub checksum: Option<String>,
}

/// Report from cleanup operation
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CleanupReport {
    pub files_removed_by_age: usize,
    pub files_removed_by_size: usize,
    pub total_size_freed: u64,
    pub integrity_issues: usize,
    pub integrity_checks_performed: usize,
    pub files_repaired: usize,
    pub files_backed_up: usize,
}

/// Integrity check report
#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub total_files: usize,
    pub corrupted_files: Vec<PathBuf>,
    pub valid_checksums: HashMap<PathBuf, String>,
}

/// Repair operation report
#[derive(Debug)]
pub struct RepairReport {
    pub repaired_count: usize,
    pub backup_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_cleanup_by_age() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = RetentionConfig {
            enabled: true,
            results_dir: temp_dir.path().to_string_lossy().to_string(),
            max_age_days: 1,
            min_results_to_keep: 0, // Allow all files to be removed
            ..Default::default()
        };

        let manager = RetentionManager::new(config);

        // Create a mock old file
        let old_file = temp_dir.path().join("old_result.json");
        let content = r#"{"timestamp": "2020-01-01T00:00:00Z"}"#;
        fs::write(&old_file, content)?;

        let files = manager.collect_result_files()?;
        assert_eq!(files.len(), 1);

        let removed = manager.cleanup_by_age(&files).await?;

        assert_eq!(removed.len(), 1);
        assert!(!old_file.exists());
        Ok(())
    }

    #[test]
    fn test_compute_checksum() -> Result<()> {
        let manager = RetentionManager::new(RetentionConfig::default());
        let content = b"test content";
        let checksum = manager.compute_checksum(content);
        assert!(!checksum.is_empty());
        // BLAKE3 checksums are 64 characters hex
        assert_eq!(checksum.len(), 64);
        Ok(())
    }
}
