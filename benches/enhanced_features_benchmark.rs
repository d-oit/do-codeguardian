use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_do_codeguardian::cache::memory_pool::{FindingPool, MemoryPoolManager, StringPool};
use do_do_codeguardian::cache::unified_cache::{
    CacheStrategyType, UnifiedCache, UnifiedCacheConfig,
};
use do_do_codeguardian::ml::enhanced_feature_extractor::EnhancedFeatureExtractor;
use do_do_codeguardian::types::{Finding, Severity};
use std::hint::black_box;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use tempfile::tempdir;

/// Enhanced features performance benchmarks
/// These benchmarks measure the performance of enhanced memory pools,
/// feature extractors, optimized caches, and integration test patterns

/// Generate test findings for memory pool benchmarks
fn generate_test_findings(count: usize) -> Vec<Finding> {
    (0..count)
        .map(|i| Finding {
            id: format!("test-finding-{}", i),
            analyzer: "test-analyzer".to_string(),
            rule: format!("test-rule-{}", i),
            severity: Severity::Medium,
            file: PathBuf::from(format!("test/file_{}.rs", i)),
            line: i as u32,
            column: Some(10),
            message: format!("Test finding message {}", i),
            description: Some(format!("Detailed description for finding {}", i)),
            suggestion: Some(format!("Suggestion for finding {}", i)),
            category: Some("test".to_string()),
            metadata: std::collections::HashMap::new(),
        })
        .collect()
}

/// Generate test strings for string pool benchmarks
fn generate_test_strings(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| format!("test-string-{}-with-some-content-to-make-it-realistic", i))
        .collect()
}

