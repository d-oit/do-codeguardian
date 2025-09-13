use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Model versioning and A/B testing system
pub struct ModelVersionManager {
    models: HashMap<String, ModelVersion>,
    active_experiments: HashMap<String, ABTestExperiment>,
    version_history: Vec<VersionHistoryEntry>,
    config: VersioningConfig,
}

/// Configuration for model versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersioningConfig {
    pub enable_ab_testing: bool,
    pub max_versions_per_model: usize,
    pub auto_promote_threshold: f64,
    pub experiment_duration_days: u32,
    pub minimum_samples_for_promotion: usize,
    pub enable_rollback: bool,
    pub backup_old_versions: bool,
}

/// Model version with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: String,
    pub name: String,
    pub version: String,
    pub model_path: PathBuf,
    pub created_at: SystemTime,
    pub performance_metrics: PerformanceMetrics,
    pub training_config: TrainingConfig,
    pub status: ModelStatus,
    pub deployment_info: DeploymentInfo,
}

/// Performance metrics for model evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub false_positive_rate: f64,
    pub false_negative_rate: f64,
    pub inference_time_ms: f64,
    pub model_size_mb: f64,
    pub validation_samples: usize,
}

/// Training configuration used for the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub dataset_version: String,
    pub training_samples: usize,
    pub validation_samples: usize,
    pub epochs: u32,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub optimizer: String,
    pub loss_function: String,
}

/// Model deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelStatus {
    Training,
    Validation,
    Testing,
    Production,
    Deprecated,
    Archived,
}

/// Deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub environment: String,
    pub deployed_at: Option<SystemTime>,
    pub traffic_percentage: f64,
    pub health_status: HealthStatus,
    pub resource_usage: ResourceUsage,
}

/// Health status of deployed model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub gpu_usage_percent: Option<f64>,
    pub requests_per_second: f64,
}

/// A/B testing experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestExperiment {
    pub id: String,
    pub name: String,
    pub model_a: String, // Model version ID
    pub model_b: String, // Model version ID
    pub traffic_split: TrafficSplit,
    pub start_date: SystemTime,
    pub end_date: Option<SystemTime>,
    pub status: ExperimentStatus,
    pub metrics: ExperimentMetrics,
    pub hypothesis: String,
    pub success_criteria: SuccessCriteria,
}

/// Traffic split configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSplit {
    pub model_a_percentage: f64,
    pub model_b_percentage: f64,
    pub control_group_percentage: f64,
}

/// Experiment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentStatus {
    Planning,
    Running,
    Paused,
    Completed,
    Cancelled,
}

/// Experiment metrics comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetrics {
    pub model_a_metrics: PerformanceMetrics,
    pub model_b_metrics: PerformanceMetrics,
    pub statistical_significance: f64,
    pub confidence_interval: (f64, f64),
    pub sample_size_a: usize,
    pub sample_size_b: usize,
}

/// Success criteria for experiments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriteria {
    pub primary_metric: String,
    pub improvement_threshold: f64,
    pub minimum_confidence: f64,
    pub maximum_duration_days: u32,
}

/// Version history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistoryEntry {
    pub timestamp: SystemTime,
    pub action: VersionAction,
    pub model_id: String,
    pub version: String,
    pub details: String,
    pub user: Option<String>,
}

/// Version management actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionAction {
    Created,
    Deployed,
    Promoted,
    Deprecated,
    RolledBack,
    Archived,
    ExperimentStarted,
    ExperimentCompleted,
}

impl Default for VersioningConfig {
    fn default() -> Self {
        Self {
            enable_ab_testing: true,
            max_versions_per_model: 10,
            auto_promote_threshold: 0.95,
            experiment_duration_days: 7,
            minimum_samples_for_promotion: 1000,
            enable_rollback: true,
            backup_old_versions: true,
        }
    }
}

impl ModelVersionManager {
    /// Create a new model version manager
    pub fn new(config: VersioningConfig) -> Self {
        Self {
            models: HashMap::new(),
            active_experiments: HashMap::new(),
            version_history: Vec::new(),
            config,
        }
    }

