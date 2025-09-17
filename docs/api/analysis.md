# Analysis API Documentation

This document provides comprehensive API documentation for CodeGuardian's code analysis interfaces and security scanning capabilities.

## Table of Contents

- [Analysis Engine](#analysis-engine)
- [Security Analysis](#security-analysis)
- [ML-Enhanced Detection](#ml-enhanced-detection)
- [Pattern Recognition](#pattern-recognition)
- [Duplicate Detection](#duplicate-detection)
- [Analysis Results](#analysis-results)
- [Analysis Configuration](#analysis-configuration)

## Analysis Engine

### Core Analysis Interface

```rust
pub trait AnalysisEngine: Send + Sync {
    async fn analyze(&self, files: &[PathBuf], config: &AnalysisConfig) -> Result<AnalysisResults>;
    async fn analyze_single(&self, file: &Path, config: &AnalysisConfig) -> Result<Vec<SecurityIssue>>;
    fn supported_languages(&self) -> Vec<String>;
    fn capabilities(&self) -> AnalysisCapabilities;
}
```

### AnalysisConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub enable_advanced_analysis: bool,
    pub max_file_size_mb: usize,
    pub max_files_per_scan: usize,
    pub enable_ml_detection: bool,
    pub confidence_threshold: f64,
    pub severity_threshold: Severity,
    pub enable_cwe_mapping: bool,
    pub custom_rules_path: Option<String>,
    pub ignore_patterns: Vec<String>,
    pub analysis_timeout_seconds: u64,
    pub parallel_workers: usize,
}
```

## Security Analysis

### SecurityAnalyzer

```rust
pub struct SecurityAnalyzer {
    config: AnalysisConfig,
    rules_engine: RulesEngine,
    ml_detector: Option<MlDetector>,
    pattern_recognizer: PatternRecognizer,
}
```

**Methods:**
```rust
impl SecurityAnalyzer {
    pub fn new(config: AnalysisConfig) -> Self;
    pub async fn analyze_files(&self, files: &[PathBuf]) -> Result<AnalysisResults>;
    pub async fn analyze_content(&self, content: &str, language: &str) -> Result<Vec<SecurityIssue>>;
    pub fn add_custom_rule(&mut self, rule: SecurityRule) -> Result<()>;
    pub fn load_rules_from_path(&mut self, path: &Path) -> Result<()>;
}
```

### SecurityIssue

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub id: String,
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub file: PathBuf,
    pub line: u32,
    pub column: Option<u32>,
    pub code: Option<String>,
    pub confidence: f64,
    pub cwe_id: Option<String>,
    pub references: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub remediation_suggestions: Vec<String>,
}
```

### Severity Levels

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}
```

### Security Categories

- **Injection**: SQL injection, command injection, etc.
- **Authentication**: Weak authentication, session management
- **Authorization**: Access control issues
- **Cryptography**: Weak encryption, improper key management
- **Input Validation**: Unvalidated input, XSS, CSRF
- **Configuration**: Security misconfigurations
- **Information Disclosure**: Sensitive data exposure
- **Error Handling**: Improper error handling
- **Code Quality**: Potential security issues in code structure

## ML-Enhanced Detection

### MlDetector (New in v0.2.0-alpha.5)

```rust
pub struct MlDetector {
    model: PatternRecognitionModel,
    confidence_threshold: f64,
    feature_extractor: FeatureExtractor,
}
```

**Methods:**
```rust
impl MlDetector {
    pub fn new(model_path: &Path, config: &MlConfig) -> Result<Self>;
    pub async fn detect_patterns(&self, content: &str) -> Result<Vec<PatternMatch>>;
    pub async fn classify_issue(&self, issue: &SecurityIssue) -> Result<ClassificationResult>;
    pub fn update_model(&mut self, training_data: &[TrainingExample]) -> Result<()>;
}
```

### PatternMatch

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_type: String,
    pub confidence: f64,
    pub location: CodeLocation,
    pub context: String,
    pub features: HashMap<String, f64>,
    pub similar_patterns: Vec<String>,
}
```

### ClassificationResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub predicted_severity: Severity,
    pub confidence: f64,
    pub similar_issues: Vec<String>,
    pub recommended_actions: Vec<String>,
}
```

## Pattern Recognition

### PatternRecognizer

```rust
pub struct PatternRecognizer {
    patterns: HashMap<String, Pattern>,
    config: PatternConfig,
}
```

**Methods:**
```rust
impl PatternRecognizer {
    pub fn new(config: PatternConfig) -> Self;
    pub async fn find_patterns(&self, content: &str, language: &str) -> Result<Vec<PatternMatch>>;
    pub fn add_pattern(&mut self, name: String, pattern: Pattern) -> Result<()>;
    pub fn load_patterns_from_file(&mut self, path: &Path) -> Result<()>;
}
```

### Pattern Types

- **Regex Patterns**: Regular expression-based detection
- **AST Patterns**: Abstract syntax tree pattern matching
- **Semantic Patterns**: Code semantics-based detection
- **Flow Patterns**: Control/data flow analysis
- **ML Patterns**: Machine learning-based pattern recognition

### Pattern

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Regex {
        pattern: String,
        case_sensitive: bool,
        multiline: bool,
    },
    Ast {
        node_type: String,
        properties: HashMap<String, String>,
    },
    Semantic {
        rule: String,
        conditions: Vec<Condition>,
    },
    Flow {
        source_pattern: String,
        sink_pattern: String,
        sanitizer_patterns: Vec<String>,
    },
}
```

## Duplicate Detection

### DuplicateDetector

```rust
pub struct DuplicateDetector {
    config: DuplicateConfig,
    hasher: ContentHasher,
    similarity_analyzer: SimilarityAnalyzer,
}
```

**Methods:**
```rust
impl DuplicateDetector {
    pub fn new(config: DuplicateConfig) -> Self;
    pub async fn find_duplicates(&self, files: &[PathBuf]) -> Result<Vec<DuplicateGroup>>;
    pub async fn find_similar_files(&self, file: &Path, threshold: f64) -> Result<Vec<SimilarityResult>>;
    pub fn calculate_similarity(&self, content1: &str, content2: &str) -> f64;
}
```

### DuplicateGroup

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub id: String,
    pub files: Vec<DuplicateFile>,
    pub similarity_score: f64,
    pub duplicate_type: DuplicateType,
    pub estimated_savings: usize,
    pub risk_level: RiskLevel,
}
```

### DuplicateType

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DuplicateType {
    Exact,           // Identical content
    NearExact,       // Minor differences (whitespace, comments)
    Similar,         // Similar structure/logic
    Refactored,      // Same functionality, different implementation
    TemplateBased,   // Generated from templates
}
```

### SimilarityResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub file: PathBuf,
    pub similarity_score: f64,
    pub matching_lines: Vec<(u32, u32)>,
    pub duplicate_type: DuplicateType,
    pub confidence: f64,
}
```

## Analysis Results

### AnalysisResults

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub issues: Vec<SecurityIssue>,
    pub summary: AnalysisSummary,
    pub metadata: AnalysisMetadata,
    pub duplicates: Vec<DuplicateGroup>,
    pub patterns: Vec<PatternMatch>,
}
```

### AnalysisSummary

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_files: usize,
    pub files_analyzed: usize,
    pub total_issues: usize,
    pub issues_by_severity: HashMap<Severity, usize>,
    pub issues_by_category: HashMap<String, usize>,
    pub duplicate_groups: usize,
    pub total_duplicates: usize,
    pub analysis_time_seconds: f64,
    pub success_rate: f64,
}
```

### AnalysisMetadata

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub tool_version: String,
    pub analysis_timestamp: DateTime<Utc>,
    pub config_hash: String,
    pub analysis_id: String,
    pub environment_info: EnvironmentInfo,
    pub performance_metrics: PerformanceMetrics,
}
```

## Analysis Configuration

### AnalysisCapabilities

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCapabilities {
    pub supported_languages: Vec<String>,
    pub supported_patterns: Vec<String>,
    pub ml_enabled: bool,
    pub duplicate_detection: bool,
    pub parallel_processing: bool,
    pub streaming_support: bool,
    pub custom_rules_support: bool,
}
```

### Rules Engine

```rust
pub struct RulesEngine {
    rules: Vec<SecurityRule>,
    config: RulesConfig,
}
```

**Methods:**
```rust
impl RulesEngine {
    pub fn new(config: RulesConfig) -> Self;
    pub async fn evaluate_rules(&self, content: &str, language: &str) -> Result<Vec<RuleMatch>>;
    pub fn add_rule(&mut self, rule: SecurityRule) -> Result<()>;
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<()>;
    pub fn enable_rule(&mut self, rule_id: &str) -> Result<()>;
    pub fn disable_rule(&mut self, rule_id: &str) -> Result<()>;
}
```

### SecurityRule

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub category: String,
    pub language: Option<String>,
    pub pattern: Pattern,
    pub conditions: Vec<Condition>,
    pub message: String,
    pub cwe_id: Option<String>,
    pub references: Vec<String>,
    pub enabled: bool,
}
```

## Usage Examples

### Basic Security Analysis

```rust
use do_codeguardian::analysis::{SecurityAnalyzer, AnalysisConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AnalysisConfig {
        enable_advanced_analysis: true,
        max_file_size_mb: 10,
        confidence_threshold: 0.7,
        ..Default::default()
    };

    let analyzer = SecurityAnalyzer::new(config);
    let files = vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/lib.rs"),
    ];

    let results = analyzer.analyze_files(&files).await?;

    println!("Analysis completed:");
    println!("- Files analyzed: {}", results.summary.files_analyzed);
    println!("- Issues found: {}", results.issues.len());
    println!("- High severity: {}", results.summary.issues_by_severity[&Severity::High]);

    Ok(())
}
```

### ML-Enhanced Analysis

```rust
use do_codeguardian::analysis::{MlDetector, MlConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ml_config = MlConfig {
        model_path: PathBuf::from("./models/security_model"),
        confidence_threshold: 0.8,
        enable_pattern_learning: true,
    };

    let detector = MlDetector::new("./models/security_model", &ml_config).await?;
    let content = "SELECT * FROM users WHERE id = $1";
    let patterns = detector.detect_patterns(content).await?;

    for pattern in patterns {
        println!("Detected pattern: {} (confidence: {:.2})",
                pattern.pattern_type, pattern.confidence);
    }

    Ok(())
}
```

### Duplicate Detection

```rust
use do_codeguardian::analysis::{DuplicateDetector, DuplicateConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DuplicateConfig {
        min_similarity_threshold: 0.8,
        max_file_size_mb: 5,
        enable_ml_similarity: true,
    };

    let detector = DuplicateDetector::new(config);
    let files = vec![
        PathBuf::from("src/"),
        PathBuf::from("tests/"),
    ];

    let duplicates = detector.find_duplicates(&files).await?;

    println!("Found {} duplicate groups", duplicates.len());
    for group in duplicates {
        println!("Group {}: {} files, similarity: {:.2}",
                group.id, group.files.len(), group.similarity_score);
    }

    Ok(())
}
```

### Custom Rule Creation

```rust
use do_codeguardian::analysis::{SecurityRule, Pattern, Severity};

let custom_rule = SecurityRule {
    id: "custom-sql-injection".to_string(),
    name: "Custom SQL Injection Detection".to_string(),
    description: "Detects potential SQL injection vulnerabilities".to_string(),
    severity: Severity::High,
    category: "Injection".to_string(),
    language: Some("rust".to_string()),
    pattern: Pattern::Regex {
        pattern: r#"format!.*\{\}"#.to_string(),
        case_sensitive: false,
        multiline: true,
    },
    conditions: vec![],
    message: "Potential SQL injection via string formatting".to_string(),
    cwe_id: Some("CWE-89".to_string()),
    references: vec!["https://owasp.org/www-community/attacks/SQL_Injection".to_string()],
    enabled: true,
};

analyzer.add_custom_rule(custom_rule)?;
```

### Pattern-Based Analysis

```rust
use do_codeguardian::analysis::{PatternRecognizer, PatternConfig};

let config = PatternConfig {
    enable_ast_patterns: true,
    enable_semantic_patterns: true,
    max_pattern_depth: 10,
};

let recognizer = PatternRecognizer::new(config);

// Add a custom pattern
recognizer.add_pattern(
    "unsafe_sql".to_string(),
    Pattern::Semantic {
        rule: "sql_injection".to_string(),
        conditions: vec![
            Condition::Contains("SELECT".to_string()),
            Condition::Contains("WHERE".to_string()),
            Condition::VariableConcatenation,
        ],
    }
)?;

let content = r#"
fn get_user(id: &str) -> Result<User> {
    let query = format!("SELECT * FROM users WHERE id = {}", id);
    database.execute(&query)
}
"#;

let matches = recognizer.find_patterns(content, "rust").await?;
println!("Found {} pattern matches", matches.len());
```

## Performance Considerations

- **File Size Limits**: Configure `max_file_size_mb` to prevent memory issues
- **Parallel Processing**: Use `parallel_workers` for large codebases
- **Caching**: Enable analysis result caching for repeated scans
- **Streaming**: Use streaming analysis for very large files
- **Timeout**: Set `analysis_timeout_seconds` to prevent hanging

## Integration with CI/CD

```rust
use do_codeguardian::analysis::AnalysisConfig;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AnalysisConfig {
        enable_advanced_analysis: true,
        severity_threshold: Severity::Medium,
        confidence_threshold: 0.6,
        ..Default::default()
    };

    let analyzer = SecurityAnalyzer::new(config);

    // Get changed files from git
    let changed_files = get_changed_files()?;

    let results = analyzer.analyze_files(&changed_files).await?;

    // Check for blocking issues
    let blocking_issues = results.issues.iter()
        .filter(|issue| issue.severity >= Severity::High)
        .count();

    if blocking_issues > 0 {
        eprintln!("Found {} high/critical severity issues", blocking_issues);
        std::process::exit(1);
    }

    println!("Security analysis passed");
    Ok(())
}

fn get_changed_files() -> Result<Vec<PathBuf>> {
    // Implementation to get changed files from git
    Ok(vec![])
}
```
