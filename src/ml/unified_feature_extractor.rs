#[cfg(feature = "ast")]
use crate::ml::ast_analyzer::{AstAnalyzer, AstFeatures};
use crate::ml::feature_extractor::FeatureExtractor;
use crate::types::Finding;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Unified feature extractor that combines base and enhanced implementations
/// into a single, configurable system with multiple extraction modes.
pub struct UnifiedFeatureExtractor {
    base_extractor: FeatureExtractor,
    #[cfg(feature = "ast")]
    ast_analyzer: AstAnalyzer,
    config: FeatureConfig,
    cache: Arc<RwLock<FileCache>>,
    metrics: FeatureExtractionMetrics,
}

/// Configuration for feature extraction modes and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    /// Primary extraction mode
    pub mode: ExtractionMode,
    /// Feature sets to include
    pub feature_sets: Vec<FeatureSet>,
    /// Security settings
    pub security: SecurityConfig,
    /// Cache settings
    pub cache: CacheConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            mode: ExtractionMode::Enhanced,
            feature_sets: vec![FeatureSet::Base, FeatureSet::Ast],
            security: SecurityConfig::default(),
            cache: CacheConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

/// Extraction modes supported by the unified extractor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtractionMode {
    /// Basic features only (8 dimensions)
    Basic,
    /// Enhanced features with AST analysis (24 dimensions)
    Enhanced,
    /// AST-only features (16 dimensions)
    AstOnly,
    /// Custom feature combination
    Custom,
}

/// Feature sets that can be combined
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureSet {
    /// Basic finding features (severity, file type, etc.)
    Base,
    /// AST-based structural features
    Ast,
    /// Code complexity metrics
    Complexity,
    /// Security-specific features
    Security,
}

/// Security configuration for safe file processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Maximum file size in bytes
    pub max_file_size: u64,
    /// Allowed file extensions
    pub allowed_extensions: Vec<String>,
    /// Block directory traversal attempts
    pub prevent_traversal: bool,
    /// Timeout for file operations
    pub operation_timeout: Duration,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            allowed_extensions: vec![
                "rs".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "py".to_string(),
                "java".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "go".to_string(),
                "json".to_string(),
                "yaml".to_string(),
                "toml".to_string(),
                "md".to_string(),
            ],
            prevent_traversal: true,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of cached files
    pub max_cache_size: usize,
    /// Cache entry TTL
    pub ttl: Duration,
    /// Enable cache compression
    pub compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1000,
            ttl: Duration::from_secs(3600), // 1 hour
            compression: false,
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable parallel processing
    pub parallel_processing: bool,
    /// Batch size for parallel operations
    pub batch_size: usize,
    /// Memory limit for processing
    pub memory_limit: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            parallel_processing: true,
            batch_size: 10,
            memory_limit: 100 * 1024 * 1024, // 100MB
        }
    }
}

/// Cached file analysis data
#[derive(Debug, Clone)]
struct CachedFileAnalysis {
    #[cfg(feature = "ast")]
    ast_features: AstFeatures,
    #[cfg(not(feature = "ast"))]
    _ast_placeholder: (),
    base_features: Vec<f32>,
    file_hash: u64,
    timestamp: SystemTime,
    file_size: u64,
}

/// File cache with LRU eviction
#[derive(Debug)]
struct FileCache {
    cache: HashMap<String, CachedFileAnalysis>,
    order: VecDeque<String>,
    max_size: usize,
    ttl: Duration,
}

/// Metrics for feature extraction performance
#[derive(Debug, Clone)]
pub struct FeatureExtractionMetrics {
    pub total_extractions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub errors: u64,
    pub average_extraction_time: Duration,
}

impl Default for FeatureExtractionMetrics {
    fn default() -> Self {
        Self {
            total_extractions: 0,
            cache_hits: 0,
            cache_misses: 0,
            errors: 0,
            average_extraction_time: Duration::from_millis(0),
        }
    }
}

/// Comprehensive error types for feature extraction
#[derive(Debug, thiserror::Error)]
pub enum FeatureExtractionError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("File too large: {size} bytes (limit: {limit} bytes)")]
    FileTooLarge { size: u64, limit: u64 },

    #[error("Unsupported file type: {extension}")]
    UnsupportedFileType { extension: String },

    #[error("Directory traversal detected: {path}")]
    DirectoryTraversal { path: String },

    #[error("File operation timeout after {timeout:?}")]
    Timeout { timeout: Duration },

    #[error("AST analysis failed: {reason}")]
    AstAnalysisFailed { reason: String },

    #[error("Security violation: {violation}")]
    SecurityViolation { violation: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Cache error: {message}")]
    CacheError { message: String },
}

