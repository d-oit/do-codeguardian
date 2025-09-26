//! Unified cache architecture using Strategy pattern
//!
//! This module provides a unified caching interface that consolidates
//! basic and enhanced cache implementations with runtime strategy selection.
//! Supports both memory-pooled and non-pooled modes for optimal performance.

#![allow(deprecated)]

use crate::types::Finding;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tracing;

/// Unified cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub total_requests: u64,
    pub hits: u64,
    pub misses: u64,
    pub config_misses: u64,
    pub file_changed_misses: u64,
    pub file_error_misses: u64,
    pub entries_added: u64,
    pub entries_evicted: u64,
    pub entries_expired: u64,
    pub total_hit_time_saved_ms: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.hits as f64 / self.total_requests as f64
        }
    }

    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    pub fn time_saved_seconds(&self) -> f64 {
        self.total_hit_time_saved_ms as f64 / 1000.0
    }

    pub fn report(&self) -> String {
        format!(
            "Cache Performance Report:\n\
             - Total requests: {}\n\
             - Hit rate: {:.1}%\n\
             - Cache hits: {} (saved {:.1}s)\n\
             - Cache misses: {} (config: {}, file changed: {}, file error: {})\n\
             - Entries added: {}\n\
             - Entries evicted: {}\n\
             - Entries expired: {}",
            self.total_requests,
            self.hit_rate() * 100.0,
            self.hits,
            self.time_saved_seconds(),
            self.misses,
            self.config_misses,
            self.file_changed_misses,
            self.file_error_misses,
            self.entries_added,
            self.entries_evicted,
            self.entries_expired
        )
    }
}

/// Current cache utilization metrics
#[derive(Debug)]
pub struct CacheUtilization {
    pub entry_count: usize,
    pub max_entries: usize,
    pub memory_usage_mb: f64,
    pub max_memory_mb: f64,
    pub hit_rate: f64,
    pub average_entry_size_kb: f64,
}

impl CacheUtilization {
    pub fn entry_utilization_percentage(&self) -> f64 {
        if self.max_entries == 0 {
            0.0
        } else {
            (self.entry_count as f64 / self.max_entries as f64) * 100.0
        }
    }

    pub fn memory_utilization_percentage(&self) -> f64 {
        if self.max_memory_mb == 0.0 {
            0.0
        } else {
            (self.memory_usage_mb / self.max_memory_mb) * 100.0
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Cache Utilization:\n\
             - Entries: {}/{} ({:.1}%)\n\
             - Memory: {:.1}/{:.1} MB ({:.1}%)\n\
             - Hit rate: {:.1}%\n\
             - Average entry size: {:.1} KB",
            self.entry_count,
            self.max_entries,
            self.entry_utilization_percentage(),
            self.memory_usage_mb,
            self.max_memory_mb,
            self.memory_utilization_percentage(),
            self.hit_rate * 100.0,
            self.average_entry_size_kb
        )
    }
}

/// Cache strategy trait defining the interface for different caching behaviors
pub trait CacheStrategy: Send + Sync {
    /// Get cached findings if valid, otherwise return None
    fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>>;

    /// Store findings in cache with metadata
    fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()>;

    /// Remove an entry from cache
    fn remove(&mut self, file_path: &Path) -> Result<()>;

    /// Clean up expired and invalid entries
    fn cleanup(&mut self, max_age_hours: u64) -> usize;

    /// Get cache statistics
    fn stats(&self) -> &CacheStats;

    /// Get current cache utilization
    fn utilization(&self) -> CacheUtilization;

    /// Clear all cache entries
    fn clear(&mut self);

    /// Save cache to disk
    fn save_to_disk(&self, cache_file: &Path) -> Result<()>;

    /// Load cache from disk
    fn load_from_disk(&mut self, cache_file: &Path) -> Result<()>;

    /// Get strategy name for identification
    fn strategy_name(&self) -> &'static str;

    /// Downcast helper
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Basic cache strategy using OptimizedCache
pub struct BasicCacheStrategy {
    cache: crate::cache::optimized_cache::OptimizedCache,
    unified_stats: CacheStats,
}

impl BasicCacheStrategy {
    pub fn new(max_entries: usize, max_memory_mb: usize) -> Self {
        Self {
            cache: crate::cache::optimized_cache::OptimizedCache::new(max_entries, max_memory_mb),
            unified_stats: CacheStats::default(),
        }
    }

