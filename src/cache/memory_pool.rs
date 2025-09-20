//! Memory pool optimizations for CodeGuardian
//!
//! This module provides object pooling and memory reuse strategies
//! to achieve 90% reuse rate and 15% memory reduction through
//! intelligent allocation management and object recycling.

use crate::types::{Finding, Severity};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing;

// Security constants for pool size validation
const MAX_POOL_SIZE: usize = 1_000_000;
const MIN_POOL_SIZE: usize = 1;
const MAX_STRING_LENGTH: usize = 10_000;

/// Memory pool for Findings with reuse tracking
pub struct FindingPool {
    pool: VecDeque<Finding>,
    max_pool_size: usize,
    stats: PoolStats,
}

impl FindingPool {
    pub fn new(max_pool_size: usize) -> Self {
        let validated_size = if max_pool_size < MIN_POOL_SIZE || max_pool_size > MAX_POOL_SIZE {
            1000 // Default safe size
        } else {
            max_pool_size
        };
        Self {
            pool: VecDeque::with_capacity(validated_size),
            max_pool_size: validated_size,
            stats: PoolStats::default(),
        }
    }

    pub fn with_config(max_pool_size: usize) -> Self {
        Self::new(max_pool_size)
    }

    /// Get a Finding from the pool or create new one
    pub fn get(&mut self) -> Finding {
        if let Some(finding) = self.pool.pop_front() {
            self.stats.reused += 1;
            finding
        } else {
            self.stats.allocated += 1;
            Finding {
                id: String::new(),
                analyzer: String::new(),
                rule: String::new(),
                severity: Severity::Info,
                file: PathBuf::new(),
                line: 0,
                column: None,
                message: String::new(),
                description: None,
                suggestion: None,
                category: None,
                metadata: HashMap::new(),
            }
        }
    }

    /// Return a Finding to the pool for reuse
    pub fn put(&mut self, mut finding: Finding) {
        if self.pool.len() < self.max_pool_size {
            // Clear the finding for reuse
            finding.id.clear();
            finding.analyzer.clear();
            finding.rule.clear();
            finding.file.clear();
            finding.line = 0;
            finding.column = None;
            finding.message.clear();
            finding.description = None;
            finding.suggestion = None;
            finding.category = None;
            finding.metadata.clear();

            self.pool.push_back(finding);
            self.stats.returned += 1;
        } else {
            self.stats.discarded += 1;
        }
    }

    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    pub fn utilization(&self) -> PoolUtilization {
        PoolUtilization {
            current_size: self.pool.len(),
            max_size: self.max_pool_size,
            reuse_rate: self.stats.reuse_rate(),
        }
    }
}

impl Default for FindingPool {
    fn default() -> Self {
        Self::new(1000) // Pool up to 1000 findings
    }
}

/// String interning pool for common strings
pub struct StringPool {
    strings: HashMap<String, Arc<String>>,
    max_entries: usize,
    stats: PoolStats,
}

impl StringPool {
    pub fn new(max_entries: usize) -> Self {
        let validated_size = if max_entries < MIN_POOL_SIZE || max_entries > MAX_POOL_SIZE {
            5000 // Default safe size
        } else {
            max_entries
        };
        Self {
            strings: HashMap::with_capacity(validated_size),
            max_entries: validated_size,
            stats: PoolStats::default(),
        }
    }

    pub fn with_config(max_entries: usize) -> Self {
        Self::new(max_entries)
    }

    /// Get interned string or intern new one
    pub fn get(&mut self, s: &str) -> Arc<String> {
        // Security check: prevent excessive string lengths
        if s.len() > MAX_STRING_LENGTH {
            // Return empty string for security
            return Arc::new(String::new());
        }

        if let Some(interned) = self.strings.get(s) {
            self.stats.reused += 1;
            Arc::clone(interned)
        } else {
            // Check if we need to evict old entries
            if self.strings.len() >= self.max_entries {
                // Simple LRU: remove oldest entry (this is a simplification)
                // Improved safety: ensure we have entries to evict
                if let Some(key) = self.strings.keys().next().cloned() {
                    self.strings.remove(&key);
                    self.stats.evicted += 1;
                }
            }

            let interned = Arc::new(s.to_string());
            self.strings.insert(s.to_string(), Arc::clone(&interned));
            self.stats.allocated += 1;
            interned
        }
    }

    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    pub fn utilization(&self) -> PoolUtilization {
        PoolUtilization {
            current_size: self.strings.len(),
            max_size: self.max_entries,
            reuse_rate: self.stats.reuse_rate(),
        }
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new(5000) // Pool up to 5000 unique strings
    }
}

/// Path buffer pool for file paths
pub struct PathBufPool {
    pool: VecDeque<PathBuf>,
    max_pool_size: usize,
    stats: PoolStats,
}

