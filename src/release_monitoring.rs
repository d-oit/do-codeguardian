// Release Monitoring Module
///
// This module provides comprehensive release monitoring capabilities for CodeGuardian,
// collecting and tracking release metrics from GitHub API including success rates,
// deployment times, post-release issues, and user adoption patterns.

use crate::error::Result;

use crate::github_api::GitHubApiClient;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tokio::sync::RwLock;

/// Release monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseMonitoringConfig {
    /// GitHub repository in format "owner/repo"
    pub repository: String,
    /// Number of recent releases to monitor
    pub max_releases_to_monitor: usize,
    /// Days to look back for post-release issues
    pub post_release_issue_window_days: i64,
    /// Path to store metrics data
    pub metrics_storage_path: String,
    /// Enable real-time monitoring
    pub enable_real_time: bool,
    /// Monitoring interval in seconds
    pub monitoring_interval_seconds: u64,
}

impl Default for ReleaseMonitoringConfig {
    fn default() -> Self {
        Self {
            repository: String::new(),
            max_releases_to_monitor: 10,
            post_release_issue_window_days: 30,
            metrics_storage_path: "release_metrics.json".to_string(),
            enable_real_time: false,
            monitoring_interval_seconds: 3600, // 1 hour
        }
    }
}

/// Individual release data from GitHub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseData {
    /// Release tag name
    pub tag_name: String,
    /// Release name
    pub name: String,
    /// Release creation date
    pub created_at: DateTime<Utc>,
    /// Release publication date
    pub published_at: Option<DateTime<Utc>>,
    /// Whether the release is a draft
    pub draft: bool,
    /// Whether the release is a pre-release
    pub prerelease: bool,
    /// Release body/description
    pub body: String,
    /// Number of downloads for assets
    pub download_count: u64,
}

/// Release metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseMetrics {
    /// Release identifier
    pub release_tag: String,
    /// Release creation timestamp
    pub created_at: DateTime<Utc>,
    /// Time to publish after creation (in hours)
    pub time_to_publish_hours: Option<f64>,
    /// Success rate (1.0 if published successfully, 0.0 if draft/failed)
    pub success_rate: f64,
    /// Number of post-release issues created within window
    pub post_release_issues: u64,
    /// Number of downloads
    pub download_count: u64,
    /// User adoption score (based on downloads and issues)
    pub user_adoption_score: f64,
    /// Deployment time in minutes (if available)
    pub deployment_time_minutes: Option<f64>,
}

/// Aggregated release metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedReleaseMetrics {
    /// Timestamp of aggregation
    pub timestamp: DateTime<Utc>,
    /// Individual release metrics
    pub releases: Vec<ReleaseMetrics>,
    /// Overall success rate across all releases
    pub overall_success_rate: f64,
    /// Average time to publish in hours
    pub average_time_to_publish: f64,
    /// Average post-release issues per release
    pub average_post_release_issues: f64,
    /// Total downloads across all releases
    pub total_downloads: u64,
    /// Average user adoption score
    pub average_user_adoption: f64,
    /// Average deployment time in minutes
    pub average_deployment_time: f64,
}

/// Release monitoring service
pub struct ReleaseMonitoringService {
    config: ReleaseMonitoringConfig,
    github_client: RwLock<GitHubApiClient>,
    metrics_history: RwLock<Vec<AggregatedReleaseMetrics>>,
}

impl ReleaseMonitoringService {
    /// Create a new release monitoring service
    pub fn new(config: ReleaseMonitoringConfig) -> Self {
        Self {
            github_client: RwLock::new(GitHubApiClient::new()),
            config,
            metrics_history: RwLock::new(Vec::new()),
        }
    }

    /// Load metrics from storage
    pub async fn load_metrics(&self) -> Result<()> {
        if Path::new(&self.config.metrics_storage_path).exists() {
            let data = fs::read_to_string(&self.config.metrics_storage_path)?;
            let metrics: Vec<AggregatedReleaseMetrics> = serde_json::from_str(&data)?;
            *self.metrics_history.write().await = metrics;
        }
        Ok(())
    }

