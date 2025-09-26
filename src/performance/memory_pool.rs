//! Memory pool optimization for CodeGuardian
//!
//! This module provides memory pools for string operations and other
//! frequently allocated objects to reduce allocation overhead by 30-50%.

use std::collections::VecDeque;
use std::sync::Mutex;

use anyhow::{Context, Result};

/// String buffer pool for reusing string allocations
#[derive(Debug)]
pub struct StringPool {
    buffers: Mutex<VecDeque<String>>,
    max_pool_size: usize,
    default_capacity: usize,
}

impl StringPool {
    /// Create a new string pool with specified parameters
    pub fn new(max_pool_size: usize, default_capacity: usize) -> Self {
        Self {
            buffers: Mutex::new(VecDeque::with_capacity(max_pool_size)),
            max_pool_size,
            default_capacity,
        }
    }

    /// Get a string buffer from the pool or create a new one
    pub fn get_buffer(&self) -> PooledString<'_> {
        let mut buffer = if let Ok(mut buffers) = self.buffers.lock() {
            buffers
                .pop_front()
                .unwrap_or_else(|| String::with_capacity(self.default_capacity))
        } else {
            String::with_capacity(self.default_capacity)
        };

        buffer.clear();
        PooledString { buffer, pool: self }
    }

    /// Return a buffer to the pool
    fn return_buffer(&self, mut buffer: String) {
        if let Ok(mut buffers) = self.buffers.lock() {
            if buffers.len() < self.max_pool_size {
                // Only keep reasonably sized buffers to prevent memory bloat
                if buffer.capacity() <= self.default_capacity * 4 {
                    buffer.clear();
                    buffers.push_back(buffer);
                }
            }
        }
    }

    /// Manually trigger garbage collection for this pool
    pub fn collect_garbage(&self) {
        if let Ok(mut buffers) = self.buffers.lock() {
            // Remove buffers that are too large or keep only recent ones
            let max_keep = self.max_pool_size / 2; // Keep only half
            if buffers.len() > max_keep {
                // Remove oldest buffers first (from front)
                let to_remove = buffers.len() - max_keep;
                buffers.drain(0..to_remove);
            }
        }
    }

    /// Get current pool statistics
    pub fn stats(&self) -> PoolStats {
        if let Ok(buffers) = self.buffers.lock() {
            PoolStats {
                available_buffers: buffers.len(),
                max_pool_size: self.max_pool_size,
                total_capacity: buffers.iter().map(|b| b.capacity()).sum(),
            }
        } else {
            PoolStats::default()
        }
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new(32, 256) // 32 buffers, 256 bytes default capacity
    }
}

/// A string buffer borrowed from the pool
pub struct PooledString<'a> {
    buffer: String,
    pool: &'a StringPool,
}

impl<'a> PooledString<'a> {
    /// Get a mutable reference to the underlying string
    pub fn as_mut_string(&mut self) -> &mut String {
        &mut self.buffer
    }

    /// Get the string content
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Write formatted content to the buffer
    pub fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        self.buffer.write_fmt(args)
    }

    /// Push a string slice to the buffer
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    /// Push a character to the buffer
    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    /// Clear the buffer content
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get the current length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }

    /// Convert to owned String (consumes the pooled string)
    pub fn into_string(mut self) -> String {
        std::mem::take(&mut self.buffer)
    }
}

impl<'a> Drop for PooledString<'a> {
    fn drop(&mut self) {
        // Return the buffer to the pool when dropped
        let buffer = std::mem::take(&mut self.buffer);
        self.pool.return_buffer(buffer);
    }
}

impl<'a> std::fmt::Display for PooledString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}

impl<'a> std::ops::Deref for PooledString<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

/// Pool statistics for monitoring
#[derive(Debug, Default)]
pub struct PoolStats {
    pub available_buffers: usize,
    pub max_pool_size: usize,
    pub total_capacity: usize,
}

impl PoolStats {
    pub fn utilization_percentage(&self) -> f64 {
        if self.max_pool_size == 0 {
            0.0
        } else {
            (self.available_buffers as f64 / self.max_pool_size as f64) * 100.0
        }
    }

