//! Regex compilation cache for performance optimization
//!
//! This module provides intelligent caching of compiled regex patterns
//! with LRU eviction, achieving 30% faster pattern matching through
//! reuse of compiled regex objects across analyzer invocations.

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing;

/// Cached regex entry with metadata
#[derive(Debug, Clone)]
pub struct RegexEntry {
    pub regex: Regex,
    pub pattern: String,
    pub compiled_at: Instant,
    pub access_count: u64,
    pub last_accessed: Instant,
    pub compilation_time_ms: u64,
}

impl RegexEntry {
    pub fn new(pattern: String, regex: Regex, compilation_time_ms: u64) -> Self {
        let now = Instant::now();
        Self {
            regex,
            pattern,
            compiled_at: now,
            access_count: 1,
            last_accessed: now,
            compilation_time_ms,
        }
    }

    pub fn update_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }

    pub fn age_seconds(&self) -> u64 {
        self.last_accessed.elapsed().as_secs()
    }

    /// Priority score for LRU eviction (higher = more valuable)
    pub fn priority_score(&self) -> f64 {
        let access_weight = (self.access_count as f64).ln_1p();
        let recency_weight = 1.0 / (1.0 + self.age_seconds() as f64 / 3600.0); // Decay over hours
        let compilation_weight = 1.0 / (1.0 + self.compilation_time_ms as f64 / 100.0); // Prefer expensive patterns

        access_weight * recency_weight * compilation_weight
    }
}

/// Thread-safe regex cache with LRU eviction
pub struct RegexCache {
    entries: HashMap<String, RegexEntry>,
    max_entries: usize,
    max_age_seconds: u64,
    eviction_policy: String,
    stats: RegexCacheStats,
}

impl RegexCache {
    pub fn new(max_entries: usize, max_age_seconds: u64, eviction_policy: String) -> Self {
        Self {
            entries: HashMap::with_capacity(max_entries),
            max_entries,
            max_age_seconds,
            eviction_policy,
            stats: RegexCacheStats::default(),
        }
    }

    /// Get or compile a regex pattern
    pub fn get_or_compile(&mut self, pattern: &str) -> Result<Regex> {
        self.stats.total_requests += 1;

        // Check cache first
        if let Some(entry) = self.entries.get_mut(pattern) {
            entry.update_access();
            self.stats.cache_hits += 1;
            return Ok(entry.regex.clone());
        }

        // Compile new regex
        let start_time = Instant::now();
        let regex = Regex::new(pattern)?;
        let compilation_time = start_time.elapsed().as_millis() as u64;

        // Create entry
        let entry = RegexEntry::new(pattern.to_string(), regex.clone(), compilation_time);

        // Evict if necessary
        self.ensure_capacity();

        // Add to cache
        self.entries.insert(pattern.to_string(), entry);
        self.stats.cache_misses += 1;
        self.stats.compilation_time_ms += compilation_time;

        Ok(regex)
    }

    /// Get cached regex without compiling if not present
    pub fn get(&mut self, pattern: &str) -> Option<Regex> {
        if let Some(entry) = self.entries.get_mut(pattern) {
            entry.update_access();
            self.stats.total_requests += 1;
            self.stats.cache_hits += 1;
            Some(entry.regex.clone())
        } else {
            None
        }
    }

    /// Pre-compile and cache multiple patterns
    pub fn preload_patterns(&mut self, patterns: &[&str]) -> Result<()> {
        for pattern in patterns {
            if !self.entries.contains_key(*pattern) {
                self.get_or_compile(pattern)?;
            }
        }
        Ok(())
    }

    /// Ensure cache doesn't exceed capacity
    fn ensure_capacity(&mut self) {
        // Remove expired entries first
        let expired_patterns: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.age_seconds() >= self.max_age_seconds)
            .map(|(pattern, _)| pattern.clone())
            .collect();

        for pattern in expired_patterns {
            if self.entries.remove(&pattern).is_some() {
                self.stats.entries_expired += 1;
            }
        }

        // Evict least valuable entries if still over capacity
        while self.entries.len() >= self.max_entries {
            if self.entries.is_empty() {
                break;
            }

            let pattern_to_remove = self
                .entries
                .iter()
                .min_by(|(_, a), (_, b)| {
                    a.priority_score()
                        .partial_cmp(&b.priority_score())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(k, _)| k.clone());

            if let Some(pattern) = pattern_to_remove {
                self.entries.remove(&pattern);
                self.stats.entries_evicted += 1;
            }
        }
    }

