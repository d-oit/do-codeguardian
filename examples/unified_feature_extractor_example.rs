//! Example demonstrating the unified feature extractor
//!
//! This example shows how to use the unified feature extractor with different
//! extraction modes and configurations.

use anyhow::Result;
use do_codeguardian::{
    ml::unified_feature_extractor::{
        ExtractionMode, FeatureConfig, FeatureSet, UnifiedFeatureExtractor,
    },
    types::{Finding, Severity},
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 CodeGuardian Unified Feature Extractor Example");
    println!("==============================================\n");

    // Create a sample finding
    let finding = Finding::new(
        "security",
        "hardcoded_secret",
        Severity::High,
        PathBuf::from("src/main.rs"),
        42,
        "Potential hardcoded secret detected".to_string(),
    )
    .with_description("Found suspicious string that may contain sensitive data".to_string())
    .with_suggestion("Consider using environment variables or secure storage".to_string());

    println!("📋 Sample Finding:");
    println!("  Rule: {}", finding.rule);
    println!("  Severity: {:?}", finding.severity);
    println!("  File: {}", finding.file.display());
    println!("  Line: {}", finding.line);
    println!("  Message: {}", finding.message);
    println!();

    // Example 1: Basic extraction mode
    println!("1️⃣ Basic Extraction Mode");
    println!("------------------------");
    let basic_config = FeatureConfig {
        mode: ExtractionMode::Basic,
        ..Default::default()
    };

    let mut basic_extractor = UnifiedFeatureExtractor::with_config(basic_config);
    let basic_features = basic_extractor.extract_features(&finding).await?;

    println!("Features extracted: {}", basic_features.len());
    println!("Feature names: {:?}", basic_extractor.get_feature_names());
    println!("Feature values: {:.3?}", basic_features);
    println!();

    // Example 2: Enhanced extraction mode
    println!("2️⃣ Enhanced Extraction Mode");
    println!("---------------------------");
    let enhanced_config = FeatureConfig {
        mode: ExtractionMode::Enhanced,
        feature_sets: vec![FeatureSet::Base, FeatureSet::Ast],
        ..Default::default()
    };

    let mut enhanced_extractor = UnifiedFeatureExtractor::with_config(enhanced_config);
    let enhanced_features = enhanced_extractor.extract_features(&finding).await?;

    println!("Features extracted: {}", enhanced_features.len());
    println!(
        "Feature names: {:?}",
        enhanced_extractor.get_feature_names()
    );
    println!("First 8 features: {:.3?}", &enhanced_features[..8]);
    println!();

    // Example 3: Custom extraction mode
    println!("3️⃣ Custom Extraction Mode");
    println!("-------------------------");
    let custom_config = FeatureConfig {
        mode: ExtractionMode::Custom,
        feature_sets: vec![FeatureSet::Base, FeatureSet::Security],
        ..Default::default()
    };

    let mut custom_extractor = UnifiedFeatureExtractor::with_config(custom_config);
    let custom_features = custom_extractor.extract_features(&finding).await?;

    println!("Features extracted: {}", custom_features.len());
    println!("Feature names: {:?}", custom_extractor.get_feature_names());
    println!("Feature values: {:.3?}", custom_features);
    println!();

    // Example 4: Feature importance analysis
    println!("4️⃣ Feature Importance Analysis");
    println!("-------------------------------");
    let analysis = enhanced_extractor
        .analyze_feature_importance(&finding)
        .await?;
    println!("{}", analysis);
    println!();

    // Example 5: Cache statistics
    println!("5️⃣ Cache Statistics");
    println!("-------------------");
    let cache_stats = enhanced_extractor.get_cache_stats().await;
    println!("{}", cache_stats);
    println!();

    // Example 6: Runtime configuration update
    println!("6️⃣ Runtime Configuration Update");
    println!("--------------------------------");
    let new_config = FeatureConfig {
        mode: ExtractionMode::AstOnly,
        security: do_codeguardian::ml::unified_feature_extractor::SecurityConfig {
            max_file_size: 5 * 1024 * 1024, // 5MB
            ..Default::default()
        },
        ..Default::default()
    };

    enhanced_extractor.update_config(new_config).await?;
    println!("Configuration updated successfully!");
    println!("New mode: {:?}", enhanced_extractor.get_config().mode);
    println!();

    // Example 7: Performance metrics
    println!("7️⃣ Performance Metrics");
    println!("----------------------");
    let metrics = enhanced_extractor.get_metrics();
    println!("Total extractions: {}", metrics.total_extractions);
    println!("Cache hits: {}", metrics.cache_hits);
    println!("Cache misses: {}", metrics.cache_misses);
    println!("Errors: {}", metrics.errors);
    println!(
        "Average extraction time: {:.2?}",
        metrics.average_extraction_time
    );
    println!();

    println!("✅ All examples completed successfully!");
    println!("\n💡 Key Benefits of Unified Feature Extractor:");
    println!("   • Multiple extraction modes (Basic, Enhanced, AST-only, Custom)");
    println!("   • Runtime configuration changes");
    println!("   • Built-in caching and security features");
    println!("   • Backward compatibility with existing APIs");
    println!("   • Comprehensive error handling and logging");
    println!("   • Performance monitoring and metrics");

    Ok(())
}
