use crate::types::{Finding, Severity};
use anyhow::Result;
use std::collections::HashMap;

/// Extract numerical features from findings for ML classification
pub struct FeatureExtractor {
    file_type_scores: HashMap<String, f32>,
    analyzer_confidence: HashMap<String, f32>,
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureExtractor {
    pub fn new() -> Self {
        let mut file_type_scores = HashMap::new();

        // Higher scores = more likely to have real issues
        file_type_scores.insert("rs".to_string(), 0.9); // Rust - high confidence
        file_type_scores.insert("js".to_string(), 0.7); // JavaScript - medium-high
        file_type_scores.insert("ts".to_string(), 0.8); // TypeScript - high
        file_type_scores.insert("py".to_string(), 0.8); // Python - high
        file_type_scores.insert("java".to_string(), 0.8); // Java - high
        file_type_scores.insert("cpp".to_string(), 0.9); // C++ - very high
        file_type_scores.insert("c".to_string(), 0.9); // C - very high
        file_type_scores.insert("go".to_string(), 0.8); // Go - high
        file_type_scores.insert("json".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("yaml".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("toml".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("md".to_string(), 0.3); // Markdown - low
        file_type_scores.insert("txt".to_string(), 0.2); // Text - very low

        let mut analyzer_confidence = HashMap::new();
        analyzer_confidence.insert("integrity".to_string(), 0.95); // Very reliable
        analyzer_confidence.insert("lint_drift".to_string(), 0.85); // Reliable
        analyzer_confidence.insert("non_production".to_string(), 0.75); // Good but some FPs

        Self {
            file_type_scores,
            analyzer_confidence,
        }
    }

    /// Extract enhanced feature vector from a finding (12 features)
    pub fn extract_features(&self, finding: &Finding) -> Result<Vec<f32>> {
        let mut features = Vec::with_capacity(12);

        // Feature 1: Severity score (0.0-1.0)
        features.push(self.severity_to_score(&finding.severity));

        // Feature 2: File type relevance (0.0-1.0)
        features.push(self.file_type_score(&finding.file));

        // Feature 3: Analyzer confidence (0.0-1.0)
        features.push(self.analyzer_confidence_score(&finding.analyzer));

        // Feature 4: Message length (normalized)
        features.push(self.normalize_message_length(&finding.message));

        // Feature 5: Line number (normalized, early lines often more important)
        features.push(self.normalize_line_number(finding.line));

        // Feature 6: Has description (0.0 or 1.0)
        features.push(if finding.description.is_some() {
            1.0
        } else {
            0.0
        });

        // Feature 7: Has suggestion (0.0 or 1.0)
        features.push(if finding.suggestion.is_some() {
            1.0
        } else {
            0.0
        });

        // Feature 8: Rule specificity (based on rule name length/complexity)
        features.push(self.rule_specificity_score(&finding.rule));

        // Feature 9: Message complexity (entropy-based)
        features.push(self.calculate_message_complexity(&finding.message));

        // Feature 10: File path depth (deeper paths often less critical)
        features.push(self.calculate_path_depth(&finding.file));

        // Feature 11: Rule category confidence (security vs quality vs performance)
        features.push(self.rule_category_confidence(&finding.rule, &finding.analyzer));

        // Feature 12: Context richness (combination of description + suggestion quality)
        features.push(self.calculate_context_richness(finding));

        Ok(features)
    }

    fn severity_to_score(&self, severity: &Severity) -> f32 {
        match severity {
            Severity::Critical => 1.0,
            Severity::High => 0.8,
            Severity::Medium => 0.6,
            Severity::Low => 0.4,
            Severity::Info => 0.2,
        }
    }

    fn file_type_score(&self, file_path: &std::path::Path) -> f32 {
        if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
            self.file_type_scores.get(extension).copied().unwrap_or(0.5)
        } else {
            0.3 // Files without extensions are less likely to have issues
        }
    }

    fn analyzer_confidence_score(&self, analyzer: &str) -> f32 {
        self.analyzer_confidence
            .get(analyzer)
            .copied()
            .unwrap_or(0.5)
    }

    fn normalize_message_length(&self, message: &str) -> f32 {
        let length = message.len() as f32;
        // Normalize to 0.0-1.0, with longer messages generally being more detailed/reliable
        (length / 200.0).min(1.0)
    }

    fn normalize_line_number(&self, line: u32) -> f32 {
        // Early lines in files are often more important (imports, declarations, etc.)
        // Use inverse relationship: line 1 = 1.0, line 1000+ = 0.1
        let normalized = 1.0 - ((line as f32 - 1.0) / 1000.0);
        normalized.clamp(0.1, 1.0)
    }

    fn rule_specificity_score(&self, rule: &str) -> f32 {
        // More specific rules (longer names, underscores) are often more reliable
        let length_score = (rule.len() as f32 / 50.0).min(1.0);
        let specificity_score = if rule.contains('_') || rule.contains('-') {
            0.8 // Specific rule names
        } else {
            0.5 // Generic rule names
        };

        (length_score + specificity_score) / 2.0
    }

    /// Update file type scores based on feedback
    #[allow(dead_code)]
    pub fn update_file_type_score(&mut self, extension: &str, is_reliable: bool) {
        let current_score = self.file_type_scores.get(extension).copied().unwrap_or(0.5);
        let adjustment = if is_reliable { 0.05 } else { -0.05 };
        let new_score = (current_score + adjustment).clamp(0.1, 0.9);
        self.file_type_scores
            .insert(extension.to_string(), new_score);
    }

    /// Update analyzer confidence based on feedback
    #[allow(dead_code)]
    pub fn update_analyzer_confidence(&mut self, analyzer: &str, is_reliable: bool) {
        let current_score = self
            .analyzer_confidence
            .get(analyzer)
            .copied()
            .unwrap_or(0.5);
        let adjustment = if is_reliable { 0.02 } else { -0.02 };
        let new_score = (current_score + adjustment).clamp(0.1, 0.95);
        self.analyzer_confidence
            .insert(analyzer.to_string(), new_score);
    }

    /// Calculate message complexity using entropy
    fn calculate_message_complexity(&self, message: &str) -> f32 {
        if message.is_empty() {
            return 0.0;
        }

        let mut char_counts = std::collections::HashMap::new();
        let chars: Vec<char> = message.chars().collect();

        for &ch in &chars {
            *char_counts.entry(ch).or_insert(0) += 1;
        }

        let len = chars.len() as f32;
        let entropy: f32 = char_counts
            .values()
            .map(|&count| {
                let p = count as f32 / len;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum();

        // Normalize entropy to 0.0-1.0 range (max entropy for ASCII is ~6.6)
        (entropy / 6.6).min(1.0)
    }

    /// Calculate file path depth score
    fn calculate_path_depth(&self, file_path: &std::path::Path) -> f32 {
        let depth = file_path.components().count();
        // Normalize: shallow paths (1-3 levels) = 1.0, deep paths (10+ levels) = 0.1
        let normalized = 1.0 - ((depth as f32 - 1.0) / 10.0);
        normalized.clamp(0.1, 1.0)
    }

    /// Determine rule category confidence based on rule and analyzer type
    fn rule_category_confidence(&self, rule: &str, analyzer: &str) -> f32 {
        // Security rules generally have higher confidence
        let security_patterns = ["secret", "injection", "xss", "crypto", "auth", "password"];
        let performance_patterns = ["loop", "memory", "clone", "inefficient", "blocking"];
        let quality_patterns = ["magic", "complex", "naming", "style", "format"];

        let rule_lower = rule.to_lowercase();
        let analyzer_lower = analyzer.to_lowercase();

        if security_patterns
            .iter()
            .any(|&p| rule_lower.contains(p) || analyzer_lower.contains(p))
        {
            0.9 // High confidence for security issues
        } else if performance_patterns
            .iter()
            .any(|&p| rule_lower.contains(p) || analyzer_lower.contains(p))
        {
            0.7 // Medium-high confidence for performance issues
        } else if quality_patterns
            .iter()
            .any(|&p| rule_lower.contains(p) || analyzer_lower.contains(p))
        {
            0.6 // Medium confidence for quality issues
        } else {
            0.5 // Default confidence
        }
    }

    /// Calculate context richness from description and suggestion quality
    fn calculate_context_richness(&self, finding: &Finding) -> f32 {
        let mut richness: f32 = 0.0;

        // Base score for having description
        if let Some(desc) = &finding.description {
            richness += 0.3;
            // Bonus for detailed descriptions
            if desc.len() > 50 {
                richness += 0.2;
            }
            // Bonus for technical terms
            if desc.contains("vulnerability")
                || desc.contains("security")
                || desc.contains("performance")
            {
                richness += 0.1;
            }
        }

        // Base score for having suggestion
        if let Some(sugg) = &finding.suggestion {
            richness += 0.3;
            // Bonus for actionable suggestions
            if sugg.contains("Use") || sugg.contains("Consider") || sugg.contains("Replace") {
                richness += 0.1;
            }
        }

        richness.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Finding;
    use std::path::PathBuf;

    #[test]
    fn test_feature_extraction() {
        let extractor = FeatureExtractor::new();

        let finding = Finding::new(
            "integrity",
            "corrupted_binary",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Potential binary file corruption detected".to_string(),
        )
        .with_description("File appears to contain corruption indicators".to_string())
        .with_suggestion("Verify file integrity".to_string());

        let features = extractor.extract_features(&finding).unwrap();

        assert_eq!(features.len(), 12);
        assert!(features[0] > 0.7); // High severity
        assert!(features[1] > 0.8); // Rust file
        assert!(features[2] > 0.9); // Integrity analyzer
        assert!(features[5] > 0.0); // Has description
        assert!(features[6] > 0.0); // Has suggestion
        assert!(features[8] > 0.0); // Message complexity
        assert!(features[9] > 0.0); // Path depth
        assert!(features[10] > 0.0); // Rule category confidence
        assert!(features[11] > 0.0); // Context richness
    }

    #[test]
    fn test_severity_scoring() {
        let extractor = FeatureExtractor::new();

        assert_eq!(extractor.severity_to_score(&Severity::Critical), 1.0);
        assert_eq!(extractor.severity_to_score(&Severity::High), 0.8);
        assert_eq!(extractor.severity_to_score(&Severity::Medium), 0.6);
        assert_eq!(extractor.severity_to_score(&Severity::Low), 0.4);
        assert_eq!(extractor.severity_to_score(&Severity::Info), 0.2);
    }
}
