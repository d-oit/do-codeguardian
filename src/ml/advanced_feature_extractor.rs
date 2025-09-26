//! Advanced Feature Extraction for Enhanced ML Performance
//!
//! This module provides sophisticated feature extraction capabilities that go beyond
//! basic AST analysis to include semantic understanding, context awareness, and
//! domain-specific security patterns.

use crate::ml::enhanced_feature_extractor::EnhancedFeatureExtractor;
use crate::ml::feature_extractor::FeatureExtractor;
use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Advanced feature extractor with semantic analysis and context awareness
pub struct AdvancedFeatureExtractor {
    base_extractor: FeatureExtractor,
    enhanced_extractor: EnhancedFeatureExtractor,
    semantic_analyzer: SemanticAnalyzer,
    context_analyzer: ContextAnalyzer,
    security_pattern_detector: SecurityPatternDetector,
    feature_cache: HashMap<String, CachedAdvancedFeatures>,
}

/// Semantic analysis for code understanding
#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    keyword_patterns: HashMap<String, f32>,
    security_keywords: Vec<String>,
    risk_indicators: Vec<String>,
}

/// Context analysis for file and project understanding
#[derive(Debug, Clone)]
pub struct ContextAnalyzer {
    file_importance_weights: HashMap<String, f32>,
    directory_context_scores: HashMap<String, f32>,
    project_metadata: Option<ProjectMetadata>,
}

/// Security-specific pattern detection
#[derive(Debug, Clone)]
pub struct SecurityPatternDetector {
    vulnerability_patterns: HashMap<String, VulnerabilityPattern>,
    crypto_patterns: Vec<String>,
    auth_patterns: Vec<String>,
    injection_patterns: Vec<String>,
}

/// Project metadata for enhanced context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_type: ProjectType,
    pub security_level: SecurityLevel,
    pub compliance_requirements: Vec<String>,
    pub tech_stack: Vec<String>,
    pub sensitive_directories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    WebApplication,
    SystemLibrary,
    CryptoLibrary,
    NetworkService,
    DesktopApplication,
    EmbeddedSystem,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Critical, // Banking, Healthcare, Defense
    High,     // E-commerce, Personal Data
    Medium,   // General Business
    Low,      // Open Source, Educational
}

/// Vulnerability pattern definition
#[derive(Debug, Clone)]
pub struct VulnerabilityPattern {
    pub name: String,
    pub confidence: f32,
    pub severity_multiplier: f32,
    pub patterns: Vec<String>,
    pub context_requirements: Vec<String>,
}

/// Advanced feature set with 48 dimensions
#[derive(Debug, Clone, Default)]
pub struct AdvancedFeatures {
    // Base features (8 dimensions)
    pub base_features: Vec<f32>,

    // Enhanced AST features (16 dimensions)
    pub ast_features: Vec<f32>,

    // Semantic features (8 dimensions)
    pub semantic_score: f32,
    pub keyword_density: f32,
    pub security_keyword_match: f32,
    pub risk_indicator_score: f32,
    pub code_pattern_complexity: f32,
    pub variable_naming_quality: f32,
    pub comment_quality_score: f32,
    pub documentation_clarity: f32,

    // Context features (8 dimensions)
    pub file_importance: f32,
    pub directory_sensitivity: f32,
    pub project_criticality: f32,
    pub relative_file_position: f32,
    pub file_size_relative: f32,
    pub modification_frequency: f32,
    pub author_expertise_score: f32,
    pub code_review_coverage: f32,

    // Security-specific features (8 dimensions)
    pub vulnerability_pattern_match: f32,
    pub crypto_usage_score: f32,
    pub auth_mechanism_complexity: f32,
    pub injection_risk_score: f32,
    pub privilege_escalation_risk: f32,
    pub data_exposure_risk: f32,
    pub network_security_score: f32,
    pub input_validation_quality: f32,
}

/// Cached advanced features with metadata
#[derive(Debug, Clone)]
struct CachedAdvancedFeatures {
    features: AdvancedFeatures,
    file_hash: u64,
    timestamp: std::time::SystemTime,
    analysis_version: u32,
}

