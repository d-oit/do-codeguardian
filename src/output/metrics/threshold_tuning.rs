//! # Threshold Tuning System
//!
//! Provides intelligent threshold tuning capabilities for monitoring alerts
//! based on production requirements, historical data, and performance characteristics.

use super::alerts::{AlertAggregation, AlertCondition, AlertOperator, AlertRule, AlertSeverity};
use super::types::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Threshold tuning manager
#[derive(Debug)]
pub struct ThresholdTuningManager {
    pub baseline_analyzer: BaselineAnalyzer,
    pub environment_profiles: HashMap<String, EnvironmentProfile>,
    pub adaptive_thresholds: AdaptiveThresholdEngine,
    pub tuning_history: Vec<TuningEvent>,
}

/// Environment profiles for different deployment contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentProfile {
    pub name: String,
    pub description: String,
    pub expected_load: LoadProfile,
    pub performance_requirements: PerformanceRequirements,
    pub security_requirements: SecurityRequirements,
    pub alert_thresholds: HashMap<String, ThresholdConfiguration>,
}

/// Load profile for different environments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProfile {
    pub concurrent_users: u32,
    pub requests_per_second: f64,
    pub peak_multiplier: f64,
    pub data_volume_mb: u64,
    pub complexity_factor: f64,
}

/// Performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_generation_time_ms: u64,
    pub max_memory_usage_mb: u64,
    pub min_success_rate: f64,
    pub max_error_rate: f64,
    pub availability_target: f64, // 99.9% = 0.999
}

/// Security requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub max_incidents_per_hour: u32,
    pub zero_tolerance_threats: Vec<String>,
    pub security_scan_frequency_minutes: u64,
    pub vulnerability_response_time_minutes: u64,
}

/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfiguration {
    pub metric_name: String,
    pub warning_threshold: MetricValue,
    pub critical_threshold: MetricValue,
    pub auto_scaling_threshold: Option<MetricValue>,
    pub baseline_value: Option<MetricValue>,
    pub variance_tolerance: f64,
    pub trend_sensitivity: f64,
}

impl ThresholdTuningManager {
    /// Create a new threshold tuning manager
    pub fn new() -> Self {
        let mut manager = Self {
            baseline_analyzer: BaselineAnalyzer::new(),
            environment_profiles: HashMap::new(),
            adaptive_thresholds: AdaptiveThresholdEngine::new(),
            tuning_history: Vec::new(),
        };

        manager.initialize_environment_profiles();
        manager
    }

    /// Tune thresholds for a specific environment
    pub async fn tune_for_environment(&mut self, environment: &str) -> Result<Vec<AlertRule>> {
        let profile = self
            .environment_profiles
            .get(environment)
            .ok_or_else(|| anyhow::anyhow!("Environment profile not found: {}", environment))?;

        let mut tuned_rules = Vec::new();

        // Performance thresholds
        tuned_rules.extend(self.create_performance_rules(profile)?);

        // Success rate thresholds
        tuned_rules.extend(self.create_success_rate_rules(profile)?);

        // Security thresholds
        tuned_rules.extend(self.create_security_rules(profile)?);

        // Memory usage thresholds
        tuned_rules.extend(self.create_memory_rules(profile)?);

        // User experience thresholds
        tuned_rules.extend(self.create_user_experience_rules(profile)?);

        // Record tuning event
        self.tuning_history.push(TuningEvent {
            timestamp: Utc::now(),
            environment: environment.to_string(),
            rules_created: tuned_rules.len(),
            trigger: TuningTrigger::Manual,
            baseline_metrics: None,
        });

        Ok(tuned_rules)
    }