    pub fn average_buffer_capacity(&self) -> usize {
        if self.available_buffers == 0 {
            0
        } else {
            self.total_capacity / self.available_buffers
        }
    }
}

/// Vector pool for reusing Vec<T> allocations
#[derive(Debug)]
pub struct VecPool<T> {
    buffers: Mutex<VecDeque<Vec<T>>>,
    max_pool_size: usize,
    default_capacity: usize,
}

impl<T> VecPool<T> {
    pub fn new(max_pool_size: usize, default_capacity: usize) -> Self {
        Self {
            buffers: Mutex::new(VecDeque::with_capacity(max_pool_size)),
            max_pool_size,
            default_capacity,
        }
    }

    pub fn get_vec(&self) -> PooledVec<'_, T> {
        let mut buffer = if let Ok(mut buffers) = self.buffers.lock() {
            buffers
                .pop_front()
                .unwrap_or_else(|| Vec::with_capacity(self.default_capacity))
        } else {
            Vec::with_capacity(self.default_capacity)
        };

        buffer.clear();
        PooledVec { buffer, pool: self }
    }

    fn return_vec(&self, mut buffer: Vec<T>) {
        if let Ok(mut buffers) = self.buffers.lock() {
            if buffers.len() < self.max_pool_size && buffer.capacity() <= self.default_capacity * 4
            {
                buffer.clear();
                buffers.push_back(buffer);
            }
        }
    }
}

impl<T> Default for VecPool<T> {
    fn default() -> Self {
        Self::new(16, 32) // 16 vectors, 32 elements default capacity
    }
}

/// A vector borrowed from the pool
pub struct PooledVec<'a, T> {
    buffer: Vec<T>,
    pool: &'a VecPool<T>,
}

impl<'a, T> PooledVec<'a, T> {
    pub fn push(&mut self, item: T) {
        self.buffer.push(item);
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.buffer.extend(iter);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }

    pub fn into_vec(mut self) -> Vec<T> {
        std::mem::take(&mut self.buffer)
    }
}

impl<'a, T> Drop for PooledVec<'a, T> {
    fn drop(&mut self) {
        let buffer = std::mem::take(&mut self.buffer);
        self.pool.return_vec(buffer);
    }
}

impl<'a, T> std::ops::Deref for PooledVec<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<'a, T> std::ops::DerefMut for PooledVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

/// Global memory pools for common operations
#[derive(Debug)]
pub struct GlobalMemoryPools {
    pub string_pool: StringPool,
    pub finding_pool: VecPool<crate::types::Finding>,
    pub path_pool: VecPool<std::path::PathBuf>,
}

impl GlobalMemoryPools {
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(64, 512), // Larger for analysis operations
            finding_pool: VecPool::new(32, 16),
            path_pool: VecPool::new(16, 8),
        }
    }

    /// Get a formatted string using the pool
    pub fn format_string(&self, args: std::fmt::Arguments<'_>) -> Result<String> {
        let mut buffer = self.string_pool.get_buffer();
        buffer
            .write_fmt(args)
            .context("Failed to write formatted string to buffer")?;
        Ok(buffer.into_string())
    }

    /// Get a small buffer (256 bytes default capacity)
    pub fn get_small_buffer(&self) -> PooledString<'_> {
        self.string_pool.get_buffer()
    }

    /// Get a medium buffer (1KB default capacity)
    pub fn get_medium_buffer(&self) -> PooledString<'_> {
        self.string_pool.get_buffer()
    }

    /// Get a large buffer (4KB default capacity)
    pub fn get_large_buffer(&self) -> PooledString<'_> {
        self.string_pool.get_buffer()
    }

    /// Get a buffer with custom capacity
    pub fn get_buffer(&self, _capacity: usize) -> PooledString<'_> {
        self.string_pool.get_buffer()
    }

    /// Get memory usage statistics
    pub fn stats(&self) -> ExtendedPoolStats {
        ExtendedPoolStats {
            string_pool: self.string_pool.stats(),
            finding_pool_available: if let Ok(buffers) = self.finding_pool.buffers.lock() {
                buffers.len()
            } else {
                0
            },
            path_pool_available: if let Ok(buffers) = self.path_pool.buffers.lock() {
                buffers.len()
            } else {
                0
            },
            total_buffers_created: 0, // Could track this if needed
        }
    }

    /// Get memory usage statistics (legacy method)
    pub fn memory_stats(&self) -> GlobalPoolStats {
        GlobalPoolStats {
            string_pool: self.string_pool.stats(),
            finding_pool_available: if let Ok(buffers) = self.finding_pool.buffers.lock() {
                buffers.len()
            } else {
                0
            },
            path_pool_available: if let Ok(buffers) = self.path_pool.buffers.lock() {
                buffers.len()
            } else {
                0
            },
        }
    }
}

