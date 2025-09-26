//! # Finding Relationships Module
//!
//! This module provides advanced relationship detection between security findings,
//! helping identify patterns, root causes, and vulnerability chains.

use super::{FindingRelationship, RelationshipType};
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Advanced relationship detector
pub struct RelationshipDetector {
    /// Rules for detecting different types of relationships
    detection_rules: Vec<Box<dyn RelationshipRule>>,
    /// Cache for computed relationships
    relationship_cache: HashMap<String, Vec<FindingRelationship>>,
}

/// Trait for relationship detection rules
pub trait RelationshipRule: Send + Sync {
    /// Name of the rule
    fn name(&self) -> &str;

    /// Relationship type this rule detects
    fn relationship_type(&self) -> RelationshipType;

    /// Detect relationships between two findings
    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64>;

    /// Additional evidence for the relationship
    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String>;
}

/// Rule for detecting same component relationships
pub struct SameComponentRule;

/// Rule for detecting causal relationships
pub struct CausalRelationshipRule;

/// Rule for detecting vulnerability chains
pub struct VulnerabilityChainRule;

/// Rule for detecting duplicate findings
pub struct DuplicateRule;

/// Rule for detecting amplification relationships
pub struct AmplificationRule;

/// Rule for detecting similar pattern relationships
pub struct SimilarPatternRule;

impl Default for RelationshipDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipDetector {
    /// Create a new relationship detector with default rules
    pub fn new() -> Self {
        let mut detector = Self {
            detection_rules: Vec::new(),
            relationship_cache: HashMap::new(),
        };

        detector.initialize_default_rules();
        detector
    }

    /// Initialize default relationship detection rules
    fn initialize_default_rules(&mut self) {
        self.detection_rules.push(Box::new(SameComponentRule));
        self.detection_rules.push(Box::new(CausalRelationshipRule));
        self.detection_rules.push(Box::new(VulnerabilityChainRule));
        self.detection_rules.push(Box::new(DuplicateRule));
        self.detection_rules.push(Box::new(AmplificationRule));
        self.detection_rules.push(Box::new(SimilarPatternRule));
    }

    /// Detect all relationships between findings
    pub fn detect_relationships(
        &mut self,
        findings: &[Finding],
    ) -> Result<Vec<FindingRelationship>> {
        let cache_key = self.generate_cache_key(findings);

        // Check cache first
        if let Some(cached_relationships) = self.relationship_cache.get(&cache_key) {
            return Ok(cached_relationships.clone());
        }

        let mut relationships = Vec::new();

        // Compare each pair of findings
        for (i, finding_a) in findings.iter().enumerate() {
            for finding_b in findings.iter().skip(i + 1) {
                // Apply each detection rule
                for rule in &self.detection_rules {
                    if let Some(confidence) = rule.detect(finding_a, finding_b) {
                        let evidence = rule.evidence(finding_a, finding_b);

                        relationships.push(FindingRelationship {
                            source_id: finding_a.id.clone(),
                            target_id: finding_b.id.clone(),
                            relationship_type: rule.relationship_type(),
                            strength: confidence,
                            description: format!(
                                "{} relationship detected between findings (confidence: {:.1}%)",
                                rule.name(),
                                confidence * 100.0
                            ),
                            evidence,
                        });
                    }
                }
            }
        }

        // Apply post-processing to refine relationships
        relationships = self.post_process_relationships(relationships);

        // Cache the results
        self.relationship_cache
            .insert(cache_key, relationships.clone());

        Ok(relationships)
    }

    /// Post-process relationships to remove duplicates and refine connections
    fn post_process_relationships(
        &self,
        mut relationships: Vec<FindingRelationship>,
    ) -> Vec<FindingRelationship> {
        // Sort by confidence (highest first)
        relationships.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Remove weak relationships if stronger ones exist between the same findings
        let mut seen_pairs = HashSet::new();
        relationships.retain(|rel| {
            let pair = if rel.source_id < rel.target_id {
                (rel.source_id.clone(), rel.target_id.clone())
            } else {
                (rel.target_id.clone(), rel.source_id.clone())
            };

            if seen_pairs.contains(&pair) {
                false // Remove weaker duplicate
            } else {
                seen_pairs.insert(pair);
                true
            }
        });

        // Filter out low-confidence relationships
        relationships.retain(|rel| rel.strength > 0.5);

        relationships
    }

