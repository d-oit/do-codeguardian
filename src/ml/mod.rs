#[cfg(feature = "ast")]
pub mod ast_analyzer;
#[cfg(feature = "ast")]
pub mod enhanced_feature_extractor;
pub mod fann_classifier;
pub mod feature_extractor;
pub mod training_data;

use crate::types::Finding;
use anyhow::Result;

/// Lightweight ML classifier using RUV-FANN for false positive reduction
pub struct MLClassifier {
    #[cfg(feature = "ml")]
    classifier: Option<fann_classifier::FannClassifier>,
    #[cfg(not(feature = "ml"))]
    classifier: Option<()>, // Placeholder when ML is disabled

    // Feature extraction strategy
    #[cfg(all(feature = "ml", feature = "ast"))]
    enhanced_extractor: enhanced_feature_extractor::EnhancedFeatureExtractor,
    #[cfg(all(feature = "ml", not(feature = "ast")))]
    base_extractor: feature_extractor::FeatureExtractor,
    #[cfg(not(feature = "ml"))]
    _extractor_placeholder: (),

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

        Self {
            classifier,

            #[cfg(all(feature = "ml", feature = "ast"))]
            enhanced_extractor: enhanced_feature_extractor::EnhancedFeatureExtractor::new(),
            #[cfg(all(feature = "ml", not(feature = "ast")))]
            base_extractor: feature_extractor::FeatureExtractor::new(),
            #[cfg(not(feature = "ml"))]
            _extractor_placeholder: (),

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

    /// Extract features using the appropriate extractor
    async fn extract_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        #[cfg(all(feature = "ml", feature = "ast"))]
        {
            self.enhanced_extractor
                .extract_enhanced_features(finding)
                .await
        }

        #[cfg(all(feature = "ml", not(feature = "ast")))]
        {
            self.base_extractor.extract_features(finding)
        }

        #[cfg(not(feature = "ml"))]
        {
            let _ = finding;
            Ok(vec![0.5; 8]) // Neutral features when ML is disabled
        }
    }

    /// Get feature importance analysis for a finding (AST-enhanced only)
    #[cfg(all(feature = "ml", feature = "ast"))]
    pub async fn analyze_feature_importance(
        &mut self,
        finding: &Finding,
    ) -> Result<enhanced_feature_extractor::FeatureImportanceAnalysis> {
        self.enhanced_extractor
            .analyze_feature_importance(finding)
            .await
    }

    /// Get information about the ML classifier configuration
    pub fn get_info(&self) -> MLClassifierInfo {
        MLClassifierInfo {
            enabled: self.enabled,
            ast_enabled: self.ast_enabled,
            feature_count: if self.ast_enabled { 24 } else { 8 },
            model_loaded: self.enabled,
        }
    }

    /// Clear any cached analysis data
    #[cfg(all(feature = "ml", feature = "ast"))]
    pub fn clear_cache(&mut self) {
        self.enhanced_extractor.clear_cache();
    }

    /// Get cache statistics (AST-enhanced only)
    #[cfg(all(feature = "ml", feature = "ast"))]
    pub fn get_cache_stats(&self) -> enhanced_feature_extractor::CacheStats {
        self.enhanced_extractor.get_cache_stats()
    }
}

/// Information about the ML classifier configuration
#[derive(Debug, Clone)]
pub struct MLClassifierInfo {
    pub enabled: bool,
    pub ast_enabled: bool,
    pub feature_count: usize,
    pub model_loaded: bool,
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
        if self.ast_enabled {
            writeln!(f, "  Feature Types: Base (8) + AST (16) = Enhanced (24)")?;
        } else {
            writeln!(f, "  Feature Types: Base only (8)")?;
        }
        Ok(())
    }
}
