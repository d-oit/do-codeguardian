//! # CLI Command for Threshold Tuning
//!
//! Provides command-line interface for monitoring threshold tuning operations.

use crate::config::Config;
use crate::output::metrics::{ThresholdTuningManager, TuningRecommendation};
use anyhow::Result;
use clap::Args;
use serde_json;
use std::path::PathBuf;

/// Threshold tuning command arguments
#[derive(Debug, Args)]
pub struct ThresholdTuningArgs {
    /// Environment to tune thresholds for
    #[arg(short, long, default_value = "production")]
    pub environment: String,

    /// Show current threshold configurations
    #[arg(long)]
    pub show_current: bool,

    /// Generate tuning recommendations based on historical data
    #[arg(long)]
    pub recommend: bool,

    /// Apply recommended thresholds automatically
    #[arg(long)]
    pub apply_recommendations: bool,

    /// Path to historical metrics data (JSON file)
    #[arg(long)]
    pub metrics_file: Option<PathBuf>,

    /// Output format for recommendations (json, table, yaml)
    #[arg(long, default_value = "table")]
    pub output_format: String,

    /// Minimum confidence level for applying recommendations (0.0-1.0)
    #[arg(long, default_value = "0.8")]
    pub confidence_threshold: f64,

    /// Export tuned thresholds to file
    #[arg(long)]
    pub export_file: Option<PathBuf>,

    /// Import thresholds from file
    #[arg(long)]
    pub import_file: Option<PathBuf>,

    /// Show detailed analysis of current performance
    #[arg(long)]
    pub analyze: bool,
}

/// Execute threshold tuning command
pub async fn execute_threshold_tuning(args: ThresholdTuningArgs, config: &Config) -> Result<()> {
    let mut tuning_manager = ThresholdTuningManager::new();

    println!("🎯 CodeGuardian Threshold Tuning System");
    println!("Environment: {}", args.environment);
    println!();

    // Import thresholds if specified
    if let Some(import_file) = &args.import_file {
        import_thresholds(&mut tuning_manager, import_file).await?;
        println!("✅ Imported thresholds from {}", import_file.display());
        return Ok(());
    }

    // Show current thresholds
    if args.show_current {
        show_current_thresholds(&tuning_manager, &args.environment).await?;
        return Ok(());
    }

    // Generate and optionally apply recommendations
    if args.recommend || args.apply_recommendations {
        let metrics_data = if let Some(metrics_file) = &args.metrics_file {
            load_historical_metrics(metrics_file).await?
        } else {
            // Use live metrics service to get recent data
            get_recent_metrics(config).await?
        };

        let recommendations = tuning_manager
            .get_tuning_recommendations(&args.environment, &metrics_data)
            .await?;

        if recommendations.is_empty() {
            println!("✅ No tuning recommendations needed. Current thresholds are optimal.");
            return Ok(());
        }

        // Display recommendations
        display_recommendations(&recommendations, &args.output_format)?;

        // Apply recommendations if requested
        if args.apply_recommendations {
            let high_confidence_recommendations: Vec<_> = recommendations
                .into_iter()
                .filter(|r| r.confidence >= args.confidence_threshold)
                .collect();

            if high_confidence_recommendations.is_empty() {
                println!(
                    "⚠️  No high-confidence recommendations to apply (threshold: {:.2})",
                    args.confidence_threshold
                );
                return Ok(());
            }

            let updated_rules = tuning_manager
                .apply_recommendations(&args.environment, high_confidence_recommendations)
                .await?;

            println!("✅ Applied {} threshold updates", updated_rules.len());

            // Export if requested
            if let Some(export_file) = &args.export_file {
                export_thresholds(&tuning_manager, &args.environment, export_file).await?;
                println!(
                    "📄 Exported updated thresholds to {}",
                    export_file.display()
                );
            }
        }
    }

    // Detailed analysis
    if args.analyze {
        perform_detailed_analysis(&tuning_manager, &args.environment).await?;
    }

    // Generate initial thresholds for environment
    if !args.recommend && !args.show_current && !args.analyze {
        let tuned_rules = tuning_manager
            .tune_for_environment(&args.environment)
            .await?;

        println!(
            "🔧 Generated {} tuned alert rules for {} environment:",
            tuned_rules.len(),
            args.environment
        );

        for rule in &tuned_rules {
            println!("  • {} ({}ms cooldown)", rule.name, rule.cooldown_minutes);
        }

        if let Some(export_file) = &args.export_file {
            export_rules_to_file(&tuned_rules, export_file).await?;
            println!("📄 Exported rules to {}", export_file.display());
        }
    }

    Ok(())
}

