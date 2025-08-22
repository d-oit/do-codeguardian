pub mod git;
pub mod progress;
pub mod security;

// Performance-related modules have been moved to src/performance/
// Re-export them for backward compatibility
pub use crate::performance::{caching as memory_pool, parallelism as adaptive_parallelism};