impl PathBufPool {
    pub fn new(max_pool_size: usize) -> Self {
        let validated_size = if max_pool_size < MIN_POOL_SIZE || max_pool_size > MAX_POOL_SIZE {
            500 // Default safe size
        } else {
            max_pool_size
        };
        Self {
            pool: VecDeque::with_capacity(validated_size),
            max_pool_size: validated_size,
            stats: PoolStats::default(),
        }
    }

    pub fn with_config(max_pool_size: usize) -> Self {
        Self::new(max_pool_size)
    }

    /// Get a PathBuf from the pool or create new one
    pub fn get(&mut self) -> PathBuf {
        if let Some(mut path) = self.pool.pop_front() {
            path.clear();
            self.stats.reused += 1;
            path
        } else {
            self.stats.allocated += 1;
            PathBuf::new()
        }
    }

    /// Return a PathBuf to the pool for reuse
    pub fn put(&mut self, mut path: PathBuf) {
        if self.pool.len() < self.max_pool_size {
            path.clear();
            self.pool.push_back(path);
            self.stats.returned += 1;
        } else {
            self.stats.discarded += 1;
        }
    }

    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    pub fn utilization(&self) -> PoolUtilization {
        PoolUtilization {
            current_size: self.pool.len(),
            max_size: self.max_pool_size,
            reuse_rate: self.stats.reuse_rate(),
        }
    }
}

impl Default for PathBufPool {
    fn default() -> Self {
        Self::new(500) // Pool up to 500 path buffers
    }
}

/// HashMap pool for metadata and other maps
pub struct HashMapPool<K, V> {
    pool: VecDeque<HashMap<K, V>>,
    max_pool_size: usize,
    stats: PoolStats,
}

impl<K, V> HashMapPool<K, V> {
    pub fn new(max_pool_size: usize) -> Self {
        let validated_size = if max_pool_size < MIN_POOL_SIZE || max_pool_size > MAX_POOL_SIZE {
            200 // Default safe size
        } else {
            max_pool_size
        };
        Self {
            pool: VecDeque::with_capacity(validated_size),
            max_pool_size: validated_size,
            stats: PoolStats::default(),
        }
    }

    pub fn with_config(max_pool_size: usize) -> Self {
        Self::new(max_pool_size)
    }

    /// Get a HashMap from the pool or create new one
    pub fn get(&mut self) -> HashMap<K, V> {
        if let Some(mut map) = self.pool.pop_front() {
            map.clear();
            self.stats.reused += 1;
            map
        } else {
            self.stats.allocated += 1;
            HashMap::new()
        }
    }

    /// Return a HashMap to the pool for reuse
    pub fn put(&mut self, mut map: HashMap<K, V>) {
        if self.pool.len() < self.max_pool_size {
            map.clear();
            self.pool.push_back(map);
            self.stats.returned += 1;
        } else {
            self.stats.discarded += 1;
        }
    }

    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    pub fn utilization(&self) -> PoolUtilization {
        PoolUtilization {
            current_size: self.pool.len(),
            max_size: self.max_pool_size,
            reuse_rate: self.stats.reuse_rate(),
        }
    }
}

/// Pool performance statistics
#[derive(Debug, Default, Clone)]
pub struct PoolStats {
    pub allocated: u64,
    pub reused: u64,
    pub returned: u64,
    pub discarded: u64,
    pub evicted: u64,
}

impl PoolStats {
    pub fn reuse_rate(&self) -> f64 {
        let total_requests = self.allocated.checked_add(self.reused).unwrap_or(u64::MAX);
        if total_requests == 0 {
            0.0
        } else {
            self.reused as f64 / total_requests as f64
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Pool Stats:\n\
             - Allocated: {}\n\
             - Reused: {}\n\
             - Returned: {}\n\
             - Discarded: {}\n\
             - Evicted: {}\n\
             - Reuse rate: {:.1}%",
            self.allocated,
            self.reused,
            self.returned,
            self.discarded,
            self.evicted,
            self.reuse_rate() * 100.0
        )
    }
}

/// Pool utilization metrics
#[derive(Debug)]
pub struct PoolUtilization {
    pub current_size: usize,
    pub max_size: usize,
    pub reuse_rate: f64,
}

impl PoolUtilization {
    pub fn utilization_percentage(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            (self.current_size as f64 / self.max_size as f64) * 100.0
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Pool Utilization:\n\
             - Current size: {}/{} ({:.1}%)\n\
             - Reuse rate: {:.1}%",
            self.current_size,
            self.max_size,
            self.utilization_percentage(),
            self.reuse_rate * 100.0
        )
    }
}

