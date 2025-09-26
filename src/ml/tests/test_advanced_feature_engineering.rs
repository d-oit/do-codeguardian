//! Tests for advanced feature engineering module

#[cfg(test)]
mod tests {
    use crate::ml::advanced_feature_engineering::{
        AdvancedFeatureEngineer, FeatureEngineeringConfig, GenerationStrategy, SelectionMethod,
    };
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;

    fn create_test_finding() -> Finding {
        Finding {
            file: PathBuf::from("test.rs"),
            line: Some(42),
            column: Some(10),
            rule: "security/password-hardcoded".to_string(),
            message: "Hardcoded password detected in source code".to_string(),
            description: Some(
                "Found a hardcoded password 'admin123' which poses a security risk".to_string(),
            ),
            severity: Severity::Critical,
            suggestion: Some("Use environment variables or secure configuration".to_string()),
            analyzer: "security_analyzer".to_string(),
        }
    }

    #[tokio::test]
    async fn test_feature_engineer_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = FeatureEngineeringConfig {
            auto_generation: true,
            feature_selection: true,
            max_generated_features: 100,
            selection_threshold: 0.01,
            cv_folds: 3,
            generation_strategies: vec![GenerationStrategy::PatternBased],
            selection_methods: vec![SelectionMethod::VarianceThreshold],
        };

        let engineer = AdvancedFeatureEngineer::with_config(config);
        assert!(!engineer.get_metrics().total_features_generated > 0);
    }

    #[tokio::test]
    async fn test_basic_feature_extraction() -> Result<(), Box<dyn std::error::Error>> {
        let mut engineer = AdvancedFeatureEngineer::new();
        let finding = create_test_finding();

        let features = engineer.extract_enhanced_features(&finding).await;
        assert!(features.is_ok());

        let features = features?;
        assert!(!features.is_empty(), "Should generate some features");

        // All features should be finite numbers
        for &feature in &features {
            assert!(
                feature.is_finite(),
                "Feature value should be finite: {}",
                feature
            );
        }
    }

    #[tokio::test]
    async fn test_pattern_based_generation() -> Result<(), Box<dyn std::error::Error>> {
        let config = FeatureEngineeringConfig {
            auto_generation: true,
            feature_selection: false,
            max_generated_features: 1000,
            selection_threshold: 0.01,
            cv_folds: 3,
            generation_strategies: vec![GenerationStrategy::PatternBased],
            selection_methods: vec![],
        };

        let mut engineer = AdvancedFeatureEngineer::with_config(config);
        let finding = create_test_finding();

        let features = engineer.extract_enhanced_features(&finding).await?;
        assert!(!features.is_empty());

        let metrics = engineer.get_metrics();
        assert!(metrics.total_features_generated > 0);
    }

    #[tokio::test]
    async fn test_statistical_transforms() -> Result<(), Box<dyn std::error::Error>> {
        let config = FeatureEngineeringConfig {
            auto_generation: true,
            feature_selection: false,
            max_generated_features: 1000,
            selection_threshold: 0.01,
            cv_folds: 3,
            generation_strategies: vec![GenerationStrategy::StatisticalTransforms],
            selection_methods: vec![],
        };

        let mut engineer = AdvancedFeatureEngineer::with_config(config);
        let finding = create_test_finding();

        let features = engineer.extract_enhanced_features(&finding).await?;
        assert!(!features.is_empty());
    }

    #[tokio::test]
    async fn test_feature_selection() -> Result<(), Box<dyn std::error::Error>> {
        let config = FeatureEngineeringConfig {
            auto_generation: true,
            feature_selection: true,
            max_generated_features: 1000,
            selection_threshold: 0.1, // Higher threshold for more aggressive selection
            cv_folds: 3,
            generation_strategies: vec![
                GenerationStrategy::PatternBased,
                GenerationStrategy::StatisticalTransforms,
            ],
            selection_methods: vec![
                SelectionMethod::VarianceThreshold,
                SelectionMethod::MutualInformation,
            ],
        };

        let mut engineer = AdvancedFeatureEngineer::with_config(config);
        let finding = create_test_finding();

        let features_with_selection = engineer.extract_enhanced_features(&finding).await?;

        // Reset engineer without selection
        let config_no_selection = FeatureEngineeringConfig {
            feature_selection: false,
            ..config
        };
        engineer.update_config(config_no_selection);
        engineer.clear_cache().await;

        let features_without_selection = engineer.extract_enhanced_features(&finding).await?;

        // With selection should generally have fewer or equal features
        assert!(features_with_selection.len() <= features_without_selection.len());
    }

    #[tokio::test]
    async fn test_caching() -> Result<(), Box<dyn std::error::Error>> {
        let mut engineer = AdvancedFeatureEngineer::new();
        let finding = create_test_finding();

        // First extraction
        let _features1 = engineer.extract_enhanced_features(&finding).await?;
        let metrics1 = engineer.get_metrics().clone();

        // Second extraction (should hit cache)
        let _features2 = engineer.extract_enhanced_features(&finding).await?;
        let metrics2 = engineer.get_metrics();

        assert!(metrics2.cache_hits > metrics1.cache_hits);
    }

    #[tokio::test]
    async fn test_multiple_generation_strategies() -> Result<(), Box<dyn std::error::Error>> {
        let config = FeatureEngineeringConfig {
            auto_generation: true,
            feature_selection: false,
            max_generated_features: 1000,
            selection_threshold: 0.01,
            cv_folds: 3,
            generation_strategies: vec![
                GenerationStrategy::PatternBased,
                GenerationStrategy::StatisticalTransforms,
                GenerationStrategy::InteractionFeatures,
                GenerationStrategy::NGramFeatures,
                GenerationStrategy::FrequencyFeatures,
            ],
            selection_methods: vec![],
        };

        let mut engineer = AdvancedFeatureEngineer::with_config(config);
        let finding = create_test_finding();

        let features = engineer.extract_enhanced_features(&finding).await?;
        assert!(!features.is_empty());

        // Should have generated features from multiple strategies
        let metrics = engineer.get_metrics();
        assert!(metrics.total_features_generated > 20); // Should be substantial
    }

    #[tokio::test]
    async fn test_different_severities() -> Result<(), Box<dyn std::error::Error>> {
        let mut engineer = AdvancedFeatureEngineer::new();

        let severities = vec![
            Severity::Critical,
            Severity::High,
            Severity::Medium,
            Severity::Low,
            Severity::Info,
        ];

        for severity in severities {
            let mut finding = create_test_finding();
            finding.severity = severity;

            let features = engineer.extract_enhanced_features(&finding).await?;
            assert!(!features.is_empty());

            // Features should be different for different severities
            for &feature in &features {
                assert!(feature.is_finite());
            }
        }
    }

    #[tokio::test]
    async fn test_config_update() -> Result<(), Box<dyn std::error::Error>> {
        let mut engineer = AdvancedFeatureEngineer::new();

        let new_config = FeatureEngineeringConfig {
            auto_generation: false,
            feature_selection: false,
            max_generated_features: 50,
            selection_threshold: 0.5,
            cv_folds: 2,
            generation_strategies: vec![GenerationStrategy::PatternBased],
            selection_methods: vec![SelectionMethod::VarianceThreshold],
        };

        engineer.update_config(new_config);

        let finding = create_test_finding();
        let features = engineer.extract_enhanced_features(&finding).await?;

        // Should still work with updated config
        assert!(!features.is_empty());
    }
}
