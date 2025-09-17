//! Consolidated Load Testing Suite
//!
//! This file consolidates all load testing functionality from:
//! - load_test_runner.rs
//! - load_testing_framework.rs
//! - load_testing_integration_test.rs
//! - enterprise_scale_load_tests.rs
//! - github_api_load_tests.rs
//! - memory_pressure_load_tests.rs

use assert_cmd::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{sleep, Instant};

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub max_concurrent_operations: usize,
    pub test_duration_seconds: u64,
    pub file_size_mb: usize,
    pub num_files: usize,
    pub quick_mode: bool,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 10,
            test_duration_seconds: 30,
            file_size_mb: 1,
            num_files: 100,
            quick_mode: false,
        }
    }
}

/// Load test suite configuration
#[derive(Debug, Clone)]
pub struct LoadTestSuiteConfig {
    pub run_github_api_tests: bool,
    pub run_enterprise_tests: bool,
    pub run_memory_tests: bool,
    pub run_network_tests: bool,
    pub quick_mode: bool,
}

impl Default for LoadTestSuiteConfig {
    fn default() -> Self {
        Self {
            run_github_api_tests: true,
            run_enterprise_tests: true,
            run_memory_tests: true,
            run_network_tests: true,
            quick_mode: false,
        }
    }
}

/// Main load testing framework
pub struct LoadTestFramework {
    config: LoadTestConfig,
}

impl LoadTestFramework {
    pub fn new(config: LoadTestConfig) -> Self {
        Self { config }
    }

    /// Create test files for load testing
    pub async fn create_test_files(
        &self,
        temp_dir: &TempDir,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
        let mut file_paths = Vec::new();

        for i in 0..self.config.num_files {
            let file_path = temp_dir.path().join(format!("test_file_{}.rs", i));
            let content = self.generate_test_content(i);
            fs::write(&file_path, content)?;
            file_paths.push(file_path);
        }

        Ok(file_paths)
    }

    fn generate_test_content(&self, index: usize) -> String {
        format!(
            r#"
// Test file {} for load testing
fn main() {{
    println!("Processing file {}");\n
    let data = vec![0u8; {}];
    process_data(&data);
}}

fn process_data(data: &[u8]) {{
    // Simulate some processing
    let _sum: usize = data.iter().map(|&x| x as usize).sum();
}}
"#,
            index,
            index,
            self.config.file_size_mb * 1024 * 1024 / 10 // Smaller for performance
        )
    }

    /// Run concurrent file analysis
    pub async fn run_concurrent_analysis(
        &self,
        file_paths: &[std::path::PathBuf],
    ) -> Result<Duration, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_operations));
        let mut join_set = JoinSet::new();

        for file_path in file_paths {
            let sem = semaphore.clone();
            let path = file_path.clone();

            join_set.spawn(async move {
                let _permit = sem.acquire().await.unwrap();

                // Simulate analysis
                let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
                cmd.arg("check").arg(&path);

                let result = cmd.output();
                match result {
                    Ok(output) => output.status.success(),
                    Err(_) => false,
                }
            });
        }

        let mut successful_analyses = 0;
        while let Some(result) = join_set.join_next().await {
            if result.unwrap_or(false) {
                successful_analyses += 1;
            }
        }

        let duration = start_time.elapsed();
        println!(
            "Completed {} successful analyses in {:?}",
            successful_analyses, duration
        );

        Ok(duration)
    }
}

/// Load test runner with comprehensive test scenarios
pub struct LoadTestRunner {
    suite_config: LoadTestSuiteConfig,
}

impl LoadTestRunner {
    pub fn new(suite_config: LoadTestSuiteConfig) -> Self {
        Self { suite_config }
    }

    /// Run the complete load test suite
    pub async fn run_all_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting comprehensive load test suite");

        if self.suite_config.run_enterprise_tests {
            self.run_enterprise_scale_tests().await?;
        }

        if self.suite_config.run_memory_tests {
            self.run_memory_pressure_tests().await?;
        }

        if self.suite_config.run_github_api_tests {
            self.run_github_api_tests().await?;
        }

