//! CLI command for advanced feature engineering operations

#[cfg(feature = "ml")]
use crate::ml::advanced_feature_engineering::{
    AdvancedFeatureEngineer, FeatureEngineeringConfig, GenerationStrategy, SelectionMethod,
};
use crate::types::{Finding, Severity};
use anyhow::Result;
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn};

/// Advanced feature engineering operations
#[derive(Args, Debug)]
pub struct FeatureEngineeringArgs {
    /// Input file or directory to analyze
    #[arg(short, long)]
    pub input: PathBuf,

    /// Enable automated feature generation
    #[arg(long, default_value = "true")]
    pub auto_generation: bool,

    /// Enable feature selection
    #[arg(long, default_value = "true")]
    pub feature_selection: bool,

    /// Maximum number of generated features
    #[arg(long, default_value = "1000")]
    pub max_features: usize,

    /// Feature selection threshold (0.0 - 1.0)
    #[arg(long, default_value = "0.01")]
    pub selection_threshold: f64,

    /// Generation strategies to use
    #[arg(long, value_delimiter = ',')]
    pub strategies: Option<Vec<String>>,

    /// Selection methods to use
    #[arg(long, value_delimiter = ',')]
    pub selection_methods: Option<Vec<String>>,

    /// Output detailed analysis
    #[arg(long)]
    pub detailed: bool,

    /// Benchmark feature engineering performance
    #[arg(long)]
    pub benchmark: bool,
}

pub async fn run_feature_engineering(args: FeatureEngineeringArgs) -> Result<()> {
    info!("Starting advanced feature engineering analysis");

    // Parse generation strategies
    let generation_strategies = parse_generation_strategies(args.strategies.as_deref())?;
    let selection_methods = parse_selection_methods(args.selection_methods.as_deref())?;

    // Create configuration
    let config = FeatureEngineeringConfig {
        auto_generation: args.auto_generation,
        feature_selection: args.feature_selection,
        max_generated_features: args.max_features,
        selection_threshold: args.selection_threshold,
        cv_folds: 5,
        generation_strategies,
        selection_methods,
    };

    // Initialize feature engineer
    let mut engineer = AdvancedFeatureEngineer::with_config(config);

    // Create sample findings for demonstration
    let sample_findings = create_sample_findings(&args.input);

    if args.benchmark {
        run_benchmark(&mut engineer, &sample_findings).await?;
    } else {
        run_analysis(&mut engineer, &sample_findings, args.detailed).await?;
    }

    Ok(())
}

async fn run_analysis(
    engineer: &mut AdvancedFeatureEngineer,
    findings: &[Finding],
    detailed: bool,
) -> Result<()> {
    info!(
        "Running feature engineering analysis on {} findings",
        findings.len()
    );

    for (i, finding) in findings.iter().enumerate() {
        info!(
            "Analyzing finding {}/{}: {}",
            i + 1,
            findings.len(),
            finding.rule
        );

        let features = engineer.extract_enhanced_features(finding).await?;

        println!("\n=== Finding {} ===", i + 1);
        println!("Rule: {}", finding.rule);
        println!("Message: {}", finding.message);
        println!("Severity: {:?}", finding.severity);
        println!("Generated {} features", features.len());

        if detailed {
            println!("Feature values:");
            for (j, &value) in features.iter().enumerate() {
                println!("  Feature {}: {:.4}", j, value);
            }
        }

        // Show top 5 feature values
        let mut indexed_features: Vec<(usize, f32)> =
            features.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed_features.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        println!("Top 5 features:");
        for (idx, value) in indexed_features.iter().take(5) {
            println!("  Feature {}: {:.4}", idx, value);
        }
    }

    // Show metrics
    let metrics = engineer.get_metrics();
    println!("\n=== Feature Engineering Metrics ===");
    println!(
        "Total features generated: {}",
        metrics.total_features_generated
    );
    println!("Features selected: {}", metrics.features_selected);
    println!("Generation time: {}ms", metrics.generation_time_ms);
    println!("Selection time: {}ms", metrics.selection_time_ms);
    println!("Cache hits: {}", metrics.cache_hits);
    println!("Cache misses: {}", metrics.cache_misses);

    Ok(())
}

async fn run_benchmark(engineer: &mut AdvancedFeatureEngineer, findings: &[Finding]) -> Result<()> {
    info!("Running feature engineering benchmark");

    let start_time = std::time::Instant::now();
    let mut total_features = 0;

    for finding in findings {
        let features = engineer.extract_enhanced_features(finding).await?;
        total_features += features.len();
    }

    let total_time = start_time.elapsed();
    let avg_time_per_finding = total_time.as_millis() / findings.len() as u128;

    println!("\n=== Benchmark Results ===");
    println!("Processed {} findings", findings.len());
    println!("Total features generated: {}", total_features);
    println!(
        "Average features per finding: {:.1}",
        total_features as f64 / findings.len() as f64
    );
    println!("Total processing time: {:?}", total_time);
    println!("Average time per finding: {}ms", avg_time_per_finding);

    let metrics = engineer.get_metrics();
    println!(
        "Cache hit ratio: {:.1}%",
        metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64 * 100.0
    );

    Ok(())
}

