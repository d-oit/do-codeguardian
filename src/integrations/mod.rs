//! External system integrations for CodeGuardian
//!
//! Provides unified interfaces for integrating with various external systems
//! including issue trackers, documentation platforms, and CI/CD systems.

pub mod azure_devops;
pub mod bitbucket;
pub mod confluence;
pub mod gitlab;
pub mod jenkins;
pub mod jira;
pub mod traits;

use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use traits::*;

/// Configuration for external system integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    pub enabled: bool,
    pub systems: HashMap<String, SystemConfig>,
    pub default_timeout_seconds: u64,
    pub retry_attempts: u32,
    pub batch_size: usize,
}

impl Default for IntegrationsConfig {
    fn default() -> Self {
        let mut systems = HashMap::new();

        // Add default configurations for supported systems
        systems.insert("jira".to_string(), SystemConfig::jira_default());
        systems.insert("confluence".to_string(), SystemConfig::confluence_default());
        systems.insert("jenkins".to_string(), SystemConfig::jenkins_default());
        systems.insert("gitlab".to_string(), SystemConfig::gitlab_default());
        systems.insert("bitbucket".to_string(), SystemConfig::bitbucket_default());
        systems.insert(
            "azure_devops".to_string(),
            SystemConfig::azure_devops_default(),
        );

        Self {
            enabled: false,
            systems,
            default_timeout_seconds: 30,
            retry_attempts: 3,
            batch_size: 50,
        }
    }
}

/// Configuration for individual external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub enabled: bool,
    pub base_url: String,
    pub auth: AuthConfig,
    pub features: SystemFeatures,
    pub rate_limits: RateLimits,
    pub custom_fields: HashMap<String, String>,
}

impl SystemConfig {
    pub fn jira_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://your-domain.atlassian.net".to_string(),
            auth: AuthConfig::BasicAuth {
                username: "".to_string(),
                token: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: true,
                duplicate_detection: true,
                workflow_automation: true,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 60,
                burst_limit: 10,
            },
            custom_fields: HashMap::new(),
        }
    }

    pub fn confluence_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://your-domain.atlassian.net/wiki".to_string(),
            auth: AuthConfig::BasicAuth {
                username: "".to_string(),
                token: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: false,
                duplicate_detection: true,
                workflow_automation: false,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 60,
                burst_limit: 10,
            },
            custom_fields: HashMap::new(),
        }
    }

    pub fn jenkins_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://jenkins.example.com".to_string(),
            auth: AuthConfig::ApiKey {
                key: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: false,
                duplicate_detection: false,
                workflow_automation: true,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 120,
                burst_limit: 20,
            },
            custom_fields: HashMap::new(),
        }
    }

    pub fn gitlab_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://gitlab.com".to_string(),
            auth: AuthConfig::Token {
                token: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: true,
                duplicate_detection: true,
                workflow_automation: true,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 300,
                burst_limit: 50,
            },
            custom_fields: HashMap::new(),
        }
    }

    pub fn bitbucket_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://api.bitbucket.org/2.0".to_string(),
            auth: AuthConfig::OAuth {
                client_id: "".to_string(),
                client_secret: "".to_string(),
                access_token: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: true,
                duplicate_detection: true,
                workflow_automation: true,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 1000,
                burst_limit: 100,
            },
            custom_fields: HashMap::new(),
        }
    }

    pub fn azure_devops_default() -> Self {
        Self {
            enabled: false,
            base_url: "https://dev.azure.com".to_string(),
            auth: AuthConfig::Token {
                token: "".to_string(),
            },
            features: SystemFeatures {
                issue_tracking: true,
                duplicate_detection: true,
                workflow_automation: true,
                reporting: true,
                webhooks: true,
            },
            rate_limits: RateLimits {
                requests_per_minute: 300,
                burst_limit: 50,
            },
            custom_fields: HashMap::new(),
        }
    }
}