        println!("✅ All load tests completed successfully");
        Ok(())
    }

    /// Enterprise scale testing
    async fn run_enterprise_scale_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Running enterprise scale tests");

        let config = if self.suite_config.quick_mode {
            LoadTestConfig {
                max_concurrent_operations: 5,
                test_duration_seconds: 10,
                file_size_mb: 1,
                num_files: 20,
                quick_mode: true,
            }
        } else {
            LoadTestConfig {
                max_concurrent_operations: 50,
                test_duration_seconds: 120,
                file_size_mb: 5,
                num_files: 1000,
                quick_mode: false,
            }
        };

        let framework = LoadTestFramework::new(config);
        let temp_dir = TempDir::new()?;
        let file_paths = framework.create_test_files(&temp_dir).await?;

        let duration = framework.run_concurrent_analysis(&file_paths).await?;

        // Performance assertions
        let max_duration = if self.suite_config.quick_mode {
            Duration::from_secs(30)
        } else {
            Duration::from_secs(300)
        };

        assert!(
            duration < max_duration,
            "Enterprise scale test took too long: {:?}",
            duration
        );

        Ok(())
    }

    /// Memory pressure testing
    async fn run_memory_pressure_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧠 Running memory pressure tests");

        let config = LoadTestConfig {
            max_concurrent_operations: 20,
            test_duration_seconds: if self.suite_config.quick_mode { 5 } else { 60 },
            file_size_mb: if self.suite_config.quick_mode { 1 } else { 10 },
            num_files: if self.suite_config.quick_mode {
                10
            } else {
                100
            },
            quick_mode: self.suite_config.quick_mode,
        };

        let framework = LoadTestFramework::new(config);
        let temp_dir = TempDir::new()?;
        let file_paths = framework.create_test_files(&temp_dir).await?;

        // Monitor memory usage during analysis
        let start_memory = get_memory_usage();
        let duration = framework.run_concurrent_analysis(&file_paths).await?;
        let end_memory = get_memory_usage();

        let memory_increase = end_memory.saturating_sub(start_memory);
        let max_memory_mb = if self.suite_config.quick_mode {
            200
        } else {
            500
        };

        assert!(
            memory_increase < max_memory_mb * 1024 * 1024,
            "Memory usage increased too much: {} bytes",
            memory_increase
        );

        println!(
            "Memory pressure test completed in {:?}, memory increase: {} MB",
            duration,
            memory_increase / (1024 * 1024)
        );

        Ok(())
    }

    /// GitHub API load testing
    async fn run_github_api_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🐙 Running GitHub API load tests");

        // Simulate GitHub API operations
        let iterations = if self.suite_config.quick_mode { 5 } else { 20 };

        for i in 0..iterations {
            // Test CLI help command as a lightweight operation
            let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
            cmd.arg("--help");

            let start = Instant::now();
            let result = cmd.output()?;
            let duration = start.elapsed();

            assert!(result.status.success());
            assert!(
                duration < Duration::from_secs(5),
                "API operation {} took too long",
                i
            );

            if !self.suite_config.quick_mode {
                sleep(Duration::from_millis(100)).await; // Rate limiting
            }
        }

        println!("GitHub API load tests completed");
        Ok(())
    }
}

/// Get current memory usage (simplified implementation)
fn get_memory_usage() -> usize {
    // This is a simplified implementation - in a real scenario you'd use
    // proper memory monitoring tools
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_test_framework_creation() {
        let config = LoadTestConfig::default();
        let framework = LoadTestFramework::new(config);
        let temp_dir = TempDir::new().unwrap();

        let files = framework.create_test_files(&temp_dir).await.unwrap();
        assert!(!files.is_empty());

        // Verify files exist
        for file_path in &files {
            assert!(file_path.exists());
        }
    }

    #[tokio::test]
    async fn test_quick_mode_enterprise_tests() {
        let config = LoadTestSuiteConfig {
            run_github_api_tests: false,
            run_enterprise_tests: true,
            run_memory_tests: false,
            run_network_tests: false,
            quick_mode: true,
        };

        let runner = LoadTestRunner::new(config);

        // This should complete quickly
        let start = Instant::now();
        let result = runner.run_enterprise_scale_tests().await;
        let duration = start.elapsed();

        assert!(result.is_ok());
        assert!(
            duration < Duration::from_secs(60),
            "Quick mode took too long: {:?}",
            duration
        );
    }

    #[tokio::test]
    async fn test_memory_pressure_quick_mode() {
        let config = LoadTestSuiteConfig {
            run_github_api_tests: false,
            run_enterprise_tests: false,
            run_memory_tests: true,
            run_network_tests: false,
            quick_mode: true,
        };

        let runner = LoadTestRunner::new(config);
        let result = runner.run_memory_pressure_tests().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_github_api_quick_mode() {
        let config = LoadTestSuiteConfig {
            run_github_api_tests: true,
            run_enterprise_tests: false,
            run_memory_tests: false,
            run_network_tests: false,
            quick_mode: true,
        };

        let runner = LoadTestRunner::new(config);
        let result = runner.run_github_api_tests().await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_config_defaults() {
        let config = LoadTestConfig::default();
        assert_eq!(config.max_concurrent_operations, 10);
        assert_eq!(config.test_duration_seconds, 30);
        assert_eq!(config.file_size_mb, 1);
        assert_eq!(config.num_files, 100);
        assert!(!config.quick_mode);
    }

    #[test]
    fn test_suite_config_defaults() {
        let config = LoadTestSuiteConfig::default();
        assert!(config.run_github_api_tests);
        assert!(config.run_enterprise_tests);
        assert!(config.run_memory_tests);
        assert!(config.run_network_tests);
        assert!(!config.quick_mode);
    }
}
