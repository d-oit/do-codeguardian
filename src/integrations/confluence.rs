//! Confluence integration for CodeGuardian

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

/// Confluence client implementation
pub struct ConfluenceClient {
    config: SystemConfig,
    client: Client,
    base_url: String,
}

impl ConfluenceClient {
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
            _ => Err(anyhow::anyhow!("Unsupported auth type for Confluence")),
        }
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/rest/api/{}", self.base_url, endpoint);
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
            Err(anyhow::anyhow!(
                "Confluence API error: {}",
                response.status()
            ))
        }
    }

    async fn post_request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<R> {
        let url = format!("{}/rest/api/{}", self.base_url, endpoint);
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
            Err(anyhow::anyhow!(
                "Confluence API error: {}",
                response.status()
            ))
        }
    }
}

#[async_trait]
impl ExternalSystemClient for ConfluenceClient {
    fn system_name(&self) -> &str {
        "confluence"
    }

    async fn health_check(&self) -> Result<SystemHealth> {
        let start = std::time::Instant::now();

        match self
            .make_request::<ConfluenceSpaceList>("space?limit=1")
            .await
        {
            Ok(_) => {
                let response_time = start.elapsed().as_millis() as u64;
                Ok(SystemHealth {
                    status: HealthStatus::Healthy,
                    response_time_ms: Some(response_time),
                    last_error: None,
                    features_available: vec![
                        "content_search".to_string(),
                        "page_creation".to_string(),
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
        let cql = format!("title ~ \"{}\" OR text ~ \"{}\"", query.title, query.title);

        let search_url = format!(
            "content/search?cql={}&limit={}",
            urlencoding::encode(&cql),
            query.max_results
        );

        let search_result: ConfluenceSearchResponse = self.make_request(&search_url).await?;

        let mut results = Vec::new();
        for page in search_result.results {
            let similarity_score = calculate_similarity(&query.title, &page.title);

            if similarity_score >= query.similarity_threshold {
                results.push(DuplicateSearchResult {
                    id: page.id,
                    title: page.title,
                    description: page.excerpt,
                    url: format!("{}{}", self.base_url, page._links.webui),
                    status: page.status,
                    created_at: page.history.created_date,
                    updated_at: page.version.when,
                    relevance_score: similarity_score,
                    similarity_score,
                    source_system: "confluence".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        Ok(results)
    }

    async fn create_issue(&self, issue: &IssueCreationRequest) -> Result<CreatedIssue> {
        // In Confluence, we create pages instead of issues
        let page_content = ConfluencePageCreation {
            r#type: "page".to_string(),
            title: issue.title.clone(),
            space: ConfluenceSpace {
                key: issue.project_key.clone().unwrap_or_default(),
            },
            body: ConfluenceBody {
                storage: ConfluenceStorage {
                    value: format!("<p>{}</p>", issue.description),
                    representation: "storage".to_string(),
                },
            },
        };

        let created: ConfluenceCreatedPage = self.post_request("content", &page_content).await?;

        Ok(CreatedIssue {
            id: created.id,
            key: None,
            url: format!("{}{}", self.base_url, created._links.webui),
            status: "current".to_string(),
            created_at: Utc::now(),
        })
    }

    async fn update_issue(&self, issue_id: &str, update: &IssueUpdate) -> Result<()> {
        // Get current page version first
        let current_page: ConfluencePage =
            self.make_request(&format!("content/{}", issue_id)).await?;

        let mut update_data = serde_json::json!({
            "version": {
                "number": current_page.version.number + 1
            },
            "type": "page"
        });

        if let Some(title) = &update.title {
            update_data["title"] = serde_json::Value::String(title.clone());
        }

        if let Some(description) = &update.description {
            update_data["body"] = serde_json::json!({
                "storage": {
                    "value": format!("<p>{}</p>", description),
                    "representation": "storage"
                }
            });
        }

        let url = format!("content/{}", issue_id);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .put(format!("{}/rest/api/{}", self.base_url, url))
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&update_data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to update Confluence page: {}",
                response.status()
            ))
        }
    }

    async fn close_issue(&self, issue_id: &str, _resolution: &IssueResolution) -> Result<()> {
        // In Confluence, we can archive or delete pages
        let url = format!("content/{}", issue_id);
        let auth_header = self.get_auth_header()?;

        let response = self
            .client
            .delete(format!("{}/rest/api/{}", self.base_url, url))
            .header("Authorization", auth_header)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to delete Confluence page: {}",
                response.status()
            ))
        }
    }

    async fn trigger_workflow(
        &self,
        _request: &WorkflowTriggerRequest,
    ) -> Result<TriggeredWorkflow> {
        Err(anyhow::anyhow!(
            "Workflow triggering not supported in Confluence"
        ))
    }

    async fn generate_report(&self, request: &ReportRequest) -> Result<SystemReport> {
        let cql = match request.report_type.as_str() {
            "duplicates" => "label = \"duplicate\"".to_string(),
            "all_pages" => "type = page".to_string(),
            _ => format!("text ~ \"{}\"", request.report_type),
        };

        let search_url = format!(
            "content/search?cql={}&limit=1000",
            urlencoding::encode(&cql)
        );

        let search_result: ConfluenceSearchResponse = self.make_request(&search_url).await?;

        let total_issues = search_result.size as u64;
        let duplicates_found = search_result
            .results
            .iter()
            .filter(|page| page.title.to_lowercase().contains("duplicate"))
            .count() as u64;

        Ok(SystemReport {
            system_name: "confluence".to_string(),
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
            supports_issue_creation: true, // Pages as issues
            supports_issue_updates: true,
            supports_duplicate_search: true,
            supports_workflow_triggers: false,
            supports_reporting: true,
            supports_webhooks: true,
            supports_bulk_operations: false,
            max_batch_size: None,
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

// Confluence API data structures
#[derive(Debug, Deserialize)]
struct ConfluenceSpaceList {
    _results: Vec<ConfluenceSpace>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfluenceSpace {
    key: String,
}

#[derive(Debug, Deserialize)]
struct ConfluenceSearchResponse {
    results: Vec<ConfluencePage>,
    size: usize,
}

#[derive(Debug, Deserialize)]
struct ConfluencePage {
    id: String,
    title: String,
    excerpt: Option<String>,
    status: String,
    version: ConfluenceVersion,
    history: ConfluenceHistory,
    _links: ConfluenceLinks,
}

#[derive(Debug, Deserialize)]
struct ConfluenceVersion {
    number: u32,
    when: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct ConfluenceHistory {
    #[serde(rename = "createdDate")]
    created_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct ConfluenceLinks {
    webui: String,
}

#[derive(Debug, Serialize)]
pub struct ConfluencePageCreation {
    r#type: String,
    title: String,
    space: ConfluenceSpace,
    body: ConfluenceBody,
}

impl ConfluencePageCreation {
    pub fn new(_config: &Config) -> Result<Self> {
        // TODO: implement
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[derive(Debug, Serialize)]
struct ConfluenceBody {
    storage: ConfluenceStorage,
}

#[derive(Debug, Serialize)]
struct ConfluenceStorage {
    value: String,
    representation: String,
}

#[derive(Debug, Deserialize)]
struct ConfluenceCreatedPage {
    id: String,
    _links: ConfluenceLinks,
}
