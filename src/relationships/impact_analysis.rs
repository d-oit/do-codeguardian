//! Impact analysis for relationship changes

use super::graph::RelationshipGraph;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use chrono::{DateTime, Utc};

/// Impact analyzer for relationship changes
pub struct ImpactAnalyzer {
    config: ImpactAnalysisConfig,
}

/// Configuration for impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisConfig {
    pub max_depth: u32,
    pub include_transitive: bool,
    pub weight_by_strength: bool,
    pub consider_bidirectional: bool,
    pub impact_threshold: f64,
}

impl Default for ImpactAnalysisConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            include_transitive: true,
            weight_by_strength: true,
            consider_bidirectional: true,
            impact_threshold: 0.1,
        }
    }
}

/// Types of changes that can trigger impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    ArtifactModified,
    ArtifactDeleted,
    ArtifactMoved,
    ArtifactRenamed,
    RelationshipAdded,
    RelationshipRemoved,
    RelationshipModified,
}

/// Result of impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisResult {
    pub change_artifact_id: String,
    pub change_type: ChangeType,
    pub impacted_artifacts: Vec<ImpactedArtifact>,
    pub impact_paths: Vec<ImpactPath>,
    pub risk_assessment: RiskAssessment,
    pub recommendations: Vec<String>,
    pub analysis_metadata: AnalysisMetadata,
}

/// Information about an impacted artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactedArtifact {
    pub artifact_id: String,
    pub impact_score: f64,
    pub impact_type: ImpactType,
    pub distance_from_change: u32,
    pub relationship_path: Vec<String>,
    pub estimated_effort: EffortEstimate,
}

/// Types of impact on artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactType {
    DirectDependency,
    IndirectDependency,
    TestImpact,
    DocumentationUpdate,
    ConfigurationChange,
    SecurityImplication,
    PerformanceImpact,
}

/// Path showing how impact propagates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactPath {
    pub source_artifact_id: String,
    pub target_artifact_id: String,
    pub path_artifacts: Vec<String>,
    pub path_relationships: Vec<String>,
    pub cumulative_impact: f64,
    pub critical_path: bool,
}

/// Risk assessment for the change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk: RiskLevel,
    pub technical_risk: RiskLevel,
    pub business_risk: RiskLevel,
    pub security_risk: RiskLevel,
    pub performance_risk: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub mitigation_strategies: Vec<String>,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Individual risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub description: String,
    pub severity: RiskLevel,
    pub likelihood: f64,
    pub impact_score: f64,
}

/// Types of risk factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactorType {
    BreakingChange,
    DataLoss,
    SecurityVulnerability,
    PerformanceDegradation,
    ServiceOutage,
    ComplianceViolation,
    UserExperienceImpact,
}

/// Effort estimation for addressing impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub estimated_hours: f64,
    pub complexity: ComplexityLevel,
    pub required_skills: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Complexity levels for effort estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Trivial,
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Metadata about the analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub analysis_time_ms: u64,
    pub artifacts_analyzed: usize,
    pub relationships_analyzed: usize,
    pub max_depth_reached: u32,
    pub analysis_timestamp: DateTime<Utc>,
}

impl ImpactAnalyzer {
    pub fn new() -> Self {
        Self {
            config: ImpactAnalysisConfig::default(),
        }
    }
}

impl Default for ImpactAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImpactAnalyzer {
    pub fn with_config(config: ImpactAnalysisConfig) -> Self {
        Self { config }
    }

    /// Analyze impact of a change to an artifact
    pub async fn analyze_impact(
        &self,
        artifact_id: &str,
        change_type: ChangeType,
        graph: &RelationshipGraph,
    ) -> Result<ImpactAnalysisResult> {
        let start_time = std::time::Instant::now();

        // Find all artifacts that could be impacted
        let impacted_artifacts = self.find_impacted_artifacts(artifact_id, &change_type, graph).await?;

        // Calculate impact paths
        let impact_paths = self.calculate_impact_paths(artifact_id, &impacted_artifacts, graph).await?;

        // Assess risks
        let risk_assessment = self.assess_risks(&change_type, &impacted_artifacts, &impact_paths).await?;

        // Generate recommendations
        let recommendations = self.generate_recommendations(&change_type, &risk_assessment, &impacted_artifacts);

        let analysis_time = start_time.elapsed().as_millis() as u64;

        Ok(ImpactAnalysisResult {
            change_artifact_id: artifact_id.to_string(),
            change_type,
            impacted_artifacts,
            impact_paths,
            risk_assessment,
            recommendations,
            analysis_metadata: AnalysisMetadata {
                analysis_time_ms: analysis_time,
                artifacts_analyzed: 0, // Would be calculated during analysis
                relationships_analyzed: 0, // Would be calculated during analysis
                max_depth_reached: self.config.max_depth,
                analysis_timestamp: Utc::now(),
            },
        })
    }

