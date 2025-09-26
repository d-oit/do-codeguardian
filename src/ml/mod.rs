pub mod adaptive_learning;
pub mod advanced_feature_engineering;
pub mod advanced_feature_extractor;
#[cfg(feature = "ast")]
pub mod ast_analyzer;
pub mod cross_validation;
#[cfg(feature = "ast")]
pub mod enhanced_feature_extractor;
pub mod ensemble;
pub mod fann_classifier;
pub mod fann_cross_validation_integration;
pub mod feature_extractor;
pub mod intelligent_caching;
pub mod model_monitoring;
pub mod multi_language_ast_analyzer;
pub mod pattern_recognition;
pub mod production_validation;
pub mod training_data;
pub mod unified_feature_extractor;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

use crate::types::Finding;
use anyhow::Result;

/// Lightweight ML classifier using RUV-FANN for false positive reduction
pub struct MLClassifier {
    #[cfg(feature = "ml")]
    classifier: Option<fann_classifier::FannClassifier>,
    #[cfg(not(feature = "ml"))]
    classifier: Option<()>, // Placeholder when ML is disabled

    // Unified feature extraction (replaces conditional compilation)
    unified_extractor: unified_feature_extractor::UnifiedFeatureExtractor,

    enabled: bool,
    ast_enabled: bool,
}

impl MLClassifier {
    pub fn new(model_path: Option<&str>) -> Self {
        #[cfg(feature = "ml")]
        let (classifier, enabled) = {
            let classifier =
                model_path.and_then(|path| fann_classifier::FannClassifier::load(path).ok());
            let enabled = classifier.is_some();
            (classifier, enabled)
        };

        #[cfg(not(feature = "ml"))]
        let (classifier, enabled) = {
            if model_path.is_some() {
                tracing::warn!(
                    "ML model path provided but ML feature is disabled. Enable with --features ml"
                );
            }
            (None, false)
        };

        let ast_enabled = cfg!(feature = "ast");

        // Create unified extractor with appropriate configuration
        let extractor_config = if ast_enabled {
            unified_feature_extractor::FeatureConfig {
                mode: unified_feature_extractor::ExtractionMode::Enhanced,
                feature_sets: vec![
                    unified_feature_extractor::FeatureSet::Base,
                    unified_feature_extractor::FeatureSet::Ast,
                ],
                ..Default::default()
            }
        } else {
            unified_feature_extractor::FeatureConfig {
                mode: unified_feature_extractor::ExtractionMode::Basic,
                feature_sets: vec![unified_feature_extractor::FeatureSet::Base],
                ..Default::default()
            }
        };

        Self {
            classifier,
            unified_extractor: unified_feature_extractor::UnifiedFeatureExtractor::with_config(
                extractor_config,
            ),
            enabled,
            ast_enabled,
        }
    }

    /// Predict relevance score for a finding (0.0 = likely false positive, 1.0 = likely true positive)
    pub async fn predict_relevance(&mut self, finding: &Finding) -> Result<f32> {
        if !self.enabled {
            return Ok(0.5); // Neutral score if ML disabled
        }

        let features = self.extract_features(finding).await?;

        #[cfg(feature = "ml")]
        {
            if let Some(classifier) = &self.classifier {
                classifier.predict(&features)
            } else {
                Ok(0.5)
            }
        }

        #[cfg(not(feature = "ml"))]
        Ok(0.5)
    }

    /// Update model with user feedback (online learning)
    pub async fn record_feedback(
        &mut self,
        finding: &Finding,
        is_true_positive: bool,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let features = self.extract_features(finding).await?;
        let target = if is_true_positive { 1.0 } else { 0.0 };

        #[cfg(feature = "ml")]
        {
            if let Some(classifier) = &mut self.classifier {
                classifier.train_incremental(&features, target)?;
            }
        }

        #[cfg(not(feature = "ml"))]
        {
            let _ = (features, target); // Suppress unused variable warnings
        }

        Ok(())
    }

    /// Filter findings based on confidence threshold
    pub async fn filter_findings(
        &mut self,
        findings: Vec<Finding>,
        threshold: f32,
    ) -> Result<Vec<Finding>> {
        if !self.enabled {
            return Ok(findings);
        }

        let mut filtered = Vec::new();

        for finding in findings {
            let relevance = self.predict_relevance(&finding).await?;
            if relevance >= threshold {
                filtered.push(finding);
            }
        }

        Ok(filtered)
    }

    /// Extract features using the unified extractor
    async fn extract_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        self.unified_extractor.extract_features(finding).await
    }

    /// Get feature importance analysis for a finding
    pub async fn analyze_feature_importance(
        &mut self,
        finding: &Finding,
    ) -> Result<unified_feature_extractor::FeatureImportanceAnalysis> {
        self.unified_extractor
            .analyze_feature_importance(finding)
            .await
    }

    /// Get information about the ML classifier configuration
    pub fn get_info(&self) -> MLClassifierInfo {
        let config = self.unified_extractor.get_config();
        MLClassifierInfo {
            enabled: self.enabled,
            ast_enabled: self.ast_enabled,
            feature_count: self.unified_extractor.get_feature_names().len(),
            model_loaded: self.enabled,
            extraction_mode: config.mode,
            feature_sets: config.feature_sets.clone(),
        }
    }

    /// Clear any cached analysis data
    pub async fn clear_cache(&mut self) {
        self.unified_extractor.clear_cache().await;
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> unified_feature_extractor::CacheStats {
        self.unified_extractor.get_cache_stats().await
    }
}

/// Information about the ML classifier configuration
#[derive(Debug, Clone)]
pub struct MLClassifierInfo {
    pub enabled: bool,
    pub ast_enabled: bool,
    pub feature_count: usize,
    pub model_loaded: bool,
    pub extraction_mode: unified_feature_extractor::ExtractionMode,
    pub feature_sets: Vec<unified_feature_extractor::FeatureSet>,
}

impl std::fmt::Display for MLClassifierInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ML Classifier Configuration:")?;
        writeln!(
            f,
            "  Status: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        )?;
        writeln!(
            f,
            "  AST Analysis: {}",
            if self.ast_enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        )?;
        writeln!(f, "  Feature Count: {}", self.feature_count)?;
        writeln!(
            f,
            "  Model Loaded: {}",
            if self.model_loaded { "Yes" } else { "No" }
        )?;
        writeln!(f, "  Extraction Mode: {:?}", self.extraction_mode)?;
        writeln!(f, "  Feature Sets: {:?}", self.feature_sets)?;
        Ok(())
    }
}
