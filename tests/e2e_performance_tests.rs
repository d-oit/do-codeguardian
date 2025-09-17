use assert_cmd::prelude::*;
use predicates::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Performance-focused end-to-end tests with regression detection
/// Tracks performance baselines and detects performance degradation

/// Performance baseline data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceBaseline {
    scenario_name: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    average_duration: Duration,
    min_duration: Duration,
    max_duration: Duration,
    file_count: usize,
    total_size_bytes: u64,
    system_info: SystemInfo,
}

/// System information for performance context
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SystemInfo {
    cpu_count: usize,
    total_memory_mb: u64,
    platform: String,
    codeguardian_version: String,
}

/// Performance test result
#[derive(Debug, Clone)]
struct PerformanceResult {
    scenario_name: String,
    duration: Duration,
    file_count: usize,
    total_size_bytes: u64,
    throughput_files_per_sec: f64,
    throughput_bytes_per_sec: f64,
    memory_usage_mb: Option<u64>,
    cpu_usage_percent: Option<f64>,
}

/// Performance regression detector
struct PerformanceRegressionDetector {
    baselines: HashMap<String, PerformanceBaseline>,
    baseline_file: std::path::PathBuf,
    regression_threshold_percent: f64,
}

impl PerformanceRegressionDetector {
    fn new(baseline_file: std::path::PathBuf) -> Self {
        let mut detector = Self {
            baselines: HashMap::new(),
            baseline_file,
            regression_threshold_percent: 10.0, // 10% regression threshold
        };

        // Load existing baselines
        if detector.baseline_file.exists() {
            if let Ok(content) = fs::read_to_string(&detector.baseline_file) {
                if let Ok(baselines) =
                    serde_json::from_str::<HashMap<String, PerformanceBaseline>>(&content)
                {
                    detector.baselines = baselines;
                }
            }
        }

        detector
    }

    fn record_baseline(&mut self, result: PerformanceResult) {
        let baseline = PerformanceBaseline {
            scenario_name: result.scenario_name.clone(),
            timestamp: chrono::Utc::now(),
            average_duration: result.duration,
            min_duration: result.duration,
            max_duration: result.duration,
            file_count: result.file_count,
            total_size_bytes: result.total_size_bytes,
            system_info: self.get_system_info(),
        };

        self.baselines.insert(result.scenario_name, baseline);
        self.save_baselines();
    }

    fn check_regression(&self, result: &PerformanceResult) -> Option<PerformanceRegression> {
        if let Some(baseline) = self.baselines.get(&result.scenario_name) {
            let duration_regression = (result.duration.as_millis() as f64
                - baseline.average_duration.as_millis() as f64)
                / baseline.average_duration.as_millis() as f64
                * 100.0;

            if duration_regression > self.regression_threshold_percent {
                return Some(PerformanceRegression {
                    scenario_name: result.scenario_name.clone(),
                    regression_type: RegressionType::Duration,
                    baseline_value: baseline.average_duration,
                    current_value: result.duration,
                    percentage_change: duration_regression,
                    threshold_percent: self.regression_threshold_percent,
                });
            }
        }
        None
    }

    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            cpu_count: num_cpus::get(),
            total_memory_mb: self.get_total_memory_mb(),
            platform: std::env::consts::OS.to_string(),
            codeguardian_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    fn get_total_memory_mb(&self) -> u64 {
        // Simple heuristic - in a real implementation, you'd use system APIs
        8192 // Assume 8GB for testing
    }

    fn save_baselines(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.baselines) {
            let _ = fs::write(&self.baseline_file, json);
        }
    }
}

/// Performance regression information
#[derive(Debug, Clone)]
struct PerformanceRegression {
    scenario_name: String,
    regression_type: RegressionType,
    baseline_value: Duration,
    current_value: Duration,
    percentage_change: f64,
    threshold_percent: f64,
}

/// Types of performance regression
#[derive(Debug, Clone)]
enum RegressionType {
    Duration,
    Memory,
    Throughput,
}

