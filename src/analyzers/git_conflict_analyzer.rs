use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Git conflict analyzer detects unresolved merge conflicts in files
pub struct GitConflictAnalyzer {
    conflict_start_pattern: Regex,
    conflict_separator_pattern: Regex,
    conflict_end_pattern: Regex,
    validate_syntax: bool,
}

impl Default for GitConflictAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl GitConflictAnalyzer {
    pub fn new() -> Self {
        Self {
            conflict_start_pattern: Regex::new(r"^<{7}(\s|$)").unwrap(),
            conflict_separator_pattern: Regex::new(r"^={7}(\s|$)").unwrap(),
            conflict_end_pattern: Regex::new(r"^>{7}(\s|$)").unwrap(),
            validate_syntax: true,
        }
    }

    pub fn with_syntax_validation(mut self, validate: bool) -> Self {
        self.validate_syntax = validate;
        self
    }

    /// Detect git merge conflict markers in file content
    fn detect_conflict_markers(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let mut conflict_state = ConflictState::None;
        let mut conflict_start_line = 0;

        for (line_num, line) in content.lines().enumerate() {
            let line_number = (line_num + 1) as u32;

            match conflict_state {
                ConflictState::None => {
                    if self.conflict_start_pattern.is_match(line) {
                        conflict_state = ConflictState::InConflict;
                        conflict_start_line = line_number;

                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "merge_conflict_start",
                                Severity::Critical,
                                file_path.to_path_buf(),
                                line_number,
                                "Git merge conflict marker detected".to_string(),
                            )
                            .with_description("Unresolved merge conflict start marker found. This indicates incomplete merge resolution.".to_string())
                            .with_suggestion("Resolve the merge conflict by choosing the appropriate code and removing conflict markers".to_string()),
                        );
                    } else if self.conflict_separator_pattern.is_match(line) {
                        // Standalone separator (orphaned)
                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "merge_conflict_separator",
                                Severity::Critical,
                                file_path.to_path_buf(),
                                line_number,
                                "Orphaned git merge conflict separator detected".to_string(),
                            )
                            .with_description("Merge conflict separator found without corresponding start marker.".to_string())
                            .with_suggestion("Remove orphaned conflict markers and ensure proper merge resolution".to_string()),
                        );
                    } else if self.conflict_end_pattern.is_match(line) {
                        // Standalone end marker (orphaned)
                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "merge_conflict_end",
                                Severity::Critical,
                                file_path.to_path_buf(),
                                line_number,
                                "Orphaned git merge conflict end marker detected".to_string(),
                            )
                            .with_description("Merge conflict end marker found without corresponding start marker.".to_string())
                            .with_suggestion("Remove orphaned conflict markers and ensure proper merge resolution".to_string()),
                        );
                    }
                }
                ConflictState::InConflict => {
                    if self.conflict_separator_pattern.is_match(line) {
                        conflict_state = ConflictState::InSeparator;

                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "merge_conflict_separator",
                                Severity::Critical,
                                file_path.to_path_buf(),
                                line_number,
                                "Git merge conflict separator detected".to_string(),
                            )
                            .with_description("Merge conflict separator marker found.".to_string())
                            .with_suggestion("Resolve the merge conflict by choosing the appropriate code and removing conflict markers".to_string()),
                        );
                    }
                }
                ConflictState::InSeparator => {
                    if self.conflict_end_pattern.is_match(line) {
                        conflict_state = ConflictState::None;

                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "merge_conflict_end",
                                Severity::Critical,
                                file_path.to_path_buf(),
                                line_number,
                                format!("Git merge conflict block ends (started at line {})", conflict_start_line),
                            )
                            .with_description("Merge conflict end marker found.".to_string())
                            .with_suggestion("Resolve the merge conflict by choosing the appropriate code and removing conflict markers".to_string()),
                        );
                    }
                }
            }
        }

        // If we're still in a conflict state at the end, it's malformed
        if conflict_state != ConflictState::None {
            findings.push(
                Finding::new(
                    "git_conflict",
                    "malformed_conflict",
                    Severity::Critical,
                    file_path.to_path_buf(),
                    conflict_start_line,
                    "Malformed git merge conflict detected".to_string(),
                )
                .with_description("Merge conflict started but never properly closed. This indicates corrupted conflict markers.".to_string())
                .with_suggestion("Check for missing conflict end markers and resolve the merge conflict properly".to_string()),
            );
        }

        findings
    }

    /// Validate file syntax if applicable (basic validation)
    fn validate_file_syntax(&self, content: &str, file_path: &Path) -> Vec<Finding> {
        if !self.validate_syntax {
            return Vec::new();
        }

        let mut findings = Vec::new();

        // Basic syntax validation for common file types
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "json" => {
                    if serde_json::from_str::<serde_json::Value>(content).is_err() {
                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "syntax_error",
                                Severity::High,
                                file_path.to_path_buf(),
                                1,
                                "JSON syntax error detected".to_string(),
                            )
                            .with_description("File contains invalid JSON syntax, possibly due to unresolved merge conflicts.".to_string())
                            .with_suggestion("Check for merge conflict markers and validate JSON syntax".to_string()),
                        );
                    }
                }
                "toml" => {
                    if toml::from_str::<toml::Value>(content).is_err() {
                        findings.push(
                            Finding::new(
                                "git_conflict",
                                "syntax_error",
                                Severity::High,
                                file_path.to_path_buf(),
                                1,
                                "TOML syntax error detected".to_string(),
                            )
                            .with_description("File contains invalid TOML syntax, possibly due to unresolved merge conflicts.".to_string())
                            .with_suggestion("Check for merge conflict markers and validate TOML syntax".to_string()),
                        );
                    }
                }
                _ => {
                    // For other file types, just check for basic structural issues
                    // that might indicate merge conflicts
                    self.detect_structural_issues(content, file_path, &mut findings);
                }
            }
        }

        findings
    }

    /// Detect structural issues that might indicate merge conflicts
    fn detect_structural_issues(
        &self,
        content: &str,
        file_path: &Path,
        findings: &mut Vec<Finding>,
    ) {
        let lines: Vec<&str> = content.lines().collect();

        // Check for suspicious patterns that might indicate unresolved conflicts
        for (line_num, line) in lines.iter().enumerate() {
            let line_number = (line_num + 1) as u32;

            // Look for repeated identical lines (common in conflicts)
            if line_num > 0
                && line_num < lines.len() - 1
                && lines[line_num - 1] == *line
                && lines[line_num + 1] == *line
                && !line.trim().is_empty()
            {
                findings.push(
                    Finding::new(
                        "git_conflict",
                        "suspicious_duplication",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        line_number,
                        "Suspicious line duplication detected".to_string(),
                    )
                    .with_description("Multiple identical consecutive lines found, which might indicate unresolved merge conflicts.".to_string())
                    .with_suggestion("Review for potential merge conflict artifacts".to_string()),
                );
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ConflictState {
    None,
    InConflict,
    InSeparator,
}

impl Analyzer for GitConflictAnalyzer {
    fn name(&self) -> &str {
        "git_conflict"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        let mut findings = Vec::new();

        // Detect conflict markers
        findings.extend(self.detect_conflict_markers(&content_str, file_path));

        // Validate syntax if enabled
        findings.extend(self.validate_file_syntax(&content_str, file_path));

        Ok(findings)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Support all text files - conflicts can appear in any file type
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            !matches!(
                ext,
                "exe"
                    | "bin"
                    | "so"
                    | "dll"
                    | "dylib"
                    | "a"
                    | "lib"
                    | "obj"
                    | "o"
                    | "png"
                    | "jpg"
                    | "jpeg"
                    | "gif"
                    | "ico"
                    | "svg"
                    | "pdf"
                    | "zip"
                    | "tar"
                    | "gz"
                    | "bz2"
                    | "xz"
            )
        } else {
            // Files without extensions might still be text files
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_complete_conflict() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
some code
<<<<<<< HEAD
version 1
=======
version 2
>>>>>>> branch
more code
"#;
        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();
        assert_eq!(findings.len(), 3); // start, separator, end
        assert!(findings.iter().any(|f| f.rule == "merge_conflict_start"));
        assert!(findings
            .iter()
            .any(|f| f.rule == "merge_conflict_separator"));
        assert!(findings.iter().any(|f| f.rule == "merge_conflict_end"));
    }

    #[test]
    fn test_detect_malformed_conflict() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
some code
<<<<<<< HEAD
version 1
=======
version 2
more code without end marker
"#;
        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();
        assert!(findings.iter().any(|f| f.rule == "malformed_conflict"));
    }

    #[test]
    fn test_no_conflicts() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
normal code
no conflicts here
"#;
        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_json_syntax_validation() {
        let analyzer = GitConflictAnalyzer::new();
        let invalid_json = r#"{"key": "value"#; // Missing closing brace
        let findings = analyzer
            .analyze(Path::new("test.json"), invalid_json.as_bytes())
            .unwrap();
        assert!(findings.iter().any(|f| f.rule == "syntax_error"));
    }

    #[test]
    fn test_supports_file() {
        let analyzer = GitConflictAnalyzer::new();
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("test.json")));
        assert!(analyzer.supports_file(Path::new("test.toml")));
        assert!(!analyzer.supports_file(Path::new("test.exe")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
    }
}