    /// Register a new model version
    pub fn register_model(&mut self, model: ModelVersion) -> Result<()> {
        // Check version limits
        let existing_versions: Vec<_> = self.models
            .values()
            .filter(|m| m.name == model.name)
            .collect();

        if existing_versions.len() >= self.config.max_versions_per_model {
            if self.config.backup_old_versions {
                self.archive_oldest_version(&model.name)?;
            } else {
                return Err(anyhow!("Maximum versions reached for model '{}'", model.name));
            }
        }

        // Add version history entry
        self.add_history_entry(VersionAction::Created, &model.id, &model.version,
                              &format!("Model {} version {} created", model.name, model.version));

        self.models.insert(model.id.clone(), model);
        Ok(())
    }

    /// Deploy a model version to production
    pub fn deploy_model(&mut self, model_id: &str, environment: &str, traffic_percentage: f64) -> Result<()> {
        let model = self.models.get_mut(model_id)
            .ok_or_else(|| anyhow!("Model not found: {}", model_id))?;

        model.status = ModelStatus::Production;
        model.deployment_info.environment = environment.to_string();
        model.deployment_info.deployed_at = Some(SystemTime::now());
        model.deployment_info.traffic_percentage = traffic_percentage;
        model.deployment_info.health_status = HealthStatus::Healthy;

        self.add_history_entry(VersionAction::Deployed, model_id, &model.version,
                              &format!("Deployed to {} with {}% traffic", environment, traffic_percentage));

        Ok(())
    }

    /// Start A/B test experiment
    pub fn start_ab_test(&mut self, experiment: ABTestExperiment) -> Result<()> {
        if !self.config.enable_ab_testing {
            return Err(anyhow!("A/B testing is disabled"));
        }

        // Validate models exist
        if !self.models.contains_key(&experiment.model_a) {
            return Err(anyhow!("Model A not found: {}", experiment.model_a));
        }
        if !self.models.contains_key(&experiment.model_b) {
            return Err(anyhow!("Model B not found: {}", experiment.model_b));
        }

        // Validate traffic split
        let total_traffic = experiment.traffic_split.model_a_percentage +
                           experiment.traffic_split.model_b_percentage +
                           experiment.traffic_split.control_group_percentage;

        if (total_traffic - 100.0).abs() > 0.01 {
            return Err(anyhow!("Traffic split must total 100%"));
        }

        self.add_history_entry(VersionAction::ExperimentStarted, &experiment.model_a, "",
                              &format!("A/B test '{}' started", experiment.name));

        self.active_experiments.insert(experiment.id.clone(), experiment);
        Ok(())
    }

    /// Update experiment metrics
    pub fn update_experiment_metrics(&mut self, experiment_id: &str, metrics: ExperimentMetrics) -> Result<()> {
        let experiment = self.active_experiments.get_mut(experiment_id)
            .ok_or_else(|| anyhow!("Experiment not found: {}", experiment_id))?;

        experiment.metrics = metrics;

        // Check for auto-promotion
        if self.should_auto_promote(experiment) {
            self.promote_model_from_experiment(experiment_id)?;
        }

        Ok(())
    }

    /// Complete an A/B test experiment
    pub fn complete_experiment(&mut self, experiment_id: &str, winner: Option<String>) -> Result<ExperimentResult> {
        let mut experiment = self.active_experiments.remove(experiment_id)
            .ok_or_else(|| anyhow!("Experiment not found: {}", experiment_id))?;

        experiment.status = ExperimentStatus::Completed;
        experiment.end_date = Some(SystemTime::now());

        let result = self.analyze_experiment_results(&experiment, winner)?;

        self.add_history_entry(VersionAction::ExperimentCompleted, &experiment.model_a, "",
                              &format!("A/B test '{}' completed. Winner: {:?}", experiment.name, result.winner));

        Ok(result)
    }

    /// Promote a model version to production
    pub fn promote_model(&mut self, model_id: &str) -> Result<()> {
        let model = self.models.get_mut(model_id)
            .ok_or_else(|| anyhow!("Model not found: {}", model_id))?;

        // Validate model performance
        if model.performance_metrics.accuracy < self.config.auto_promote_threshold {
            return Err(anyhow!("Model accuracy {} below promotion threshold {}",
                              model.performance_metrics.accuracy, self.config.auto_promote_threshold));
        }

        model.status = ModelStatus::Production;
        model.deployment_info.traffic_percentage = 100.0;

        self.add_history_entry(VersionAction::Promoted, model_id, &model.version,
                              "Model promoted to production");

        Ok(())
    }