impl AdvancedFeatureExtractor {
    const ANALYSIS_VERSION: u32 = 1;
    const MAX_CACHE_SIZE: usize = 500;

    pub fn new() -> Self {
        Self {
            base_extractor: FeatureExtractor::new(),
            enhanced_extractor: EnhancedFeatureExtractor::new(),
            semantic_analyzer: SemanticAnalyzer::new(),
            context_analyzer: ContextAnalyzer::new(),
            security_pattern_detector: SecurityPatternDetector::new(),
            feature_cache: HashMap::new(),
        }
    }

    /// Extract comprehensive 48-dimension feature vector
    pub async fn extract_advanced_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        // Check cache first
        let cache_key = format!("{}:{}", finding.file.display(), finding.line);
        if let Some(cached) = self.feature_cache.get(&cache_key) {
            if cached.analysis_version == Self::ANALYSIS_VERSION {
                return Ok(cached.features.to_vector());
            }
        }

        // Extract features from all analyzers
        let base_features = self.base_extractor.extract_features(finding)?;
        let enhanced_features = self
            .enhanced_extractor
            .extract_enhanced_features(finding)
            .await?;

        // Split enhanced features into base and AST parts
        let ast_features = enhanced_features[8..].to_vec();

        // Perform advanced analysis
        let semantic_features = self.semantic_analyzer.analyze_finding(finding).await?;
        let context_features = self.context_analyzer.analyze_context(finding).await?;
        let security_features = self
            .security_pattern_detector
            .analyze_security_patterns(finding)
            .await?;

        // Combine all features
        let advanced_features = AdvancedFeatures {
            base_features,
            ast_features,
            semantic_score: semantic_features.0,
            keyword_density: semantic_features.1,
            security_keyword_match: semantic_features.2,
            risk_indicator_score: semantic_features.3,
            code_pattern_complexity: semantic_features.4,
            variable_naming_quality: semantic_features.5,
            comment_quality_score: semantic_features.6,
            documentation_clarity: semantic_features.7,
            file_importance: context_features.0,
            directory_sensitivity: context_features.1,
            project_criticality: context_features.2,
            relative_file_position: context_features.3,
            file_size_relative: context_features.4,
            modification_frequency: context_features.5,
            author_expertise_score: context_features.6,
            code_review_coverage: context_features.7,
            vulnerability_pattern_match: security_features.0,
            crypto_usage_score: security_features.1,
            auth_mechanism_complexity: security_features.2,
            injection_risk_score: security_features.3,
            privilege_escalation_risk: security_features.4,
            data_exposure_risk: security_features.5,
            network_security_score: security_features.6,
            input_validation_quality: security_features.7,
        };

        // Cache the result
        self.cache_features(&cache_key, &advanced_features)?;

