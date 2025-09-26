//! # Enterprise Features Module
//!
//! This module provides enterprise-grade features for CodeGuardian output systems,
//! including audit trails, compliance reporting, multi-tenant isolation, and data encryption.

pub mod access_control;
pub mod audit;
pub mod compliance;
pub mod encryption;
pub mod multi_tenant;

use crate::types::AnalysisResults;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Enterprise configuration for advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    /// Enable audit trail logging
    pub enable_audit_trail: bool,
    /// Enable compliance reporting
    pub enable_compliance_reporting: bool,
    /// Enable multi-tenant isolation
    pub enable_multi_tenant: bool,
    /// Enable data encryption at rest
    pub enable_encryption: bool,
    /// Audit retention period in days
    pub audit_retention_days: u32,
    /// Compliance frameworks to support
    pub compliance_frameworks: Vec<ComplianceFramework>,
    /// Encryption configuration
    pub encryption_config: encryption::EncryptionConfig,
    /// Access control settings
    pub access_control: access_control::AccessControlConfig,
}

/// Supported compliance frameworks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceFramework {
    SOC2,
    ISO27001,
    GDPR,
    HIPAA,
    PciDss,
    NIST,
    Custom(String),
}

/// Enterprise tenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique tenant identifier
    pub id: Uuid,
    /// Tenant name
    pub name: String,
    /// Tenant organization
    pub organization: String,
    /// Tenant subscription tier
    pub subscription_tier: SubscriptionTier,
    /// Tenant-specific configuration
    pub config: TenantConfig,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Tenant status
    pub status: TenantStatus,
}

/// Subscription tiers for different feature access
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionTier {
    Basic,
    Professional,
    Enterprise,
    Custom(String),
}

/// Tenant-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    /// Maximum storage quota in GB
    pub storage_quota_gb: u64,
    /// Maximum API requests per hour
    pub api_rate_limit: u32,
    /// Enabled features for this tenant
    pub enabled_features: Vec<String>,
    /// Custom security policies
    pub security_policies: HashMap<String, serde_json::Value>,
    /// Data retention policy in days
    pub data_retention_days: u32,
}

/// Tenant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    Active,
    Suspended,
    Terminated,
    Migrating,
}

/// Enterprise context for operations
#[derive(Debug, Clone)]
pub struct EnterpriseContext {
    /// Current tenant
    pub tenant: Option<Tenant>,
    /// User performing the operation
    pub user: Option<User>,
    /// Operation being performed
    pub operation: String,
    /// Request ID for tracing
    pub request_id: Uuid,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
}

/// User information for access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// User roles
    pub roles: Vec<String>,
    /// User permissions
    pub permissions: Vec<String>,
    /// Tenant association
    pub tenant_id: Option<Uuid>,
}

/// Enterprise manager for coordinating enterprise features
pub struct EnterpriseManager {
    config: EnterpriseConfig,
    audit_logger: audit::AuditLogger,
    compliance_reporter: compliance::ComplianceReporter,
    tenant_manager: multi_tenant::TenantManager,
    encryption_service: encryption::EncryptionService,
    access_controller: access_control::AccessController,
}

impl Default for EnterpriseConfig {
    fn default() -> Self {
        Self {
            enable_audit_trail: true,
            enable_compliance_reporting: true,
            enable_multi_tenant: false, // Disabled by default
            enable_encryption: true,
            audit_retention_days: 365,
            compliance_frameworks: vec![ComplianceFramework::SOC2, ComplianceFramework::ISO27001],
            encryption_config: encryption::EncryptionConfig::default(),
            access_control: access_control::AccessControlConfig::default(),
        }
    }
}

impl Default for TenantConfig {
    fn default() -> Self {
        Self {
            storage_quota_gb: 100,
            api_rate_limit: 1000,
            enabled_features: vec![
                "basic_analysis".to_string(),
                "output_formatting".to_string(),
                "api_access".to_string(),
            ],
            security_policies: HashMap::new(),
            data_retention_days: 90,
        }
    }
}

impl EnterpriseManager {
    /// Create a new enterprise manager
    pub fn new(config: EnterpriseConfig) -> Result<Self> {
        let audit_logger = if config.enable_audit_trail {
            audit::AuditLogger::new(&config)?
        } else {
            audit::AuditLogger::disabled()
        };

        let compliance_reporter = if config.enable_compliance_reporting {
            compliance::ComplianceReporter::new(&config)?
        } else {
            compliance::ComplianceReporter::disabled()
        };

        let tenant_manager = if config.enable_multi_tenant {
            multi_tenant::TenantManager::new(&config)?
        } else {
            multi_tenant::TenantManager::single_tenant()
        };

        let encryption_service = if config.enable_encryption {
            encryption::EncryptionService::new(&config.encryption_config)?
        } else {
            encryption::EncryptionService::disabled()
        };

        let access_controller = access_control::AccessController::new(&config.access_control)?;

        Ok(Self {
            config,
            audit_logger,
            compliance_reporter,
            tenant_manager,
            encryption_service,
            access_controller,
        })
    }

