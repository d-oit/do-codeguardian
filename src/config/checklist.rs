use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Centralized security checklist management for synchronization across components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityChecklist {
    pub version: String,
    pub last_updated: SystemTime,
    pub categories: HashMap<String, ChecklistCategory>,
    pub global_rules: Vec<ChecklistRule>,
    pub repository_specific: HashMap<String, Vec<ChecklistRule>>,
    pub synchronization_config: SynchronizationConfig,
}

/// Category of security checks (e.g., authentication, encryption, input validation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistCategory {
    pub name: String,
    pub description: String,
    pub priority: ChecklistPriority,
    pub rules: Vec<ChecklistRule>,
    pub dependencies: Vec<String>, // Other categories this depends on
}

/// Individual security checklist rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub priority: ChecklistPriority,
    pub rule_type: ChecklistRuleType,
    pub validation_pattern: Option<String>,
    pub remediation_steps: Vec<String>,
    pub applicable_file_types: Vec<String>,
    pub tags: Vec<String>,
    pub enabled: bool,
    pub last_modified: SystemTime,
}

/// Priority levels for checklist items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChecklistPriority {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Types of checklist rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChecklistRuleType {
    CodePattern,   // Pattern matching in code
    Configuration, // Configuration validation
    Dependency,    // Dependency security check
    Documentation, // Documentation requirement
    Process,       // Process/workflow requirement
    Architecture,  // Architectural constraint
}

/// Configuration for checklist synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationConfig {
    pub auto_sync: bool,
    pub sync_interval_hours: u64,
    pub central_repository: Option<String>,
    pub sync_branches: Vec<String>,
    pub conflict_resolution: ConflictResolution,
    pub backup_enabled: bool,
    pub backup_retention_days: u32,
}

/// How to handle conflicts during synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    PreferLocal,  // Keep local changes
    PreferRemote, // Use remote changes
    Manual,       // Require manual resolution
    MergeRules,   // Attempt to merge rules intelligently
}

/// Synchronization result
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub success: bool,
    pub changes_applied: usize,
    pub conflicts_detected: usize,
    pub errors: Vec<String>,
    pub summary: String,
}

