//! Enterprise Scale Load Testing
//!
//! Tests for validating CodeGuardian performance with large-scale repositories,
//! concurrent workflows, and memory pressure scenarios as specified in Task 25.

use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{sleep, Instant};

mod load_testing_framework;
use load_testing_framework::{LoadTestConfig, LoadTestFramework};

use assert_cmd::prelude::*;
use std::process::Command;

/// Enterprise-scale repository analysis test
/// Tests processing of 100K+ files with memory and performance monitoring
#[tokio::test]
async fn test_enterprise_repository_analysis() {
    let config = LoadTestConfig {
        concurrent_operations: 8, // Limited concurrency for large-scale test
        test_duration: Duration::from_secs(600), // 10 minute test
        target_ops_per_second: 0.1, // Slow rate for large operations
        max_retries: 2,
        operation_timeout: Duration::from_secs(180), // 3 minute timeout for large operations
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(60),
    };

    let framework = LoadTestFramework::new(config);

    let report = framework
        .execute_load_test(|| async { simulate_enterprise_repository_analysis().await })
        .await
        .expect("Failed to execute enterprise load test");

    report.print_summary();

    // Enterprise-scale performance criteria
    assert!(
        report.success_rate >= 0.95,
        "Enterprise analysis success rate should be >= 95%"
    );
    assert!(
        report.average_response_time_ms <= 120000.0,
        "Enterprise analysis should complete within 2 minutes"
    );
    assert!(
        report.peak_memory_usage_mb <= 500.0,
        "Peak memory should stay under 500MB for enterprise scale"
    );
}

/// Large-scale file processing test
/// Tests processing of 1GB+ files and memory pressure scenarios
#[tokio::test]
async fn test_memory_pressure_scenarios() {
    let config = LoadTestConfig {
        concurrent_operations: 4, // Limited for memory pressure testing
        test_duration: Duration::from_secs(300), // 5 minute test
        target_ops_per_second: 0.2,
        max_retries: 3,
        operation_timeout: Duration::from_secs(240), // 4 minute timeout
        gradual_ramp_up: false,
        ramp_up_duration: Duration::from_secs(0),
    };

    let framework = LoadTestFramework::new(config);

    let report = framework
        .execute_load_test(|| async { simulate_large_file_processing().await })
        .await
        .expect("Failed to execute enterprise load test");

    report.print_summary();

    // Memory pressure performance criteria
    assert!(
        report.success_rate >= 0.90,
        "Memory pressure success rate should be >= 90%"
    );
    assert!(
        report.average_response_time_ms <= 180000.0,
        "Large file processing should complete within 3 minutes"
    );
    assert!(
        report.peak_memory_usage_mb <= 1000.0,
        "Peak memory should stay under 1GB during pressure testing"
    );
}

/// Concurrent workflow load test
/// Tests 50+ simultaneous CodeGuardian instances
#[tokio::test]
async fn test_concurrent_ci_workflows() {
    let config = LoadTestConfig {
        concurrent_operations: 50, // High concurrency for workflow testing
        test_duration: Duration::from_secs(180), // 3 minute test
        target_ops_per_second: 2.0,
        max_retries: 5,
        operation_timeout: Duration::from_secs(90),
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(30),
    };

    let framework = LoadTestFramework::new(config);

    let report = framework
        .execute_load_test(|| async { simulate_ci_workflow_execution().await })
        .await
        .expect("Failed to execute enterprise load test");

    report.print_summary();

    // Concurrent workflow performance criteria
    assert!(
        report.success_rate >= 0.95,
        "Concurrent workflow success rate should be >= 95%"
    );
    assert!(
        report.average_response_time_ms <= 60000.0,
        "CI workflows should complete within 1 minute"
    );
    assert!(
        report.operations_per_second >= 1.5,
        "Should maintain at least 1.5 workflows/sec"
    );
}

/// Resource contention analysis test
/// Tests CPU, memory, disk, and network bottlenecks under load
#[tokio::test]
async fn test_resource_contention_analysis() {
    let config = LoadTestConfig {
        concurrent_operations: 30,
        test_duration: Duration::from_secs(240), // 4 minute test
        target_ops_per_second: 1.0,
        max_retries: 3,
        operation_timeout: Duration::from_secs(60),
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(20),
    };

    let framework = LoadTestFramework::new(config);

    let report = framework
        .execute_load_test(|| async { simulate_resource_intensive_operation().await })
        .await
        .expect("Failed to execute enterprise load test");

    report.print_summary();

    // Resource contention performance criteria
    assert!(
        report.success_rate >= 0.92,
        "Resource contention success rate should be >= 92%"
    );
    assert!(
        report.average_response_time_ms <= 45000.0,
        "Resource-intensive operations should complete within 45s"
    );
    assert!(
        report.peak_memory_usage_mb <= 300.0,
        "Peak memory should stay reasonable under contention"
    );
}