    /// Initialize predefined environment profiles
    fn initialize_environment_profiles(&mut self) {
        // Development environment
        self.environment_profiles.insert(
            "development".to_string(),
            EnvironmentProfile {
                name: "Development".to_string(),
                description: "Local development and testing environment".to_string(),
                expected_load: LoadProfile {
                    concurrent_users: 1,
                    requests_per_second: 1.0,
                    peak_multiplier: 2.0,
                    data_volume_mb: 10,
                    complexity_factor: 1.0,
                },
                performance_requirements: PerformanceRequirements {
                    max_generation_time_ms: 30000, // 30 seconds is acceptable for dev
                    max_memory_usage_mb: 1024,     // 1GB
                    min_success_rate: 0.8,         // 80% is acceptable for dev
                    max_error_rate: 0.2,
                    availability_target: 0.95, // 95%
                },
                security_requirements: SecurityRequirements {
                    max_incidents_per_hour: 10,
                    zero_tolerance_threats: vec!["code_injection".to_string()],
                    security_scan_frequency_minutes: 60,
                    vulnerability_response_time_minutes: 480, // 8 hours
                },
                alert_thresholds: HashMap::new(),
            },
        );

        // Staging environment
        self.environment_profiles.insert(
            "staging".to_string(),
            EnvironmentProfile {
                name: "Staging".to_string(),
                description: "Pre-production staging environment".to_string(),
                expected_load: LoadProfile {
                    concurrent_users: 10,
                    requests_per_second: 5.0,
                    peak_multiplier: 3.0,
                    data_volume_mb: 100,
                    complexity_factor: 1.2,
                },
                performance_requirements: PerformanceRequirements {
                    max_generation_time_ms: 10000, // 10 seconds
                    max_memory_usage_mb: 2048,     // 2GB
                    min_success_rate: 0.95,        // 95%
                    max_error_rate: 0.05,
                    availability_target: 0.99, // 99%
                },
                security_requirements: SecurityRequirements {
                    max_incidents_per_hour: 5,
                    zero_tolerance_threats: vec!["code_injection".to_string(), "xss".to_string()],
                    security_scan_frequency_minutes: 30,
                    vulnerability_response_time_minutes: 120, // 2 hours
                },
                alert_thresholds: HashMap::new(),
            },
        );

        // Production environment
        self.environment_profiles.insert(
            "production".to_string(),
            EnvironmentProfile {
                name: "Production".to_string(),
                description: "Live production environment".to_string(),
                expected_load: LoadProfile {
                    concurrent_users: 1000,
                    requests_per_second: 50.0,
                    peak_multiplier: 5.0,
                    data_volume_mb: 1000,
                    complexity_factor: 1.5,
                },
                performance_requirements: PerformanceRequirements {
                    max_generation_time_ms: 5000, // 5 seconds
                    max_memory_usage_mb: 4096,    // 4GB
                    min_success_rate: 0.999,      // 99.9%
                    max_error_rate: 0.001,
                    availability_target: 0.9999, // 99.99%
                },
                security_requirements: SecurityRequirements {
                    max_incidents_per_hour: 1,
                    zero_tolerance_threats: vec![
                        "code_injection".to_string(),
                        "xss".to_string(),
                        "sql_injection".to_string(),
                        "unauthorized_access".to_string(),
                    ],
                    security_scan_frequency_minutes: 15,
                    vulnerability_response_time_minutes: 30, // 30 minutes
                },
                alert_thresholds: HashMap::new(),
            },
        );

        // Enterprise environment
        self.environment_profiles.insert(
            "enterprise".to_string(),
            EnvironmentProfile {
                name: "Enterprise".to_string(),
                description: "High-scale enterprise environment".to_string(),
                expected_load: LoadProfile {
                    concurrent_users: 10000,
                    requests_per_second: 500.0,
                    peak_multiplier: 10.0,
                    data_volume_mb: 10000,
                    complexity_factor: 2.0,
                },
                performance_requirements: PerformanceRequirements {
                    max_generation_time_ms: 2000, // 2 seconds
                    max_memory_usage_mb: 8192,    // 8GB
                    min_success_rate: 0.9999,     // 99.99%
                    max_error_rate: 0.0001,
                    availability_target: 0.99999, // 99.999%
                },
                security_requirements: SecurityRequirements {
                    max_incidents_per_hour: 0,
                    zero_tolerance_threats: vec![
                        "code_injection".to_string(),
                        "xss".to_string(),
                        "sql_injection".to_string(),
                        "unauthorized_access".to_string(),
                        "data_breach".to_string(),
                        "privilege_escalation".to_string(),
                    ],
                    security_scan_frequency_minutes: 5,
                    vulnerability_response_time_minutes: 15, // 15 minutes
                },
                alert_thresholds: HashMap::new(),
            },
        );
    }