    /// Find artifacts that could be impacted by the change
    async fn find_impacted_artifacts(
        &self,
        source_artifact_id: &str,
        change_type: &ChangeType,
        graph: &RelationshipGraph,
    ) -> Result<Vec<ImpactedArtifact>> {
        let mut impacted = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        // Start with the changed artifact
        queue.push_back((source_artifact_id.to_string(), 0u32, 1.0f64, vec![]));

        while let Some((current_id, depth, cumulative_impact, path)) = queue.pop_front() {
            if depth > self.config.max_depth || visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id.clone());

            // Skip the source artifact itself
            if depth > 0 {
                let impact_type = self.determine_impact_type(change_type, depth);
                let impact_score = self.calculate_impact_score(&cumulative_impact, depth);

                if impact_score >= self.config.impact_threshold {
                    impacted.push(ImpactedArtifact {
                        artifact_id: current_id.clone(),
                        impact_score,
                        impact_type: impact_type.clone(),
                        distance_from_change: depth,
                        relationship_path: path.clone(),
                        estimated_effort: self.estimate_effort(impact_score, &impact_type),
                    });
                }
            }

            // Find neighbors if we haven't reached max depth
            if depth < self.config.max_depth {
                if let Ok(neighbors) = graph.get_neighbors(&current_id) {
                    for neighbor_id in neighbors {
                        if !visited.contains(&neighbor_id) {
                            let mut new_path = path.clone();
                            new_path.push(current_id.clone());

                            // Calculate impact propagation (simplified)
                            let propagated_impact = cumulative_impact * 0.8; // Decay factor

                            queue.push_back((neighbor_id, depth + 1, propagated_impact, new_path));
                        }
                    }
                }
            }
        }