        Ok(advanced_features.to_vector())
    }

    /// Configure project metadata for enhanced context analysis
    pub fn configure_project(&mut self, metadata: ProjectMetadata) {
        self.context_analyzer.project_metadata = Some(metadata);
    }

    /// Get feature names for the 48-dimension vector
    pub fn get_feature_names() -> Vec<String> {
        let mut names = Vec::new();

        // Base features (8)
        names.extend(vec![
            "severity_score",
            "file_type_relevance",
            "analyzer_confidence",
            "message_length",
            "line_position",
            "has_description",
            "has_suggestion",
            "rule_specificity",
        ]);

        // AST features (16)
        names.extend(vec![
            "ast_cyclomatic_complexity",
            "ast_nesting_depth",
            "ast_function_count",
            "ast_struct_count",
            "ast_enum_count",
            "ast_impl_block_count",
            "ast_unsafe_block_count",
            "ast_panic_call_count",
            "ast_unwrap_call_count",
            "ast_expect_call_count",
            "ast_comment_density",
            "ast_documentation_coverage",
            "ast_test_function_ratio",
            "ast_string_literal_count",
            "ast_numeric_literal_count",
            "ast_macro_usage_count",
        ]);

        // Semantic features (8)
        names.extend(vec![
            "semantic_score",
            "keyword_density",
            "security_keyword_match",
            "risk_indicator_score",
            "code_pattern_complexity",
            "variable_naming_quality",
            "comment_quality_score",
            "documentation_clarity",
        ]);

        // Context features (8)
        names.extend(vec![
            "file_importance",
            "directory_sensitivity",
            "project_criticality",
            "relative_file_position",
            "file_size_relative",
            "modification_frequency",
            "author_expertise_score",
            "code_review_coverage",
        ]);

        // Security features (8)
        names.extend(vec![
            "vulnerability_pattern_match",
            "crypto_usage_score",
            "auth_mechanism_complexity",
            "injection_risk_score",
            "privilege_escalation_risk",
            "data_exposure_risk",
            "network_security_score",
            "input_validation_quality",
        ]);

        names.into_iter().map(|s| s.to_string()).collect()
    }

    /// Cache features with size management
    fn cache_features(&mut self, key: &str, features: &AdvancedFeatures) -> Result<()> {
        if self.feature_cache.len() >= Self::MAX_CACHE_SIZE {
            // Remove oldest entry (simple FIFO)
            if let Some(oldest_key) = self.feature_cache.keys().next().cloned() {
                self.feature_cache.remove(&oldest_key);
            }
        }

        let cached = CachedAdvancedFeatures {
            features: features.clone(),
            file_hash: self.calculate_content_hash(&format!("{:?}", features)),
            timestamp: std::time::SystemTime::now(),
            analysis_version: Self::ANALYSIS_VERSION,
        };

        self.feature_cache.insert(key.to_string(), cached);
        Ok(())
    }

    fn calculate_content_hash(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}

impl AdvancedFeatures {
    /// Convert to 48-dimension feature vector
    pub fn to_vector(&self) -> Vec<f32> {
        let mut vector = Vec::with_capacity(48);

        // Base features (8)
        vector.extend_from_slice(&self.base_features);

        // AST features (16)
        vector.extend_from_slice(&self.ast_features);

        // Semantic features (8)
        vector.extend_from_slice(&[
            self.semantic_score,
            self.keyword_density,
            self.security_keyword_match,
            self.risk_indicator_score,
            self.code_pattern_complexity,
            self.variable_naming_quality,
            self.comment_quality_score,
            self.documentation_clarity,
        ]);

        // Context features (8)
        vector.extend_from_slice(&[
            self.file_importance,
            self.directory_sensitivity,
            self.project_criticality,
            self.relative_file_position,
            self.file_size_relative,
            self.modification_frequency,
            self.author_expertise_score,
            self.code_review_coverage,
        ]);

        // Security features (8)
        vector.extend_from_slice(&[
            self.vulnerability_pattern_match,
            self.crypto_usage_score,
            self.auth_mechanism_complexity,
            self.injection_risk_score,
            self.privilege_escalation_risk,
            self.data_exposure_risk,
            self.network_security_score,
            self.input_validation_quality,
        ]);

        vector
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut keyword_patterns = HashMap::new();

        // Security-related keywords with weights
        keyword_patterns.insert("unsafe".to_string(), 0.9);
        keyword_patterns.insert("password".to_string(), 0.8);
        keyword_patterns.insert("secret".to_string(), 0.9);
        keyword_patterns.insert("token".to_string(), 0.7);
        keyword_patterns.insert("auth".to_string(), 0.6);
        keyword_patterns.insert("crypto".to_string(), 0.8);
        keyword_patterns.insert("hash".to_string(), 0.5);
        keyword_patterns.insert("encrypt".to_string(), 0.7);
        keyword_patterns.insert("decrypt".to_string(), 0.7);
        keyword_patterns.insert("validate".to_string(), 0.6);

        let security_keywords = vec![
            "vulnerability".to_string(),
            "exploit".to_string(),
            "attack".to_string(),
            "malicious".to_string(),
            "injection".to_string(),
            "xss".to_string(),
            "csrf".to_string(),
            "privilege".to_string(),
            "escalation".to_string(),
            "backdoor".to_string(),
            "trojan".to_string(),
            "malware".to_string(),
        ];

        let risk_indicators = vec![
            "todo".to_string(),
            "fixme".to_string(),
            "hack".to_string(),
            "temporary".to_string(),
            "workaround".to_string(),
            "broken".to_string(),
            "deprecated".to_string(),
            "legacy".to_string(),
            "insecure".to_string(),
        ];

        Self {
            keyword_patterns,
            security_keywords,
            risk_indicators,
        }
    }

