//! # Metrics Integration
//!
//! Integration utilities for incorporating metrics collection into output operations.

use super::*;
use crate::types::AnalysisResults;
use crate::output::{OutputResult, OutputFormat};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Metrics-enabled output processor
#[derive(Debug)]
pub struct MetricsEnabledOutputProcessor {
    metrics_service: Arc<Mutex<OutputMetricsService>>,
}

impl MetricsEnabledOutputProcessor {
    /// Create a new metrics-enabled output processor
    pub fn new() -> Self {
        Self {
            metrics_service: Arc::new(Mutex::new(OutputMetricsService::new())),
        }
    }

    /// Process analysis results with metrics collection
    pub async fn process_with_metrics(
        &self,
        results: &AnalysisResults,
        format: OutputFormat,
        formatter: &dyn crate::output::OutputFormatter,
    ) -> Result<OutputResult> {
        let start_time = std::time::Instant::now();

        // Format the results
        let output_result = formatter.format(results)?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        // Record metrics
        let mut service = self.metrics_service.lock().await;
        service.record_output_metrics(
            results,
            &output_result,
            &format.to_string(),
            generation_time_ms,
        ).await?;

        Ok(output_result)
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> Result<SystemHealth> {
        let service = self.metrics_service.lock().await;
        service.get_health_status().await
    }

    /// Generate metrics report
    pub async fn generate_report(&self) -> Result<MetricsReport> {
        let service = self.metrics_service.lock().await;
        service.generate_report(None).await
    }

    /// Get metrics service reference
    pub fn metrics_service(&self) -> Arc<Mutex<OutputMetricsService>> {
        Arc::clone(&self.metrics_service)
    }
}

/// Metrics middleware for output operations
pub struct MetricsMiddleware {
    processor: MetricsEnabledOutputProcessor,
}

impl MetricsMiddleware {
    /// Create new metrics middleware
    pub fn new() -> Self {
        Self {
            processor: MetricsEnabledOutputProcessor::new(),
        }
    }

    /// Wrap an output operation with metrics collection
    pub async fn wrap_operation<F, Fut>(
        &self,
        operation: F,
        results: &AnalysisResults,
        format: OutputFormat,
    ) -> Result<OutputResult>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<OutputResult>>,
    {
        let start_time = std::time::Instant::now();

        // Execute the operation
        let output_result = operation.await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        // Record metrics
        let service = self.processor.metrics_service();
        let mut service_lock = service.lock().await;
        service_lock.record_output_metrics(
            results,
            &output_result,
            &format.to_string(),
            generation_time_ms,
        ).await?;

        Ok(output_result)
    }
}

/// Example usage of the metrics framework
pub async fn example_usage() -> Result<()> {
    // Create metrics-enabled processor
    let processor = MetricsEnabledOutputProcessor::new();

    // Simulate some analysis results
    let results = AnalysisResults::new("test_config".to_string());

    // Create a simple formatter (in real usage, you'd use actual formatters)
    struct MockFormatter;
    impl crate::output::OutputFormatter for MockFormatter {
        fn format(&self, _results: &AnalysisResults) -> Result<OutputResult> {
            Ok(OutputResult::new("mock output".to_string(), "json", "test".to_string()))
        }

        fn content_type(&self) -> &'static str { "application/json" }
        fn metadata(&self) -> crate::output::FormatMetadata {
            crate::output::FormatMetadata {
                name: "mock",
                version: "1.0",
                supports_compression: false,
                max_recommended_size: None,
            }
        }
        fn file_extension(&self) -> &'static str { "json" }
    }

    let formatter = MockFormatter;

    // Process with metrics
    let output_result = processor.process_with_metrics(
        &results,
        OutputFormat::Json,
        &formatter,
    ).await?;

    println!("Output generated: {}", output_result.content);

    // Get health status
    let health = processor.get_health_status().await?;
    println!("System health: {:?}", health.overall_status);

    // Generate report
    let report = processor.generate_report().await?;
    println!("Generated report with {} recommendations", report.recommendations.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_processor() -> Result<(), Box<dyn std::error::Error>> {
        let processor = MetricsEnabledOutputProcessor::new();

        // Test health status
        let health = processor.get_health_status().await?;
        assert_eq!(health.overall_status, HealthStatus::Unknown); // No data yet

        // Test report generation
        let report = processor.generate_report().await?;
        assert!(report.summary.total_operations == 0); // No operations recorded yet
    }

    #[tokio::test]
    async fn test_metrics_middleware() -> Result<(), Box<dyn std::error::Error>> {
        let middleware = MetricsMiddleware::new();

        // Simulate an operation
        let results = AnalysisResults::new("test".to_string());

        let output_result = middleware.wrap_operation(
            || async {
                Ok(OutputResult::new("test output".to_string(), "json", "test".to_string()))
            },
            &results,
            OutputFormat::Json,
        ).await?;

        assert_eq!(output_result.content, "test output");
    }
}
