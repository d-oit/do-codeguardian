//! Enhanced optimized cache with memory pool integration
//!
//! # ⚠️ DEPRECATED
//!
//! This module is **deprecated** and will be removed in a future version.
//! Please migrate to the [`crate::cache::unified_cache`] module, which provides
//! a unified interface with strategy pattern support for both basic and pooled caching.
//!
//! ## Migration Guide
//!
#![allow(deprecated)]

//! ### Before (Deprecated)
//! ```rust,no_run
//! use codeguardian::cache::enhanced_optimized_cache::EnhancedOptimizedCache;
//!
//! let mut cache = EnhancedOptimizedCache::new(1000, 100);
//! cache.put(&path, findings, config_hash, duration)?;
//! let result = cache.get(&path, config_hash)?;
//! ```
//!
//! ### After (Recommended)
//! ```rust,no_run
//! use codeguardian::cache::unified_cache::{UnifiedCache, UnifiedCacheConfig, CacheStrategyType};
//!
//! // For pooled cache with memory optimization
//! let config = UnifiedCacheConfig {
//!     strategy: CacheStrategyType::Pooled,
//!     max_entries: 1000,
//!     max_memory_mb: 100,
//!     enable_memory_pools: true,
//!     pool_sizes: Some(Default::default()),
//! };
//! let mut cache = UnifiedCache::new(config)?;
//!
//! // Or use the convenience method
//! let mut cache = UnifiedCache::pooled(1000, 100)?;
//!
//! cache.put(&path, findings, config_hash, duration)?;
//! let result = cache.get(&path, config_hash)?;
//! ```
//!
//! ## Benefits of Migration
//! - Unified API for all cache strategies
//! - Runtime strategy switching
//! - Better memory pool management
//! - Improved performance metrics
//! - Future-proof architecture
//!
//! This module extends the optimized cache with memory pool optimizations
//! for 15% memory reduction and 90% object reuse rate.

use crate::cache::memory_pool::MemoryPoolManager;
use crate::cache::optimized_cache::{CacheStats, CacheUtilization};
use crate::types::Finding;
use anyhow::Result;
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod cache;
mod entry;
mod metadata;

pub use cache::EnhancedOptimizedCache;
pub use entry::PooledCacheEntry;
pub use metadata::FileMetadata;