        // Sort by impact score descending
        impacted.sort_by(|a, b| b.impact_score.partial_cmp(&a.impact_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(impacted)
    }

    /// Calculate impact paths between artifacts
    async fn calculate_impact_paths(
        &self,
        source_id: &str,
        impacted_artifacts: &[ImpactedArtifact],
        _graph: &RelationshipGraph,
    ) -> Result<Vec<ImpactPath>> {
        let mut paths = Vec::new();

        for impacted in impacted_artifacts {
            // Use the relationship path from the impacted artifact
            let path = ImpactPath {
                source_artifact_id: source_id.to_string(),
                target_artifact_id: impacted.artifact_id.clone(),
                path_artifacts: impacted.relationship_path.clone(),
                path_relationships: vec![], // Would be populated with actual relationship IDs
                cumulative_impact: impacted.impact_score,
                critical_path: impacted.impact_score > 0.7,
            };

            paths.push(path);
        }

        Ok(paths)
    }

    /// Assess risks associated with the change
    async fn assess_risks(
        &self,
        change_type: &ChangeType,
        impacted_artifacts: &[ImpactedArtifact],
        impact_paths: &[ImpactPath],
    ) -> Result<RiskAssessment> {
        let mut risk_factors = Vec::new();

        // Assess based on change type
        match change_type {
            ChangeType::ArtifactDeleted => {
                risk_factors.push(RiskFactor {
                    factor_type: RiskFactorType::BreakingChange,
                    description: "Artifact deletion may break dependent components".to_string(),
                    severity: RiskLevel::High,
                    likelihood: 0.8,
                    impact_score: 0.9,
                });
            },
            ChangeType::ArtifactModified => {
                risk_factors.push(RiskFactor {
                    factor_type: RiskFactorType::BreakingChange,
                    description: "Modifications may introduce breaking changes".to_string(),
                    severity: RiskLevel::Medium,
                    likelihood: 0.4,
                    impact_score: 0.6,
                });
            },
            _ => {
                risk_factors.push(RiskFactor {
                    factor_type: RiskFactorType::UserExperienceImpact,
                    description: "Change may affect user experience".to_string(),
                    severity: RiskLevel::Low,
                    likelihood: 0.3,
                    impact_score: 0.4,
                });
            }
        }

        // Assess based on number of impacted artifacts
        if impacted_artifacts.len() > 10 {
            risk_factors.push(RiskFactor {
                factor_type: RiskFactorType::ServiceOutage,
                description: "Large number of impacted artifacts increases outage risk".to_string(),
                severity: RiskLevel::High,
                likelihood: 0.6,
                impact_score: 0.8,
            });
        }

        // Assess based on critical paths
        let critical_paths = impact_paths.iter().filter(|p| p.critical_path).count();
        if critical_paths > 0 {
            risk_factors.push(RiskFactor {
                factor_type: RiskFactorType::PerformanceDegradation,
                description: "Critical paths may be affected".to_string(),
                severity: RiskLevel::Medium,
                likelihood: 0.5,
                impact_score: 0.7,
            });
        }

        // Calculate overall risk levels
        let overall_risk = self.calculate_overall_risk(&risk_factors);
        let technical_risk = self.calculate_technical_risk(&risk_factors);
        let business_risk = self.calculate_business_risk(&risk_factors);
        let security_risk = self.calculate_security_risk(&risk_factors);
        let performance_risk = self.calculate_performance_risk(&risk_factors);

        let mitigation_strategies = self.generate_mitigation_strategies(&risk_factors);

        Ok(RiskAssessment {
            overall_risk,
            technical_risk,
            business_risk,
            security_risk,
            performance_risk,
            risk_factors,
            mitigation_strategies,
        })
    }

    /// Determine the type of impact based on change type and distance
    fn determine_impact_type(&self, change_type: &ChangeType, depth: u32) -> ImpactType {
        match (change_type, depth) {
            (ChangeType::ArtifactDeleted, 1) => ImpactType::DirectDependency,
            (ChangeType::ArtifactModified, 1) => ImpactType::DirectDependency,
            (_, 1) => ImpactType::DirectDependency,
            (_, 2) => ImpactType::IndirectDependency,
            _ => ImpactType::IndirectDependency,
        }
    }

    /// Calculate impact score based on cumulative impact and distance
    fn calculate_impact_score(&self, cumulative_impact: &f64, depth: u32) -> f64 {
        if self.config.weight_by_strength {
            cumulative_impact / (depth as f64).sqrt()
        } else {
            1.0 / (depth as f64)
        }
    }

    /// Estimate effort required to address impact
    fn estimate_effort(&self, impact_score: f64, impact_type: &ImpactType) -> EffortEstimate {
        let base_hours = match impact_type {
            ImpactType::DirectDependency => 4.0,
            ImpactType::IndirectDependency => 2.0,
            ImpactType::TestImpact => 1.0,
            ImpactType::DocumentationUpdate => 0.5,
            ImpactType::ConfigurationChange => 1.5,
            ImpactType::SecurityImplication => 8.0,
            ImpactType::PerformanceImpact => 6.0,
        };

        let estimated_hours = base_hours * impact_score;

        let complexity = if estimated_hours < 1.0 {
            ComplexityLevel::Trivial
        } else if estimated_hours < 4.0 {
            ComplexityLevel::Simple
        } else if estimated_hours < 8.0 {
            ComplexityLevel::Moderate
        } else if estimated_hours < 16.0 {
            ComplexityLevel::Complex
        } else {
            ComplexityLevel::VeryComplex
        };

        let required_skills = match impact_type {
            ImpactType::SecurityImplication => vec!["Security".to_string(), "Code Review".to_string()],
            ImpactType::PerformanceImpact => vec!["Performance Optimization".to_string()],
            ImpactType::DirectDependency => vec!["Software Development".to_string()],
            _ => vec!["General Development".to_string()],
        };

        EffortEstimate {
            estimated_hours,
            complexity,
            required_skills,
            dependencies: vec![],
        }
    }

    /// Calculate overall risk level
    fn calculate_overall_risk(&self, risk_factors: &[RiskFactor]) -> RiskLevel {
        if risk_factors.iter().any(|rf| rf.severity == RiskLevel::Critical) {
            RiskLevel::Critical
        } else if risk_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            RiskLevel::High
        } else if risk_factors.iter().any(|rf| rf.severity == RiskLevel::Medium) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// Calculate technical risk level
    fn calculate_technical_risk(&self, risk_factors: &[RiskFactor]) -> RiskLevel {
        let technical_factors: Vec<_> = risk_factors.iter()
            .filter(|rf| matches!(rf.factor_type,
                RiskFactorType::BreakingChange |
                RiskFactorType::PerformanceDegradation |
                RiskFactorType::ServiceOutage
            ))
            .collect();

        if technical_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            RiskLevel::High
        } else if technical_factors.iter().any(|rf| rf.severity == RiskLevel::Medium) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// Calculate business risk level
    fn calculate_business_risk(&self, risk_factors: &[RiskFactor]) -> RiskLevel {
        let business_factors: Vec<_> = risk_factors.iter()
            .filter(|rf| matches!(rf.factor_type,
                RiskFactorType::UserExperienceImpact |
                RiskFactorType::ComplianceViolation |
                RiskFactorType::ServiceOutage
            ))
            .collect();

        if business_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            RiskLevel::High
        } else if business_factors.iter().any(|rf| rf.severity == RiskLevel::Medium) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// Calculate security risk level
    fn calculate_security_risk(&self, risk_factors: &[RiskFactor]) -> RiskLevel {
        let security_factors: Vec<_> = risk_factors.iter()
            .filter(|rf| matches!(rf.factor_type, RiskFactorType::SecurityVulnerability))
            .collect();

        if security_factors.iter().any(|rf| rf.severity == RiskLevel::Critical) {
            RiskLevel::Critical
        } else if security_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            RiskLevel::High
        } else {
            RiskLevel::Low
        }
    }