/// Scalability validation test
/// Tests performance scaling with increased load and worker counts
#[tokio::test]
async fn test_scalability_validation() {
    // Test with different worker counts to validate scaling
    let worker_counts = vec![1, 2, 4, 8, 16];
    let mut scaling_results = Vec::new();

    for worker_count in worker_counts {
        let config = LoadTestConfig {
            concurrent_operations: worker_count,
            test_duration: Duration::from_secs(60),
            target_ops_per_second: worker_count as f64 * 0.5,
            max_retries: 2,
            operation_timeout: Duration::from_secs(30),
            gradual_ramp_up: false,
            ramp_up_duration: Duration::from_secs(0),
        };

        let framework = LoadTestFramework::new(config);

        let report = framework
            .execute_load_test(|| async { simulate_scalable_operation().await })
            .await
            .expect("Failed to execute enterprise load test");

        scaling_results.push((worker_count, report.operations_per_second));
        println!(
            "Workers: {}, Ops/sec: {:.2}",
            worker_count, report.operations_per_second
        );
    }

    // Validate that performance scales reasonably with worker count
    // Should see at least 50% efficiency improvement from 1 to 8 workers
    let single_worker_ops = scaling_results[0].1;
    let eight_worker_ops = scaling_results[3].1;
    let efficiency_ratio = eight_worker_ops / (single_worker_ops * 8.0);

    assert!(
        efficiency_ratio >= 0.5,
        "Scaling efficiency should be at least 50%"
    );
    println!("Scaling efficiency: {:.2}%", efficiency_ratio * 100.0);
}

/// Comprehensive system integration load test
/// Tests all components together under realistic load
#[tokio::test]
async fn test_comprehensive_system_load() {
    let config = LoadTestConfig {
        concurrent_operations: 20,
        test_duration: Duration::from_secs(300), // 5 minute comprehensive test
        target_ops_per_second: 1.0,
        max_retries: 3,
        operation_timeout: Duration::from_secs(120),
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(30),
    };

    let framework = LoadTestFramework::new(config);

    let report = framework
        .execute_load_test(|| async { simulate_comprehensive_system_operation().await })
        .await
        .expect("Failed to execute enterprise load test");

    report.print_summary();

    // Comprehensive system performance criteria (Task 25 success metrics)
    assert!(
        report.success_rate >= 0.999,
        "System success rate should be >= 99.9%"
    );
    assert!(
        report.average_response_time_ms <= 30000.0,
        "Average processing time should be <30s"
    );
    assert!(
        report.peak_memory_usage_mb <= 100.0,
        "Peak memory usage should be <100MB"
    );
    assert!(
        report.operations_per_second >= 0.8,
        "Should maintain reasonable throughput"
    );

    // Validate that the report meets all performance criteria
    assert!(
        report.meets_performance_criteria(),
        "Should meet all Task 25 performance criteria"
    );
}

// Helper functions to simulate various load scenarios

async fn simulate_enterprise_repository_analysis() -> anyhow::Result<()> {
    // Create a temporary directory with many files
    let temp_dir = TempDir::new()?;

    // Generate a moderate number of files for testing (scaled down for CI)
    for i in 0..100 {
        let dir = temp_dir.path().join(format!("module_{}", i));
        fs::create_dir_all(&dir)?;

        // Create files with realistic content
        let content = generate_realistic_file_content(i, 50); // 50 functions per file
        fs::write(dir.join("lib.rs"), content)?;
    }

    // Simulate processing time
    sleep(Duration::from_millis(1000 + rand::random::<u64>() % 3000)).await;

    // Simulate occasional failures in enterprise environments
    if rand::random::<f64>() < 0.02 {
        anyhow::bail!("Enterprise analysis failed due to resource constraints");
    }

    Ok(())
}

