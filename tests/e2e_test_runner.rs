use assert_cmd::cargo::CommandCargoExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::sync::Mutex;

/// Comprehensive end-to-end test runner with scenario management
/// Provides structured testing framework for CodeGuardian workflows

/// Test scenario configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub category: ScenarioCategory,
    pub priority: TestPriority,
    pub platforms: Vec<String>, // ["linux", "windows", "macos"]
    pub timeout: Duration,
    pub setup_steps: Vec<TestStep>,
    pub test_steps: Vec<TestStep>,
    pub cleanup_steps: Vec<TestStep>,
    pub expected_results: ExpectedResults,
    pub tags: Vec<String>,
}

/// Test step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    pub name: String,
    pub action: TestAction,
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub working_dir: Option<String>,
    pub timeout: Option<Duration>,
    pub expect_success: bool,
    pub expected_output_contains: Vec<String>,
    pub expected_files: Vec<String>,
}

/// Test action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestAction {
    CreateFile {
        path: String,
        content: String,
    },
    CreateDir {
        path: String,
    },
    RunCommand {
        command: String,
    },
    GitCommand {
        args: Vec<String>,
    },
    CodeGuardianCommand {
        subcommand: String,
        args: Vec<String>,
    },
    AssertFileExists {
        path: String,
    },
    AssertFileContains {
        path: String,
        content: String,
    },
    AssertOutputContains {
        pattern: String,
    },
    Sleep {
        duration_ms: u64,
    },
    Custom {
        function: String,
    },
}

/// Expected test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedResults {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub output_contains: Vec<String>,
    pub files_created: Vec<String>,
    pub performance_requirements: Option<PerformanceRequirements>,
}

/// Performance requirements for regression testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_duration: Duration,
    pub max_memory_mb: Option<u64>,
    pub min_files_per_second: Option<f64>,
}

/// Test scenario categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioCategory {
    BasicWorkflow,
    SecurityAnalysis,
    GitIntegration,
    Performance,
    ErrorHandling,
    CrossPlatform,
    CiCd,
    MlTraining,
    Dashboard,
    BulkOperations,
}

/// Test priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Test execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub scenario_name: String,
    pub success: bool,
    pub duration: Duration,
    pub exit_code: Option<i32>,
    pub output: String,
    pub error_output: String,
    pub files_created: Vec<String>,
    pub performance_metrics: Option<PerformanceMetrics>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Performance metrics collected during test execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub duration: Duration,
    pub memory_usage_mb: Option<u64>,
    pub files_processed: u64,
    pub files_per_second: f64,
    pub cpu_usage_percent: Option<f64>,
}

/// E2E Test Runner with scenario management
pub struct E2ETestRunner {
    scenarios: Vec<TestScenario>,
    results: Arc<Mutex<Vec<TestResult>>>,
    temp_dir: TempDir,
    config: TestRunnerConfig,
}

/// Configuration for the test runner
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    pub parallel_execution: bool,
    pub max_parallel_workers: usize,
    pub fail_fast: bool,
    pub collect_performance_metrics: bool,
    pub output_directory: PathBuf,
    pub baseline_file: Option<PathBuf>,
    pub platform_filter: Option<String>,
    pub category_filter: Option<ScenarioCategory>,
    pub tag_filter: Vec<String>,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            parallel_execution: false,
            max_parallel_workers: num_cpus::get(),
            fail_fast: false,
            collect_performance_metrics: true,
            output_directory: PathBuf::from("test-results"),
            baseline_file: None,
            platform_filter: None,
            category_filter: None,
            tag_filter: Vec::new(),
        }
    }
}

impl E2ETestRunner {
    /// Create a new E2E test runner
    pub fn new() -> Self {
        Self {
            scenarios: Vec::new(),
            results: Arc::new(Mutex::new(Vec::new())),
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            config: TestRunnerConfig::default(),
        }
    }

    /// Create a new test runner with custom configuration
    pub fn with_config(config: TestRunnerConfig) -> Self {
        Self {
            scenarios: Vec::new(),
            results: Arc::new(Mutex::new(Vec::new())),
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            config,
        }
    }

    /// Add a test scenario
    pub fn add_scenario(&mut self, scenario: TestScenario) {
        self.scenarios.push(scenario);
    }

