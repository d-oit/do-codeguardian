//! # Alert Management System
//!
//! Provides comprehensive alert management capabilities for output metrics.

use super::types::*;
use super::Alert;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

/// Alert management system
#[derive(Debug)]
pub struct AlertManager {
    rules: Vec<AlertRule>,
    active_alerts: RwLock<HashMap<String, Alert>>,
    alert_history: RwLock<VecDeque<Alert>>,
    config: AlertConfig,
}

#[derive(Debug, Clone)]
pub struct AlertConfig {
    pub max_history_size: usize,
    pub alert_retention_days: u32,
    pub enable_auto_resolve: bool,
    pub auto_resolve_threshold_minutes: u64,
    pub escalation_enabled: bool,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            max_history_size: 1000,
            alert_retention_days: 30,
            enable_auto_resolve: true,
            auto_resolve_threshold_minutes: 30,
            escalation_enabled: true,
        }
    }
}

/// Alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub enabled: bool,
    pub cooldown_minutes: u64,
    pub last_triggered: Option<DateTime<Utc>>,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub operator: AlertOperator,
    pub threshold: MetricValue,
    pub time_window_seconds: u64,
    pub aggregation: AlertAggregation,
}

/// Alert operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Spike,
    Drop,
}

/// Alert aggregation methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertAggregation {
    Average,
    Sum,
    Count,
    Min,
    Max,
    Percentile95,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

/// Alert escalation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub id: String,
    pub name: String,
    pub trigger_condition: EscalationTrigger,
    pub actions: Vec<EscalationAction>,
    pub enabled: bool,
}

/// Escalation trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationTrigger {
    AlertUnresolved(Duration),
    MultipleAlerts { count: usize, time_window: Duration },
    SeverityUpgrade,
}