    /// Create performance-related alert rules
    fn create_performance_rules(&self, profile: &EnvironmentProfile) -> Result<Vec<AlertRule>> {
        let mut rules = Vec::new();

        // Generation time warning (80% of max)
        rules.push(AlertRule {
            id: format!(
                "perf_generation_time_warning_{}",
                profile.name.to_lowercase()
            ),
            name: format!("Generation Time Warning - {}", profile.name),
            description: format!(
                "Output generation time approaching limit for {} environment",
                profile.name
            ),
            metric_name: "generation_time_ms".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(
                    (profile.performance_requirements.max_generation_time_ms as f64 * 0.8) as i64,
                ),
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 5 } else { 15 },
            last_triggered: None,
        });

        // Generation time critical (95% of max)
        rules.push(AlertRule {
            id: format!(
                "perf_generation_time_critical_{}",
                profile.name.to_lowercase()
            ),
            name: format!("Generation Time Critical - {}", profile.name),
            description: format!(
                "Output generation time exceeds critical threshold for {} environment",
                profile.name
            ),
            metric_name: "generation_time_ms".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(
                    (profile.performance_requirements.max_generation_time_ms as f64 * 0.95) as i64,
                ),
                time_window_seconds: 180,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 2 } else { 10 },
            last_triggered: None,
        });

        Ok(rules)
    }

    /// Create success rate alert rules
    fn create_success_rate_rules(&self, profile: &EnvironmentProfile) -> Result<Vec<AlertRule>> {
        let mut rules = Vec::new();

        // Success rate warning (slightly below target)
        let warning_threshold = profile.performance_requirements.min_success_rate - 0.01;
        rules.push(AlertRule {
            id: format!("success_rate_warning_{}", profile.name.to_lowercase()),
            name: format!("Success Rate Warning - {}", profile.name),
            description: format!(
                "Success rate below optimal for {} environment",
                profile.name
            ),
            metric_name: "success_rate".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::LessThan,
                threshold: MetricValue::Float(warning_threshold),
                time_window_seconds: 600,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 10 } else { 30 },
            last_triggered: None,
        });

        // Success rate critical (significantly below target)
        let critical_threshold = profile.performance_requirements.min_success_rate - 0.05;
        rules.push(AlertRule {
            id: format!("success_rate_critical_{}", profile.name.to_lowercase()),
            name: format!("Success Rate Critical - {}", profile.name),
            description: format!(
                "Success rate critically low for {} environment",
                profile.name
            ),
            metric_name: "success_rate".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::LessThan,
                threshold: MetricValue::Float(critical_threshold.max(0.5)), // Never go below 50%
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 5 } else { 15 },
            last_triggered: None,
        });

        Ok(rules)
    }

    /// Create security alert rules
    fn create_security_rules(&self, profile: &EnvironmentProfile) -> Result<Vec<AlertRule>> {
        let mut rules = Vec::new();

        // Security incidents threshold
        rules.push(AlertRule {
            id: format!("security_incidents_{}", profile.name.to_lowercase()),
            name: format!("Security Incidents - {}", profile.name),
            description: format!(
                "Security incidents detected in {} environment",
                profile.name
            ),
            metric_name: "security_incidents".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(
                    profile.security_requirements.max_incidents_per_hour as i64,
                ),
                time_window_seconds: 3600, // 1 hour
                aggregation: AlertAggregation::Sum,
            },
            severity: if profile.name == "Production" {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 5 } else { 30 },
            last_triggered: None,
        });

        // Zero-tolerance threats (immediate critical alert)
        for threat in &profile.security_requirements.zero_tolerance_threats {
            rules.push(AlertRule {
                id: format!(
                    "security_zero_tolerance_{}_{}",
                    threat,
                    profile.name.to_lowercase()
                ),
                name: format!("Zero Tolerance Threat: {} - {}", threat, profile.name),
                description: format!(
                    "Zero tolerance security threat '{}' detected in {} environment",
                    threat, profile.name
                ),
                metric_name: format!("security_threat_{}", threat),
                condition: AlertCondition {
                    operator: AlertOperator::GreaterThan,
                    threshold: MetricValue::Integer(0),
                    time_window_seconds: 60,
                    aggregation: AlertAggregation::Sum,
                },
                severity: AlertSeverity::Critical,
                enabled: true,
                cooldown_minutes: 1, // Very short cooldown for security
                last_triggered: None,
            });
        }

        Ok(rules)
    }

    /// Create memory usage alert rules
    fn create_memory_rules(&self, profile: &EnvironmentProfile) -> Result<Vec<AlertRule>> {
        let mut rules = Vec::new();

        // Memory warning (80% of max)
        rules.push(AlertRule {
            id: format!("memory_warning_{}", profile.name.to_lowercase()),
            name: format!("Memory Usage Warning - {}", profile.name),
            description: format!("Memory usage high in {} environment", profile.name),
            metric_name: "memory_usage_bytes".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(
                    (profile.performance_requirements.max_memory_usage_mb * 1024 * 1024) as i64
                        * 80
                        / 100,
                ),
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 10 } else { 20 },
            last_triggered: None,
        });

        // Memory critical (95% of max)
        rules.push(AlertRule {
            id: format!("memory_critical_{}", profile.name.to_lowercase()),
            name: format!("Memory Usage Critical - {}", profile.name),
            description: format!(
                "Memory usage critically high in {} environment",
                profile.name
            ),
            metric_name: "memory_usage_bytes".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::GreaterThan,
                threshold: MetricValue::Integer(
                    (profile.performance_requirements.max_memory_usage_mb * 1024 * 1024) as i64
                        * 95
                        / 100,
                ),
                time_window_seconds: 180,
                aggregation: AlertAggregation::Max,
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 5 } else { 15 },
            last_triggered: None,
        });

        Ok(rules)
    }

    /// Create user experience alert rules
    fn create_user_experience_rules(&self, profile: &EnvironmentProfile) -> Result<Vec<AlertRule>> {
        let mut rules = Vec::new();

        // Satisfaction score thresholds based on environment
        let min_satisfaction = match profile.name.as_str() {
            "Production" | "Enterprise" => 4.5,
            "Staging" => 4.0,
            _ => 3.5,
        };

        rules.push(AlertRule {
            id: format!("user_satisfaction_{}", profile.name.to_lowercase()),
            name: format!("User Satisfaction - {}", profile.name),
            description: format!(
                "User satisfaction below target in {} environment",
                profile.name
            ),
            metric_name: "satisfaction_score".to_string(),
            condition: AlertCondition {
                operator: AlertOperator::LessThan,
                threshold: MetricValue::Float(min_satisfaction),
                time_window_seconds: 1800, // 30 minutes
                aggregation: AlertAggregation::Average,
            },
            severity: if profile.name == "Production" {
                AlertSeverity::Warning
            } else {
                AlertSeverity::Info
            },
            enabled: true,
            cooldown_minutes: if profile.name == "Production" { 30 } else { 60 },
            last_triggered: None,
        });

        Ok(rules)
    }

    /// Get recommendations for threshold adjustments
    pub async fn get_tuning_recommendations(
        &self,
        environment: &str,
        metrics_history: &[OutputMetrics],
    ) -> Result<Vec<TuningRecommendation>> {
        let profile = self
            .environment_profiles
            .get(environment)
            .ok_or_else(|| anyhow::anyhow!("Environment profile not found: {}", environment))?;

        let mut recommendations = Vec::new();

        // Analyze performance patterns
        recommendations.extend(self.analyze_performance_patterns(profile, metrics_history)?);

        // Analyze error patterns
        recommendations.extend(self.analyze_error_patterns(profile, metrics_history)?);

        // Analyze resource utilization
        recommendations.extend(self.analyze_resource_patterns(profile, metrics_history)?);

        Ok(recommendations)
    }

    fn analyze_performance_patterns(
        &self,
        profile: &EnvironmentProfile,
        metrics: &[OutputMetrics],
    ) -> Result<Vec<TuningRecommendation>> {
        let mut recommendations = Vec::new();

        if metrics.is_empty() {
            return Ok(recommendations);
        }

        // Calculate performance statistics
        let avg_generation_time = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .sum::<u64>() as f64
            / metrics.len() as f64;

        let _max_generation_time = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .max()
            .unwrap_or(0);

        let p95_generation_time = self.calculate_percentile(
            &metrics
                .iter()
                .map(|m| m.performance.generation_time_ms as f64)
                .collect::<Vec<_>>(),
            0.95,
        );

        // Current threshold
        let current_threshold = profile.performance_requirements.max_generation_time_ms;

        // Recommendations based on analysis
        if avg_generation_time < (current_threshold as f64 * 0.5) {
            recommendations.push(TuningRecommendation {
                metric_name: "generation_time_ms".to_string(),
                current_threshold: MetricValue::Integer(current_threshold as i64),
                recommended_threshold: MetricValue::Integer((avg_generation_time * 2.0) as i64),
                reasoning: format!(
                    "Average generation time ({:.0}ms) is much lower than current threshold ({}ms). Consider lowering threshold for faster detection.",
                    avg_generation_time, current_threshold
                ),
                confidence: 0.8,
                priority: RecommendationPriority::Medium,
            });
        } else if p95_generation_time > (current_threshold as f64 * 0.9) {
            recommendations.push(TuningRecommendation {
                metric_name: "generation_time_ms".to_string(),
                current_threshold: MetricValue::Integer(current_threshold as i64),
                recommended_threshold: MetricValue::Integer((p95_generation_time * 1.2) as i64),
                reasoning: format!(
                    "95th percentile ({:.0}ms) is close to threshold ({}ms). Consider raising threshold to reduce false positives.",
                    p95_generation_time, current_threshold
                ),
                confidence: 0.9,
                priority: RecommendationPriority::High,
            });
        }

        Ok(recommendations)
    }

    fn analyze_error_patterns(
        &self,
        profile: &EnvironmentProfile,
        metrics: &[OutputMetrics],
    ) -> Result<Vec<TuningRecommendation>> {
        let mut recommendations = Vec::new();

        if metrics.is_empty() {
            return Ok(recommendations);
        }

        let success_rate = metrics.iter().filter(|m| m.functionality.success).count() as f64
            / metrics.len() as f64;

        let current_threshold = profile.performance_requirements.min_success_rate;

        if success_rate > (current_threshold + 0.05) {
            recommendations.push(TuningRecommendation {
                metric_name: "success_rate".to_string(),
                current_threshold: MetricValue::Float(current_threshold),
                recommended_threshold: MetricValue::Float((success_rate - 0.02).max(0.95)),
                reasoning: format!(
                    "Actual success rate ({:.3}) consistently exceeds threshold ({:.3}). Consider raising threshold for better quality assurance.",
                    success_rate, current_threshold
                ),
                confidence: 0.85,
                priority: RecommendationPriority::Medium,
            });
        } else if success_rate < (current_threshold - 0.02) {
            recommendations.push(TuningRecommendation {
                metric_name: "success_rate".to_string(),
                current_threshold: MetricValue::Float(current_threshold),
                recommended_threshold: MetricValue::Float((success_rate + 0.01).min(current_threshold)),
                reasoning: format!(
                    "Actual success rate ({:.3}) below threshold ({:.3}). Consider temporarily lowering threshold while addressing root causes.",
                    success_rate, current_threshold
                ),
                confidence: 0.7,
                priority: RecommendationPriority::High,
            });
        }

        Ok(recommendations)
    }

    fn analyze_resource_patterns(
        &self,
        profile: &EnvironmentProfile,
        metrics: &[OutputMetrics],
    ) -> Result<Vec<TuningRecommendation>> {
        let mut recommendations = Vec::new();

        if metrics.is_empty() {
            return Ok(recommendations);
        }

        let avg_memory_usage = metrics
            .iter()
            .map(|m| m.performance.memory_usage_bytes)
            .sum::<u64>() as f64
            / metrics.len() as f64;

        let current_threshold_bytes =
            profile.performance_requirements.max_memory_usage_mb * 1024 * 1024;

        if avg_memory_usage < (current_threshold_bytes as f64 * 0.3) {
            recommendations.push(TuningRecommendation {
                metric_name: "memory_usage_bytes".to_string(),
                current_threshold: MetricValue::Integer(current_threshold_bytes as i64),
                recommended_threshold: MetricValue::Integer((avg_memory_usage * 3.0) as i64),
                reasoning: format!(
                    "Average memory usage ({:.1}MB) is much lower than threshold ({:.1}MB). Consider lowering for better resource monitoring.",
                    avg_memory_usage / (1024.0 * 1024.0),
                    current_threshold_bytes as f64 / (1024.0 * 1024.0)
                ),
                confidence: 0.75,
                priority: RecommendationPriority::Low,
            });
        }

        Ok(recommendations)
    }

    fn calculate_percentile(&self, values: &[f64], percentile: f64) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let index = (percentile * (sorted.len() - 1) as f64).round() as usize;
        sorted[index.min(sorted.len() - 1)]
    }

    /// Apply recommended thresholds
    pub async fn apply_recommendations(
        &mut self,
        environment: &str,
        recommendations: Vec<TuningRecommendation>,
    ) -> Result<Vec<AlertRule>> {
        let mut updated_rules = Vec::new();

        for recommendation in recommendations {
            if recommendation.confidence >= 0.7 {
                // Only apply high-confidence recommendations
                // Create updated rule with new threshold
                let updated_rule = self.create_updated_rule(environment, &recommendation)?;
                updated_rules.push(updated_rule);

                // Record the tuning event
                self.tuning_history.push(TuningEvent {
                    timestamp: Utc::now(),
                    environment: environment.to_string(),
                    rules_created: 1,
                    trigger: TuningTrigger::Recommendation,
                    baseline_metrics: None,
                });
            }
        }

        Ok(updated_rules)
    }

    fn create_updated_rule(
        &self,
        environment: &str,
        recommendation: &TuningRecommendation,
    ) -> Result<AlertRule> {
        let profile = self
            .environment_profiles
            .get(environment)
            .ok_or_else(|| anyhow::anyhow!("Environment profile not found: {}", environment))?;

        Ok(AlertRule {
            id: format!(
                "tuned_{}_{}",
                recommendation.metric_name,
                environment.to_lowercase()
            ),
            name: format!("Tuned {} - {}", recommendation.metric_name, profile.name),
            description: format!(
                "Auto-tuned threshold for {} in {} environment",
                recommendation.metric_name, profile.name
            ),
            metric_name: recommendation.metric_name.clone(),
            condition: AlertCondition {
                operator: if recommendation.metric_name.contains("time")
                    || recommendation.metric_name.contains("usage")
                {
                    AlertOperator::GreaterThan
                } else {
                    AlertOperator::LessThan
                },
                threshold: recommendation.recommended_threshold.clone(),
                time_window_seconds: 300,
                aggregation: AlertAggregation::Average,
            },
            severity: match recommendation.priority {
                RecommendationPriority::High => AlertSeverity::Critical,
                RecommendationPriority::Medium => AlertSeverity::Warning,
                RecommendationPriority::Low => AlertSeverity::Info,
            },
            enabled: true,
            cooldown_minutes: if environment == "production" { 10 } else { 20 },
            last_triggered: None,
        })
    }
}