    /// Load scenarios from a JSON file
    pub fn load_scenarios_from_file(
        &mut self,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let scenarios: Vec<TestScenario> = serde_json::from_str(&content)?;
        self.scenarios.extend(scenarios);
        Ok(())
    }

    /// Filter scenarios based on configuration
    pub fn filter_scenarios(&self) -> Vec<&TestScenario> {
        self.scenarios
            .iter()
            .filter(|scenario| {
                // Platform filter
                if let Some(ref platform) = self.config.platform_filter {
                    if !scenario.platforms.contains(platform) {
                        return false;
                    }
                }

                // Category filter
                if let Some(ref category) = self.config.category_filter {
                    if std::mem::discriminant(&scenario.category)
                        != std::mem::discriminant(category)
                    {
                        return false;
                    }
                }

                // Tag filter
                if !self.config.tag_filter.is_empty() {
                    let scenario_tags: std::collections::HashSet<_> =
                        scenario.tags.iter().collect();
                    let filter_tags: std::collections::HashSet<_> =
                        self.config.tag_filter.iter().collect();
                    if !filter_tags.is_subset(&scenario_tags) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    /// Run all filtered scenarios
    pub async fn run_all_scenarios(&self) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        let scenarios = self.filter_scenarios();

        if self.config.parallel_execution {
            self.run_scenarios_parallel(scenarios).await
        } else {
            self.run_scenarios_sequential(scenarios).await
        }
    }

    /// Run scenarios sequentially
    async fn run_scenarios_sequential(
        &self,
        scenarios: Vec<&TestScenario>,
    ) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for scenario in scenarios {
            let result = self.run_scenario(scenario).await?;
            results.push(result.clone());

            {
                let mut all_results = self.results.lock().await;
                all_results.push(result);
            }

            if self.config.fail_fast && !results.last().unwrap().success {
                break;
            }
        }

        Ok(results)
    }

    /// Run scenarios in parallel
    async fn run_scenarios_parallel(
        &self,
        scenarios: Vec<&TestScenario>,
    ) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        use futures::future::join_all;

        let tasks: Vec<_> = scenarios
            .into_iter()
            .map(|scenario| {
                let runner = self;
                async move { runner.run_scenario(scenario).await }
            })
            .collect();

        let results = join_all(tasks).await;

        let mut final_results = Vec::new();
        for result in results {
            let result = result?;
            final_results.push(result.clone());

            {
                let mut all_results = self.results.lock().await;
                all_results.push(result);
            }
        }

        Ok(final_results)
    }

    /// Run a single scenario
    async fn run_scenario(
        &self,
        scenario: &TestScenario,
    ) -> Result<TestResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let mut result = TestResult {
            scenario_name: scenario.name.clone(),
            success: false,
            duration: Duration::default(),
            exit_code: None,
            output: String::new(),
            error_output: String::new(),
            files_created: Vec::new(),
            performance_metrics: None,
            timestamp: chrono::Utc::now(),
        };

        // Create scenario-specific working directory
        let scenario_dir = self.temp_dir.path().join(&scenario.name);
        fs::create_dir_all(&scenario_dir)?;

        // Setup phase
        if let Err(e) = self
            .execute_steps(&scenario.setup_steps, &scenario_dir)
            .await
        {
            result.error_output = format!("Setup failed: {}", e);
            result.duration = start_time.elapsed();
            return Ok(result);
        }

        // Test execution phase
        let test_result = self
            .execute_steps(&scenario.test_steps, &scenario_dir)
            .await;

        // Cleanup phase (always run)
        let _ = self
            .execute_steps(&scenario.cleanup_steps, &scenario_dir)
            .await;

        // Collect results
        result.duration = start_time.elapsed();
        result.success = test_result.is_ok();

        if let Err(e) = test_result {
            result.error_output = format!("Test execution failed: {}", e);
        }

        // Collect performance metrics if enabled
        if self.config.collect_performance_metrics {
            result.performance_metrics =
                Some(self.collect_performance_metrics(&scenario_dir).await);
        }

        // Validate expected results
        if result.success {
            result.success =
                self.validate_expected_results(&scenario.expected_results, &result, &scenario_dir);
        }

        Ok(result)
    }