    /// Analyze semantic features of a finding
    pub async fn analyze_finding(
        &self,
        finding: &Finding,
    ) -> Result<(f32, f32, f32, f32, f32, f32, f32, f32)> {
        let content = self.get_file_content(&finding.file).await?;
        let finding_context = self.extract_finding_context(&content, finding.line)?;

        let semantic_score = self.calculate_semantic_score(&finding_context);
        let keyword_density = self.calculate_keyword_density(&finding_context);
        let security_keyword_match = self.calculate_security_keyword_match(&finding_context);
        let risk_indicator_score = self.calculate_risk_indicator_score(&finding_context);
        let code_pattern_complexity = self.analyze_code_pattern_complexity(&finding_context);
        let variable_naming_quality = self.analyze_variable_naming_quality(&finding_context);
        let comment_quality_score = self.analyze_comment_quality(&finding_context);
        let documentation_clarity = self.analyze_documentation_clarity(&finding_context);

        Ok((
            semantic_score,
            keyword_density,
            security_keyword_match,
            risk_indicator_score,
            code_pattern_complexity,
            variable_naming_quality,
            comment_quality_score,
            documentation_clarity,
        ))
    }

    async fn get_file_content(&self, file_path: &Path) -> Result<String> {
        tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
    }

    fn extract_finding_context(&self, content: &str, line_number: u32) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let line_idx = (line_number as usize).saturating_sub(1);

        // Extract context: 5 lines before and after
        let start = line_idx.saturating_sub(5);
        let end = (line_idx + 6).min(lines.len());

