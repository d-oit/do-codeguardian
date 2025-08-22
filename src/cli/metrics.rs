use crate::cli::MetricsArgs;
use crate::ml::MLClassifier;
use anyhow::Result;
use std::path::Path;

pub async fn run(args: MetricsArgs) -> Result<()> {
    // Check if model file exists
    if !Path::new(&args.model_path).exists() {
        eprintln!("❌ Model file not found: {}", args.model_path);
        eprintln!("💡 Train a model first with: codeguardian train --bootstrap");
        return Ok(());
    }

    // Load the ML classifier
    let mut classifier = MLClassifier::new(Some(&args.model_path));

    if !classifier.is_metrics_enabled() {
        eprintln!("⚠️  ML classifier not enabled or no metrics available");
        eprintln!("💡 Run some analysis with --ml-model to collect metrics");
        return Ok(());
    }

    match args.command {
        crate::cli::MetricsCommand::Show => {
            // Generate and display metrics report
            let report = classifier.generate_metrics_report();
            println!("{}", report);
        }
        crate::cli::MetricsCommand::Export { output } => {
            // Export metrics to JSON file
            classifier.export_metrics(&output)?;
            if !args.quiet {
                println!("✅ Metrics exported to: {}", output);
            }
        }
        crate::cli::MetricsCommand::Summary => {
            // Show condensed summary
            if let Some(metrics) = classifier.get_metrics() {
                print_metrics_summary(metrics);
            } else {
                println!("No metrics available");
            }
        }
    }

    Ok(())
}

fn print_metrics_summary(metrics: &crate::ml::ModelMetrics) {
    println!("📊 ML Model Metrics Summary");
    println!("===========================");

    // Training info
    let tm = &metrics.training_metrics;
    println!(
        "🎯 Training: {} examples, {:.6} error, {}ms",
        tm.dataset_size, tm.final_training_error, tm.training_duration_ms
    );

    // Inference performance
    let im = &metrics.inference_metrics;
    if im.total_predictions > 0 {
        println!(
            "⚡ Inference: {} predictions, {:.1}ms avg, {:.1}/sec",
            im.total_predictions, im.avg_inference_time_ms, im.predictions_per_second
        );
    }

    // Classification accuracy
    let cm = &metrics.classification_metrics;
    let total_classified =
        cm.true_positives + cm.false_positives + cm.true_negatives + cm.false_negatives;
    if total_classified > 0 {
        println!(
            "🎯 Accuracy: {:.1}% (P: {:.1}%, R: {:.1}%, F1: {:.1}%)",
            cm.accuracy * 100.0,
            cm.precision * 100.0,
            cm.recall * 100.0,
            cm.f1_score * 100.0
        );
    }

    // Alerts
    let alerts = &metrics.temporal_metrics.performance_alerts;
    if !alerts.is_empty() {
        println!("🚨 Alerts: {} active", alerts.len());
        for alert in alerts.iter().take(3) {
            let severity_emoji = match alert.severity {
                crate::ml::metrics::AlertSeverity::Info => "ℹ️",
                crate::ml::metrics::AlertSeverity::Warning => "⚠️",
                crate::ml::metrics::AlertSeverity::Critical => "🔴",
            };
            println!("   {} {}", severity_emoji, alert.message);
        }
        if alerts.len() > 3 {
            println!("   ... and {} more", alerts.len() - 3);
        }
    }
}
