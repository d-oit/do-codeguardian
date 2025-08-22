#![allow(dead_code)]

use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

// Constants for ML convergence detection and thresholds
const CONVERGENCE_LOSS_THRESHOLD: f64 = 1e-4; // Minimum loss improvement to consider convergence
const CONVERGENCE_WINDOW_SIZE: usize = 5; // Number of epochs to check for stability
const DEFAULT_CONFIDENCE_THRESHOLD: f32 = 0.3; // Default threshold for filtering findings
const SLIDING_WINDOW_SIZE: usize = 1000; // Size of sliding window for real-time metrics
const MIN_SAMPLES_FOR_P95: usize = 20; // Minimum samples needed for P95 calculation
const P95_PERCENTILE: f32 = 0.95; // P95 percentile value
const MIN_WINDOW_SIZE_TEMPORAL: usize = 10; // Minimum window size for temporal analysis
const MAX_RECENT_RECORDS: usize = 100; // Maximum recent records for analysis
const MIN_LABELED_RECORDS: usize = 5; // Minimum labeled records for accuracy calculation
const MAX_DAILY_ACCURACY_DAYS: usize = 30; // Days to keep daily accuracy data
const MAX_WEEKLY_THROUGHPUT_WEEKS: usize = 12; // Weeks to keep throughput data
const ACCURACY_ALERT_THRESHOLD: f32 = 0.8; // Threshold for accuracy drop alerts
const MIN_CLASSIFIED_SAMPLES: u64 = 100; // Minimum samples for accuracy alerts
const LATENCY_ALERT_THRESHOLD_MS: f32 = 50.0; // Threshold for latency increase alerts
const CONFIDENCE_CALIBRATION_RANGE: std::ops::RangeInclusive<f32> = 0.3..=0.9; // Acceptable confidence range
const CONFIDENCE_CALIBRATION_TARGET: f32 = 0.5; // Target confidence for calibration
const RECOMMENDATION_ACCURACY_THRESHOLD: f32 = 0.85; // Threshold for accuracy recommendations
const RECOMMENDATION_FPR_THRESHOLD: f32 = 0.1; // Threshold for false positive rate recommendations
const RECOMMENDATION_INFERENCE_TIME_THRESHOLD_MS: f32 = 20.0; // Threshold for inference time recommendations
const BALANCE_RATIO_LOWER_THRESHOLD: f32 = 0.5; // Lower threshold for balance ratio
const BALANCE_RATIO_UPPER_THRESHOLD: f32 = 2.0; // Upper threshold for balance ratio
const RECOMMENDATION_CONFIDENCE_THRESHOLD: f32 = 0.6; // Threshold for confidence recommendations
const CALIBRATION_BINS: usize = 10; // Number of bins for confidence calibration analysis

