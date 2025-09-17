//! Jenkins integration for CodeGuardian

use super::traits::*;
use super::SystemConfig;
use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

/// Jenkins client implementation
pub struct JenkinsClient {
    config: SystemConfig,
    client: Client,
    base_url: String,
}

impl JenkinsClient {
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
            super::AuthConfig::ApiKey { key } => Ok(format!("Bearer {}", key)),
            super::AuthConfig::BasicAuth { username, token } => {
                let credentials = BASE64.encode(format!("{}:{}", username, token));
                Ok(format!("Basic {}", credentials))
            }
            _ => Err(anyhow::anyhow!("Unsupported auth type for Jenkins")),
        }
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint);
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
            Err(anyhow::anyhow!("Jenkins API error: {}", response.status()))
        }
    }

    async fn post_request(&self, endpoint: &str, data: Option<&str>) -> Result<String> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let auth_header = self.get_auth_header()?;

        let mut request = self.client.post(&url).header("Authorization", auth_header);

        if let Some(body) = data {
            request = request
                .header("Content-Type", "application/json")
                .body(body.to_string());
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let text = response.text().await?;
            Ok(text)
        } else {
            Err(anyhow::anyhow!("Jenkins API error: {}", response.status()))
        }
    }
}

#[async_trait]
impl ExternalSystemClient for JenkinsClient {
    fn system_name(&self) -> &str {
        "jenkins"
    }