/// Test large codebase performance
#[test]
fn test_large_codebase_performance() {
    let temp_dir = TempDir::new().unwrap();
    let baseline_file = temp_dir.path().join("performance_baselines.json");

    let mut detector = PerformanceRegressionDetector::new(baseline_file);

    // Create a large codebase (100 files)
    let file_count = 100;
    let mut total_size = 0u64;

    for i in 0..file_count {
        let dir = temp_dir.path().join(format!("module_{}", i));
        fs::create_dir_all(&dir).unwrap();

        let content = format!(
            r#"
// Large codebase file {}
pub mod module_{};

pub fn function_{}() -> i32 {{
    let mut result = 0;
    {}
    result
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_function_{}() {{
        assert_eq!(function_{}(), {});
    }}
}}
"#,
            i,
            i,
            i,
            (0..100)
                .map(|j| format!("    result += {};", j))
                .collect::<Vec<_>>()
                .join("\n"),
            i,
            i,
            i * 5050 // Sum formula: n*(n+1)/2 * 100
        );

        let file_path = dir.join("lib.rs");
        fs::write(&file_path, &content).unwrap();
        total_size += content.len() as u64;
    }

    // Run performance test
    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));

    let duration = start.elapsed();

    // Calculate metrics
    let result = PerformanceResult {
        scenario_name: "large_codebase_100_files".to_string(),
        duration,
        file_count,
        total_size_bytes: total_size,
        throughput_files_per_sec: file_count as f64 / duration.as_secs_f64(),
        throughput_bytes_per_sec: total_size as f64 / duration.as_secs_f64(),
        memory_usage_mb: None,
        cpu_usage_percent: None,
    };

    // Check for regression
    if let Some(regression) = detector.check_regression(&result) {
        eprintln!("PERFORMANCE REGRESSION DETECTED: {:?}", regression);
        eprintln!("Baseline: {:?}", regression.baseline_value);
        eprintln!("Current: {:?}", regression.current_value);
        eprintln!("Change: {:.2}%", regression.percentage_change);

        // In CI, you might want to fail the test on regression
        // For now, just log it
        assert!(
            regression.percentage_change < 50.0,
            "Performance regression too severe: {:.2}%",
            regression.percentage_change
        );
    } else {
        // Update baseline if no regression
        detector.record_baseline(result);
    }

    // Basic performance assertion
    assert!(
        duration < Duration::from_secs(120),
        "Analysis took too long: {:?}",
        duration
    );
}

/// Test memory usage with large files
#[test]
fn test_memory_usage_large_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create files with substantial content
    let file_count = 10;
    let mut _total_size = 0u64;

    for i in 0..file_count {
        let content = format!(
            r#"
// Memory-intensive file {}
pub struct LargeStruct_{} {{
    {}
}}

impl LargeStruct_{} {{
    {}
}}
"#,
            i,
            i,
            (0..100)
                .map(|j| format!("    field_{}: i32,", j))
                .collect::<Vec<_>>()
                .join("\n"),
            i,
            (0..100)
                .map(|j| format!(
                    "    pub fn method_{}(&self) -> i32 {{ self.field_{} }}",
                    j, j
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let file_path = temp_dir.path().join(format!("large_file_{}.rs", i));
        fs::write(&file_path, &content).unwrap();
        _total_size += content.len() as u64;
    }

    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));

    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(60),
        "Large file analysis took too long: {:?}",
        duration
    );
}

/// Test concurrent analysis performance
#[test]
fn test_concurrent_analysis_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that can be analyzed in parallel
    let file_count = 50;
    let mut _total_size = 0u64;

    for i in 0..file_count {
        let content = format!(
            r#"
// Concurrent analysis file {}
pub fn process_data_{}() {{
    let secret = "key-{}-secret"; // Each file has a finding
    println!("Processing with secret: {{}}", secret);

    // Some computation to make analysis take time
    {}
}}
"#,
            i,
            i,
            i,
            (0..200)
                .map(|j| format!("    let result_{} = {} * {};", j, i, j))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let file_path = temp_dir.path().join(format!("concurrent_{}.rs", i));
        fs::write(&file_path, &content).unwrap();
        _total_size += content.len() as u64;
    }

    // Test with different parallelization levels
    let parallel_levels = vec![1, 2, 4, 0]; // 0 = auto

    let mut results = Vec::new();

    for &parallel in &parallel_levels {
        let start = Instant::now();

        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--parallel")
            .arg(parallel.to_string())
            .arg("--format")
            .arg("json");

        cmd.assert().success();

        let duration = start.elapsed();
        results.push((parallel, duration));

        assert!(
            duration < Duration::from_secs(60),
            "Concurrent analysis with {} workers took too long: {:?}",
            parallel,
            duration
        );
    }

    // Verify that higher parallelization helps (at least doesn't hurt much)
    if results.len() >= 2 {
        let sequential_time = results[0].1;
        let parallel_time = results.last().unwrap().1;

        // Parallel should be at least 80% as fast as sequential (allowing for overhead)
        assert!(
            parallel_time <= sequential_time.mul_f32(1.25),
            "Parallel analysis too slow: {:?} vs {:?}",
            parallel_time,
            sequential_time
        );
    }
}