/// Comprehensive ML model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Model identification
    pub model_id: String,
    pub model_version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Training metrics
    pub training_metrics: TrainingMetrics,

    /// Inference metrics
    pub inference_metrics: InferenceMetrics,

    /// Classification performance
    pub classification_metrics: ClassificationMetrics,

    /// Performance over time
    pub temporal_metrics: TemporalMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Training dataset statistics
    pub dataset_size: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub balance_ratio: f32,

    /// Training performance
    pub epochs_trained: u32,
    pub final_training_error: f32,
    pub training_duration_ms: u64,
    pub convergence_epoch: Option<u32>,

    /// Model architecture
    pub input_features: usize,
    pub hidden_layers: Vec<usize>,
    pub total_parameters: usize,
    pub learning_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceMetrics {
    /// Performance statistics
    pub total_predictions: u64,
    pub avg_inference_time_ms: f32,
    pub max_inference_time_ms: f32,
    pub min_inference_time_ms: f32,
    pub p95_inference_time_ms: f32,

    /// Throughput metrics
    pub predictions_per_second: f32,
    pub batch_processing_efficiency: f32,

    /// Resource usage
    pub memory_usage_mb: f32,
    pub cpu_utilization_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationMetrics {
    /// Confusion matrix components
    pub true_positives: u64,
    pub true_negatives: u64,
    pub false_positives: u64,
    pub false_negatives: u64,

    /// Derived metrics
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub specificity: f32,
    pub false_positive_rate: f32,

    /// Confidence distribution
    pub confidence_histogram: HashMap<String, u64>, // "0.0-0.1", "0.1-0.2", etc.
    pub avg_confidence: f32,
    pub confidence_calibration: f32,

    /// Per-severity performance
    pub severity_metrics: HashMap<Severity, SeverityMetrics>,

    /// Per-analyzer performance
    pub analyzer_metrics: HashMap<String, AnalyzerMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityMetrics {
    pub total_findings: u64,
    pub filtered_findings: u64,
    pub filter_rate: f32,
    pub avg_confidence: f32,
    pub accuracy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerMetrics {
    pub total_findings: u64,
    pub filtered_findings: u64,
    pub filter_rate: f32,
    pub avg_confidence: f32,
    pub false_positive_reduction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMetrics {
    /// Performance trends over time
    pub daily_accuracy: Vec<(String, f32)>, // Date as ISO string
    pub weekly_throughput: Vec<(String, u64)>,
    pub monthly_error_rates: Vec<(String, f32)>,

    /// Model drift detection
    pub feature_drift_scores: HashMap<String, f32>,
    pub prediction_drift_score: f32,
    pub last_drift_check: chrono::DateTime<chrono::Utc>,

    /// Performance degradation alerts
    pub performance_alerts: Vec<PerformanceAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metric_value: f32,
    pub threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    AccuracyDrop,
    LatencyIncrease,
    MemoryUsageHigh,
    ModelDrift,
    ConfidenceCalibrationPoor,
    ThroughputDrop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Checks if the model has converged based on loss history.
/// Returns `Ok(true)` if converged, `Ok(false)` if not, or an error if data is insufficient.
pub fn check_convergence(loss_history: &VecDeque<f64>) -> Result<bool> {
    if loss_history.len() < CONVERGENCE_WINDOW_SIZE {
        return Err(anyhow::anyhow!(
            "Insufficient loss history for convergence check. Need at least {} values, got {}",
            CONVERGENCE_WINDOW_SIZE,
            loss_history.len()
        ));
    }

    // Calculate average loss change over the recent window
    let recent_losses: Vec<f64> = loss_history
        .iter()
        .rev()
        .take(CONVERGENCE_WINDOW_SIZE)
        .cloned()
        .collect();

    let avg_change = recent_losses
        .windows(2)
        .map(|w| (w[1] - w[0]).abs()) // Absolute change between consecutive losses
        .sum::<f64>()
        / (CONVERGENCE_WINDOW_SIZE - 1) as f64;

    // Check if change is below threshold
    Ok(avg_change < CONVERGENCE_LOSS_THRESHOLD)
}

/// Advanced convergence detection using multiple criteria
pub fn check_advanced_convergence(
    loss_history: &VecDeque<f64>,
    validation_history: &VecDeque<f64>,
    patience: usize,
) -> Result<bool> {
    // Basic loss-based convergence
    if !check_convergence(loss_history)? {
        return Ok(false);
    }

    // Check if validation loss is also stable (if available)
    if !validation_history.is_empty()
        && validation_history.len() >= CONVERGENCE_WINDOW_SIZE
        && !check_convergence(validation_history)?
    {
        return Ok(false);
    }

    // Check for early stopping patience
    if loss_history.len() > patience {
        let recent_losses: Vec<f64> = loss_history.iter().rev().take(patience).cloned().collect();
        let min_recent_loss = recent_losses
            .iter()
            .cloned()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::INFINITY);

        let overall_min_loss = loss_history
            .iter()
            .cloned()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::INFINITY);

        // If no improvement in patience window, consider converged
        if (overall_min_loss - min_recent_loss).abs() < CONVERGENCE_LOSS_THRESHOLD {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Real-time metrics collector for ML model performance
pub struct MetricsCollector {
    metrics: ModelMetrics,
    inference_times: Vec<Duration>,
    predictions: Vec<(f32, bool)>, // (confidence, actual_label)
    start_time: Instant,

    // Sliding window for real-time metrics
    current_window: Vec<InferenceRecord>,
}

#[derive(Debug, Clone)]
struct InferenceRecord {
    timestamp: Instant,
    #[allow(dead_code)]
    inference_time: Duration,
    #[allow(dead_code)]
    confidence: f32,
    predicted_positive: bool,
    actual_positive: Option<bool>,
    #[allow(dead_code)]
    finding: Finding,
}

impl MetricsCollector {
    pub fn new(model_id: String, model_version: String) -> Self {
        Self {
            metrics: ModelMetrics {
                model_id,
                model_version,
                created_at: chrono::Utc::now(),
                training_metrics: TrainingMetrics::default(),
                inference_metrics: InferenceMetrics::default(),
                classification_metrics: ClassificationMetrics::default(),
                temporal_metrics: TemporalMetrics::default(),
            },
            inference_times: Vec::new(),
            predictions: Vec::new(),
            start_time: Instant::now(),
            current_window: Vec::new(),
        }
    }

    /// Record a single inference operation
    pub fn record_inference(
        &mut self,
        finding: &Finding,
        confidence: f32,
        inference_time: Duration,
        actual_label: Option<bool>,
    ) {
        let predicted_positive = confidence >= CONFIDENCE_CALIBRATION_TARGET;

        // Add to sliding window
        let record = InferenceRecord {
            timestamp: Instant::now(),
            inference_time,
            confidence,
            predicted_positive,
            actual_positive: actual_label,
            finding: finding.clone(),
        };

        self.current_window.push(record);

        // Maintain window size
        if self.current_window.len() > SLIDING_WINDOW_SIZE {
            self.current_window.remove(0);
        }

        // Update metrics
        self.update_inference_metrics(inference_time);
        if let Some(actual) = actual_label {
            self.update_classification_metrics(confidence, predicted_positive, actual);
        }
        self.update_severity_metrics(finding, confidence);
        self.update_analyzer_metrics(finding, confidence);
    }

    /// Record training completion
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub fn record_training(
        &mut self,
        dataset_size: usize,
        true_positives: usize,
        false_positives: usize,
        epochs: u32,
        final_error: f32,
        duration: Duration,
        architecture: &[usize],
        learning_rate: f32,
    ) {
        self.metrics.training_metrics = TrainingMetrics {
            dataset_size,
            true_positives,
            false_positives,
            balance_ratio: true_positives as f32 / false_positives.max(1) as f32,
            epochs_trained: epochs,
            final_training_error: final_error,
            training_duration_ms: duration.as_millis() as u64,
            convergence_epoch: Some(epochs), // Assume convergence at final epoch if not specified
            input_features: architecture[0],
            hidden_layers: architecture[1..architecture.len() - 1].to_vec(),
            total_parameters: self.calculate_parameters(architecture),
            learning_rate,
        };
    }

    /// Record training completion with convergence detection
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub fn record_training_with_convergence(
        &mut self,
        dataset_size: usize,
        true_positives: usize,
        false_positives: usize,
        epochs: u32,
        final_error: f32,
        duration: Duration,
        architecture: &[usize],
        learning_rate: f32,
        loss_history: &VecDeque<f64>,
    ) -> Result<()> {
        let convergence_epoch = if check_convergence(loss_history)? {
            Some(epochs.saturating_sub(CONVERGENCE_WINDOW_SIZE as u32))
        } else {
            None
        };

        self.metrics.training_metrics = TrainingMetrics {
            dataset_size,
            true_positives,
            false_positives,
            balance_ratio: true_positives as f32 / false_positives.max(1) as f32,
            epochs_trained: epochs,
            final_training_error: final_error,
            training_duration_ms: duration.as_millis() as u64,
            convergence_epoch,
            input_features: architecture[0],
            hidden_layers: architecture[1..architecture.len() - 1].to_vec(),
            total_parameters: self.calculate_parameters(architecture),
            learning_rate,
        };

        Ok(())
    }

    /// Calculate model performance metrics
    pub fn calculate_metrics(&mut self) -> &ModelMetrics {
        self.update_temporal_metrics();
        self.check_performance_alerts();
        &self.metrics
    }

    /// Export metrics to JSON file
    pub fn export_metrics(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.metrics)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Generate human-readable metrics report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🔍 ML Model Performance Report\n");
        report.push_str("================================\n\n");

        // Model info
        report.push_str("📋 Model Information\n");
        report.push_str(&format!("  ID: {}\n", self.metrics.model_id));
        report.push_str(&format!("  Version: {}\n", self.metrics.model_version));
        report.push_str(&format!(
            "  Created: {}\n\n",
            self.metrics.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Training metrics
        let tm = &self.metrics.training_metrics;
        report.push_str("🎯 Training Performance\n");
        report.push_str(&format!("  Dataset size: {} examples\n", tm.dataset_size));
        report.push_str(&format!("  Balance ratio: {:.2}\n", tm.balance_ratio));
        report.push_str(&format!("  Final error: {:.6}\n", tm.final_training_error));
        report.push_str(&format!("  Training time: {}ms\n", tm.training_duration_ms));
        report.push_str(&format!(
            "  Architecture: {} → {:?} → 1\n\n",
            tm.input_features, tm.hidden_layers
        ));

        // Inference metrics
        let im = &self.metrics.inference_metrics;
        if im.total_predictions > 0 {
            report.push_str("⚡ Inference Performance\n");
            report.push_str(&format!("  Total predictions: {}\n", im.total_predictions));
            report.push_str(&format!(
                "  Avg inference time: {:.2}ms\n",
                im.avg_inference_time_ms
            ));
            report.push_str(&format!(
                "  P95 inference time: {:.2}ms\n",
                im.p95_inference_time_ms
            ));
            report.push_str(&format!(
                "  Throughput: {:.1} predictions/sec\n\n",
                im.predictions_per_second
            ));
        }

        // Classification metrics
        let cm = &self.metrics.classification_metrics;
        if cm.true_positives + cm.false_positives + cm.true_negatives + cm.false_negatives > 0 {
            report.push_str("📊 Classification Performance\n");
            report.push_str(&format!("  Accuracy: {:.1}%\n", cm.accuracy * 100.0));
            report.push_str(&format!("  Precision: {:.1}%\n", cm.precision * 100.0));
            report.push_str(&format!("  Recall: {:.1}%\n", cm.recall * 100.0));
            report.push_str(&format!("  F1 Score: {:.1}%\n", cm.f1_score * 100.0));
            report.push_str(&format!(
                "  False Positive Rate: {:.1}%\n\n",
                cm.false_positive_rate * 100.0
            ));
        }

        // Alerts
        if !self.metrics.temporal_metrics.performance_alerts.is_empty() {
            report.push_str("🚨 Performance Alerts\n");
            for alert in &self.metrics.temporal_metrics.performance_alerts {
                let severity_emoji = match alert.severity {
                    AlertSeverity::Info => "ℹ️",
                    AlertSeverity::Warning => "⚠️",
                    AlertSeverity::Critical => "🔴",
                };
                report.push_str(&format!("  {} {}\n", severity_emoji, alert.message));
            }
            report.push('\n');
        }

        report.push_str("💡 Recommendations\n");
        report.push_str(&self.generate_recommendations());

        report
    }

    // Private helper methods
    fn update_inference_metrics(&mut self, inference_time: Duration) {
        self.inference_times.push(inference_time);

        let im = &mut self.metrics.inference_metrics;
        im.total_predictions += 1;

        let time_ms = inference_time.as_secs_f32() * 1000.0;
        im.avg_inference_time_ms = (im.avg_inference_time_ms * (im.total_predictions - 1) as f32
            + time_ms)
            / im.total_predictions as f32;
        im.max_inference_time_ms = im.max_inference_time_ms.max(time_ms);
        im.min_inference_time_ms = if im.min_inference_time_ms == 0.0 {
            time_ms
        } else {
            im.min_inference_time_ms.min(time_ms)
        };

        // Calculate P95
        if self.inference_times.len() >= MIN_SAMPLES_FOR_P95 {
            let mut sorted_times: Vec<f32> = self
                .inference_times
                .iter()
                .map(|d| d.as_secs_f32() * 1000.0)
                .collect();
            sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p95_index = (sorted_times.len() as f32 * P95_PERCENTILE) as usize;
            im.p95_inference_time_ms = sorted_times[p95_index.min(sorted_times.len() - 1)];
        }

        // Calculate throughput
        let elapsed = self.start_time.elapsed().as_secs_f32();
        if elapsed > 0.0 {
            im.predictions_per_second = im.total_predictions as f32 / elapsed;
        }
    }

    fn update_classification_metrics(&mut self, confidence: f32, predicted: bool, actual: bool) {
        let cm = &mut self.metrics.classification_metrics;

        // Store prediction for future analysis
        self.predictions.push((confidence, actual));

        match (predicted, actual) {
            (true, true) => cm.true_positives += 1,
            (true, false) => cm.false_positives += 1,
            (false, true) => cm.false_negatives += 1,
            (false, false) => cm.true_negatives += 1,
        }

        // Recalculate derived metrics
        let total = cm.true_positives + cm.true_negatives + cm.false_positives + cm.false_negatives;
        if total > 0 {
            cm.accuracy = (cm.true_positives + cm.true_negatives) as f32 / total as f32;
            cm.precision = if cm.true_positives + cm.false_positives > 0 {
                cm.true_positives as f32 / (cm.true_positives + cm.false_positives) as f32
            } else {
                0.0
            };
            cm.recall = if cm.true_positives + cm.false_negatives > 0 {
                cm.true_positives as f32 / (cm.true_positives + cm.false_negatives) as f32
            } else {
                0.0
            };
            cm.f1_score = if cm.precision + cm.recall > 0.0 {
                2.0 * cm.precision * cm.recall / (cm.precision + cm.recall)
            } else {
                0.0
            };
            cm.specificity = if cm.true_negatives + cm.false_positives > 0 {
                cm.true_negatives as f32 / (cm.true_negatives + cm.false_positives) as f32
            } else {
                0.0
            };
            cm.false_positive_rate = 1.0 - cm.specificity;
        }

        // Update confidence histogram
        let bucket = format!(
            "{:.1}-{:.1}",
            (confidence * 10.0).floor() / 10.0,
            (confidence * 10.0).floor() / 10.0 + 0.1
        );
        *cm.confidence_histogram.entry(bucket).or_insert(0) += 1;

        // Update average confidence
        let total_predictions = cm.confidence_histogram.values().sum::<u64>();
        cm.avg_confidence = (cm.avg_confidence * (total_predictions - 1) as f32 + confidence)
            / total_predictions as f32;
    }

    fn update_severity_metrics(&mut self, finding: &Finding, confidence: f32) {
        let severity_metrics = self
            .metrics
            .classification_metrics
            .severity_metrics
            .entry(finding.severity.clone())
            .or_insert(SeverityMetrics {
                total_findings: 0,
                filtered_findings: 0,
                filter_rate: 0.0,
                avg_confidence: 0.0,
                accuracy: 0.0,
            });

        severity_metrics.total_findings += 1;
        if confidence < DEFAULT_CONFIDENCE_THRESHOLD {
            severity_metrics.filtered_findings += 1;
        }
        severity_metrics.filter_rate =
            severity_metrics.filtered_findings as f32 / severity_metrics.total_findings as f32;
        severity_metrics.avg_confidence = (severity_metrics.avg_confidence
            * (severity_metrics.total_findings - 1) as f32
            + confidence)
            / severity_metrics.total_findings as f32;
    }

    fn update_analyzer_metrics(&mut self, finding: &Finding, confidence: f32) {
        let analyzer_metrics = self
            .metrics
            .classification_metrics
            .analyzer_metrics
            .entry(finding.analyzer.clone())
            .or_insert(AnalyzerMetrics {
                total_findings: 0,
                filtered_findings: 0,
                filter_rate: 0.0,
                avg_confidence: 0.0,
                false_positive_reduction: 0.0,
            });

        analyzer_metrics.total_findings += 1;
        if confidence < DEFAULT_CONFIDENCE_THRESHOLD {
            analyzer_metrics.filtered_findings += 1;
        }
        analyzer_metrics.filter_rate =
            analyzer_metrics.filtered_findings as f32 / analyzer_metrics.total_findings as f32;
        analyzer_metrics.avg_confidence = (analyzer_metrics.avg_confidence
            * (analyzer_metrics.total_findings - 1) as f32
            + confidence)
            / analyzer_metrics.total_findings as f32;
    }

    fn update_temporal_metrics(&mut self) {
        // Analyze recent inference records for temporal trends
        if self.current_window.len() > MIN_WINDOW_SIZE_TEMPORAL {
            let recent_records = &self.current_window
                [self.current_window.len().saturating_sub(MAX_RECENT_RECORDS)..];

            // Calculate recent accuracy if we have labeled data
            let labeled_records: Vec<_> = recent_records
                .iter()
                .filter(|r| r.actual_positive.is_some())
                .collect();

            if labeled_records.len() > MIN_LABELED_RECORDS {
                let correct_predictions = labeled_records
                    .iter()
                    .filter(|r| r.predicted_positive == r.actual_positive.unwrap())
                    .count();
                let recent_accuracy = correct_predictions as f32 / labeled_records.len() as f32;

                // Store daily accuracy with timestamp
                let date_str = chrono::Utc::now().format("%Y-%m-%d").to_string();
                self.metrics
                    .temporal_metrics
                    .daily_accuracy
                    .push((date_str, recent_accuracy));

                // Keep only last configured days
                if self.metrics.temporal_metrics.daily_accuracy.len() > MAX_DAILY_ACCURACY_DAYS {
                    self.metrics.temporal_metrics.daily_accuracy.remove(0);
                }
            }

            // Calculate recent throughput
            let recent_time_span = recent_records
                .last()
                .unwrap()
                .timestamp
                .duration_since(recent_records.first().unwrap().timestamp);
            if recent_time_span.as_secs() > 0 {
                let throughput = recent_records.len() as f32 / recent_time_span.as_secs_f32();
                let week_str = chrono::Utc::now().format("%Y-W%U").to_string();
                self.metrics
                    .temporal_metrics
                    .weekly_throughput
                    .push((week_str, throughput as u64));

                // Keep only last configured weeks
                if self.metrics.temporal_metrics.weekly_throughput.len()
                    > MAX_WEEKLY_THROUGHPUT_WEEKS
                {
                    self.metrics.temporal_metrics.weekly_throughput.remove(0);
                }
            }
        }
    }

    fn check_performance_alerts(&mut self) {
        let accuracy = self.metrics.classification_metrics.accuracy;
        let total_classified = self.metrics.classification_metrics.true_positives
            + self.metrics.classification_metrics.false_positives
            + self.metrics.classification_metrics.true_negatives
            + self.metrics.classification_metrics.false_negatives;
        let avg_inference_time = self.metrics.inference_metrics.avg_inference_time_ms;
        let avg_confidence = self.metrics.classification_metrics.avg_confidence;

        // Check for accuracy drops
        if accuracy < ACCURACY_ALERT_THRESHOLD && total_classified > MIN_CLASSIFIED_SAMPLES {
            self.add_alert(
                AlertType::AccuracyDrop,
                AlertSeverity::Warning,
                format!("Model accuracy dropped to {:.1}%", accuracy * 100.0),
                accuracy,
                ACCURACY_ALERT_THRESHOLD,
            );
        }

        // Check for latency increases
        if avg_inference_time > LATENCY_ALERT_THRESHOLD_MS {
            self.add_alert(
                AlertType::LatencyIncrease,
                AlertSeverity::Warning,
                format!(
                    "Average inference time increased to {:.1}ms",
                    avg_inference_time
                ),
                avg_inference_time,
                LATENCY_ALERT_THRESHOLD_MS,
            );
        }

        // Check for poor confidence calibration
        if !CONFIDENCE_CALIBRATION_RANGE.contains(&avg_confidence) {
            self.add_alert(
                AlertType::ConfidenceCalibrationPoor,
                AlertSeverity::Info,
                format!(
                    "Average confidence is {:.2}, consider recalibration",
                    avg_confidence
                ),
                avg_confidence,
                CONFIDENCE_CALIBRATION_TARGET,
            );
        }
    }

    fn add_alert(
        &mut self,
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        value: f32,
        threshold: f32,
    ) {
        let alert = PerformanceAlert {
            alert_type,
            severity,
            message,
            timestamp: chrono::Utc::now(),
            metric_value: value,
            threshold,
        };
        self.metrics.temporal_metrics.performance_alerts.push(alert);
    }

    #[allow(dead_code)]
    fn calculate_parameters(&self, architecture: &[usize]) -> usize {
        let mut params = 0;
        for i in 0..architecture.len() - 1 {
            params += architecture[i] * architecture[i + 1] + architecture[i + 1];
            // weights + biases
        }
        params
    }

    fn generate_recommendations(&self) -> String {
        let mut recommendations = String::new();
        let cm = &self.metrics.classification_metrics;
        let im = &self.metrics.inference_metrics;
        let tm = &self.metrics.training_metrics;

        if cm.accuracy < RECOMMENDATION_ACCURACY_THRESHOLD {
            recommendations.push_str(
                "  • Consider collecting more training data or rebalancing the dataset\n",
            );
        }

        if cm.false_positive_rate > RECOMMENDATION_FPR_THRESHOLD {
            recommendations
                .push_str("  • Adjust confidence threshold to reduce false positive rate\n");
        }

        if im.avg_inference_time_ms > RECOMMENDATION_INFERENCE_TIME_THRESHOLD_MS {
            recommendations
                .push_str("  • Consider model optimization or quantization for faster inference\n");
        }

        if tm.balance_ratio < BALANCE_RATIO_LOWER_THRESHOLD
            || tm.balance_ratio > BALANCE_RATIO_UPPER_THRESHOLD
        {
            recommendations
                .push_str("  • Rebalance training dataset for better model performance\n");
        }

        // Analyze prediction patterns for additional recommendations
        if !self.predictions.is_empty() {
            let avg_confidence = self.predictions.iter().map(|(conf, _)| conf).sum::<f32>()
                / self.predictions.len() as f32;
            if avg_confidence < RECOMMENDATION_CONFIDENCE_THRESHOLD {
                recommendations.push_str(
                    "  • Model shows low confidence - consider retraining with more diverse data\n",
                );
            }
        }

        if recommendations.is_empty() {
            recommendations.push_str(
                "  • Model performance looks good! Consider periodic retraining with new data\n",
            );
        }

        recommendations
    }

    /// Analyze confidence calibration using stored predictions
    #[allow(dead_code)]
    pub fn analyze_confidence_calibration(&self) -> f32 {
        if self.predictions.is_empty() {
            return 0.0;
        }

        // Simple calibration analysis: how well do confidence scores match actual accuracy
        let mut calibration_error = 0.0;
        let bins = CALIBRATION_BINS;

        for bin in 0..bins {
            let bin_start = bin as f32 / bins as f32;
            let bin_end = (bin + 1) as f32 / bins as f32;

            let bin_predictions: Vec<_> = self
                .predictions
                .iter()
                .filter(|(conf, _)| *conf >= bin_start && *conf < bin_end)
                .collect();

            if !bin_predictions.is_empty() {
                let avg_confidence = bin_predictions.iter().map(|(conf, _)| *conf).sum::<f32>()
                    / bin_predictions.len() as f32;
                let actual_accuracy = bin_predictions
                    .iter()
                    .filter(|(conf, actual)| (*conf >= 0.5) == *actual)
                    .count() as f32
                    / bin_predictions.len() as f32;
                calibration_error += (avg_confidence - actual_accuracy).abs();
            }
        }

        calibration_error / bins as f32
    }

    /// Get recent inference statistics from the sliding window
    #[allow(dead_code)]
    pub fn get_recent_inference_stats(&self) -> (f32, f32, usize) {
        if self.current_window.is_empty() {
            return (0.0, 0.0, 0);
        }

        let recent_avg_confidence = self
            .current_window
            .iter()
            .map(|record| record.confidence)
            .sum::<f32>()
            / self.current_window.len() as f32;

        let recent_avg_inference_time = self
            .current_window
            .iter()
            .map(|record| record.inference_time.as_secs_f32() * 1000.0)
            .sum::<f32>()
            / self.current_window.len() as f32;

        let recent_findings_count = self.current_window.len();

        (
            recent_avg_confidence,
            recent_avg_inference_time,
            recent_findings_count,
        )
    }

    /// Get findings by severity from recent window
    #[allow(dead_code)]
    pub fn get_findings_by_severity(&self) -> std::collections::HashMap<Severity, usize> {
        let mut severity_counts = std::collections::HashMap::new();

        for record in &self.current_window {
            *severity_counts
                .entry(record.finding.severity.clone())
                .or_insert(0) += 1;
        }

        severity_counts
    }
}

// Default implementations
impl Default for TrainingMetrics {
    fn default() -> Self {
        Self {
            dataset_size: 0,
            true_positives: 0,
            false_positives: 0,
            balance_ratio: 0.0,
            epochs_trained: 0,
            final_training_error: 0.0,
            training_duration_ms: 0,
            convergence_epoch: None,
            input_features: 0,
            hidden_layers: Vec::new(),
            total_parameters: 0,
            learning_rate: 0.0,
        }
    }
}

impl Default for InferenceMetrics {
    fn default() -> Self {
        Self {
            total_predictions: 0,
            avg_inference_time_ms: 0.0,
            max_inference_time_ms: 0.0,
            min_inference_time_ms: 0.0,
            p95_inference_time_ms: 0.0,
            predictions_per_second: 0.0,
            batch_processing_efficiency: 0.0,
            memory_usage_mb: 0.0,
            cpu_utilization_percent: 0.0,
        }
    }
}

impl Default for ClassificationMetrics {
    fn default() -> Self {
        Self {
            true_positives: 0,
            true_negatives: 0,
            false_positives: 0,
            false_negatives: 0,
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            specificity: 0.0,
            false_positive_rate: 0.0,
            confidence_histogram: HashMap::new(),
            avg_confidence: 0.0,
            confidence_calibration: 0.0,
            severity_metrics: HashMap::new(),
            analyzer_metrics: HashMap::new(),
        }
    }
}

impl Default for TemporalMetrics {
    fn default() -> Self {
        Self {
            daily_accuracy: Vec::new(),
            weekly_throughput: Vec::new(),
            monthly_error_rates: Vec::new(),
            feature_drift_scores: HashMap::new(),
            prediction_drift_score: 0.0,
            last_drift_check: chrono::Utc::now(),
            performance_alerts: Vec::new(),
        }
    }
}