    /// Rollback to previous model version
    pub fn rollback_model(&mut self, model_name: &str, target_version: Option<String>) -> Result<String> {
        if !self.config.enable_rollback {
            return Err(anyhow!("Rollback is disabled"));
        }

        let target_model_id = if let Some(version) = target_version {
            // Find specific version
            self.models.iter()
                .find(|(_, model)| model.name == model_name && model.version == version)
                .map(|(id, _)| id.clone())
                .ok_or_else(|| anyhow!("Version {} not found for model {}", version, model_name))?
        } else {
            // Find latest stable version
            self.find_latest_stable_version(model_name)?
        };

        // Set current production model to deprecated
        for model in self.models.values_mut() {
            if model.name == model_name && model.status == ModelStatus::Production {
                model.status = ModelStatus::Deprecated;
                model.deployment_info.traffic_percentage = 0.0;
            }
        }

        // Promote target model
        self.promote_model(&target_model_id)?;

        self.add_history_entry(VersionAction::RolledBack, &target_model_id, "",
                              &format!("Rolled back model {} to version", model_name));

        Ok(target_model_id)
    }

    /// Get model performance comparison
    pub fn compare_models(&self, model_ids: &[String]) -> Result<ModelComparison> {
        let mut models = Vec::new();

        for model_id in model_ids {
            let model = self.models.get(model_id)
                .ok_or_else(|| anyhow!("Model not found: {}", model_id))?;
            models.push(model.clone());
        }

        Ok(ModelComparison {
            models,
            comparison_metrics: self.calculate_comparison_metrics(&models),
            recommendation: self.generate_model_recommendation(&models),
        })
    }

    /// Get active experiments
    pub fn get_active_experiments(&self) -> Vec<&ABTestExperiment> {
        self.active_experiments.values()
            .filter(|exp| exp.status == ExperimentStatus::Running)
            .collect()
    }

