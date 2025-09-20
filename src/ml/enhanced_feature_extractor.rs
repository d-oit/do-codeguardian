#[cfg(feature = "ast")]
use crate::ml::ast_analyzer::{AstAnalyzer, AstFeatures};
use crate::ml::feature_extractor::FeatureExtractor;
use crate::types::Finding;
use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};
use std::path::Path;

/// Enhanced feature extractor that combines traditional features with AST analysis
pub struct EnhancedFeatureExtractor {
    base_extractor: FeatureExtractor,
    #[cfg(feature = "ast")]
    ast_analyzer: AstAnalyzer,
    file_cache: HashMap<String, CachedFileAnalysis>,
    cache_order: VecDeque<String>,
}

#[derive(Debug, Clone)]
struct CachedFileAnalysis {
    #[cfg(feature = "ast")]
    ast_features: AstFeatures,
    #[cfg(not(feature = "ast"))]
    _placeholder: (),
    #[allow(dead_code)]
    file_hash: u64,
    timestamp: std::time::SystemTime,
}

impl EnhancedFeatureExtractor {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit
    const MAX_CACHE_SIZE: usize = 1000; // Prevent unbounded cache growth

    pub fn new() -> Self {
        Self {
            base_extractor: FeatureExtractor::new(),
            #[cfg(feature = "ast")]
            ast_analyzer: AstAnalyzer::new(),
            file_cache: HashMap::new(),
            cache_order: VecDeque::new(),
        }
    }

    /// Extract enhanced feature vector combining traditional and AST features
    pub async fn extract_enhanced_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        // Get base features (8 dimensions)
        let mut base_features = self.base_extractor.extract_features(finding)?;

        // Get AST features if available
        #[cfg(feature = "ast")]
        let ast_features = self.get_ast_features_async(&finding.file).await?;

        #[cfg(not(feature = "ast"))]
        let ast_features = vec![0.0; 16]; // Placeholder when AST is disabled

        // Combine features: 8 base + 16 AST = 24 total dimensions
        base_features.extend(ast_features);

        Ok(base_features)
    }

    /// Get AST features for a file, using cache when possible
    #[cfg(feature = "ast")]
    async fn get_ast_features_async(&mut self, file_path: &Path) -> Result<Vec<f32>> {
        // Canonicalize path to resolve symlinks and relative paths
        let canonical_path = file_path
            .canonicalize()
            .context("Failed to canonicalize file path")?;
        let file_path_str = canonical_path.to_string_lossy().to_string();

        // Check if we have cached analysis
        if let Some(cached) = self.file_cache.get(&file_path_str) {
            // Check if cache is still valid (file hasn't changed)
            if let Ok(metadata) = tokio::fs::metadata(&canonical_path).await {
                if let Ok(modified) = metadata.modified() {
                    if modified <= cached.timestamp {
                        return Ok(cached.ast_features.to_feature_vector());
                    }
                }
            }
        }

        // Get file metadata for size validation
        let metadata = tokio::fs::metadata(&canonical_path)
            .await
            .context("Failed to read file metadata")?;

        // Security check: file size limit
        if metadata.len() > Self::MAX_FILE_SIZE {
            return Err(anyhow::anyhow!(
                "File too large: {} bytes (limit: {} bytes)",
                metadata.len(),
                Self::MAX_FILE_SIZE
            ));
        }

        // Read and analyze file
        let content = tokio::fs::read_to_string(&canonical_path)
            .await
            .context("Failed to read file content")?;

        let ast_features = self
            .ast_analyzer
            .extract_ast_features(&canonical_path, &content)?;

        // Cache the analysis with size limit
        let cached_analysis = CachedFileAnalysis {
            ast_features: ast_features.clone(),
            file_hash: self.calculate_file_hash(&content),
            timestamp: std::time::SystemTime::now(),
        };

        // Manage cache size
        if self.file_cache.len() >= Self::MAX_CACHE_SIZE {
            if let Some(oldest_key) = self.cache_order.pop_front() {
                self.file_cache.remove(&oldest_key);
            }
        }

        self.file_cache
            .insert(file_path_str.clone(), cached_analysis);
        self.cache_order.push_back(file_path_str);

        Ok(ast_features.to_feature_vector())
    }

    #[cfg(not(feature = "ast"))]
    fn get_ast_features(&mut self, _file_path: &Path) -> Result<Vec<f32>> {
        // Return zero features when AST analysis is disabled
        Ok(vec![0.0; 16])
    }

    fn calculate_file_hash(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    /// Get feature names for the enhanced feature vector
    pub fn get_feature_names() -> Vec<String> {
        let mut names = vec![
            // Base features (8)
            "severity_score".to_string(),
            "file_type_relevance".to_string(),
            "analyzer_confidence".to_string(),
            "message_length".to_string(),
            "line_position".to_string(),
            "has_description".to_string(),
            "has_suggestion".to_string(),
            "rule_specificity".to_string(),
        ];

        // AST features (16)
        #[cfg(feature = "ast")]
        {
            let ast_names: Vec<String> = AstFeatures::feature_names()
                .iter()
                .map(|name| format!("ast_{}", name))
                .collect();
            names.extend(ast_names);
        }

        #[cfg(not(feature = "ast"))]
        {
            // Placeholder names when AST is disabled
            for i in 0..16 {
                names.push(format!("ast_placeholder_{}", i));
            }
        }

        names
    }

    /// Analyze feature importance for a given finding
    pub async fn analyze_feature_importance(
        &mut self,
        finding: &Finding,
    ) -> Result<FeatureImportanceAnalysis> {
        let features = self.extract_enhanced_features(finding).await?;
        let feature_names = Self::get_feature_names();

        let mut analysis = FeatureImportanceAnalysis {
            total_features: features.len(),
            base_feature_contribution: 0.0,
            ast_feature_contribution: 0.0,
            top_features: Vec::new(),
        };

        // Calculate contributions
        let base_sum: f32 = features[..8].iter().sum();
        let ast_sum: f32 = features[8..].iter().sum();
        let total_sum = base_sum + ast_sum;

        if total_sum > 0.0 {
            analysis.base_feature_contribution = base_sum / total_sum;
            analysis.ast_feature_contribution = ast_sum / total_sum;
        }

        // Find top contributing features
        let mut feature_pairs: Vec<(String, f32)> = feature_names
            .into_iter()
            .zip(features.iter())
            .map(|(name, &value)| (name, value))
            .collect();

        feature_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        analysis.top_features = feature_pairs.into_iter().take(5).collect();

        Ok(analysis)
    }

    /// Clear the file analysis cache
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
        self.cache_order.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            cached_files: self.file_cache.len(),
            cache_size_bytes: self.file_cache.len() * std::mem::size_of::<CachedFileAnalysis>(),
            max_cache_size: Self::MAX_CACHE_SIZE,
        }
    }
}