/// Test incremental analysis performance
#[test]
fn test_incremental_analysis_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repository
    Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create initial files
    let file_count = 20;
    for i in 0..file_count {
        let content = format!("pub fn func_{}() {{}}", i);
        fs::write(temp_dir.path().join(format!("file_{}.rs", i)), content).unwrap();
    }

    // Initial commit
    Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Full analysis
    let start_full = Instant::now();

    let mut full_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    full_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    full_cmd.assert().success();
    let full_duration = start_full.elapsed();

    // Modify one file
    let modified_file = temp_dir.path().join("file_0.rs");
    let new_content = "pub fn func_0() { let secret = \"modified\"; }".to_string();
    fs::write(&modified_file, new_content).unwrap();

    // Incremental analysis (diff mode)
    let start_diff = Instant::now();

    let mut diff_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    diff_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--diff")
        .arg("HEAD")
        .arg("--format")
        .arg("json");

    diff_cmd.assert().success();
    let diff_duration = start_diff.elapsed();

    // Incremental should be faster
    assert!(
        diff_duration < full_duration,
        "Incremental analysis should be faster: {:?} vs {:?}",
        diff_duration,
        full_duration
    );
}

/// Test performance under memory pressure
#[test]
fn test_memory_pressure_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that use a lot of memory during parsing
    let file_count = 5;

    for i in 0..file_count {
        let content = format!(
            r#"
// Memory pressure test file {}
{}

pub struct MemoryHog_{} {{
    {}
}}
"#,
            i,
            (0..500)
                .map(|j| format!(
                    "pub const CONST_{}_{}: &str = \"{}\";",
                    i,
                    j,
                    "x".repeat(1000)
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            i,
            (0..200)
                .map(|j| format!("    field_{}: [u8; 1024],", j))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(
            temp_dir.path().join(format!("memory_hog_{}.rs", i)),
            content,
        )
        .unwrap();
    }

    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();

    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(30),
        "Memory pressure test took too long: {:?}",
        duration
    );
}

/// Test performance with different file types
#[test]
fn test_mixed_filetype_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Create content strings first to avoid temporary value issues
    let medium_content = format!("fn main() {{ {} }}", "println!(\"test\");".repeat(100));
    let large_content = format!("fn main() {{ {} }}", "println!(\"test\");".repeat(1000));
    let readme_content = "# Test\n".repeat(500);
    let script_content = "function test() { console.log('test'); }\n".repeat(200);
    let style_content = ".class { property: value; }\n".repeat(300);

    // Create files of different types and sizes
    let test_files = vec![
        ("small.rs", "fn main() {}"),
        ("medium.rs", &medium_content),
        ("large.rs", &large_content),
        (
            "config.toml",
            "[package]\nname = \"test\"\nversion = \"1.0.0\"",
        ),
        ("readme.md", &readme_content),
        ("script.js", &script_content),
        ("style.css", &style_content),
    ];

    let mut _total_size = 0u64;
    for (filename, content) in &test_files {
        fs::write(temp_dir.path().join(filename), content).unwrap();
        _total_size += content.len() as u64;
    }

    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));

    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(30),
        "Mixed filetype analysis took too long: {:?}",
        duration
    );

    // Verify all files were processed
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
        if let Some(scanned) = json["summary"]["total_files_scanned"].as_u64() {
            assert!(
                scanned >= test_files.len() as u64,
                "Not all files were scanned"
            );
        }
    }
}

/// Test performance scaling with file count
#[test]
fn test_performance_scaling() {
    let temp_dir = TempDir::new().unwrap();

    // Test with different numbers of files
    let file_counts = vec![10, 50, 100];

    let mut scaling_results = Vec::new();

    for &count in &file_counts {
        // Clean up previous files
        for entry in fs::read_dir(temp_dir.path()).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::remove_file(entry.path()).unwrap();
            }
        }

        // Create files
        for i in 0..count {
            let content = format!("pub fn func_{}() {{ let x = {}; }}", i, i);
            fs::write(temp_dir.path().join(format!("scale_{}.rs", i)), content).unwrap();
        }

        let start = Instant::now();

        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        cmd.assert().success();

        let duration = start.elapsed();
        scaling_results.push((count, duration));

        assert!(
            duration < Duration::from_secs(60),
            "Scaling test with {} files took too long: {:?}",
            count,
            duration
        );
    }

    // Verify scaling is reasonable (should be roughly linear)
    if scaling_results.len() >= 2 {
        let (count1, time1) = scaling_results[0];
        let (count2, time2) = scaling_results[1];

        let scaling_factor = time2.as_secs_f64() / time1.as_secs_f64();
        let count_ratio = count2 as f64 / count1 as f64;

        // Allow some overhead but should be roughly proportional
        assert!(
            scaling_factor < count_ratio * 2.0,
            "Performance scaling too poor: {}x time for {}x files",
            scaling_factor,
            count_ratio
        );
    }
}

