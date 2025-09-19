use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use do_codeguardian::{
    config::Config,
    core::GuardianEngine,
    performance::{
        OptimizationRecommendation, PerformanceAnalyzer as PerfAnalyzer, PerformanceMetrics,
    },
    utils::progress::ProgressReporter,
};
use std::hint::black_box;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tempfile::tempdir;
use tokio::runtime::Runtime;

/// Optimization Recommendations Benchmark Suite
/// This suite provides automated optimization recommendations
/// based on performance analysis and bottleneck identification

/// Generate optimization test scenarios
fn generate_optimization_test_scenarios() -> Vec<OptimizationScenario> {
    vec![
        OptimizationScenario {
            name: "memory_intensive".to_string(),
            description: "High memory usage scenario".to_string(),
            setup_metrics: Box::new(|metrics: &Arc<PerformanceMetrics>| {
                metrics.update_memory_usage(300 * 1024 * 1024); // 300MB
                for _ in 0..10 {
                    metrics.record_file_processed(Duration::from_millis(500));
                }
            }),
        },
        OptimizationScenario {
            name: "cpu_intensive".to_string(),
            description: "High CPU usage scenario".to_string(),
            setup_metrics: Box::new(|metrics: &Arc<PerformanceMetrics>| {
                for _ in 0..100 {
                    metrics.record_file_processed(Duration::from_millis(200));
                }
            }),
        },
        OptimizationScenario {
            name: "cache_inefficient".to_string(),
            description: "Poor cache performance scenario".to_string(),
            setup_metrics: Box::new(|metrics: &Arc<PerformanceMetrics>| {
                for _ in 0..10 {
                    metrics.record_cache_miss();
                }
                for _ in 0..2 {
                    metrics.record_cache_hit();
                }
            }),
        },
        OptimizationScenario {
            name: "io_bound".to_string(),
            description: "I/O bound operations scenario".to_string(),
            setup_metrics: Box::new(|metrics: &Arc<PerformanceMetrics>| {
                for _ in 0..20 {
                    metrics.record_file_processed(Duration::from_millis(1000));
                }
            }),
        },
    ]
}

struct OptimizationScenario {
    name: String,
    description: String,
    setup_metrics: Box<dyn Fn(&Arc<PerformanceMetrics>)>,
}