/// Checklist validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub missing_rules: Vec<String>,
    pub outdated_rules: Vec<String>,
    pub conflicts: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for SecurityChecklist {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityChecklist {
    /// Create a new security checklist with default rules
    pub fn new() -> Self {
        let mut checklist = Self {
            version: "1.0.0".to_string(),
            last_updated: SystemTime::now(),
            categories: HashMap::new(),
            global_rules: Vec::new(),
            repository_specific: HashMap::new(),
            synchronization_config: SynchronizationConfig::default(),
        };

        // Initialize with default security categories and rules
        checklist.initialize_default_categories();
        checklist.initialize_default_rules();
        checklist
    }

    /// Load checklist from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let checklist: SecurityChecklist = toml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse checklist file: {}", e))?;
        Ok(checklist)
    }

    /// Save checklist to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize checklist: {}", e))?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Synchronize with remote checklist repository
    pub async fn synchronize_with_remote(&mut self, remote_url: &str) -> Result<SyncResult> {
        let mut result = SyncResult {
            success: false,
            changes_applied: 0,
            conflicts_detected: 0,
            errors: Vec::new(),
            summary: String::new(),
        };

        // Download remote checklist
        let remote_checklist = match self.fetch_remote_checklist(remote_url).await {
            Ok(checklist) => checklist,
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to fetch remote checklist: {}", e));
                return Ok(result);
            }
        };

        // Compare versions and detect conflicts
        let conflicts = self.detect_conflicts(&remote_checklist);
        result.conflicts_detected = conflicts.len();

        // Apply synchronization based on conflict resolution strategy
        match self.synchronization_config.conflict_resolution {
            ConflictResolution::PreferRemote => {
                result.changes_applied = self.apply_remote_changes(&remote_checklist)?;
            }
            ConflictResolution::PreferLocal => {
                // Keep local changes, only add new rules from remote
                result.changes_applied = self.merge_new_rules(&remote_checklist)?;
            }
            ConflictResolution::MergeRules => {
                result.changes_applied = self.intelligent_merge(&remote_checklist)?;
            }
            ConflictResolution::Manual => {
                result.summary = "Manual conflict resolution required".to_string();
                return Ok(result);
            }
        }

        // Update metadata
        self.last_updated = SystemTime::now();
        result.success = true;
        result.summary = format!(
            "Synchronization completed: {} changes applied, {} conflicts detected",
            result.changes_applied, result.conflicts_detected
        );

        Ok(result)
    }

    /// Validate checklist against repository requirements
    pub fn validate_for_repository(&self, repo_path: &Path) -> Result<ValidationResult> {
        let mut result = ValidationResult {
            valid: true,
            missing_rules: Vec::new(),
            outdated_rules: Vec::new(),
            conflicts: Vec::new(),
            recommendations: Vec::new(),
        };

        // Check for required security categories
        let required_categories = vec![
            "authentication",
            "authorization",
            "input_validation",
            "cryptography",
            "error_handling",
            "logging",
        ];

        for category in &required_categories {
            if !self.categories.contains_key(*category) {
                result
                    .missing_rules
                    .push(format!("Missing category: {}", category));
                result.valid = false;
            }
        }

        // Validate rules for repository-specific requirements
        if let Some(repo_name) = repo_path.file_name().and_then(|n| n.to_str()) {
            if let Some(repo_rules) = self.repository_specific.get(repo_name) {
                for rule in repo_rules {
                    if !rule.enabled {
                        result.recommendations.push(format!(
                            "Consider enabling rule '{}' for this repository",
                            rule.name
                        ));
                    }
                }
            }
        }

        // Check for outdated rules (older than 90 days)
        let ninety_days_ago = SystemTime::now() - std::time::Duration::from_secs(90 * 24 * 3600);
        for rule in &self.global_rules {
            if rule.last_modified < ninety_days_ago {
                result.outdated_rules.push(rule.id.clone());
            }
        }

        // Validate rule dependencies
        for category in self.categories.values() {
            for dep in &category.dependencies {
                if !self.categories.contains_key(dep) {
                    result.conflicts.push(format!(
                        "Category '{}' depends on missing category '{}'",
                        category.name, dep
                    ));
                    result.valid = false;
                }
            }
        }

        Ok(result)
    }

    /// Add or update a security rule
    pub fn add_or_update_rule(&mut self, rule: ChecklistRule) -> Result<()> {
        // Validate rule
        if rule.id.is_empty() || rule.name.is_empty() {
            return Err(anyhow!("Rule ID and name cannot be empty"));
        }

        // Check if category exists
        if !self.categories.contains_key(&rule.category) {
            return Err(anyhow!("Category '{}' does not exist", rule.category));
        }

        // Add to global rules or update existing
        if let Some(existing_rule) = self.global_rules.iter_mut().find(|r| r.id == rule.id) {
            *existing_rule = rule;
        } else {
            self.global_rules.push(rule);
        }

        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Remove a security rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<bool> {
        let initial_len = self.global_rules.len();
        self.global_rules.retain(|rule| rule.id != rule_id);

        // Also remove from categories
        for category in self.categories.values_mut() {
            category.rules.retain(|rule| rule.id != rule_id);
        }

        let removed = self.global_rules.len() < initial_len;
        if removed {
            self.last_updated = SystemTime::now();
        }

        Ok(removed)
    }

    /// Get rules by category
    pub fn get_rules_by_category(&self, category: &str) -> Vec<&ChecklistRule> {
        self.global_rules
            .iter()
            .filter(|rule| rule.category == category)
            .collect()
    }

    /// Get rules by priority
    pub fn get_rules_by_priority(&self, priority: ChecklistPriority) -> Vec<&ChecklistRule> {
        self.global_rules
            .iter()
            .filter(|rule| rule.priority == priority)
            .collect()
    }

    /// Get enabled rules for specific file types
    pub fn get_rules_for_file_type(&self, file_extension: &str) -> Vec<&ChecklistRule> {
        self.global_rules
            .iter()
            .filter(|rule| {
                rule.enabled
                    && (rule.applicable_file_types.is_empty()
                        || rule
                            .applicable_file_types
                            .contains(&file_extension.to_string()))
            })
            .collect()
    }

    /// Initialize default security categories
    fn initialize_default_categories(&mut self) {
        let categories = vec![
            (
                "authentication",
                "User authentication and session management",
                ChecklistPriority::Critical,
            ),
            (
                "authorization",
                "Access control and permission management",
                ChecklistPriority::Critical,
            ),
            (
                "input_validation",
                "Input sanitization and validation",
                ChecklistPriority::High,
            ),
            (
                "cryptography",
                "Encryption and cryptographic operations",
                ChecklistPriority::High,
            ),
            (
                "error_handling",
                "Secure error handling and logging",
                ChecklistPriority::Medium,
            ),
            (
                "logging",
                "Security logging and monitoring",
                ChecklistPriority::Medium,
            ),
            (
                "configuration",
                "Secure configuration management",
                ChecklistPriority::Medium,
            ),
            (
                "dependencies",
                "Third-party dependency security",
                ChecklistPriority::High,
            ),
        ];

        for (name, description, priority) in categories {
            let category = ChecklistCategory {
                name: name.to_string(),
                description: description.to_string(),
                priority,
                rules: Vec::new(),
                dependencies: Vec::new(),
            };
            self.categories.insert(name.to_string(), category);
        }

        // Set up dependencies
        if let Some(auth_category) = self.categories.get_mut("authorization") {
            auth_category
                .dependencies
                .push("authentication".to_string());
        }
    }

    /// Initialize default security rules
    fn initialize_default_rules(&mut self) {
        let default_rules = vec![
            ChecklistRule {
                id: "AUTH_001".to_string(),
                name: "Password Complexity".to_string(),
                description: "Ensure passwords meet complexity requirements".to_string(),
                category: "authentication".to_string(),
                priority: ChecklistPriority::Critical,
                rule_type: ChecklistRuleType::CodePattern,
                validation_pattern: Some(
                    r"(?i)(password|passwd|pwd).*[a-z].*[A-Z].*\d".to_string(),
                ),
                remediation_steps: vec![
                    "Implement password complexity validation".to_string(),
                    "Require minimum 8 characters with mixed case and numbers".to_string(),
                ],
                applicable_file_types: vec!["rs".to_string(), "js".to_string(), "py".to_string()],
                tags: vec!["password".to_string(), "security".to_string()],
                enabled: true,
                last_modified: SystemTime::now(),
            },
            ChecklistRule {
                id: "INPUT_001".to_string(),
                name: "SQL Injection Prevention".to_string(),
                description: "Prevent SQL injection vulnerabilities".to_string(),
                category: "input_validation".to_string(),
                priority: ChecklistPriority::Critical,
                rule_type: ChecklistRuleType::CodePattern,
                validation_pattern: Some(r"(?i)(select|insert|update|delete).*\+.*".to_string()),
                remediation_steps: vec![
                    "Use parameterized queries".to_string(),
                    "Validate and sanitize all user inputs".to_string(),
                ],
                applicable_file_types: vec![
                    "rs".to_string(),
                    "js".to_string(),
                    "py".to_string(),
                    "java".to_string(),
                ],
                tags: vec![
                    "sql".to_string(),
                    "injection".to_string(),
                    "database".to_string(),
                ],
                enabled: true,
                last_modified: SystemTime::now(),
            },
            ChecklistRule {
                id: "CRYPTO_001".to_string(),
                name: "Strong Encryption".to_string(),
                description: "Use strong encryption algorithms".to_string(),
                category: "cryptography".to_string(),
                priority: ChecklistPriority::High,
                rule_type: ChecklistRuleType::CodePattern,
                validation_pattern: Some(r"(?i)(aes|rsa|sha256|sha512)".to_string()),
                remediation_steps: vec![
                    "Use AES-256 for symmetric encryption".to_string(),
                    "Use RSA-2048 or higher for asymmetric encryption".to_string(),
                ],
                applicable_file_types: vec!["rs".to_string(), "js".to_string(), "py".to_string()],
                tags: vec!["encryption".to_string(), "crypto".to_string()],
                enabled: true,
                last_modified: SystemTime::now(),
            },
        ];

        for rule in default_rules {
            self.global_rules.push(rule);
        }
    }

    /// Fetch remote checklist (placeholder for actual implementation)
    async fn fetch_remote_checklist(&self, _remote_url: &str) -> Result<SecurityChecklist> {
        // In a real implementation, this would fetch from a remote repository
        // For now, return a mock remote checklist
        Ok(SecurityChecklist::new())
    }

    /// Detect conflicts between local and remote checklists
    fn detect_conflicts(&self, remote: &SecurityChecklist) -> Vec<String> {
        let mut conflicts = Vec::new();

        // Check for rule conflicts
        for local_rule in &self.global_rules {
            if let Some(remote_rule) = remote.global_rules.iter().find(|r| r.id == local_rule.id) {
                if local_rule.last_modified != remote_rule.last_modified {
                    conflicts.push(format!(
                        "Rule '{}' modified in both local and remote",
                        local_rule.id
                    ));
                }
            }
        }

        conflicts
    }

    /// Apply all changes from remote checklist
    fn apply_remote_changes(&mut self, remote: &SecurityChecklist) -> Result<usize> {
        let mut changes = 0;

        // Replace global rules
        self.global_rules = remote.global_rules.clone();
        changes += self.global_rules.len();

        // Replace categories
        self.categories = remote.categories.clone();

        // Update version
        self.version = remote.version.clone();

        Ok(changes)
    }

    /// Merge only new rules from remote
    fn merge_new_rules(&mut self, remote: &SecurityChecklist) -> Result<usize> {
        let mut changes = 0;

        for remote_rule in &remote.global_rules {
            if !self.global_rules.iter().any(|r| r.id == remote_rule.id) {
                self.global_rules.push(remote_rule.clone());
                changes += 1;
            }
        }

        Ok(changes)
    }

    /// Intelligent merge of local and remote changes
    fn intelligent_merge(&mut self, remote: &SecurityChecklist) -> Result<usize> {
        let mut changes = 0;

        for remote_rule in &remote.global_rules {
            if let Some(local_rule) = self
                .global_rules
                .iter_mut()
                .find(|r| r.id == remote_rule.id)
            {
                // Merge based on modification time
                if remote_rule.last_modified > local_rule.last_modified {
                    *local_rule = remote_rule.clone();
                    changes += 1;
                }
            } else {
                // New rule from remote
                self.global_rules.push(remote_rule.clone());
                changes += 1;
            }
        }

        Ok(changes)
    }
}

