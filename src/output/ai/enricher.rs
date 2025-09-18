//! # AI Enhancement Engine
//!
//! This module provides the core AI enhancement capabilities for CodeGuardian analysis results.

use super::*;
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

/// Basic AI enhancement engine implementation
pub struct BasicEnhancementEngine {
    /// Classification patterns for semantic enrichment
    classification_patterns: HashMap<String, Vec<String>>,
    /// Relationship detection rules
    relationship_rules: Vec<RelationshipRule>,
}

/// Rule for detecting relationships between findings
#[derive(Debug, Clone)]
struct RelationshipRule {
    /// Rule name
    name: String,
    /// Relationship type this rule detects
    relationship_type: RelationshipType,
    /// Minimum confidence for the relationship
    min_confidence: f64,
    /// Pattern matching function
    matcher: fn(&Finding, &Finding) -> Option<f64>,
}

impl BasicEnhancementEngine {
    /// Create a new basic enhancement engine
    pub fn new() -> Result<Self> {
        let mut engine = Self {
            classification_patterns: HashMap::new(),
            relationship_rules: Vec::new(),
        };

        engine.initialize_patterns();
        engine.initialize_relationship_rules();

        Ok(engine)
    }

    /// Initialize classification patterns
    fn initialize_patterns(&mut self) {
        // Security patterns
        let security_patterns = vec![
            "sql injection".to_string(),
            "xss".to_string(),
            "cross-site scripting".to_string(),
            "csrf".to_string(),
            "authentication".to_string(),
            "authorization".to_string(),
            "password".to_string(),
            "encryption".to_string(),
            "vulnerability".to_string(),
            "security".to_string(),
            "insecure".to_string(),
            "unsafe".to_string(),
            "hardcoded".to_string(),
            "secret".to_string(),
            "token".to_string(),
            "key".to_string(),
            "credential".to_string(),
            "exploit".to_string(),
            "attack".to_string(),
            "malware".to_string(),
            "virus".to_string(),
            "trojan".to_string(),
            "ransomware".to_string(),
        ];
        self.classification_patterns
            .insert("Security".to_string(), security_patterns);

        // Performance patterns
        let performance_patterns = vec![
            "performance".to_string(),
            "slow".to_string(),
            "memory leak".to_string(),
            "inefficient".to_string(),
            "optimization".to_string(),
            "bottleneck".to_string(),
            "timeout".to_string(),
            "latency".to_string(),
            "cpu".to_string(),
            "memory".to_string(),
            "disk".to_string(),
            "io".to_string(),
            "throughput".to_string(),
            "response time".to_string(),
            "scalability".to_string(),
        ];
        self.classification_patterns
            .insert("Performance".to_string(), performance_patterns);

        // Code quality patterns
        let quality_patterns = vec![
            "code smell".to_string(),
            "duplicate".to_string(),
            "complexity".to_string(),
            "maintainability".to_string(),
            "refactor".to_string(),
            "technical debt".to_string(),
            "readability".to_string(),
        ];
        self.classification_patterns
            .insert("Quality".to_string(), quality_patterns);

        // Reliability patterns
        let reliability_patterns = vec![
            "null pointer".to_string(),
            "exception".to_string(),
            "error handling".to_string(),
            "crash".to_string(),
            "reliability".to_string(),
            "stability".to_string(),
        ];
        self.classification_patterns
            .insert("Reliability".to_string(), reliability_patterns);
    }

