//! # AI Processor Module
//!
//! This module provides the core AI processing capabilities for CodeGuardian,
//! integrating with the existing AI enhancement engine to provide intelligent
//! analysis and insights.

use crate::config::Config;
use crate::output::ai::{
    create_enhancement_engine, AIEnhancementConfig, AIEnhancementEngine, EnhancedAnalysisResults,
};
use crate::types::AnalysisResults;
use anyhow::Result;
use std::sync::Arc;

/// AI Processor for handling intelligent analysis
pub struct AIProcessor {
    /// Configuration
    config: Arc<Config>,
    /// AI enhancement engine
    engine: Box<dyn AIEnhancementEngine>,
}

impl AIProcessor {
    /// Create a new AI processor
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let engine = create_enhancement_engine()?;

        Ok(Self { config, engine })
    }

    /// Check if AI processing is available
    pub fn is_available(&self) -> bool {
        self.engine.is_available()
    }

    /// Get available AI capabilities
    pub fn get_capabilities(&self) -> Vec<String> {
        self.engine.get_capabilities()
    }

    /// Process analysis results with AI enhancement
    pub async fn process_results(
        &self,
        results: &AnalysisResults,
    ) -> Result<EnhancedAnalysisResults> {
        let ai_config = AIEnhancementConfig {
            enable_semantic_enrichment: self.config.ai.enable_semantic_enrichment,
            enable_relationship_detection: self.config.ai.enable_relationship_detection,
            enable_insight_generation: self.config.ai.enable_insight_generation,
            enable_context_analysis: self.config.ai.enable_context_analysis,
            min_confidence_threshold: self.config.ai.min_confidence_threshold,
            max_processing_time: self.config.ai.max_processing_time,
            enable_historical_analysis: self.config.ai.enable_historical_analysis,
            model_config: crate::output::ai::ModelConfig {
                classification_model: None,
                relationship_model: None,
                context_model: None,
                use_pretrained: true,
                cache_directory: self.config.ai.model_cache_directory.clone(),
            },
        };

        self.engine.enhance_results(results, &ai_config)
    }

    /// Process results with custom AI configuration
    pub async fn process_results_with_config(
        &self,
        results: &AnalysisResults,
        ai_config: &AIEnhancementConfig,
    ) -> Result<EnhancedAnalysisResults> {
        self.engine.enhance_results(results, ai_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::types::{AnalysisResults, Finding, Severity};
    use std::path::PathBuf;

    fn create_test_config() -> Config {
        let mut config = Config::default();
        config.ai.enabled = true;
        config
    }

    fn create_test_results() -> AnalysisResults {
        let mut results = AnalysisResults::new("test".to_string());
        let finding = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test security issue".to_string(),
        );
        results.add_finding(finding);
        results
    }

    #[test]
    fn test_ai_processor_creation() {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config);
        assert!(processor.is_ok());
    }

    #[test]
    fn test_ai_processor_capabilities() {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config).unwrap();
        let capabilities = processor.get_capabilities();
        assert!(!capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_ai_processing() {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config).unwrap();
        let results = create_test_results();

        let enhanced = processor.process_results(&results).await;
        assert!(enhanced.is_ok());

        let enhanced_results = enhanced.unwrap();
        assert_eq!(enhanced_results.base_results.findings.len(), 1);
    }
}