        Ok(lines[start..end].join("\n"))
    }

    fn calculate_semantic_score(&self, context: &str) -> f32 {
        let mut score = 0.0;
        let context_lower = context.to_lowercase();

        for (keyword, weight) in &self.keyword_patterns {
            if context_lower.contains(keyword) {
                score += weight;
            }
        }

        (score / 10.0).min(1.0) // Normalize to 0-1
    }

    fn calculate_keyword_density(&self, context: &str) -> f32 {
        let words: Vec<&str> = context.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let keyword_count = words
            .iter()
            .filter(|word| self.keyword_patterns.contains_key(&word.to_lowercase()))
            .count();

        keyword_count as f32 / words.len() as f32
    }

    fn calculate_security_keyword_match(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let matches = self
            .security_keywords
            .iter()
            .filter(|keyword| context_lower.contains(*keyword))
            .count();

        (matches as f32 / self.security_keywords.len() as f32).min(1.0)
    }

    fn calculate_risk_indicator_score(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let matches = self
            .risk_indicators
            .iter()
            .filter(|indicator| context_lower.contains(*indicator))
            .count();

        (matches as f32 / 3.0).min(1.0) // Normalize based on common indicators
    }

    fn analyze_code_pattern_complexity(&self, context: &str) -> f32 {
        // Count nested structures, complex expressions
        let nesting_chars = context
            .chars()
            .filter(|c| matches!(c, '{' | '(' | '['))
            .count();
        let lines = context.lines().count();

        if lines == 0 {
            return 0.0;
        }

        (nesting_chars as f32 / lines as f32 / 2.0).min(1.0)
    }

    fn analyze_variable_naming_quality(&self, context: &str) -> f32 {
        // Simple heuristic: check for descriptive variable names
        let words: Vec<&str> = context.split_whitespace().collect();
        let descriptive_names = words
            .iter()
            .filter(|word| {
                word.len() > 3
                    && word.chars().any(|c| c.is_lowercase())
                    && word.chars().any(|c| c == '_' || c.is_uppercase())
            })
            .count();

        if words.is_empty() {
            return 0.5; // Neutral score
        }

        (descriptive_names as f32 / words.len() as f32 * 10.0).min(1.0)
    }

    fn analyze_comment_quality(&self, context: &str) -> f32 {
        let lines: Vec<&str> = context.lines().collect();
        let comment_lines = lines
            .iter()
            .filter(|line| line.trim().starts_with("//") || line.trim().starts_with("/*"))
            .count();

        if lines.is_empty() {
            return 0.0;
        }

        let comment_ratio = comment_lines as f32 / lines.len() as f32;

        // Quality heuristic: good comments are neither too sparse nor too dense
        if comment_ratio > 0.1 && comment_ratio < 0.5 {
            0.8
        } else if comment_ratio > 0.05 {
            0.6
        } else {
            0.3
        }
    }

    fn analyze_documentation_clarity(&self, context: &str) -> f32 {
        // Check for doc comments and their quality
        let doc_comments = context
            .lines()
            .filter(|line| line.trim().starts_with("///") || line.trim().starts_with("/**"))
            .count();

        let total_lines = context.lines().count();

        if total_lines == 0 {
            return 0.0;
        }

        let doc_ratio = doc_comments as f32 / total_lines as f32;
        (doc_ratio * 5.0).min(1.0)
    }
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        let mut file_importance_weights = HashMap::new();

        // File importance based on common patterns
        file_importance_weights.insert("main.rs".to_string(), 1.0);
        file_importance_weights.insert("lib.rs".to_string(), 0.9);
        file_importance_weights.insert("mod.rs".to_string(), 0.7);
        file_importance_weights.insert("config".to_string(), 0.8);
        file_importance_weights.insert("security".to_string(), 0.9);
        file_importance_weights.insert("auth".to_string(), 0.8);
        file_importance_weights.insert("crypto".to_string(), 0.9);

        let mut directory_context_scores = HashMap::new();
        directory_context_scores.insert("src".to_string(), 0.9);
        directory_context_scores.insert("security".to_string(), 1.0);
        directory_context_scores.insert("auth".to_string(), 0.9);
        directory_context_scores.insert("crypto".to_string(), 1.0);
        directory_context_scores.insert("config".to_string(), 0.8);
        directory_context_scores.insert("test".to_string(), 0.3);
        directory_context_scores.insert("example".to_string(), 0.2);
        directory_context_scores.insert("doc".to_string(), 0.1);

        Self {
            file_importance_weights,
            directory_context_scores,
            project_metadata: None,
        }
    }

    /// Analyze contextual features
    pub async fn analyze_context(
        &self,
        finding: &Finding,
    ) -> Result<(f32, f32, f32, f32, f32, f32, f32, f32)> {
        let file_importance = self.calculate_file_importance(&finding.file);
        let directory_sensitivity = self.calculate_directory_sensitivity(&finding.file);
        let project_criticality = self.calculate_project_criticality();
        let relative_file_position =
            self.calculate_relative_file_position(&finding.file, finding.line);
        let file_size_relative = self.calculate_file_size_relative(&finding.file).await;
        let modification_frequency = self.calculate_modification_frequency(&finding.file).await;
        let author_expertise_score = self.calculate_author_expertise_score(&finding.file).await;
        let code_review_coverage = self.calculate_code_review_coverage(&finding.file).await;

        Ok((
            file_importance,
            directory_sensitivity,
            project_criticality,
            relative_file_position,
            file_size_relative,
            modification_frequency,
            author_expertise_score,
            code_review_coverage,
        ))
    }

    fn calculate_file_importance(&self, file_path: &Path) -> f32 {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            // Check exact matches first
            if let Some(&weight) = self.file_importance_weights.get(file_name) {
                return weight;
            }

            // Check partial matches
            for (pattern, &weight) in &self.file_importance_weights {
                if file_name.contains(pattern) {
                    return weight;
                }
            }
        }

        0.5 // Default importance
    }

    fn calculate_directory_sensitivity(&self, file_path: &Path) -> f32 {
        let path_str = file_path.to_string_lossy().to_lowercase();

        for (dir_pattern, &score) in &self.directory_context_scores {
            if path_str.contains(dir_pattern) {
                return score;
            }
        }

        0.5 // Default sensitivity
    }

    fn calculate_project_criticality(&self) -> f32 {
        match &self.project_metadata {
            Some(metadata) => match metadata.security_level {
                SecurityLevel::Critical => 1.0,
                SecurityLevel::High => 0.8,
                SecurityLevel::Medium => 0.6,
                SecurityLevel::Low => 0.4,
            },
            None => 0.5, // Unknown project
        }
    }

    fn calculate_relative_file_position(&self, file_path: &Path, line_number: u32) -> f32 {
        // Simplified: early lines are more important
        let normalized_line = (line_number as f32 / 1000.0).min(1.0);
        1.0 - normalized_line // Invert so early lines have higher scores
    }

    async fn calculate_file_size_relative(&self, file_path: &Path) -> f32 {
        match tokio::fs::metadata(file_path).await {
            Ok(metadata) => {
                let size_kb = metadata.len() as f32 / 1024.0;
                // Normalize file size (larger files might be more important up to a point)
                if size_kb < 1.0 {
                    0.3 // Very small files
                } else if size_kb < 10.0 {
                    0.7 // Small files
                } else if size_kb < 100.0 {
                    1.0 // Medium files (most important)
                } else {
                    0.8 // Large files (slightly less important due to complexity)
                }
            }
            Err(_) => 0.5, // Default if metadata unavailable
        }
    }

    async fn calculate_modification_frequency(&self, _file_path: &Path) -> f32 {
        // Placeholder - would integrate with git history
        // More frequently modified files might indicate active development
        0.5 // Default score
    }

    async fn calculate_author_expertise_score(&self, _file_path: &Path) -> f32 {
        // Placeholder - would integrate with git blame and author history
        // Authors with more commits in security-related files have higher expertise
        0.5 // Default score
    }

    async fn calculate_code_review_coverage(&self, _file_path: &Path) -> f32 {
        // Placeholder - would integrate with PR/review history
        // Files with thorough code review have higher scores
        0.5 // Default score
    }
}