    /// Execute a series of test steps
    async fn execute_steps(
        &self,
        steps: &[TestStep],
        working_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for step in steps {
            self.execute_step(step, working_dir).await?;
        }
        Ok(())
    }

    /// Execute a single test step
    async fn execute_step(
        &self,
        step: &TestStep,
        working_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let step_working_dir = if let Some(ref dir) = step.working_dir {
            working_dir.join(dir)
        } else {
            working_dir.to_path_buf()
        };

        match &step.action {
            TestAction::CreateFile { path, content } => {
                let file_path = step_working_dir.join(path);
                fs::create_dir_all(file_path.parent().unwrap())?;
                fs::write(&file_path, content)?;
            }
            TestAction::CreateDir { path } => {
                fs::create_dir_all(step_working_dir.join(path))?;
            }
            TestAction::RunCommand { command } => {
                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(command).current_dir(&step_working_dir);

                for (key, value) in &step.env_vars {
                    cmd.env(key, value);
                }

                // TODO: Implement timeout handling for test steps
                // if let Some(timeout) = step.timeout {
                //     // cmd.timeout(timeout);
                // }

                let output = cmd.output()?;

                if step.expect_success && !output.status.success() {
                    return Err(format!(
                        "Command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                    .into());
                }

                // Check expected output
                let stdout = String::from_utf8_lossy(&output.stdout);
                for pattern in &step.expected_output_contains {
                    if !stdout.contains(pattern) {
                        return Err(format!(
                            "Expected output '{}' not found in: {}",
                            pattern, stdout
                        )
                        .into());
                    }
                }
            }
            TestAction::GitCommand { args } => {
                let mut cmd = Command::new("git");
                cmd.args(args).current_dir(&step_working_dir);

                for (key, value) in &step.env_vars {
                    cmd.env(key, value);
                }

                // TODO: Implement timeout handling for test steps
                // if let Some(timeout) = step.timeout {
                //     // cmd.timeout(timeout);
                // }

                let output = cmd.output()?;

                if step.expect_success && !output.status.success() {
                    return Err(format!(
                        "Git command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                    .into());
                }
            }
            TestAction::CodeGuardianCommand { subcommand, args } => {
                let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
                cmd.arg(subcommand);

                for arg in args {
                    cmd.arg(arg);
                }

                cmd.current_dir(&step_working_dir);

                for (key, value) in &step.env_vars {
                    cmd.env(key, value);
                }

                if let Some(timeout) = step.timeout {
                    // cmd.timeout(timeout);
                }

                let output = cmd.output()?;

                if step.expect_success && !output.status.success() {
                    return Err(format!(
                        "CodeGuardian command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                    .into());
                }

                // Check expected output
                let stdout = String::from_utf8_lossy(&output.stdout);
                for pattern in &step.expected_output_contains {
                    if !stdout.contains(pattern) {
                        return Err(format!(
                            "Expected output '{}' not found in: {}",
                            pattern, stdout
                        )
                        .into());
                    }
                }
            }
            TestAction::AssertFileExists { path } => {
                if !step_working_dir.join(path).exists() {
                    return Err(format!("Expected file does not exist: {}", path).into());
                }
            }
            TestAction::AssertFileContains { path, content } => {
                let file_path = step_working_dir.join(path);
                let file_content = fs::read_to_string(&file_path)?;
                if !file_content.contains(content) {
                    return Err(format!(
                        "File {} does not contain expected content: {}",
                        path, content
                    )
                    .into());
                }
            }
            TestAction::Sleep { duration_ms } => {
                tokio::time::sleep(Duration::from_millis(*duration_ms)).await;
            }
            TestAction::Custom { function: _ } => {
                // Custom functions would be implemented here
                // For now, just succeed
            }
            _ => {
                return Err(format!("Unsupported test action: {:?}", step.action).into());
            }
        }

        Ok(())
    }

    /// Validate expected results against actual results
    fn validate_expected_results(
        &self,
        expected: &ExpectedResults,
        actual: &TestResult,
        working_dir: &Path,
    ) -> bool {
        // Check success status
        if expected.success != actual.success {
            return false;
        }

        // Check exit code if specified
        if let Some(expected_code) = expected.exit_code {
            if Some(expected_code) != actual.exit_code {
                return false;
            }
        }

        // Check output contains expected patterns
        for pattern in &expected.output_contains {
            if !actual.output.contains(pattern) {
                return false;
            }
        }

        // Check files were created
        for file_path in &expected.files_created {
            if !working_dir.join(file_path).exists() {
                return false;
            }
        }

        // Check performance requirements
        if let Some(ref perf_req) = expected.performance_requirements {
            if let Some(ref metrics) = actual.performance_metrics {
                if metrics.duration > perf_req.max_duration {
                    return false;
                }

                if let Some(max_mem) = perf_req.max_memory_mb {
                    if let Some(actual_mem) = metrics.memory_usage_mb {
                        if actual_mem > max_mem {
                            return false;
                        }
                    }
                }

                if let Some(min_fps) = perf_req.min_files_per_second {
                    if metrics.files_per_second < min_fps {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Collect performance metrics for a test run
    async fn collect_performance_metrics(&self, _working_dir: &Path) -> PerformanceMetrics {
        // Basic implementation - could be enhanced with actual system monitoring
        PerformanceMetrics {
            duration: Duration::default(),
            memory_usage_mb: None, // Would need system monitoring
            files_processed: 0,
            files_per_second: 0.0,
            cpu_usage_percent: None,
        }
    }

    /// Generate test report
    pub async fn generate_report(
        &self,
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let results = self.results.lock().await;

        let report = TestReport {
            timestamp: chrono::Utc::now(),
            total_scenarios: results.len(),
            passed_scenarios: results.iter().filter(|r| r.success).count(),
            failed_scenarios: results.iter().filter(|r| !r.success).count(),
            total_duration: results.iter().map(|r| r.duration).sum(),
            results: results.clone(),
            performance_summary: self.generate_performance_summary(&results),
        };

        let json = serde_json::to_string_pretty(&report)?;
        fs::write(output_path, json)?;

        Ok(())
    }

    /// Generate performance summary from results
    fn generate_performance_summary(&self, results: &[TestResult]) -> PerformanceSummary {
        let performance_results: Vec<_> = results
            .iter()
            .filter_map(|r| r.performance_metrics.as_ref())
            .collect();

        let avg_duration = if !performance_results.is_empty() {
            performance_results
                .iter()
                .map(|p| p.duration)
                .sum::<Duration>()
                / performance_results.len() as u32
        } else {
            Duration::default()
        };

        PerformanceSummary {
            average_duration: avg_duration,
            total_files_processed: performance_results.iter().map(|p| p.files_processed).sum(),
            average_files_per_second: performance_results
                .iter()
                .map(|p| p.files_per_second)
                .sum::<f64>()
                / performance_results.len() as f64,
        }
    }

    /// Compare results against baseline for regression detection
    pub async fn check_performance_regression(
        &self,
        baseline_path: &Path,
    ) -> Result<RegressionReport, Box<dyn std::error::Error>> {
        let baseline_content = fs::read_to_string(baseline_path)?;
        let baseline_report: TestReport = serde_json::from_str(&baseline_content)?;

        let current_results = self.results.lock().await;

        let regressions: Vec<_> = current_results
            .iter()
            .filter_map(|current| {
                let baseline_result = baseline_report
                    .results
                    .iter()
                    .find(|b| b.scenario_name == current.scenario_name)?;

                if let (Some(current_perf), Some(baseline_perf)) = (
                    &current.performance_metrics,
                    &baseline_result.performance_metrics,
                ) {
                    let duration_regression =
                        current_perf.duration > baseline_perf.duration.mul_f32(1.1); // 10% regression
                    let throughput_regression =
                        current_perf.files_per_second < baseline_perf.files_per_second * 0.9; // 10% drop

                    if duration_regression || throughput_regression {
                        Some(PerformanceRegression {
                            scenario_name: current.scenario_name.clone(),
                            duration_regression: if duration_regression {
                                Some(RegressionDetail {
                                    baseline: baseline_perf.duration,
                                    current: current_perf.duration,
                                    percentage_change: ((current_perf.duration.as_millis() as f64
                                        - baseline_perf.duration.as_millis() as f64)
                                        / baseline_perf.duration.as_millis() as f64)
                                        * 100.0,
                                })
                            } else {
                                None
                            },
                            throughput_regression: if throughput_regression {
                                Some(RegressionDetail {
                                    baseline: Duration::from_secs_f64(
                                        1.0 / baseline_perf.files_per_second,
                                    ),
                                    current: Duration::from_secs_f64(
                                        1.0 / current_perf.files_per_second,
                                    ),
                                    percentage_change: ((baseline_perf.files_per_second
                                        - current_perf.files_per_second)
                                        / baseline_perf.files_per_second)
                                        * 100.0,
                                })
                            } else {
                                None
                            },
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let total_regressions = regressions.len();

        Ok(RegressionReport {
            timestamp: chrono::Utc::now(),
            regressions,
            total_regressions,
        })
    }
}

/// Test report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub total_duration: Duration,
    pub results: Vec<TestResult>,
    pub performance_summary: PerformanceSummary,
}

/// Performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub average_duration: Duration,
    pub total_files_processed: u64,
    pub average_files_per_second: f64,
}

/// Performance regression report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub regressions: Vec<PerformanceRegression>,
    pub total_regressions: usize,
}

/// Individual performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    pub scenario_name: String,
    pub duration_regression: Option<RegressionDetail>,
    pub throughput_regression: Option<RegressionDetail>,
}

/// Regression detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionDetail {
    pub baseline: Duration,
    pub current: Duration,
    pub percentage_change: f64,
}

// Helper functions for E2E tests
pub fn create_sample_rust_project(temp_dir: &TempDir) {
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"
[package]
name = "sample-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
}
"#,
    )
    .unwrap();
}

pub fn create_sample_javascript_project(temp_dir: &TempDir) {
    fs::write(
        temp_dir.path().join("package.json"),
        r#"
{
  "name": "sample-app",
  "version": "1.0.0"
}
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("index.js"),
        r#"
console.log("Hello, world!");
"#,
    )
    .unwrap();
}

pub fn create_git_repository(temp_dir: &TempDir) {
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
}

#[cfg(test)]
mod integration_helpers {
    use super::*;

    #[test]
    fn test_helper_rust_project_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_sample_rust_project(&temp_dir);

        assert!(temp_dir.path().join("Cargo.toml").exists());
        assert!(temp_dir.path().join("src/main.rs").exists());
    }

    #[test]
    fn test_helper_javascript_project_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_sample_javascript_project(&temp_dir);

        assert!(temp_dir.path().join("package.json").exists());
        assert!(temp_dir.path().join("index.js").exists());
    }

    #[test]
    fn test_helper_git_repository_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_git_repository(&temp_dir);

        assert!(temp_dir.path().join(".git").exists());
    }
}

/// Comprehensive E2E test suite using the scenario runner
#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_full_development_workflow_scenario() {
        let mut runner = E2ETestRunner::new();

        let scenario = TestScenario {
            name: "full_development_workflow".to_string(),
            description: "Complete development workflow from project creation to security analysis"
                .to_string(),
            category: ScenarioCategory::BasicWorkflow,
            priority: TestPriority::Critical,
            platforms: vec![
                "linux".to_string(),
                "macos".to_string(),
                "windows".to_string(),
            ],
            timeout: Duration::from_secs(300),
            setup_steps: vec![
                TestStep {
                    name: "create_project_structure".to_string(),
                    action: TestAction::CreateDir {
                        path: "src".to_string(),
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: None,
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
                TestStep {
                    name: "create_cargo_toml".to_string(),
                    action: TestAction::CreateFile {
                        path: "Cargo.toml".to_string(),
                        content: r#"
[package]
name = "sample-project"
version = "0.1.0"
edition = "2021"
"#
                        .to_string(),
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: None,
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec!["Cargo.toml".to_string()],
                },
                TestStep {
                    name: "create_main_rs".to_string(),
                    action: TestAction::CreateFile {
                        path: "src/main.rs".to_string(),
                        content: r#"
fn main() {
    println!("Hello, world!");
}
"#
                        .to_string(),
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: None,
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec!["src/main.rs".to_string()],
                },
                TestStep {
                    name: "init_git_repo".to_string(),
                    action: TestAction::GitCommand {
                        args: vec!["init".to_string()],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(10)),
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
                TestStep {
                    name: "git_config".to_string(),
                    action: TestAction::GitCommand {
                        args: vec![
                            "config".to_string(),
                            "user.email".to_string(),
                            "test@example.com".to_string(),
                        ],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(5)),
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
            ],
            test_steps: vec![
                TestStep {
                    name: "init_codeguardian".to_string(),
                    action: TestAction::CodeGuardianCommand {
                        subcommand: "init".to_string(),
                        args: vec![],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(30)),
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec!["codeguardian.toml".to_string()],
                },
                TestStep {
                    name: "add_security_issue".to_string(),
                    action: TestAction::CreateFile {
                        path: "src/lib.rs".to_string(),
                        content: r#"
pub fn authenticate(password: &str) -> bool {
    let hardcoded_pass = "admin123"; // Security issue
    password == hardcoded_pass
}

pub fn process_data() {
    println!("Processing...");
}
"#
                        .to_string(),
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: None,
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec!["src/lib.rs".to_string()],
                },
                TestStep {
                    name: "git_add_and_commit".to_string(),
                    action: TestAction::GitCommand {
                        args: vec!["add".to_string(), ".".to_string()],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(10)),
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
                TestStep {
                    name: "git_commit".to_string(),
                    action: TestAction::GitCommand {
                        args: vec![
                            "commit".to_string(),
                            "-m".to_string(),
                            "Initial commit".to_string(),
                        ],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(10)),
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
                TestStep {
                    name: "run_full_analysis".to_string(),
                    action: TestAction::CodeGuardianCommand {
                        subcommand: "check".to_string(),
                        args: vec![".".to_string(), "--format".to_string(), "json".to_string()],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(60)),
                    expect_success: true,
                    expected_output_contains: vec!["hardcoded_secret".to_string()],
                    expected_files: vec![],
                },
                TestStep {
                    name: "fix_security_issue".to_string(),
                    action: TestAction::CreateFile {
                        path: "src/lib.rs".to_string(),
                        content: r#"
use std::env;

pub fn authenticate(password: &str) -> bool {
    let expected_pass = env::var("ADMIN_PASSWORD").unwrap_or_default();
    password == expected_pass
}

pub fn process_data() {
    println!("Processing data...");
}
"#
                        .to_string(),
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: None,
                    expect_success: true,
                    expected_output_contains: vec![],
                    expected_files: vec![],
                },
                TestStep {
                    name: "run_diff_analysis".to_string(),
                    action: TestAction::CodeGuardianCommand {
                        subcommand: "check".to_string(),
                        args: vec![
                            ".".to_string(),
                            "--diff".to_string(),
                            "HEAD".to_string(),
                            "--format".to_string(),
                            "json".to_string(),
                        ],
                    },
                    args: vec![],
                    env_vars: HashMap::new(),
                    working_dir: None,
                    timeout: Some(Duration::from_secs(60)),
                    expect_success: true,
                    expected_output_contains: vec![], // Should have fewer issues
                    expected_files: vec![],
                },
            ],
            cleanup_steps: vec![TestStep {
                name: "cleanup_temp_files".to_string(),
                action: TestAction::RunCommand {
                    command: "rm -rf results.json codeguardian.toml".to_string(),
                },
                args: vec![],
                env_vars: HashMap::new(),
                working_dir: None,
                timeout: Some(Duration::from_secs(5)),
                expect_success: false, // Allow failure for cleanup
                expected_output_contains: vec![],
                expected_files: vec![],
            }],
            expected_results: ExpectedResults {
                success: true,
                exit_code: Some(0),
                output_contains: vec!["hardcoded_secret".to_string()],
                files_created: vec!["codeguardian.toml".to_string(), "src/lib.rs".to_string()],
                performance_requirements: Some(PerformanceRequirements {
                    max_duration: Duration::from_secs(120),
                    max_memory_mb: Some(500),
                    min_files_per_second: Some(1.0),
                }),
            },
            tags: vec![
                "workflow".to_string(),
                "security".to_string(),
                "git".to_string(),
            ],
        };

        runner.add_scenario(scenario);
        let results = runner.run_all_scenarios().await.unwrap();

        assert_eq!(results.len(), 1);
        assert!(
            results[0].success,
            "Full development workflow scenario should pass"
        );

        // Generate report
        let report_path = runner.config.output_directory.join("e2e_test_report.json");
        fs::create_dir_all(&runner.config.output_directory).unwrap();
        runner.generate_report(&report_path).await.unwrap();
        assert!(report_path.exists());
    }

    #[tokio::test]
    async fn test_error_handling_scenarios() {
        let mut runner = E2ETestRunner::new();

        // Scenario for testing error handling
        let scenario = TestScenario {
            name: "error_handling_invalid_path".to_string(),
            description: "Test error handling when analyzing non-existent paths".to_string(),
            category: ScenarioCategory::ErrorHandling,
            priority: TestPriority::High,
            platforms: vec![
                "linux".to_string(),
                "macos".to_string(),
                "windows".to_string(),
            ],
            timeout: Duration::from_secs(30),
            setup_steps: vec![],
            test_steps: vec![TestStep {
                name: "run_check_on_invalid_path".to_string(),
                action: TestAction::CodeGuardianCommand {
                    subcommand: "check".to_string(),
                    args: vec!["/nonexistent/path/that/does/not/exist".to_string()],
                },
                args: vec![],
                env_vars: HashMap::new(),
                working_dir: None,
                timeout: Some(Duration::from_secs(10)),
                expect_success: false, // Should fail gracefully
                expected_output_contains: vec![],
                expected_files: vec![],
            }],
            cleanup_steps: vec![],
            expected_results: ExpectedResults {
                success: false, // Expect failure for invalid path
                exit_code: None,
                output_contains: vec![],
                files_created: vec![],
                performance_requirements: None,
            },
            tags: vec!["error-handling".to_string(), "edge-case".to_string()],
        };

        runner.add_scenario(scenario);
        let results = runner.run_all_scenarios().await.unwrap();

        assert_eq!(results.len(), 1);
        // Error handling scenarios should fail as expected
        assert!(
            !results[0].success,
            "Error handling scenario should fail as expected"
        );
    }

    #[tokio::test]
    async fn test_performance_regression_detection() {
        let mut runner = E2ETestRunner::new();

        // Create a simple performance scenario
        let scenario = TestScenario {
            name: "performance_test_small".to_string(),
            description: "Small performance test for regression detection".to_string(),
            category: ScenarioCategory::Performance,
            priority: TestPriority::Medium,
            platforms: vec![
                "linux".to_string(),
                "macos".to_string(),
                "windows".to_string(),
            ],
            timeout: Duration::from_secs(60),
            setup_steps: vec![TestStep {
                name: "create_test_files".to_string(),
                action: TestAction::CreateFile {
                    path: "test.rs".to_string(),
                    content: "fn main() {}".to_string(),
                },
                args: vec![],
                env_vars: HashMap::new(),
                working_dir: None,
                timeout: None,
                expect_success: true,
                expected_output_contains: vec![],
                expected_files: vec!["test.rs".to_string()],
            }],
            test_steps: vec![TestStep {
                name: "run_performance_check".to_string(),
                action: TestAction::CodeGuardianCommand {
                    subcommand: "check".to_string(),
                    args: vec![
                        "test.rs".to_string(),
                        "--format".to_string(),
                        "json".to_string(),
                    ],
                },
                args: vec![],
                env_vars: HashMap::new(),
                working_dir: None,
                timeout: Some(Duration::from_secs(30)),
                expect_success: true,
                expected_output_contains: vec!["Files scanned".to_string()],
                expected_files: vec![],
            }],
            cleanup_steps: vec![],
            expected_results: ExpectedResults {
                success: true,
                exit_code: Some(0),
                output_contains: vec!["Files scanned".to_string()],
                files_created: vec![],
                performance_requirements: Some(PerformanceRequirements {
                    max_duration: Duration::from_secs(10),
                    max_memory_mb: Some(100),
                    min_files_per_second: Some(0.1),
                }),
            },
            tags: vec!["performance".to_string(), "regression".to_string()],
        };

        runner.add_scenario(scenario);
        let results = runner.run_all_scenarios().await.unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].success, "Performance test should pass");

        // Test report generation
        let report_path = runner
            .config
            .output_directory
            .join("performance_test_report.json");
        fs::create_dir_all(&runner.config.output_directory).unwrap();
        runner.generate_report(&report_path).await.unwrap();
        assert!(report_path.exists());
    }
}