/// Authentication configuration for external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthConfig {
    BasicAuth {
        username: String,
        token: String,
    },
    Token {
        token: String,
    },
    ApiKey {
        key: String,
    },
    OAuth {
        client_id: String,
        client_secret: String,
        access_token: String,
    },
}

/// Features supported by external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemFeatures {
    pub issue_tracking: bool,
    pub duplicate_detection: bool,
    pub workflow_automation: bool,
    pub reporting: bool,
    pub webhooks: bool,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
}

/// Integration manager for coordinating external system interactions
pub struct IntegrationManager {
    config: IntegrationsConfig,
    clients: HashMap<String, Box<dyn ExternalSystemClient>>,
}

impl IntegrationManager {
    pub fn new(config: IntegrationsConfig) -> Self {
        Self {
            config,
            clients: HashMap::new(),
        }
    }

    /// Initialize all enabled integrations
    pub async fn initialize(&mut self) -> Result<()> {
        for (system_name, system_config) in &self.config.systems {
            if system_config.enabled {
                let client = self.create_client(system_name, system_config).await?;
                self.clients.insert(system_name.clone(), client);
                tracing::info!("Initialized integration for: {}", system_name);
            }
        }
        Ok(())
    }

    /// Create a client for a specific external system
    async fn create_client(
        &self,
        system_name: &str,
        config: &SystemConfig,
    ) -> Result<Box<dyn ExternalSystemClient>> {
        match system_name {
            "jira" => Ok(Box::new(jira::JiraClient::new(config.clone()).await?)),
            "confluence" => Ok(Box::new(
                confluence::ConfluenceClient::new(config.clone()).await?,
            )),
            "jenkins" => Ok(Box::new(jenkins::JenkinsClient::new(config.clone()).await?)),
            "gitlab" => Ok(Box::new(gitlab::GitLabClient::new(config.clone()).await?)),
            "bitbucket" => Ok(Box::new(
                bitbucket::BitbucketClient::new(config.clone()).await?,
            )),
            "azure_devops" => Ok(Box::new(
                azure_devops::AzureDevOpsClient::new(config.clone()).await?,
            )),
            _ => Err(anyhow::anyhow!("Unsupported system: {}", system_name)),
        }
    }

    /// Search for duplicates across all enabled systems
    pub async fn search_duplicates_across_systems(
        &self,
        query: &DuplicateSearchQuery,
    ) -> Result<Vec<DuplicateSearchResult>> {
        let mut all_results = Vec::new();

        for (system_name, client) in &self.clients {
            if let Some(system_config) = self.config.systems.get(system_name) {
                if system_config.features.duplicate_detection {
                    match client.search_duplicates(query).await {
                        Ok(mut results) => {
                            // Tag results with source system
                            for result in &mut results {
                                result.source_system = system_name.clone();
                            }
                            all_results.extend(results);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to search duplicates in {}: {}", system_name, e);
                        }
                    }
                }
            }
        }

        // Sort by relevance score
        all_results.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(all_results)
    }

