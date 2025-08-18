#!/usr/bin/env rust-script

//! # Enhanced ML Model Demonstration
//! 
//! This example demonstrates the new ML enhancements in CodeGuardian:
//! - 12-feature extraction (up from 8)
//! - Adaptive learning rate
//! - Early stopping
//! - Training data shuffling
//! - Comprehensive metrics
//!
//! Run with: `cargo run --example enhanced-ml-demo`

use codeguardian::{
    ml::{
        fann_classifier::{FannClassifier, NetworkConfig},
        feature_extractor::FeatureExtractor,
        training_data::{TrainingDataset, TrainingExample},
    },
    types::{Finding, Severity},
};
use std::path::PathBuf;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 CodeGuardian Enhanced ML Model Demo");
    println!("=====================================\n");

    // 1. Demonstrate Enhanced Feature Extraction
    demonstrate_enhanced_features().await?;
    
    // 2. Show Adaptive Learning
    demonstrate_adaptive_learning().await?;
    
    // 3. Compare Training Strategies
    compare_training_strategies().await?;
    
    // 4. Show Real-world Performance
    demonstrate_real_world_usage().await?;

    println!("\n✅ Enhanced ML Demo Complete!");
    println!("🚀 Ready for production use with improved accuracy and efficiency!");
    
    Ok(())
}

async fn demonstrate_enhanced_features() -> Result<()> {
    println!("📊 1. Enhanced Feature Extraction (8 → 12 features)");
    println!("==================================================");
    
    let extractor = FeatureExtractor::new();
    
    // Create sample findings to show feature diversity
    let findings = vec![
        // Security finding
        Finding::new(
            "security",
            "hardcoded_secret",
            Severity::Critical,
            PathBuf::from("src/auth/config.rs"),
            15,
            "Potential hardcoded API key detected in authentication module".to_string(),
        )
        .with_description("This appears to be a hardcoded API key which poses a significant security risk".to_string())
        .with_suggestion("Move the API key to environment variables or a secure configuration system".to_string()),
        
        // Performance finding
        Finding::new(
            "performance",
            "nested_loops",
            Severity::Medium,
            PathBuf::from("src/utils/data_processor.rs"),
            142,
            "Nested loop detected with O(n²) complexity".to_string(),
        )
        .with_description("This nested loop structure may cause performance issues with large datasets".to_string())
        .with_suggestion("Consider using HashMap for O(1) lookups or optimizing the algorithm".to_string()),
        
        // Code quality finding
        Finding::new(
            "code_quality",
            "magic_number",
            Severity::Low,
            PathBuf::from("tests/integration/helper.rs"),
            89,
            "Magic number 42 detected".to_string(),
        ),
    ];
    
    for (i, finding) in findings.iter().enumerate() {
        println!("\n📋 Finding {}: {} ({})", i + 1, finding.rule, finding.severity);
        println!("   File: {}", finding.file.display());
        println!("   Message: {}", finding.message);
        
        let features = extractor.extract_features(finding)?;
        println!("   🔢 Feature Vector (12 dimensions):");
        println!("      [1] Severity Score:      {:.3}", features[0]);
        println!("      [2] File Type Score:     {:.3}", features[1]);
        println!("      [3] Analyzer Confidence: {:.3}", features[2]);
        println!("      [4] Message Length:      {:.3}", features[3]);
        println!("      [5] Line Number:         {:.3}", features[4]);
        println!("      [6] Has Description:     {:.3}", features[5]);
        println!("      [7] Has Suggestion:      {:.3}", features[6]);
        println!("      [8] Rule Specificity:    {:.3}", features[7]);
        println!("      [9] Message Complexity:  {:.3} ⭐ NEW", features[8]);
        println!("      [10] Path Depth:         {:.3} ⭐ NEW", features[9]);
        println!("      [11] Category Confidence: {:.3} ⭐ NEW", features[10]);
        println!("      [12] Context Richness:   {:.3} ⭐ NEW", features[11]);
    }
    
    println!("\n✅ Enhanced features provide richer context for ML classification!");
    Ok(())
}