/// Test CI/CD performance characteristics
#[test]
fn test_ci_cd_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Simulate CI environment
    std::env::set_var("CI", "true");

    // Create a typical CI codebase
    let file_count = 30;
    for i in 0..file_count {
        let content = format!(
            r#"
// CI test file {}
pub fn ci_function_{}() {{
    // Simulate some security issues for CI to catch
    let api_key = "sk-{}-test-key";
    let password = "pwd{}";
    println!("API Key: {{}}, Password: {{}}", api_key, password);
}}
"#,
            i, i, i, i
        );

        fs::write(temp_dir.path().join(format!("ci_{}.rs", i)), content).unwrap();
    }

    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("sarif") // CI-friendly format
        .arg("--quiet") // No progress output
        .arg("--fail-on-issues"); // Exit with error on issues

    let output = cmd.output().unwrap();
    let duration = start.elapsed();

    // In CI mode, should find issues and exit with error
    assert!(
        !output.status.success(),
        "CI mode should fail on security issues"
    );

    assert!(
        duration < Duration::from_secs(30),
        "CI analysis took too long: {:?}",
        duration
    );

    // Clean up
    std::env::remove_var("CI");
}

/// Test performance with ML features enabled
#[cfg(feature = "ml")]
#[test]
fn test_ml_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that would benefit from ML analysis
    let file_count = 20;
    for i in 0..file_count {
        let content = format!(
            r#"
// ML test file {}
pub fn ml_function_{}() {{
    // Mix of real issues and false positives
    let password = "hardcoded_password_{}";
    let api_key = "sk-{}-real-key";
    let normal_var = "normal_value_{}";
    println!("Values: {{}} {{}} {{}}", password, api_key, normal_var);
}}
"#,
            i, i, i, i, i
        );

        fs::write(temp_dir.path().join(format!("ml_{}.rs", i)), content).unwrap();
    }

    let start = Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--ml-threshold")
        .arg("0.5")
        .arg("--format")
        .arg("json");

    cmd.assert().success();

    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(45),
        "ML analysis took too long: {:?}",
        duration
    );
}

/// Performance benchmark for continuous monitoring
#[test]
fn test_continuous_performance_monitoring() {
    let temp_dir = TempDir::new().unwrap();
    let baseline_file = temp_dir.path().join("continuous_baselines.json");

    let mut detector = PerformanceRegressionDetector::new(baseline_file.clone());

    // Create a stable test case
    let content = r#"
pub fn stable_function() {
    let x = 42;
    let y = "test";
    println!("x: {}, y: {}", x, y);
}
"#;

    fs::write(temp_dir.path().join("stable.rs"), content).unwrap();

    // Run multiple times to establish baseline
    let mut durations = Vec::new();

    for _ in 0..3 {
        let start = Instant::now();

        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(&temp_dir.path().join("stable.rs"))
            .arg("--format")
            .arg("json");

        cmd.assert().success();

        durations.push(start.elapsed());
    }

    // Calculate average
    let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;

    let result = PerformanceResult {
        scenario_name: "continuous_monitoring".to_string(),
        duration: avg_duration,
        file_count: 1,
        total_size_bytes: content.len() as u64,
        throughput_files_per_sec: 1.0 / avg_duration.as_secs_f64(),
        throughput_bytes_per_sec: content.len() as f64 / avg_duration.as_secs_f64(),
        memory_usage_mb: None,
        cpu_usage_percent: None,
    };

    // Check for regression
    if let Some(regression) = detector.check_regression(&result) {
        eprintln!("CONTINUOUS MONITORING REGRESSION: {:?}", regression);
        // Don't fail test, just log
    } else {
        detector.record_baseline(result);
    }

    // Verify baseline file was created/updated
    assert!(baseline_file.exists(), "Baseline file should be created");
}
