//! GitLab integration for CodeGuardian

use super::traits::*;
use super::SystemConfig;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GitLab client implementation
pub struct GitLabClient {
    config: SystemConfig,
    client: Client,
    base_url: String,
}

impl GitLabClient {
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
            super::AuthConfig::Token { token } => Ok(format!("Bearer {}", token)),
            _ => Err(anyhow::anyhow!("Unsupported auth type for GitLab")),
        }
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/api/v4/{}", self.base_url, endpoint);
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
            Err(anyhow::anyhow!("GitLab API error: {}", response.status()))
        }
    }

    async fn post_request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<R> {
        let url = format!("{}/api/v4/{}", self.base_url, endpoint);
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
            Err(anyhow::anyhow!("GitLab API error: {}", response.status()))
        }
    }
}

#[async_trait]
impl ExternalSystemClient for GitLabClient {
    fn system_name(&self) -> &str {
        "gitlab"
    }

    async fn health_check(&self) -> Result<SystemHealth> {
        let start = std::time::Instant::now();

        match self.make_request::<GitLabVersion>("version").await {
            Ok(_) => {
                let response_time = start.elapsed().as_millis() as u64;
                Ok(SystemHealth {
                    status: HealthStatus::Healthy,
                    response_time_ms: Some(response_time),
                    last_error: None,
                    features_available: vec![
                        "issue_tracking".to_string(),
                        "pipeline_triggers".to_string(),
                        "duplicate_detection".to_string(),
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
        let search_url = format!(
            "issues?search={}&per_page={}",
            urlencoding::encode(&query.title),
            query.max_results
        );

        let issues: Vec<GitLabIssue> = self.make_request(&search_url).await?;

        let mut results = Vec::new();
        for issue in issues {
            let similarity_score = calculate_similarity(&query.title, &issue.title);

            if similarity_score >= query.similarity_threshold {
                results.push(DuplicateSearchResult {
                    id: issue.iid.to_string(),
                    title: issue.title,
                    description: issue.description,
                    url: issue.web_url,
                    status: issue.state,
                    created_at: issue.created_at,
                    updated_at: issue.updated_at,
                    relevance_score: similarity_score,
                    similarity_score,
                    source_system: "gitlab".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        Ok(results)
    }

    async fn create_issue(&self, issue: &IssueCreationRequest) -> Result<CreatedIssue> {
        let gitlab_issue = GitLabIssueCreation {
            title: issue.title.clone(),
            description: Some(issue.description.clone()),
            labels: if issue.labels.is_empty() {
                None
            } else {
                Some(issue.labels.join(","))
            },
        };

        let project_id = issue.project_key.as_deref().unwrap_or("1");
        let created: GitLabIssue = self
            .post_request(&format!("projects/{}/issues", project_id), &gitlab_issue)
            .await?;

        Ok(CreatedIssue {
            id: created.iid.to_string(),
            key: Some(format!("#{}", created.iid)),
            url: created.web_url,
            status: created.state,
            created_at: created.created_at,
        })
    }

    async fn update_issue(&self, issue_id: &str, update: &IssueUpdate) -> Result<()> {
        let mut update_data = serde_json::Map::new();

        if let Some(title) = &update.title {
            update_data.insert(
                "title".to_string(),
                serde_json::Value::String(title.clone()),
            );
        }

        if let Some(description) = &update.description {
            update_data.insert(
                "description".to_string(),
                serde_json::Value::String(description.clone()),
            );
        }

        if let Some(status) = &update.status {
            let state_event = match status.as_str() {
                "closed" => "close",
                "open" => "reopen",
                _ => return Ok(()),
            };
            update_data.insert(
                "state_event".to_string(),
                serde_json::Value::String(state_event.to_string()),
            );
        }

        let project_id = "1"; // Would need to be passed or configured
        let url = format!("projects/{}/issues/{}", project_id, issue_id);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .put(format!("{}/api/v4/{}", self.base_url, url))
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&update_data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to update GitLab issue: {}",
                response.status()
            ))
        }
    }

    async fn close_issue(&self, issue_id: &str, _resolution: &IssueResolution) -> Result<()> {
        let update = IssueUpdate {
            status: Some("closed".to_string()),
            ..Default::default()
        };
        self.update_issue(issue_id, &update).await
    }

    async fn trigger_workflow(
        &self,
        request: &WorkflowTriggerRequest,
    ) -> Result<TriggeredWorkflow> {
        let trigger_data = GitLabPipelineTrigger {
            r#ref: request.branch.clone().unwrap_or_else(|| "main".to_string()),
            variables: request.parameters.clone(),
        };

        let project_id = "1"; // Would need to be configured
        let pipeline: GitLabPipeline = self
            .post_request(&format!("projects/{}/pipeline", project_id), &trigger_data)
            .await?;

        Ok(TriggeredWorkflow {
            id: pipeline.id.to_string(),
            name: request.workflow_name.clone(),
            url: pipeline.web_url,
            status: pipeline.status,
            triggered_at: pipeline.created_at,
        })
    }

    async fn generate_report(&self, request: &ReportRequest) -> Result<SystemReport> {
        let issues: Vec<GitLabIssue> = self.make_request("issues?per_page=100").await?;

        let total_issues = issues.len() as u64;
        let duplicates_found = issues
            .iter()
            .filter(|issue| {
                issue
                    .labels
                    .iter()
                    .any(|label| label.to_lowercase().contains("duplicate"))
            })
            .count() as u64;

        Ok(SystemReport {
            system_name: "gitlab".to_string(),
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
            supports_workflow_triggers: true,
            supports_reporting: true,
            supports_webhooks: true,
            supports_bulk_operations: false,
            max_batch_size: None,
            rate_limits: Some(RateLimitInfo {
                requests_per_minute: 300,
                burst_limit: 50,
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

// GitLab API data structures
#[derive(Debug, Deserialize)]
struct GitLabVersion {
    _version: String,
}

#[derive(Debug, Deserialize)]
struct GitLabIssue {
    iid: u32,
    title: String,
    description: Option<String>,
    state: String,
    web_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    labels: Vec<String>,
}

#[derive(Debug, Serialize)]
struct GitLabIssueCreation {
    title: String,
    description: Option<String>,
    labels: Option<String>,
}

#[derive(Debug, Serialize)]
struct GitLabPipelineTrigger {
    r#ref: String,
    variables: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GitLabPipeline {
    id: u32,
    status: String,
    web_url: String,
    created_at: DateTime<Utc>,
}