/// Baseline analyzer for establishing performance baselines
#[derive(Debug)]
pub struct BaselineAnalyzer {
    pub baselines: HashMap<String, PerformanceBaseline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub metric_name: String,
    pub baseline_value: f64,
    pub standard_deviation: f64,
    pub sample_count: u64,
    pub last_updated: DateTime<Utc>,
    pub confidence_interval: (f64, f64),
}

impl BaselineAnalyzer {
    pub fn new() -> Self {
        Self {
            baselines: HashMap::new(),
        }
    }

    /// Establish baseline from historical metrics
    pub fn establish_baseline(
        &mut self,
        metric_name: &str,
        values: &[f64],
    ) -> Result<PerformanceBaseline> {
        if values.is_empty() {
            return Err(anyhow::anyhow!("Cannot establish baseline from empty data"));
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        // 95% confidence interval
        let margin = 1.96 * std_dev / (values.len() as f64).sqrt();
        let confidence_interval = (mean - margin, mean + margin);

        let baseline = PerformanceBaseline {
            metric_name: metric_name.to_string(),
            baseline_value: mean,
            standard_deviation: std_dev,
            sample_count: values.len() as u64,
            last_updated: Utc::now(),
            confidence_interval,
        };

        self.baselines
            .insert(metric_name.to_string(), baseline.clone());
        Ok(baseline)
    }
}

/// Adaptive threshold engine
#[derive(Debug)]
pub struct AdaptiveThresholdEngine {
    pub adaptation_rate: f64,
    pub min_samples: u64,
    pub max_adaptation_percent: f64,
}

impl AdaptiveThresholdEngine {
    pub fn new() -> Self {
        Self {
            adaptation_rate: 0.1,        // 10% adaptation rate
            min_samples: 100,            // Minimum samples before adaptation
            max_adaptation_percent: 0.5, // Maximum 50% change
        }
    }

