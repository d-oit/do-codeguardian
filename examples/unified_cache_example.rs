//! Example demonstrating the unified cache architecture
//!
//! This example shows how to use the Strategy pattern-based unified cache
//! that consolidates basic and pooled cache implementations.

use anyhow::Result;
use do_codeguardian::cache::unified_cache::{
    CacheStrategyType, MemoryPoolSizes, UnifiedCache, UnifiedCacheConfig,
};
use do_codeguardian::types::Finding;
use std::path::Path;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Unified Cache Architecture Example ===\n");

    // Create a temporary directory for testing
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("example.rs");

    // Create some test content
    std::fs::write(&test_file, "fn example() { println!(\"Hello, World!\"); }")?;

    // Example 1: Basic Cache Strategy
    println!("1. Basic Cache Strategy");
    let mut basic_cache = UnifiedCache::basic(100, 50)?; // 100 entries, 50MB

    let findings = vec![Finding {
        id: "test-finding".to_string(),
        analyzer: "example".to_string(),
        rule: "test-rule".to_string(),
        severity: do_codeguardian::types::Severity::Info,
        file: test_file.clone(),
        line: 1,
        column: None,
        message: "Example finding".to_string(),
        description: Some("This is an example finding".to_string()),
        suggestion: Some("Consider improving this code".to_string()),
        category: Some("Example".to_string()),
        metadata: std::collections::HashMap::new(),
    }];

    // Cache some findings
    basic_cache
        .put(&test_file, findings.clone(), "config_hash_123", 150)
        .await?;
    println!("✓ Cached findings with basic strategy");

    // Retrieve cached findings
    if let Some(cached) = basic_cache.get(&test_file, "config_hash_123").await? {
        println!("✓ Retrieved {} cached findings", cached.len());
    }

    // Show cache stats
    let stats = basic_cache.stats();
    println!(
        "  Cache stats: {} requests, {:.1}% hit rate",
        stats.total_requests,
        stats.hit_rate() * 100.0
    );

    println!();

    // Example 2: Pooled Cache Strategy
    println!("2. Pooled Cache Strategy (with memory pools)");
    let pooled_config = UnifiedCacheConfig {
        strategy: CacheStrategyType::Pooled,
        max_entries: 200,
        max_memory_mb: 100,
        enable_memory_pools: true,
        pool_sizes: Some(MemoryPoolSizes {
            findings_pool_size: 1000,
            strings_pool_size: 5000,
            pathbuf_pool_size: 500,
            hashmap_pool_size: 200,
        }),
    };

    let mut pooled_cache = UnifiedCache::new(pooled_config)?;

    // Cache findings with pooled strategy
    pooled_cache
        .put(&test_file, findings, "config_hash_456", 200)
        .await?;
    println!("✓ Cached findings with pooled strategy");

    // Show memory pool stats
    if let Some(pool_stats) = pooled_cache.memory_pool_stats() {
        println!(
            "  Memory pool stats: {:.1}% reuse rate",
            pool_stats.overall_reuse_rate() * 100.0
        );
    }

    if let Some(savings) = pooled_cache.memory_savings() {
        println!("  Memory savings: {:.2} MB", savings.total_mb_saved);
    }

    println!();

    // Example 3: Runtime Strategy Switching
    println!("3. Runtime Strategy Switching");
    let new_config = UnifiedCacheConfig {
        strategy: CacheStrategyType::Basic,
        max_entries: 50,
        max_memory_mb: 25,
        enable_memory_pools: false,
        pool_sizes: None,
    };

    pooled_cache.switch_strategy(new_config)?;
    println!("✓ Switched from pooled to basic strategy at runtime");
    println!("  Current strategy: {}", pooled_cache.strategy_name());

    println!();

    // Example 4: Cache Utilization
    println!("4. Cache Utilization");
    let utilization = pooled_cache.utilization();
    println!(
        "  Entry utilization: {}/{} ({:.1}%)",
        utilization.entry_count,
        utilization.max_entries,
        utilization.entry_utilization_percentage()
    );
    println!(
        "  Memory utilization: {:.1}/{:.1} MB ({:.1}%)",
        utilization.memory_usage_mb,
        utilization.max_memory_mb,
        utilization.memory_utilization_percentage()
    );

    println!();

    // Example 5: Cache Cleanup
    println!("5. Cache Maintenance");
    let removed = pooled_cache.cleanup(24); // Remove entries older than 24 hours
    println!("✓ Cleaned up {} expired entries", removed);

    pooled_cache.clear();
    println!("✓ Cleared all cache entries");

    println!("\n=== Example Complete ===");
    println!("The unified cache provides:");
    println!("• Strategy pattern for different caching behaviors");
    println!("• Runtime strategy switching");
    println!("• Memory pool integration for pooled strategy");
    println!("• Comprehensive statistics and monitoring");
    println!("• Backward compatibility with existing cache APIs");

    Ok(())
}