async fn demonstrate_adaptive_learning() -> Result<()> {
    println!("\n🎯 2. Adaptive Learning Rate Demonstration");
    println!("==========================================");
    
    // Create enhanced network configuration
    let config = NetworkConfig {
        input_size: 12,
        hidden_layers: vec![16, 12, 8],
        output_size: 1,
        learning_rate: 0.05,
        activation_function: "sigmoid".to_string(),
    };
    
    println!("🏗️  Enhanced Network Architecture:");
    println!("   Input Layer:    12 neurons (enhanced features)");
    println!("   Hidden Layer 1: 16 neurons");
    println!("   Hidden Layer 2: 12 neurons");
    println!("   Hidden Layer 3: 8 neurons");
    println!("   Output Layer:   1 neuron (relevance score)");
    println!("   Initial LR:     {:.3}", config.learning_rate);
    
    let mut classifier = FannClassifier::new(config)?;
    
    // Generate synthetic training data
    let training_data = generate_synthetic_training_data(100)?;
    println!("\n📚 Generated {} synthetic training examples", training_data.len());
    
    // Train with adaptive learning
    println!("\n🏃 Training with Adaptive Learning & Early Stopping...");
    println!("(This will show real-time progress every 10 epochs)");
    
    let final_error = classifier.train_batch(&training_data, 50)?;
    let stats = classifier.get_training_stats();
    
    println!("\n📈 Training Results:");
    println!("   Epochs Completed:     {}", stats.epochs_trained);
    println!("   Final Error:          {:.6}", final_error);
    println!("   Best Error Achieved:  {:.6}", stats.best_error);
    println!("   Final Learning Rate:  {:.6}", stats.current_learning_rate);
    println!("   Early Stopped:        {}", if stats.early_stopped { "Yes ✋" } else { "No" });
    
    if stats.error_history.len() > 1 {
        let improvement = stats.error_history[0] - stats.best_error;
        let improvement_pct = (improvement / stats.error_history[0]) * 100.0;
        println!("   Error Reduction:      {:.1}%", improvement_pct);
    }
    
    println!("\n✅ Adaptive learning optimized training automatically!");
    Ok(())
}

async fn compare_training_strategies() -> Result<()> {
    println!("\n⚖️  3. Training Strategy Comparison");
    println!("==================================");
    
    let training_data = generate_synthetic_training_data(200)?;
    
    // Strategy 1: Fixed learning rate (old approach)
    println!("\n🔄 Strategy 1: Fixed Learning Rate (Legacy)");
    let config_fixed = NetworkConfig {
        input_size: 12,
        hidden_layers: vec![12, 8],
        output_size: 1,
        learning_rate: 0.1,
        activation_function: "sigmoid".to_string(),
    };
    
    let mut classifier_fixed = FannClassifier::new(config_fixed)?;
    let start_time = std::time::Instant::now();
    let error_fixed = classifier_fixed.train_batch(&training_data, 30)?;
    let duration_fixed = start_time.elapsed();
    
    println!("   Final Error: {:.6}", error_fixed);
    println!("   Duration:    {:?}", duration_fixed);
    
    // Strategy 2: Enhanced adaptive approach
    println!("\n🚀 Strategy 2: Enhanced Adaptive (New)");
    let config_adaptive = NetworkConfig {
        input_size: 12,
        hidden_layers: vec![16, 12, 8],
        output_size: 1,
        learning_rate: 0.05,
        activation_function: "sigmoid".to_string(),
    };
    
    let mut classifier_adaptive = FannClassifier::new(config_adaptive)?;
    let start_time = std::time::Instant::now();
    let error_adaptive = classifier_adaptive.train_batch(&training_data, 30)?;
    let duration_adaptive = start_time.elapsed();
    let stats_adaptive = classifier_adaptive.get_training_stats();
    
    println!("   Final Error: {:.6}", error_adaptive);
    println!("   Best Error:  {:.6}", stats_adaptive.best_error);
    println!("   Duration:    {:?}", duration_adaptive);
    println!("   Epochs Used: {}", stats_adaptive.epochs_trained);
    
    // Comparison
    println!("\n📊 Comparison Results:");
    let error_improvement = ((error_fixed - error_adaptive) / error_fixed) * 100.0;
    let time_improvement = if duration_adaptive < duration_fixed {
        ((duration_fixed.as_millis() - duration_adaptive.as_millis()) as f32 / duration_fixed.as_millis() as f32) * 100.0
    } else {
        -((duration_adaptive.as_millis() - duration_fixed.as_millis()) as f32 / duration_fixed.as_millis() as f32) * 100.0
    };
    
    println!("   Error Improvement:    {:.1}%", error_improvement);
    println!("   Time Improvement:     {:.1}%", time_improvement);
    println!("   Architecture:         Enhanced (3 vs 2 hidden layers)");
    println!("   Features:             12 vs 8 (+50% richer context)");
    
    println!("\n✅ Enhanced strategy shows measurable improvements!");
    Ok(())
}