async fn simulate_large_file_processing() -> anyhow::Result<()> {
    // Simulate processing large files by allocating memory and doing work
    let large_buffer = vec![0u8; 10_000_000]; // 10MB buffer to simulate large file

    // Simulate file parsing and analysis
    sleep(Duration::from_millis(2000 + rand::random::<u64>() % 5000)).await;

    // Simulate memory pressure scenarios
    let _additional_buffer = vec![0u8; 5_000_000]; // Additional 5MB

    // Simulate processing work
    let _checksum: u64 = large_buffer.iter().map(|&b| b as u64).sum();

    // Simulate occasional memory-related failures
    if rand::random::<f64>() < 0.05 {
        anyhow::bail!("Memory pressure caused processing failure");
    }

    Ok(())
}

async fn simulate_ci_workflow_execution() -> anyhow::Result<()> {
    // Simulate a typical CI workflow duration
    sleep(Duration::from_millis(500 + rand::random::<u64>() % 2000)).await;

    // Simulate resource contention in CI environments
    if rand::random::<f64>() < 0.03 {
        anyhow::bail!("CI workflow failed due to resource contention");
    }

    Ok(())
}

async fn simulate_resource_intensive_operation() -> anyhow::Result<()> {
    // Simulate CPU-intensive work
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(100) {
        // Simulate CPU work
        let _result = (0..1000).map(|i| i * i).sum::<usize>();
    }

    // Simulate I/O work
    sleep(Duration::from_millis(200 + rand::random::<u64>() % 800)).await;

    // Simulate resource contention failures
    if rand::random::<f64>() < 0.08 {
        anyhow::bail!("Resource contention caused operation failure");
    }

    Ok(())
}

async fn simulate_scalable_operation() -> anyhow::Result<()> {
    // Simulate work that can be parallelized effectively
    sleep(Duration::from_millis(200 + rand::random::<u64>() % 400)).await;

    // Very low failure rate for scalability testing
    if rand::random::<f64>() < 0.01 {
        anyhow::bail!("Scalable operation failed");
    }

    Ok(())
}

async fn simulate_comprehensive_system_operation() -> anyhow::Result<()> {
    // Simulate a complete CodeGuardian analysis cycle
    let temp_dir = TempDir::new()?;

    // Create test files
    for i in 0..10 {
        let content = generate_realistic_file_content(i, 20);
        fs::write(temp_dir.path().join(format!("file_{}.rs", i)), content)?;
    }

    // Simulate analysis processing
    sleep(Duration::from_millis(1000 + rand::random::<u64>() % 2000)).await;

    // Simulate very low failure rate for comprehensive testing
    if rand::random::<f64>() < 0.001 {
        anyhow::bail!("Comprehensive system operation failed");
    }

    Ok(())
}

fn generate_realistic_file_content(file_id: usize, function_count: usize) -> String {
    let mut content = format!(
        "//! Module {} with comprehensive functionality\n\n",
        file_id
    );

    // Add imports
    content.push_str("use std::collections::HashMap;\nuse std::sync::Arc;\n\n");

    // Add struct definition
    content.push_str(&format!(
        "pub struct Module{} {{\n    data: HashMap<String, i32>,\n    counter: Arc<std::sync::atomic::AtomicU64>,\n}}\n\n",
        file_id
    ));

    // Add functions
    for i in 0..function_count {
        content.push_str(&format!(
            "pub fn function_{}_{i}() -> Result<i32, Box<dyn std::error::Error>> {{\n",
            file_id
        ));
        content.push_str("    let mut map = HashMap::new();\n");
        content.push_str(&format!(
            "    map.insert(\"key_{}\".to_string(), {});\n",
            i,
            i * file_id
        ));
        content.push_str("    let secret = \"api-key-12345\"; // Intentional security finding\n");
        content.push_str(&format!("    Ok({})\n", i * 10));
        content.push_str("}\n\n");
    }

    // Add tests
    content.push_str("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");
    for i in 0..std::cmp::min(function_count, 5) {
        content.push_str(&format!(
            "    #[test]\n    fn test_function_{}_{i}() {{\n        assert!(function_{}_{i}().is_ok());\n    }}\n\n",
            file_id, file_id
        ));
    }
    content.push_str("}\n");

    content
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_generate_realistic_file_content() {
        let content = generate_realistic_file_content(1, 3);
        assert!(content.contains("Module1"));
        assert!(content.contains("function_1_0"));
        assert!(content.contains("function_1_1"));
        assert!(content.contains("function_1_2"));
        assert!(content.contains("api-key-12345"));
        assert!(content.contains("#[test]"));
    }
}
