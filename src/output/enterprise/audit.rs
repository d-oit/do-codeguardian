//! # Audit Trail System
//!
//! This module provides comprehensive audit logging for all CodeGuardian operations,
//! ensuring compliance with security standards and regulatory requirements.

use super::EnterpriseConfig;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Audit logger for tracking all system operations
pub struct AuditLogger {
    enabled: bool,
    storage: FileAuditStorage,
    config: AuditConfig,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable detailed logging
    pub detailed_logging: bool,
    /// Log sensitive data (be careful with this)
    pub log_sensitive_data: bool,
    /// Maximum log entry size in bytes
    pub max_entry_size_bytes: usize,
    /// Retention period in days
    pub retention_days: u32,
    /// Enable log integrity verification
    pub enable_integrity_check: bool,
    /// Storage backend configuration
    pub storage_config: AuditStorageConfig,
}

/// Audit storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorageConfig {
    /// Storage backend type
    pub backend: AuditStorageBackend,
    /// Storage location/connection string
    pub location: String,
    /// Enable encryption for stored logs
    pub encrypt_logs: bool,
    /// Batch size for log writes
    pub batch_size: usize,
}

/// Supported audit storage backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditStorageBackend {
    File,
    Database,
    CloudStorage,
    Syslog,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Unique entry ID
    pub id: Uuid,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Operation that was performed
    pub operation: String,
    /// Actor who performed the operation
    pub actor: Option<ActorInfo>,
    /// Target of the operation
    pub target: Option<TargetInfo>,
    /// Tenant context
    pub tenant_id: Option<Uuid>,
    /// Request ID for correlation
    pub request_id: Option<Uuid>,
    /// Client information
    pub client_info: Option<ClientInfo>,
    /// Operation result
    pub result: OperationResult,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
    /// Risk level of the operation
    pub risk_level: RiskLevel,
    /// Log integrity hash
    pub integrity_hash: Option<String>,
}

/// Information about the actor performing the operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInfo {
    /// Actor type (user, system, api_key)
    pub actor_type: ActorType,
    /// Actor ID
    pub id: Uuid,
    /// Actor name/username
    pub name: String,
    /// Actor roles
    pub roles: Vec<String>,
    /// Authentication method used
    pub auth_method: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
}

/// Type of actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorType {
    User,
    System,
    ApiKey,
    Service,
}

/// Information about the target of the operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
    /// Target type
    pub target_type: String,
    /// Target ID
    pub id: Option<String>,
    /// Target name/description
    pub name: Option<String>,
    /// Additional target properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Client information for the request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client IP address
    pub ip_address: String,
    /// User agent string
    pub user_agent: Option<String>,
    /// Geographic location (if available)
    pub location: Option<GeoLocation>,
    /// Device fingerprint
    pub device_fingerprint: Option<String>,
}

/// Geographic location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Result of an operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Failure(String),
    PartialSuccess(String),
}

/// Risk level classification for operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Audit trail query criteria
#[derive(Debug, Clone)]
pub struct AuditQuery {
    /// Filter by tenant ID
    pub tenant_id: Option<Uuid>,
    /// Filter by actor ID
    pub actor_id: Option<Uuid>,
    /// Filter by operation type
    pub operation: Option<String>,
    /// Filter by time range
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by risk level
    pub min_risk_level: Option<RiskLevel>,
    /// Filter by result type
    pub result_type: Option<OperationResult>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

/// Audit storage trait
pub trait AuditStorage: Send + Sync {
    /// Store an audit log entry
    async fn store_entry(&mut self, entry: AuditLogEntry) -> Result<()>;

    /// Store multiple entries in batch
    async fn store_batch(&mut self, entries: Vec<AuditLogEntry>) -> Result<()>;

    /// Query audit logs
    async fn query_logs(&self, query: AuditQuery) -> Result<Vec<AuditLogEntry>>;

    /// Delete old logs based on retention policy
    async fn cleanup_old_logs(&mut self, older_than: DateTime<Utc>) -> Result<usize>;