    /// Save metrics to storage
    pub async fn save_metrics(&self) -> Result<()> {
        let metrics = self.metrics_history.read().await.clone();
        let data = serde_json::to_string_pretty(&metrics)?;
        fs::write(&self.config.metrics_storage_path, data)?;
        Ok(())
    }

    /// Collect release data from GitHub
    pub async fn collect_release_data(&self) -> Result<Vec<ReleaseData>> {
        let mut client = self.github_client.write().await;

        // Get list of releases
        let args = [
            "release",
            "list",
            "--repo",
            &self.config.repository,
            "--limit",
            &self.config.max_releases_to_monitor.to_string(),
            "--json",
            "tagName,name,createdAt,publishedAt,isDraft,isPrerelease,body",
        ];

        let output = client.execute_gh_command(&args).await?;
        let releases: Vec<ReleaseData> = serde_json::from_str(&output)?;

        // Enrich with download counts
        let mut enriched_releases = Vec::new();
        for release in releases {
            let download_count = self
                .get_release_download_count(&mut client, &release.tag_name)
                .await?;
            let mut enriched = release.clone();
            enriched.download_count = download_count;
            enriched_releases.push(enriched);
        }

        Ok(enriched_releases)
    }

    /// Get download count for a specific release
    async fn get_release_download_count(
        &self,
        client: &mut GitHubApiClient,
        tag_name: &str,
    ) -> Result<u64> {
        let args = [
            "release",
            "view",
            tag_name,
            "--repo",
            &self.config.repository,
            "--json",
            "assets",
        ];

        let output = client.execute_gh_command(&args).await?;
        let response: serde_json::Value = serde_json::from_str(&output)?;

        let total_downloads = response
            .get("assets")
            .and_then(|assets| assets.as_array())
            .map(|assets| {
                assets
                    .iter()
                    .filter_map(|asset| asset.get("downloadCount").and_then(|dc| dc.as_u64()))
                    .sum()
            })
            .unwrap_or(0);

        Ok(total_downloads)
    }

    /// Collect post-release issues for a release
    async fn collect_post_release_issues(&self, release_date: DateTime<Utc>) -> Result<u64> {
        let mut client = self.github_client.write().await;

        let window_end = release_date + Duration::days(self.config.post_release_issue_window_days);
        let now = Utc::now();

        // Only count issues if we're past the window
        if now < window_end {
            return Ok(0);
        }

        let since_date = release_date.format("%Y-%m-%d").to_string();
        let until_date = window_end.format("%Y-%m-%d").to_string();

        let args = [
            "issue",
            "list",
            "--repo",
            &self.config.repository,
            "--state",
            "all",
            "--since",
            &since_date,
            "--json",
            "number,createdAt,title",
            "-q",
            ".[] | select(.createdAt >= $since and .createdAt <= $until)",
            "--jq",
            &format!("length"),
        ];

        // Note: This is a simplified approach. In practice, you'd need more sophisticated
        // filtering to only count issues related to the release
        let output = client.execute_gh_command(&args).await?;
        let count: u64 = output.trim().parse().unwrap_or(0);

        Ok(count)
    }

