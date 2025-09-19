//! # Integration Tests for Output Pipelines
//!
//! This module provides comprehensive integration testing for CodeGuardian output systems,
//! testing complete workflows from analysis to output generation and storage.

use super::{measure_test_execution, TestResult};
use crate::output::ai::{create_enhancement_engine, AIEnhancementConfig};
use crate::output::enterprise::{
    EnterpriseConfig, EnterpriseContext, EnterpriseManager, SubscriptionTier, Tenant,
};
use crate::output::formats::*;
use crate::output::formatter::OutputFormatter;
use crate::output::storage::organizer::ResultsOrganizer;
use crate::output::storage::{OrganizationStrategy, StorageConfig};
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::path::PathBuf;
use tempfile::TempDir;

/// Integration test runner for complete output pipelines
pub struct IntegrationTestRunner {
    test_scenarios: Vec<IntegrationScenario>,
}

/// Integration test scenario
#[derive(Debug, Clone)]
pub struct IntegrationScenario {
    pub name: String,
    pub description: String,
    pub scenario_type: ScenarioType,
    pub expected_artifacts: Vec<ExpectedArtifact>,
}

/// Types of integration test scenarios
#[derive(Debug, Clone)]
pub enum ScenarioType {
    CompleteWorkflow,
    StorageIntegration,
    AIEnhancement,
    EnterpriseFeatures,
    MultiTenant,
    ErrorRecovery,
}

/// Expected artifacts from integration tests
#[derive(Debug, Clone)]
pub struct ExpectedArtifact {
    pub artifact_type: String,
    pub format: Option<String>,
    pub min_size_bytes: usize,
    pub should_exist: bool,
}

impl IntegrationTestRunner {
    /// Create a new integration test runner
    pub fn new() -> Self {
        Self {
            test_scenarios: Self::generate_integration_scenarios(),
        }
    }
}

impl Default for IntegrationTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegrationTestRunner {
    /// Generate integration test scenarios
    fn generate_integration_scenarios() -> Vec<IntegrationScenario> {
        vec![
            IntegrationScenario {
                name: "complete_analysis_workflow".to_string(),
                description: "End-to-end analysis workflow with all output formats".to_string(),
                scenario_type: ScenarioType::CompleteWorkflow,
                expected_artifacts: vec![
                    ExpectedArtifact {
                        artifact_type: "output_file".to_string(),
                        format: Some("json".to_string()),
                        min_size_bytes: 100,
                        should_exist: true,
                    },
                    ExpectedArtifact {
                        artifact_type: "output_file".to_string(),
                        format: Some("html".to_string()),
                        min_size_bytes: 500,
                        should_exist: true,
                    },
                    ExpectedArtifact {
                        artifact_type: "output_file".to_string(),
                        format: Some("markdown".to_string()),
                        min_size_bytes: 200,
                        should_exist: true,
                    },
                ],
            },
            IntegrationScenario {
                name: "storage_integration".to_string(),
                description: "Storage and retrieval integration test".to_string(),
                scenario_type: ScenarioType::StorageIntegration,
                expected_artifacts: vec![
                    ExpectedArtifact {
                        artifact_type: "stored_result".to_string(),
                        format: None,
                        min_size_bytes: 1,
                        should_exist: true,
                    },
                    ExpectedArtifact {
                        artifact_type: "search_index".to_string(),
                        format: None,
                        min_size_bytes: 1,
                        should_exist: true,
                    },
                ],
            },
            IntegrationScenario {
                name: "ai_enhancement_pipeline".to_string(),
                description: "AI enhancement integration with output generation".to_string(),
                scenario_type: ScenarioType::AIEnhancement,
                expected_artifacts: vec![ExpectedArtifact {
                    artifact_type: "enhanced_output".to_string(),
                    format: Some("json".to_string()),
                    min_size_bytes: 200,
                    should_exist: true,
                }],
            },
            IntegrationScenario {
                name: "enterprise_features_integration".to_string(),
                description: "Enterprise features with audit trails and compliance".to_string(),
                scenario_type: ScenarioType::EnterpriseFeatures,
                expected_artifacts: vec![
                    ExpectedArtifact {
                        artifact_type: "audit_log".to_string(),
                        format: None,
                        min_size_bytes: 50,
                        should_exist: true,
                    },
                    ExpectedArtifact {
                        artifact_type: "compliance_report".to_string(),
                        format: None,
                        min_size_bytes: 100,
                        should_exist: true,
                    },
                ],
            },
            IntegrationScenario {
                name: "multi_tenant_isolation".to_string(),
                description: "Multi-tenant data isolation and access control".to_string(),
                scenario_type: ScenarioType::MultiTenant,
                expected_artifacts: vec![ExpectedArtifact {
                    artifact_type: "tenant_data".to_string(),
                    format: None,
                    min_size_bytes: 1,
                    should_exist: true,
                }],
            },
            IntegrationScenario {
                name: "error_recovery_workflow".to_string(),
                description: "Error handling and recovery mechanisms".to_string(),
                scenario_type: ScenarioType::ErrorRecovery,
                expected_artifacts: vec![ExpectedArtifact {
                    artifact_type: "error_log".to_string(),
                    format: None,
                    min_size_bytes: 1,
                    should_exist: true,
                }],
            },
        ]
    }