impl SecurityPatternDetector {
    pub fn new() -> Self {
        let mut vulnerability_patterns = HashMap::new();

        // SQL Injection patterns
        vulnerability_patterns.insert(
            "sql_injection".to_string(),
            VulnerabilityPattern {
                name: "SQL Injection".to_string(),
                confidence: 0.9,
                severity_multiplier: 1.5,
                patterns: vec![
                    "SELECT.*FROM.*WHERE".to_string(),
                    "INSERT.*INTO.*VALUES".to_string(),
                    "UPDATE.*SET.*WHERE".to_string(),
                    "DELETE.*FROM.*WHERE".to_string(),
                    "DROP.*TABLE".to_string(),
                    "UNION.*SELECT".to_string(),
                ],
                context_requirements: vec!["user_input".to_string(), "database".to_string()],
            },
        );

        // XSS patterns
        vulnerability_patterns.insert(
            "xss".to_string(),
            VulnerabilityPattern {
                name: "Cross-Site Scripting".to_string(),
                confidence: 0.8,
                severity_multiplier: 1.3,
                patterns: vec![
                    "innerHTML".to_string(),
                    "document.write".to_string(),
                    "eval(".to_string(),
                    "dangerouslySetInnerHTML".to_string(),
                    "<script>".to_string(),
                    "javascript:".to_string(),
                ],
                context_requirements: vec!["web".to_string(), "html".to_string()],
            },
        );

        // Command Injection patterns
        vulnerability_patterns.insert(
            "command_injection".to_string(),
            VulnerabilityPattern {
                name: "Command Injection".to_string(),
                confidence: 0.85,
                severity_multiplier: 1.6,
                patterns: vec![
                    "system(".to_string(),
                    "exec(".to_string(),
                    "shell_exec".to_string(),
                    "popen(".to_string(),
                    "Runtime.getRuntime().exec".to_string(),
                ],
                context_requirements: vec!["system_command".to_string()],
            },
        );

        let crypto_patterns = vec![
            "AES".to_string(),
            "DES".to_string(),
            "RSA".to_string(),
            "SHA".to_string(),
            "MD5".to_string(),
            "encrypt".to_string(),
            "decrypt".to_string(),
            "cipher".to_string(),
            "keystore".to_string(),
            "certificate".to_string(),
            "pkcs".to_string(),
        ];

        let auth_patterns = vec![
            "authenticate".to_string(),
            "authorize".to_string(),
            "login".to_string(),
            "logout".to_string(),
            "session".to_string(),
            "token".to_string(),
            "jwt".to_string(),
            "oauth".to_string(),
            "passport".to_string(),
            "credential".to_string(),
            "permission".to_string(),
        ];

        let injection_patterns = vec![
            "SELECT".to_string(),
            "INSERT".to_string(),
            "UPDATE".to_string(),
            "DELETE".to_string(),
            "DROP".to_string(),
            "UNION".to_string(),
            "script".to_string(),
            "eval".to_string(),
            "exec".to_string(),
            "system".to_string(),
            "shell".to_string(),
        ];

        Self {
            vulnerability_patterns,
            crypto_patterns,
            auth_patterns,
            injection_patterns,
        }
    }

