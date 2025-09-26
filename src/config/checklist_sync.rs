use crate::analyzers::AnalyzerRegistry;
use crate::config::checklist::{ChecklistRule, SecurityChecklist, SyncResult, ValidationResult};
use crate::types::Finding;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::time::interval;

/// Service for synchronizing security checklists across repositories and components
pub struct ChecklistSynchronizationService {
    checklist: SecurityChecklist,
    sync_targets: Vec<SyncTarget>,
    last_sync: Option<SystemTime>,
    sync_interval: Duration,
    auto_sync_enabled: bool,
}

/// Target for checklist synchronization
#[derive(Debug, Clone)]
pub struct SyncTarget {
    pub name: String,
    pub target_type: SyncTargetType,
    pub url: String,
    pub branch: Option<String>,
    pub credentials: Option<String>,
    pub enabled: bool,
}

/// Types of synchronization targets
#[derive(Debug, Clone)]
pub enum SyncTargetType {
    GitRepository,
    HttpEndpoint,
    FileSystem(PathBuf),
    Database(String), // Connection string
}

/// Synchronization statistics
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub total_targets: usize,
    pub successful_syncs: usize,
    pub failed_syncs: usize,
    pub rules_synchronized: usize,
    pub conflicts_resolved: usize,
    pub last_sync_duration: Duration,
}

impl ChecklistSynchronizationService {
    /// Create a new synchronization service
    pub fn new(checklist: SecurityChecklist) -> Self {
        let sync_interval =
            Duration::from_secs(checklist.synchronization_config.sync_interval_hours * 3600);

        Self {
            checklist,
            sync_targets: Vec::new(),
            last_sync: None,
            sync_interval,
            auto_sync_enabled: false,
        }
    }

    /// Load service from configuration file
    pub fn load_from_file<P: AsRef<Path>>(checklist_path: P) -> Result<Self> {
        let checklist = SecurityChecklist::load_from_file(checklist_path)?;
        Ok(Self::new(checklist))
    }

    /// Add a synchronization target
    pub fn add_sync_target(&mut self, target: SyncTarget) {
        self.sync_targets.push(target);
    }

    /// Remove a synchronization target
    pub fn remove_sync_target(&mut self, name: &str) -> bool {
        let initial_len = self.sync_targets.len();
        self.sync_targets.retain(|target| target.name != name);
        self.sync_targets.len() < initial_len
    }

    /// Enable automatic synchronization
    pub fn enable_auto_sync(&mut self) {
        self.auto_sync_enabled = true;
    }

    /// Disable automatic synchronization
    pub fn disable_auto_sync(&mut self) {
        self.auto_sync_enabled = false;
    }

    /// Start automatic synchronization background task
    pub async fn start_auto_sync(&mut self) -> Result<()> {
        if !self.auto_sync_enabled {
            return Ok(());
        }

        let mut sync_timer = interval(self.sync_interval);

        loop {
            sync_timer.tick().await;

            if let Err(e) = self.sync_all_targets().await {
                tracing::error!("Auto-sync failed: {}", e);
            }
        }
    }

    /// Synchronize with all configured targets
    pub async fn sync_all_targets(&mut self) -> Result<SyncStats> {
        let start_time = SystemTime::now();
        let mut stats = SyncStats {
            total_targets: self.sync_targets.len(),
            successful_syncs: 0,
            failed_syncs: 0,
            rules_synchronized: 0,
            conflicts_resolved: 0,
            last_sync_duration: Duration::from_secs(0),
        };

        // Clone targets to avoid borrowing issues
        let targets = self.sync_targets.clone();
        for target in &targets {
            if !target.enabled {
                continue;
            }

            match self.sync_with_target(target).await {
                Ok(sync_result) => {
                    stats.successful_syncs += 1;
                    stats.rules_synchronized += sync_result.changes_applied;
                    stats.conflicts_resolved += sync_result.conflicts_detected;

                    tracing::info!(
                        "Successfully synced with target '{}': {} changes applied",
                        target.name,
                        sync_result.changes_applied
                    );
                }
                Err(e) => {
                    stats.failed_syncs += 1;
                    tracing::error!("Failed to sync with target '{}': {}", target.name, e);
                }
            }
        }

        self.last_sync = Some(SystemTime::now());
        stats.last_sync_duration = start_time.elapsed().unwrap_or_default();

        Ok(stats)
    }