    /// Create issues across multiple systems
    pub async fn create_issue_across_systems(
        &self,
        issue: &IssueCreationRequest,
    ) -> Result<Vec<IssueCreationResult>> {
        let mut results = Vec::new();

        for (system_name, client) in &self.clients {
            if let Some(system_config) = self.config.systems.get(system_name) {
                if system_config.features.issue_tracking {
                    match client.create_issue(issue).await {
                        Ok(result) => {
                            results.push(IssueCreationResult {
                                system: system_name.clone(),
                                success: true,
                                issue_id: Some(result.id),
                                issue_url: Some(result.url),
                                error: None,
                            });
                        }
                        Err(e) => {
                            results.push(IssueCreationResult {
                                system: system_name.clone(),
                                success: false,
                                issue_id: None,
                                issue_url: None,
                                error: Some(e.to_string()),
                            });
                            tracing::warn!("Failed to create issue in {}: {}", system_name, e);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Trigger workflows across multiple systems
    pub async fn trigger_workflows(
        &self,
        workflow_request: &WorkflowTriggerRequest,
    ) -> Result<Vec<WorkflowTriggerResult>> {
        let mut results = Vec::new();

        for (system_name, client) in &self.clients {
            if let Some(system_config) = self.config.systems.get(system_name) {
                if system_config.features.workflow_automation {
                    match client.trigger_workflow(workflow_request).await {
                        Ok(result) => {
                            results.push(WorkflowTriggerResult {
                                system: system_name.clone(),
                                success: true,
                                workflow_id: Some(result.id),
                                workflow_url: Some(result.url),
                                error: None,
                            });
                        }
                        Err(e) => {
                            results.push(WorkflowTriggerResult {
                                system: system_name.clone(),
                                success: false,
                                workflow_id: None,
                                workflow_url: None,
                                error: Some(e.to_string()),
                            });
                            tracing::warn!("Failed to trigger workflow in {}: {}", system_name, e);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Generate unified reports across all systems
    pub async fn generate_unified_report(
        &self,
        report_request: &ReportRequest,
    ) -> Result<UnifiedReport> {
        let mut system_reports = HashMap::new();

        for (system_name, client) in &self.clients {
            if let Some(system_config) = self.config.systems.get(system_name) {
                if system_config.features.reporting {
                    match client.generate_report(report_request).await {
                        Ok(report) => {
                            system_reports.insert(system_name.clone(), report);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to generate report from {}: {}", system_name, e);
                        }
                    }
                }
            }
        }

        Ok(UnifiedReport {
            generated_at: chrono::Utc::now(),
            report_type: report_request.report_type.clone(),
            system_reports: system_reports.clone(),
            summary: self.generate_report_summary(&system_reports),
        })
    }

    fn generate_report_summary(
        &self,
        system_reports: &HashMap<String, SystemReport>,
    ) -> ReportSummary {
        let total_issues = system_reports.values().map(|r| r.total_issues).sum();
        let total_duplicates = system_reports.values().map(|r| r.duplicates_found).sum();
        let systems_count = system_reports.len();

        ReportSummary {
            total_systems: systems_count,
            total_issues,
            total_duplicates,
            duplicate_rate: if total_issues > 0 {
                (total_duplicates as f64 / total_issues as f64) * 100.0
            } else {
                0.0
            },
            systems_with_data: system_reports.keys().cloned().collect(),
        }
    }

    /// Get integration health status
    pub async fn get_health_status(&self) -> Result<IntegrationHealthStatus> {
        let mut system_health = HashMap::new();

        for (system_name, client) in &self.clients {
            let health = client.health_check().await.unwrap_or(SystemHealth {
                status: HealthStatus::Unhealthy,
                response_time_ms: None,
                last_error: Some("Health check failed".to_string()),
                features_available: Vec::new(),
            });
            system_health.insert(system_name.clone(), health);
        }

        let healthy_systems = system_health
            .values()
            .filter(|h| h.status == HealthStatus::Healthy)
            .count();
        let total_systems = system_health.len();

        Ok(IntegrationHealthStatus {
            overall_status: if healthy_systems == total_systems {
                HealthStatus::Healthy
            } else if healthy_systems > 0 {
                HealthStatus::Degraded
            } else {
                HealthStatus::Unhealthy
            },
            system_health,
            healthy_systems,
            total_systems,
            last_checked: chrono::Utc::now(),
        })
    }
}

/// Result structures for integration operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCreationResult {
    pub system: String,
    pub success: bool,
    pub issue_id: Option<String>,
    pub issue_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTriggerResult {
    pub system: String,
    pub success: bool,
    pub workflow_id: Option<String>,
    pub workflow_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub report_type: String,
    pub system_reports: HashMap<String, SystemReport>,
    pub summary: ReportSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_systems: usize,
    pub total_issues: u64,
    pub total_duplicates: u64,
    pub duplicate_rate: f64,
    pub systems_with_data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHealthStatus {
    pub overall_status: HealthStatus,
    pub system_health: HashMap<String, SystemHealth>,
    pub healthy_systems: usize,
    pub total_systems: usize,
    pub last_checked: chrono::DateTime<chrono::Utc>,
}