    /// Get model version history
    pub fn get_version_history(&self, model_name: Option<&str>) -> Vec<&VersionHistoryEntry> {
        self.version_history.iter()
            .filter(|entry| {
                if let Some(name) = model_name {
                    if let Some(model) = self.models.get(&entry.model_id) {
                        model.name == name
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .collect()
    }

    /// Export versioning data
    pub fn export_versioning_data(&self, output_path: &Path) -> Result<()> {
        let export_data = VersioningExportData {
            models: self.models.clone(),
            active_experiments: self.active_experiments.clone(),
            version_history: self.version_history.clone(),
            config: self.config.clone(),
            export_timestamp: SystemTime::now(),
        };

        let json_data = serde_json::to_string_pretty(&export_data)?;
        std::fs::write(output_path, json_data)?;

        Ok(())
    }

    // Private helper methods

    fn archive_oldest_version(&mut self, model_name: &str) -> Result<()> {
        let oldest_model_id = self.models.iter()
            .filter(|(_, model)| model.name == model_name)
            .min_by_key(|(_, model)| model.created_at)
            .map(|(id, _)| id.clone())
            .ok_or_else(|| anyhow!("No models found for archiving"))?;

        if let Some(mut model) = self.models.remove(&oldest_model_id) {
            model.status = ModelStatus::Archived;
            self.add_history_entry(VersionAction::Archived, &oldest_model_id, &model.version,
                                  "Model archived due to version limit");
        }

        Ok(())
    }

    fn should_auto_promote(&self, experiment: &ABTestExperiment) -> bool {
        let metrics_a = &experiment.metrics.model_a_metrics;
        let metrics_b = &experiment.metrics.model_b_metrics;

        // Check if model B significantly outperforms model A
        metrics_b.accuracy > metrics_a.accuracy + 0.05 && // 5% improvement
        experiment.metrics.statistical_significance > 0.95 && // 95% confidence
        experiment.metrics.sample_size_b >= self.config.minimum_samples_for_promotion
    }

    fn promote_model_from_experiment(&mut self, experiment_id: &str) -> Result<()> {
        let experiment = self.active_experiments.get(experiment_id)
            .ok_or_else(|| anyhow!("Experiment not found: {}", experiment_id))?;

        let winner_id = if experiment.metrics.model_b_metrics.accuracy >
                          experiment.metrics.model_a_metrics.accuracy {
            &experiment.model_b
        } else {
            &experiment.model_a
        };

        self.promote_model(winner_id)?;
        Ok(())
    }

    fn analyze_experiment_results(&self, experiment: &ABTestExperiment, winner: Option<String>) -> Result<ExperimentResult> {
        let statistical_winner = if experiment.metrics.model_b_metrics.accuracy >
                                    experiment.metrics.model_a_metrics.accuracy {
            Some(experiment.model_b.clone())
        } else if experiment.metrics.model_a_metrics.accuracy >
                  experiment.metrics.model_b_metrics.accuracy {
            Some(experiment.model_a.clone())
        } else {
            None
        };

        let final_winner = winner.or(statistical_winner);

        Ok(ExperimentResult {
            experiment_id: experiment.id.clone(),
            winner: final_winner,
            confidence: experiment.metrics.statistical_significance,
            improvement: self.calculate_improvement(&experiment.metrics),
            recommendation: self.generate_experiment_recommendation(experiment),
        })
    }

    fn find_latest_stable_version(&self, model_name: &str) -> Result<String> {
        self.models.iter()
            .filter(|(_, model)| {
                model.name == model_name &&
                model.status == ModelStatus::Production &&
                model.performance_metrics.accuracy >= self.config.auto_promote_threshold
            })
            .max_by_key(|(_, model)| model.created_at)
            .map(|(id, _)| id.clone())
            .ok_or_else(|| anyhow!("No stable version found for model {}", model_name))
    }

    fn calculate_comparison_metrics(&self, models: &[ModelVersion]) -> ComparisonMetrics {
        let accuracies: Vec<f64> = models.iter().map(|m| m.performance_metrics.accuracy).collect();
        let inference_times: Vec<f64> = models.iter().map(|m| m.performance_metrics.inference_time_ms).collect();

        ComparisonMetrics {
            best_accuracy: accuracies.iter().fold(0.0, |a, &b| a.max(b)),
            worst_accuracy: accuracies.iter().fold(1.0, |a, &b| a.min(b)),
            average_accuracy: accuracies.iter().sum::<f64>() / accuracies.len() as f64,
            fastest_inference: inference_times.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            slowest_inference: inference_times.iter().fold(0.0, |a, &b| a.max(b)),
            average_inference: inference_times.iter().sum::<f64>() / inference_times.len() as f64,
        }
    }

    fn generate_model_recommendation(&self, models: &[ModelVersion]) -> String {
        if models.is_empty() {
            return "No models to compare".to_string();
        }

        let best_model = models.iter()
            .max_by(|a, b| a.performance_metrics.accuracy.partial_cmp(&b.performance_metrics.accuracy).unwrap())
            .unwrap();

        format!("Recommended model: {} (v{}) with {:.2}% accuracy",
                best_model.name, best_model.version, best_model.performance_metrics.accuracy * 100.0)
    }

    fn calculate_improvement(&self, metrics: &ExperimentMetrics) -> f64 {
        let baseline = metrics.model_a_metrics.accuracy;
        let treatment = metrics.model_b_metrics.accuracy;
        ((treatment - baseline) / baseline) * 100.0
    }

    fn generate_experiment_recommendation(&self, experiment: &ABTestExperiment) -> String {
        let improvement = self.calculate_improvement(&experiment.metrics);

        if improvement > 5.0 && experiment.metrics.statistical_significance > 0.95 {
            "Strong recommendation to deploy model B".to_string()
        } else if improvement > 2.0 && experiment.metrics.statistical_significance > 0.90 {
            "Moderate recommendation to deploy model B".to_string()
        } else if improvement < -2.0 {
            "Recommendation to keep model A".to_string()
        } else {
            "Inconclusive results, consider extending experiment".to_string()
        }
    }

    fn add_history_entry(&mut self, action: VersionAction, model_id: &str, version: &str, details: &str) {
        self.version_history.push(VersionHistoryEntry {
            timestamp: SystemTime::now(),
            action,
            model_id: model_id.to_string(),
            version: version.to_string(),
            details: details.to_string(),
            user: None, // Could be populated from context
        });
    }
}

/// Model comparison result
#[derive(Debug, Clone)]
pub struct ModelComparison {
    pub models: Vec<ModelVersion>,
    pub comparison_metrics: ComparisonMetrics,
    pub recommendation: String,
}

/// Comparison metrics across models
#[derive(Debug, Clone)]
pub struct ComparisonMetrics {
    pub best_accuracy: f64,
    pub worst_accuracy: f64,
    pub average_accuracy: f64,
    pub fastest_inference: f64,
    pub slowest_inference: f64,
    pub average_inference: f64,
}

/// Experiment result
#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub winner: Option<String>,
    pub confidence: f64,
    pub improvement: f64,
    pub recommendation: String,
}

/// Export data structure
#[derive(Debug, Serialize, Deserialize)]
struct VersioningExportData {
    models: HashMap<String, ModelVersion>,
    active_experiments: HashMap<String, ABTestExperiment>,
    version_history: Vec<VersionHistoryEntry>,
    config: VersioningConfig,
    export_timestamp: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registration() {
        let config = VersioningConfig::default();
        let mut manager = ModelVersionManager::new(config);

        let model = create_test_model("test_model", "1.0.0");
        let result = manager.register_model(model);

        assert!(result.is_ok());
        assert_eq!(manager.models.len(), 1);
    }

    #[test]
    fn test_ab_test_creation() {
        let config = VersioningConfig::default();
        let mut manager = ModelVersionManager::new(config);

        // Register test models
        let model_a = create_test_model("test_model", "1.0.0");
        let model_b = create_test_model("test_model", "2.0.0");

        manager.register_model(model_a.clone()).unwrap();
        manager.register_model(model_b.clone()).unwrap();

        let experiment = create_test_experiment(&model_a.id, &model_b.id);
        let result = manager.start_ab_test(experiment);

        assert!(result.is_ok());
        assert_eq!(manager.active_experiments.len(), 1);
    }

    fn create_test_model(name: &str, version: &str) -> ModelVersion {
        ModelVersion {
            id: format!("{}_{}", name, version),
            name: name.to_string(),
            version: version.to_string(),
            model_path: PathBuf::from(format!("models/{}/{}.bin", name, version)),
            created_at: SystemTime::now(),
            performance_metrics: PerformanceMetrics {
                accuracy: 0.85,
                precision: 0.83,
                recall: 0.87,
                f1_score: 0.85,
                false_positive_rate: 0.15,
                false_negative_rate: 0.13,
                inference_time_ms: 50.0,
                model_size_mb: 100.0,
                validation_samples: 1000,
            },
            training_config: TrainingConfig {
                dataset_version: "v1.0".to_string(),
                training_samples: 10000,
                validation_samples: 2000,
                epochs: 10,
                learning_rate: 0.001,
                batch_size: 32,
                optimizer: "Adam".to_string(),
                loss_function: "CrossEntropy".to_string(),
            },
            status: ModelStatus::Training,
            deployment_info: DeploymentInfo {
                environment: "staging".to_string(),
                deployed_at: None,
                traffic_percentage: 0.0,
                health_status: HealthStatus::Unknown,
                resource_usage: ResourceUsage {
                    cpu_usage_percent: 0.0,
                    memory_usage_mb: 0.0,
                    gpu_usage_percent: None,
                    requests_per_second: 0.0,
                },
            },
        }
    }

    fn create_test_experiment(model_a_id: &str, model_b_id: &str) -> ABTestExperiment {
        ABTestExperiment {
            id: "test_experiment".to_string(),
            name: "Model A vs Model B".to_string(),
            model_a: model_a_id.to_string(),
            model_b: model_b_id.to_string(),
            traffic_split: TrafficSplit {
                model_a_percentage: 45.0,
                model_b_percentage: 45.0,
                control_group_percentage: 10.0,
            },
            start_date: SystemTime::now(),
            end_date: None,
            status: ExperimentStatus::Planning,
            metrics: ExperimentMetrics {
                model_a_metrics: PerformanceMetrics {
                    accuracy: 0.85,
                    precision: 0.83,
                    recall: 0.87,
                    f1_score: 0.85,
                    false_positive_rate: 0.15,
                    false_negative_rate: 0.13,
                    inference_time_ms: 50.0,
                    model_size_mb: 100.0,
                    validation_samples: 1000,
                },
                model_b_metrics: PerformanceMetrics {
                    accuracy: 0.88,
                    precision: 0.86,
                    recall: 0.90,
                    f1_score: 0.88,
                    false_positive_rate: 0.12,
                    false_negative_rate: 0.10,
                    inference_time_ms: 45.0,
                    model_size_mb: 95.0,
                    validation_samples: 1000,
                },
                statistical_significance: 0.95,
                confidence_interval: (0.02, 0.05),
                sample_size_a: 1000,
                sample_size_b: 1000,
            },
            hypothesis: "Model B will outperform Model A".to_string(),
            success_criteria: SuccessCriteria {
                primary_metric: "accuracy".to_string(),
                improvement_threshold: 0.02,
                minimum_confidence: 0.95,
                maximum_duration_days: 14,
            },
        }
    }
}