    async fn health_check(&self) -> Result<SystemHealth> {
        let start = std::time::Instant::now();

        match self.make_request::<JenkinsInfo>("api/json").await {
            Ok(_) => {
                let response_time = start.elapsed().as_millis() as u64;
                Ok(SystemHealth {
                    status: HealthStatus::Healthy,
                    response_time_ms: Some(response_time),
                    last_error: None,
                    features_available: vec![
                        "job_triggering".to_string(),
                        "build_monitoring".to_string(),
                        "pipeline_execution".to_string(),
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
        // Jenkins doesn't have traditional duplicate search, but we can search for similar job names
        let jobs: JenkinsJobs = self
            .make_request("api/json?tree=jobs[name,url,lastBuild[timestamp]]")
            .await?;

        let mut results = Vec::new();
        for job in jobs.jobs {
            let similarity_score = calculate_similarity(&query.title, &job.name);

            if similarity_score >= query.similarity_threshold {
                let created_at = job
                    .last_build
                    .and_then(|build| DateTime::from_timestamp_millis(build.timestamp))
                    .unwrap_or_else(Utc::now);

                results.push(DuplicateSearchResult {
                    id: job.name.clone(),
                    title: job.name,
                    description: Some("Jenkins Job".to_string()),
                    url: job.url,
                    status: "active".to_string(),
                    created_at,
                    updated_at: created_at,
                    relevance_score: similarity_score,
                    similarity_score,
                    source_system: "jenkins".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        Ok(results)
    }

    async fn create_issue(&self, _issue: &IssueCreationRequest) -> Result<CreatedIssue> {
        // Jenkins doesn't create issues, but we could create jobs
        Err(anyhow::anyhow!("Issue creation not supported in Jenkins"))
    }

    async fn update_issue(&self, _issue_id: &str, _update: &IssueUpdate) -> Result<()> {
        Err(anyhow::anyhow!("Issue updates not supported in Jenkins"))
    }

    async fn close_issue(&self, _issue_id: &str, _resolution: &IssueResolution) -> Result<()> {
        Err(anyhow::anyhow!("Issue closing not supported in Jenkins"))
    }

    async fn trigger_workflow(
        &self,
        request: &WorkflowTriggerRequest,
    ) -> Result<TriggeredWorkflow> {
        let job_name = &request.workflow_name;
        let endpoint = if request.parameters.is_empty() {
            format!("job/{}/build", job_name)
        } else {
            format!("job/{}/buildWithParameters", job_name)
        };

        // Convert parameters to form data
        let mut form_data = Vec::new();
        for (key, value) in &request.parameters {
            if let Some(str_value) = value.as_str() {
                form_data.push(format!("{}={}", key, urlencoding::encode(str_value)));
            }
        }

        let _response = if form_data.is_empty() {
            self.post_request(&endpoint, None).await?
        } else {
            let body = form_data.join("&");
            let url = format!("{}/{}", self.base_url, endpoint);
            let auth_header = self.get_auth_header()?;

            let response = self
                .client
                .post(&url)
                .header("Authorization", auth_header)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await?;

            if response.status().is_success() {
                response.text().await?
            } else {
                return Err(anyhow::anyhow!(
                    "Failed to trigger Jenkins job: {}",
                    response.status()
                ));
            }
        };

        // Get the build number from the queue
        let build_number = self.get_next_build_number(job_name).await?;

        Ok(TriggeredWorkflow {
            id: format!("{}-{}", job_name, build_number),
            name: job_name.clone(),
            url: format!("{}/job/{}/{}/", self.base_url, job_name, build_number),
            status: "triggered".to_string(),
            triggered_at: Utc::now(),
        })
    }

    async fn generate_report(&self, request: &ReportRequest) -> Result<SystemReport> {
        let jobs: JenkinsJobs = self
            .make_request("api/json?tree=jobs[name,builds[number,result,timestamp]]")
            .await?;

        let mut total_builds = 0u64;
        let mut failed_builds = 0u64;

        for job in &jobs.jobs {
            if let Some(builds) = &job.builds {
                total_builds += builds.len() as u64;
                failed_builds += builds
                    .iter()
                    .filter(|build| build.result.as_deref() == Some("FAILURE"))
                    .count() as u64;
            }
        }

        Ok(SystemReport {
            system_name: "jenkins".to_string(),
            report_type: request.report_type.clone(),
            generated_at: Utc::now(),
            total_issues: total_builds,
            duplicates_found: failed_builds, // Using failed builds as "issues"
            duplicate_rate: if total_builds > 0 {
                (failed_builds as f64 / total_builds as f64) * 100.0
            } else {
                0.0
            },
            time_period: request
                .start_date
                .zip(request.end_date)
                .map(|(start, end)| super::traits::TimePeriod { start, end }),
            metrics: HashMap::from([
                (
                    "total_jobs".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(jobs.jobs.len())),
                ),
                (
                    "total_builds".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(total_builds)),
                ),
            ]),
            details: vec![],
        })
    }

    fn get_capabilities(&self) -> SystemCapabilities {
        SystemCapabilities {
            supports_issue_creation: false,
            supports_issue_updates: false,
            supports_duplicate_search: true, // Job name similarity
            supports_workflow_triggers: true,
            supports_reporting: true,
            supports_webhooks: true,
            supports_bulk_operations: false,
            max_batch_size: None,
            rate_limits: Some(RateLimitInfo {
                requests_per_minute: 120,
                burst_limit: 20,
                current_usage: None,
                reset_time: None,
            }),
        }
    }
}

impl JenkinsClient {
    async fn get_next_build_number(&self, job_name: &str) -> Result<u32> {
        let job_info: JenkinsJobInfo = self
            .make_request(&format!("job/{}/api/json", job_name))
            .await?;
        Ok(job_info.next_build_number)
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

// Jenkins API data structures
#[derive(Debug, Deserialize)]
struct JenkinsInfo {
    #[serde(rename = "nodeDescription")]
    _node_description: String,
    #[serde(rename = "nodeName")]
    _node_name: String,
}

#[derive(Debug, Deserialize)]
struct JenkinsJobs {
    jobs: Vec<JenkinsJob>,
}

#[derive(Debug, Deserialize)]
struct JenkinsJob {
    name: String,
    url: String,
    #[serde(rename = "lastBuild")]
    last_build: Option<JenkinsBuildRef>,
    builds: Option<Vec<JenkinsBuild>>,
}

#[derive(Debug, Deserialize)]
struct JenkinsBuildRef {
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct JenkinsBuild {
    _number: u32,
    result: Option<String>,
    _timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct JenkinsJobInfo {
    #[serde(rename = "nextBuildNumber")]
    next_build_number: u32,
}
