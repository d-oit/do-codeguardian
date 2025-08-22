use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Memory pool for efficient object reuse
pub struct MemoryPool<T> {
    pool: Arc<Mutex<VecDeque<T>>>,
    max_size: usize,
    create_fn: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> MemoryPool<T> {
    /// Create a new memory pool
    pub fn new<F>(max_size: usize, create_fn: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
            create_fn: Box::new(create_fn),
        }
    }

    /// Get an object from the pool or create a new one
    pub fn get(&self) -> T {
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(obj) = pool.pop_front() {
                return obj;
            }
        }
        (self.create_fn)()
    }

    /// Return an object to the pool for reuse
    pub fn put(&self, obj: T) {
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.max_size {
                pool.push_back(obj);
            }
        }
    }

    /// Get pool statistics
    pub fn stats(&self) -> MemoryPoolStats {
        if let Ok(pool) = self.pool.lock() {
            MemoryPoolStats {
                current_size: pool.len(),
                max_size: self.max_size,
            }
        } else {
            MemoryPoolStats {
                current_size: 0,
                max_size: self.max_size,
            }
        }
    }
}

#[derive(Debug)]
pub struct MemoryPoolStats {
    pub current_size: usize,
    pub max_size: usize,
}

/// String buffer pool for efficient string operations
pub type StringBufferPool = MemoryPool<String>;

impl StringBufferPool {
    /// Create a string buffer pool with default capacity
    pub fn new_string_pool(capacity: usize) -> Self {
        Self::new(100, move || String::with_capacity(capacity))
    }

    /// Get a string buffer and clear it
    pub fn get_buffer(&self) -> String {
        let mut buffer = self.get();
        buffer.clear();
        buffer
    }
}

/// Vector pool for efficient vector operations
pub struct VectorPool<T> {
    pool: Arc<Mutex<VecDeque<Vec<T>>>>,
    max_size: usize,
    default_capacity: usize,
}

impl<T> VectorPool<T> {
    /// Create a new vector pool
    pub fn new(max_size: usize, default_capacity: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
            default_capacity,
        }
    }

    /// Get a vector from the pool or create a new one
    pub fn get(&self) -> Vec<T> {
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(mut vec) = pool.pop_front() {
                vec.clear();
                return vec;
            }
        }
        Vec::with_capacity(self.default_capacity)
    }

    /// Return a vector to the pool for reuse
    pub fn put(&self, mut vec: Vec<T>) {
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.max_size {
                vec.clear();
                vec.shrink_to(self.default_capacity);
                pool.push_back(vec);
            }
        }
    }

    /// Get pool statistics
    pub fn stats(&self) -> MemoryPoolStats {
        if let Ok(pool) = self.pool.lock() {
            MemoryPoolStats {
                current_size: pool.len(),
                max_size: self.max_size,
            }
        } else {
            MemoryPoolStats {
                current_size: 0,
                max_size: self.max_size,
            }
        }
    }
}

/// Findings vector pool for efficient memory reuse
pub type FindingsPool = VectorPool<crate::types::Finding>;

impl FindingsPool {
    /// Create a findings pool with appropriate capacity
    pub fn new_findings_pool() -> Self {
        Self::new(50, 100) // Pool of 50 vectors, each with capacity of 100 findings
    }
}

/// Global memory pools for common allocations
pub struct GlobalMemoryPools {
    pub string_buffers: StringBufferPool,
    pub findings: FindingsPool,
}

impl GlobalMemoryPools {
    /// Create global memory pools with default settings
    pub fn new() -> Self {
        Self {
            string_buffers: StringBufferPool::new_string_pool(1024),
            findings: FindingsPool::new_findings_pool(),
        }
    }
}

impl Default for GlobalMemoryPools {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalMemoryPools {

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> GlobalMemoryStats {
        GlobalMemoryStats {
            string_buffer_stats: self.string_buffers.stats(),
            findings_stats: self.findings.stats(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalMemoryStats {
    pub string_buffer_stats: MemoryPoolStats,
    pub findings_stats: MemoryPoolStats,
}

/// Thread-local memory pools for better performance
pub mod thread_local_pools {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static STRING_POOL: RefCell<Option<StringBufferPool>> = const { RefCell::new(None) };
        static FINDINGS_POOL: RefCell<Option<FindingsPool>> = const { RefCell::new(None) };
    }

    /// Initialize thread-local pools
    pub fn init() {
        STRING_POOL.with(|pool| {
            *pool.borrow_mut() = Some(StringBufferPool::new_string_pool(1024));
        });
        FINDINGS_POOL.with(|pool| {
            *pool.borrow_mut() = Some(FindingsPool::new_findings_pool());
        });
    }

    /// Get a string buffer from thread-local pool
    pub fn get_string_buffer() -> String {
        STRING_POOL.with(|pool| {
            if let Some(ref pool) = *pool.borrow() {
                pool.get_buffer()
            } else {
                String::with_capacity(1024)
            }
        })
    }

    /// Return a string buffer to thread-local pool
    pub fn put_string_buffer(buffer: String) {
        STRING_POOL.with(|pool| {
            if let Some(ref pool) = *pool.borrow() {
                pool.put(buffer);
            }
        });
    }

    /// Get a findings vector from thread-local pool
    pub fn get_findings_vec() -> Vec<crate::types::Finding> {
        FINDINGS_POOL.with(|pool| {
            if let Some(ref pool) = *pool.borrow() {
                pool.get()
            } else {
                Vec::with_capacity(100)
            }
        })
    }

    /// Return a findings vector to thread-local pool
    pub fn put_findings_vec(vec: Vec<crate::types::Finding>) {
        FINDINGS_POOL.with(|pool| {
            if let Some(ref pool) = *pool.borrow() {
                pool.put(vec);
            }
        });
    }
}

/// Memory-efficient string operations
pub mod string_utils {
    use super::thread_local_pools;

    /// Format a string using a reusable buffer
    pub fn format_buffered(args: std::fmt::Arguments) -> String {
        use std::fmt::Write;
        let mut buffer = thread_local_pools::get_string_buffer();
        let _ = buffer.write_fmt(args);
        buffer
    }

    /// Create a string with pre-allocated capacity
    pub fn string_with_capacity(capacity: usize) -> String {
        String::with_capacity(capacity)
    }

    /// Join strings efficiently
    pub fn join_strings(strings: &[&str], separator: &str) -> String {
        if strings.is_empty() {
            return String::new();
        }

        let total_len = strings.iter().map(|s| s.len()).sum::<usize>()
            + separator.len() * (strings.len().saturating_sub(1));

        let mut result = String::with_capacity(total_len);
        result.push_str(strings[0]);

        for s in &strings[1..] {
            result.push_str(separator);
            result.push_str(s);
        }

        result
    }
}