    /// Analyze security-specific patterns
    pub async fn analyze_security_patterns(
        &self,
        finding: &Finding,
    ) -> Result<(f32, f32, f32, f32, f32, f32, f32, f32)> {
        let content = self.get_file_content(&finding.file).await?;
        let finding_context = self.extract_finding_context(&content, finding.line)?;

        let vulnerability_pattern_match =
            self.detect_vulnerability_patterns(&finding_context, finding);
        let crypto_usage_score = self.analyze_crypto_usage(&finding_context);
        let auth_mechanism_complexity = self.analyze_auth_complexity(&finding_context);
        let injection_risk_score = self.analyze_injection_risk(&finding_context);
        let privilege_escalation_risk = self.analyze_privilege_escalation_risk(&finding_context);
        let data_exposure_risk = self.analyze_data_exposure_risk(&finding_context);
        let network_security_score = self.analyze_network_security(&finding_context);
        let input_validation_quality = self.analyze_input_validation(&finding_context);

        Ok((
            vulnerability_pattern_match,
            crypto_usage_score,
            auth_mechanism_complexity,
            injection_risk_score,
            privilege_escalation_risk,
            data_exposure_risk,
            network_security_score,
            input_validation_quality,
        ))
    }

    async fn get_file_content(&self, file_path: &Path) -> Result<String> {
        tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
    }

    fn extract_finding_context(&self, content: &str, line_number: u32) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let line_idx = (line_number as usize).saturating_sub(1);

        // Extract broader context for security analysis: 10 lines before and after
        let start = line_idx.saturating_sub(10);
        let end = (line_idx + 11).min(lines.len());