fn benchmark_memory_pool_finding_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_findings");

    for pool_size in [100, 1000, 10000].iter() {
        let mut pool = FindingPool::new(*pool_size);
        let findings = generate_test_findings(1000);

        group.throughput(Throughput::Elements(1000));

        // Benchmark get operations
        group.bench_with_input(
            BenchmarkId::new("get_findings", pool_size),
            &findings,
            |b, findings| {
                b.iter(|| {
                    for _ in 0..1000 {
                        let _finding = pool.get();
                    }
                })
            },
        );

        // Benchmark put operations
        group.bench_with_input(
            BenchmarkId::new("put_findings", pool_size),
            &findings,
            |b, findings| {
                b.iter(|| {
                    for finding in findings.iter() {
                        pool.put(finding.clone());
                    }
                })
            },
        );

        // Benchmark get-put cycle (reuse)
        group.bench_with_input(
            BenchmarkId::new("get_put_cycle", pool_size),
            &findings,
            |b, findings| {
                b.iter(|| {
                    for finding in findings.iter() {
                        let mut temp = pool.get();
                        // Simulate some usage
                        temp.line += 1;
                        pool.put(temp);
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_memory_pool_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_strings");

    for pool_size in [500, 5000, 50000].iter() {
        let mut pool = StringPool::new(*pool_size);
        let strings = generate_test_strings(1000);

        group.throughput(Throughput::Elements(1000));

        // Benchmark string interning
        group.bench_with_input(
            BenchmarkId::new("intern_strings", pool_size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    for s in strings.iter() {
                        let _interned = pool.get(s);
                    }
                })
            },
        );

        // Benchmark string reuse
        group.bench_with_input(
            BenchmarkId::new("reuse_strings", pool_size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    // First intern all strings
                    for s in strings.iter() {
                        let _interned = pool.get(s);
                    }
                    // Then reuse them
                    for s in strings.iter() {
                        let _interned = pool.get(s);
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_memory_pool_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_concurrent");

    let manager = Arc::new(MemoryPoolManager::new());
    let num_threads = 4;
    let operations_per_thread = 1000;

    group.throughput(Throughput::Elements(
        (num_threads * operations_per_thread) as u64,
    ));

    group.bench_function("concurrent_finding_pool", |b| {
        b.iter(|| {
            let mut handles = vec![];

            for _ in 0..num_threads {
                let manager_clone = Arc::clone(&manager);
                let handle = thread::spawn(move || {
                    let pool = manager_clone.finding_pool();
                    for _ in 0..operations_per_thread {
                        let finding = {
                            let mut pool_lock = pool.lock().unwrap();
                            pool_lock.get()
                        };
                        // Simulate some work
                        thread::yield_now();
                        {
                            let mut pool_lock = pool.lock().unwrap();
                            pool_lock.put(finding);
                        }
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.bench_function("concurrent_string_pool", |b| {
        b.iter(|| {
            let mut handles = vec![];

            for i in 0..num_threads {
                let manager_clone = Arc::clone(&manager);
                let thread_id = i;
                let handle = thread::spawn(move || {
                    let pool = manager_clone.string_pool();
                    for j in 0..operations_per_thread {
                        let test_string = format!("thread-{}-string-{}", thread_id, j);
                        let _interned = {
                            let mut pool_lock = pool.lock().unwrap();
                            pool_lock.get(&test_string)
                        };
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.finish();
}

fn benchmark_enhanced_feature_extractor(c: &mut Criterion) {
    let mut group = c.benchmark_group("enhanced_feature_extractor");

    let mut extractor = EnhancedFeatureExtractor::new();
    let temp_dir = tempdir().unwrap();

    // Create test files of different sizes
    for size_kb in [1, 10, 100].iter() {
        let file_path = temp_dir.path().join(format!("test_{}kb.rs", size_kb));
        let content = format!(
            "fn main() {{\n    println!(\"Hello World\");\n    {}\n}}\n",
            (0..*size_kb * 50)
                .map(|i| format!("    let x{} = {};\n", i, i))
                .collect::<String>()
        );
        std::fs::write(&file_path, content).unwrap();

        let finding = Finding {
            id: "test-finding".to_string(),
            analyzer: "test".to_string(),
            rule: "test-rule".to_string(),
            severity: Severity::High,
            file: file_path.clone(),
            line: 1,
            column: Some(1),
            message: "Test finding".to_string(),
            description: Some("Test description".to_string()),
            suggestion: Some("Test suggestion".to_string()),
            category: Some("test".to_string()),
            metadata: std::collections::HashMap::new(),
        };

        group.throughput(Throughput::Bytes((*size_kb * 1024) as u64));

        group.bench_with_input(
            BenchmarkId::new("extract_features", size_kb),
            &finding,
            |b, finding| {
                b.iter(|| {
                    let _features = black_box(
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async { extractor.extract_enhanced_features(finding).await })
                            .unwrap(),
                    );
                })
            },
        );
    }

    group.finish();
}

fn benchmark_unified_cache_pooled(c: &mut Criterion) {
    let mut group = c.benchmark_group("unified_cache_pooled");

    let temp_dir = tempdir().unwrap();
    let config = UnifiedCacheConfig {
        strategy: CacheStrategyType::Pooled,
        max_entries: 1000,
        max_memory_mb: 50,
        enable_memory_pools: true,
        pool_sizes: None,
    };
    let mut cache = UnifiedCache::new(config).unwrap();

    // Create test files and findings
    let mut test_files = vec![];
    for i in 0..100 {
        let file_path = temp_dir.path().join(format!("cache_test_{}.rs", i));
        let content = format!("fn func_{}() {{ println!(\"test {}\"); }}", i, i);
        std::fs::write(&file_path, &content).unwrap();

        let findings = generate_test_findings(5);
        test_files.push((file_path, findings, format!("config-hash-{}", i)));
    }

    // Benchmark cache put operations
    group.bench_function("cache_put_operations", |b| {
        b.iter(|| {
            for (file_path, findings, config_hash) in &test_files {
                let _ = cache.put(file_path, findings.clone(), config_hash, 100);
            }
        })
    });

    // Benchmark cache get operations (after populating)
    group.bench_function("cache_get_operations", |b| {
        b.iter(|| {
            for (file_path, _findings, config_hash) in &test_files {
                let _result = cache.get(file_path, config_hash);
            }
        })
    });

    // Benchmark cache hit rate
    group.bench_function("cache_hit_rate", |b| {
        b.iter(|| {
            // Mix of hits and misses
            for i in 0..100 {
                if i % 2 == 0 {
                    // Hit
                    let (file_path, _findings, config_hash) = &test_files[i % test_files.len()];
                    let _result = cache.get(file_path, config_hash);
                } else {
                    // Miss (different config)
                    let (file_path, _findings, _config_hash) = &test_files[i % test_files.len()];
                    let _result = cache.get(file_path, "different-config");
                }
            }
        })
    });

    group.finish();
}

fn benchmark_memory_usage_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage_patterns");

    // Benchmark memory pool memory efficiency
    group.bench_function("memory_pool_efficiency", |b| {
        b.iter(|| {
            let manager = MemoryPoolManager::new();

            // Simulate heavy usage
            for _ in 0..1000 {
                let finding = {
                    let pool = manager.finding_pool();
                    let mut pool_lock = pool.lock().unwrap();
                    pool_lock.get()
                };

                let string = {
                    let pool = manager.string_pool();
                    let mut pool_lock = pool.lock().unwrap();
                    pool_lock.get("test-string")
                };

                // Simulate processing
                thread::yield_now();

                // Return to pools
                {
                    let pool = manager.finding_pool();
                    let mut pool_lock = pool.lock().unwrap();
                    pool_lock.put(finding);
                }
                drop(string); // Arc will handle cleanup
            }

            let _stats = manager.stats();
        })
    });

    // Benchmark cache memory management
    group.bench_function("cache_memory_management", |b| {
        b.iter(|| {
            let config = UnifiedCacheConfig {
                strategy: CacheStrategyType::Pooled,
                max_entries: 100,
                max_memory_mb: 10,
                enable_memory_pools: true,
                pool_sizes: None,
            };
            let mut cache = UnifiedCache::new(config).unwrap();
            let temp_dir = tempdir().unwrap();

            // Fill cache and trigger evictions
            for i in 0..200 {
                let file_path = temp_dir.path().join(format!("mem_test_{}.rs", i));
                std::fs::write(&file_path, format!("fn test_{}() {{}}", i)).unwrap();

                let findings = generate_test_findings(3);
                let _ = cache.put(&file_path, findings, &format!("config-{}", i), 50);
            }

            let _utilization = cache.utilization();
            let _savings = cache.memory_savings();
        })
    });

    group.finish();
}

fn benchmark_scalability_with_input_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_input_sizes");

    // Test memory pool scalability
    for input_size in [100, 1000, 10000].iter() {
        let findings = generate_test_findings(*input_size);

        group.throughput(Throughput::Elements(*input_size as u64));

        group.bench_with_input(
            BenchmarkId::new("memory_pool_scalability", input_size),
            &findings,
            |b, findings| {
                let mut pool = FindingPool::new(findings.len() * 2);
                b.iter(|| {
                    for finding in findings.iter() {
                        pool.put(finding.clone());
                    }
                    for _ in 0..findings.len() {
                        let _ = pool.get();
                    }
                })
            },
        );
    }

    // Test feature extractor scalability
    for file_size_kb in [10, 100, 1000].iter() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir
            .path()
            .join(format!("scale_test_{}kb.rs", file_size_kb));
        let content = (0..*file_size_kb * 100)
            .map(|i| format!("fn func_{}() {{ let x = {}; }}\n", i, i))
            .collect::<String>();
        std::fs::write(&file_path, content).unwrap();

        let finding = Finding {
            id: "scale-test".to_string(),
            analyzer: "test".to_string(),
            rule: "scale-rule".to_string(),
            severity: Severity::Info,
            file: file_path,
            line: 1,
            column: None,
            message: "Scale test finding".to_string(),
            description: None,
            suggestion: None,
            category: None,
            metadata: std::collections::HashMap::new(),
        };

        group.throughput(Throughput::Bytes((*file_size_kb * 1024) as u64));

        group.bench_with_input(
            BenchmarkId::new("feature_extractor_scalability", file_size_kb),
            &finding,
            |b, finding| {
                let mut extractor = EnhancedFeatureExtractor::new();
                b.iter(|| {
                    let _ =
                        black_box(tokio::runtime::Runtime::new().unwrap().block_on(async {
                            extractor.extract_enhanced_features(finding).await
                        }));
                })
            },
        );
    }

    group.finish();
}

fn benchmark_integration_test_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("integration_test_patterns");

    // Benchmark comprehensive integration test setup
    group.bench_function("integration_test_setup", |b| {
        b.iter(|| {
            // Simulate integration test setup
            let temp_dir = tempdir().unwrap();
            let config_path = temp_dir.path().join("config.toml");
            let test_files_dir = temp_dir.path().join("test_files");

            std::fs::create_dir(&test_files_dir).unwrap();

            // Create test configuration
            let config_content = r#"
                [core]
                max_file_size_mb = 10
                parallel_workers = 4

                [cache]
                enabled = true
                max_entries = 1000
                max_memory_mb = 100

                [output]
                format = "json"
                verbose = false
            "#;
            std::fs::write(&config_path, config_content).unwrap();

            // Create test files
            for i in 0..10 {
                let file_path = test_files_dir.join(format!("integration_test_{}.rs", i));
                let content = format!(
                    "fn integration_test_{}() {{ println!(\"test {}\"); }}",
                    i, i
                );
                std::fs::write(&file_path, content).unwrap();
            }

            // Simulate loading configuration
            let _config: toml::Value = toml::from_str(&config_content).unwrap();
        })
    });

    // Benchmark cross-component interaction
    group.bench_function("cross_component_interaction", |b| {
        b.iter(|| {
            let manager = MemoryPoolManager::new();
            let config = UnifiedCacheConfig {
                strategy: CacheStrategyType::Pooled,
                max_entries: 100,
                max_memory_mb: 10,
                enable_memory_pools: true,
                pool_sizes: None,
            };
            let mut cache = UnifiedCache::new(config).unwrap();
            let mut extractor = EnhancedFeatureExtractor::new();

            let temp_dir = tempdir().unwrap();

            // Simulate cross-component workflow
            for i in 0..50 {
                let file_path = temp_dir.path().join(format!("component_test_{}.rs", i));
                let content = format!("fn component_test_{}() {{}}", i);
                std::fs::write(&file_path, content).unwrap();

                // Use memory pools
                let finding = {
                    let pool = manager.finding_pool();
                    let mut pool_lock = pool.lock().unwrap();
                    let mut finding = pool_lock.get();
                    finding.file = file_path.clone();
                    finding
                };

                // Extract features
                let _features = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async { extractor.extract_enhanced_features(&finding).await })
                    .unwrap();

                // Cache results
                let _ = cache.put(&file_path, vec![finding], "test-config", 10);
            }

            let _pool_stats = manager.stats();
            let _cache_stats = cache.stats();
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_memory_pool_finding_operations,
    benchmark_memory_pool_string_operations,
    benchmark_memory_pool_concurrent_access,
    benchmark_enhanced_feature_extractor,
    benchmark_unified_cache_pooled,
    benchmark_memory_usage_patterns,
    benchmark_scalability_with_input_sizes,
    benchmark_integration_test_patterns,
);

criterion_main!(benches);
