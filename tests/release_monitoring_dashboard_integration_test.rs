//! Basic Integration Test for Release Monitoring Dashboard Functionality

#[cfg(all(feature = "dashboard", feature = "release-monitoring"))]
mod release_monitoring_tests {
    use anyhow::Result;
    use chrono::{Duration, Utc};
    use do_codeguardian::{
        dashboard::{
            DashboardService, DashboardConfig, DashboardMetrics,
            DuplicateStats, PreventionStats, SystemHealth, PerformanceMetrics,
            ReleaseMetrics,
        },
        release_monitoring::{
            ReleaseMonitoringService, ReleaseMonitoringConfig, ReleaseData,
        },
    };
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_release_monitoring_service_creation() -> Result<()> {
        println!("🚀 Testing Release Monitoring Service Creation");

        let temp_dir = tempdir()?;
        let metrics_path = temp_dir.path().join("test_release_metrics.json");

        let config = ReleaseMonitoringConfig {
            repository: "test-org/test-repo".to_string(),
            max_releases_to_monitor: 5,
            post_release_issue_window_days: 30,
            metrics_storage_path: metrics_path.to_string_lossy().to_string(),
            enable_real_time: false,
            monitoring_interval_seconds: 3600,
        };

        let _service = ReleaseMonitoringService::new(config);
        println!("✅ Release monitoring service created successfully");
        Ok(())
    }

    #[tokio::test]
    async fn test_dashboard_service_creation() -> Result<()> {
        println!("📊 Testing Dashboard Service Creation");

        let temp_dir = tempdir()?;
        let metrics_path = temp_dir.path().join("dashboard_test_metrics.json");

        let rm_config = ReleaseMonitoringConfig {
            repository: "test-org/test-repo".to_string(),
            max_releases_to_monitor: 5,
            post_release_issue_window_days: 30,
            metrics_storage_path: metrics_path.to_string_lossy().to_string(),
            enable_real_time: false,
            monitoring_interval_seconds: 3600,
        };

        let dashboard_config = DashboardConfig {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port: 8080,
            refresh_interval_seconds: 30,
            max_history_days: 30,
            enable_real_time: true,
            custom_views: vec![],
            release_monitoring_config: Some(rm_config),
        };

        let _dashboard_service = DashboardService::new(dashboard_config);
        println!("✅ Dashboard service created successfully");
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_creation() -> Result<()> {
        println!("📈 Testing Metrics Creation");

        let dashboard_metrics = DashboardMetrics {
            timestamp: Utc::now(),
            duplicate_stats: DuplicateStats {
                total_duplicates_found: 50,
                duplicates_by_type: HashMap::from([
                    ("code".to_string(), 40),
                    ("documentation".to_string(), 10),
                ]),
                duplicates_by_severity: HashMap::from([
                    ("high".to_string(), 10),
                    ("medium".to_string(), 30),
                    ("low".to_string(), 10),
                ]),
                false_positive_rate: 0.05,
                detection_accuracy: 0.95,
            },
            prevention_stats: PreventionStats {
                duplicates_prevented: 40,
                duplicates_created: 10,
                prevention_rate: 0.80,
                time_saved_hours: 100.0,
                cost_savings_estimate: 5000.0,
            },
            system_health: SystemHealth {
                api_success_rate: 0.98,
                average_response_time_ms: 150.0,
                error_rate: 0.02,
                uptime_percentage: 99.5,
                active_connections: 10,
            },
            performance_metrics: PerformanceMetrics {
                average_processing_time_ms: 200.0,
                throughput_per_minute: 100.0,
                memory_usage_mb: 256.0,
                cpu_usage_percentage: 30.0,
                queue_length: 2,
            },
            release_metrics: Some(ReleaseMetrics {
                overall_success_rate: 0.95,
                average_deployment_time: 45.0,
                total_releases: 3,
                average_user_adoption: 8.5,
                total_downloads: 500,
                average_post_release_issues: 2.0,
            }),
        };

        assert!(dashboard_metrics.release_metrics.is_some());
        println!("✅ Metrics created successfully with release data");
        Ok(())
    }
}