    /// Synchronize with a specific target
    pub async fn sync_with_target(&mut self, target: &SyncTarget) -> Result<SyncResult> {
        match &target.target_type {
            SyncTargetType::GitRepository => self.sync_with_git_repository(target).await,
            SyncTargetType::HttpEndpoint => self.sync_with_http_endpoint(target).await,
            SyncTargetType::FileSystem(path) => self.sync_with_filesystem(target, path).await,
            SyncTargetType::Database(connection) => {
                self.sync_with_database(target, connection).await
            }
        }
    }

    /// Validate checklist against analyzer registry
    pub fn validate_against_analyzers(
        &self,
        registry: &AnalyzerRegistry,
    ) -> Result<ValidationResult> {
        let mut result = ValidationResult {
            valid: true,
            missing_rules: Vec::new(),
            outdated_rules: Vec::new(),
            conflicts: Vec::new(),
            recommendations: Vec::new(),
        };

        // Check if checklist rules cover all analyzer capabilities
        let analyzer_names = vec![
            "security",
            "duplicate",
            "performance",
            "dependency",
            "integrity",
            "cross_file_duplicate",
        ];

        for analyzer_name in &analyzer_names {
            let analyzer_rules: Vec<_> = self
                .checklist
                .global_rules
                .iter()
                .filter(|rule| rule.tags.contains(&analyzer_name.to_string()))
                .collect();

            if analyzer_rules.is_empty() {
                result
                    .missing_rules
                    .push(format!("No rules found for analyzer: {}", analyzer_name));
                result.valid = false;
            }
        }

        // Check for analyzer-specific requirements
        self.validate_security_analyzer_rules(&mut result);
        self.validate_duplicate_analyzer_rules(&mut result);
        self.validate_performance_analyzer_rules(&mut result);

        Ok(result)
    }

    /// Apply checklist rules to findings for validation
    pub fn validate_findings(&self, findings: &[Finding]) -> Result<Vec<Finding>> {
        let mut validated_findings = Vec::new();

        for finding in findings {
            if let Some(applicable_rules) = self.get_applicable_rules_for_finding(finding) {
                let mut validated_finding = finding.clone();

                // Apply rule-based validation
                for rule in applicable_rules {
                    if let Some(pattern) = &rule.validation_pattern {
                        if self.validate_finding_against_pattern(finding, pattern) {
                            validated_finding =
                                self.enhance_finding_with_rule(validated_finding, rule);
                        }
                    }
                }

                validated_findings.push(validated_finding);
            } else {
                // No applicable rules - include finding as-is
                validated_findings.push(finding.clone());
            }
        }

        Ok(validated_findings)
    }

    /// Get checklist rules applicable to a specific finding
    fn get_applicable_rules_for_finding(&self, finding: &Finding) -> Option<Vec<&ChecklistRule>> {
        let file_extension = finding
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let applicable_rules: Vec<_> = self
            .checklist
            .global_rules
            .iter()
            .filter(|rule| {
                rule.enabled
                    && (rule.applicable_file_types.is_empty()
                        || rule
                            .applicable_file_types
                            .contains(&file_extension.to_string()))
                    && (rule.tags.contains(&finding.category) || rule.category == finding.category)
            })
            .collect();

        if applicable_rules.is_empty() {
            None
        } else {
            Some(applicable_rules)
        }
    }

    /// Validate finding against a rule pattern
    fn validate_finding_against_pattern(&self, finding: &Finding, pattern: &str) -> bool {
        if let Ok(regex) = regex::Regex::new(pattern) {
            regex.is_match(&finding.message)
                || regex.is_match(&finding.description.as_deref().unwrap_or(""))
        } else {
            false
        }
    }