    /// Generate cache key for findings
    fn generate_cache_key(&self, findings: &[Finding]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for finding in findings {
            finding.id.hash(&mut hasher);
        }
        format!("relationships_{:x}", hasher.finish())
    }

    /// Get relationship statistics
    pub fn get_relationship_stats(
        &self,
        relationships: &[FindingRelationship],
    ) -> RelationshipStats {
        let mut stats = RelationshipStats {
            total_relationships: 0,
            by_type: HashMap::new(),
            high_confidence: 0,
            medium_confidence: 0,
            low_confidence: 0,
            average_confidence: 0.0,
        };

        stats.total_relationships = relationships.len();

        for rel in relationships {
            *stats
                .by_type
                .entry(format!("{:?}", rel.relationship_type))
                .or_insert(0) += 1;

            if rel.strength > 0.8 {
                stats.high_confidence += 1;
            } else if rel.strength > 0.6 {
                stats.medium_confidence += 1;
            } else {
                stats.low_confidence += 1;
            }
        }

        stats.average_confidence = relationships.iter().map(|r| r.strength).sum::<f64>()
            / relationships.len().max(1) as f64;

        stats
    }
}

/// Statistics about detected relationships
#[derive(Debug, Default)]
pub struct RelationshipStats {
    pub total_relationships: usize,
    pub by_type: HashMap<String, usize>,
    pub high_confidence: usize,
    pub medium_confidence: usize,
    pub low_confidence: usize,
    pub average_confidence: f64,
}

impl RelationshipRule for SameComponentRule {
    fn name(&self) -> &str {
        "Same Component"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::SameComponent
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Same file = high confidence same component
        if finding_a.file == finding_b.file {
            // Closer line numbers = higher confidence
            let line_distance = (finding_a.line as i32 - finding_b.line as i32).abs();
            let confidence = if line_distance <= 10 {
                0.95
            } else if line_distance <= 50 {
                0.85
            } else if line_distance <= 100 {
                0.75
            } else {
                0.65
            };

            return Some(confidence);
        }

        // Same directory = medium confidence
        if let (Some(dir_a), Some(dir_b)) = (finding_a.file.parent(), finding_b.file.parent()) {
            if dir_a == dir_b {
                return Some(0.6);
            }
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if finding_a.file == finding_b.file {
            evidence.push(format!(
                "Both findings in same file: {}",
                finding_a.file.display()
            ));
            let line_distance = (finding_a.line as i32 - finding_b.line as i32).abs();
            evidence.push(format!("Line distance: {} lines", line_distance));
        } else if let (Some(dir_a), Some(dir_b)) =
            (finding_a.file.parent(), finding_b.file.parent())
        {
            if dir_a == dir_b {
                evidence.push(format!(
                    "Both findings in same directory: {}",
                    dir_a.display()
                ));
            }
        }

        evidence
    }
}

impl CausalRelationshipRule {
    fn is_auth_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("authentication") || text.contains("login") || text.contains("auth")
    }

    fn is_authz_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("authorization")
            || text.contains("access control")
            || text.contains("permission")
    }

    fn is_validation_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("validation") || text.contains("input") || text.contains("sanitiz")
    }

    fn is_injection_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("injection") || text.contains("xss") || text.contains("sql")
    }

    fn is_memory_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("memory") || text.contains("buffer") || text.contains("leak")
    }

    fn is_crash_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("crash") || text.contains("segfault") || text.contains("null pointer")
    }
}

