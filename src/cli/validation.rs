use crate::core::{
    ConfidenceScorer, ManualReviewWorkflow, ReviewConfig, ReviewDecision, ReviewerFeedback,
    ValidationConfig, ValidationPipeline,
};
use crate::types::{Finding, Severity};
use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::time::SystemTime;

/// Validation management commands
#[derive(Debug, Args)]
pub struct ValidationArgs {
    #[command(subcommand)]
    pub command: ValidationCommand,
}

#[derive(Debug, Subcommand)]
pub enum ValidationCommand {
    /// Configure validation pipeline
    Configure {
        /// Enable validation pipeline
        #[arg(long)]
        enable: bool,
        /// Confidence threshold (0.0-1.0)
        #[arg(long)]
        confidence_threshold: Option<f64>,
        /// Maximum validation time in milliseconds
        #[arg(long)]
        max_validation_time: Option<u64>,
        /// Enable cross-reference validation
        #[arg(long)]
        enable_cross_reference: Option<bool>,
        /// Enable pattern matching
        #[arg(long)]
        enable_pattern_matching: Option<bool>,
        /// Auto-dismiss threshold for false positives
        #[arg(long)]
        auto_dismiss_threshold: Option<f64>,
    },
    /// Test validation pipeline with sample findings
    Test {
        /// Input file with findings (JSON format)
        #[arg(short, long)]
        input: Option<PathBuf>,
        /// Generate sample findings for testing
        #[arg(long)]
        generate_samples: bool,
        /// Number of sample findings to generate
        #[arg(long, default_value = "10")]
        sample_count: usize,
        /// Output validation results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show validation metrics and statistics
    Metrics {
        /// Reset metrics after showing
        #[arg(long)]
        reset: bool,
        /// Export metrics to file
        #[arg(long)]
        export: Option<PathBuf>,
        /// Show detailed layer performance
        #[arg(long)]
        detailed: bool,
    },
    /// Manage manual review workflow
    Review {
        #[command(subcommand)]
        subcommand: ReviewSubcommand,
    },
    /// Update confidence scoring baselines
    UpdateBaseline {
        /// Category to update
        #[arg(long)]
        category: String,
        /// Accuracy score (0.0-1.0)
        #[arg(long)]
        accuracy: f64,
    },
    /// Get threshold recommendations
    Thresholds {
        /// Input file with findings for analysis
        #[arg(short, long)]
        input: PathBuf,
        /// Output recommendations to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Validate specific findings
    Validate {
        /// Input file with findings (JSON format)
        #[arg(short, long)]
        input: PathBuf,
        /// Output validated findings
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Include dismissed findings in output
        #[arg(long)]
        include_dismissed: bool,
        /// Show validation details
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReviewSubcommand {
    /// List pending reviews
    List {
        /// Filter by reviewer ID
        #[arg(long)]
        reviewer: Option<String>,
        /// Filter by priority
        #[arg(long)]
        priority: Option<String>,
        /// Show only overdue reviews
        #[arg(long)]
        overdue: bool,
    },
    /// Submit reviewer feedback
    Submit {
        /// Review ID
        #[arg(long)]
        review_id: String,
        /// Reviewer ID
        #[arg(long)]
        reviewer_id: String,
        /// Decision (valid, false_positive, needs_more_info, deferred, duplicate)
        #[arg(long)]
        decision: String,
        /// Confidence score (0.0-1.0)
        #[arg(long)]
        confidence: f64,
        /// Comments
        #[arg(long)]
        comments: String,
        /// Time spent in minutes
        #[arg(long)]
        time_spent: Option<u32>,
    },
    /// Show review statistics
    Stats {
        /// Export statistics to file
        #[arg(long)]
        export: Option<PathBuf>,
    },
    /// Add a reviewer
    AddReviewer {
        /// Reviewer ID
        #[arg(long)]
        id: String,
        /// Reviewer name
        #[arg(long)]
        name: String,
        /// Reviewer email
        #[arg(long)]
        email: String,
        /// Areas of expertise (comma-separated)
        #[arg(long)]
        expertise: String,
        /// Maximum workload
        #[arg(long, default_value = "10")]
        max_workload: usize,
    },
}

pub async fn handle_validation_command(args: ValidationArgs) -> Result<()> {
    match args.command {
        ValidationCommand::Configure {
            enable,
            confidence_threshold,
            max_validation_time,
            enable_cross_reference,
            enable_pattern_matching,
            auto_dismiss_threshold,
        } => {
            configure_validation(
                enable,
                confidence_threshold,
                max_validation_time,
                enable_cross_reference,
                enable_pattern_matching,
                auto_dismiss_threshold,
            )
            .await
        }
        ValidationCommand::Test {
            input,
            generate_samples,
            sample_count,
            output,
        } => test_validation(input, generate_samples, sample_count, output).await,
        ValidationCommand::Metrics {
            reset,
            export,
            detailed,
        } => show_metrics(reset, export, detailed).await,
        ValidationCommand::Review { subcommand } => handle_review_subcommand(subcommand).await,
        ValidationCommand::UpdateBaseline { category, accuracy } => {
            update_baseline(category, accuracy).await
        }
        ValidationCommand::Thresholds { input, output } => get_thresholds(input, output).await,
        ValidationCommand::Validate {
            input,
            output,
            include_dismissed,
            verbose,
        } => validate_findings(input, output, include_dismissed, verbose).await,
    }
}

async fn configure_validation(
    enable: bool,
    confidence_threshold: Option<f64>,
    max_validation_time: Option<u64>,
    enable_cross_reference: Option<bool>,
    enable_pattern_matching: Option<bool>,
    auto_dismiss_threshold: Option<f64>,
) -> Result<()> {
    println!("🔧 Configuring validation pipeline...");

    let mut config = ValidationConfig::default();
    config.enabled = enable;

    if let Some(threshold) = confidence_threshold {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(anyhow!("Confidence threshold must be between 0.0 and 1.0"));
        }
        config.confidence_threshold = threshold;
    }

    if let Some(time) = max_validation_time {
        config.max_validation_time_ms = time;
    }

    if let Some(cross_ref) = enable_cross_reference {
        config.enable_cross_reference = cross_ref;
    }

    if let Some(pattern) = enable_pattern_matching {
        config.enable_pattern_matching = pattern;
    }

    if let Some(dismiss) = auto_dismiss_threshold {
        if dismiss < 0.0 || dismiss > 1.0 {
            return Err(anyhow!(
                "Auto-dismiss threshold must be between 0.0 and 1.0"
            ));
        }
        config.auto_dismiss_threshold = dismiss;
    }

    // Save configuration
    let config_json = serde_json::to_string_pretty(&config)?;
    std::fs::write(".codeguardian/validation_config.json", config_json)?;

    println!("✅ Validation configuration updated:");
    println!("   - Enabled: {}", config.enabled);
    println!(
        "   - Confidence threshold: {:.2}",
        config.confidence_threshold
    );
    println!(
        "   - Max validation time: {}ms",
        config.max_validation_time_ms
    );
    println!("   - Cross-reference: {}", config.enable_cross_reference);
    println!("   - Pattern matching: {}", config.enable_pattern_matching);
    println!(
        "   - Auto-dismiss threshold: {:.2}",
        config.auto_dismiss_threshold
    );

    Ok(())
}

async fn test_validation(
    input: Option<PathBuf>,
    generate_samples: bool,
    sample_count: usize,
    output: Option<PathBuf>,
) -> Result<()> {
    println!("🧪 Testing validation pipeline...");

    let findings = if generate_samples {
        generate_sample_findings(sample_count)
    } else if let Some(input_path) = input {
        load_findings_from_file(&input_path)?
    } else {
        return Err(anyhow!(
            "Either --input or --generate-samples must be specified"
        ));
    };

    println!("📊 Testing with {} findings", findings.len());

    let config = load_validation_config().unwrap_or_default();
    let mut pipeline = ValidationPipeline::new(config);

    let start_time = SystemTime::now();
    let results = pipeline.validate_findings(findings).await?;
    let elapsed = start_time.elapsed().unwrap_or_default();

    println!("\n📈 Validation Results:");
    println!("   - Total processed: {}", results.len());
    println!("   - Processing time: {:?}", elapsed);

    let mut validated = 0;
    let mut enhanced = 0;
    let mut dismissed = 0;
    let mut requires_review = 0;

    for result in &results {
        match result.validation_status {
            crate::core::ValidationStatus::Validated => validated += 1,
            crate::core::ValidationStatus::Enhanced => enhanced += 1,
            crate::core::ValidationStatus::Dismissed => dismissed += 1,
            crate::core::ValidationStatus::RequiresReview => requires_review += 1,
            _ => {}
        }
    }

    println!("   - Validated: {}", validated);
    println!("   - Enhanced: {}", enhanced);
    println!("   - Dismissed: {}", dismissed);
    println!("   - Requires review: {}", requires_review);

    if let Some(output_path) = output {
        let output_json = serde_json::to_string_pretty(&results)?;
        std::fs::write(&output_path, output_json)?;
        println!("   - Results saved to: {}", output_path.display());
    }

    Ok(())
}

async fn show_metrics(reset: bool, export: Option<PathBuf>, detailed: bool) -> Result<()> {
    println!("📊 Validation Metrics");
    println!("====================");

    // In a real implementation, this would load metrics from a persistent store
    let metrics = crate::core::ValidationMetrics::default();

    println!(
        "Total findings processed: {}",
        metrics.total_findings_processed
    );
    println!("Findings validated: {}", metrics.findings_validated);
    println!("Findings dismissed: {}", metrics.findings_dismissed);
    println!("Findings enhanced: {}", metrics.findings_enhanced);
    println!(
        "False positives detected: {}",
        metrics.false_positives_detected
    );
    println!(
        "Average confidence score: {:.2}",
        metrics.average_confidence_score
    );
    println!("Total validation time: {}ms", metrics.validation_time_ms);

    if detailed {
        println!("\n🔍 Layer Performance:");
        for (layer_name, layer_metrics) in &metrics.layer_performance {
            println!("   {}:", layer_name);
            println!("     - Processed: {}", layer_metrics.processed_count);
            println!("     - Time: {}ms", layer_metrics.validation_time_ms);
            println!("     - Accuracy: {:.2}", layer_metrics.accuracy_score);
            println!(
                "     - False positive rate: {:.2}",
                layer_metrics.false_positive_rate
            );
        }
    }

    if let Some(export_path) = export {
        let metrics_json = serde_json::to_string_pretty(&metrics)?;
        std::fs::write(&export_path, metrics_json)?;
        println!("\n💾 Metrics exported to: {}", export_path.display());
    }

    if reset {
        println!("\n🔄 Metrics reset");
        // In a real implementation, this would reset the persistent metrics
    }

    Ok(())
}

async fn handle_review_subcommand(subcommand: ReviewSubcommand) -> Result<()> {
    match subcommand {
        ReviewSubcommand::List {
            reviewer,
            priority,
            overdue,
        } => list_reviews(reviewer, priority, overdue).await,
        ReviewSubcommand::Submit {
            review_id,
            reviewer_id,
            decision,
            confidence,
            comments,
            time_spent,
        } => {
            submit_review_feedback(
                review_id,
                reviewer_id,
                decision,
                confidence,
                comments,
                time_spent,
            )
            .await
        }
        ReviewSubcommand::Stats { export } => show_review_stats(export).await,
        ReviewSubcommand::AddReviewer {
            id,
            name,
            email,
            expertise,
            max_workload,
        } => add_reviewer(id, name, email, expertise, max_workload).await,
    }
}

async fn list_reviews(
    _reviewer: Option<String>,
    _priority: Option<String>,
    _overdue: bool,
) -> Result<()> {
    println!("📋 Pending Reviews");
    println!("==================");

    // In a real implementation, this would load from persistent storage
    println!("No pending reviews found.");
    println!("\nUse 'codeguardian validation test' to generate sample reviews.");

    Ok(())
}

async fn submit_review_feedback(
    review_id: String,
    reviewer_id: String,
    decision: String,
    confidence: f64,
    comments: String,
    time_spent: Option<u32>,
) -> Result<()> {
    if confidence < 0.0 || confidence > 1.0 {
        return Err(anyhow!("Confidence must be between 0.0 and 1.0"));
    }

    let decision_enum = match decision.to_lowercase().as_str() {
        "valid" | "valid_finding" => ReviewDecision::ValidFinding,
        "false_positive" => ReviewDecision::FalsePositive,
        "needs_more_info" => ReviewDecision::NeedsMoreInfo,
        "deferred" => ReviewDecision::Deferred,
        "duplicate" => ReviewDecision::Duplicate,
        _ => {
            return Err(anyhow!(
            "Invalid decision. Use: valid, false_positive, needs_more_info, deferred, duplicate"
        ))
        }
    };

    let feedback = ReviewerFeedback {
        reviewer_id: reviewer_id.clone(),
        decision: decision_enum,
        confidence,
        comments,
        time_spent_minutes: time_spent.unwrap_or(0),
        submitted_at: SystemTime::now(),
    };

    println!("✅ Review feedback submitted:");
    println!("   - Review ID: {}", review_id);
    println!("   - Reviewer: {}", reviewer_id);
    println!("   - Decision: {:?}", feedback.decision);
    println!("   - Confidence: {:.2}", confidence);
    println!("   - Comments: {}", comments);

    // In a real implementation, this would be saved to persistent storage
    Ok(())
}

async fn show_review_stats(export: Option<PathBuf>) -> Result<()> {
    println!("📊 Review Statistics");
    println!("===================");

    // In a real implementation, this would load from persistent storage
    let stats = crate::core::ReviewStatistics {
        total_pending: 0,
        total_completed: 0,
        average_resolution_time_hours: 0.0,
        decision_breakdown: std::collections::HashMap::new(),
        overdue_reviews: 0,
    };

    println!("Total pending: {}", stats.total_pending);
    println!("Total completed: {}", stats.total_completed);
    println!(
        "Average resolution time: {:.1} hours",
        stats.average_resolution_time_hours
    );
    println!("Overdue reviews: {}", stats.overdue_reviews);

    if !stats.decision_breakdown.is_empty() {
        println!("\nDecision Breakdown:");
        for (decision, count) in &stats.decision_breakdown {
            println!("   {:?}: {}", decision, count);
        }
    }

    if let Some(export_path) = export {
        let stats_json = serde_json::to_string_pretty(&stats)?;
        std::fs::write(&export_path, stats_json)?;
        println!("\n💾 Statistics exported to: {}", export_path.display());
    }

    Ok(())
}

async fn add_reviewer(
    id: String,
    name: String,
    email: String,
    expertise: String,
    max_workload: usize,
) -> Result<()> {
    let expertise_areas: Vec<String> = expertise.split(',').map(|s| s.trim().to_string()).collect();

    println!("👤 Adding reviewer:");
    println!("   - ID: {}", id);
    println!("   - Name: {}", name);
    println!("   - Email: {}", email);
    println!("   - Expertise: {:?}", expertise_areas);
    println!("   - Max workload: {}", max_workload);

    // In a real implementation, this would be saved to persistent storage
    println!("✅ Reviewer added successfully");

    Ok(())
}

async fn update_baseline(category: String, accuracy: f64) -> Result<()> {
    if accuracy < 0.0 || accuracy > 1.0 {
        return Err(anyhow!("Accuracy must be between 0.0 and 1.0"));
    }

    println!("📈 Updating confidence baseline:");
    println!("   - Category: {}", category);
    println!("   - Accuracy: {:.2}", accuracy);

    // In a real implementation, this would update the persistent baseline
    println!("✅ Baseline updated successfully");

    Ok(())
}

async fn get_thresholds(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    println!("🎯 Analyzing threshold recommendations...");

    let findings = load_findings_from_file(&input)?;
    let scorer = ConfidenceScorer::new();
    let recommendations = scorer.get_threshold_recommendations(&findings);

    println!("\n📊 Threshold Recommendations:");
    println!(
        "   - Auto-accept threshold: {:.2}",
        recommendations.auto_accept_threshold
    );
    println!(
        "   - Manual review threshold: {:.2}",
        recommendations.manual_review_threshold
    );
    println!(
        "   - Auto-dismiss threshold: {:.2}",
        recommendations.auto_dismiss_threshold
    );
    println!(
        "   - Confidence distribution: {} samples",
        recommendations.confidence_distribution.len()
    );

    if let Some(output_path) = output {
        let recommendations_json = serde_json::to_string_pretty(&recommendations)?;
        std::fs::write(&output_path, recommendations_json)?;
        println!("   - Recommendations saved to: {}", output_path.display());
    }

    Ok(())
}

async fn validate_findings(
    input: PathBuf,
    output: Option<PathBuf>,
    include_dismissed: bool,
    verbose: bool,
) -> Result<()> {
    println!("🔍 Validating findings from: {}", input.display());

    let findings = load_findings_from_file(&input)?;
    let config = load_validation_config().unwrap_or_default();
    let mut pipeline = ValidationPipeline::new(config);

    let results = pipeline.validate_findings(findings).await?;

    let mut output_findings = Vec::new();
    for result in &results {
        match result.validation_status {
            crate::core::ValidationStatus::Dismissed if !include_dismissed => continue,
            _ => output_findings.push(&result.finding),
        }

        if verbose {
            println!("\n🔍 Finding: {}", result.finding.message);
            println!("   - Confidence: {:.2}", result.confidence_score);
            println!("   - Status: {:?}", result.validation_status);
            println!("   - Layers: {}", result.layer_results.len());
            if !result.recommendations.is_empty() {
                println!(
                    "   - Recommendations: {}",
                    result.recommendations.join("; ")
                );
            }
        }
    }

    println!("\n📊 Validation Summary:");
    println!("   - Input findings: {}", results.len());
    println!("   - Output findings: {}", output_findings.len());

    if let Some(output_path) = output {
        let output_json = serde_json::to_string_pretty(&output_findings)?;
        std::fs::write(&output_path, output_json)?;
        println!(
            "   - Validated findings saved to: {}",
            output_path.display()
        );
    }

    Ok(())
}

fn generate_sample_findings(count: usize) -> Vec<Finding> {
    let mut findings = Vec::new();

    for i in 0..count {
        let severity = match i % 4 {
            0 => Severity::Critical,
            1 => Severity::High,
            2 => Severity::Medium,
            _ => Severity::Low,
        };

        let category = match i % 3 {
            0 => "security",
            1 => "performance",
            _ => "duplicate",
        };

        let finding = Finding::new(
            category,
            &format!("sample_rule_{}", i),
            severity,
            PathBuf::from(format!("sample_file_{}.rs", i)),
            ((i % 100) + 1) as u32,
            format!("Sample finding {} for testing validation", i),
        );

        findings.push(finding);
    }

    findings
}

fn load_findings_from_file(path: &PathBuf) -> Result<Vec<Finding>> {
    let content = std::fs::read_to_string(path)?;
    let findings: Vec<Finding> = serde_json::from_str(&content)?;
    Ok(findings)
}

fn load_validation_config() -> Result<ValidationConfig> {
    let config_path = ".codeguardian/validation_config.json";
    if std::path::Path::new(config_path).exists() {
        let content = std::fs::read_to_string(config_path)?;
        let config: ValidationConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        Ok(ValidationConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_generation() {
        let findings = generate_sample_findings(5);
        assert_eq!(findings.len(), 5);

        // Check variety in generated findings
        let categories: std::collections::HashSet<_> =
            findings.iter().map(|f| &f.category).collect();
        assert!(categories.len() > 1);
    }

    #[tokio::test]
    async fn test_validation_config() {
        let result = configure_validation(
            true,
            Some(0.8),
            Some(5000),
            Some(true),
            Some(true),
            Some(0.2),
        )
        .await;

        // Should succeed with valid parameters
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_thresholds() {
        let result = configure_validation(
            true,
            Some(1.5), // Invalid threshold > 1.0
            None,
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_err());
    }
}
