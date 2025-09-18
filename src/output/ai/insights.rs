//! # AI Insights Module
//!
//! This module generates intelligent insights and recommendations from analysis results.

use super::{FindingRelationship, Insight, InsightType, Recommendation};
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::collections::HashMap;

/// Insight generator for creating actionable recommendations
pub struct InsightGenerator {
    /// Insight generation rules
    rules: Vec<Box<dyn InsightRule>>,
}

/// Trait for insight generation rules
pub trait InsightRule: Send + Sync {
    /// Rule name
    fn name(&self) -> &str;

    /// Generate insights from findings
    fn generate_insights(
        &self,
        findings: &[Finding],
        relationships: &[FindingRelationship],
    ) -> Vec<Insight>;
}

/// Security pattern insight rule
pub struct SecurityPatternRule;

/// Quality hotspot insight rule
pub struct QualityHotspotRule;

/// Risk accumulation insight rule
pub struct RiskAccumulationRule;

/// Performance insight rule
pub struct PerformanceInsightRule;

impl Default for InsightGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl InsightGenerator {
    /// Create a new insight generator
    pub fn new() -> Self {
        let mut generator = Self { rules: Vec::new() };

        generator.initialize_default_rules();
        generator
    }

    /// Initialize default insight rules
    fn initialize_default_rules(&mut self) {
        self.rules.push(Box::new(SecurityPatternRule));
        self.rules.push(Box::new(QualityHotspotRule));
        self.rules.push(Box::new(RiskAccumulationRule));
        self.rules.push(Box::new(PerformanceInsightRule));
    }

