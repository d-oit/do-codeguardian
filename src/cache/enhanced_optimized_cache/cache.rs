use super::*;

/// Enhanced optimized cache with memory pool integration
#[deprecated(
    since = "0.1.0",
    note = "Use `crate::cache::unified_cache::UnifiedCache` with `CacheStrategyType::Pooled` instead. \
            See the migration guide in the module documentation for details."
)]
pub struct EnhancedOptimizedCache {
    entries: HashMap<PathBuf, super::PooledCacheEntry>,
    max_entries: usize,
    max_memory_mb: usize,
    current_memory_bytes: usize,
    stats: CacheStats,
    memory_pools: MemoryPoolManager,
}

impl EnhancedOptimizedCache {
    #[deprecated(
        since = "0.1.0",
        note = "Use `UnifiedCache::pooled(max_entries, max_memory_mb)` instead"
    )]
    pub fn new(max_entries: usize, max_memory_mb: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(max_entries),
            max_entries,
            max_memory_mb,
            current_memory_bytes: 0,
            stats: CacheStats::default(),
            memory_pools: MemoryPoolManager::new(),
        }
    }

    #[deprecated(
        since = "0.1.0",
        note = "Memory pools are automatically configured in UnifiedCache"
    )]
    pub fn with_memory_pools(mut self, pools: MemoryPoolManager) -> Self {
        self.memory_pools = pools;
        self
    }

    /// Get cached findings if valid, otherwise return None
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::get()` instead")]
    pub fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>> {
        self.stats.total_requests += 1;

        if let Some(entry) = self.entries.get_mut(file_path) {
            // Quick check: if config changed, entry is invalid
            if entry.config_hash != config_hash {
                self.stats.config_misses += 1;
                self.remove_entry(file_path);
                return Ok(None);
            }

            // Validate against current file state
            match super::FileMetadata::from_file(file_path) {
                Ok(metadata) => {
                    if entry.is_valid(&metadata, config_hash) {
                        entry.update_access();
                        self.stats.hits += 1;
                        self.stats.total_hit_time_saved_ms += entry.analysis_duration_ms;
                        return Ok(Some(entry.findings.clone()));
                    } else {
                        self.stats.file_changed_misses += 1;
                        self.remove_entry(file_path);
                    }
                }
                Err(_) => {
                    // File might have been deleted
                    self.stats.file_error_misses += 1;
                    self.remove_entry(file_path);
                }
            }
        }

        self.stats.misses += 1;
        Ok(None)
    }

    /// Store findings in cache with pooled memory management
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::put()` instead")]
    pub fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()> {
        let metadata = super::FileMetadata::from_file(file_path)?;

        // Use pooled strings for config hash
        let config_hash_pooled = self
            .memory_pools
            .string_pool()
            .lock()
            .unwrap()
            .get(config_hash);

        let entry = super::PooledCacheEntry::new(
            findings,
            metadata.content_hash,
            (*config_hash_pooled).clone(),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(metadata.modified_time),
            metadata.size,
            analysis_duration_ms,
        );

        let entry_size = self.estimate_entry_size(&entry);

        // Check if we need to make room
        self.ensure_capacity(entry_size)?;

        // Remove existing entry if present
        if self.entries.contains_key(file_path) {
            self.remove_entry(file_path);
        }

        // Use pooled PathBuf for key
        let _path_key = self.memory_pools.path_pool().lock().unwrap().get();
        let path_key = file_path.to_path_buf(); // For now, keep original logic

        // Add new entry
        self.current_memory_bytes += entry_size;
        self.entries.insert(path_key, entry);
        self.stats.entries_added += 1;

        Ok(())
    }

    /// Remove an entry and return objects to pools
    fn remove_entry(&mut self, file_path: &Path) {
        if let Some(entry) = self.entries.remove(file_path) {
            let entry_size = self.estimate_entry_size(&entry);
            self.current_memory_bytes = self.current_memory_bytes.saturating_sub(entry_size);
            self.stats.entries_evicted += 1;

            // Return findings to memory pool for reuse
            if let Ok(mut finding_pool) = self.memory_pools.finding_pool().lock() {
                for finding in entry.findings {
                    finding_pool.put(finding);
                }
            }
        }
    }

    /// Ensure cache has capacity for new entry
    fn ensure_capacity(&mut self, new_entry_size: usize) -> Result<()> {
        let max_memory_bytes = self.max_memory_mb * 1024 * 1024;

        // Check memory limit
        while self.current_memory_bytes + new_entry_size > max_memory_bytes
            || self.entries.len() >= self.max_entries
        {
            if self.entries.is_empty() {
                break;
            }

            self.evict_least_valuable_entry();
        }

        Ok(())
    }

    /// Evict the least valuable entry based on priority score
    fn evict_least_valuable_entry(&mut self) {
        if let Some((path_to_remove, _)) = self
            .entries
            .iter()
            .min_by(|(_, a), (_, b)| {
                a.priority_score()
                    .partial_cmp(&b.priority_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            self.remove_entry(&path_to_remove);
        }
    }

    /// Estimate memory usage of a cache entry
    fn estimate_entry_size(&self, entry: &super::PooledCacheEntry) -> usize {
        let base_size = std::mem::size_of::<super::PooledCacheEntry>();
        let findings_size = entry.findings.len() * std::mem::size_of::<Finding>();
        let string_sizes = entry.file_hash.len() + entry.config_hash.len();

        // Add estimated size for Finding string fields
        let finding_strings_size: usize = entry
            .findings
            .iter()
            .map(|f| {
                f.message.len()
                    + f.description.as_ref().map_or(0, |s| s.len())
                    + f.suggestion.as_ref().map_or(0, |s| s.len())
            })
            .sum();

        base_size + findings_size + string_sizes + finding_strings_size
    }

    /// Clean up expired and invalid entries
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::cleanup()` instead")]
    pub fn cleanup(&mut self, max_age_hours: u64) -> usize {
        let max_age_seconds = max_age_hours * 3600;
        let mut removed_count = 0;

        let paths_to_remove: Vec<PathBuf> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.age_seconds() >= max_age_seconds)
            .map(|(path, _)| path.clone())
            .collect();

        for path in paths_to_remove {
            self.remove_entry(&path);
            removed_count += 1;
        }

        self.stats.entries_expired += removed_count;
        removed_count as usize
    }

    /// Get cache statistics
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::stats()` instead")]
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get current cache utilization
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::utilization()` instead")]
    pub fn utilization(&self) -> CacheUtilization {
        CacheUtilization {
            entry_count: self.entries.len(),
            max_entries: self.max_entries,
            memory_usage_mb: self.current_memory_bytes as f64 / (1024.0 * 1024.0),
            max_memory_mb: self.max_memory_mb as f64,
            hit_rate: self.stats.hit_rate(),
            average_entry_size_kb: if self.entries.is_empty() {
                0.0
            } else {
                (self.current_memory_bytes as f64 / self.entries.len() as f64) / 1024.0
            },
        }
    }

    /// Get memory pool statistics
    #[deprecated(
        since = "0.1.0",
        note = "Use `UnifiedCache::memory_pool_stats()` instead"
    )]
    pub fn memory_pool_stats(&self) -> crate::cache::memory_pool::MemoryPoolStats {
        self.memory_pools.stats()
    }

    /// Get memory savings estimate
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::memory_savings()` instead")]
    pub fn memory_savings(&self) -> crate::cache::memory_pool::MemorySavings {
        self.memory_pools.memory_savings_estimate()
    }

    /// Get access to memory pools (for UnifiedCache compatibility)
    pub fn memory_pools(&self) -> &MemoryPoolManager {
        &self.memory_pools
    }

    /// Clear all cache entries and return objects to pools
    #[deprecated(since = "0.1.0", note = "Use `UnifiedCache::clear()` instead")]
    pub fn clear(&mut self) {
        let entries_to_clear: Vec<(PathBuf, super::PooledCacheEntry)> =
            self.entries.drain().collect();

        {
            // let mut finding_pool = self.memory_pools.finding_pool().lock().unwrap();
            for (_, entry) in &entries_to_clear {
                for _finding in entry.findings.clone() {
                    //     finding_pool.put(finding);
                }
            }
        }

        self.current_memory_bytes = 0;
        self.stats.entries_evicted += entries_to_clear.len() as u64;
    }
}

impl Default for EnhancedOptimizedCache {
    fn default() -> Self {
        Self::new(1000, 100) // 1000 entries, 100MB max
    }
}