/// Show current threshold configurations
async fn show_current_thresholds(
    tuning_manager: &ThresholdTuningManager,
    environment: &str,
) -> Result<()> {
    if let Some(profile) = tuning_manager.environment_profiles.get(environment) {
        println!("📊 Current Thresholds for {} Environment", profile.name);
        println!();

        println!("🚀 Performance Requirements:");
        println!(
            "  • Max Generation Time: {}ms",
            profile.performance_requirements.max_generation_time_ms
        );
        println!(
            "  • Max Memory Usage: {}MB",
            profile.performance_requirements.max_memory_usage_mb
        );
        println!(
            "  • Min Success Rate: {:.1}%",
            profile.performance_requirements.min_success_rate * 100.0
        );
        println!(
            "  • Availability Target: {:.2}%",
            profile.performance_requirements.availability_target * 100.0
        );
        println!();

        println!("🔒 Security Requirements:");
        println!(
            "  • Max Incidents/Hour: {}",
            profile.security_requirements.max_incidents_per_hour
        );
        println!(
            "  • Zero Tolerance Threats: {}",
            profile
                .security_requirements
                .zero_tolerance_threats
                .join(", ")
        );
        println!(
            "  • Scan Frequency: {} minutes",
            profile
                .security_requirements
                .security_scan_frequency_minutes
        );
        println!();

        println!("📈 Expected Load Profile:");
        println!(
            "  • Concurrent Users: {}",
            profile.expected_load.concurrent_users
        );
        println!(
            "  • Requests/Second: {:.1}",
            profile.expected_load.requests_per_second
        );
        println!(
            "  • Peak Multiplier: {:.1}x",
            profile.expected_load.peak_multiplier
        );
        println!(
            "  • Data Volume: {}MB",
            profile.expected_load.data_volume_mb
        );
    } else {
        println!("❌ Environment profile '{}' not found", environment);
        println!("Available environments: development, staging, production, enterprise");
    }

    Ok(())
}

/// Display tuning recommendations
fn display_recommendations(recommendations: &[TuningRecommendation], format: &str) -> Result<()> {
    println!("🔍 Threshold Tuning Recommendations:");
    println!();

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(recommendations)?);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(recommendations)?);
        }
        _ => {
            println!(
                "{:<20} {:<15} {:<15} {:<10} {:<50}",
                "Metric", "Current", "Recommended", "Priority", "Reasoning"
            );
            println!("{}", "-".repeat(120));

            for rec in recommendations {
                let current_str = format_metric_value(&rec.current_threshold);
                let recommended_str = format_metric_value(&rec.recommended_threshold);
                let priority_str = format!("{:?}", rec.priority);

                println!(
                    "{:<20} {:<15} {:<15} {:<10} {:<50}",
                    rec.metric_name,
                    current_str,
                    recommended_str,
                    priority_str,
                    truncate_string(&rec.reasoning, 48)
                );
            }
        }
    }

    println!();
    Ok(())
}

/// Format metric value for display
fn format_metric_value(value: &crate::output::metrics::MetricValue) -> String {
    match value {
        crate::output::metrics::MetricValue::Integer(i) => {
            if *i > 1000 {
                format!("{:.1}k", *i as f64 / 1000.0)
            } else {
                i.to_string()
            }
        }
        crate::output::metrics::MetricValue::Float(f) => {
            if *f < 1.0 {
                format!("{:.3}", f)
            } else {
                format!("{:.1}", f)
            }
        }
        crate::output::metrics::MetricValue::Boolean(b) => b.to_string(),
        crate::output::metrics::MetricValue::String(s) => s.clone(),
        crate::output::metrics::MetricValue::Array(_) => "array".to_string(),
        crate::output::metrics::MetricValue::Object(_) => "object".to_string(),
    }
}

/// Truncate string for table display
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Load historical metrics from file
async fn load_historical_metrics(
    file_path: &PathBuf,
) -> Result<Vec<crate::output::metrics::OutputMetrics>> {
    let content = tokio::fs::read_to_string(file_path).await?;
    let metrics: Vec<crate::output::metrics::OutputMetrics> = serde_json::from_str(&content)?;
    println!(
        "📊 Loaded {} historical metric samples from {}",
        metrics.len(),
        file_path.display()
    );
    Ok(metrics)
}

/// Get recent metrics from live service
async fn get_recent_metrics(
    _config: &Config,
) -> Result<Vec<crate::output::metrics::OutputMetrics>> {
    // In a real implementation, this would query the metrics service
    // For now, return empty vector to indicate no historical data
    println!("📊 Using live metrics service (no historical data file specified)");
    Ok(Vec::new())
}

