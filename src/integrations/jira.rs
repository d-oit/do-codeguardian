//! Jira integration for CodeGuardian

use super::traits::*;
use super::SystemConfig;
use crate::config::base::Config;
use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Jira client implementation
pub struct JiraClient {
    config: SystemConfig,
    client: Client,
    base_url: String,
}

impl JiraClient {
    pub async fn new(config: SystemConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            base_url: config.base_url.clone(),
            config,
            client,
        })
    }

    fn get_auth_header(&self) -> Result<String> {
        match &self.config.auth {
            super::AuthConfig::BasicAuth { username, token } => {
                let credentials = BASE64.encode(format!("{}:{}", username, token));
                Ok(format!("Basic {}", credentials))
            }
            super::AuthConfig::Token { token } => Ok(format!("Bearer {}", token)),
            _ => Err(anyhow::anyhow!("Unsupported auth type for Jira")),
        }
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/rest/api/3/{}", self.base_url, endpoint);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<T>().await?;
            Ok(data)
        } else {
            Err(anyhow::anyhow!("Jira API error: {}", response.status()))
        }
    }

    async fn post_request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<R> {
        let url = format!("{}/rest/api/3/{}", self.base_url, endpoint);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<R>().await?;
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Jira API error: {}", response.status()))
        }
    }
}

#[async_trait]
impl ExternalSystemClient for JiraClient {
    fn system_name(&self) -> &str {
        "jira"
    }

    async fn health_check(&self) -> Result<SystemHealth> {
        let start = std::time::Instant::now();

        match self.make_request::<JiraServerInfo>("serverInfo").await {
            Ok(_) => {
                let response_time = start.elapsed().as_millis() as u64;
                Ok(SystemHealth {
                    status: HealthStatus::Healthy,
                    response_time_ms: Some(response_time),
                    last_error: None,
                    features_available: vec![
                        "issue_creation".to_string(),
                        "duplicate_search".to_string(),
                        "reporting".to_string(),
                    ],
                })
            }
            Err(e) => Ok(SystemHealth {
                status: HealthStatus::Unhealthy,
                response_time_ms: None,
                last_error: Some(e.to_string()),
                features_available: vec![],
            }),
        }
    }