    /// Generate insights from analysis results
    pub fn generate_insights(
        &self,
        results: &AnalysisResults,
        relationships: &[FindingRelationship],
    ) -> Result<Vec<Insight>> {
        let mut insights = Vec::new();

        for rule in &self.rules {
            let rule_insights = rule.generate_insights(&results.findings, relationships);
            insights.extend(rule_insights);
        }

        // Sort by priority and confidence
        insights.sort_by(|a, b| {
            let a_priority = self.priority_score(&a.priority);
            let b_priority = self.priority_score(&b.priority);

            b_priority.cmp(&a_priority).then_with(|| {
                b.confidence
                    .partial_cmp(&a.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        Ok(insights)
    }

    /// Convert priority string to numeric score
    fn priority_score(&self, priority: &str) -> u8 {
        match priority.to_lowercase().as_str() {
            "critical" => 4,
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }
}

impl InsightRule for SecurityPatternRule {
    fn name(&self) -> &str {
        "Security Pattern Detection"
    }

    fn generate_insights(
        &self,
        findings: &[Finding],
        relationships: &[FindingRelationship],
    ) -> Vec<Insight> {
        let mut insights = Vec::new();
        let mut insight_counter = 0;

        // Detect security vulnerability clusters
        let security_findings: Vec<_> = findings
            .iter()
            .filter(|f| self.is_security_finding(f))
            .collect();

        if security_findings.len() >= 3 {
            insight_counter += 1;
            insights.push(Insight {
                id: format!("security_cluster_{}", insight_counter),
                insight_type: InsightType::SecurityPattern,
                title: "Security Vulnerability Cluster Detected".to_string(),
                description: format!(
                    "Found {} security-related findings that may indicate systemic security weaknesses. This cluster suggests the need for a comprehensive security review.",
                    security_findings.len()
                ),
                confidence: 0.8,
                priority: "High".to_string(),
                affected_findings: security_findings.iter().map(|f| f.id.clone()).collect(),
                recommendations: vec![
                    Recommendation {
                        id: "sec_review_001".to_string(),
                        action: "Conduct comprehensive security audit".to_string(),
                        priority: 1,
                        estimated_effort: Some(16.0),
                        expected_benefit: "Significantly reduce security risk exposure".to_string(),
                        implementation_details: Some("Engage security specialists to review all findings and implement systematic fixes".to_string()),
                        required_resources: vec!["Security specialist".to_string(), "Development team".to_string()],
                    },
                    Recommendation {
                        id: "sec_training_001".to_string(),
                        action: "Implement security training for development team".to_string(),
                        priority: 2,
                        estimated_effort: Some(8.0),
                        expected_benefit: "Prevent future security vulnerabilities".to_string(),
                        implementation_details: None,
                        required_resources: vec!["Training materials".to_string(), "Training time".to_string()],
                    }
                ],
                supporting_data: HashMap::new(),
            });
        }

        // Detect authentication/authorization chains
        let auth_chain = self.detect_auth_chain(findings, relationships);
        if !auth_chain.is_empty() {
            insight_counter += 1;
            insights.push(Insight {
                id: format!("auth_chain_{}", insight_counter),
                insight_type: InsightType::SecurityPattern,
                title: "Authentication/Authorization Vulnerability Chain".to_string(),
                description: "Detected a chain of authentication and authorization vulnerabilities that could be exploited together for privilege escalation".to_string(),
                confidence: 0.9,
                priority: "Critical".to_string(),
                affected_findings: auth_chain,
                recommendations: vec![
                    Recommendation {
                        id: "auth_fix_001".to_string(),
                        action: "Implement comprehensive authentication and authorization framework".to_string(),
                        priority: 1,
                        estimated_effort: Some(24.0),
                        expected_benefit: "Prevent privilege escalation attacks".to_string(),
                        implementation_details: Some("Review and strengthen authentication mechanisms, implement proper access controls".to_string()),
                        required_resources: vec!["Security architect".to_string(), "Senior developer".to_string()],
                    }
                ],
                supporting_data: HashMap::new(),
            });
        }

        insights
    }
}

impl SecurityPatternRule {
    fn is_security_finding(&self, finding: &Finding) -> bool {
        let text = format!(
            "{} {} {}",
            finding.message,
            finding.rule,
            finding.description.as_deref().unwrap_or("")
        )
        .to_lowercase();

        let security_keywords = [
            "security",
            "vulnerability",
            "attack",
            "exploit",
            "injection",
            "xss",
            "csrf",
            "authentication",
            "authorization",
            "encryption",
        ];

        security_keywords
            .iter()
            .any(|keyword| text.contains(keyword))
    }

    fn detect_auth_chain(
        &self,
        findings: &[Finding],
        relationships: &[FindingRelationship],
    ) -> Vec<String> {
        let mut auth_findings = Vec::new();

        for finding in findings {
            let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
            if text.contains("auth") || text.contains("login") || text.contains("permission") {
                auth_findings.push(finding.id.clone());
            }
        }

        // Check if auth findings are related
        for rel in relationships {
            if auth_findings.contains(&rel.source_id) && auth_findings.contains(&rel.target_id) {
                return auth_findings;
            }
        }

        Vec::new()
    }
}

impl InsightRule for QualityHotspotRule {
    fn name(&self) -> &str {
        "Code Quality Hotspot Detection"
    }

    fn generate_insights(
        &self,
        findings: &[Finding],
        _relationships: &[FindingRelationship],
    ) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Group findings by file
        let mut file_counts = HashMap::new();
        for finding in findings {
            *file_counts.entry(finding.file.clone()).or_insert(0) += 1;
        }

        // Find hotspot files (files with many issues)
        let hotspots: Vec<_> = file_counts
            .iter()
            .filter(|(_, count)| **count >= 5)
            .collect();

        if !hotspots.is_empty() {
            insights.push(Insight {
                id: "quality_hotspots_001".to_string(),
                insight_type: InsightType::QualityIssue,
                title: "Code Quality Hotspots Identified".to_string(),
                description: format!(
                    "Found {} files with high concentrations of issues (5+ issues per file). These files may need refactoring.",
                    hotspots.len()
                ),
                confidence: 0.9,
                priority: "Medium".to_string(),
                affected_findings: findings.iter()
                    .filter(|f| hotspots.iter().any(|(path, _)| *path == &f.file))
                    .map(|f| f.id.clone())
                    .collect(),
                recommendations: vec![
                    Recommendation {
                        id: "refactor_001".to_string(),
                        action: "Prioritize refactoring of hotspot files".to_string(),
                        priority: 1,
                        estimated_effort: Some(hotspots.len() as f32 * 4.0),
                        expected_benefit: "Improve code maintainability and reduce future issues".to_string(),
                        implementation_details: Some("Break down large files, extract common functionality, improve code structure".to_string()),
                        required_resources: vec!["Senior developer".to_string(), "Code review time".to_string()],
                    }
                ],
                supporting_data: {
                    let mut data = HashMap::new();
                    data.insert("hotspot_files".to_string(), serde_json::json!(hotspots.len()));
                    data
                },
            });
        }

        insights
    }
}

impl InsightRule for RiskAccumulationRule {
    fn name(&self) -> &str {
        "Risk Accumulation Detection"
    }

    fn generate_insights(
        &self,
        findings: &[Finding],
        _relationships: &[FindingRelationship],
    ) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Calculate risk accumulation score
        let critical_count = findings
            .iter()
            .filter(|f| f.severity == Severity::Critical)
            .count();
        let high_count = findings
            .iter()
            .filter(|f| f.severity == Severity::High)
            .count();

        let risk_score = critical_count * 10 + high_count * 5;

        if risk_score > 20 {
            insights.push(Insight {
                id: "risk_accumulation_001".to_string(),
                insight_type: InsightType::RiskAccumulation,
                title: "High Risk Accumulation Detected".to_string(),
                description: format!(
                    "The combination of {} critical and {} high-severity issues creates significant cumulative risk (score: {}).",
                    critical_count, high_count, risk_score
                ),
                confidence: 0.85,
                priority: "High".to_string(),
                affected_findings: findings.iter()
                    .filter(|f| matches!(f.severity, Severity::Critical | Severity::High))
                    .map(|f| f.id.clone())
                    .collect(),
                recommendations: vec![
                    Recommendation {
                        id: "risk_mitigation_001".to_string(),
                        action: "Implement immediate risk mitigation plan".to_string(),
                        priority: 1,
                        estimated_effort: Some(critical_count as f32 * 4.0 + high_count as f32 * 2.0),
                        expected_benefit: "Reduce overall security and operational risk".to_string(),
                        implementation_details: Some("Prioritize critical issues first, then address high-severity issues systematically".to_string()),
                        required_resources: vec!["Development team".to_string(), "Security review".to_string()],
                    }
                ],
                supporting_data: {
                    let mut data = HashMap::new();
                    data.insert("risk_score".to_string(), serde_json::json!(risk_score));
                    data.insert("critical_issues".to_string(), serde_json::json!(critical_count));
                    data.insert("high_issues".to_string(), serde_json::json!(high_count));
                    data
                },
            });
        }

        insights
    }
}

impl InsightRule for PerformanceInsightRule {
    fn name(&self) -> &str {
        "Performance Impact Analysis"
    }

    fn generate_insights(
        &self,
        findings: &[Finding],
        _relationships: &[FindingRelationship],
    ) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Find performance-related issues
        let perf_findings: Vec<_> = findings
            .iter()
            .filter(|f| self.is_performance_finding(f))
            .collect();

        if perf_findings.len() >= 2 {
            insights.push(Insight {
                id: "performance_impact_001".to_string(),
                insight_type: InsightType::PerformanceIssue,
                title: "Performance Impact Detected".to_string(),
                description: format!(
                    "Found {} performance-related issues that may impact user experience and system responsiveness.",
                    perf_findings.len()
                ),
                confidence: 0.75,
                priority: "Medium".to_string(),
                affected_findings: perf_findings.iter().map(|f| f.id.clone()).collect(),
                recommendations: vec![
                    Recommendation {
                        id: "perf_optimization_001".to_string(),
                        action: "Conduct performance optimization review".to_string(),
                        priority: 2,
                        estimated_effort: Some(perf_findings.len() as f32 * 3.0),
                        expected_benefit: "Improve application responsiveness and user experience".to_string(),
                        implementation_details: Some("Profile application performance, optimize bottlenecks, implement caching where appropriate".to_string()),
                        required_resources: vec!["Performance specialist".to_string(), "Development time".to_string()],
                    }
                ],
                supporting_data: HashMap::new(),
            });
        }

        insights
    }
}

impl PerformanceInsightRule {
    fn is_performance_finding(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        let perf_keywords = [
            "performance",
            "slow",
            "timeout",
            "memory",
            "cpu",
            "optimization",
            "bottleneck",
        ];

        perf_keywords.iter().any(|keyword| text.contains(keyword))
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
    }

    #[test]
    fn test_insight_generator() {
        let generator = InsightGenerator::new();
        let findings = vec![
            create_test_finding("1", "SQL injection vulnerability", Severity::Critical),
            create_test_finding("2", "XSS vulnerability", Severity::High),
            create_test_finding("3", "Authentication bypass", Severity::Critical),
        ];

        let insights = generator
            .generate_insights(
                &crate::types::AnalysisResults {
                    schema_version: "1.0.0".to_string(),
                    tool_metadata: crate::types::ToolMetadata {
                        name: "test".to_string(),
                        version: "1.0.0".to_string(),
                        config_hash: "test".to_string(),
                        timestamp: chrono::Utc::now(),
                    },
                    findings,
                    summary: Default::default(),
                    config_hash: "test".to_string(),
                    timestamp: chrono::Utc::now(),
                },
                &[],
            )
            .unwrap();

        assert!(!insights.is_empty());
        assert!(insights
            .iter()
            .any(|i| i.insight_type == InsightType::SecurityPattern));
    }

    #[test]
    fn test_security_pattern_rule() {
        let rule = SecurityPatternRule;
        let findings = vec![
            create_test_finding("1", "SQL injection found", Severity::High),
            create_test_finding("2", "XSS vulnerability detected", Severity::High),
            create_test_finding("3", "Authentication bypass possible", Severity::Critical),
        ];

        let insights = rule.generate_insights(&findings, &[]);
        assert!(!insights.is_empty());
        assert_eq!(insights[0].insight_type, InsightType::SecurityPattern);
    }
}