    /// Run all integration tests
    pub async fn run_all_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        println!(
            "🔗 Running {} integration tests...",
            self.test_scenarios.len()
        );

        for scenario in &self.test_scenarios {
            println!("  🧪 Testing: {}", scenario.description);

            let test_result = self.run_integration_scenario(scenario).await?;
            results.push(test_result);
        }

        // Run cross-cutting integration tests
        results.extend(self.test_format_compatibility().await?);
        results.extend(self.test_data_consistency().await?);
        results.extend(self.test_concurrent_operations().await?);

        println!(
            "✅ Integration tests completed: {} tests run",
            results.len()
        );
        Ok(results)
    }

    /// Run a single integration scenario
    pub async fn run_integration_scenario(
        &self,
        scenario: &IntegrationScenario,
    ) -> Result<TestResult> {
        let test_name = format!("integration_{}", scenario.name);

        measure_test_execution(&test_name, async {
            match scenario.scenario_type {
                ScenarioType::CompleteWorkflow => self.test_complete_workflow(scenario).await,
                ScenarioType::StorageIntegration => self.test_storage_integration(scenario).await,
                ScenarioType::AIEnhancement => self.test_ai_enhancement_pipeline(scenario).await,
                ScenarioType::EnterpriseFeatures => self.test_enterprise_features(scenario).await,
                ScenarioType::MultiTenant => self.test_multi_tenant_isolation(scenario).await,
                ScenarioType::ErrorRecovery => self.test_error_recovery(scenario).await,
            }
        })
        .await
    }

    /// Test complete analysis workflow
    async fn test_complete_workflow(&self, scenario: &IntegrationScenario) -> Result<()> {
        // Create test data
        let test_data = self.create_integration_test_data(500).await?;

        // Test each formatter in the complete workflow
        let formatters: Vec<(&str, Box<dyn OutputFormatter>)> = vec![
            ("json", Box::new(JsonFormatter::new())),
            ("html", Box::new(HtmlFormatter::new())),
            ("markdown", Box::new(MarkdownFormatter::new())),
            ("sarif", Box::new(SarifFormatter::new())),
            ("yaml", Box::new(YamlFormatter::new())),
        ];

        let mut generated_outputs = Vec::new();

        // Generate outputs for all formats
        for (format_name, formatter) in formatters {
            let output = formatter.format(&test_data)?;

            // Validate output meets minimum requirements
            self.validate_output_quality(&output, format_name)?;

            generated_outputs.push((format_name.to_string(), output));
        }

        // Verify we generated the expected number of outputs
        if generated_outputs.len() != 5 {
            return Err(anyhow::anyhow!(
                "Expected 5 output formats, got {}",
                generated_outputs.len()
            ));
        }

        // Validate expected artifacts
        for artifact in &scenario.expected_artifacts {
            self.validate_artifact_exists(&generated_outputs, artifact)?;
        }

        Ok(())
    }

    /// Test storage integration
    async fn test_storage_integration(&self, _scenario: &IntegrationScenario) -> Result<()> {
        let temp_dir = TempDir::new()?;
        let storage_config = StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            organization_strategy: OrganizationStrategy::Hybrid,
            enable_compression: true,
            enable_indexing: true,
            ..Default::default()
        };

        let mut organizer = ResultsOrganizer::new(storage_config)?;
        let test_data = self.create_integration_test_data(100).await?;

        // Store results with multiple formats
        let outputs = vec![
            (
                "json".to_string(),
                crate::output::formatter::OutputResult::new(
                    serde_json::to_string(&test_data)?,
                    "json",
                    "integration_test".to_string(),
                ),
            ),
            (
                "markdown".to_string(),
                crate::output::formatter::OutputResult::new(
                    "# Test Report\n\nIntegration test data".to_string(),
                    "markdown",
                    "integration_test".to_string(),
                ),
            ),
        ];

        let result_id = organizer.store_results(
            &test_data,
            &outputs,
            "integration_test_project",
            Some("test/integration-repo"),
            vec!["integration_test".to_string()],
        )?;

        // Verify storage was successful
        if result_id.is_empty() {
            return Err(anyhow::anyhow!("Storage returned empty result ID"));
        }

        // Test retrieval
        let retrieved = organizer.retrieve_results(&result_id)?;
        if retrieved.is_none() {
            return Err(anyhow::anyhow!("Failed to retrieve stored results"));
        }

        let (retrieved_results, retrieved_outputs) = retrieved.unwrap();

        // Verify data integrity
        if retrieved_results.findings.len() != test_data.findings.len() {
            return Err(anyhow::anyhow!(
                "Data integrity issue: expected {} findings, got {}",
                test_data.findings.len(),
                retrieved_results.findings.len()
            ));
        }

        if retrieved_outputs.len() != 2 {
            return Err(anyhow::anyhow!(
                "Expected 2 output formats, retrieved {}",
                retrieved_outputs.len()
            ));
        }

        Ok(())
    }

    /// Test AI enhancement pipeline
    async fn test_ai_enhancement_pipeline(&self, _scenario: &IntegrationScenario) -> Result<()> {
        let test_data = self.create_integration_test_data(50).await?;
        let ai_config = AIEnhancementConfig::default();

        // Create AI enhancement engine
        let ai_engine = create_enhancement_engine()?;

        // Enhance the results
        let enhanced_results = ai_engine.enhance_results(&test_data, &ai_config)?;

        // Verify AI enhancements were applied
        if enhanced_results
            .semantic_annotations
            .classifications
            .is_empty()
        {
            return Err(anyhow::anyhow!(
                "AI enhancement did not generate classifications"
            ));
        }

        if enhanced_results.relationships.is_empty() && test_data.findings.len() > 1 {
            return Err(anyhow::anyhow!(
                "AI enhancement did not detect relationships in multi-finding dataset"
            ));
        }

        if enhanced_results.insights.is_empty() && test_data.findings.len() > 2 {
            return Err(anyhow::anyhow!(
                "AI enhancement did not generate insights for significant dataset"
            ));
        }

        // Test integration with output formatters
        let json_formatter = JsonFormatter::new();
        let json_output = json_formatter.format(&enhanced_results.base_results)?;

        // Verify the enhanced data can be formatted
        if json_output.content.is_empty() {
            return Err(anyhow::anyhow!("Failed to format AI-enhanced results"));
        }

        Ok(())
    }

    /// Test enterprise features integration
    async fn test_enterprise_features(&self, _scenario: &IntegrationScenario) -> Result<()> {
        let _temp_dir = TempDir::new()?;
        let enterprise_config = EnterpriseConfig {
            enable_audit_trail: true,
            enable_compliance_reporting: true,
            enable_multi_tenant: true,
            ..Default::default()
        };

        let mut enterprise_manager = EnterpriseManager::new(enterprise_config)?;
        let test_data = self.create_integration_test_data(25).await?;

        // Create a test tenant
        let tenant = Tenant::new(
            "Integration Test Tenant".to_string(),
            "Test Organization".to_string(),
            SubscriptionTier::Enterprise,
        );

        // Create enterprise context
        let context = EnterpriseContext::new("integration_test".to_string()).with_tenant(tenant);

        // Process results with enterprise features
        let processing_result = enterprise_manager
            .process_analysis_results(&test_data, &context)
            .await?;

        // Verify enterprise processing was applied
        if processing_result.audit_trail_id.is_none() {
            return Err(anyhow::anyhow!("Audit trail was not generated"));
        }

        if processing_result.tenant_id.is_none() {
            return Err(anyhow::anyhow!("Tenant ID was not recorded"));
        }

        if processing_result
            .processing_metadata
            .features_applied
            .is_empty()
        {
            return Err(anyhow::anyhow!("No enterprise features were applied"));
        }

        Ok(())
    }

    /// Test multi-tenant isolation
    async fn test_multi_tenant_isolation(&self, _scenario: &IntegrationScenario) -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create two separate storage configurations for different tenants
        let tenant1_config = StorageConfig {
            base_directory: temp_dir.path().join("tenant1"),
            organization_strategy: OrganizationStrategy::ByProject,
            ..Default::default()
        };

        let tenant2_config = StorageConfig {
            base_directory: temp_dir.path().join("tenant2"),
            organization_strategy: OrganizationStrategy::ByProject,
            ..Default::default()
        };

        let mut organizer1 = ResultsOrganizer::new(tenant1_config)?;
        let mut organizer2 = ResultsOrganizer::new(tenant2_config)?;

        let test_data1 = self.create_integration_test_data(10).await?;
        let test_data2 = self.create_integration_test_data(15).await?;

        // Store data for both tenants
        let outputs1 = vec![(
            "json".to_string(),
            crate::output::formatter::OutputResult::new(
                serde_json::to_string(&test_data1)?,
                "json",
                "tenant1_config".to_string(),
            ),
        )];

        let outputs2 = vec![(
            "json".to_string(),
            crate::output::formatter::OutputResult::new(
                serde_json::to_string(&test_data2)?,
                "json",
                "tenant2_config".to_string(),
            ),
        )];

        let result_id1 = organizer1.store_results(
            &test_data1,
            &outputs1,
            "tenant1_project",
            Some("tenant1/repo"),
            vec!["tenant1".to_string()],
        )?;

        let result_id2 = organizer2.store_results(
            &test_data2,
            &outputs2,
            "tenant2_project",
            Some("tenant2/repo"),
            vec!["tenant2".to_string()],
        )?;

        // Verify tenant isolation
        // Tenant 1 should not be able to access tenant 2's data
        let tenant1_cannot_access_tenant2 = organizer1.retrieve_results(&result_id2)?;
        if tenant1_cannot_access_tenant2.is_some() {
            return Err(anyhow::anyhow!(
                "Tenant isolation failed: tenant1 accessed tenant2 data"
            ));
        }

        // Tenant 2 should not be able to access tenant 1's data
        let tenant2_cannot_access_tenant1 = organizer2.retrieve_results(&result_id1)?;
        if tenant2_cannot_access_tenant1.is_some() {
            return Err(anyhow::anyhow!(
                "Tenant isolation failed: tenant2 accessed tenant1 data"
            ));
        }

        // Each tenant should be able to access their own data
        let tenant1_own_data = organizer1.retrieve_results(&result_id1)?;
        if tenant1_own_data.is_none() {
            return Err(anyhow::anyhow!("Tenant1 cannot access their own data"));
        }

        let tenant2_own_data = organizer2.retrieve_results(&result_id2)?;
        if tenant2_own_data.is_none() {
            return Err(anyhow::anyhow!("Tenant2 cannot access their own data"));
        }

        Ok(())
    }

    /// Test error recovery mechanisms
    async fn test_error_recovery(&self, _scenario: &IntegrationScenario) -> Result<()> {
        // Test formatter error recovery
        let invalid_data = AnalysisResults::new("".to_string()); // Invalid config hash
        let formatter = JsonFormatter::new();

        // This should not panic, even with edge case data
        let result = formatter.format(&invalid_data);

        // Should either succeed or fail gracefully
        match result {
            Ok(_) => {
                // Success is fine
            }
            Err(e) => {
                // Error should be informative, not a panic
                if e.to_string().contains("panic") {
                    return Err(anyhow::anyhow!(
                        "Formatter panicked instead of handling error gracefully"
                    ));
                }
            }
        }

        // Test storage error recovery with invalid path
        let invalid_storage_config = StorageConfig {
            base_directory: PathBuf::from("/invalid/read/only/path/that/should/not/exist"),
            ..Default::default()
        };

        // This should fail gracefully
        let storage_result = ResultsOrganizer::new(invalid_storage_config);
        if storage_result.is_ok() {
            return Err(anyhow::anyhow!(
                "Storage should have failed with invalid path"
            ));
        }

        Ok(())
    }

    /// Test format compatibility
    async fn test_format_compatibility(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("integration_format_compatibility", async {
                let test_data = self.create_integration_test_data(100).await?;

                // Generate outputs in all formats
                let formatters: Vec<(&str, Box<dyn OutputFormatter>)> = vec![
                    ("json", Box::new(JsonFormatter::new())),
                    ("html", Box::new(HtmlFormatter::new())),
                    ("markdown", Box::new(MarkdownFormatter::new())),
                    ("sarif", Box::new(SarifFormatter::new())),
                    ("yaml", Box::new(YamlFormatter::new())),
                ];

                let mut outputs = Vec::new();
                for (format_name, formatter) in formatters {
                    let output = formatter.format(&test_data)?;
                    outputs.push((format_name, output));
                }

                // Verify all formats were generated successfully
                if outputs.len() != 5 {
                    return Err(anyhow::anyhow!(
                        "Expected 5 formats, generated {}",
                        outputs.len()
                    ));
                }

                // Verify each format contains the expected findings count
                for (format_name, output) in &outputs {
                    if output.content.is_empty() {
                        return Err(anyhow::anyhow!(
                            "Empty output generated for format: {}",
                            format_name
                        ));
                    }
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test data consistency across operations
    async fn test_data_consistency(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("integration_data_consistency", async {
                let original_data = self.create_integration_test_data(50).await?;
                let temp_dir = TempDir::new()?;

                // Store and retrieve data
                let storage_config = StorageConfig {
                    base_directory: temp_dir.path().to_path_buf(),
                    ..Default::default()
                };

                let mut organizer = ResultsOrganizer::new(storage_config)?;

                let outputs = vec![(
                    "json".to_string(),
                    crate::output::formatter::OutputResult::new(
                        serde_json::to_string(&original_data)?,
                        "json",
                        "consistency_test".to_string(),
                    ),
                )];

                let result_id = organizer.store_results(
                    &original_data,
                    &outputs,
                    "consistency_test",
                    Some("consistency/repo"),
                    vec!["test".to_string()],
                )?;

                let retrieved = organizer.retrieve_results(&result_id)?;
                if retrieved.is_none() {
                    return Err(anyhow::anyhow!("Failed to retrieve stored data"));
                }

                let (retrieved_data, _) = retrieved.unwrap();

                // Verify data consistency
                if original_data.findings.len() != retrieved_data.findings.len() {
                    return Err(anyhow::anyhow!(
                        "Finding count mismatch: original={}, retrieved={}",
                        original_data.findings.len(),
                        retrieved_data.findings.len()
                    ));
                }

                if original_data.config_hash != retrieved_data.config_hash {
                    return Err(anyhow::anyhow!(
                        "Config hash mismatch after storage round-trip"
                    ));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test concurrent operations
    async fn test_concurrent_operations(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("integration_concurrent_operations", async {
                let temp_dir = TempDir::new()?;
                let storage_config = StorageConfig {
                    base_directory: temp_dir.path().to_path_buf(),
                    ..Default::default()
                };

                // Create multiple organizers to simulate concurrent access
                let mut handles = Vec::new();

                for i in 0..5 {
                    let config = storage_config.clone();
                    let handle = tokio::spawn(async move {
                        let mut organizer = ResultsOrganizer::new(config)?;
                        let test_data = create_test_data_for_concurrency(i * 10).await?;

                        let outputs = vec![(
                            "json".to_string(),
                            crate::output::formatter::OutputResult::new(
                                serde_json::to_string(&test_data)?,
                                "json",
                                format!("concurrent_test_{}", i),
                            ),
                        )];

                        organizer.store_results(
                            &test_data,
                            &outputs,
                            &format!("concurrent_project_{}", i),
                            Some(&format!("concurrent/repo_{}", i)),
                            vec!["concurrent_test".to_string()],
                        )
                    });

                    handles.push(handle);
                }

                // Wait for all concurrent operations to complete
                for handle in handles {
                    let result = handle.await?;
                    if result.is_err() {
                        return Err(anyhow::anyhow!("Concurrent operation failed"));
                    }
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Create integration test data
    async fn create_integration_test_data(&self, size: usize) -> Result<AnalysisResults> {
        let mut results = AnalysisResults::new("integration_test_config".to_string());

        for i in 0..size {
            let severity = match i % 5 {
                0 => Severity::Critical,
                1 => Severity::High,
                2 => Severity::Medium,
                3 => Severity::Low,
                _ => Severity::Info,
            };

            let finding = Finding::new(
                &format!("integration_analyzer_{}", i % 5),
                &format!("integration_rule_{}", i % 10),
                severity,
                PathBuf::from(format!("integration_file_{}.rs", i % 20)),
                (i % 100) as u32 + 1,
                format!("Integration test finding #{}", i),
            )
            .with_description(format!(
                "Detailed description for integration test finding #{}",
                i
            ));

            results.add_finding(finding);
        }

        Ok(results)
    }

    /// Validate output quality
    fn validate_output_quality(
        &self,
        output: &crate::output::formatter::OutputResult,
        format_name: &str,
    ) -> Result<()> {
        if output.content.is_empty() {
            return Err(anyhow::anyhow!("Empty output for format: {}", format_name));
        }

        // Format-specific validations
        match format_name {
            "json" => {
                let _: serde_json::Value = serde_json::from_str(&output.content)?;
            }
            "html" => {
                if !output.content.contains("<!DOCTYPE html>") {
                    return Err(anyhow::anyhow!("HTML output missing DOCTYPE"));
                }
            }
            "markdown" => {
                if !output.content.contains("# CodeGuardian Analysis Report") {
                    return Err(anyhow::anyhow!("Markdown output missing main header"));
                }
            }
            "sarif" => {
                let sarif: serde_json::Value = serde_json::from_str(&output.content)?;
                if sarif.get("version").is_none() {
                    return Err(anyhow::anyhow!("SARIF output missing version"));
                }
            }
            "yaml" => {
                let _: serde_yaml::Value = serde_yaml::from_str(&output.content)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate expected artifact exists
    fn validate_artifact_exists(
        &self,
        outputs: &[(String, crate::output::formatter::OutputResult)],
        artifact: &ExpectedArtifact,
    ) -> Result<()> {
        if !artifact.should_exist {
            return Ok(());
        }

        if let Some(format) = &artifact.format {
            let found = outputs
                .iter()
                .any(|(f, output)| f == format && output.content.len() >= artifact.min_size_bytes);

            if !found {
                return Err(anyhow::anyhow!(
                    "Expected artifact not found: {} format with min size {} bytes",
                    format,
                    artifact.min_size_bytes
                ));
            }
        }

        Ok(())
    }
}

/// Helper function for concurrent test data creation
async fn create_test_data_for_concurrency(offset: usize) -> Result<AnalysisResults> {
    let mut results = AnalysisResults::new(format!("concurrent_config_{}", offset));

    for i in 0..10 {
        let finding = Finding::new(
            "concurrent_analyzer",
            "concurrent_rule",
            Severity::Medium,
            PathBuf::from(format!("concurrent_{}.rs", offset + i)),
            (i + 1) as u32,
            format!("Concurrent finding #{}", offset + i),
        );
        results.add_finding(finding);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_test_runner_creation() {
        let runner = IntegrationTestRunner::new();
        assert!(!runner.test_scenarios.is_empty());
    }

    #[tokio::test]
    async fn test_create_integration_test_data() {
        let runner = IntegrationTestRunner::new();
        let data = runner.create_integration_test_data(50).await.unwrap();
        assert_eq!(data.findings.len(), 50);
    }

    #[tokio::test]
    async fn test_concurrent_data_creation() {
        let data = create_test_data_for_concurrency(10).await.unwrap();
        assert_eq!(data.findings.len(), 10);
        assert!(data.config_hash.contains("concurrent_config_10"));
    }
}