        Ok(lines[start..end].join("\n"))
    }

    fn detect_vulnerability_patterns(&self, context: &str, finding: &Finding) -> f32 {
        let context_lower = context.to_lowercase();
        let mut max_score = 0.0;

        for pattern in self.vulnerability_patterns.values() {
            let mut pattern_score = 0.0;
            let mut matches = 0;

            // Check for pattern matches
            for pattern_text in &pattern.patterns {
                if context_lower.contains(&pattern_text.to_lowercase()) {
                    matches += 1;
                    pattern_score += pattern.confidence;
                }
            }

            if matches > 0 {
                // Apply severity multiplier based on finding severity
                let severity_boost = match finding.severity {
                    Severity::Critical => 1.0,
                    Severity::High => 0.8,
                    Severity::Medium => 0.6,
                    Severity::Low => 0.4,
                    Severity::Info => 0.2,
                };

                pattern_score = (pattern_score / pattern.patterns.len() as f32)
                    * pattern.severity_multiplier
                    * severity_boost;

                max_score = max_score.max(pattern_score);
            }
        }

        max_score.min(1.0)
    }

    fn analyze_crypto_usage(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let matches = self
            .crypto_patterns
            .iter()
            .filter(|pattern| context_lower.contains(&pattern.to_lowercase()))
            .count();

        let base_score = matches as f32 / self.crypto_patterns.len() as f32;

        // Boost score for strong crypto indicators
        let strong_crypto_boost = if context_lower.contains("aes")
            || context_lower.contains("rsa")
            || context_lower.contains("sha256")
        {
            0.3
        } else {
            0.0
        };

        // Penalty for weak crypto
        let weak_crypto_penalty = if context_lower.contains("md5")
            || context_lower.contains("sha1")
            || context_lower.contains("des")
        {
            -0.2
        } else {
            0.0
        };

        (base_score + strong_crypto_boost + weak_crypto_penalty).clamp(0.0, 1.0)
    }

    fn analyze_auth_complexity(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let auth_matches = self
            .auth_patterns
            .iter()
            .filter(|pattern| context_lower.contains(&pattern.to_lowercase()))
            .count();

        let base_score = auth_matches as f32 / 5.0; // Normalize to typical auth complexity

        // Boost for modern auth patterns
        let modern_auth_boost = if context_lower.contains("jwt")
            || context_lower.contains("oauth")
            || context_lower.contains("mfa")
        {
            0.2
        } else {
            0.0
        };

        (base_score + modern_auth_boost).min(1.0)
    }

    fn analyze_injection_risk(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let injection_matches = self
            .injection_patterns
            .iter()
            .filter(|pattern| context_lower.contains(&pattern.to_lowercase()))
            .count();

        let base_risk = injection_matches as f32 / 10.0; // Normalize

        // High risk indicators
        let high_risk_boost = if context_lower.contains("user_input")
            || context_lower.contains("request")
            || context_lower.contains("params")
        {
            0.3
        } else {
            0.0
        };

        // Protection indicators (reduce risk)
        let protection_reduction = if context_lower.contains("sanitize")
            || context_lower.contains("validate")
            || context_lower.contains("escape")
        {
            -0.2
        } else {
            0.0
        };

        (base_risk + high_risk_boost + protection_reduction).clamp(0.0, 1.0)
    }

    fn analyze_privilege_escalation_risk(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let risk_indicators = [
            "sudo",
            "admin",
            "root",
            "privilege",
            "escalate",
            "setuid",
            "setgid",
            "chmod",
            "chown",
            "exec",
            "system",
            "shell",
        ];

        let matches = risk_indicators
            .iter()
            .filter(|&indicator| context_lower.contains(indicator))
            .count();

        (matches as f32 / 5.0).min(1.0)
    }

    fn analyze_data_exposure_risk(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let exposure_indicators = [
            "password",
            "secret",
            "token",
            "key",
            "credential",
            "private",
            "ssn",
            "credit_card",
            "email",
            "phone",
            "address",
            "personal",
        ];

        let matches = exposure_indicators
            .iter()
            .filter(|&indicator| context_lower.contains(indicator))
            .count();

        let base_risk = (matches as f32 / 8.0).min(1.0);

        // Check for protection measures
        let protection_present = context_lower.contains("encrypt")
            || context_lower.contains("hash")
            || context_lower.contains("secure");

        if protection_present {
            base_risk * 0.7 // Reduce risk if protection is present
        } else {
            base_risk
        }
    }

    fn analyze_network_security(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let security_indicators = [
            "https",
            "tls",
            "ssl",
            "certificate",
            "encryption",
            "secure",
            "firewall",
            "vpn",
            "tunnel",
        ];

        let insecure_indicators = ["http://", "ftp://", "telnet", "insecure", "plaintext"];

        let secure_matches = security_indicators
            .iter()
            .filter(|&indicator| context_lower.contains(indicator))
            .count();

        let insecure_matches = insecure_indicators
            .iter()
            .filter(|&indicator| context_lower.contains(indicator))
            .count();

        let security_score = secure_matches as f32 / 5.0;
        let insecurity_penalty = insecure_matches as f32 / 3.0;

        (security_score - insecurity_penalty).clamp(0.0, 1.0)
    }

    fn analyze_input_validation(&self, context: &str) -> f32 {
        let context_lower = context.to_lowercase();
        let validation_indicators = [
            "validate",
            "sanitize",
            "escape",
            "filter",
            "whitelist",
            "blacklist",
            "regex",
            "pattern",
            "length",
            "range",
            "type_check",
        ];

        let matches = validation_indicators
            .iter()
            .filter(|&indicator| context_lower.contains(indicator))
            .count();

        let base_score = (matches as f32 / 6.0).min(1.0);

        // Boost for comprehensive validation
        let comprehensive_boost = if matches >= 3 { 0.2 } else { 0.0 };

        (base_score + comprehensive_boost).min(1.0)
    }
}

impl Default for AdvancedFeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}