    async fn search_duplicates(
        &self,
        query: &DuplicateSearchQuery,
    ) -> Result<Vec<DuplicateSearchResult>> {
        let jql = format!(
            "text ~ \"{}\" AND project = \"{}\" ORDER BY created DESC",
            query.title,
            query.project_key.as_deref().unwrap_or("*")
        );

        let search_request = JiraSearchRequest {
            jql,
            max_results: query.max_results,
            fields: vec![
                "summary".to_string(),
                "description".to_string(),
                "status".to_string(),
                "created".to_string(),
                "updated".to_string(),
            ],
        };

        let search_result: JiraSearchResponse =
            self.post_request("search", &search_request).await?;

        let mut results = Vec::new();
        for issue in search_result.issues {
            let similarity_score = calculate_similarity(&query.title, &issue.fields.summary);

            if similarity_score >= query.similarity_threshold {
                results.push(DuplicateSearchResult {
                    id: issue.id,
                    title: issue.fields.summary,
                    description: issue.fields.description,
                    url: format!("{}/browse/{}", self.base_url, issue.key),
                    status: issue.fields.status.name,
                    created_at: issue.fields.created,
                    updated_at: issue.fields.updated,
                    relevance_score: similarity_score,
                    similarity_score,
                    source_system: "jira".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        Ok(results)
    }

    async fn create_issue(&self, issue: &IssueCreationRequest) -> Result<CreatedIssue> {
        let jira_issue = JiraIssueCreation {
            fields: JiraIssueFields {
                project: JiraProject {
                    key: issue.project_key.clone().unwrap_or_default(),
                },
                summary: issue.title.clone(),
                description: Some(issue.description.clone()),
                issuetype: JiraIssueType {
                    name: issue.issue_type.clone(),
                },
                priority: Some(JiraPriority {
                    name: match issue.priority {
                        IssuePriority::Critical => "Highest".to_string(),
                        IssuePriority::High => "High".to_string(),
                        IssuePriority::Medium => "Medium".to_string(),
                        IssuePriority::Low => "Low".to_string(),
                        IssuePriority::Trivial => "Lowest".to_string(),
                    },
                }),
                labels: if issue.labels.is_empty() {
                    None
                } else {
                    Some(issue.labels.clone())
                },
            },
        };

        let created: JiraCreatedIssue = self.post_request("issue", &jira_issue).await?;

        Ok(CreatedIssue {
            id: created.id,
            key: Some(created.key.clone()),
            url: format!("{}/browse/{}", self.base_url, created.key),
            status: "Open".to_string(),
            created_at: Utc::now(),
        })
    }

    async fn update_issue(&self, issue_id: &str, update: &IssueUpdate) -> Result<()> {
        let mut fields = HashMap::new();

        if let Some(title) = &update.title {
            fields.insert("summary", serde_json::Value::String(title.clone()));
        }

        if let Some(description) = &update.description {
            fields.insert(
                "description",
                serde_json::Value::String(description.clone()),
            );
        }

        let update_request = serde_json::json!({
            "fields": fields
        });

        let url = format!("issue/{}", issue_id);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .put(format!("{}/rest/api/3/{}", self.base_url, url))
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&update_request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to update Jira issue: {}",
                response.status()
            ))
        }
    }

    async fn close_issue(&self, issue_id: &str, resolution: &IssueResolution) -> Result<()> {
        let transition_request = serde_json::json!({
            "transition": {
                "id": "2" // Standard "Close Issue" transition
            },
            "fields": {
                "resolution": {
                    "name": resolution.resolution_type
                }
            }
        });

        let url = format!("issue/{}/transitions", issue_id);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .post(format!("{}/rest/api/3/{}", self.base_url, url))
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&transition_request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to close Jira issue: {}",
                response.status()
            ))
        }
    }

    async fn trigger_workflow(
        &self,
        _request: &WorkflowTriggerRequest,
    ) -> Result<TriggeredWorkflow> {
        // Jira doesn't have traditional workflows like CI/CD systems
        // This could be implemented as triggering automation rules or transitions
        Err(anyhow::anyhow!("Workflow triggering not supported in Jira"))
    }

    async fn generate_report(&self, request: &ReportRequest) -> Result<SystemReport> {
        let jql = match request.report_type.as_str() {
            "duplicates" => "labels = \"duplicate\" OR resolution = \"Duplicate\"".to_string(),
            "all_issues" => "project is not EMPTY".to_string(),
            _ => format!("text ~ \"{}\"", request.report_type),
        };

        let search_request = JiraSearchRequest {
            jql,
            max_results: 1000,
            fields: vec![
                "summary".to_string(),
                "status".to_string(),
                "created".to_string(),
            ],
        };

        let search_result: JiraSearchResponse =
            self.post_request("search", &search_request).await?;

        let total_issues = search_result.total as u64;
        let duplicates_found = search_result
            .issues
            .iter()
            .filter(|issue| {
                issue
                    .fields
                    .status
                    .name
                    .to_lowercase()
                    .contains("duplicate")
            })
            .count() as u64;

        Ok(SystemReport {
            system_name: "jira".to_string(),
            report_type: request.report_type.clone(),
            generated_at: Utc::now(),
            total_issues,
            duplicates_found,
            duplicate_rate: if total_issues > 0 {
                (duplicates_found as f64 / total_issues as f64) * 100.0
            } else {
                0.0
            },
            time_period: request
                .start_date
                .zip(request.end_date)
                .map(|(start, end)| super::traits::TimePeriod { start, end }),
            metrics: HashMap::new(),
            details: vec![],
        })
    }

    fn get_capabilities(&self) -> SystemCapabilities {
        SystemCapabilities {
            supports_issue_creation: true,
            supports_issue_updates: true,
            supports_duplicate_search: true,
            supports_workflow_triggers: false,
            supports_reporting: true,
            supports_webhooks: true,
            supports_bulk_operations: true,
            max_batch_size: Some(50),
            rate_limits: Some(RateLimitInfo {
                requests_per_minute: 60,
                burst_limit: 10,
                current_usage: None,
                reset_time: None,
            }),
        }
    }
}

// Helper function to calculate text similarity
fn calculate_similarity(text1: &str, text2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

// Jira API data structures
#[derive(Debug, Deserialize)]
struct JiraServerInfo {
    #[serde(rename = "serverTitle")]
    _server_title: String,
    _version: String,
}

#[derive(Debug, Serialize)]
struct JiraSearchRequest {
    jql: String,
    #[serde(rename = "maxResults")]
    max_results: usize,
    fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JiraSearchResponse {
    total: usize,
    issues: Vec<JiraIssue>,
}

#[derive(Debug, Deserialize)]
struct JiraIssue {
    id: String,
    key: String,
    fields: JiraIssueFieldsResponse,
}

#[derive(Debug, Deserialize)]
struct JiraIssueFieldsResponse {
    summary: String,
    description: Option<String>,
    status: JiraStatus,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct JiraStatus {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct JiraIssueCreation {
    fields: JiraIssueFields,
}

impl JiraIssueCreation {
    pub fn new(_config: &Config) -> Result<Self> {
        // TODO: implement
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[derive(Debug, Serialize)]
struct JiraIssueFields {
    project: JiraProject,
    summary: String,
    description: Option<String>,
    issuetype: JiraIssueType,
    priority: Option<JiraPriority>,
    labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct JiraProject {
    key: String,
}

#[derive(Debug, Serialize)]
struct JiraIssueType {
    name: String,
}

#[derive(Debug, Serialize)]
struct JiraPriority {
    name: String,
}

#[derive(Debug, Deserialize)]
struct JiraCreatedIssue {
    id: String,
    key: String,
}