    /// Calculate adaptive threshold based on recent performance
    pub fn calculate_adaptive_threshold(
        &self,
        baseline: &PerformanceBaseline,
        recent_values: &[f64],
    ) -> Option<f64> {
        if recent_values.len() < self.min_samples as usize {
            return None;
        }

        let recent_mean = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
        let adaptation = (recent_mean - baseline.baseline_value) * self.adaptation_rate;

        // Limit adaptation to prevent extreme changes
        let max_change = baseline.baseline_value * self.max_adaptation_percent;
        let limited_adaptation = adaptation.max(-max_change).min(max_change);

        Some(baseline.baseline_value + limited_adaptation)
    }
}

/// Tuning recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningRecommendation {
    pub metric_name: String,
    pub current_threshold: MetricValue,
    pub recommended_threshold: MetricValue,
    pub reasoning: String,
    pub confidence: f64,
    pub priority: RecommendationPriority,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
}

/// Tuning event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningEvent {
    pub timestamp: DateTime<Utc>,
    pub environment: String,
    pub rules_created: usize,
    pub trigger: TuningTrigger,
    pub baseline_metrics: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuningTrigger {
    Manual,
    Scheduled,
    Recommendation,
    PerformanceDegradation,
    EnvironmentChange,
}

impl Default for ThresholdTuningManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BaselineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AdaptiveThresholdEngine {
    fn default() -> Self {
        Self::new()
    }
}
