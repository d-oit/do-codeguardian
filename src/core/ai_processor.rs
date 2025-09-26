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
        tracing::debug!(
            "Starting AI processing for {} findings",
            results.findings.len()
        );

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

        tracing::debug!(
            "AI config: semantic={}, relationships={}, insights={}, context={}",
            ai_config.enable_semantic_enrichment,
            ai_config.enable_relationship_detection,
            ai_config.enable_insight_generation,
            ai_config.enable_context_analysis
        );

        let enhanced = self.engine.enhance_results(results, &ai_config)?;

        tracing::debug!(
            "AI processing complete: {} classifications, {} relationships, {} insights",
            enhanced.semantic_annotations.classifications.len(),
            enhanced.relationships.len(),
            enhanced.insights.len()
        );

        Ok(enhanced)
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
        let finding1 = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test security issue".to_string(),
        );
        let finding2 = Finding::new(
            "test_analyzer",
            "test_rule2",
            Severity::Medium,
            PathBuf::from("test.rs"),
            20,
            "Another issue".to_string(),
        );
        results.add_finding(finding1);
        results.add_finding(finding2);
        results
    }

    #[test]
    fn test_ai_processor_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config);
        assert!(processor.is_ok());

        Ok(())
    }

    #[test]
    fn test_ai_processor_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config)?;
        let capabilities = processor.get_capabilities();
        assert!(!capabilities.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_ai_processing() -> Result<(), Box<dyn std::error::Error>> {
        let config = Arc::new(create_test_config());
        let processor = AIProcessor::new(config).unwrap();
        let results = create_test_results();

        let enhanced = processor.process_results(&results).await;
        assert!(enhanced.is_ok());

        let enhanced_results = enhanced.unwrap();
        assert_eq!(enhanced_results.base_results.findings.len(), 2);

        // Debug: Print the enhanced results to see what's populated
        println!(
            "Classifications: {}",
            enhanced_results.semantic_annotations.classifications.len()
        );
        println!("Relationships: {}", enhanced_results.relationships.len());
        println!("Insights: {}", enhanced_results.insights.len());

        // Check that enrichment is working
        assert!(
            !enhanced_results
                .semantic_annotations
                .classifications
                .is_empty(),
            "Classifications should not be empty"
        );
        // With 2 findings from same analyzer, should have relationships
        assert!(
            !enhanced_results.relationships.is_empty(),
            "Relationships should not be empty for multiple findings"
        );

        Ok(())
    }
}