    /// Process analysis results with enterprise features
    pub async fn process_analysis_results(
        &mut self,
        results: &AnalysisResults,
        context: &EnterpriseContext,
    ) -> Result<EnterpriseProcessingResult> {
        // Validate access permissions
        self.access_controller
            .validate_access(context, "process_analysis")
            .await?;

        // Log audit trail
        self.audit_logger
            .log_operation(
                &context.operation,
                context.tenant.as_ref().map(|t| t.id),
                context.user.as_ref().map(|u| u.id),
                Some(serde_json::json!({
                    "findings_count": results.findings.len(),
                    "request_id": context.request_id,
                })),
            )
            .await?;

        // Apply tenant-specific processing
        let processed_results = if let Some(tenant) = &context.tenant {
            self.tenant_manager
                .apply_tenant_policies(results, tenant)
                .await?
        } else {
            results.clone()
        };

        // Generate compliance data if required
        let compliance_data = if self.config.enable_compliance_reporting {
            Some(
                self.compliance_reporter
                    .generate_compliance_data(&processed_results)
                    .await?,
            )
        } else {
            None
        };

        // Encrypt sensitive data if required
        let encrypted_data = if self.config.enable_encryption {
            Some(
                self.encryption_service
                    .encrypt_analysis_results(&processed_results)
                    .await?,
            )
        } else {
            None
        };

        Ok(EnterpriseProcessingResult {
            processed_results,
            compliance_data,
            encrypted_data,
            audit_trail_id: Some(Uuid::new_v4()),
            tenant_id: context.tenant.as_ref().map(|t| t.id),
            processing_metadata: ProcessingMetadata {
                processed_at: Utc::now(),
                processing_duration_ms: 0, // Would be calculated in real implementation
                features_applied: self.get_applied_features(context),
            },
        })
    }

    /// Create a new tenant
    pub async fn create_tenant(
        &mut self,
        name: String,
        organization: String,
        subscription_tier: SubscriptionTier,
        context: &EnterpriseContext,
    ) -> Result<Tenant> {
        // Validate admin permissions
        self.access_controller
            .validate_access(context, "create_tenant")
            .await?;

        let tenant = self
            .tenant_manager
            .create_tenant(name, organization, subscription_tier)
            .await?;

        // Log tenant creation
        self.audit_logger
            .log_operation(
                "create_tenant",
                Some(tenant.id),
                context.user.as_ref().map(|u| u.id),
                Some(serde_json::json!({
                    "tenant_name": tenant.name,
                    "organization": tenant.organization,
                    "subscription_tier": tenant.subscription_tier,
                })),
            )
            .await?;

        Ok(tenant)
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(
        &mut self,
        framework: ComplianceFramework,
        context: &EnterpriseContext,
    ) -> Result<compliance::ComplianceReport> {
        // Validate compliance access
        self.access_controller
            .validate_access(context, "generate_compliance_report")
            .await?;

        let report = self
            .compliance_reporter
            .generate_report(&framework, context.tenant.as_ref())
            .await?;

        // Log compliance report generation
        self.audit_logger
            .log_operation(
                "generate_compliance_report",
                context.tenant.as_ref().map(|t| t.id),
                context.user.as_ref().map(|u| u.id),
                Some(serde_json::json!({
                    "framework": framework.clone(),
                    "report_id": report.id,
                })),
            )
            .await?;

        Ok(report)
    }

    /// Get audit trail for a tenant
    pub async fn get_audit_trail(
        &self,
        tenant_id: Option<Uuid>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        context: &EnterpriseContext,
    ) -> Result<Vec<audit::AuditLogEntry>> {
        // Validate audit access
        self.access_controller
            .validate_access(context, "view_audit_trail")
            .await?;

        self.audit_logger
            .get_audit_trail(tenant_id, start_date, end_date)
            .await
    }

    /// Get tenant usage statistics
    pub async fn get_tenant_usage_stats(
        &self,
        tenant_id: Uuid,
        context: &EnterpriseContext,
    ) -> Result<TenantUsageStats> {
        // Validate tenant access
        self.access_controller
            .validate_tenant_access(context, tenant_id)
            .await?;

        self.tenant_manager.get_usage_stats(tenant_id).await
    }

    /// Update tenant configuration
    pub async fn update_tenant_config(
        &mut self,
        tenant_id: Uuid,
        new_config: TenantConfig,
        context: &EnterpriseContext,
    ) -> Result<()> {
        // Validate admin permissions
        self.access_controller
            .validate_access(context, "update_tenant_config")
            .await?;

        self.tenant_manager
            .update_tenant_config(tenant_id, new_config)
            .await?;

        // Log configuration update
        self.audit_logger
            .log_operation(
                "update_tenant_config",
                Some(tenant_id),
                context.user.as_ref().map(|u| u.id),
                Some(serde_json::json!({
                    "tenant_id": tenant_id,
                })),
            )
            .await?;

        Ok(())
    }

    /// Get applied features for context
    fn get_applied_features(&self, context: &EnterpriseContext) -> Vec<String> {
        let mut features = Vec::new();

        if self.config.enable_audit_trail {
            features.push("audit_trail".to_string());
        }
        if self.config.enable_compliance_reporting {
            features.push("compliance_reporting".to_string());
        }
        if self.config.enable_multi_tenant {
            features.push("multi_tenant".to_string());
        }
        if self.config.enable_encryption {
            features.push("encryption".to_string());
        }

        // Add tenant-specific features
        if let Some(tenant) = &context.tenant {
            features.extend(tenant.config.enabled_features.clone());
        }

        features
    }
}

/// Result of enterprise processing
#[derive(Debug, Clone)]
pub struct EnterpriseProcessingResult {
    pub processed_results: AnalysisResults,
    pub compliance_data: Option<compliance::ComplianceData>,
    pub encrypted_data: Option<encryption::EncryptedData>,
    pub audit_trail_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub processing_metadata: ProcessingMetadata,
}

/// Metadata about enterprise processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetadata {
    pub processed_at: DateTime<Utc>,
    pub processing_duration_ms: u64,
    pub features_applied: Vec<String>,
}