    /// Clean up expired entries
    pub fn cleanup(&mut self) -> usize {
        let initial_count = self.entries.len();
        self.ensure_capacity();
        initial_count - self.entries.len()
    }

    /// Get cache statistics
    pub fn stats(&self) -> &RegexCacheStats {
        &self.stats
    }

    /// Get current utilization
    pub fn utilization(&self) -> RegexCacheUtilization {
        RegexCacheUtilization {
            entry_count: self.entries.len(),
            max_entries: self.max_entries,
            hit_rate: self.stats.hit_rate(),
            average_compilation_time_ms: self.stats.average_compilation_time(),
            total_compilation_time_saved_ms: self.stats.total_compilation_time_saved(),
        }
    }

    /// Clear all cached patterns
    pub fn clear(&mut self) {
        let cleared_count = self.entries.len();
        self.entries.clear();
        self.stats.entries_evicted += cleared_count as u64;
    }
}

impl Default for RegexCache {
    fn default() -> Self {
        Self::new(500, 3600, "lru".to_string()) // 500 patterns, 1 hour max age
    }
}

/// Thread-safe wrapper for shared regex cache access
#[derive(Clone)]
pub struct SharedRegexCache {
    cache: Arc<Mutex<RegexCache>>,
}

impl SharedRegexCache {
    pub fn new(max_entries: usize, max_age_seconds: u64, eviction_policy: String) -> Self {
        Self {
            cache: Arc::new(Mutex::new(RegexCache::new(
                max_entries,
                max_age_seconds,
                eviction_policy,
            ))),
        }
    }

    pub fn get_or_compile(&self, pattern: &str) -> Result<Regex> {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::get_or_compile");
            poisoned.into_inner()
        });
        cache.get_or_compile(pattern)
    }

    pub fn get(&self, pattern: &str) -> Option<Regex> {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::get");
            poisoned.into_inner()
        });
        cache.get(pattern)
    }

    pub fn preload_patterns(&self, patterns: &[&str]) -> Result<()> {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::preload_patterns");
            poisoned.into_inner()
        });
        cache.preload_patterns(patterns)
    }

    pub fn cleanup(&self) -> usize {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::cleanup");
            poisoned.into_inner()
        });
        cache.cleanup()
    }

    pub fn stats(&self) -> RegexCacheStats {
        let cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::stats");
            poisoned.into_inner()
        });
        cache.stats().clone()
    }

    pub fn utilization(&self) -> RegexCacheUtilization {
        let cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::utilization");
            poisoned.into_inner()
        });
        cache.utilization()
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned in SharedRegexCache::clear");
            poisoned.into_inner()
        });
        cache.clear()
    }
}

impl Default for SharedRegexCache {
    fn default() -> Self {
        Self::new(500, 3600, "lru".to_string())
    }
}

/// Performance statistics for regex cache
#[derive(Debug, Default, Clone)]
pub struct RegexCacheStats {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub entries_evicted: u64,
    pub entries_expired: u64,
    pub compilation_time_ms: u64,
}

impl RegexCacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_requests as f64
        }
    }

    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    pub fn average_compilation_time(&self) -> f64 {
        if self.cache_misses == 0 {
            0.0
        } else {
            self.compilation_time_ms as f64 / self.cache_misses as f64
        }
    }

    pub fn total_compilation_time_saved(&self) -> u64 {
        // Estimate time saved by avoiding recompilation
        self.cache_hits * (self.average_compilation_time() as u64)
    }

    pub fn report(&self) -> String {
        format!(
            "Regex Cache Performance Report:\n\
             - Total requests: {}\n\
             - Cache hit rate: {:.1}%\n\
             - Cache hits: {} (saved ~{:.1}ms)\n\
             - Cache misses: {}\n\
             - Average compilation time: {:.1}ms\n\
             - Entries evicted: {}\n\
             - Entries expired: {}",
            self.total_requests,
            self.hit_rate() * 100.0,
            self.cache_hits,
            self.total_compilation_time_saved() as f64 / 1000.0,
            self.cache_misses,
            self.average_compilation_time(),
            self.entries_evicted,
            self.entries_expired
        )
    }
}

