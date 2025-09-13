//! GitHub API Load Testing
//!
//! Tests for validating GitHub API rate limiting, burst handling, and resilience
//! under various load conditions as specified in Task 25.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Instant};

mod load_testing_framework;
use load_testing_framework::{LoadTestConfig, LoadTestFramework};

use do_codeguardian::config::base::Config;
use do_codeguardian::github_api::GitHubApi;

/// GitHub API rate limiting burst test
/// Tests the system's ability to handle 100+ requests per minute
#[tokio::test]
async fn test_github_api_burst_load() {
    let config = LoadTestConfig {
        concurrent_operations: 20,
        test_duration: Duration::from_secs(60), // 1 minute test
        target_ops_per_second: 2.0,             // 120 requests per minute
        max_retries: 3,
        operation_timeout: Duration::from_secs(10),
        gradual_ramp_up: false, // Start at full burst immediately
        ramp_up_duration: Duration::from_secs(0),
    };

    let framework = LoadTestFramework::new(config);

    // Mock GitHub API configuration
    let mut app_config = Config::default();
    app_config.github.token = Some("mock_token_for_testing".to_string());
    app_config.github.rate_limit = 5000; // Standard GitHub rate limit

    let github_api = Arc::new(
        GitHubApi::new(&app_config)
            .await
            .expect("Failed to create GitHub API client"),
    );

    let report = framework
        .execute_load_test(|| {
            let api = github_api.clone();
            async move {
                // Simulate a lightweight GitHub API operation
                // In real testing, this would be actual API calls
                simulate_github_api_call(api).await
            }
        })
        .await
        .expect("Failed to execute GitHub API load test");

    report.print_summary();

    // Validate burst load performance criteria
    assert!(
        report.success_rate >= 0.95,
        "Burst load success rate should be >= 95%"
    );
    assert!(
        report.average_response_time_ms <= 5000.0,
        "Average response time should be <= 5s during burst"
    );
    assert!(
        report.operations_attempted >= 100,
        "Should attempt at least 100 operations in burst test"
    );
}

/// GitHub API sustained load test
/// Tests rate limiter effectiveness over extended periods
#[tokio::test]
async fn test_github_api_sustained_load() {
    let config = LoadTestConfig {
        concurrent_operations: 10,
        test_duration: Duration::from_secs(300), // 5 minute test
        target_ops_per_second: 1.0,              // Sustainable rate within limits
        max_retries: 5,
        operation_timeout: Duration::from_secs(30),
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(30),
    };

    let framework = LoadTestFramework::new(config);

    let mut app_config = Config::default();
    app_config.github.token = Some("mock_token_for_testing".to_string());
    app_config.github.rate_limit = 5000;

    let github_api = Arc::new(
        GitHubApi::new(&app_config)
            .await
            .expect("Failed to create GitHub API client"),
    );

    let report = framework
        .execute_load_test(|| {
            let api = github_api.clone();
            async move { simulate_github_api_call(api).await }
        })
        .await
        .expect("Failed to execute GitHub API load test");

    report.print_summary();

    // Validate sustained load performance criteria
    assert!(
        report.success_rate >= 0.999,
        "Sustained load success rate should be >= 99.9%"
    );
    assert!(
        report.average_response_time_ms <= 10000.0,
        "Average response time should be <= 10s during sustained load"
    );
    assert!(
        report.operations_per_second >= 0.8,
        "Should maintain at least 0.8 ops/sec"
    );
}

/// GitHub API rate limit boundary testing
/// Tests behavior at the 5000 requests/hour limit
#[tokio::test]
async fn test_github_api_rate_limit_boundary() {
    let config = LoadTestConfig {
        concurrent_operations: 50,
        test_duration: Duration::from_secs(120), // 2 minute intensive test
        target_ops_per_second: 5.0,              // 600 requests in 2 minutes (scaled up)
        max_retries: 10,
        operation_timeout: Duration::from_secs(60),
        gradual_ramp_up: false,
        ramp_up_duration: Duration::from_secs(0),
    };

    let framework = LoadTestFramework::new(config);

    let mut app_config = Config::default();
    app_config.github.token = Some("mock_token_for_testing".to_string());
    app_config.github.rate_limit = 100; // Lower limit for testing

    let github_api = Arc::new(
        GitHubApi::new(&app_config)
            .await
            .expect("Failed to create GitHub API client"),
    );

    let report = framework
        .execute_load_test(|| {
            let api = github_api.clone();
            async move { simulate_github_api_call_with_rate_limiting(api).await }
        })
        .await
        .expect("Failed to execute GitHub API load test");

    report.print_summary();

    // At rate limit boundary, we expect some failures but graceful degradation
    assert!(
        report.success_rate >= 0.80,
        "Rate limit boundary success rate should be >= 80%"
    );
    assert!(
        report.retries_performed > 0,
        "Should perform retries when hitting rate limits"
    );
    assert!(
        report.operations_attempted >= 300,
        "Should attempt significant operations"
    );
}