/// Tenant usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUsageStats {
    pub tenant_id: Uuid,
    pub storage_used_gb: f64,
    pub api_requests_today: u32,
    pub api_requests_this_month: u32,
    pub total_analyses: u64,
    pub total_findings: u64,
    pub last_activity: DateTime<Utc>,
    pub quota_utilization: QuotaUtilization,
}

/// Quota utilization information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaUtilization {
    pub storage_percent: f64,
    pub api_requests_percent: f64,
    pub is_over_quota: bool,
    pub warnings: Vec<String>,
}

impl Tenant {
    /// Create a new tenant
    pub fn new(name: String, organization: String, subscription_tier: SubscriptionTier) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name,
            organization,
            subscription_tier,
            config: TenantConfig::default(),
            created_at: now,
            updated_at: now,
            status: TenantStatus::Active,
        }
    }

    /// Check if tenant has a specific feature enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.config.enabled_features.contains(&feature.to_string())
    }

    /// Check if tenant is within quota limits
    pub fn is_within_quota(&self, usage_stats: &TenantUsageStats) -> bool {
        !usage_stats.quota_utilization.is_over_quota
    }
}

impl EnterpriseContext {
    /// Create a new enterprise context
    pub fn new(operation: String) -> Self {
        Self {
            tenant: None,
            user: None,
            operation,
            request_id: Uuid::new_v4(),
            client_ip: None,
            user_agent: None,
        }
    }

    /// Set tenant for the context
    pub fn with_tenant(mut self, tenant: Tenant) -> Self {
        self.tenant = Some(tenant);
        self
    }

    /// Set user for the context
    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// Set client information
    pub fn with_client_info(mut self, ip: Option<String>, user_agent: Option<String>) -> Self {
        self.client_ip = ip;
        self.user_agent = user_agent;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_creation() -> Result<(), Box<dyn std::error::Error>> {
        let tenant = Tenant::new(
            "Test Tenant".to_string(),
            "Test Organization".to_string(),
            SubscriptionTier::Professional,
        );

        assert_eq!(tenant.name, "Test Tenant");
        assert_eq!(tenant.organization, "Test Organization");
        assert_eq!(tenant.subscription_tier, SubscriptionTier::Professional);
        assert_eq!(tenant.status, TenantStatus::Active);
    }

    #[test]
    fn test_enterprise_context_creation() -> Result<(), Box<dyn std::error::Error>> {
        let context = EnterpriseContext::new("test_operation".to_string());
        assert_eq!(context.operation, "test_operation");
        assert!(context.tenant.is_none());
        assert!(context.user.is_none());
    }

    #[test]
    fn test_tenant_feature_check() -> Result<(), Box<dyn std::error::Error>> {
        let mut tenant = Tenant::new(
            "Test Tenant".to_string(),
            "Test Org".to_string(),
            SubscriptionTier::Basic,
        );

        tenant
            .config
            .enabled_features
            .push("advanced_analysis".to_string());

        assert!(tenant.has_feature("advanced_analysis"));
        assert!(!tenant.has_feature("premium_feature"));
    }

    #[tokio::test]
    async fn test_enterprise_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = EnterpriseConfig::default();
        let manager = EnterpriseManager::new(config);
        assert!(manager.is_ok());
    }
}