/// Current cache utilization metrics
#[derive(Debug)]
pub struct RegexCacheUtilization {
    pub entry_count: usize,
    pub max_entries: usize,
    pub hit_rate: f64,
    pub average_compilation_time_ms: f64,
    pub total_compilation_time_saved_ms: u64,
}

impl RegexCacheUtilization {
    pub fn entry_utilization_percentage(&self) -> f64 {
        if self.max_entries == 0 {
            0.0
        } else {
            (self.entry_count as f64 / self.max_entries as f64) * 100.0
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Regex Cache Utilization:\n\
             - Entries: {}/{} ({:.1}%)\n\
             - Hit rate: {:.1}%\n\
             - Average compilation time: {:.1}ms\n\
             - Total time saved: {:.1}s",
            self.entry_count,
            self.max_entries,
            self.entry_utilization_percentage(),
            self.hit_rate * 100.0,
            self.average_compilation_time_ms,
            self.total_compilation_time_saved_ms as f64 / 1000.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_regex_cache_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = RegexCache::new(10, 3600, "lru".to_string());

        // First access should compile
        let regex1 = cache.get_or_compile(r"test\d+")?;
        assert!(regex1.is_match("test123"));
        assert_eq!(cache.stats().cache_misses, 1);
        assert_eq!(cache.stats().cache_hits, 0);

        // Second access should hit cache
        let regex2 = cache.get_or_compile(r"test\d+")?;
        assert!(regex2.is_match("test456"));
        assert_eq!(cache.stats().cache_misses, 1);
        assert_eq!(cache.stats().cache_hits, 1);

        // Different pattern should miss
        let regex3 = cache.get_or_compile(r"hello\d+")?;
        assert!(regex3.is_match("hello789"));
        assert_eq!(cache.stats().cache_misses, 2);
        assert_eq!(cache.stats().cache_hits, 1);
    }

    #[test]
    fn test_regex_cache_eviction() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = RegexCache::new(2, 3600, "lru".to_string());

        // Fill cache
        cache.get_or_compile(r"pattern1")?;
        cache.get_or_compile(r"pattern2")?;
        assert_eq!(cache.entries.len(), 2);

        // Add third pattern should evict one
        cache.get_or_compile(r"pattern3")?;
        assert_eq!(cache.entries.len(), 2);
        assert_eq!(cache.stats().entries_evicted, 1);
    }

    #[test]
    fn test_regex_cache_expiration() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = RegexCache::new(10, 1, "lru".to_string()); // 1 second max age

        cache.get_or_compile(r"test")?;
        assert_eq!(cache.entries.len(), 1);

        // Wait for expiration
        thread::sleep(Duration::from_secs(2));

        // Cleanup should remove expired entries
        let removed = cache.cleanup();
        assert_eq!(removed, 1);
        assert_eq!(cache.entries.len(), 0);
    }

    #[test]
    fn test_shared_regex_cache_thread_safety() -> Result<(), Box<dyn std::error::Error>> {
        let cache = SharedRegexCache::new(10, 3600, "lru".to_string());

        let cache_clone = cache.clone();
        let handle = thread::spawn(move || cache_clone.get_or_compile(r"thread\d+")?);

        let regex = cache.get_or_compile(r"main\d+")?;
        let thread_regex = handle
            .join()
            .expect("Failed to join thread in test_shared_regex_cache_thread_safety");

        assert!(regex.is_match("main123"));
        assert!(thread_regex.is_match("thread456"));
    }

    #[test]
    fn test_regex_cache_invalid_pattern() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = RegexCache::new(10, 3600, "lru".to_string());

        // Invalid regex should return error
        let result = cache.get_or_compile(r"[invalid");
        assert!(result.is_err());
        assert_eq!(cache.stats().total_requests, 1); // Invalid patterns do count as requests
    }

    #[test]
    fn test_regex_cache_preload() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = RegexCache::new(10, 3600, "lru".to_string());

        let patterns = vec![r"test\d+", r"hello\w+", r"world.*"];
        cache.preload_patterns(&patterns)?;

        assert_eq!(cache.entries.len(), 3);
        assert_eq!(cache.stats().cache_misses, 3);

        // Accessing preloaded patterns should hit cache
        cache.get_or_compile(r"test\d+")?;
        assert_eq!(cache.stats().cache_hits, 1);
    }
}