    /// Convert OptimizedCache stats to unified format
    fn convert_stats(&self) -> CacheStats {
        let opt_stats = self.cache.stats();
        CacheStats {
            total_requests: opt_stats.total_requests,
            hits: opt_stats.hits,
            misses: opt_stats.misses,
            config_misses: opt_stats.config_misses,
            file_changed_misses: opt_stats.file_changed_misses,
            file_error_misses: opt_stats.file_error_misses,
            entries_added: opt_stats.entries_added,
            entries_evicted: opt_stats.entries_evicted,
            entries_expired: opt_stats.entries_expired,
            total_hit_time_saved_ms: opt_stats.total_hit_time_saved_ms,
        }
    }
}

impl CacheStrategy for BasicCacheStrategy {
    fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>> {
        let result = self.cache.get(file_path, config_hash)?;
        // Update unified stats
        self.unified_stats = self.convert_stats();
        Ok(result)
    }

    fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()> {
        self.cache
            .put(file_path, findings, config_hash, analysis_duration_ms)?;
        self.unified_stats = self.convert_stats();
        Ok(())
    }

    fn remove(&mut self, file_path: &Path) -> Result<()> {
        // Since remove_entry is private, we'll use a workaround
        // by attempting to get the entry with invalid config (which will remove it if it exists)
        let _ = self.cache.get(file_path, "invalid_config_to_remove")?;
        self.unified_stats = self.convert_stats();
        Ok(())
    }

    fn cleanup(&mut self, max_age_hours: u64) -> usize {
        let removed = self.cache.cleanup(max_age_hours);
        self.unified_stats = self.convert_stats();
        removed
    }

    fn stats(&self) -> &CacheStats {
        &self.unified_stats
    }

    fn utilization(&self) -> CacheUtilization {
        let opt_util = self.cache.utilization();
        CacheUtilization {
            entry_count: opt_util.entry_count,
            max_entries: opt_util.max_entries,
            memory_usage_mb: opt_util.memory_usage_mb,
            max_memory_mb: opt_util.max_memory_mb,
            hit_rate: opt_util.hit_rate,
            average_entry_size_kb: opt_util.average_entry_size_kb,
        }
    }

    fn clear(&mut self) {
        self.cache.clear();
        self.unified_stats = CacheStats::default();
    }

    fn save_to_disk(&self, _cache_file: &Path) -> Result<()> {
        // For now, we'll use a simple approach - in practice, we'd need async here
        // This is a limitation of the current OptimizedCache implementation
        tracing::warn!(
            "BasicCacheStrategy save_to_disk is synchronous, consider using async version"
        );
        Ok(())
    }

    fn load_from_disk(&mut self, _cache_file: &Path) -> Result<()> {
        // Similar limitation
        tracing::warn!(
            "BasicCacheStrategy load_from_disk is synchronous, consider using async version"
        );
        Ok(())
    }

    fn strategy_name(&self) -> &'static str {
        "BasicCache"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Pooled cache strategy using EnhancedOptimizedCache with memory pools
pub struct PooledCacheStrategy {
    cache: crate::cache::enhanced_optimized_cache::EnhancedOptimizedCache,
    unified_stats: CacheStats,
}

impl PooledCacheStrategy {
    pub fn new(max_entries: usize, max_memory_mb: usize) -> Self {
        Self {
            cache: crate::cache::enhanced_optimized_cache::EnhancedOptimizedCache::new(
                max_entries,
                max_memory_mb,
            ),
            unified_stats: CacheStats::default(),
        }
    }

    pub fn with_memory_pools(
        mut self,
        pools: crate::cache::memory_pool::MemoryPoolManager,
    ) -> Self {
        self.cache = self.cache.with_memory_pools(pools);
        self
    }

    /// Convert EnhancedOptimizedCache stats to unified format
    fn convert_stats(&self) -> CacheStats {
        let enh_stats = self.cache.stats();
        CacheStats {
            total_requests: enh_stats.total_requests,
            hits: enh_stats.hits,
            misses: enh_stats.misses,
            config_misses: enh_stats.config_misses,
            file_changed_misses: enh_stats.file_changed_misses,
            file_error_misses: enh_stats.file_error_misses,
            entries_added: enh_stats.entries_added,
            entries_evicted: enh_stats.entries_evicted,
            entries_expired: enh_stats.entries_expired,
            total_hit_time_saved_ms: enh_stats.total_hit_time_saved_ms,
        }
    }
}

impl CacheStrategy for PooledCacheStrategy {
    fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>> {
        let result = self.cache.get(file_path, config_hash)?;
        self.unified_stats = self.convert_stats();
        Ok(result)
    }

    fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()> {
        self.cache
            .put(file_path, findings, config_hash, analysis_duration_ms)?;
        self.unified_stats = self.convert_stats();
        Ok(())
    }

    fn remove(&mut self, file_path: &Path) -> Result<()> {
        // Since remove_entry is private, we'll use a workaround
        // by attempting to get the entry with invalid config (which will remove it if it exists)
        let _ = self.cache.get(file_path, "invalid_config_to_remove")?;
        self.unified_stats = self.convert_stats();
        Ok(())
    }

    fn cleanup(&mut self, max_age_hours: u64) -> usize {
        let removed = self.cache.cleanup(max_age_hours);
        self.unified_stats = self.convert_stats();
        removed
    }

    fn stats(&self) -> &CacheStats {
        &self.unified_stats
    }

    fn utilization(&self) -> CacheUtilization {
        let enh_util = self.cache.utilization();
        CacheUtilization {
            entry_count: enh_util.entry_count,
            max_entries: enh_util.max_entries,
            memory_usage_mb: enh_util.memory_usage_mb,
            max_memory_mb: enh_util.max_memory_mb,
            hit_rate: enh_util.hit_rate,
            average_entry_size_kb: enh_util.average_entry_size_kb,
        }
    }

    fn clear(&mut self) {
        self.cache.clear();
        self.unified_stats = CacheStats::default();
    }

    fn save_to_disk(&self, _cache_file: &Path) -> Result<()> {
        // Enhanced cache doesn't have disk persistence yet
        // This could be added in the future
        tracing::warn!("PooledCacheStrategy does not support disk persistence yet");
        Ok(())
    }

    fn load_from_disk(&mut self, _cache_file: &Path) -> Result<()> {
        // Enhanced cache doesn't have disk persistence yet
        tracing::warn!("PooledCacheStrategy does not support disk loading yet");
        Ok(())
    }

    fn strategy_name(&self) -> &'static str {
        "PooledCache"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Configuration for cache strategy selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStrategyType {
    Basic,
    Pooled,
}

/// Configuration for UnifiedCache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheConfig {
    pub strategy: CacheStrategyType,
    pub max_entries: usize,
    pub max_memory_mb: usize,
    pub enable_memory_pools: bool,
    pub pool_sizes: Option<MemoryPoolSizes>,
}

impl Default for UnifiedCacheConfig {
    fn default() -> Self {
        Self {
            strategy: CacheStrategyType::Basic,
            max_entries: 1000,
            max_memory_mb: 100,
            enable_memory_pools: false,
            pool_sizes: None,
        }
    }
}

/// Memory pool size configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolSizes {
    pub findings_pool_size: usize,
    pub strings_pool_size: usize,
    pub pathbuf_pool_size: usize,
    pub hashmap_pool_size: usize,
}

impl Default for MemoryPoolSizes {
    fn default() -> Self {
        Self {
            findings_pool_size: 1000,
            strings_pool_size: 5000,
            pathbuf_pool_size: 500,
            hashmap_pool_size: 200,
        }
    }
}

/// Unified cache using composition over inheritance with strategy pattern
pub struct UnifiedCache {
    strategy: Box<dyn CacheStrategy>,
    config: UnifiedCacheConfig,
}

impl UnifiedCache {
    /// Create a new unified cache with the specified configuration
    pub fn new(config: UnifiedCacheConfig) -> Result<Self> {
        let strategy: Box<dyn CacheStrategy> = match config.strategy {
            CacheStrategyType::Basic => Box::new(BasicCacheStrategy::new(
                config.max_entries,
                config.max_memory_mb,
            )),
            CacheStrategyType::Pooled => {
                let mut pooled = PooledCacheStrategy::new(config.max_entries, config.max_memory_mb);

                if config.enable_memory_pools {
                    let pool_sizes = config.pool_sizes.clone().unwrap_or_default();
                    let memory_pools = crate::cache::memory_pool::MemoryPoolManager::with_config(
                        pool_sizes.findings_pool_size,
                        pool_sizes.strings_pool_size,
                        pool_sizes.pathbuf_pool_size,
                        pool_sizes.hashmap_pool_size,
                    );
                    pooled = pooled.with_memory_pools(memory_pools);
                }

                Box::new(pooled)
            }
        };

        tracing::info!(
            "Initialized UnifiedCache with strategy: {}",
            strategy.strategy_name()
        );

        Ok(Self { strategy, config })
    }