impl Default for SynchronizationConfig {
    fn default() -> Self {
        Self {
            auto_sync: false,
            sync_interval_hours: 24,
            central_repository: None,
            sync_branches: vec!["main".to_string(), "master".to_string()],
            conflict_resolution: ConflictResolution::Manual,
            backup_enabled: true,
            backup_retention_days: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_checklist_creation() -> Result<(), Box<dyn std::error::Error>> {
        let checklist = SecurityChecklist::new();
        assert!(!checklist.categories.is_empty());
        assert!(!checklist.global_rules.is_empty());
        assert_eq!(checklist.version, "1.0.0");
    }

    #[test]
    fn test_rule_management() -> Result<(), Box<dyn std::error::Error>> {
        let mut checklist = SecurityChecklist::new();

        let rule = ChecklistRule {
            id: "TEST_001".to_string(),
            name: "Test Rule".to_string(),
            description: "Test description".to_string(),
            category: "authentication".to_string(),
            priority: ChecklistPriority::Medium,
            rule_type: ChecklistRuleType::CodePattern,
            validation_pattern: None,
            remediation_steps: vec!["Step 1".to_string()],
            applicable_file_types: vec!["rs".to_string()],
            tags: vec!["test".to_string()],
            enabled: true,
            last_modified: SystemTime::now(),
        };

        // Add rule
        assert!(checklist.add_or_update_rule(rule).is_ok());
        assert!(checklist.global_rules.iter().any(|r| r.id == "TEST_001"));

        // Remove rule
        assert!(checklist.remove_rule("TEST_001")?);
        assert!(!checklist.global_rules.iter().any(|r| r.id == "TEST_001"));
    }

    #[test]
    fn test_file_operations() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("checklist.toml");

        let checklist = SecurityChecklist::new();

        // Save to file
        assert!(checklist.save_to_file(&file_path).is_ok());
        assert!(file_path.exists());

        // Load from file
        let loaded_checklist = SecurityChecklist::load_from_file(&file_path)?;
        assert_eq!(checklist.version, loaded_checklist.version);
        assert_eq!(
            checklist.global_rules.len(),
            loaded_checklist.global_rules.len()
        );
    }

    #[test]
    fn test_rule_filtering() -> Result<(), Box<dyn std::error::Error>> {
        let checklist = SecurityChecklist::new();

        // Test category filtering
        let auth_rules = checklist.get_rules_by_category("authentication");
        assert!(!auth_rules.is_empty());

        // Test priority filtering
        let critical_rules = checklist.get_rules_by_priority(ChecklistPriority::Critical);
        assert!(!critical_rules.is_empty());

        // Test file type filtering
        let rust_rules = checklist.get_rules_for_file_type("rs");
        assert!(!rust_rules.is_empty());
    }

    #[test]
    fn test_validation() -> Result<(), Box<dyn std::error::Error>> {
        let checklist = SecurityChecklist::new();
        let temp_dir = TempDir::new()?;

        let result = checklist.validate_for_repository(temp_dir.path())?;
        assert!(result.valid);
    }
}