/// Import thresholds from file
async fn import_thresholds(
    tuning_manager: &mut ThresholdTuningManager,
    file_path: &PathBuf,
) -> Result<()> {
    let content = tokio::fs::read_to_string(file_path).await?;
    let profile: crate::output::metrics::EnvironmentProfile = serde_json::from_str(&content)?;
    tuning_manager
        .environment_profiles
        .insert(profile.name.clone(), profile.clone());
    Ok(())
}

/// Export thresholds to file
async fn export_thresholds(
    tuning_manager: &ThresholdTuningManager,
    environment: &str,
    file_path: &PathBuf,
) -> Result<()> {
    if let Some(profile) = tuning_manager.environment_profiles.get(environment) {
        let content = serde_json::to_string_pretty(profile)?;
        tokio::fs::write(file_path, content).await?;
    } else {
        return Err(anyhow::anyhow!(
            "Environment profile not found: {}",
            environment
        ));
    }
    Ok(())
}

/// Export rules to file
async fn export_rules_to_file(
    rules: &[crate::output::metrics::alerts::AlertRule],
    file_path: &PathBuf,
) -> Result<()> {
    let content = serde_json::to_string_pretty(rules)?;
    tokio::fs::write(file_path, content).await?;
    Ok(())
}

/// Perform detailed analysis
async fn perform_detailed_analysis(
    tuning_manager: &ThresholdTuningManager,
    environment: &str,
) -> Result<()> {
    println!("🔬 Detailed Analysis for {} Environment", environment);
    println!();

    if let Some(profile) = tuning_manager.environment_profiles.get(environment) {
        // Performance analysis
        println!("📈 Performance Analysis:");
        let max_time = profile.performance_requirements.max_generation_time_ms;
        let warning_threshold = (max_time as f64 * 0.8) as u64;
        let critical_threshold = (max_time as f64 * 0.95) as u64;

        println!("  • Warning at: {}ms (80% of max)", warning_threshold);
        println!("  • Critical at: {}ms (95% of max)", critical_threshold);
        println!("  • Maximum allowed: {}ms", max_time);
        println!();

        // Memory analysis
        println!("💾 Memory Analysis:");
        let max_memory = profile.performance_requirements.max_memory_usage_mb;
        let warning_memory = (max_memory as f64 * 0.8) as u64;
        let critical_memory = (max_memory as f64 * 0.95) as u64;

        println!("  • Warning at: {}MB (80% of max)", warning_memory);
        println!("  • Critical at: {}MB (95% of max)", critical_memory);
        println!("  • Maximum allowed: {}MB", max_memory);
        println!();

        // Success rate analysis
        println!("✅ Success Rate Analysis:");
        let min_success = profile.performance_requirements.min_success_rate;
        let warning_success = min_success - 0.01;
        let critical_success = min_success - 0.05;

        println!("  • Target: {:.1}%", min_success * 100.0);
        println!("  • Warning below: {:.1}%", warning_success * 100.0);
        println!(
            "  • Critical below: {:.1}%",
            critical_success.max(0.5) * 100.0
        );
        println!();

        // Security analysis
        println!("🔒 Security Analysis:");
        println!(
            "  • Max incidents/hour: {}",
            profile.security_requirements.max_incidents_per_hour
        );
        println!(
            "  • Zero tolerance for: {}",
            profile
                .security_requirements
                .zero_tolerance_threats
                .join(", ")
        );
        println!(
            "  • Response time target: {} minutes",
            profile
                .security_requirements
                .vulnerability_response_time_minutes
        );
        println!();

        // Recommendations
        println!("💡 Optimization Recommendations:");
        if environment == "development" {
            println!("  • Consider using more lenient thresholds for faster development");
            println!("  • Focus on major performance regressions rather than minor fluctuations");
        } else if environment == "production" {
            println!("  • Ensure alerting is responsive but not overly sensitive");
            println!("  • Consider implementing gradual threshold tightening");
            println!("  • Monitor user experience metrics closely");
        } else if environment == "enterprise" {
            println!("  • Implement very tight monitoring with rapid response");
            println!("  • Consider predictive alerting based on trends");
            println!("  • Ensure 24/7 monitoring coverage");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_metric_value() {
        use crate::output::metrics::MetricValue;

        assert_eq!(format_metric_value(&MetricValue::Integer(500)), "500");
        assert_eq!(format_metric_value(&MetricValue::Integer(1500)), "1.5k");
        assert_eq!(format_metric_value(&MetricValue::Float(0.995)), "0.995");
        assert_eq!(format_metric_value(&MetricValue::Float(4.2)), "4.2");
        assert_eq!(format_metric_value(&MetricValue::Boolean(true)), "true");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(
            truncate_string("this is a very long string", 10),
            "this is..."
        );
    }
}