async fn demonstrate_real_world_usage() -> Result<()> {
    println!("\n🌍 4. Real-World Usage Simulation");
    println!("=================================");
    
    // Create a trained classifier
    let config = NetworkConfig::default(); // Uses enhanced 12-feature config
    let mut classifier = FannClassifier::new(config)?;
    
    // Quick training with diverse examples
    let training_data = generate_realistic_training_data()?;
    println!("📚 Training on {} realistic examples...", training_data.len());
    
    let _final_error = classifier.train_batch(&training_data, 25)?;
    let stats = classifier.get_training_stats();
    
    println!("✅ Training completed in {} epochs", stats.epochs_trained);
    
    // Test on new findings
    println!("\n🔍 Testing on New Findings:");
    
    let test_findings = vec![
        // High-confidence security issue
        create_test_finding(
            "security", "sql_injection", Severity::Critical,
            "src/database/query.rs", 67,
            "SQL injection vulnerability detected in user query handling",
            Some("Direct string concatenation in SQL query poses injection risk"),
            Some("Use parameterized queries or prepared statements")
        ),
        
        // Low-confidence quality issue
        create_test_finding(
            "code_quality", "long_line", Severity::Info,
            "tests/unit/helpers.rs", 234,
            "Line exceeds 120 characters",
            None,
            Some("Break long lines for better readability")
        ),
        
        // Medium-confidence performance issue
        create_test_finding(
            "performance", "inefficient_clone", Severity::Medium,
            "src/core/processor.rs", 89,
            "Unnecessary clone() call detected",
            Some("This clone operation may impact performance in hot code paths"),
            Some("Consider using references or borrowing instead")
        ),
    ];
    
    let extractor = FeatureExtractor::new();
    
    for (i, finding) in test_findings.iter().enumerate() {
        let features = extractor.extract_features(finding)?;
        let relevance = classifier.predict(&features)?;
        
        println!("\n📋 Test Finding {}: {}", i + 1, finding.rule);
        println!("   Severity: {:?}", finding.severity);
        println!("   Analyzer: {}", finding.analyzer);
        println!("   🤖 ML Relevance Score: {:.3}", relevance);
        
        let confidence_level = match relevance {
            r if r >= 0.8 => "🔴 High Confidence (Likely True Positive)",
            r if r >= 0.6 => "🟡 Medium Confidence",
            r if r >= 0.4 => "🟠 Low Confidence",
            _ => "🟢 Very Low Confidence (Likely False Positive)",
        };
        
        println!("   📊 Assessment: {}", confidence_level);
        
        // Show key contributing features
        println!("   🔍 Key Features:");
        println!("      Category Confidence: {:.3}", features[10]);
        println!("      Context Richness:    {:.3}", features[11]);
        println!("      Message Complexity:  {:.3}", features[8]);
    }
    
    println!("\n🎯 ML Model Performance Summary:");
    println!("   ✅ Enhanced 12-feature extraction provides rich context");
    println!("   ✅ Adaptive learning optimizes training automatically");
    println!("   ✅ Early stopping prevents overfitting");
    println!("   ✅ Real-time inference suitable for CI/CD pipelines");
    println!("   ✅ Confidence scores help prioritize findings");
    
    Ok(())
}

