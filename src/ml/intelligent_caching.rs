//! Intelligent Caching and Optimization System for CodeGuardian ML
//!
//! This module provides smart caching strategies, prediction optimization,
//! and resource management for machine learning operations.

use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Intelligent caching system for ML predictions and features
pub struct IntelligentCache {
    /// Prediction cache with smart eviction
    prediction_cache: Arc<RwLock<PredictionCache>>,
    /// Feature cache with deduplication
    feature_cache: Arc<RwLock<FeatureCache>>,
    /// Model cache for multiple models
    model_cache: Arc<RwLock<ModelCache>>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache analytics and metrics
    analytics: Arc<RwLock<CacheAnalytics>>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum prediction cache size
    pub max_prediction_cache_size: usize,
    /// Maximum feature cache size
    pub max_feature_cache_size: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Enable intelligent eviction
    pub intelligent_eviction: bool,
    /// Enable compression
    pub compression_enabled: bool,
    /// Cache hit ratio threshold for optimization
    pub hit_ratio_threshold: f64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_prediction_cache_size: 10000,
            max_feature_cache_size: 50000,
            cache_ttl_seconds: 3600, // 1 hour
            intelligent_eviction: true,
            compression_enabled: true,
            hit_ratio_threshold: 0.8,
        }
    }
}

/// Prediction cache with intelligent management
pub struct PredictionCache {
    /// Cached predictions
    predictions: HashMap<FindingKey, CachedPrediction>,
    /// Access frequency tracking
    access_frequency: HashMap<FindingKey, AccessInfo>,
    /// LRU order
    lru_order: VecDeque<FindingKey>,
    /// Cache configuration
    config: CacheConfig,
}

/// Feature cache with deduplication
pub struct FeatureCache {
    /// Cached features
    features: HashMap<FindingKey, CachedFeatures>,
    /// Feature similarity index for deduplication
    similarity_index: HashMap<u64, Vec<FindingKey>>,
    /// Access patterns
    access_patterns: HashMap<FindingKey, AccessPattern>,
}

/// Model cache for multiple ML models
pub struct ModelCache {
    /// Cached models
    models: HashMap<String, CachedModel>,
    /// Model performance metrics
    performance_metrics: HashMap<String, ModelPerformanceCache>,
    /// Model loading times
    loading_times: HashMap<String, std::time::Duration>,
}

/// Finding key for caching
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FindingKey {
    /// File path hash
    pub file_hash: u64,
    /// Rule identifier
    pub rule: String,
    /// Content hash (for content-based caching)
    pub content_hash: u64,
    /// Severity level
    pub severity: Severity,
}

/// Cached prediction with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPrediction {
    /// Prediction value
    pub prediction: f32,
    /// Confidence score
    pub confidence: f32,
    /// Timestamp when cached
    pub cached_at: chrono::DateTime<chrono::Utc>,
    /// Number of times accessed
    pub access_count: usize,
    /// Last access time
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    /// Cache hit score (for intelligent eviction)
    pub cache_score: f64,
}

/// Cached features with compression
#[derive(Debug, Clone)]
pub struct CachedFeatures {
    /// Feature vector (potentially compressed)
    pub features: Vec<f32>,
    /// Feature names
    pub feature_names: Vec<String>,
    /// Compression metadata
    pub compression_info: CompressionInfo,
    /// Cache metadata
    pub cache_metadata: CacheMetadata,
}

/// Cached model information
#[derive(Debug, Clone)]
pub struct CachedModel {
    /// Model identifier
    pub model_id: String,
    /// Model data (serialized)
    pub model_data: Vec<u8>,
    /// Model metadata
    pub metadata: ModelMetadata,
    /// Last used timestamp
    pub last_used: chrono::DateTime<chrono::Utc>,
}

/// Access information for cache optimization
#[derive(Debug, Clone)]
pub struct AccessInfo {
    /// Total access count
    pub count: usize,
    /// Access frequency (accesses per hour)
    pub frequency: f64,
    /// Last access time
    pub last_access: chrono::DateTime<chrono::Utc>,
    /// Access pattern (sequential, random, burst)
    pub pattern: AccessPattern,
}

/// Access patterns for intelligent caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessPattern {
    /// Sequential access (file by file)
    Sequential,
    /// Random access
    Random,
    /// Burst access (many requests in short time)
    Burst,
    /// Periodic access
    Periodic { interval_minutes: u64 },
}

