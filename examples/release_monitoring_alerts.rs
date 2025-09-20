//! Example: Release Monitoring with Alerts and Notifications
//!
//! This example demonstrates how to set up release monitoring with
//! comprehensive alerting and notification capabilities.

use anyhow::Result;
use do_codeguardian::{
    config::{AlertThresholdsConfig, Config, EmailConfig, NotificationConfig, SlackConfig},
    dashboard::DashboardService,
    performance::{
        monitoring::ProductionMonitoringService, notifications::AlertNotificationService,
        PerformanceMetrics,
    },
    release_monitoring::ReleaseMonitoringService,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 CodeGuardian Release Monitoring with Alerts Example");
    println!("==================================================");

    // Load configuration
    let config = load_example_config();

    // Initialize core services
    let metrics = Arc::new(PerformanceMetrics::new());
    let release_monitoring = Arc::new(ReleaseMonitoringService::new(
        config.dashboard.release_monitoring_config.unwrap(),
    ));

    // Initialize notification service
    let notification_service = Arc::new(AlertNotificationService::new(
        config.performance.monitoring.notifications.clone(),
    ));

    // Initialize dashboard service
    let dashboard = Arc::new(DashboardService::new(config.dashboard));

    // Create production monitoring service with release monitoring
    let mut monitoring_service = ProductionMonitoringService::new(
        Arc::clone(&metrics),
        Some(Arc::clone(&dashboard)),
        None, // regex_cache
        None, // memory_pools
        Some(Arc::clone(&release_monitoring)),
        config.performance.monitoring.alert_thresholds.clone(),
        Some(config.performance.monitoring.notifications.clone()),
    );

    println!("✅ Services initialized successfully");

    // Simulate monitoring loop
    println!("\n📊 Starting monitoring loop...");
    for i in 0..5 {
        println!("\n--- Monitoring Cycle {} ---", i + 1);

        // Update metrics (in real usage, this would happen automatically)
        release_monitoring.update_metrics().await?;
        monitoring_service.update().await?;

        // Check for alerts
        let alerts = monitoring_service.check_alerts();
        if !alerts.is_empty() {
            println!("🚨 Found {} alerts:", alerts.len());
            for alert in &alerts {
                println!("  - {}: {}", alert.alert_type.clone() as u8, alert.message);
            }

            // Process notifications
            notification_service.process_alerts(alerts).await?;
        } else {
            println!("✅ No alerts detected");
        }

        // Wait before next cycle
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    println!("\n🎉 Release monitoring example completed!");
    println!("💡 In production, this would run continuously and send real notifications");

    Ok(())
}

/// Load example configuration with release monitoring settings
fn load_example_config() -> Config {
    let mut config = Config::default();

    // Configure alert thresholds
    config.performance.monitoring.alert_thresholds = AlertThresholdsConfig {
        cache_hit_rate_warning: 0.7,
        memory_reuse_rate_warning: 0.5,
        max_processing_time_critical: 2000.0,
        release_success_rate_warning: 0.95,
        max_post_release_issues_warning: 5.0,
        max_deployment_time_warning: 60.0,
        alert_escalation_minutes: 30,
    };

    // Configure notifications
    config.performance.monitoring.notifications = NotificationConfig {
        enabled: true,
        email: Some(EmailConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "alerts@example.com".to_string(),
            password: "example-password".to_string(),
            from_address: "alerts@example.com".to_string(),
            to_addresses: vec!["devops@example.com".to_string()],
            use_tls: true,
        }),
        slack: Some(SlackConfig {
            webhook_url: "https://hooks.slack.com/services/example".to_string(),
            channel: "#alerts".to_string(),
            username: "CodeGuardian".to_string(),
            icon_emoji: ":warning:".to_string(),
        }),
        webhook: None,
        escalation: Default::default(),
    };

    config
}