impl Default for EnhancedFeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FeatureImportanceAnalysis {
    pub total_features: usize,
    pub base_feature_contribution: f32,
    pub ast_feature_contribution: f32,
    pub top_features: Vec<(String, f32)>,
}

#[derive(Debug)]
pub struct CacheStats {
    pub cached_files: usize,
    pub cache_size_bytes: usize,
    pub max_cache_size: usize,
}

impl std::fmt::Display for FeatureImportanceAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Feature Importance Analysis:")?;
        writeln!(f, "  Total features: {}", self.total_features)?;
        writeln!(
            f,
            "  Base feature contribution: {:.1}%",
            self.base_feature_contribution * 100.0
        )?;
        writeln!(
            f,
            "  AST feature contribution: {:.1}%",
            self.ast_feature_contribution * 100.0
        )?;
        writeln!(f, "  Top contributing features:")?;
        for (i, (name, value)) in self.top_features.iter().enumerate() {
            writeln!(f, "    {}. {}: {:.3}", i + 1, name, value)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache: {} files, ~{} KB (max: {})",
            self.cached_files,
            self.cache_size_bytes / 1024,
            self.max_cache_size
        )
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
    fn test_enhanced_feature_extractor_creation() {
        let extractor = EnhancedFeatureExtractor::new();
        assert_eq!(extractor.file_cache.len(), 0);
    }

    #[test]
    fn test_feature_names() {
        let names = EnhancedFeatureExtractor::get_feature_names();
        assert_eq!(names.len(), 24); // 8 base + 16 AST
        assert!(names[0].contains("severity"));
        assert!(names[8].contains("ast"));
    }

    #[test]
    fn test_enhanced_features_extraction() {
        let mut extractor = EnhancedFeatureExtractor::new();

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
            .block_on(async { extractor.extract_enhanced_features(&finding).await.unwrap() });
        assert_eq!(features.len(), 24); // 8 base + 16 AST features
    }

    #[test]
    fn test_feature_importance_analysis() {
        let mut extractor = EnhancedFeatureExtractor::new();

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
        assert_eq!(analysis.total_features, 24);
        assert!(analysis.base_feature_contribution >= 0.0);
        assert!(analysis.ast_feature_contribution >= 0.0);
        assert_eq!(analysis.top_features.len(), 5);
    }

    #[cfg(feature = "ast")]
    #[tokio::test]
    async fn test_file_size_limit() {
        let mut extractor = EnhancedFeatureExtractor::new();

        // Create a temporary file larger than 10MB
        let temp_file = NamedTempFile::new().unwrap();
        let large_content = vec![b'a'; 15 * 1024 * 1024]; // 15MB
        fs::write(&temp_file, &large_content).await.unwrap();

        let result = extractor.get_ast_features_async(temp_file.path()).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("too large"));
        assert!(err_msg.contains("10485760")); // 10MB in bytes
    }

    #[cfg(feature = "ast")]
    #[tokio::test]
    async fn test_valid_file_size() {
        let mut extractor = EnhancedFeatureExtractor::new();

        // Create a temporary file smaller than 10MB
        let temp_file = NamedTempFile::new().unwrap();
        let content = b"fn main() { println!(\"Hello\"); }";
        fs::write(&temp_file, content).await.unwrap();

        let result = extractor.get_ast_features_async(temp_file.path()).await;
        // Should succeed or fail based on AST parsing, but not due to size
        // We just check it's not a size error
        if let Err(e) = &result {
            assert!(!e.to_string().contains("too large"));
        }
    }

    #[cfg(feature = "ast")]
    #[tokio::test]
    async fn test_cache_size_limit() {
        let mut extractor = EnhancedFeatureExtractor::new();

        // Create multiple small files to fill cache
        for i in 0..1100 {
            // More than MAX_CACHE_SIZE
            let temp_file = NamedTempFile::new().unwrap();
            let content = format!("fn func_{}() {{}}", i);
            fs::write(&temp_file, content).await.unwrap();

            let _ = extractor.get_ast_features_async(temp_file.path()).await;
        }

        let stats = extractor.get_cache_stats();
        assert!(stats.cached_files <= EnhancedFeatureExtractor::MAX_CACHE_SIZE);
    }
}
