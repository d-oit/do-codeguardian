//! Alert notification system for performance and release monitoring
//!
//! Provides comprehensive notification capabilities for alerts including
//! email, Slack, webhook integrations, and escalation policies.

use crate::performance::monitoring::{PerformanceAlert, AlertSeverity, AlertEscalation};
use crate::config::{NotificationConfig, EmailConfig, SlackConfig, WebhookConfig, EscalationConfig};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Alert notification service
pub struct AlertNotificationService {
    config: NotificationConfig,
    alert_history: RwLock<HashMap<String, Vec<AlertNotification>>>,
}

impl AlertNotificationService {
    /// Create a new notification service
    pub fn new(config: NotificationConfig) -> Self {
        Self {
            config,
            alert_history: RwLock::new(HashMap::new()),
        }
    }

    /// Process and send notifications for alerts
    pub async fn process_alerts(&self, alerts: Vec<PerformanceAlert>) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        for alert in alerts {
            self.process_single_alert(alert).await?;
        }

        Ok(())
    }

    /// Process a single alert
    async fn process_single_alert(&self, alert: PerformanceAlert) -> Result<()> {
        let alert_key = self.generate_alert_key(&alert);

        // Check if this is a new alert or escalation
        let should_notify = self.should_notify(&alert_key, &alert).await;

        if should_notify {
            // Send notifications
            self.send_notifications(&alert).await?;

            // Record notification
            self.record_notification(&alert_key, &alert).await;
        }

        Ok(())
    }

    /// Determine if an alert should trigger a notification
    async fn should_notify(&self, alert_key: &str, alert: &PerformanceAlert) -> bool {
        let history = self.alert_history.read().await;
        let alert_history = history.get(alert_key);

        match alert_history {
            Some(history) => {
                // Check if this is an escalation
                if let Some(last_notification) = history.last() {
                    let time_since_last = Utc::now().signed_duration_since(last_notification.timestamp);
                    let escalation_threshold = match alert.severity {
                        AlertSeverity::Critical => self.config.escalation.critical_escalation_minutes,
                        AlertSeverity::Warning => self.config.escalation.warning_escalation_minutes,
                        AlertSeverity::Info => 60, // 1 hour for info alerts
                    };

                    time_since_last.num_minutes() >= escalation_threshold as i64
                } else {
                    true // No previous notifications
                }
            }
            None => true, // New alert
        }
    }

    /// Send notifications through all configured channels
    async fn send_notifications(&self, alert: &PerformanceAlert) -> Result<()> {
        // Send email notification
        if let Some(ref email_config) = self.config.email {
            self.send_email_notification(alert, email_config).await?;
        }

        // Send Slack notification
        if let Some(ref slack_config) = self.config.slack {
            self.send_slack_notification(alert, slack_config).await?;
        }

        // Send webhook notification
        if let Some(ref webhook_config) = self.config.webhook {
            self.send_webhook_notification(alert, webhook_config).await?;
        }

        Ok(())
    }

    /// Send email notification
    async fn send_email_notification(&self, alert: &PerformanceAlert, config: &EmailConfig) -> Result<()> {
        let subject = format!("CodeGuardian Alert: {}", alert.alert_type.clone() as u8);
        let body = self.format_alert_message(alert, "email");

        // In a real implementation, you would use an SMTP library here
        // For now, we'll just log the notification
        println!("Email notification would be sent:");
        println!("To: {:?}", config.to_addresses);
        println!("Subject: {}", subject);
        println!("Body: {}", body);

        Ok(())
    }

    /// Send Slack notification
    async fn send_slack_notification(&self, alert: &PerformanceAlert, config: &SlackConfig) -> Result<()> {
        let payload = serde_json::json!({
            "channel": config.channel,
            "username": config.username,
            "icon_emoji": config.icon_emoji,
            "text": self.format_alert_message(alert, "slack"),
            "attachments": [{
                "color": self.get_severity_color(&alert.severity),
                "fields": [
                    {
                        "title": "Alert Type",
                        "value": format!("{:?}", alert.alert_type),
                        "short": true
                    },
                    {
                        "title": "Severity",
                        "value": format!("{:?}", alert.severity),
                        "short": true
                    },
                    {
                        "title": "Current Value",
                        "value": format!("{:.2}", alert.current_value),
                        "short": true
                    },
                    {
                        "title": "Threshold",
                        "value": format!("{:.2}", alert.threshold),
                        "short": true
                    }
                ]
            }]
        });

        // In a real implementation, you would send an HTTP POST to the webhook URL
        println!("Slack notification would be sent:");
        println!("Webhook: {}", config.webhook_url);
        println!("Payload: {}", serde_json::to_string_pretty(&payload)?);

        Ok(())
    }

    /// Send webhook notification
    async fn send_webhook_notification(&self, alert: &PerformanceAlert, config: &WebhookConfig) -> Result<()> {
        let payload = serde_json::json!({
            "alert_type": format!("{:?}", alert.alert_type),
            "severity": format!("{:?}", alert.severity),
            "message": alert.message,
            "threshold": alert.threshold,
            "current_value": alert.current_value,
            "timestamp": alert.timestamp,
            "release_tag": alert.release_tag,
            "escalation_level": format!("{:?}", alert.escalation_level)
        });

        // In a real implementation, you would send an HTTP POST to the webhook URL
        println!("Webhook notification would be sent:");
        println!("URL: {}", config.url);
        println!("Headers: {:?}", config.headers);
        println!("Payload: {}", serde_json::to_string_pretty(&payload)?);

        Ok(())
    }

    /// Format alert message for different channels
    fn format_alert_message(&self, alert: &PerformanceAlert, channel: &str) -> String {
        let emoji = match alert.severity {
            AlertSeverity::Critical => "🚨",
            AlertSeverity::Warning => "⚠️",
            AlertSeverity::Info => "ℹ️",
        };

        let release_info = if let Some(ref tag) = alert.release_tag {
            format!(" (Release: {})", tag)
        } else {
            String::new()
        };

        match channel {
            "slack" => format!(
                "{} *{} Alert*{}: {}\n• Current: {:.2}\n• Threshold: {:.2}",
                emoji,
                alert.alert_type.clone() as u8,
                release_info,
                alert.message,
                alert.current_value,
                alert.threshold
            ),
            "email" => format!(
                "{} {} Alert{}\n\nMessage: {}\nCurrent Value: {:.2}\nThreshold: {:.2}\nTimestamp: {}\n\nPlease investigate and resolve this issue.",
                emoji,
                alert.alert_type.clone() as u8,
                release_info,
                alert.message,
                alert.current_value,
                alert.threshold,
                alert.timestamp
            ),
            _ => alert.message.clone(),
        }
    }

    /// Get color for Slack message based on severity
    fn get_severity_color(&self, severity: &AlertSeverity) -> &'static str {
        match severity {
            AlertSeverity::Critical => "danger",
            AlertSeverity::Warning => "warning",
            AlertSeverity::Info => "good",
        }
    }

    /// Generate a unique key for an alert
    fn generate_alert_key(&self, alert: &PerformanceAlert) -> String {
        format!(
            "{:?}_{}_{}",
            alert.alert_type,
            alert.release_tag.as_ref().unwrap_or(&"global".to_string()),
            alert.severity.clone() as u8
        )
    }

    /// Record a notification in history
    async fn record_notification(&self, alert_key: &str, alert: &PerformanceAlert) {
        let notification = AlertNotification {
            timestamp: Utc::now(),
            alert_type: alert.alert_type.clone(),
            severity: alert.severity.clone(),
            message: alert.message.clone(),
        };

        let mut history = self.alert_history.write().await;
        history.entry(alert_key.to_string()).or_insert_with(Vec::new).push(notification);
    }

    /// Get alert history for a specific alert key
    pub async fn get_alert_history(&self, alert_key: &str) -> Vec<AlertNotification> {
        let history = self.alert_history.read().await;
        history.get(alert_key).cloned().unwrap_or_default()
    }

    /// Clear old alert history (older than specified days)
    pub async fn cleanup_history(&self, max_age_days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(max_age_days);
        let mut history = self.alert_history.write().await;

        for notifications in history.values_mut() {
            notifications.retain(|n| n.timestamp > cutoff);
        }

        // Remove empty entries
        history.retain(|_, notifications| !notifications.is_empty());
    }
}