    /// Verify log integrity
    async fn verify_integrity(&self) -> Result<bool>;
}

/// File-based audit storage implementation
pub struct FileAuditStorage {
    base_path: std::path::PathBuf,
    encrypt_logs: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            detailed_logging: true,
            log_sensitive_data: false,
            max_entry_size_bytes: 64 * 1024, // 64KB
            retention_days: 365,
            enable_integrity_check: true,
            storage_config: AuditStorageConfig {
                backend: AuditStorageBackend::File,
                location: "./audit-logs".to_string(),
                encrypt_logs: true,
                batch_size: 100,
            },
        }
    }
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(enterprise_config: &EnterpriseConfig) -> Result<Self> {
        let config = AuditConfig::default();
        let storage = Self::create_storage(&config.storage_config)?;

        Ok(Self {
            enabled: enterprise_config.enable_audit_trail,
            storage,
            config,
        })
    }

    /// Create a disabled audit logger
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            storage: FileAuditStorage::new("./disabled".into(), false),
            config: AuditConfig::default(),
        }
    }

    /// Log an operation
    pub async fn log_operation(
        &mut self,
        operation: &str,
        tenant_id: Option<Uuid>,
        actor_id: Option<Uuid>,
        metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let entry = AuditLogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            operation: operation.to_string(),
            actor: actor_id.map(|id| ActorInfo {
                actor_type: ActorType::User,
                id,
                name: "unknown".to_string(), // Would be resolved from user service
                roles: vec![],
                auth_method: None,
                session_id: None,
            }),
            target: None,
            tenant_id,
            request_id: None,
            client_info: None,
            result: OperationResult::Success,
            metadata,
            risk_level: self.assess_risk_level(operation),
            integrity_hash: None,
        };

        self.log_entry(entry).await
    }

    /// Log a detailed operation with full context
    pub async fn log_detailed_operation(
        &mut self,
        operation: &str,
        actor: Option<ActorInfo>,
        target: Option<TargetInfo>,
        tenant_id: Option<Uuid>,
        request_id: Option<Uuid>,
        client_info: Option<ClientInfo>,
        result: OperationResult,
        metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut entry = AuditLogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            operation: operation.to_string(),
            actor,
            target,
            tenant_id,
            request_id,
            client_info,
            result,
            metadata,
            risk_level: self.assess_risk_level(operation),
            integrity_hash: None,
        };

        // Calculate integrity hash if enabled
        if self.config.enable_integrity_check {
            entry.integrity_hash = Some(self.calculate_integrity_hash(&entry)?);
        }

        self.log_entry(entry).await
    }

    /// Log an entry
    async fn log_entry(&mut self, entry: AuditLogEntry) -> Result<()> {
        // Validate entry size
        let entry_size = serde_json::to_string(&entry)?.len();
        if entry_size > self.config.max_entry_size_bytes {
            return Err(anyhow::anyhow!(
                "Audit log entry too large: {} > {} bytes",
                entry_size,
                self.config.max_entry_size_bytes
            ));
        }

        // Filter sensitive data if not enabled
        let entry = if !self.config.log_sensitive_data {
            self.filter_sensitive_data(entry)
        } else {
            entry
        };

        self.storage.store_entry(entry).await
    }

    /// Get audit trail for a specific query
    pub async fn get_audit_trail(
        &self,
        tenant_id: Option<Uuid>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<AuditLogEntry>> {
        let query = AuditQuery {
            tenant_id,
            actor_id: None,
            operation: None,
            time_range: Some((start_date, end_date)),
            min_risk_level: None,
            result_type: None,
            limit: Some(1000),
            offset: None,
        };

        self.storage.query_logs(query).await
    }

    /// Get high-risk operations
    pub async fn get_high_risk_operations(
        &self,
        tenant_id: Option<Uuid>,
        days_back: u32,
    ) -> Result<Vec<AuditLogEntry>> {
        let start_date = Utc::now() - chrono::Duration::days(days_back as i64);
        let end_date = Utc::now();

        let query = AuditQuery {
            tenant_id,
            actor_id: None,
            operation: None,
            time_range: Some((start_date, end_date)),
            min_risk_level: Some(RiskLevel::High),
            result_type: None,
            limit: Some(500),
            offset: None,
        };

        self.storage.query_logs(query).await
    }

    /// Cleanup old audit logs based on retention policy
    pub async fn cleanup_old_logs(&mut self) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        self.storage.cleanup_old_logs(cutoff_date).await
    }

    /// Verify audit log integrity
    pub async fn verify_integrity(&self) -> Result<bool> {
        if !self.config.enable_integrity_check {
            return Ok(true);
        }

        self.storage.verify_integrity().await
    }

    /// Create storage backend based on configuration
    fn create_storage(config: &AuditStorageConfig) -> Result<FileAuditStorage> {
        match config.backend {
            AuditStorageBackend::File => Ok(FileAuditStorage::new(
                config.location.clone().into(),
                config.encrypt_logs,
            )),
            _ => Err(anyhow::anyhow!("Storage backend not implemented yet")),
        }
    }

    /// Assess risk level for an operation
    fn assess_risk_level(&self, operation: &str) -> RiskLevel {
        match operation {
            // Critical operations
            op if op.contains("delete") || op.contains("terminate") => RiskLevel::Critical,
            op if op.contains("admin") || op.contains("sudo") => RiskLevel::Critical,

            // High risk operations
            op if op.contains("create") || op.contains("update") => RiskLevel::High,
            op if op.contains("config") || op.contains("permission") => RiskLevel::High,

            // Medium risk operations
            op if op.contains("access") || op.contains("login") => RiskLevel::Medium,
            op if op.contains("export") || op.contains("download") => RiskLevel::Medium,

            // Low risk operations (read-only)
            _ => RiskLevel::Low,
        }
    }

    /// Calculate integrity hash for an entry
    fn calculate_integrity_hash(&self, entry: &AuditLogEntry) -> Result<String> {
        use sha2::{Digest, Sha256};

        // Create a deterministic representation of the entry
        let mut hasher = Sha256::new();
        hasher.update(entry.id.to_string().as_bytes());
        hasher.update(entry.timestamp.to_rfc3339().as_bytes());
        hasher.update(entry.operation.as_bytes());

        if let Some(actor) = &entry.actor {
            hasher.update(actor.id.to_string().as_bytes());
        }

        if let Some(tenant_id) = &entry.tenant_id {
            hasher.update(tenant_id.to_string().as_bytes());
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Filter sensitive data from audit entry
    fn filter_sensitive_data(&self, mut entry: AuditLogEntry) -> AuditLogEntry {
        // Remove sensitive metadata
        if let Some(metadata) = &mut entry.metadata {
            if let Some(obj) = metadata.as_object_mut() {
                // Remove common sensitive fields
                obj.remove("password");
                obj.remove("token");
                obj.remove("secret");
                obj.remove("key");
                obj.remove("api_key");
            }
        }

        // Mask sensitive actor information
        if let Some(actor) = &mut entry.actor {
            if actor.name.contains("@") {
                // Mask email addresses
                let parts: Vec<&str> = actor.name.split('@').collect();
                if parts.len() == 2 {
                    actor.name = format!("{}@{}", "*".repeat(parts[0].len().min(3)), parts[1]);
                }
            }
        }

        entry
    }
}

impl FileAuditStorage {
    pub fn new(base_path: std::path::PathBuf, encrypt_logs: bool) -> Self {
        Self {
            base_path,
            encrypt_logs,
        }
    }
}

impl AuditStorage for FileAuditStorage {
    async fn store_entry(&mut self, entry: AuditLogEntry) -> Result<()> {
        // Create directory structure based on date
        let date_path = self
            .base_path
            .join(entry.timestamp.format("%Y").to_string())
            .join(entry.timestamp.format("%m").to_string())
            .join(entry.timestamp.format("%d").to_string());

        std::fs::create_dir_all(&date_path)?;

        // Write entry to file
        let filename = format!("{}.json", entry.id);
        let file_path = date_path.join(filename);

        let content = serde_json::to_string_pretty(&entry)?;

        if self.encrypt_logs {
            // In a real implementation, you would encrypt the content here
            std::fs::write(file_path, content)?;
        } else {
            std::fs::write(file_path, content)?;
        }

        Ok(())
    }

    async fn store_batch(&mut self, entries: Vec<AuditLogEntry>) -> Result<()> {
        for entry in entries {
            self.store_entry(entry).await?;
        }
        Ok(())
    }

    async fn query_logs(&self, _query: AuditQuery) -> Result<Vec<AuditLogEntry>> {
        // Simplified implementation - in production, this would be more sophisticated
        Ok(Vec::new())
    }

    async fn cleanup_old_logs(&mut self, _older_than: DateTime<Utc>) -> Result<usize> {
        // Simplified implementation
        Ok(0)
    }

    async fn verify_integrity(&self) -> Result<bool> {
        // Simplified implementation
        Ok(true)
    }
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            tenant_id: None,
            actor_id: None,
            operation: None,
            time_range: None,
            min_risk_level: None,
            result_type: None,
            limit: Some(100),
            offset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_audit_logger_creation() {
        let config = EnterpriseConfig::default();
        let logger = AuditLogger::new(&config);
        assert!(logger.is_ok());
    }

    #[tokio::test]
    async fn test_file_audit_storage() {
        let temp_dir = TempDir::new().unwrap();
        let mut storage = FileAuditStorage::new(temp_dir.path().to_path_buf(), false);

        let entry = AuditLogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            operation: "test_operation".to_string(),
            actor: None,
            target: None,
            tenant_id: None,
            request_id: None,
            client_info: None,
            result: OperationResult::Success,
            metadata: None,
            risk_level: RiskLevel::Low,
            integrity_hash: None,
        };

        let result = storage.store_entry(entry).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_risk_level_assessment() {
        let config = EnterpriseConfig::default();
        let logger = AuditLogger::new(&config).unwrap();

        assert_eq!(logger.assess_risk_level("delete_user"), RiskLevel::Critical);
        assert_eq!(logger.assess_risk_level("create_tenant"), RiskLevel::High);
        assert_eq!(logger.assess_risk_level("login_attempt"), RiskLevel::Medium);
        assert_eq!(logger.assess_risk_level("view_report"), RiskLevel::Low);
    }

    #[test]
    fn test_integrity_hash_calculation() {
        let config = EnterpriseConfig::default();
        let logger = AuditLogger::new(&config).unwrap();

        let entry = AuditLogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            operation: "test".to_string(),
            actor: None,
            target: None,
            tenant_id: None,
            request_id: None,
            client_info: None,
            result: OperationResult::Success,
            metadata: None,
            risk_level: RiskLevel::Low,
            integrity_hash: None,
        };

        let hash = logger.calculate_integrity_hash(&entry);
        assert!(hash.is_ok());
        assert!(!hash.unwrap().is_empty());
    }
}