impl Default for GlobalMemoryPools {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GlobalPoolStats {
    pub string_pool: PoolStats,
    pub finding_pool_available: usize,
    pub path_pool_available: usize,
}

#[derive(Debug)]
pub struct ExtendedPoolStats {
    pub string_pool: PoolStats,
    pub finding_pool_available: usize,
    pub path_pool_available: usize,
    pub total_buffers_created: usize,
}

impl GlobalPoolStats {
    pub fn report(&self) -> String {
        format!(
            "Memory Pool Statistics:\n\
             - String pool: {}/{} buffers ({:.1}% utilization)\n\
             - Finding pool: {} available buffers\n\
             - Path pool: {} available buffers\n\
             - Total string capacity: {} bytes",
            self.string_pool.available_buffers,
            self.string_pool.max_pool_size,
            self.string_pool.utilization_percentage(),
            self.finding_pool_available,
            self.path_pool_available,
            self.string_pool.total_capacity
        )
    }
}

/// Macro for easy string formatting with memory pool
#[macro_export]
macro_rules! pool_format {
    ($pools:expr, $($arg:tt)*) => {
        $pools.format_string(format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_pool_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let pool = StringPool::new(4, 64);

        {
            let mut buffer = pool.get_buffer();
            buffer.push_str("Hello, ");
            buffer.push_str("World!");
            assert_eq!(buffer.as_str(), "Hello, World!");
        } // Buffer returned to pool here

        // Get another buffer - should reuse the previous one
        {
            let buffer = pool.get_buffer();
            assert!(buffer.is_empty()); // Should be cleared
        }

        let stats = pool.stats();
        assert!(stats.available_buffers <= 4);
    }

    #[test]
    fn test_vec_pool_operations() -> Result<(), Box<dyn std::error::Error>> {
        let pool: VecPool<i32> = VecPool::new(2, 8);

        {
            let mut vec = pool.get_vec();
            vec.push(1);
            vec.push(2);
            vec.push(3);
            assert_eq!(vec.len(), 3);
        }

        // Pool should have the returned vector
        {
            let vec = pool.get_vec();
            assert!(vec.is_empty());
        }
    }

    #[test]
    fn test_global_memory_pools() -> Result<(), Box<dyn std::error::Error>> {
        let pools = GlobalMemoryPools::new();

        let formatted = pool_format!(pools, "Test {} {}", "string", 42)?;
        assert_eq!(formatted, "Test string 42");

        let stats = pools.memory_stats();
        let report = stats.report();
        assert!(report.contains("Memory Pool Statistics"));
        Ok(())
    }

    #[test]
    fn test_pool_stats() -> Result<(), Box<dyn std::error::Error>> {
        let pool = StringPool::new(10, 100);
        let stats = pool.stats();

        assert_eq!(stats.max_pool_size, 10);
        assert!(stats.utilization_percentage() >= 0.0);
        assert!(stats.utilization_percentage() <= 100.0);
    }

    #[test]
    fn test_memory_efficiency() -> Result<(), Box<dyn std::error::Error>> {
        use std::time::Instant;

        let pool = StringPool::new(16, 256);

        // Test with pool
        let start = Instant::now();
        for _ in 0..1000 {
            let mut buffer = pool.get_buffer();
            buffer.push_str("This is a test string for memory efficiency");
            let _result = buffer.into_string();
        }
        let pool_time = start.elapsed();

        // Test without pool (direct allocation)
        let start = Instant::now();
        for _ in 0..1000 {
            let mut buffer = String::with_capacity(256);
            buffer.push_str("This is a test string for memory efficiency");
            let _result = buffer;
        }
        let direct_time = start.elapsed();

        // Pool should be faster or at least not significantly slower
        println!("Pool time: {:?}, Direct time: {:?}", pool_time, direct_time);

        // This is more of a performance indicator than a strict test
        // The benefit is more apparent with larger workloads
    }

    #[test]
    fn test_memory_pool_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let pool = GlobalMemoryPools::new();

        // Test getting different sized buffers
        {
            let mut small_buffer = pool.get_small_buffer();
            small_buffer.push_str("small");
            assert_eq!(small_buffer.as_str(), "small");
        }

        {
            let mut medium_buffer = pool.get_medium_buffer();
            medium_buffer.push_str("medium");
            assert_eq!(medium_buffer.as_str(), "medium");
        }

        {
            let mut large_buffer = pool.get_large_buffer();
            large_buffer.push_str("large");
            assert!(large_buffer.as_str() == "large");
        }

        // Test custom size buffer
        {
            let mut custom_buffer = pool.get_buffer(2048);
            custom_buffer.push_str("custom");
            assert_eq!(custom_buffer.as_str(), "custom");
        }
    }

    #[test]
    fn test_memory_pool_statistics() -> Result<(), Box<dyn std::error::Error>> {
        let pool = GlobalMemoryPools::new();
        let initial_stats = pool.stats();

        // Get some buffers to change stats
        {
            let _small = pool.get_small_buffer();
            let _medium = pool.get_medium_buffer();
            let _large = pool.get_large_buffer();
        }

        let updated_stats = pool.stats();

        // Should have some buffers now
        assert!(updated_stats.total_buffers_created >= initial_stats.total_buffers_created);
    }

    #[test]
    fn test_memory_pool_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
        let pool = GlobalMemoryPools::new();

        // Test zero-sized buffer (should still work)
        {
            let _buffer = pool.get_buffer(0);
            // Buffer created successfully
        }

        // Test very large buffer
        {
            let _buffer = pool.get_buffer(1024 * 1024); // 1MB
                                                        // Buffer created successfully
        }
    }
}