    /// Enhance finding with rule information
    fn enhance_finding_with_rule(&self, mut finding: Finding, rule: &ChecklistRule) -> Finding {
        // Add rule information to finding
        if finding.description.is_none() {
            finding.description = Some(rule.description.clone());
        }

        // Add remediation suggestions
        if !rule.remediation_steps.is_empty() {
            let remediation = rule.remediation_steps.join("; ");
            finding.suggestion = Some(remediation);
        }

        finding
    }

    /// Synchronize with Git repository
    async fn sync_with_git_repository(&mut self, target: &SyncTarget) -> Result<SyncResult> {
        // In a real implementation, this would:
        // 1. Clone/pull the repository
        // 2. Read the checklist file
        // 3. Merge changes
        // 4. Push updates back

        tracing::info!("Syncing with Git repository: {}", target.url);

        // Mock implementation
        Ok(SyncResult {
            success: true,
            changes_applied: 0,
            conflicts_detected: 0,
            errors: Vec::new(),
            summary: "Git repository sync completed".to_string(),
        })
    }

    /// Synchronize with HTTP endpoint
    async fn sync_with_http_endpoint(&mut self, target: &SyncTarget) -> Result<SyncResult> {
        // In a real implementation, this would:
        // 1. Fetch checklist from HTTP endpoint
        // 2. Compare and merge changes
        // 3. POST updates back to endpoint

        tracing::info!("Syncing with HTTP endpoint: {}", target.url);

        // Mock implementation
        Ok(SyncResult {
            success: true,
            changes_applied: 0,
            conflicts_detected: 0,
            errors: Vec::new(),
            summary: "HTTP endpoint sync completed".to_string(),
        })
    }

    /// Synchronize with filesystem
    async fn sync_with_filesystem(
        &mut self,
        _target: &SyncTarget,
        path: &Path,
    ) -> Result<SyncResult> {
        if !path.exists() {
            return Err(anyhow!(
                "Filesystem path does not exist: {}",
                path.display()
            ));
        }

        let checklist_file = path.join("checklist.toml");
        if checklist_file.exists() {
            let _remote_checklist = SecurityChecklist::load_from_file(&checklist_file)?;
            let sync_result = self.checklist.synchronize_with_remote("filesystem").await?;

            // Save updated checklist back to filesystem
            self.checklist.save_to_file(&checklist_file)?;

            Ok(sync_result)
        } else {
            // Create new checklist file
            self.checklist.save_to_file(&checklist_file)?;

            Ok(SyncResult {
                success: true,
                changes_applied: 1,
                conflicts_detected: 0,
                errors: Vec::new(),
                summary: "Created new checklist file".to_string(),
            })
        }
    }

    /// Synchronize with database
    async fn sync_with_database(
        &mut self,
        target: &SyncTarget,
        _connection: &str,
    ) -> Result<SyncResult> {
        // In a real implementation, this would:
        // 1. Connect to database
        // 2. Query for checklist rules
        // 3. Sync changes
        // 4. Update database with local changes

        tracing::info!("Syncing with database: {}", target.name);

        // Mock implementation
        Ok(SyncResult {
            success: true,
            changes_applied: 0,
            conflicts_detected: 0,
            errors: Vec::new(),
            summary: "Database sync completed".to_string(),
        })
    }

    /// Validate security analyzer rules
    fn validate_security_analyzer_rules(&self, result: &mut ValidationResult) {
        let security_categories = vec![
            "authentication",
            "authorization",
            "cryptography",
            "input_validation",
        ];

        for category in &security_categories {
            let category_rules = self.checklist.get_rules_by_category(category);
            if category_rules.is_empty() {
                result
                    .missing_rules
                    .push(format!("Missing security rules for category: {}", category));
                result.valid = false;
            }
        }
    }