impl RelationshipRule for CausalRelationshipRule {
    fn name(&self) -> &str {
        "Causal Relationship"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::Causes
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Authentication issues can cause authorization issues
        if self.is_auth_issue(finding_a) && self.is_authz_issue(finding_b) {
            return Some(0.8);
        }

        // Input validation issues can cause injection attacks
        if self.is_validation_issue(finding_a) && self.is_injection_issue(finding_b) {
            return Some(0.75);
        }

        // Memory issues can cause crashes
        if self.is_memory_issue(finding_a) && self.is_crash_issue(finding_b) {
            return Some(0.7);
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if self.is_auth_issue(finding_a) && self.is_authz_issue(finding_b) {
            evidence.push("Authentication bypass can lead to authorization failures".to_string());
        }

        if self.is_validation_issue(finding_a) && self.is_injection_issue(finding_b) {
            evidence.push("Lack of input validation enables injection attacks".to_string());
        }

        evidence
    }
}

impl VulnerabilityChainRule {
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
            "password",
            "token",
            "session",
            "privilege",
            "access",
        ];

        security_keywords
            .iter()
            .any(|keyword| text.contains(keyword))
    }
}

impl RelationshipRule for VulnerabilityChainRule {
    fn name(&self) -> &str {
        "Vulnerability Chain"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::VulnerabilityChain
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Both are security findings with high severity
        if self.is_security_finding(finding_a)
            && self.is_security_finding(finding_b)
            && matches!(finding_a.severity, Severity::Critical | Severity::High)
            && matches!(finding_b.severity, Severity::Critical | Severity::High)
        {
            // Same component makes it more likely to be a chain
            if finding_a.file == finding_b.file {
                return Some(0.7);
            } else {
                return Some(0.5);
            }
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if self.is_security_finding(finding_a) && self.is_security_finding(finding_b) {
            evidence.push("Both findings are security-related".to_string());

            if matches!(finding_a.severity, Severity::Critical | Severity::High) {
                evidence.push(
                    "High severity security issues can form vulnerability chains".to_string(),
                );
            }

            if finding_a.file == finding_b.file {
                evidence.push("Same file location increases chain likelihood".to_string());
            }
        }

        evidence
    }
}

impl RelationshipRule for DuplicateRule {
    fn name(&self) -> &str {
        "Duplicate"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::Duplicate
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Exact same analyzer and rule
        if finding_a.analyzer == finding_b.analyzer && finding_a.rule == finding_b.rule {
            // Same file and very close lines = likely duplicate
            if finding_a.file == finding_b.file {
                let line_distance = (finding_a.line as i32 - finding_b.line as i32).abs();
                if line_distance <= 5 {
                    return Some(0.95);
                }
            }

            // Same message = likely duplicate even in different files
            let message_similarity =
                calculate_message_similarity(&finding_a.message, &finding_b.message);
            if message_similarity > 0.9 {
                return Some(0.9);
            }
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if finding_a.analyzer == finding_b.analyzer && finding_a.rule == finding_b.rule {
            evidence.push("Same analyzer and rule".to_string());

            if finding_a.file == finding_b.file {
                let line_distance = (finding_a.line as i32 - finding_b.line as i32).abs();
                evidence.push(format!("Same file, {} lines apart", line_distance));
            }

            let message_similarity =
                calculate_message_similarity(&finding_a.message, &finding_b.message);
            if message_similarity > 0.8 {
                evidence.push(format!(
                    "Similar messages ({:.1}% similarity)",
                    message_similarity * 100.0
                ));
            }
        }

        evidence
    }
}

impl AmplificationRule {
    fn is_performance_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("performance") || text.contains("slow") || text.contains("timeout")
    }

    fn is_security_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("security") || text.contains("vulnerability")
    }

    fn is_error_handling_issue(&self, finding: &Finding) -> bool {
        let text = format!("{} {}", finding.message, finding.rule).to_lowercase();
        text.contains("error") || text.contains("exception") || text.contains("handle")
    }
}