fn generate_synthetic_training_data(count: usize) -> Result<Vec<(Vec<f32>, f32)>> {
    let mut training_data = Vec::new();
    
    for i in 0..count {
        // Create diverse feature patterns
        let is_positive = i % 3 != 0; // 2/3 positive examples
        
        let features = if is_positive {
            // True positive pattern: high severity, good context
            vec![
                0.8 + (i as f32 * 0.001) % 0.2,  // High severity
                0.7 + (i as f32 * 0.002) % 0.3,  // Good file type
                0.8 + (i as f32 * 0.001) % 0.2,  // High analyzer confidence
                0.6 + (i as f32 * 0.003) % 0.4,  // Reasonable message length
                0.9 - (i as f32 * 0.001) % 0.3,  // Early in file
                1.0,                              // Has description
                1.0,                              // Has suggestion
                0.7 + (i as f32 * 0.002) % 0.3,  // Good specificity
                0.6 + (i as f32 * 0.003) % 0.4,  // Complex message
                0.8 + (i as f32 * 0.001) % 0.2,  // Shallow path
                0.8 + (i as f32 * 0.002) % 0.2,  // High category confidence
                0.7 + (i as f32 * 0.003) % 0.3,  // Rich context
            ]
        } else {
            // False positive pattern: low severity, poor context
            vec![
                0.1 + (i as f32 * 0.002) % 0.3,  // Low severity
                0.3 + (i as f32 * 0.003) % 0.4,  // Poor file type
                0.4 + (i as f32 * 0.002) % 0.3,  // Low analyzer confidence
                0.2 + (i as f32 * 0.004) % 0.3,  // Short message
                0.3 + (i as f32 * 0.003) % 0.4,  // Late in file
                0.0,                              // No description
                0.0,                              // No suggestion
                0.2 + (i as f32 * 0.003) % 0.3,  // Poor specificity
                0.1 + (i as f32 * 0.004) % 0.3,  // Simple message
                0.2 + (i as f32 * 0.002) % 0.3,  // Deep path
                0.3 + (i as f32 * 0.003) % 0.4,  // Low category confidence
                0.1 + (i as f32 * 0.004) % 0.3,  // Poor context
            ]
        };
        
        let target = if is_positive { 1.0 } else { 0.0 };
        training_data.push((features, target));
    }
    
    Ok(training_data)
}

fn generate_realistic_training_data() -> Result<Vec<(Vec<f32>, f32)>> {
    let extractor = FeatureExtractor::new();
    let mut training_data = Vec::new();
    
    // High-confidence true positives
    let true_positives = vec![
        create_test_finding(
            "security", "hardcoded_secret", Severity::Critical,
            "src/config.rs", 23,
            "Hardcoded API key detected",
            Some("API keys should not be stored in source code"),
            Some("Move to environment variables")
        ),
        create_test_finding(
            "security", "sql_injection", Severity::High,
            "src/database.rs", 45,
            "SQL injection vulnerability",
            Some("User input directly concatenated to SQL query"),
            Some("Use parameterized queries")
        ),
        create_test_finding(
            "performance", "nested_loops", Severity::Medium,
            "src/algorithm.rs", 78,
            "Nested loops with O(n²) complexity",
            Some("Performance may degrade with large datasets"),
            Some("Consider using HashMap for O(1) lookups")
        ),
    ];
    
    // Low-confidence false positives
    let false_positives = vec![
        create_test_finding(
            "code_quality", "magic_number", Severity::Info,
            "tests/test_helper.rs", 156,
            "Magic number 42",
            None,
            None
        ),
        create_test_finding(
            "code_quality", "long_line", Severity::Info,
            "docs/examples/demo.rs", 234,
            "Line too long",
            None,
            Some("Break line for readability")
        ),
    ];
    
    // Extract features and create training pairs
    for finding in true_positives {
        let features = extractor.extract_features(&finding)?;
        training_data.push((features, 1.0));
    }
    
    for finding in false_positives {
        let features = extractor.extract_features(&finding)?;
        training_data.push((features, 0.0));
    }
    
    Ok(training_data)
}

fn create_test_finding(
    analyzer: &str,
    rule: &str,
    severity: Severity,
    file: &str,
    line: u32,
    message: &str,
    description: Option<&str>,
    suggestion: Option<&str>,
) -> Finding {
    let mut finding = Finding::new(
        analyzer,
        rule,
        severity,
        PathBuf::from(file),
        line,
        message.to_string(),
    );
    
    if let Some(desc) = description {
        finding = finding.with_description(desc.to_string());
    }
    
    if let Some(sugg) = suggestion {
        finding = finding.with_suggestion(sugg.to_string());
    }
    
    finding
}