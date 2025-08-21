pub mod fann_classifier;
pub mod feature_extractor;
pub mod training_data;
pub mod metrics;

use crate::types::Finding;
use anyhow::Result;
use std::time::Instant;
pub use metrics::{MetricsCollector, ModelMetrics};

/// Lightweight ML classifier using RUV-FANN for false positive reduction
pub struct MLClassifier {
    classifier: Option<fann_classifier::FannClassifier>,
    feature_extractor: feature_extractor::FeatureExtractor,
    enabled: bool,
    metrics_collector: Option<MetricsCollector>,
}

impl MLClassifier {
    pub fn new(model_path: Option<&str>) -> Self {
        let classifier = model_path
            .and_then(|path| {
                // Only try to load if the file exists
                if std::path::Path::new(path).exists() {
                    fann_classifier::FannClassifier::load(path).ok()
                } else {
                    None
                }
            });
        
        let enabled = classifier.is_some();
        
        let metrics_collector = if enabled {
            Some(MetricsCollector::new(
                "codeguardian-model".to_string(),
                env!("CARGO_PKG_VERSION").to_string(),
            ))
        } else {
            None
        };

        Self {
            classifier,
            feature_extractor: feature_extractor::FeatureExtractor::new(),
            enabled,
            metrics_collector,
        }
    }

    /// Predict relevance score for a finding (0.0 = likely false positive, 1.0 = likely true positive)
    pub fn predict_relevance(&mut self, finding: &Finding) -> Result<f32> {
        if !self.enabled {
            return Ok(0.5); // Neutral score if ML disabled
        }

        let start_time = Instant::now();
        let features = self.feature_extractor.extract_features(finding)?;
        
        let confidence = if let Some(classifier) = &self.classifier {
            classifier.predict(&features)?
        } else {
            0.5
        };

        let inference_time = start_time.elapsed();

        // Record metrics if collector is available
        if let Some(metrics_collector) = &mut self.metrics_collector {
            // Note: We don't have actual labels here, so we pass None
            // In a real system, you'd collect this from user feedback
            metrics_collector.record_inference(finding, confidence, inference_time, None);
        }

        Ok(confidence)
    }

    /// Update model with user feedback (online learning)
    #[allow(dead_code)]
    pub fn record_feedback(&mut self, finding: &Finding, is_true_positive: bool) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let start_time = Instant::now();
        let features = self.feature_extractor.extract_features(finding)?;
        let target = if is_true_positive { 1.0 } else { 0.0 };

        if let Some(classifier) = &mut self.classifier {
            classifier.train_incremental(&features, target)?;
        }

        let inference_time = start_time.elapsed();

        // Record feedback in metrics
        if let Some(metrics_collector) = &mut self.metrics_collector {
            let confidence = if let Some(classifier) = &self.classifier {
                classifier.predict(&features).unwrap_or(0.5)
            } else {
                0.5
            };
            metrics_collector.record_inference(finding, confidence, inference_time, Some(is_true_positive));
        }

        Ok(())
    }

    /// Filter findings based on confidence threshold
    pub fn filter_findings(&mut self, findings: Vec<Finding>, threshold: f32) -> Result<Vec<Finding>> {
        if !self.enabled {
            return Ok(findings);
        }

        let mut filtered = Vec::new();
        
        for finding in findings {
            let relevance = self.predict_relevance(&finding)?;
            if relevance >= threshold {
                filtered.push(finding);
            }
        }

        Ok(filtered)
    }

    /// Get current model performance metrics
    pub fn get_metrics(&mut self) -> Option<&ModelMetrics> {
        self.metrics_collector.as_mut().map(|collector| collector.calculate_metrics())
    }

    /// Export metrics to JSON file
    pub fn export_metrics(&self, path: &str) -> Result<()> {
        if let Some(metrics_collector) = &self.metrics_collector {
            metrics_collector.export_metrics(path)?;
        }
        Ok(())
    }

    /// Generate human-readable metrics report
    pub fn generate_metrics_report(&self) -> String {
        if let Some(metrics_collector) = &self.metrics_collector {
            metrics_collector.generate_report()
        } else {
            "ML classifier not enabled - no metrics available".to_string()
        }
    }

    /// Record training completion for metrics
    #[allow(dead_code)]
    pub fn record_training_completion(
        &mut self,
        dataset_size: usize,
        true_positives: usize,
        false_positives: usize,
        epochs: u32,
        final_error: f32,
        duration: std::time::Duration,
        architecture: &[usize],
        learning_rate: f32,
    ) {
        if let Some(metrics_collector) = &mut self.metrics_collector {
            metrics_collector.record_training(
                dataset_size,
                true_positives,
                false_positives,
                epochs,
                final_error,
                duration,
                architecture,
                learning_rate,
            );
        }
    }

    /// Check if ML classifier is enabled and collecting metrics
    pub fn is_metrics_enabled(&self) -> bool {
        self.metrics_collector.is_some()
    }
}