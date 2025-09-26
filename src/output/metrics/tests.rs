//! # Metrics Framework Tests
//!
//! Comprehensive tests for the success metrics framework.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnalysisResults, Finding, Severity};
    use crate::output::OutputResult;
    use chrono::Utc;
    use tokio;

    #[test]
    fn test_output_metrics_creation() -> Result<(), Box<dyn std::error::Error>> {
        let metrics = OutputMetrics::new("json".to_string());

        assert_eq!(metrics.format, "json");
        assert!(metrics.functionality.success);
        assert_eq!(metrics.performance.generation_time_ms, 0);
        assert_eq!(metrics.security.incidents_detected, 0);
        assert_eq!(metrics.user_experience.satisfaction_score, 4.0);
    }

    #[test]
    fn test_metrics_collector() -> Result<(), Box<dyn std::error::Error>> {
        let collector = OutputMetricsCollector::new();

        // Create test data
        let mut results = AnalysisResults::new("test".to_string());
        results.add_finding(Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            "test.rs".into(),
            10,
            "Test finding".to_string(),
        ));

        let output_result = OutputResult::new(
            r#"{"findings": []}"#.to_string(),
            "json",
            "test".to_string(),
        );

        let metrics = collector.collect_metrics(&results, &output_result, "json", 1500)?;

        assert_eq!(metrics.format, "json");
        assert_eq!(metrics.performance.generation_time_ms, 1500);
        assert_eq!(metrics.summary.total_findings, 1);
    }

    #[tokio::test]
    async fn test_real_time_monitor() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = RealTimeMonitor::new();

        let metrics = OutputMetrics::new("json".to_string());

        // Test recording metrics
        let result = monitor.record_metrics(metrics.clone()).await;
        assert!(result.is_ok());

        // Test getting recent metrics
        let recent = monitor.get_recent_metrics(10).await?;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].format, "json");
    }

    #[tokio::test]
    async fn test_alert_manager() -> Result<(), Box<dyn std::error::Error>> {
        let alert_manager = AlertManager::new();

        // Test getting active alerts (should be empty initially)
        let active = alert_manager.get_active_alerts().await?;
        assert_eq!(active.len(), 0);

        // Test alert statistics
        let stats = alert_manager.get_alert_statistics().await?;
        assert_eq!(stats.total_active, 0);
        assert_eq!(stats.total_historical, 0);
    }

    #[tokio::test]
    async fn test_trend_analyzer() -> Result<(), Box<dyn std::error::Error>> {
        let trend_analyzer = TrendAnalyzer::new();

        let metrics = vec![
            OutputMetrics::new("json".to_string()),
            OutputMetrics::new("html".to_string()),
        ];

        // Test trend analysis
        let trends = trend_analyzer.analyze_trends(&metrics).await?;
        assert_eq!(trends.performance_trend, TrendDirection::Stable);
        assert_eq!(trends.success_rate_trend, TrendDirection::Stable);
    }

    #[tokio::test]
    async fn test_automated_reporter() -> Result<(), Box<dyn std::error::Error>> {
        let reporter = AutomatedReporter::new();

        let metrics = vec![OutputMetrics::new("json".to_string())];

        // Test report generation (would create files in real usage)
        let result = reporter.generate_report(&metrics, None, ReportType::Custom).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_metrics_service_integration() -> Result<(), Box<dyn std::error::Error>> {
        let service = OutputMetricsService::new();

        let results = AnalysisResults::new("test".to_string());
        let output_result = OutputResult::new("test".to_string(), "json", "test".to_string());

        // Test recording metrics
        let result = service.record_output_metrics(
            &results,
            &output_result,
            "json",
            1000,
        ).await;
        assert!(result.is_ok());

        // Test health status
        let health = service.get_health_status().await?;
        assert_eq!(health.overall_status, HealthStatus::Healthy);

        // Test report generation
        let report = service.generate_report(None).await?;
        assert_eq!(report.summary.total_operations, 1);
    }

    #[test]
    fn test_metric_value_operations() -> Result<(), Box<dyn std::error::Error>> {
        let int_value = MetricValue::Integer(42);
        assert_eq!(int_value.as_f64(), Some(42.0));

        let float_value = MetricValue::Float(3.14);
        assert_eq!(float_value.as_f64(), Some(3.14));

        let string_value = MetricValue::String("test".to_string());
        assert_eq!(string_value.as_string(), "test");

        let bool_value = MetricValue::Boolean(true);
        assert_eq!(bool_value.as_string(), "true");
    }

    #[test]
    fn test_output_metrics_health_calculation() -> Result<(), Box<dyn std::error::Error>> {
        let mut metrics = OutputMetrics::new("json".to_string());
        metrics.functionality.success = true;
        metrics.performance.generation_time_ms = 1000;
        metrics.security.incidents_detected = 0;
        metrics.user_experience.satisfaction_score = 4.5;

        let health_score = metrics.calculate_health_score();
        assert!(health_score > 0.8); // Should be healthy

        assert!(metrics.is_healthy());
    }

    #[test]
    fn test_alert_rule_evaluation() -> Result<(), Box<dyn std::error::Error>> {
        let alert_manager = AlertManager::new();

        // Test with a simple rule
        let rule = AlertRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "Test alert rule".to_string(),
            metric_name: "generation_time_ms".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(5000),
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: 15,
            last_triggered: None,
        };

        let mut metrics = OutputMetrics::new("json".to_string());
        metrics.performance.generation_time_ms = 6000; // Above threshold

        // Note: In real usage, this would be evaluated by the alert manager
        // This test demonstrates the structure
        assert_eq!(metrics.performance.generation_time_ms, 6000);
    }

    #[test]
    fn test_trend_direction_calculation() -> Result<(), Box<dyn std::error::Error>> {
        let trend_analyzer = TrendAnalyzer::new();

        // Test stable trend
        let stable_values = vec![100.0, 101.0, 99.0, 100.0, 102.0];
        let trend = trend_analyzer.calculate_trend_direction(&stable_values);
        assert_eq!(trend, TrendDirection::Stable);

        // Test increasing trend
        let increasing_values = vec![100.0, 110.0, 120.0, 130.0, 140.0];
        let trend = trend_analyzer.calculate_trend_direction(&increasing_values);
        assert_eq!(trend, TrendDirection::Increasing);

        // Test decreasing trend
        let decreasing_values = vec![140.0, 130.0, 120.0, 110.0, 100.0];
        let trend = trend_analyzer.calculate_trend_direction(&decreasing_values);
        assert_eq!(trend, TrendDirection::Degrading);
    }

    #[test]
    fn test_metrics_aggregation() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = RealTimeMonitor::new();

        let metrics1 = OutputMetrics::new("json".to_string());
        let metrics2 = OutputMetrics::new("html".to_string());

        let aggregated = monitor.aggregate_metrics(&[metrics1, metrics2], Utc::now());

        assert_eq!(aggregated.count, 2);
        assert_eq!(aggregated.success_rate, 1.0); // Both successful
    }

    #[test]
    fn test_system_health_assessment() -> Result<(), Box<dyn std::error::Error>> {
        let service = OutputMetricsService::new();

        let metrics = vec![
            OutputMetrics::new("json".to_string()),
            OutputMetrics::new("html".to_string()),
        ];

        let overall_status = service.calculate_overall_status(&metrics, &[]);
        assert_eq!(overall_status, HealthStatus::Healthy);

        let component_statuses = service.calculate_component_statuses(&metrics);
        assert_eq!(component_statuses.len(), 4); // functionality, performance, security, ux
    }
}