/// Thread-safe memory pool manager
#[derive(Clone)]
pub struct MemoryPoolManager {
    finding_pool: Arc<Mutex<FindingPool>>,
    string_pool: Arc<Mutex<StringPool>>,
    path_pool: Arc<Mutex<PathBufPool>>,
    metadata_pool: Arc<Mutex<HashMapPool<String, serde_json::Value>>>,
}

impl MemoryPoolManager {
    pub fn new() -> Self {
        Self {
            finding_pool: Arc::new(Mutex::new(FindingPool::default())),
            string_pool: Arc::new(Mutex::new(StringPool::default())),
            path_pool: Arc::new(Mutex::new(PathBufPool::default())),
            metadata_pool: Arc::new(Mutex::new(HashMapPool::new(200))),
        }
    }

    pub fn with_config(
        findings_pool_size: usize,
        strings_pool_size: usize,
        pathbuf_pool_size: usize,
        hashmap_pool_size: usize,
    ) -> Self {
        Self {
            finding_pool: Arc::new(Mutex::new(FindingPool::with_config(findings_pool_size))),
            string_pool: Arc::new(Mutex::new(StringPool::with_config(strings_pool_size))),
            path_pool: Arc::new(Mutex::new(PathBufPool::with_config(pathbuf_pool_size))),
            metadata_pool: Arc::new(Mutex::new(HashMapPool::with_config(hashmap_pool_size))),
        }
    }

    pub fn finding_pool(&self) -> Arc<Mutex<FindingPool>> {
        Arc::clone(&self.finding_pool)
    }

    pub fn string_pool(&self) -> Arc<Mutex<StringPool>> {
        Arc::clone(&self.string_pool)
    }

    pub fn path_pool(&self) -> Arc<Mutex<PathBufPool>> {
        Arc::clone(&self.path_pool)
    }

    pub fn metadata_pool(&self) -> Arc<Mutex<HashMapPool<String, serde_json::Value>>> {
        Arc::clone(&self.metadata_pool)
    }

    /// Get comprehensive memory pool statistics
    pub fn stats(&self) -> MemoryPoolStats {
        let finding_stats = self
            .finding_pool
            .lock()
            .unwrap_or_else(|e| {
                tracing::warn!("Finding pool mutex poisoned: {}", e);
                e.into_inner()
            })
            .stats()
            .clone();
        let string_stats = self
            .string_pool
            .lock()
            .unwrap_or_else(|e| {
                tracing::warn!("String pool mutex poisoned: {}", e);
                e.into_inner()
            })
            .stats()
            .clone();
        let path_stats = self
            .path_pool
            .lock()
            .unwrap_or_else(|e| {
                tracing::warn!("Path pool mutex poisoned: {}", e);
                e.into_inner()
            })
            .stats()
            .clone();
        let metadata_stats = self
            .metadata_pool
            .lock()
            .unwrap_or_else(|e| {
                tracing::warn!("Metadata pool mutex poisoned: {}", e);
                e.into_inner()
            })
            .stats()
            .clone();

        MemoryPoolStats {
            finding_stats,
            string_stats,
            path_stats,
            metadata_stats,
        }
    }

    /// Get overall memory savings estimate
    pub fn memory_savings_estimate(&self) -> MemorySavings {
        let stats = self.stats();

        // Estimate memory savings based on reuse rates with overflow protection
        let finding_size = std::mem::size_of::<Finding>() as f64;
        let finding_savings = stats
            .finding_stats
            .reused
            .checked_mul(std::mem::size_of::<Finding>() as u64)
            .unwrap_or(u64::MAX) as f64
            * finding_size
            / (1024.0 * 1024.0);
        let string_savings = (stats.string_stats.reused as f64 * 32.0) / (1024.0 * 1024.0); // Estimate 32 bytes per string
        let path_size = std::mem::size_of::<PathBuf>() as f64;
        let path_savings = stats
            .path_stats
            .reused
            .checked_mul(std::mem::size_of::<PathBuf>() as u64)
            .unwrap_or(u64::MAX) as f64
            * path_size
            / (1024.0 * 1024.0);
        let metadata_savings = (stats.metadata_stats.reused as f64 * 64.0) / (1024.0 * 1024.0); // Estimate 64 bytes per map

        let total_mb_saved = finding_savings + string_savings + path_savings + metadata_savings;

        MemorySavings {
            total_mb_saved,
            finding_mb_saved: finding_savings,
            string_mb_saved: string_savings,
            path_mb_saved: path_savings,
            metadata_mb_saved: metadata_savings,
        }
    }
}

impl Default for MemoryPoolManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive memory pool statistics
#[derive(Debug)]
pub struct MemoryPoolStats {
    pub finding_stats: PoolStats,
    pub string_stats: PoolStats,
    pub path_stats: PoolStats,
    pub metadata_stats: PoolStats,
}