/// Compression information
#[derive(Debug, Clone)]
pub struct CompressionInfo {
    /// Whether features are compressed
    pub is_compressed: bool,
    /// Original size in bytes
    pub original_size: usize,
    /// Compressed size in bytes
    pub compressed_size: usize,
    /// Compression ratio
    pub compression_ratio: f64,
}

/// Cache metadata
#[derive(Debug, Clone)]
pub struct CacheMetadata {
    /// When the item was cached
    pub cached_at: chrono::DateTime<chrono::Utc>,
    /// Cache priority (higher = more important)
    pub priority: u8,
    /// Expiration time
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Cache tags for categorization
    pub tags: Vec<String>,
}

/// Model metadata
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    /// Model version
    pub version: String,
    /// Model type
    pub model_type: String,
    /// Training date
    pub trained_at: chrono::DateTime<chrono::Utc>,
    /// Model size in bytes
    pub size_bytes: usize,
    /// Performance metrics
    pub performance: ModelPerformanceCache,
}

/// Model performance cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceCache {
    /// Accuracy
    pub accuracy: f64,
    /// Precision
    pub precision: f64,
    /// Recall
    pub recall: f64,
    /// F1 score
    pub f1_score: f64,
    /// Average prediction time
    pub avg_prediction_time: std::time::Duration,
}

/// Cache analytics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAnalytics {
    /// Cache hit rates
    pub hit_rates: CacheHitRates,
    /// Cache performance metrics
    pub performance_metrics: CachePerformanceMetrics,
    /// Cache usage statistics
    pub usage_statistics: CacheUsageStatistics,
    /// Cache optimization suggestions
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
}

/// Cache hit rates by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHitRates {
    /// Overall hit rate
    pub overall: f64,
    /// Prediction cache hit rate
    pub predictions: f64,
    /// Feature cache hit rate
    pub features: f64,
    /// Model cache hit rate
    pub models: f64,
    /// Hit rate by severity
    pub by_severity: HashMap<Severity, f64>,
    /// Hit rate by analyzer
    pub by_analyzer: HashMap<String, f64>,
}

/// Cache performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceMetrics {
    /// Average cache lookup time
    pub avg_lookup_time: std::time::Duration,
    /// Average cache write time
    pub avg_write_time: std::time::Duration,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Cache efficiency score
    pub efficiency_score: f64,
    /// Eviction rate
    pub eviction_rate: f64,
}

/// Cache usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheUsageStatistics {
    /// Total cache requests
    pub total_requests: usize,
    /// Total cache hits
    pub total_hits: usize,
    /// Total cache misses
    pub total_misses: usize,
    /// Total evictions
    pub total_evictions: usize,
    /// Cache size utilization
    pub size_utilization: f64,
    /// Popular cache entries
    pub popular_entries: Vec<PopularCacheEntry>,
}

/// Popular cache entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularCacheEntry {
    /// Entry identifier
    pub key: String,
    /// Access count
    pub access_count: usize,
    /// Hit ratio for this entry
    pub hit_ratio: f64,
    /// Last accessed
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Cache optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// Suggestion type
    pub suggestion_type: OptimizationType,
    /// Description
    pub description: String,
    /// Expected impact
    pub expected_impact: f64,
    /// Implementation priority
    pub priority: Priority,
}

