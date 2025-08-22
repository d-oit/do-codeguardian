//! Unified performance optimization module
//!
//! This module consolidates all performance-related functionality into a well-organized
//! hierarchy with clear separation of concerns. It provides backward compatibility
//! through re-exports while offering improved maintainability and organization.

// Sub-modules
pub mod caching;
pub mod engine;
pub mod monitoring;
pub mod parallelism;

// Re-exports for backward compatibility
// These allow existing code to continue working without changes

// From engine module
#[allow(unused_imports)]
pub use engine::{
    AdaptiveAnalysisScheduler, LargeCodebaseIterator, OptimizedFileProcessor,
    OptimizedPatternMatcher, PerformanceEngine, PerformanceMetrics,
};

// From caching module
#[allow(unused_imports)]
pub use caching::{
    FindingsPool, GlobalMemoryPools, MemoryPool, MemoryPoolStats, StringBufferPool, VectorPool,
};

// From parallelism module
#[allow(unused_imports)]
pub use parallelism::{
    AdaptiveParallelismController, AdaptiveParallelismMetrics, SystemLoad, SystemLoadMonitor,
};

// From monitoring module
#[allow(unused_imports)]
pub use monitoring::{
    create_performance_dashboard, create_performance_monitor, AlertSeverity, AlertType,
    PerformanceAlert, PerformanceDashboard, PerformanceMetrics as MonitoringMetrics,
    PerformanceMonitor, PerformanceSummary,
};

// Additional re-exports for common types
#[allow(unused_imports)]
pub use caching::thread_local_pools;
#[allow(unused_imports)]
pub use engine::io_optimization;
#[allow(unused_imports)]
pub use engine::monitoring as engine_monitoring;