    /// Create a basic cache (backward compatibility)
    pub fn basic(max_entries: usize, max_memory_mb: usize) -> Result<Self> {
        Self::new(UnifiedCacheConfig {
            strategy: CacheStrategyType::Basic,
            max_entries,
            max_memory_mb,
            enable_memory_pools: false,
            pool_sizes: None,
        })
    }

    /// Create a pooled cache with memory optimization
    pub fn pooled(max_entries: usize, max_memory_mb: usize) -> Result<Self> {
        Self::new(UnifiedCacheConfig {
            strategy: CacheStrategyType::Pooled,
            max_entries,
            max_memory_mb,
            enable_memory_pools: true,
            pool_sizes: Some(MemoryPoolSizes::default()),
        })
    }

    /// Switch cache strategy at runtime
    pub fn switch_strategy(&mut self, new_config: UnifiedCacheConfig) -> Result<()> {
        // Save current cache state if possible
        let cache_file = PathBuf::from(".codeguardian/cache/temp_migration.json");
        if let Err(e) = self.strategy.save_to_disk(&cache_file) {
            tracing::warn!("Failed to save cache state during strategy switch: {}", e);
        }

        // Create new strategy
        let mut new_strategy: Box<dyn CacheStrategy> = match new_config.strategy {
            CacheStrategyType::Basic => Box::new(BasicCacheStrategy::new(
                new_config.max_entries,
                new_config.max_memory_mb,
            )),
            CacheStrategyType::Pooled => {
                let mut pooled =
                    PooledCacheStrategy::new(new_config.max_entries, new_config.max_memory_mb);

                if new_config.enable_memory_pools {
                    let pool_sizes = new_config.pool_sizes.clone().unwrap_or_default();
                    let memory_pools = crate::cache::memory_pool::MemoryPoolManager::with_config(
                        pool_sizes.findings_pool_size,
                        pool_sizes.strings_pool_size,
                        pool_sizes.pathbuf_pool_size,
                        pool_sizes.hashmap_pool_size,
                    );
                    pooled = pooled.with_memory_pools(memory_pools);
                }

                Box::new(pooled)
            }
        };

        // Try to load previous state into new strategy
        if let Err(e) = new_strategy.load_from_disk(&cache_file) {
            tracing::warn!("Failed to load cache state into new strategy: {}", e);
        }

        // Clean up temp file
        std::fs::remove_file(&cache_file).ok();

        self.strategy = new_strategy;
        self.config = new_config;

        tracing::info!(
            "Switched cache strategy to: {}",
            self.strategy.strategy_name()
        );

        Ok(())
    }

    /// Get cached findings if valid
    pub fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>> {
        self.strategy.get(file_path, config_hash)
    }

    /// Store findings in cache
    pub fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()> {
        self.strategy
            .put(file_path, findings, config_hash, analysis_duration_ms)
    }

    /// Remove an entry from cache
    pub fn remove(&mut self, file_path: &Path) -> Result<()> {
        self.strategy.remove(file_path)
    }

    /// Clean up expired entries
    pub fn cleanup(&mut self, max_age_hours: u64) -> usize {
        self.strategy.cleanup(max_age_hours)
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        self.strategy.stats()
    }

    /// Get cache utilization
    pub fn utilization(&self) -> CacheUtilization {
        self.strategy.utilization()
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.strategy.clear();
    }

    /// Save cache to disk
    pub fn save_to_disk(&self, cache_file: &Path) -> Result<()> {
        self.strategy.save_to_disk(cache_file)
    }

    /// Load cache from disk
    pub fn load_from_disk(&mut self, cache_file: &Path) -> Result<()> {
        self.strategy.load_from_disk(cache_file)
    }

    /// Get current strategy name
    pub fn strategy_name(&self) -> &'static str {
        self.strategy.strategy_name()
    }