    /// Calculate metrics for releases
    pub async fn calculate_metrics(
        &self,
        releases: &[ReleaseData],
    ) -> Result<AggregatedReleaseMetrics> {
        let mut release_metrics = Vec::new();

        for release in releases {
            let post_release_issues = self.collect_post_release_issues(release.created_at).await?;

            let time_to_publish_hours = release
                .published_at
                .map(|published| (published - release.created_at).num_seconds() as f64 / 3600.0);

            let success_rate = if release.draft { 0.0 } else { 1.0 };

            // Calculate user adoption score (simplified formula)
            let user_adoption_score = if release.download_count > 0 {
                let issue_penalty = (post_release_issues as f64 * 0.1).min(1.0);
                (release.download_count as f64 / 100.0).min(10.0) * (1.0 - issue_penalty)
            } else {
                0.0
            };

            let metrics = ReleaseMetrics {
                release_tag: release.tag_name.clone(),
                created_at: release.created_at,
                time_to_publish_hours,
                success_rate,
                post_release_issues,
                download_count: release.download_count,
                user_adoption_score,
                deployment_time_minutes: None, // Would need CI/CD integration
            };

            release_metrics.push(metrics);
        }

        // Calculate aggregates
        let total_releases = release_metrics.len() as f64;
        let overall_success_rate =
            release_metrics.iter().map(|m| m.success_rate).sum::<f64>() / total_releases;
        let average_time_to_publish = release_metrics
            .iter()
            .filter_map(|m| m.time_to_publish_hours)
            .sum::<f64>()
            / release_metrics
                .iter()
                .filter(|m| m.time_to_publish_hours.is_some())
                .count() as f64;
        let average_post_release_issues = release_metrics
            .iter()
            .map(|m| m.post_release_issues as f64)
            .sum::<f64>()
            / total_releases;
        let total_downloads = release_metrics.iter().map(|m| m.download_count).sum();
        let average_user_adoption = release_metrics
            .iter()
            .map(|m| m.user_adoption_score)
            .sum::<f64>()
            / total_releases;
        let average_deployment_time = 0.0; // Placeholder

        Ok(AggregatedReleaseMetrics {
            timestamp: Utc::now(),
            releases: release_metrics,
            overall_success_rate,
            average_time_to_publish: average_time_to_publish.max(0.0),
            average_post_release_issues,
            total_downloads,
            average_user_adoption,
            average_deployment_time,
        })
    }

    /// Update metrics by collecting new data
    pub async fn update_metrics(&self) -> Result<()> {
        let releases = self.collect_release_data().await?;
        let metrics = self.calculate_metrics(&releases).await?;

        self.metrics_history.write().await.push(metrics);
        self.save_metrics().await?;

        Ok(())
    }

    /// Get latest metrics
    pub async fn get_latest_metrics(&self) -> Option<AggregatedReleaseMetrics> {
        self.metrics_history.read().await.last().cloned()
    }

    /// Get metrics for a time range
    pub async fn get_metrics_for_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<AggregatedReleaseMetrics> {
        self.metrics_history
            .read()
            .await
            .iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Get release trends over time
    pub async fn get_release_trends(&self, days: i64) -> Result<ReleaseTrends> {
        let start = Utc::now() - Duration::days(days);
        let metrics = self.get_metrics_for_range(start, Utc::now()).await;

        let success_rate_trend = metrics.iter().map(|m| m.overall_success_rate).collect();
        let adoption_trend = metrics.iter().map(|m| m.average_user_adoption).collect();
        let issues_trend = metrics
            .iter()
            .map(|m| m.average_post_release_issues)
            .collect();

        Ok(ReleaseTrends {
            timestamps: metrics.iter().map(|m| m.timestamp).collect(),
            success_rates: success_rate_trend,
            user_adoption_scores: adoption_trend,
            post_release_issues: issues_trend,
        })
    }
}

/// Release trends for dashboard visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseTrends {
    pub timestamps: Vec<DateTime<Utc>>,
    pub success_rates: Vec<f64>,
    pub user_adoption_scores: Vec<f64>,
    pub post_release_issues: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_metrics_calculation() {
        let config = ReleaseMonitoringConfig {
            repository: "test/repo".to_string(),
            metrics_storage_path: "test_metrics.json".to_string(),
            ..Default::default()
        };

        let service = ReleaseMonitoringService::new(config);

        let releases = vec![ReleaseData {
            tag_name: "v1.0.0".to_string(),
            name: "Version 1.0.0".to_string(),
            created_at: Utc::now() - Duration::hours(24),
            published_at: Some(Utc::now() - Duration::hours(2)),
            draft: false,
            prerelease: false,
            body: "Release notes".to_string(),
            download_count: 100,
        }];

        let metrics = service.calculate_metrics(&releases).await.unwrap();
        assert_eq!(metrics.releases.len(), 1);
        assert!(metrics.overall_success_rate > 0.0);
    }

    #[test]
    fn test_config_defaults() {
        let config = ReleaseMonitoringConfig::default();
        assert_eq!(config.max_releases_to_monitor, 10);
        assert_eq!(config.post_release_issue_window_days, 30);
    }
}