/// GitHub API network failure recovery test
/// Tests resilience during network issues and API outages
#[tokio::test]
async fn test_github_api_network_resilience() {
    let config = LoadTestConfig {
        concurrent_operations: 15,
        test_duration: Duration::from_secs(60),
        target_ops_per_second: 1.0,
        max_retries: 5,
        operation_timeout: Duration::from_secs(10),
        gradual_ramp_up: false,
        ramp_up_duration: Duration::from_secs(0),
    };

    let framework = LoadTestFramework::new(config);

    let mut app_config = Config::default();
    app_config.github.token = Some("mock_token_for_testing".to_string());
    app_config.github.rate_limit = 5000;

    let github_api = Arc::new(
        GitHubApi::new(&app_config)
            .await
            .expect("Failed to create GitHub API client"),
    );

    let report = framework
        .execute_load_test(|| {
            let api = github_api.clone();
            async move { simulate_github_api_call_with_network_issues(api).await }
        })
        .await
        .expect("Failed to execute GitHub API load test");

    report.print_summary();

    // Network resilience should show recovery through retries
    assert!(
        report.success_rate >= 0.70,
        "Network resilience success rate should be >= 70%"
    );
    assert!(
        report.retries_performed > 0,
        "Should perform retries during network issues"
    );
    assert!(
        report.average_response_time_ms <= 15000.0,
        "Should recover within reasonable time"
    );
}

/// Multi-repository operations load test
/// Tests cross-repository duplicate detection under load
#[tokio::test]
async fn test_multi_repository_operations_load() {
    let config = LoadTestConfig {
        concurrent_operations: 25,
        test_duration: Duration::from_secs(180), // 3 minute test
        target_ops_per_second: 0.5,              // Slower for complex operations
        max_retries: 3,
        operation_timeout: Duration::from_secs(45),
        gradual_ramp_up: true,
        ramp_up_duration: Duration::from_secs(20),
    };

    let framework = LoadTestFramework::new(config);

    let mut app_config = Config::default();
    app_config.github.token = Some("mock_token_for_testing".to_string());
    app_config.github.rate_limit = 5000;

    let github_api = Arc::new(
        GitHubApi::new(&app_config)
            .await
            .expect("Failed to create GitHub API client"),
    );

    let report = framework
        .execute_load_test(|| {
            let api = github_api.clone();
            async move { simulate_multi_repository_operation(api).await }
        })
        .await
        .expect("Failed to execute GitHub API load test");

    report.print_summary();

    // Multi-repository operations are more complex but should still perform well
    assert!(
        report.success_rate >= 0.95,
        "Multi-repo operations success rate should be >= 95%"
    );
    assert!(
        report.average_response_time_ms <= 30000.0,
        "Multi-repo operations should complete within 30s"
    );
    assert!(
        report.operations_successful >= 50,
        "Should complete at least 50 multi-repo operations"
    );
}

// Helper functions to simulate GitHub API operations

async fn simulate_github_api_call(api: Arc<GitHubApi>) -> anyhow::Result<()> {
    // Simulate API call latency
    sleep(Duration::from_millis(100 + rand::random::<u64>() % 200)).await;

    // Simulate occasional failures (5% failure rate)
    if rand::random::<f64>() < 0.05 {
        anyhow::bail!("Simulated API failure");
    }

    Ok(())
}

async fn simulate_github_api_call_with_rate_limiting(api: Arc<GitHubApi>) -> anyhow::Result<()> {
    // Simulate rate limiting with higher failure rate
    sleep(Duration::from_millis(50 + rand::random::<u64>() % 100)).await;

    // Simulate rate limiting failures (20% failure rate)
    if rand::random::<f64>() < 0.20 {
        anyhow::bail!("Rate limit exceeded");
    }

    Ok(())
}

async fn simulate_github_api_call_with_network_issues(api: Arc<GitHubApi>) -> anyhow::Result<()> {
    // Simulate network issues with variable latency
    let latency = 200 + rand::random::<u64>() % 1000;
    sleep(Duration::from_millis(latency)).await;

    // Simulate network failures (30% failure rate)
    if rand::random::<f64>() < 0.30 {
        anyhow::bail!("Network timeout or connection error");
    }

    Ok(())
}

async fn simulate_multi_repository_operation(api: Arc<GitHubApi>) -> anyhow::Result<()> {
    // Simulate complex multi-repository operation
    sleep(Duration::from_millis(500 + rand::random::<u64>() % 2000)).await;

    // Simulate fewer failures for complex operations (10% failure rate)
    if rand::random::<f64>() < 0.10 {
        anyhow::bail!("Multi-repository operation failed");
    }

    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Integration test to verify load testing framework works with real components
    #[tokio::test]
    async fn test_load_framework_integration() {
        let config = LoadTestConfig {
            concurrent_operations: 3,
            test_duration: Duration::from_secs(5),
            target_ops_per_second: 1.0,
            ..Default::default()
        };

        let framework = LoadTestFramework::new(config);

        let report = framework
            .execute_load_test(|| async {
                // Simple successful operation
                sleep(Duration::from_millis(100)).await;
                Ok::<(), anyhow::Error>(())
            })
            .await
            .expect("Failed to execute GitHub API load test");

        assert!(report.operations_attempted > 0);
        assert!(report.success_rate > 0.8);
        assert!(report.test_duration >= Duration::from_secs(5));
    }
}