    /// Calculate performance risk level
    fn calculate_performance_risk(&self, risk_factors: &[RiskFactor]) -> RiskLevel {
        let performance_factors: Vec<_> = risk_factors.iter()
            .filter(|rf| matches!(rf.factor_type, RiskFactorType::PerformanceDegradation))
            .collect();

        if performance_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            RiskLevel::High
        } else if performance_factors.iter().any(|rf| rf.severity == RiskLevel::Medium) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// Generate mitigation strategies
    fn generate_mitigation_strategies(&self, risk_factors: &[RiskFactor]) -> Vec<String> {
        let mut strategies = Vec::new();

        for risk_factor in risk_factors {
            match risk_factor.factor_type {
                RiskFactorType::BreakingChange => {
                    strategies.push("Implement comprehensive testing before deployment".to_string());
                    strategies.push("Use feature flags to control rollout".to_string());
                },
                RiskFactorType::SecurityVulnerability => {
                    strategies.push("Conduct security review and penetration testing".to_string());
                    strategies.push("Implement additional security controls".to_string());
                },
                RiskFactorType::PerformanceDegradation => {
                    strategies.push("Perform load testing and performance monitoring".to_string());
                    strategies.push("Implement performance optimization measures".to_string());
                },
                RiskFactorType::ServiceOutage => {
                    strategies.push("Plan for gradual rollout with rollback capability".to_string());
                    strategies.push("Ensure monitoring and alerting are in place".to_string());
                },
                _ => {
                    strategies.push("Monitor impact and be prepared to rollback if needed".to_string());
                },
            }
        }

        // Remove duplicates
        strategies.sort();
        strategies.dedup();

        strategies
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(
        &self,
        change_type: &ChangeType,
        risk_assessment: &RiskAssessment,
        impacted_artifacts: &[ImpactedArtifact],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // General recommendations based on change type
        match change_type {
            ChangeType::ArtifactDeleted => {
                recommendations.push("Verify all dependencies are updated before deletion".to_string());
                recommendations.push("Create backup of artifact before deletion".to_string());
            },
            ChangeType::ArtifactModified => {
                recommendations.push("Run comprehensive test suite after modifications".to_string());
                recommendations.push("Review all dependent components for compatibility".to_string());
            },
            _ => {
                recommendations.push("Monitor system behavior after change".to_string());
            }
        }

        // Recommendations based on risk level
        match risk_assessment.overall_risk {
            RiskLevel::Critical => {
                recommendations.push("Consider postponing change until risks are mitigated".to_string());
                recommendations.push("Require additional approvals before proceeding".to_string());
            },
            RiskLevel::High => {
                recommendations.push("Implement additional safeguards and monitoring".to_string());
                recommendations.push("Plan for immediate rollback capability".to_string());
            },
            RiskLevel::Medium => {
                recommendations.push("Proceed with caution and enhanced monitoring".to_string());
            },
            RiskLevel::Low => {
                recommendations.push("Standard change management procedures apply".to_string());
            }
        }

        // Recommendations based on number of impacted artifacts
        if impacted_artifacts.len() > 20 {
            recommendations.push("Consider breaking change into smaller increments".to_string());
        } else if impacted_artifacts.len() > 5 {
            recommendations.push("Coordinate with teams responsible for impacted artifacts".to_string());
        }

        recommendations
    }
}