/// Benchmark optimization recommendation generation
fn bench_optimization_recommendations(c: &mut Criterion) {
    let scenarios = generate_optimization_test_scenarios();

    let mut group = c.benchmark_group("optimization_recommendations");

    for scenario in scenarios {
        group.bench_with_input(
            BenchmarkId::new("generate_recommendations", &scenario.name),
            &scenario,
            |b, scenario| {
                let metrics = Arc::new(PerformanceMetrics::new());
                let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

                // Setup the scenario
                (scenario.setup_metrics)(&metrics);

                b.iter(|| {
                    let recommendations = analyzer.analyze_performance();
                    std::hint::black_box(recommendations);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark optimization impact analysis
fn bench_optimization_impact_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("optimization_impact");

    group.bench_function("before_after_optimization_comparison", |b| {
        let test_dir = generate_test_repository();

        b.iter(|| {
            let config = Config::default();
            rt.block_on(async {
                let progress_before = ProgressReporter::new(false);
                // Simulate before optimization
                let mut engine_before = GuardianEngine::new(config.clone(), progress_before)
                    .await
                    .unwrap();

                let files = engine_before
                    .get_all_files(&[test_dir.path().to_path_buf()])
                    .await
                    .unwrap();
                let start_before = std::time::Instant::now();
                let result_before = engine_before.analyze_files(&files, 1).await;
                let duration_before = start_before.elapsed();

                // Simulate after optimization (with more threads)
                let progress_after = ProgressReporter::new(false);
                let mut engine_after = GuardianEngine::new(config, progress_after).await.unwrap();

                let files = engine_after
                    .get_all_files(&[test_dir.path().to_path_buf()])
                    .await
                    .unwrap();
                let start_after = std::time::Instant::now();
                let result_after = engine_after.analyze_files(&files, 4).await;
                let duration_after = start_after.elapsed();

                // Calculate optimization impact
                let speedup = duration_before.as_secs_f64() / duration_after.as_secs_f64();

                std::hint::black_box((result_before.unwrap(), result_after.unwrap(), speedup));
            });
        });
    });

    group.finish();
}

/// Benchmark specific optimization strategies
fn bench_specific_optimization_strategies(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("optimization_strategies");

    // Memory pool optimization
    group.bench_function("memory_pool_optimization", |b| {
        b.iter(|| {
            // Simulate memory pool operations
            let mut allocations = Vec::new();
            for _ in 0..1000 {
                allocations.push(vec![0u8; 1024]); // 1KB allocations
            }
            std::hint::black_box(allocations.len());
        });
    });

    // Parallel processing optimization
    group.bench_function("parallel_processing_optimization", |b| {
        let test_dir = generate_test_repository();

        b.iter(|| {
            let config = Config::default();
            rt.block_on(async {
                let progress = ProgressReporter::new(false);
                let mut engine = GuardianEngine::new(config, progress).await.unwrap();

                let files = engine
                    .get_all_files(&[test_dir.path().to_path_buf()])
                    .await
                    .unwrap();
                // Test different parallelism levels
                let result = engine.analyze_files(&files, 8).await;
                std::hint::black_box(result.unwrap());
            });
        });
    });

    // Caching optimization
    group.bench_function("caching_optimization", |b| {
        let metrics = Arc::new(PerformanceMetrics::new());

        b.iter(|| {
            // Simulate cache operations
            for _ in 0..100 {
                metrics.record_cache_hit();
            }
            for _ in 0..10 {
                metrics.record_cache_miss();
            }

            let hit_rate = metrics.get_cache_hit_rate();
            std::hint::black_box(hit_rate);
        });
    });

    group.finish();
}

/// Benchmark optimization recommendation prioritization
fn bench_recommendation_prioritization(c: &mut Criterion) {
    let mut group = c.benchmark_group("recommendation_prioritization");

    group.bench_function("prioritize_critical_recommendations", |b| {
        let metrics = Arc::new(PerformanceMetrics::new());
        let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

        // Setup critical performance issues
        metrics.update_memory_usage(500 * 1024 * 1024); // 500MB - critical
        for _ in 0..5 {
            metrics.record_cache_miss(); // Poor cache performance
        }
        for _ in 0..100 {
            metrics.record_file_processed(Duration::from_millis(1000)); // Very slow
        }

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();

            // Prioritize by criticality
            let mut critical = Vec::new();
            let mut high = Vec::new();
            let mut medium = Vec::new();
            let mut low = Vec::new();

            for rec in recommendations {
                match rec.priority {
                    do_codeguardian::performance::Priority::Critical => critical.push(rec),
                    do_codeguardian::performance::Priority::High => high.push(rec),
                    do_codeguardian::performance::Priority::Medium => medium.push(rec),
                    do_codeguardian::performance::Priority::Low => low.push(rec),
                }
            }

            std::hint::black_box((critical.len(), high.len(), medium.len(), low.len()));
        });
    });

    group.finish();
}

/// Benchmark optimization implementation effort analysis
fn bench_implementation_effort_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("implementation_effort");

    group.bench_function("analyze_implementation_effort", |b| {
        let metrics = Arc::new(PerformanceMetrics::new());
        let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

        // Setup various performance issues
        metrics.update_memory_usage(250 * 1024 * 1024);
        for _ in 0..50 {
            metrics.record_file_processed(Duration::from_millis(300));
        }

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();

            // Analyze implementation effort distribution
            let mut low_effort = Vec::new();
            let mut medium_effort = Vec::new();
            let mut high_effort = Vec::new();

            for rec in recommendations {
                match rec.implementation_effort {
                    do_codeguardian::performance::ImplementationEffort::Low => low_effort.push(rec),
                    do_codeguardian::performance::ImplementationEffort::Medium => {
                        medium_effort.push(rec)
                    }
                    do_codeguardian::performance::ImplementationEffort::High => {
                        high_effort.push(rec)
                    }
                }
            }

            std::hint::black_box((low_effort.len(), medium_effort.len(), high_effort.len()));
        });
    });

    group.finish();
}

/// Benchmark optimization cost-benefit analysis
fn bench_cost_benefit_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("cost_benefit_analysis");

    group.bench_function("calculate_optimization_roi", |b| {
        let metrics = Arc::new(PerformanceMetrics::new());
        let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

        // Setup performance data
        for _ in 0..100 {
            metrics.record_file_processed(Duration::from_millis(500));
        }
        metrics.update_memory_usage(200 * 1024 * 1024);

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();

            // Calculate estimated ROI for each recommendation
            let mut total_estimated_benefit = 0.0;
            for rec in &recommendations {
                // Parse estimated improvement (simplified)
                let improvement_factor = match rec.estimated_improvement.as_str() {
                    "20-40% faster analysis" => 0.3,
                    "10-30% faster processing" => 0.2,
                    "50-70% less memory usage" => 0.6,
                    _ => 0.1,
                };

                let current_avg_time = metrics.get_average_processing_time().as_millis() as f64;
                let estimated_benefit = current_avg_time * improvement_factor;
                total_estimated_benefit += estimated_benefit;
            }

            std::hint::black_box(total_estimated_benefit);
        });
    });

    group.finish();
}

/// Generate test repository for optimization testing
fn generate_test_repository() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();

    for i in 0..20 {
        let file_path = temp_dir.path().join(format!("test_file_{}.rs", i));
        let content = format!(
            "pub fn function_{}() {{\n    println!(\"test {}\");\n}}\n",
            i, i
        );
        std::fs::write(&file_path, content).unwrap();
    }

    temp_dir
}

criterion_group!(
    benches,
    bench_optimization_recommendations,
    bench_optimization_impact_analysis,
    bench_specific_optimization_strategies,
    bench_recommendation_prioritization,
    bench_implementation_effort_analysis,
    bench_cost_benefit_analysis
);
criterion_main!(benches);