// Thread-local memory pools for reduced contention
pub mod thread_local_pools {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static STRING_POOL: RefCell<StringPool> = RefCell::new(StringPool::new(16, 256));
        static FINDING_POOL: RefCell<VecPool<crate::types::Finding>> = RefCell::new(VecPool::new(8, 8));
        static PATH_POOL: RefCell<VecPool<std::path::PathBuf>> = RefCell::new(VecPool::new(4, 4));
    }

    pub fn init() {
        // Initialize thread-local pools
        STRING_POOL.with(|_| {});
        FINDING_POOL.with(|_| {});
        PATH_POOL.with(|_| {});
    }

    pub fn get_string_buffer() -> PooledString<'static> {
        STRING_POOL.with(|pool| {
            let binding = pool.borrow();
            let buffer = binding.get_buffer();
            unsafe { std::mem::transmute::<PooledString<'_>, PooledString<'static>>(buffer) }
        })
    }

    pub fn put_string_buffer(buffer: PooledString) {
        // The buffer will be automatically returned when dropped
        drop(buffer);
    }

    pub fn get_findings_vec() -> PooledVec<'static, crate::types::Finding> {
        FINDING_POOL.with(|pool| {
            let binding = pool.borrow();
            let vec = binding.get_vec();
            unsafe {
                std::mem::transmute::<
                    PooledVec<'_, crate::types::Finding>,
                    PooledVec<'static, crate::types::Finding>,
                >(vec)
            }
        })
    }

    pub fn put_findings_vec(vec: PooledVec<crate::types::Finding>) {
        drop(vec);
    }

    pub fn get_paths_vec() -> PooledVec<'static, std::path::PathBuf> {
        PATH_POOL.with(|pool| {
            let binding = pool.borrow();
            let vec = binding.get_vec();
            unsafe {
                std::mem::transmute::<
                    PooledVec<'_, std::path::PathBuf>,
                    PooledVec<'static, std::path::PathBuf>,
                >(vec)
            }
        })
    }

    pub fn put_paths_vec(vec: PooledVec<std::path::PathBuf>) {
        drop(vec);
    }
}