/// Types of cache optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Increase cache size
    IncreaseSize,
    /// Adjust TTL
    AdjustTTL,
    /// Enable compression
    EnableCompression,
    /// Change eviction strategy
    ChangeEvictionStrategy,
    /// Add prefetching
    AddPrefetching,
    /// Partition cache
    PartitionCache,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl IntelligentCache {
    /// Create a new intelligent cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            prediction_cache: Arc::new(RwLock::new(PredictionCache::new(config.clone()))),
            feature_cache: Arc::new(RwLock::new(FeatureCache::new())),
            model_cache: Arc::new(RwLock::new(ModelCache::new())),
            config,
            analytics: Arc::new(RwLock::new(CacheAnalytics::default())),
        }
    }

    /// Get prediction from cache or compute if not available
    pub async fn get_or_compute_prediction<F, Fut>(
        &self,
        finding: &Finding,
        compute_fn: F,
    ) -> Result<(f32, bool)>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<f32>>,
    {
        let key = self.generate_finding_key(finding);

        // Try to get from cache first
        {
            let mut cache = self.prediction_cache.write().await;
            if let Some(cached) = cache.get(&key) {
                self.update_analytics_hit().await;
                return Ok((cached.prediction, true)); // Cache hit
            }
        }

        // Cache miss - compute prediction
        let prediction = compute_fn().await?;

        // Store in cache
        {
            let mut cache = self.prediction_cache.write().await;
            cache.insert(key, prediction);
        }

        self.update_analytics_miss().await;
        Ok((prediction, false)) // Cache miss
    }

    /// Get features from cache or compute if not available
    pub async fn get_or_compute_features<F, Fut>(
        &self,
        finding: &Finding,
        compute_fn: F,
    ) -> Result<(Vec<f32>, bool)>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Vec<f32>>>,
    {
        let key = self.generate_finding_key(finding);

        // Try to get from cache first
        {
            let cache = self.feature_cache.read().await;
            if let Some(cached) = cache.get(&key) {
                self.update_analytics_hit().await;
                return Ok((cached.features.clone(), true)); // Cache hit
            }
        }

        // Cache miss - compute features
        let features = compute_fn().await?;

        // Store in cache
        {
            let mut cache = self.feature_cache.write().await;
            cache.insert(key, features.clone())?;
        }

        self.update_analytics_miss().await;
        Ok((features, false)) // Cache miss
    }

    /// Generate a cache key for a finding
    fn generate_finding_key(&self, finding: &Finding) -> FindingKey {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        finding.file.hash(&mut hasher);
        let file_hash = hasher.finish();

        let mut content_hasher = std::collections::hash_map::DefaultHasher::new();
        finding.message.hash(&mut content_hasher);
        finding.rule.hash(&mut content_hasher);
        let content_hash = content_hasher.finish();

        FindingKey {
            file_hash,
            rule: finding.rule.clone(),
            content_hash,
            severity: finding.severity.clone(),
        }
    }

    /// Update analytics for cache hit
    async fn update_analytics_hit(&self) {
        let mut analytics = self.analytics.write().await;
        analytics.usage_statistics.total_requests += 1;
        analytics.usage_statistics.total_hits += 1;

        // Update hit rates
        let total = analytics.usage_statistics.total_requests as f64;
        let hits = analytics.usage_statistics.total_hits as f64;
        analytics.hit_rates.overall = hits / total;
    }

    /// Update analytics for cache miss
    async fn update_analytics_miss(&self) {
        let mut analytics = self.analytics.write().await;
        analytics.usage_statistics.total_requests += 1;
        analytics.usage_statistics.total_misses += 1;

        // Update hit rates
        let total = analytics.usage_statistics.total_requests as f64;
        let hits = analytics.usage_statistics.total_hits as f64;
        analytics.hit_rates.overall = hits / total;
    }

    /// Get cache analytics
    pub async fn get_analytics(&self) -> CacheAnalytics {
        let analytics = self.analytics.read().await;
        analytics.clone()
    }

    /// Optimize cache based on usage patterns
    pub async fn optimize_cache(&self) -> Result<Vec<OptimizationSuggestion>> {
        let analytics = self.get_analytics().await;
        let mut suggestions = Vec::new();

        // Analyze hit rates and suggest optimizations
        if analytics.hit_rates.overall < self.config.hit_ratio_threshold {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: OptimizationType::IncreaseSize,
                description: format!(
                    "Cache hit rate ({:.1}%) is below threshold ({:.1}%). Consider increasing cache size.",
                    analytics.hit_rates.overall * 100.0,
                    self.config.hit_ratio_threshold * 100.0
                ),
                expected_impact: 0.15,
                priority: Priority::High,
            });
        }

        // Check if compression would be beneficial
        if !self.config.compression_enabled
            && analytics.performance_metrics.memory_usage > 100_000_000
        {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: OptimizationType::EnableCompression,
                description:
                    "High memory usage detected. Enable compression to reduce memory footprint."
                        .to_string(),
                expected_impact: 0.3,
                priority: Priority::Medium,
            });
        }

        // Update analytics with suggestions
        {
            let mut analytics_mut = self.analytics.write().await;
            analytics_mut.optimization_suggestions = suggestions.clone();
        }

        Ok(suggestions)
    }

    /// Clear expired cache entries
    pub async fn cleanup_expired_entries(&self) -> Result<usize> {
        let now = chrono::Utc::now();
        let mut cleaned_count = 0;

        // Clean prediction cache
        {
            let mut cache = self.prediction_cache.write().await;
            cleaned_count += cache.cleanup_expired(now);
        }

        // Clean feature cache
        {
            let mut cache = self.feature_cache.write().await;
            cleaned_count += cache.cleanup_expired(now);
        }

        info!("Cleaned up {} expired cache entries", cleaned_count);
        Ok(cleaned_count)
    }
}