impl RelationshipRule for AmplificationRule {
    fn name(&self) -> &str {
        "Amplification"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::Amplifies
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Performance issue + security issue = amplified risk
        if self.is_performance_issue(finding_a) && self.is_security_issue(finding_b) {
            return Some(0.6);
        }

        // Error handling issue + any other issue = amplified impact
        if self.is_error_handling_issue(finding_a) {
            return Some(0.55);
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if self.is_performance_issue(finding_a) && self.is_security_issue(finding_b) {
            evidence.push("Performance issues can amplify security vulnerabilities".to_string());
        }

        if self.is_error_handling_issue(finding_a) {
            evidence.push("Poor error handling amplifies impact of other issues".to_string());
        }

        evidence
    }
}

impl RelationshipRule for SimilarPatternRule {
    fn name(&self) -> &str {
        "Similar Pattern"
    }

    fn relationship_type(&self) -> RelationshipType {
        RelationshipType::SimilarPattern
    }

    fn detect(&self, finding_a: &Finding, finding_b: &Finding) -> Option<f64> {
        if finding_a.id == finding_b.id {
            return None;
        }

        // Same analyzer suggests similar detection patterns
        if finding_a.analyzer == finding_b.analyzer {
            let message_similarity =
                calculate_message_similarity(&finding_a.message, &finding_b.message);
            if message_similarity > 0.6 {
                return Some(message_similarity * 0.8); // Scale down confidence
            }
        }

        None
    }

    fn evidence(&self, finding_a: &Finding, finding_b: &Finding) -> Vec<String> {
        let mut evidence = Vec::new();

        if finding_a.analyzer == finding_b.analyzer {
            evidence.push(format!("Same analyzer: {}", finding_a.analyzer));

            let message_similarity =
                calculate_message_similarity(&finding_a.message, &finding_b.message);
            if message_similarity > 0.6 {
                evidence.push(format!(
                    "Similar messages ({:.1}% similarity)",
                    message_similarity * 100.0
                ));
            }
        }

        evidence
    }
}

/// Calculate similarity between two messages using word overlap
fn calculate_message_similarity(message1: &str, message2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = message1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = message2.split_whitespace().collect();

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

    fn create_test_finding(id: &str, message: &str, file: &str, line: u32) -> Finding {
        let mut finding = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from(file),
            line,
            message.to_string(),
        );
        finding.id = id.to_string();
        finding
    }

    #[test]
    fn test_same_component_rule() -> Result<(), Box<dyn std::error::Error>> {
        let rule = SameComponentRule;
        let finding_a = create_test_finding("test_id_a", "First issue", "test.rs", 10);
        let finding_b = create_test_finding("test_id_b", "Second issue", "test.rs", 15);

        let confidence = rule.detect(&finding_a, &finding_b);
        assert!(confidence.is_some());
        assert!(confidence? > 0.9); // Same file, close lines

        let evidence = rule.evidence(&finding_a, &finding_b);
        assert!(!evidence.is_empty());
    }

    #[test]
    fn test_relationship_detector() -> Result<(), Box<dyn std::error::Error>> {
        let mut detector = RelationshipDetector::new();
        let mut finding1 = create_test_finding("1", "authentication bypass", "auth.rs", 10);
        finding1.analyzer = "security".to_string();
        finding1.rule = "auth_bypass".to_string();

        let mut finding2 = create_test_finding("2", "authorization failure", "auth.rs", 50);
        finding2.analyzer = "security".to_string();
        finding2.rule = "authz_failure".to_string();

        let findings = vec![finding1, finding2];

        let relationships = detector.detect_relationships(&findings)?;
        assert!(!relationships.is_empty());

        let stats = detector.get_relationship_stats(&relationships);
        assert!(stats.total_relationships > 0);
    }

    #[test]
    fn test_duplicate_detection() -> Result<(), Box<dyn std::error::Error>> {
        let rule = DuplicateRule;
        let mut finding_a = create_test_finding("1", "Same message", "test.rs", 10);
        finding_a.analyzer = "test_analyzer".to_string();
        finding_a.rule = "test_rule".to_string();

        let mut finding_b = create_test_finding("2", "Same message", "test.rs", 12);
        finding_b.analyzer = "test_analyzer".to_string();
        finding_b.rule = "test_rule".to_string();

        let confidence = rule.detect(&finding_a, &finding_b)?;
        assert!(confidence > 0.9);
    }
}