/// Escalation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationAction {
    SendEmail { recipients: Vec<String> },
    SendSlack { channel: String },
    CreateTicket { system: String, priority: String },
    PageOnCall,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new() -> Self {
        let mut manager = Self {
            rules: Vec::new(),
            active_alerts: RwLock::new(HashMap::new()),
            alert_history: RwLock::new(VecDeque::new()),
            config: AlertConfig::default(),
        };

        manager.initialize_default_rules();
        manager
    }

    /// Check metrics against alert rules and generate alerts
    pub async fn check_alerts(&self, metrics: &OutputMetrics) -> Result<Option<Alert>> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            // Check cooldown
            if let Some(last_triggered) = rule.last_triggered {
                let cooldown_duration = Duration::minutes(rule.cooldown_minutes as i64);
                if Utc::now() - last_triggered < cooldown_duration {
                    continue;
                }
            }

            // Evaluate the rule condition
            if self.evaluate_rule(rule, metrics).await? {
                let alert = self.create_alert(rule, metrics).await?;
                return Ok(Some(alert));
            }
        }

        Ok(None)
    }

    /// Get all active alerts
    pub async fn get_active_alerts(&self) -> Result<Vec<Alert>> {
        let active_alerts = self.active_alerts.read().await;
        Ok(active_alerts.values().cloned().collect())
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Result<Vec<Alert>> {
        let history = self.alert_history.read().await;
        Ok(history.iter().rev().take(limit).cloned().collect())
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut active_alerts = self.active_alerts.write().await;
        if let Some(mut alert) = active_alerts.remove(alert_id) {
            alert.resolved = true;
            let mut history = self.alert_history.write().await;
            history.push_back(alert);

            // Maintain history size
            while history.len() > self.config.max_history_size {
                history.pop_front();
            }
        }

        Ok(())
    }

    /// Add a new alert rule
    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    /// Update an existing alert rule
    pub fn update_rule(&mut self, rule_id: &str, updated_rule: AlertRule) -> Result<()> {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            *rule = updated_rule;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Alert rule not found: {}", rule_id))
        }
    }

    /// Remove an alert rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<()> {
        if let Some(pos) = self.rules.iter().position(|r| r.id == rule_id) {
            self.rules.remove(pos);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Alert rule not found: {}", rule_id))
        }
    }

    /// Get alert statistics
    pub async fn get_alert_statistics(&self) -> Result<AlertStatistics> {
        let active_alerts = self.active_alerts.read().await;
        let history = self.alert_history.read().await;

        let total_active = active_alerts.len();
        let total_historical = history.len();

        let critical_count = active_alerts
            .values()
            .filter(|a| matches!(a.severity, AlertSeverity::Critical))
            .count();
        let warning_count = active_alerts
            .values()
            .filter(|a| matches!(a.severity, AlertSeverity::Warning))
            .count();
        let info_count = active_alerts
            .values()
            .filter(|a| matches!(a.severity, AlertSeverity::Info))
            .count();

        let avg_resolution_time = self.calculate_avg_resolution_time(&history);

        Ok(AlertStatistics {
            total_active,
            total_historical,
            critical_count,
            warning_count,
            info_count,
            avg_resolution_time,
            most_common_alert_type: self.get_most_common_alert_type(&history),
        })
    }

    /// Process automatic alert resolution
    pub async fn process_auto_resolution(&self) -> Result<()> {
        if !self.config.enable_auto_resolve {
            return Ok(());
        }

        let mut alerts_to_resolve = Vec::new();
        let active_alerts = self.active_alerts.read().await;

        for (alert_id, alert) in active_alerts.iter() {
            let time_since_triggered = Utc::now() - alert.timestamp;
            if time_since_triggered
                > Duration::minutes(self.config.auto_resolve_threshold_minutes as i64)
            {
                alerts_to_resolve.push(alert_id.clone());
            }
        }

        drop(active_alerts);

        for alert_id in alerts_to_resolve {
            self.resolve_alert(&alert_id).await?;
        }

        Ok(())
    }

    /// Clean up old alerts
    pub async fn cleanup_old_alerts(&self) -> Result<()> {
        let cutoff = Utc::now() - Duration::days(self.config.alert_retention_days as i64);

        let mut history = self.alert_history.write().await;
        while let Some(oldest) = history.front() {
            if oldest.timestamp < cutoff {
                history.pop_front();
            } else {
                break;
            }
        }

        Ok(())
    }

    fn initialize_default_rules(&mut self) {
        // Performance alert rules
        self.add_rule(AlertRule {
            id: "perf_high_generation_time".to_string(),
            name: "High Generation Time".to_string(),
            description: "Output generation time is too high".to_string(),
            metric_name: "generation_time_ms".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(10000),
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: 15,
            last_triggered: None,
        });

        // Success rate alert rules
        self.add_rule(AlertRule {
            id: "success_rate_low".to_string(),
            name: "Low Success Rate".to_string(),
            description: "Output success rate has dropped below threshold".to_string(),
            metric_name: "success".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::LessThan,
                threshold: MetricValue::Float(0.9),
                time_window_seconds: 600,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: 30,
            last_triggered: None,
        });

        // Security alert rules
        self.add_rule(AlertRule {
            id: "security_incidents_high".to_string(),
            name: "High Security Incidents".to_string(),
            description: "Number of security incidents has increased".to_string(),
            metric_name: "security_incidents".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(5),
                time_window_seconds: 3600,
                aggregation: AlertAggregation::Sum,
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: 60,
            last_triggered: None,
        });

        // Memory usage alert rules
        self.add_rule(AlertRule {
            id: "memory_usage_high".to_string(),
            name: "High Memory Usage".to_string(),
            description: "Memory usage is too high".to_string(),
            metric_name: "memory_usage_bytes".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(500 * 1024 * 1024), // 500MB
                time_window_seconds: 300,
                aggregation: AlertAggregation::Max,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: 10,
            last_triggered: None,
        });
    }

    async fn evaluate_rule(&self, rule: &AlertRule, metrics: &OutputMetrics) -> Result<bool> {
        // For now, evaluate based on current metrics
        // In a real implementation, this would consider historical data and time windows
        let metric_value = match rule.metric_name.as_str() {
            "generation_time_ms" => {
                MetricValue::Integer(metrics.performance.generation_time_ms as i64)
            }
            "success" => MetricValue::Boolean(metrics.functionality.success),
            "security_incidents" => {
                MetricValue::Integer(metrics.security.incidents_detected as i64)
            }
            "memory_usage_bytes" => {
                MetricValue::Integer(metrics.performance.memory_usage_bytes as i64)
            }
            "satisfaction_score" => MetricValue::Float(metrics.user_experience.satisfaction_score),
            _ => return Ok(false), // Unknown metric
        };

        self.evaluate_condition(&rule.condition, &metric_value)
    }

    fn evaluate_condition(&self, condition: &AlertCondition, value: &MetricValue) -> Result<bool> {
        match condition.operator {
            AlertOperator::GreaterThan => {
                self.compare_values(value, &condition.threshold, |a, b| a > b)
            }
            AlertOperator::LessThan => {
                self.compare_values(value, &condition.threshold, |a, b| a < b)
            }
            AlertOperator::Equal => self.compare_values(value, &condition.threshold, |a, b| a == b),
            AlertOperator::NotEqual => {
                self.compare_values(value, &condition.threshold, |a, b| a != b)
            }
            AlertOperator::GreaterThanOrEqual => {
                self.compare_values(value, &condition.threshold, |a, b| a >= b)
            }
            AlertOperator::LessThanOrEqual => {
                self.compare_values(value, &condition.threshold, |a, b| a <= b)
            }
            AlertOperator::Spike | AlertOperator::Drop => {
                // Would need historical data for spike/drop detection
                Ok(false)
            }
        }
    }

    fn compare_values<F>(&self, a: &MetricValue, b: &MetricValue, cmp: F) -> Result<bool>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (a.as_f64(), b.as_f64()) {
            (Some(a_val), Some(b_val)) => Ok(cmp(a_val, b_val)),
            _ => Ok(false), // Can't compare non-numeric values
        }
    }

    async fn create_alert(&self, rule: &AlertRule, metrics: &OutputMetrics) -> Result<Alert> {
        let message = format!(
            "Alert '{}' triggered: {} (Format: {}, Timestamp: {})",
            rule.name, rule.description, metrics.format, metrics.timestamp
        );

        let alert = Alert {
            id: format!("alert_{}_{}", rule.id, Utc::now().timestamp()),
            severity: rule.severity,
            message,
            timestamp: Utc::now(),
            resolved: false,
        };

        // Add to active alerts
        let mut active_alerts = self.active_alerts.write().await;
        active_alerts.insert(alert.id.clone(), alert.clone());

        Ok(alert)
    }

    fn calculate_avg_resolution_time(&self, history: &VecDeque<Alert>) -> Option<Duration> {
        let resolved_alerts: Vec<_> = history.iter().filter(|a| a.resolved).collect();

        if resolved_alerts.is_empty() {
            return None;
        }

        // In a real implementation, we'd need to track resolution timestamps
        // For now, return a placeholder
        Some(Duration::minutes(15))
    }

    fn get_most_common_alert_type(&self, history: &VecDeque<Alert>) -> Option<String> {
        let mut type_counts = HashMap::new();

        for alert in history {
            // Extract alert type from message (simplified)
            let alert_type = if alert.message.contains("generation") {
                "Performance"
            } else if alert.message.contains("success") {
                "Success Rate"
            } else if alert.message.contains("security") {
                "Security"
            } else {
                "Other"
            };

            *type_counts.entry(alert_type.to_string()).or_insert(0) += 1;
        }

        type_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(alert_type, _)| alert_type)
    }
}