impl PredictionCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            predictions: HashMap::new(),
            access_frequency: HashMap::new(),
            lru_order: VecDeque::new(),
            config,
        }
    }

    pub fn get(&mut self, key: &FindingKey) -> Option<&CachedPrediction> {
        if let Some(prediction) = self.predictions.get_mut(key) {
            // Update access information
            prediction.access_count += 1;
            prediction.last_accessed = chrono::Utc::now();

            // Update LRU order
            self.lru_order.retain(|k| k != key);
            self.lru_order.push_back(key.clone());

            // Update access frequency
            if let Some(access_info) = self.access_frequency.get_mut(key) {
                access_info.count += 1;
                access_info.last_access = chrono::Utc::now();
                access_info.frequency = self.calculate_frequency(access_info);
            }

            Some(prediction)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: FindingKey, prediction: f32) {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::seconds(self.config.cache_ttl_seconds as i64);

        let cached_prediction = CachedPrediction {
            prediction,
            confidence: 0.8, // Would be computed based on model uncertainty
            cached_at: now,
            access_count: 1,
            last_accessed: now,
            cache_score: 1.0,
        };

        // Check if cache is full and evict if needed
        if self.predictions.len() >= self.config.max_prediction_cache_size {
            self.evict_entries(1);
        }

        self.predictions.insert(key.clone(), cached_prediction);
        self.lru_order.push_back(key.clone());

        // Initialize access frequency tracking
        self.access_frequency.insert(
            key,
            AccessInfo {
                count: 1,
                frequency: 0.0,
                last_access: now,
                pattern: AccessPattern::Random,
            },
        );
    }

    fn evict_entries(&mut self, count: usize) {
        if self.config.intelligent_eviction {
            self.intelligent_eviction(count);
        } else {
            self.lru_eviction(count);
        }
    }

    fn intelligent_eviction(&mut self, count: usize) {
        // Calculate cache scores for all entries
        let mut scored_entries: Vec<(FindingKey, f64)> = self
            .predictions
            .iter()
            .map(|(key, prediction)| {
                let access_info = self.access_frequency.get(key)?;
                let score = self.calculate_cache_score(prediction, access_info);
                (key.clone(), score)
            })
            .collect();

        // Sort by score (ascending - lower scores get evicted first)
        scored_entries.sort_by(|a, b| a.1.partial_cmp(&b.1)?);

        // Evict lowest scoring entries
        for (key, _) in scored_entries.into_iter().take(count) {
            self.predictions.remove(&key);
            self.access_frequency.remove(&key);
            self.lru_order.retain(|k| k != &key);
        }
    }

    fn lru_eviction(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(key) = self.lru_order.pop_front() {
                self.predictions.remove(&key);
                self.access_frequency.remove(&key);
            }
        }
    }

    fn calculate_cache_score(
        &self,
        prediction: &CachedPrediction,
        access_info: &AccessInfo,
    ) -> f64 {
        // Multi-factor scoring: frequency, recency, confidence
        let frequency_score = access_info.frequency.min(10.0) / 10.0; // Normalized to 0-1
        let recency_score = {
            let hours_since_access = chrono::Utc::now()
                .signed_duration_since(prediction.last_accessed)
                .num_hours() as f64;
            (24.0 - hours_since_access.min(24.0)) / 24.0 // More recent = higher score
        };
        let confidence_score = prediction.confidence as f64;

        // Weighted combination
        0.4 * frequency_score + 0.3 * recency_score + 0.3 * confidence_score
    }

    fn calculate_frequency(&self, access_info: &AccessInfo) -> f64 {
        let hours_since_first = chrono::Utc::now()
            .signed_duration_since(access_info.last_access)
            .num_hours() as f64;

        if hours_since_first > 0.0 {
            access_info.count as f64 / hours_since_first
        } else {
            access_info.count as f64
        }
    }

    pub fn cleanup_expired(&mut self, now: chrono::DateTime<chrono::Utc>) -> usize {
        let ttl_duration = chrono::Duration::seconds(self.config.cache_ttl_seconds as i64);
        let mut expired_keys = Vec::new();

        for (key, prediction) in &self.predictions {
            if now.signed_duration_since(prediction.cached_at) > ttl_duration {
                expired_keys.push(key.clone());
            }
        }

        for key in &expired_keys {
            self.predictions.remove(key);
            self.access_frequency.remove(key);
            self.lru_order.retain(|k| k != key);
        }

        expired_keys.len()
    }
}