fn parse_generation_strategies(strategies: Option<&[String]>) -> Result<Vec<GenerationStrategy>> {
    let default_strategies = vec![
        GenerationStrategy::PatternBased,
        GenerationStrategy::StatisticalTransforms,
        GenerationStrategy::InteractionFeatures,
    ];

    let Some(strategies) = strategies else {
        return Ok(default_strategies);
    };

    let mut result = Vec::new();
    for strategy in strategies {
        match strategy.to_lowercase().as_str() {
            "pattern" | "pattern-based" => result.push(GenerationStrategy::PatternBased),
            "statistical" | "stats" => result.push(GenerationStrategy::StatisticalTransforms),
            "interaction" | "interactions" => result.push(GenerationStrategy::InteractionFeatures),
            "ngram" | "n-gram" => result.push(GenerationStrategy::NGramFeatures),
            "frequency" | "freq" => result.push(GenerationStrategy::FrequencyFeatures),
            _ => warn!("Unknown generation strategy: {}", strategy),
        }
    }

    if result.is_empty() {
        Ok(default_strategies)
    } else {
        Ok(result)
    }
}

fn parse_selection_methods(methods: Option<&[String]>) -> Result<Vec<SelectionMethod>> {
    let default_methods = vec![
        SelectionMethod::MutualInformation,
        SelectionMethod::VarianceThreshold,
    ];

    let Some(methods) = methods else {
        return Ok(default_methods);
    };

    let mut result = Vec::new();
    for method in methods {
        match method.to_lowercase().as_str() {
            "mi" | "mutual-information" => result.push(SelectionMethod::MutualInformation),
            "variance" | "var" => result.push(SelectionMethod::VarianceThreshold),
            "rfe" | "recursive" => result.push(SelectionMethod::RecursiveElimination),
            "correlation" | "corr" => result.push(SelectionMethod::CorrelationBased),
            "lasso" => result.push(SelectionMethod::LassoRegularization),
            _ => warn!("Unknown selection method: {}", method),
        }
    }

    if result.is_empty() {
        Ok(default_methods)
    } else {
        Ok(result)
    }
}

fn create_sample_findings(input_path: &PathBuf) -> Vec<Finding> {
    use std::collections::HashMap;
    vec![
        Finding {
            id: "finding-1".to_string(),
            analyzer: "security_analyzer".to_string(),
            rule: "security/password-hardcoded".to_string(),
            severity: Severity::Critical,
            file: input_path.clone(),
            line: 42,
            column: Some(10),
            message: "Hardcoded password detected in source code".to_string(),
            description: Some(
                "Found a hardcoded password 'admin123' which poses a security risk".to_string(),
            ),
            suggestion: Some("Use environment variables or secure configuration".to_string()),
            category: Some("security".to_string()),
            metadata: HashMap::new(),
        },
        Finding {
            id: "finding-2".to_string(),
            analyzer: "performance_analyzer".to_string(),
            rule: "performance/inefficient-loop".to_string(),
            severity: Severity::Medium,
            file: input_path.clone(),
            line: 128,
            column: Some(5),
            message: "Inefficient nested loop detected".to_string(),
            description: Some("Nested loop with O(n²) complexity could be optimized".to_string()),
            suggestion: Some("Consider using hash maps or more efficient algorithms".to_string()),
            category: Some("performance".to_string()),
            metadata: HashMap::new(),
        },
        Finding {
            id: "finding-3".to_string(),
            analyzer: "security_analyzer".to_string(),
            rule: "security/sql-injection".to_string(),
            severity: Severity::High,
            file: input_path.clone(),
            line: 201,
            column: Some(15),
            message: "Potential SQL injection vulnerability".to_string(),
            description: Some(
                "Direct string concatenation in SQL query without sanitization".to_string(),
            ),
            suggestion: Some("Use parameterized queries or prepared statements".to_string()),
            category: Some("security".to_string()),
            metadata: HashMap::new(),
        },
        Finding {
            id: "finding-4".to_string(),
            analyzer: "lint_analyzer".to_string(),
            rule: "style/unused-variable".to_string(),
            severity: Severity::Low,
            file: input_path.clone(),
            line: 89,
            column: Some(20),
            message: "Unused variable 'temp'".to_string(),
            description: Some("Variable 'temp' is declared but never used".to_string()),
            suggestion: Some("Remove unused variable or add underscore prefix".to_string()),
            category: Some("style".to_string()),
            metadata: HashMap::new(),
        },
        Finding {
            id: "finding-5".to_string(),
            analyzer: "security_analyzer".to_string(),
            rule: "security/crypto-weak".to_string(),
            severity: Severity::High,
            file: input_path.clone(),
            line: 156,
            column: Some(8),
            message: "Weak cryptographic algorithm detected".to_string(),
            description: Some(
                "MD5 hash algorithm is cryptographically broken and should not be used".to_string(),
            ),
            suggestion: Some("Use SHA-256 or better cryptographic hash functions".to_string()),
            category: Some("security".to_string()),
            metadata: HashMap::new(),
        },
    ]
}