/// Alert statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStatistics {
    pub total_active: usize,
    pub total_historical: usize,
    pub critical_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub avg_resolution_time: Option<Duration>,
    pub most_common_alert_type: Option<String>,
}

/// Alert notification system
#[derive(Debug)]
pub struct AlertNotifier {
    config: NotificationConfig,
}

#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub email_enabled: bool,
    pub email_recipients: Vec<String>,
    pub slack_enabled: bool,
    pub slack_webhook_url: Option<String>,
    pub slack_channel: String,
}

impl AlertNotifier {
    /// Create a new alert notifier
    pub fn new() -> Self {
        Self {
            config: NotificationConfig {
                email_enabled: false,
                email_recipients: Vec::new(),
                slack_enabled: false,
                slack_webhook_url: None,
                slack_channel: "#alerts".to_string(),
            },
        }
    }

    /// Send alert notification
    pub async fn notify(&self, alert: &Alert) -> Result<()> {
        if self.config.email_enabled {
            self.send_email_notification(alert).await?;
        }

        if self.config.slack_enabled {
            self.send_slack_notification(alert).await?;
        }

        Ok(())
    }

    async fn send_email_notification(&self, _alert: &Alert) -> Result<()> {
        // Email sending implementation
        // This would integrate with an email service like SendGrid, AWS SES, etc.
        println!(
            "Email notification would be sent for alert: {}",
            _alert.message
        );
        Ok(())
    }

    async fn send_slack_notification(&self, _alert: &Alert) -> Result<()> {
        // Slack notification implementation
        // This would use the Slack Web API or incoming webhooks
        println!(
            "Slack notification would be sent for alert: {}",
            _alert.message
        );
        Ok(())
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AlertNotifier {
    fn default() -> Self {
        Self::new()
    }
}