impl FeatureCache {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
            similarity_index: HashMap::new(),
            access_patterns: HashMap::new(),
        }
    }

    pub fn get(&self, key: &FindingKey) -> Option<&CachedFeatures> {
        self.features.get(key)
    }

    pub fn insert(&mut self, key: FindingKey, features: Vec<f32>) -> Result<()> {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(1); // Default TTL

        // Calculate feature hash for similarity detection
        let feature_hash = self.calculate_feature_hash(&features);

        // Check for similar features (deduplication)
        if let Some(similar_keys) = self.similarity_index.get(&feature_hash) {
            if !similar_keys.is_empty() {
                // Features are very similar to existing cache entry
                debug!("Similar features found, using existing cache entry");
                return Ok(());
            }
        }

        let cached_features = CachedFeatures {
            features: features.clone(),
            feature_names: Vec::new(), // Would be filled with actual names
            compression_info: CompressionInfo {
                is_compressed: false,
                original_size: features.len() * 4, // 4 bytes per f32
                compressed_size: features.len() * 4,
                compression_ratio: 1.0,
            },
            cache_metadata: CacheMetadata {
                cached_at: now,
                priority: 5, // Medium priority
                expires_at,
                tags: vec!["features".to_string()],
            },
        };

        self.features.insert(key.clone(), cached_features);

        // Update similarity index
        self.similarity_index
            .entry(feature_hash)
            .or_insert_with(Vec::new)
            .push(key.clone());

        // Initialize access pattern
        self.access_patterns.insert(key, AccessPattern::Random);

        Ok(())
    }

    fn calculate_feature_hash(&self, features: &[f32]) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        // Hash first few features and length for quick similarity check
        features.len().hash(&mut hasher);
        for &feature in features.iter().take(10) {
            ((feature * 1000.0) as i32).hash(&mut hasher); // Discretize for similarity
        }

        hasher.finish()
    }

    pub fn cleanup_expired(&mut self, now: chrono::DateTime<chrono::Utc>) -> usize {
        let mut expired_keys = Vec::new();

        for (key, cached_features) in &self.features {
            if now > cached_features.cache_metadata.expires_at {
                expired_keys.push(key.clone());
            }
        }

        for key in &expired_keys {
            self.features.remove(key);
            self.access_patterns.remove(key);

            // Clean up similarity index
            for similar_keys in self.similarity_index.values_mut() {
                similar_keys.retain(|k| k != key);
            }
        }

        expired_keys.len()
    }
}

impl ModelCache {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            performance_metrics: HashMap::new(),
            loading_times: HashMap::new(),
        }
    }

    pub fn get_model(&mut self, model_id: &str) -> Option<&CachedModel> {
        if let Some(model) = self.models.get_mut(model_id) {
            model.last_used = chrono::Utc::now();
            Some(model)
        } else {
            None
        }
    }

    pub fn insert_model(&mut self, model_id: String, model_data: Vec<u8>) {
        let metadata = ModelMetadata {
            version: "1.0".to_string(),
            model_type: "neural_network".to_string(),
            trained_at: chrono::Utc::now(),
            size_bytes: model_data.len(),
            performance: ModelPerformanceCache {
                accuracy: 0.85,
                precision: 0.8,
                recall: 0.8,
                f1_score: 0.8,
                avg_prediction_time: std::time::Duration::from_millis(10),
            },
        };

        let cached_model = CachedModel {
            model_id: model_id.clone(),
            model_data,
            metadata,
            last_used: chrono::Utc::now(),
        };

        self.models.insert(model_id, cached_model);
    }
}

impl Default for CacheAnalytics {
    fn default() -> Self {
        Self {
            hit_rates: CacheHitRates {
                overall: 0.0,
                predictions: 0.0,
                features: 0.0,
                models: 0.0,
                by_severity: HashMap::new(),
                by_analyzer: HashMap::new(),
            },
            performance_metrics: CachePerformanceMetrics {
                avg_lookup_time: std::time::Duration::from_micros(100),
                avg_write_time: std::time::Duration::from_micros(200),
                memory_usage: 0,
                efficiency_score: 0.0,
                eviction_rate: 0.0,
            },
            usage_statistics: CacheUsageStatistics {
                total_requests: 0,
                total_hits: 0,
                total_misses: 0,
                total_evictions: 0,
                size_utilization: 0.0,
                popular_entries: Vec::new(),
            },
            optimization_suggestions: Vec::new(),
        }
    }
}