/// Alert notification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    pub timestamp: DateTime<Utc>,
    pub alert_type: crate::performance::monitoring::AlertType,
    pub severity: AlertSeverity,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::performance::monitoring::{AlertType, AlertSeverity, AlertEscalation};

    #[test]
    fn test_notification_config_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = crate::config::NotificationConfig::default();
        assert!(!config.enabled);
        assert!(config.email.is_none());
        assert!(config.slack.is_none());
        assert!(config.webhook.is_none());
    }

    #[test]
    fn test_alert_key_generation() -> Result<(), Box<dyn std::error::Error>> {
        let service = AlertNotificationService::new(crate::config::NotificationConfig::default());

        let alert = PerformanceAlert {
            alert_type: AlertType::CachePerformance,
            severity: AlertSeverity::Warning,
            message: "Test alert".to_string(),
            threshold: 0.7,
            current_value: 0.5,
            timestamp: Utc::now(),
            release_tag: Some("v1.0.0".to_string()),
            escalation_level: AlertEscalation::None,
            acknowledged: false,
        };

        let key = service.generate_alert_key(&alert);
        assert!(key.contains("CachePerformance"));
        assert!(key.contains("v1.0.0"));
    }

    #[test]
    fn test_severity_color_mapping() -> Result<(), Box<dyn std::error::Error>> {
        let service = AlertNotificationService::new(crate::config::NotificationConfig::default());

        assert_eq!(service.get_severity_color(&AlertSeverity::Critical), "danger");
        assert_eq!(service.get_severity_color(&AlertSeverity::Warning), "warning");
        assert_eq!(service.get_severity_color(&AlertSeverity::Info), "good");
    }
}