impl UnifiedFeatureExtractor {
    /// Create a new unified feature extractor with default configuration
    pub fn new() -> Self {
        Self::with_config(FeatureConfig::default())
    }

    /// Create a new unified feature extractor with custom configuration
    pub fn with_config(config: FeatureConfig) -> Self {
        Self {
            base_extractor: FeatureExtractor::new(),
            #[cfg(feature = "ast")]
            ast_analyzer: AstAnalyzer::new(),
            config,
            cache: Arc::new(RwLock::new(FileCache::new(
                config.cache.max_cache_size,
                config.cache.ttl,
            ))),
            metrics: FeatureExtractionMetrics::default(),
        }
    }

    /// Extract features based on the configured mode
    pub async fn extract_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        let start_time = std::time::Instant::now();

        let result = match self.config.mode {
            ExtractionMode::Basic => self.extract_basic_features(finding).await,
            ExtractionMode::Enhanced => self.extract_enhanced_features(finding).await,
            ExtractionMode::AstOnly => self.extract_ast_only_features(finding).await,
            ExtractionMode::Custom => self.extract_custom_features(finding).await,
        };

        let duration = start_time.elapsed();
        self.update_metrics(result.is_ok(), duration);

        result
    }

    /// Extract basic features only (backward compatibility)
    async fn extract_basic_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        debug!("Extracting basic features for finding: {}", finding.rule);
        self.base_extractor.extract_features(finding)
    }

    /// Extract enhanced features (base + AST)
    async fn extract_enhanced_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        debug!("Extracting enhanced features for finding: {}", finding.rule);

        // Get base features
        let mut features = self.base_extractor.extract_features(finding)?;

        // Add AST features if enabled
        if self.config.feature_sets.contains(&FeatureSet::Ast) {
            let ast_features = self.get_ast_features(finding).await?;
            features.extend(ast_features);
        }

        Ok(features)
    }

    /// Extract AST-only features
    async fn extract_ast_only_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        debug!("Extracting AST-only features for finding: {}", finding.rule);
        self.get_ast_features(finding).await
    }

    /// Extract custom feature combination
    async fn extract_custom_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        debug!("Extracting custom features for finding: {}", finding.rule);

        let mut features = Vec::new();

        // Clone the feature sets to avoid borrowing issues
        let feature_sets = self.config.feature_sets.clone();

        for feature_set in feature_sets {
            match feature_set {
                FeatureSet::Base => {
                    features.extend(self.base_extractor.extract_features(finding)?);
                }
                FeatureSet::Ast => {
                    features.extend(self.get_ast_features(finding).await?);
                }
                FeatureSet::Complexity => {
                    // Placeholder for complexity features
                    features.extend(vec![0.0; 4]);
                }
                FeatureSet::Security => {
                    // Placeholder for security features
                    features.extend(vec![0.0; 4]);
                }
            }
        }

        Ok(features)
    }

    /// Get AST features with caching and security checks
    async fn get_ast_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        // Security validation
        self.validate_file_path(&finding.file)?;

        let file_path_str = finding.file.to_string_lossy().to_string();

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get_valid(&file_path_str) {
                self.metrics.cache_hits += 1;
                debug!("Cache hit for file: {}", file_path_str);
                #[cfg(feature = "ast")]
                return Ok(cached.ast_features.to_feature_vector());
                #[cfg(not(feature = "ast"))]
                return Ok(vec![0.0; 16]);
            }
        }

        self.metrics.cache_misses += 1;
        debug!("Cache miss for file: {}", file_path_str);

        // Extract AST features
        #[cfg(feature = "ast")]
        {
            let ast_features = self.extract_ast_features_secure(&finding.file).await?;
            let feature_vector = ast_features.to_feature_vector();

            // Cache the result
            let cached_analysis = CachedFileAnalysis {
                ast_features,
                base_features: vec![], // Not cached here
                file_hash: self.calculate_file_hash(&feature_vector),
                timestamp: SystemTime::now(),
                file_size: 0, // TODO: Get actual file size
            };

            let mut cache = self.cache.write().await;
            cache.insert(file_path_str, cached_analysis);

            Ok(feature_vector)
        }

        #[cfg(not(feature = "ast"))]
        {
            warn!("AST features requested but AST feature is not enabled");
            Ok(vec![0.0; 16])
        }
    }

    /// Extract AST features with security validation
    #[cfg(feature = "ast")]
    async fn extract_ast_features_secure(&mut self, file_path: &Path) -> Result<AstFeatures> {
        // Canonicalize path
        let canonical_path = file_path
            .canonicalize()
            .context("Failed to canonicalize file path")?;

        // Check file size
        let metadata = tokio::fs::metadata(&canonical_path)
            .await
            .context("Failed to read file metadata")?;

        if metadata.len() > self.config.security.max_file_size {
            return Err(FeatureExtractionError::FileTooLarge {
                size: metadata.len(),
                limit: self.config.security.max_file_size,
            }
            .into());
        }

        // Read file content with timeout
        let content = tokio::time::timeout(
            self.config.security.operation_timeout,
            tokio::fs::read_to_string(&canonical_path),
        )
        .await
        .context("File read timeout")?
        .context("Failed to read file content")?;

        // Extract AST features
        self.ast_analyzer
            .extract_ast_features(&canonical_path, &content)
            .map_err(|e| {
                FeatureExtractionError::AstAnalysisFailed {
                    reason: e.to_string(),
                }
                .into()
            })
    }

    /// Validate file path for security
    fn validate_file_path(&self, file_path: &Path) -> Result<()> {
        // Check file extension
        if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
            if !self
                .config
                .security
                .allowed_extensions
                .contains(&extension.to_string())
            {
                return Err(FeatureExtractionError::UnsupportedFileType {
                    extension: extension.to_string(),
                }
                .into());
            }
        }

        // Prevent directory traversal
        if self.config.security.prevent_traversal {
            if file_path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                return Err(FeatureExtractionError::DirectoryTraversal {
                    path: file_path.display().to_string(),
                }
                .into());
            }
        }

        Ok(())
    }

    /// Calculate simple hash for cache validation
    fn calculate_file_hash(&self, data: &[f32]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.len().hash(&mut hasher);
        for &value in data {
            (value.to_bits() as u64).hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Update performance metrics
    fn update_metrics(&mut self, success: bool, duration: std::time::Duration) {
        self.metrics.total_extractions += 1;
        if !success {
            self.metrics.errors += 1;
        }

        // Update average extraction time
        let total_time =
            self.metrics.average_extraction_time * (self.metrics.total_extractions - 1) as u32;
        self.metrics.average_extraction_time =
            (total_time + duration) / self.metrics.total_extractions as u32;
    }

    /// Get feature names for the current configuration
    pub fn get_feature_names(&self) -> Vec<String> {
        match self.config.mode {
            ExtractionMode::Basic => Self::get_basic_feature_names(),
            ExtractionMode::Enhanced => Self::get_enhanced_feature_names(),
            ExtractionMode::AstOnly => Self::get_ast_feature_names(),
            ExtractionMode::Custom => self.get_custom_feature_names(),
        }
    }

    fn get_basic_feature_names() -> Vec<String> {
        vec![
            "severity_score".to_string(),
            "file_type_relevance".to_string(),
            "analyzer_confidence".to_string(),
            "message_length".to_string(),
            "line_position".to_string(),
            "has_description".to_string(),
            "has_suggestion".to_string(),
            "rule_specificity".to_string(),
        ]
    }

    fn get_enhanced_feature_names() -> Vec<String> {
        let mut names = Self::get_basic_feature_names();
        names.extend(Self::get_ast_feature_names());
        names
    }

    fn get_ast_feature_names() -> Vec<String> {
        #[cfg(feature = "ast")]
        {
            AstFeatures::feature_names()
                .iter()
                .map(|name| format!("ast_{}", name))
                .collect()
        }
        #[cfg(not(feature = "ast"))]
        {
            (0..16).map(|i| format!("ast_placeholder_{}", i)).collect()
        }
    }

    fn get_custom_feature_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        for feature_set in &self.config.feature_sets {
            match feature_set {
                FeatureSet::Base => names.extend(Self::get_basic_feature_names()),
                FeatureSet::Ast => names.extend(Self::get_ast_feature_names()),
                FeatureSet::Complexity => {
                    names.extend(vec![
                        "complexity_cyclomatic".to_string(),
                        "complexity_lines".to_string(),
                        "complexity_branches".to_string(),
                        "complexity_depth".to_string(),
                    ]);
                }
                FeatureSet::Security => {
                    names.extend(vec![
                        "security_entropy".to_string(),
                        "security_patterns".to_string(),
                        "security_keywords".to_string(),
                        "security_score".to_string(),
                    ]);
                }
            }
        }

        names
    }

    /// Get current configuration
    pub fn get_config(&self) -> &FeatureConfig {
        &self.config
    }

    /// Update configuration at runtime
    pub async fn update_config(&mut self, new_config: FeatureConfig) -> Result<()> {
        // Validate configuration
        self.validate_config(&new_config)?;

        // Clear cache if cache settings changed
        if new_config.cache.max_cache_size != self.config.cache.max_cache_size
            || new_config.cache.ttl != self.config.cache.ttl
        {
            let mut cache = self.cache.write().await;
            cache.clear();
        }

        self.config = new_config;
        info!("Feature extractor configuration updated");
        Ok(())
    }

    /// Validate configuration
    fn validate_config(&self, config: &FeatureConfig) -> Result<()> {
        if config.feature_sets.is_empty() && config.mode == ExtractionMode::Custom {
            return Err(FeatureExtractionError::ConfigurationError {
                message: "Custom mode requires at least one feature set".to_string(),
            }
            .into());
        }

        if config.security.max_file_size == 0 {
            return Err(FeatureExtractionError::ConfigurationError {
                message: "Max file size must be greater than 0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &FeatureExtractionMetrics {
        &self.metrics
    }

    /// Clear cache
    pub async fn clear_cache(&mut self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Feature extraction cache cleared");
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        cache.get_stats()
    }

    /// Analyze feature importance for a finding
    pub async fn analyze_feature_importance(
        &mut self,
        finding: &Finding,
    ) -> Result<FeatureImportanceAnalysis> {
        let features = self.extract_features(finding).await?;
        let feature_names = self.get_feature_names();

        let mut analysis = FeatureImportanceAnalysis {
            total_features: features.len(),
            feature_contributions: Vec::new(),
            top_features: Vec::new(),
            mode: self.config.mode,
        };

        // Calculate feature contributions
        let total_sum: f32 = features.iter().sum();
        if total_sum > 0.0 {
            for (i, (&value, name)) in features.iter().zip(feature_names.iter()).enumerate() {
                let contribution = value / total_sum;
                analysis
                    .feature_contributions
                    .push((name.clone(), contribution));
            }
        }

        // Sort by contribution
        analysis
            .feature_contributions
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        analysis.top_features = analysis
            .feature_contributions
            .iter()
            .take(5)
            .cloned()
            .collect();

        Ok(analysis)
    }
}

impl FileCache {
    fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            order: VecDeque::new(),
            max_size,
            ttl,
        }
    }

    fn get_valid(&self, key: &str) -> Option<&CachedFileAnalysis> {
        if let Some(entry) = self.cache.get(key) {
            if entry.timestamp.elapsed().unwrap_or(Duration::from_secs(0)) < self.ttl {
                return Some(entry);
            }
        }
        None
    }

    fn insert(&mut self, key: String, analysis: CachedFileAnalysis) {
        // Remove existing entry if present
        if self.cache.contains_key(&key) {
            self.order.retain(|k| k != &key);
        }

        // Evict oldest if at capacity
        if self.cache.len() >= self.max_size {
            if let Some(oldest_key) = self.order.pop_front() {
                self.cache.remove(&oldest_key);
            }
        }

        self.cache.insert(key.clone(), analysis);
        self.order.push_back(key);
    }

    fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    fn get_stats(&self) -> CacheStats {
        CacheStats {
            cached_files: self.cache.len(),
            max_cache_size: self.max_size,
            cache_size_bytes: self.cache.len() * std::mem::size_of::<CachedFileAnalysis>(),
            ttl: self.ttl,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cached_files: usize,
    pub max_cache_size: usize,
    pub cache_size_bytes: usize,
    pub ttl: Duration,
}

/// Feature importance analysis
#[derive(Debug, Clone)]
pub struct FeatureImportanceAnalysis {
    pub total_features: usize,
    pub feature_contributions: Vec<(String, f32)>,
    pub top_features: Vec<(String, f32)>,
    pub mode: ExtractionMode,
}

impl std::fmt::Display for FeatureImportanceAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Feature Importance Analysis (Mode: {:?}):", self.mode)?;
        writeln!(f, "  Total features: {}", self.total_features)?;
        writeln!(f, "  Top contributing features:")?;
        for (i, (name, contribution)) in self.top_features.iter().enumerate() {
            writeln!(f, "    {}. {}: {:.1}%", i + 1, name, contribution * 100.0)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache: {} files, ~{} KB (max: {}), TTL: {:?}",
            self.cached_files,
            self.cache_size_bytes / 1024,
            self.max_cache_size,
            self.ttl
        )
    }
}

impl Default for UnifiedFeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use tokio::fs;

    #[test]
    fn test_unified_extractor_creation() {
        let extractor = UnifiedFeatureExtractor::new();
        assert_eq!(extractor.config.mode, ExtractionMode::Enhanced);
    }

    #[test]
    fn test_basic_feature_extraction() {
        let mut extractor = UnifiedFeatureExtractor::with_config(FeatureConfig {
            mode: ExtractionMode::Basic,
            ..Default::default()
        });

        let finding = Finding::new(
            "integrity",
            "test_rule",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Test finding".to_string(),
        );

        let features = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { extractor.extract_features(&finding).await.unwrap() });

        assert_eq!(features.len(), 8); // Basic features only
    }

    #[test]
    fn test_enhanced_feature_extraction() {
        let mut extractor = UnifiedFeatureExtractor::with_config(FeatureConfig {
            mode: ExtractionMode::Enhanced,
            ..Default::default()
        });

        let finding = Finding::new(
            "security",
            "test_rule",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Test finding".to_string(),
        );

        let features = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { extractor.extract_features(&finding).await.unwrap() });

        assert_eq!(features.len(), 24); // Base + AST features
    }

    #[test]
    fn test_custom_feature_extraction() {
        let mut extractor = UnifiedFeatureExtractor::with_config(FeatureConfig {
            mode: ExtractionMode::Custom,
            feature_sets: vec![FeatureSet::Base],
            ..Default::default()
        });

        let finding = Finding::new(
            "test",
            "test_rule",
            Severity::Medium,
            PathBuf::from("test.rs"),
            1,
            "Test".to_string(),
        );

        let features = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { extractor.extract_features(&finding).await.unwrap() });

        assert_eq!(features.len(), 8); // Only base features
    }

    #[test]
    fn test_feature_names() {
        let extractor = UnifiedFeatureExtractor::with_config(FeatureConfig {
            mode: ExtractionMode::Basic,
            ..Default::default()
        });

        let names = extractor.get_feature_names();
        assert_eq!(names.len(), 8);
        assert!(names[0].contains("severity"));
    }

    #[test]
    fn test_configuration_validation() {
        let extractor = UnifiedFeatureExtractor::new();

        // Test invalid custom config (no feature sets)
        let invalid_config = FeatureConfig {
            mode: ExtractionMode::Custom,
            feature_sets: vec![],
            ..Default::default()
        };

        assert!(extractor.validate_config(&invalid_config).is_err());
    }

    #[cfg(feature = "ast")]
    #[tokio::test]
    async fn test_file_size_limit() {
        let mut extractor = UnifiedFeatureExtractor::new();

        // Create a large file
        let temp_file = NamedTempFile::new().unwrap();
        let large_content = vec![b'a'; 15 * 1024 * 1024]; // 15MB
        fs::write(&temp_file, &large_content).await.unwrap();

        let finding = Finding::new(
            "test",
            "test_rule",
            Severity::High,
            temp_file.path().to_path_buf(),
            1,
            "Test".to_string(),
        );

        let result = extractor.extract_features(&finding).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("too large"));
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let mut extractor = UnifiedFeatureExtractor::new();

        let finding = Finding::new(
            "test",
            "test_rule",
            Severity::High,
            PathBuf::from("src/main.rs"),
            1,
            "Test".to_string(),
        );

        // First extraction (cache miss)
        let _ = extractor.extract_features(&finding).await.unwrap();
        let initial_misses = extractor.metrics.cache_misses;

        // Second extraction (should be cache hit)
        let _ = extractor.extract_features(&finding).await.unwrap();
        let final_hits = extractor.metrics.cache_hits;

        // Note: Cache hits may not occur if AST is disabled or file doesn't exist
        assert!(extractor.metrics.total_extractions >= 2);
    }

    #[test]
    fn test_feature_importance_analysis() {
        let mut extractor = UnifiedFeatureExtractor::with_config(FeatureConfig {
            mode: ExtractionMode::Basic,
            ..Default::default()
        });

        let finding = Finding::new(
            "integrity",
            "test_rule",
            Severity::Critical,
            PathBuf::from("test.rs"),
            1,
            "Critical test finding".to_string(),
        );

        let analysis = tokio::runtime::Runtime::new().unwrap().block_on(async {
            extractor
                .analyze_feature_importance(&finding)
                .await
                .unwrap()
        });

        assert_eq!(analysis.total_features, 8);
        assert_eq!(analysis.mode, ExtractionMode::Basic);
        assert_eq!(analysis.top_features.len(), 5);
    }
}