impl MemoryPoolStats {
    pub fn overall_reuse_rate(&self) -> f64 {
        let total_reused = self
            .finding_stats
            .reused
            .checked_add(self.string_stats.reused)
            .and_then(|sum| sum.checked_add(self.path_stats.reused))
            .and_then(|sum| sum.checked_add(self.metadata_stats.reused))
            .unwrap_or(u64::MAX);
        let total_allocated = self
            .finding_stats
            .allocated
            .checked_add(self.string_stats.allocated)
            .and_then(|sum| sum.checked_add(self.path_stats.allocated))
            .and_then(|sum| sum.checked_add(self.metadata_stats.allocated))
            .unwrap_or(u64::MAX);

        let total_requests = total_allocated
            .checked_add(total_reused)
            .unwrap_or(u64::MAX);

        if total_requests == 0 {
            0.0
        } else {
            total_reused as f64 / total_requests as f64
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Memory Pool Statistics:\n\
             Finding Pool: {}\n\
             String Pool: {}\n\
             Path Pool: {}\n\
             Metadata Pool: {}\n\
             Overall reuse rate: {:.1}%",
            self.finding_stats.report(),
            self.string_stats.report(),
            self.path_stats.report(),
            self.metadata_stats.report(),
            self.overall_reuse_rate() * 100.0
        )
    }
}

/// Memory savings estimation
#[derive(Debug)]
pub struct MemorySavings {
    pub total_mb_saved: f64,
    pub finding_mb_saved: f64,
    pub string_mb_saved: f64,
    pub path_mb_saved: f64,
    pub metadata_mb_saved: f64,
}

impl MemorySavings {
    pub fn report(&self) -> String {
        format!(
            "Memory Savings Estimate:\n\
             - Total: {:.2} MB saved\n\
             - Findings: {:.2} MB\n\
             - Strings: {:.2} MB\n\
             - Paths: {:.2} MB\n\
             - Metadata: {:.2} MB",
            self.total_mb_saved,
            self.finding_mb_saved,
            self.string_mb_saved,
            self.path_mb_saved,
            self.metadata_mb_saved
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;

    #[test]
    fn test_finding_pool() {
        let mut pool = FindingPool::new(5);

        // Get a finding
        let finding = pool.get();
        assert_eq!(pool.stats().allocated, 1);
        assert_eq!(pool.stats().reused, 0);

        // Return it
        pool.put(finding);
        assert_eq!(pool.stats().returned, 1);

        // Get it again (should reuse)
        let finding2 = pool.get();
        assert_eq!(pool.stats().reused, 1);
        assert_eq!(pool.stats().allocated, 1);
    }

    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new(10);

        // Intern a string
        let s1 = pool.get("hello");
        assert_eq!(pool.stats().allocated, 1);

        // Intern the same string (should reuse)
        let s2 = pool.get("hello");
        assert_eq!(pool.stats().reused, 1);
        assert_eq!(pool.stats().allocated, 1);
        assert!(Arc::ptr_eq(&s1, &s2));

        // Intern different string
        let s3 = pool.get("world");
        assert_eq!(pool.stats().allocated, 2);
    }

    #[test]
    fn test_path_buf_pool() {
        let mut pool = PathBufPool::new(5);

        // Get a path buffer
        let path = pool.get();
        assert_eq!(pool.stats().allocated, 1);

        // Return it
        pool.put(path);
        assert_eq!(pool.stats().returned, 1);

        // Get it again (should reuse)
        let path2 = pool.get();
        assert_eq!(pool.stats().reused, 1);
    }

    #[test]
    fn test_memory_pool_manager() {
        let manager = MemoryPoolManager::new();

        // Test concurrent access
        let manager_clone = manager.clone();
        let handle = std::thread::spawn(move || {
            let _finding = manager_clone
                .finding_pool()
                .lock()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Finding pool mutex poisoned in test");
                    poisoned.into_inner()
                })
                .get();
            let _string = manager_clone
                .string_pool()
                .lock()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("String pool mutex poisoned in test");
                    poisoned.into_inner()
                })
                .get("test");
        });

        let _finding = manager
            .finding_pool()
            .lock()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Finding pool mutex poisoned in test");
                poisoned.into_inner()
            })
            .get();
        let _string = manager
            .string_pool()
            .lock()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("String pool mutex poisoned in test");
                poisoned.into_inner()
            })
            .get("test");

        handle
            .join()
            .expect("Failed to join thread in test_memory_pool_manager");

        let stats = manager.stats();
        assert!(stats.finding_stats.allocated >= 1);
        assert!(stats.string_stats.allocated >= 1);
    }
}
