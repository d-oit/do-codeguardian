#!/usr/bin/env rust-script

//! AST-Enhanced Feature Engineering Demonstration
//!
//! This script shows the power of AST-based analysis for ML feature extraction

use std::path::PathBuf;

#[cfg(all(feature = "ml", feature = "ast"))]
fn demonstrate_ast_features() -> anyhow::Result<()> {
    use do_codeguardian::ml::{
        ast_analyzer::AstAnalyzer, enhanced_feature_extractor::EnhancedFeatureExtractor,
        feature_extractor::FeatureExtractor,
    };
    use do_codeguardian::types::{Finding, Severity};

    println!("🚀 AST-Enhanced Feature Engineering Demo");
    println!("=========================================");

    // Create test Rust code with various complexity patterns
    let test_code = r#"
use std::collections::HashMap;

/// A complex function with multiple decision points
pub fn complex_function(data: &[i32]) -> Result<HashMap<i32, String>, String> {
    let mut result = HashMap::new();
    
    if data.is_empty() {
        return Err("Empty data".to_string());
    }
    
    for (index, &value) in data.iter().enumerate() {
        match value {
            0..=10 => {
                if value % 2 == 0 {
                    result.insert(value, format!("even_{}", index));
                } else {
                    result.insert(value, format!("odd_{}", index));
                }
            }
            11..=100 => {
                let processed = unsafe {
                    // Unsafe block for demonstration
                    std::ptr::read(&value as *const i32)
                };
                result.insert(processed, "medium".to_string());
            }
            _ => {
                // This might panic!
                let risky = data[index + 1].to_string();
                result.insert(value, risky);
            }
        }
    }
    
    Ok(result)
}

#[test]
fn test_complex_function() {
    let data = vec![1, 2, 3];
    let result = complex_function(&data).unwrap();
    assert!(!result.is_empty());
}

struct DataProcessor {
    cache: HashMap<String, i32>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    pub fn process(&mut self, key: &str) -> i32 {
        *self.cache.entry(key.to_string()).or_insert_with(|| {
            // Some complex computation
            key.len() as i32 * 42
        })
    }
}

enum ProcessingMode {
    Fast,
    Thorough,
    Debug,
}
"#;

    // Save test code to a temporary file
    std::fs::write("tmp_test_code.rs", test_code)?;

    println!("\n📊 Analyzing test Rust code...");
    println!("Code complexity: {} lines", test_code.lines().count());

    // 1. Traditional feature extraction
    println!("\n🔍 Traditional Feature Extraction:");
    let base_extractor = FeatureExtractor::new();
    let finding = Finding::new(
        "security",
        "unsafe_code",
        Severity::High,
        PathBuf::from("tmp_test_code.rs"),
        25,
        "Unsafe block detected in complex function".to_string(),
    );

    let base_features = base_extractor.extract_features(&finding)?;
    println!(
        "  Base features (8): {:?}",
        base_features
            .iter()
            .map(|f| format!("{:.3}", f))
            .collect::<Vec<_>>()
    );

    // 2. AST-enhanced feature extraction
    println!("\n🧠 AST-Enhanced Feature Extraction:");
    let mut enhanced_extractor = EnhancedFeatureExtractor::new();
    let enhanced_features = enhanced_extractor.extract_enhanced_features(&finding)?;
    println!(
        "  Enhanced features (24): {:?}",
        enhanced_features
            .iter()
            .map(|f| format!("{:.3}", f))
            .collect::<Vec<_>>()
    );

    // 3. Feature importance analysis
    println!("\n📈 Feature Importance Analysis:");
    let importance = enhanced_extractor.analyze_feature_importance(&finding)?;
    println!("{}", importance);

    // 4. AST-specific analysis
    println!("\n🌳 Direct AST Analysis:");
    let ast_analyzer = AstAnalyzer::new();
    let ast_features =
        ast_analyzer.extract_ast_features(&PathBuf::from("tmp_test_code.rs"), test_code)?;

    println!(
        "  Cyclomatic complexity: {:.1}",
        ast_features.cyclomatic_complexity
    );
    println!("  Function count: {:.0}", ast_features.function_count);
    println!("  Struct count: {:.0}", ast_features.struct_count);
    println!("  Enum count: {:.0}", ast_features.enum_count);
    println!("  Unsafe blocks: {:.0}", ast_features.unsafe_block_count);
    println!("  Panic calls: {:.0}", ast_features.panic_call_count);
    println!(
        "  String literals: {:.0}",
        ast_features.string_literal_count
    );
    println!("  Max nesting depth: {:.0}", ast_features.nesting_depth);

    // 5. Feature comparison
    println!("\n⚖️  Feature Enhancement Comparison:");
    let feature_names = EnhancedFeatureExtractor::get_feature_names();
    println!("  Total features: {}", feature_names.len());
    println!("  Base features: 8 (traditional text-based analysis)");
    println!("  AST features: 16 (code structure analysis)");
    println!(
        "  Enhancement ratio: {:.1}x more information",
        enhanced_features.len() as f32 / base_features.len() as f32
    );

    // 6. Cache statistics
    println!("\n💾 Cache Performance:");
    let cache_stats = enhanced_extractor.get_cache_stats();
    println!("  {}", cache_stats);

    // Clean up
    std::fs::remove_file("tmp_test_code.rs").ok();

    println!("\n✨ AST Enhancement Benefits:");
    println!("  🎯 Deeper Code Understanding: Analyzes actual code structure, not just text");
    println!("  🔍 Security Pattern Detection: Identifies unsafe blocks, panic calls, unwraps");
    println!("  📊 Complexity Metrics: Cyclomatic complexity, nesting depth, function counts");
    println!("  🚀 Performance: Cached analysis for repeated file access");
    println!("  🎛️  Granular Control: 24 features vs 8 for much finer classification");

    Ok(())
}

#[cfg(not(all(feature = "ml", feature = "ast")))]
fn demonstrate_ast_features() -> anyhow::Result<()> {
    println!("❌ AST-enhanced features require both 'ml' and 'ast' features.");
    println!("   Build with: cargo run --features ml,ast --bin tmp_rovodev_ast_demo");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    demonstrate_ast_features()
}