    /// Validate duplicate analyzer rules
    fn validate_duplicate_analyzer_rules(&self, result: &mut ValidationResult) {
        let duplicate_rules: Vec<_> = self
            .checklist
            .global_rules
            .iter()
            .filter(|rule| rule.tags.contains(&"duplicate".to_string()))
            .collect();

        if duplicate_rules.is_empty() {
            result
                .missing_rules
                .push("No rules found for duplicate detection".to_string());
            result.valid = false;
        }
    }

    /// Validate performance analyzer rules
    fn validate_performance_analyzer_rules(&self, result: &mut ValidationResult) {
        let performance_rules: Vec<_> = self
            .checklist
            .global_rules
            .iter()
            .filter(|rule| rule.tags.contains(&"performance".to_string()))
            .collect();

        if performance_rules.is_empty() {
            result.recommendations.push(
                "Consider adding performance-related rules for better analysis coverage"
                    .to_string(),
            );
        }
    }

    /// Get synchronization statistics
    pub fn get_sync_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();

        stats.insert(
            "total_targets".to_string(),
            self.sync_targets.len().to_string(),
        );
        stats.insert(
            "auto_sync_enabled".to_string(),
            self.auto_sync_enabled.to_string(),
        );
        stats.insert(
            "sync_interval_hours".to_string(),
            (self.sync_interval.as_secs() / 3600).to_string(),
        );

        if let Some(last_sync) = self.last_sync {
            if let Ok(duration) = last_sync.duration_since(SystemTime::UNIX_EPOCH) {
                stats.insert(
                    "last_sync_timestamp".to_string(),
                    duration.as_secs().to_string(),
                );
            }
        }

        stats.insert(
            "total_rules".to_string(),
            self.checklist.global_rules.len().to_string(),
        );
        stats.insert(
            "total_categories".to_string(),
            self.checklist.categories.len().to_string(),
        );

        stats
    }

    /// Export synchronization configuration
    pub fn export_sync_config(&self) -> Result<String> {
        let config = serde_json::json!({
            "checklist_version": self.checklist.version,
            "sync_targets": self.sync_targets,
            "auto_sync_enabled": self.auto_sync_enabled,
            "sync_interval_hours": self.sync_interval.as_secs() / 3600,
            "last_sync": self.last_sync,
        });

        Ok(serde_json::to_string_pretty(&config)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_sync_service_creation() -> Result<(), Box<dyn std::error::Error>> {
        let checklist = SecurityChecklist::new();
        let service = ChecklistSynchronizationService::new(checklist);

        assert_eq!(service.sync_targets.len(), 0);
        assert!(!service.auto_sync_enabled);
    }

    #[tokio::test]
    async fn test_sync_target_management() -> Result<(), Box<dyn std::error::Error>> {
        let checklist = SecurityChecklist::new();
        let mut service = ChecklistSynchronizationService::new(checklist);

        let target = SyncTarget {
            name: "test-target".to_string(),
            target_type: SyncTargetType::HttpEndpoint,
            url: "https://example.com/checklist".to_string(),
            branch: None,
            credentials: None,
            enabled: true,
        };

        service.add_sync_target(target);
        assert_eq!(service.sync_targets.len(), 1);

        assert!(service.remove_sync_target("test-target"));
        assert_eq!(service.sync_targets.len(), 0);
    }

    #[tokio::test]
    async fn test_filesystem_sync() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let checklist = SecurityChecklist::new();
        let mut service = ChecklistSynchronizationService::new(checklist);

        let target = SyncTarget {
            name: "filesystem-target".to_string(),
            target_type: SyncTargetType::FileSystem(temp_dir.path().to_path_buf()),
            url: temp_dir.path().to_string_lossy().to_string(),
            branch: None,
            credentials: None,
            enabled: true,
        };

        let result = service.sync_with_target(&target).await;
        assert!(result.is_ok());

        let sync_result = result?;
        assert!(sync_result.success);
        assert_eq!(sync_result.changes_applied, 1); // Created new file
    }
}