    /// Initialize relationship detection rules
    fn initialize_relationship_rules(&mut self) {
        // Same file relationship
        self.relationship_rules.push(RelationshipRule {
            name: "Same File".to_string(),
            relationship_type: RelationshipType::SameComponent,
            min_confidence: 0.8,
            matcher: |a, b| {
                if a.file == b.file && a.id != b.id {
                    Some(0.8)
                } else {
                    None
                }
            },
        });

        // Similar message relationship
        self.relationship_rules.push(RelationshipRule {
            name: "Similar Message".to_string(),
            relationship_type: RelationshipType::SimilarPattern,
            min_confidence: 0.7,
            matcher: |a, b| {
                if a.id != b.id {
                    let similarity = calculate_text_similarity(&a.message, &b.message);
                    if similarity > 0.7 {
                        Some(similarity)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        });

        // Same analyzer and rule relationship
        self.relationship_rules.push(RelationshipRule {
            name: "Same Rule".to_string(),
            relationship_type: RelationshipType::SimilarPattern,
            min_confidence: 0.9,
            matcher: |a, b| {
                if a.id != b.id && a.analyzer == b.analyzer && a.rule == b.rule {
                    Some(0.9)
                } else {
                    None
                }
            },
        });

        // Same analyzer relationship
        self.relationship_rules.push(RelationshipRule {
            name: "Same Analyzer".to_string(),
            relationship_type: RelationshipType::SameComponent,
            min_confidence: 0.6,
            matcher: |a, b| {
                if a.id != b.id && a.analyzer == b.analyzer {
                    Some(0.6)
                } else {
                    None
                }
            },
        });
    }

    /// Classify a finding using pattern matching
    fn classify_finding(&self, finding: &Finding) -> FindingClassification {
        let text = format!(
            "{} {} {}",
            finding.message,
            finding.description.as_deref().unwrap_or(""),
            finding.rule
        )
        .to_lowercase();

        let mut scores = HashMap::new();

        // Calculate scores for each category
        for (category, patterns) in &self.classification_patterns {
            let mut score = 0.0;
            let mut matches = 0;

            for pattern in patterns {
                if text.contains(pattern) {
                    score += 1.0;
                    matches += 1;
                }
            }

            if matches > 0 {
                // Normalize score - give higher weight to multiple matches
                score = (matches as f64 / patterns.len() as f64).min(1.0);
                scores.insert(category.clone(), score);
            }
        }

        // Find primary category (highest score)
        let primary_category = if scores.is_empty() {
            "General".to_string()
        } else {
            scores
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(k, _)| k.clone())
                .unwrap()
        };

        let confidence = if scores.is_empty() {
            0.5 // Default confidence for general category
        } else {
            scores.get(&primary_category).copied().unwrap_or(0.5)
        };

        // Secondary categories (scores > 0.3)
        let secondary_categories: Vec<String> = scores
            .iter()
            .filter(|(k, v)| **v > 0.3 && *k != &primary_category)
            .map(|(k, _)| k.clone())
            .collect();

        // Generate reasoning
        let reasoning = format!(
            "Classified as {} based on pattern matching with confidence {:.2}",
            primary_category, confidence
        );

        // Generate suggested tags
        let mut suggested_tags = vec![primary_category.to_lowercase()];
        if confidence > 0.8 {
            suggested_tags.push("high-confidence".to_string());
        }
        suggested_tags.push(finding.severity.to_string().to_lowercase());

        FindingClassification {
            primary_category,
            secondary_categories,
            confidence,
            reasoning,
            suggested_tags,
        }
    }

    /// Assess business impact of a finding
    fn assess_impact(
        &self,
        finding: &Finding,
        classification: &FindingClassification,
    ) -> ImpactAssessment {
        let criticality = match (&finding.severity, classification.primary_category.as_str()) {
            (Severity::Critical, "Security") => "Critical",
            (Severity::High, "Security") => "High",
            (Severity::Critical, _) => "High",
            (Severity::High, _) => "Medium",
            (Severity::Medium, "Security") => "Medium",
            (Severity::Medium, _) => "Low",
            _ => "Low",
        };

        let affected_areas = match classification.primary_category.as_str() {
            "Security" => vec!["Security".to_string(), "Compliance".to_string()],
            "Performance" => vec!["User Experience".to_string(), "Operations".to_string()],
            "Quality" => vec![
                "Maintenance".to_string(),
                "Development Velocity".to_string(),
            ],
            "Reliability" => vec!["Operations".to_string(), "User Experience".to_string()],
            _ => vec!["General".to_string()],
        };

        let estimated_effort = match finding.severity {
            Severity::Critical => Some(8.0),
            Severity::High => Some(4.0),
            Severity::Medium => Some(2.0),
            Severity::Low => Some(1.0),
            Severity::Info => Some(0.5),
        };

        let operational_risk =
            match (&finding.severity, classification.primary_category.as_str()) {
                (Severity::Critical, "Security") => "High - Potential security breach",
                (Severity::Critical, _) => "Medium - System stability risk",
                (Severity::High, "Security") => "Medium - Security vulnerability",
                (Severity::High, _) => "Low - Minor operational impact",
                _ => "Minimal",
            }
            .to_string();

        let compliance_impact = if classification.primary_category == "Security" {
            vec![
                "Data Protection".to_string(),
                "Security Standards".to_string(),
            ]
        } else {
            vec![]
        };

        ImpactAssessment {
            criticality: criticality.to_string(),
            affected_areas,
            estimated_effort,
            operational_risk,
            compliance_impact,
        }
    }

    /// Detect relationships between findings
    fn detect_relationships(&self, findings: &[Finding]) -> Vec<FindingRelationship> {
        let mut relationships = Vec::new();

        tracing::debug!(
            "Checking {} finding pairs for relationships",
            findings.len() * (findings.len() - 1) / 2
        );

        for (i, finding_a) in findings.iter().enumerate() {
            for finding_b in findings.iter().skip(i + 1) {
                for rule in &self.relationship_rules {
                    if let Some(confidence) = (rule.matcher)(finding_a, finding_b) {
                        if confidence >= rule.min_confidence {
                            tracing::debug!(
                                "Found relationship between {} and {}: {} ({:.2})",
                                finding_a.id,
                                finding_b.id,
                                rule.name,
                                confidence
                            );
                            relationships.push(FindingRelationship {
                                source_id: finding_a.id.clone(),
                                target_id: finding_b.id.clone(),
                                relationship_type: rule.relationship_type.clone(),
                                strength: confidence,
                                description: format!(
                                    "Detected {} relationship with {:.1}% confidence",
                                    rule.name,
                                    confidence * 100.0
                                ),
                                evidence: vec![rule.name.clone()],
                            });
                        }
                    }
                }
            }
        }

        tracing::debug!("Total relationships detected: {}", relationships.len());
        relationships
    }

    /// Generate insights from findings and relationships
    fn generate_insights(
        &self,
        findings: &[Finding],
        _relationships: &[FindingRelationship],
        classifications: &HashMap<String, FindingClassification>,
    ) -> Vec<Insight> {
        let mut insights = Vec::new();
        let mut insight_id_counter = 0;

        // Security pattern insights
        let security_findings: Vec<_> = findings
            .iter()
            .filter(|f| {
                classifications
                    .get(&f.id)
                    .map(|c| c.primary_category == "Security")
                    .unwrap_or(false)
            })
            .collect();

        if security_findings.len() > 3 {
            insight_id_counter += 1;
            insights.push(Insight {
                id: format!("insight_{}", insight_id_counter),
                insight_type: InsightType::SecurityPattern,
                title: "Multiple Security Issues Detected".to_string(),
                description: format!(
                    "Found {} security-related findings that may indicate systemic security concerns",
                    security_findings.len()
                ),
                confidence: 0.8,
                priority: "High".to_string(),
                affected_findings: security_findings.iter().map(|f| f.id.clone()).collect(),
                recommendations: vec![
                    Recommendation {
                        id: "rec_security_review".to_string(),
                        action: "Conduct comprehensive security review".to_string(),
                        priority: 1,
                        estimated_effort: Some(16.0),
                        expected_benefit: "Reduce security risk exposure".to_string(),
                        implementation_details: Some("Review all security findings and implement fixes systematically".to_string()),
                        required_resources: vec!["Security specialist".to_string(), "Development time".to_string()],
                    }
                ],
                supporting_data: HashMap::new(),
            });
        }

        // File hotspot insights
        let mut file_finding_counts = HashMap::new();
        for finding in findings {
            *file_finding_counts.entry(finding.file.clone()).or_insert(0) += 1;
        }

        let hotspot_files: Vec<_> = file_finding_counts
            .iter()
            .filter(|(_, count)| **count > 5)
            .collect();

        if !hotspot_files.is_empty() {
            insight_id_counter += 1;
            insights.push(Insight {
                id: format!("insight_{}", insight_id_counter),
                insight_type: InsightType::QualityIssue,
                title: "Code Quality Hotspots Identified".to_string(),
                description: format!(
                    "Found {} files with high concentration of issues",
                    hotspot_files.len()
                ),
                confidence: 0.9,
                priority: "Medium".to_string(),
                affected_findings: findings
                    .iter()
                    .filter(|f| hotspot_files.iter().any(|(path, _)| *path == &f.file))
                    .map(|f| f.id.clone())
                    .collect(),
                recommendations: vec![Recommendation {
                    id: "rec_refactor_hotspots".to_string(),
                    action: "Prioritize refactoring of hotspot files".to_string(),
                    priority: 2,
                    estimated_effort: Some(24.0),
                    expected_benefit: "Improve code quality and reduce future issues".to_string(),
                    implementation_details: None,
                    required_resources: vec!["Development time".to_string()],
                }],
                supporting_data: HashMap::new(),
            });
        }

        insights
    }
}

impl AIEnhancementEngine for BasicEnhancementEngine {
    fn enhance_results(
        &self,
        results: &AnalysisResults,
        config: &AIEnhancementConfig,
    ) -> Result<EnhancedAnalysisResults> {
        let start_time = Instant::now();
        let models_used = vec!["BasicPatternMatcher".to_string()];

        // Initialize enhancement components
        let mut semantic_annotations = SemanticAnnotations {
            classifications: HashMap::new(),
            impact_assessments: HashMap::new(),
            tech_debt_indicators: HashMap::new(),
            security_risks: HashMap::new(),
        };

        // Semantic enrichment
        if config.enable_semantic_enrichment {
            tracing::debug!(
                "Performing semantic enrichment for {} findings",
                results.findings.len()
            );
            for finding in &results.findings {
                let classification = self.classify_finding(finding);
                let impact = self.assess_impact(finding, &classification);

                tracing::debug!(
                    "Finding {} classified as {} with confidence {:.2}",
                    finding.id,
                    classification.primary_category,
                    classification.confidence
                );

                semantic_annotations
                    .classifications
                    .insert(finding.id.clone(), classification);
                semantic_annotations
                    .impact_assessments
                    .insert(finding.id.clone(), impact);
            }
            tracing::debug!(
                "Semantic enrichment complete: {} classifications",
                semantic_annotations.classifications.len()
            );
        }

        // Relationship detection
        let relationships = if config.enable_relationship_detection {
            tracing::debug!(
                "Performing relationship detection for {} findings",
                results.findings.len()
            );
            let rels = self.detect_relationships(&results.findings);
            tracing::debug!(
                "Relationship detection complete: {} relationships found",
                rels.len()
            );
            rels
        } else {
            tracing::debug!("Relationship detection disabled");
            Vec::new()
        };

        // Insight generation
        let insights = if config.enable_insight_generation {
            self.generate_insights(
                &results.findings,
                &relationships,
                &semantic_annotations.classifications,
            )
        } else {
            Vec::new()
        };

        // Basic context (simplified for now)
        let context = ContextData {
            project_context: ProjectContext {
                project_type: None,
                languages: vec![], // Could be inferred from file extensions
                frameworks: vec![],
                maturity_level: None,
                team_size: None,
                development_methodology: None,
            },
            code_context: HashMap::new(),
            historical_context: HistoricalContext {
                previous_results: vec![],
                trends: vec![],
                recurring_issues: vec![],
            },
            environment_context: EnvironmentContext {
                deployment_environment: None,
                performance_requirements: vec![],
                security_requirements: vec![],
                compliance_requirements: vec![],
            },
        };

        let processing_duration = start_time.elapsed().as_millis() as u64;

        // Calculate quality score based on completeness
        let mut quality_score = 0.0;
        if !semantic_annotations.classifications.is_empty() {
            quality_score += 0.3;
        }
        if !relationships.is_empty() {
            quality_score += 0.3;
        }
        if !insights.is_empty() {
            quality_score += 0.4;
        }

        let enhancement_metadata = EnhancementMetadata {
            engine_version: "1.0.0".to_string(),
            processed_at: chrono::Utc::now(),
            processing_duration_ms: processing_duration,
            models_used,
            quality_score,
            confidence_distribution: HashMap::new(), // Would need more sophisticated calculation
        };

        Ok(EnhancedAnalysisResults {
            base_results: results.clone(),
            semantic_annotations,
            relationships,
            insights,
            context,
            enhancement_metadata,
        })
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec![
            "Pattern-based Classification".to_string(),
            "Impact Assessment".to_string(),
            "Relationship Detection".to_string(),
            "Basic Insight Generation".to_string(),
        ]
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// Calculate text similarity using simple word overlap
fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;

    fn create_test_finding(id: &str, message: &str, severity: Severity) -> Finding {
        Finding::new(
            "test_analyzer",
            "test_rule",
            severity,
            PathBuf::from("test.rs"),
            10,
            message.to_string(),
        )
        .with_description("Test description".to_string())
    }

    #[test]
    fn test_basic_enhancement_engine_creation() {
        let engine = BasicEnhancementEngine::new().unwrap();
        assert!(engine.is_available());
        assert!(!engine.get_capabilities().is_empty());
    }

    #[test]
    fn test_finding_classification() {
        let engine = BasicEnhancementEngine::new().unwrap();
        let finding =
            create_test_finding("1", "SQL injection vulnerability detected", Severity::High);

        let classification = engine.classify_finding(&finding);
        assert_eq!(classification.primary_category, "Security");
        assert!(classification.confidence > 0.0);
    }

    #[test]
    fn test_relationship_detection() {
        let engine = BasicEnhancementEngine::new().unwrap();
        let findings = vec![
            create_test_finding("1", "Issue in test.rs", Severity::High),
            create_test_finding("2", "Another issue in test.rs", Severity::Medium),
        ];

        let relationships = engine.detect_relationships(&findings);
        assert!(
            !relationships.is_empty(),
            "Should find relationships between findings in same file"
        );
        // Check that we have at least SameComponent relationships
        let has_same_component = relationships
            .iter()
            .any(|r| r.relationship_type == RelationshipType::SameComponent);
        assert!(
            has_same_component,
            "Should find SameComponent relationships for same file"
        );
    }

    #[test]
    fn test_enhancement_process() {
        let engine = BasicEnhancementEngine::new().unwrap();
        let mut results = AnalysisResults::new("test_config".to_string());

        results.add_finding(create_test_finding(
            "1",
            "SQL injection vulnerability",
            Severity::High,
        ));
        results.add_finding(create_test_finding(
            "2",
            "XSS vulnerability detected",
            Severity::High,
        ));
        results.add_finding(create_test_finding(
            "3",
            "Authentication bypass",
            Severity::Critical,
        ));
        results.add_finding(create_test_finding(
            "4",
            "Performance issue detected",
            Severity::Medium,
        ));

        let config = AIEnhancementConfig::default();
        let enhanced = engine.enhance_results(&results, &config).unwrap();

        assert_eq!(enhanced.base_results.findings.len(), 4);
        assert!(!enhanced.semantic_annotations.classifications.is_empty());
        assert!(
            !enhanced.relationships.is_empty(),
            "Relationships should not be empty"
        );
        assert!(!enhanced.insights.is_empty());
        assert!(enhanced.enhancement_metadata.quality_score > 0.0);
    }
}