    /// Get current configuration
    pub fn config(&self) -> &UnifiedCacheConfig {
        &self.config
    }

    /// Get memory pool statistics (if using pooled strategy)
    pub fn memory_pool_stats(&self) -> Option<crate::cache::memory_pool::MemoryPoolStats> {
        self.strategy
            .as_any()
            .downcast_ref::<PooledCacheStrategy>()
            .map(|pooled| pooled.cache.memory_pools().stats())
    }

    /// Get memory savings estimate (if using pooled strategy)
    pub fn memory_savings(&self) -> Option<crate::cache::memory_pool::MemorySavings> {
        self.strategy
            .as_any()
            .downcast_ref::<PooledCacheStrategy>()
            .map(|pooled| pooled.cache.memory_pools().memory_savings_estimate())
    }
}

impl Default for UnifiedCache {
    fn default() -> Self {
        Self::new(UnifiedCacheConfig::default()).expect("Failed to create default UnifiedCache")
    }
}

// Re-export types for backward compatibility
pub use crate::cache::optimized_cache::{
    CacheStats as OptimizedCacheStats, CacheUtilization as OptimizedCacheUtilization,
};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_unified_cache_basic() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = UnifiedCache::basic(10, 10)?;
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        let findings = vec![];
        let config_hash = "test_config";

        // Put and get
        cache.put(&test_file, findings.clone(), config_hash, 100)?;
        let result = cache.get(&test_file, config_hash)?;

        assert!(result.is_some());
        assert_eq!(cache.strategy_name(), "BasicCache");

        Ok(())
    }

    #[test]
    fn test_unified_cache_pooled() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = UnifiedCache::pooled(10, 10)?;
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        let findings = vec![];
        let config_hash = "test_config";

        // Put and get
        cache.put(&test_file, findings.clone(), config_hash, 100)?;
        let result = cache.get(&test_file, config_hash)?;

        assert!(result.is_some());
        assert_eq!(cache.strategy_name(), "PooledCache");

        // Check memory pool stats
        let pool_stats = cache.memory_pool_stats();
        assert!(pool_stats.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_strategy_switching() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = UnifiedCache::basic(10, 10)?;
        assert_eq!(cache.strategy_name(), "BasicCache");

        // Switch to pooled
        let new_config = UnifiedCacheConfig {
            strategy: CacheStrategyType::Pooled,
            max_entries: 10,
            max_memory_mb: 10,
            enable_memory_pools: true,
            pool_sizes: Some(MemoryPoolSizes::default()),
        };

        cache.switch_strategy(new_config)?;
        assert_eq!(cache.strategy_name(), "PooledCache");

        Ok(())
    }
}

impl UnifiedCache {
    /// Save cache to disk asynchronously
    pub async fn save_async(&self, cache_file: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = cache_file.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Save config
        let config_data = serde_json::to_string_pretty(&self.config)?;
        fs::write(cache_file, config_data).await?;

        // Save strategy data
        let data_file = cache_file.with_extension("data");
        self.strategy.save_to_disk(&data_file)?;

        tracing::debug!(
            "Cache saved to {} and {}",
            cache_file.display(),
            data_file.display()
        );
        Ok(())
    }

    /// Load cache from disk asynchronously
    pub async fn load_async(cache_file: &Path) -> Result<Self> {
        if !cache_file.exists() {
            tracing::debug!(
                "Cache file {} does not exist, creating new cache",
                cache_file.display()
            );
            return Self::new(UnifiedCacheConfig::default());
        }

        let config_data = fs::read_to_string(cache_file).await?;
        let config: UnifiedCacheConfig = serde_json::from_str(&config_data)?;

        let mut cache = Self::new(config)?;

        let data_file = cache_file.with_extension("data");
        if data_file.exists() {
            cache.strategy.load_from_disk(&data_file)?;
        }

        tracing::debug!(
            "Cache loaded from {} and {}",
            cache_file.display(),
            data_file.display()
        );
        Ok(cache)
    }

    /// Save cache periodically in background
    pub fn start_periodic_save(
        self: Arc<Self>,
        cache_file: PathBuf,
        interval_seconds: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                if let Err(e) = self.save_async(&cache_file).await {
                    tracing::warn!("Failed to save cache periodically: {}", e);
                }
            }
        })
    }
}
