use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

/// Standard folder names that should follow kebab-case convention
const STANDARD_FOLDERS: &[&str] = &[
    "user-guide",
    "api",
    "architecture",
    "contributing",
    "deployment",
    "examples",
    "troubleshooting",
    "documentation",
    "docs",
];

/// Reserved or problematic file names that should be avoided
const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Analyzer for checking file and folder naming conventions and issues.
///
/// The NamingChecker performs comprehensive analysis of file and folder names
/// to identify potential issues that could lead to confusion, security problems,
/// or maintenance difficulties. It checks for typos in standard folder names,
/// inconsistent naming conventions, reserved names, and other naming anti-patterns.
pub struct NamingChecker {
    /// Set of standard folder names for quick lookup
    standard_folders: HashSet<String>,
    /// Set of reserved names for quick lookup
    reserved_names: HashSet<String>,
}

impl Default for NamingChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingChecker {
    /// Creates a new NamingChecker with default configuration.
    ///
    /// Initializes the analyzer with standard folder names, reserved names,
    /// and file extension mappings for comprehensive naming analysis.
    pub fn new() -> Self {
        let standard_folders = STANDARD_FOLDERS.iter().map(|s| s.to_string()).collect();
        let reserved_names = RESERVED_NAMES.iter().map(|s| s.to_string()).collect();

        Self {
            standard_folders,
            reserved_names,
        }
    }

    /// Checks if a folder name is a likely typo of a standard folder name.
    ///
    /// Uses edit distance to detect typos in standard folder names.
    /// Returns the closest match if the distance is small enough.
    fn detect_folder_typo(&self, folder_name: &str) -> Option<String> {
        let mut best_match = None;
        let mut best_distance = usize::MAX;

        for standard in &self.standard_folders {
            let distance = self.edit_distance(folder_name, standard);
            if distance < best_distance && distance <= 2 {
                // Allow up to 2 character differences
                best_distance = distance;
                best_match = Some(standard.clone());
            }
        }

        best_match
    }

    /// Calculates the edit distance between two strings using dynamic programming.
    ///
    /// This is a simplified Levenshtein distance implementation for detecting
    /// typos in folder names.
    fn edit_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
            row[0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };

                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i - 1][j] + 1, // deletion
                        matrix[i][j - 1] + 1, // insertion
                    ),
                    matrix[i - 1][j - 1] + cost, // substitution
                );
            }
        }

        matrix[len1][len2]
    }

    /// Detects the naming convention used in a name.
    ///
    /// Returns a string indicating the detected convention:
    /// - "kebab-case" for names with hyphens
    /// - "snake_case" for names with underscores
    /// - "camelCase" for names starting with lowercase and containing uppercase
    /// - "PascalCase" for names starting with uppercase
    /// - "unknown" for other patterns
    fn detect_naming_convention(&self, name: &str) -> &'static str {
        if name.contains('-') {
            "kebab-case"
        } else if name.contains('_') {
            "snake_case"
        } else if name.chars().next().is_some_and(|c| c.is_lowercase())
            && name.chars().any(|c| c.is_uppercase())
        {
            "camelCase"
        } else if name.chars().next().is_some_and(|c| c.is_uppercase()) {
            "PascalCase"
        } else {
            "unknown"
        }
    }

    /// Checks if a file has an appropriate extension for its content.
    ///
    /// Performs basic checks based on file extension and content patterns.
    /// This is a heuristic check and may not be 100% accurate.
    fn check_file_extension(&self, file_path: &Path, content: &[u8]) -> Option<String> {
        let _file_name = file_path.file_name()?.to_str()?;
        let extension = file_path.extension()?.to_str()?;

        let content_str = String::from_utf8_lossy(content);

        match extension {
            "rs" => {
                if !content_str.contains("fn ")
                    && !content_str.contains("struct ")
                    && !content_str.contains("impl ")
                    && !content_str.contains("use ")
                {
                    Some(
                        "File has .rs extension but doesn't appear to contain Rust code"
                            .to_string(),
                    )
                } else {
                    None
                }
            }
            "py" => {
                if !content_str.contains("def ")
                    && !content_str.contains("class ")
                    && !content_str.contains("import ")
                    && !content_str.contains("from ")
                {
                    Some(
                        "File has .py extension but doesn't appear to contain Python code"
                            .to_string(),
                    )
                } else {
                    None
                }
            }
            "js" => {
                if !content_str.contains("function")
                    && !content_str.contains("const ")
                    && !content_str.contains("let ")
                    && !content_str.contains("var ")
                {
                    Some(
                        "File has .js extension but doesn't appear to contain JavaScript code"
                            .to_string(),
                    )
                } else {
                    None
                }
            }
            "json" => {
                // Try to parse as JSON to verify it's valid JSON
                if serde_json::from_str::<serde_json::Value>(&content_str).is_err() {
                    Some("File has .json extension but contains invalid JSON".to_string())
                } else {
                    None
                }
            }
            "md" => {
                // Markdown files should contain some markdown elements
                if !content_str.contains('#')
                    && !content_str.contains('*')
                    && !content_str.contains('[')
                    && !content_str.contains('`')
                {
                    Some(
                        "File has .md extension but doesn't appear to contain Markdown formatting"
                            .to_string(),
                    )
                } else {
                    None
                }
            }
            "sh" | "bash" => {
                if !content_str.contains("#!/bin/") && !content_str.contains("#!/usr/bin/") {
                    Some(format!(
                        "File has .{} extension but doesn't appear to be a shell script",
                        extension
                    ))
                } else {
                    None
                }
            }
            _ => None, // For other extensions, we don't have specific checks
        }
    }

    /// Analyzes a file path for naming issues.
    ///
    /// This method performs comprehensive analysis of the file path including:
    /// - Checking for reserved names
    /// - Detecting typos in standard folder names
    /// - Checking naming convention consistency
    /// - Validating file extensions
    fn analyze_path(&self, file_path: &Path, content: &[u8]) -> Vec<Finding> {
        let mut findings = Vec::new();
        let _path_str = file_path.to_string_lossy();

        // Check for reserved names
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            // Check the full filename and also the name without extension
            let base_name = file_path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or(file_name);
            if self
                .reserved_names
                .contains(file_name.to_uppercase().as_str())
                || self
                    .reserved_names
                    .contains(base_name.to_uppercase().as_str())
            {
                findings.push(
                    Finding::new(
                        "naming_checker",
                        "reserved_name",
                        Severity::High,
                        file_path.to_path_buf(),
                        1,
                        format!("Reserved system name '{}' detected", base_name),
                    )
                    .with_description(
                        "This name conflicts with system reserved names and may cause issues on certain operating systems".to_string(),
                    )
                    .with_suggestion(
                        "Choose a different name that doesn't conflict with system reserved names".to_string(),
                    ),
                );
            }
        }

        // Check each component of the path
        for component in file_path.components() {
            if let std::path::Component::Normal(name) = component {
                if let Some(name_str) = name.to_str() {
                    // Check for typos in standard folder names
                    if let Some(suggestion) = self.detect_folder_typo(name_str) {
                        findings.push(
                            Finding::new(
                                "naming_checker",
                                "folder_name_typo",
                                Severity::Medium,
                                file_path.to_path_buf(),
                                1,
                                format!("Possible typo in folder name '{}'", name_str),
                            )
                            .with_description(
                                format!("Folder name '{}' appears to be a typo of standard folder name '{}'", name_str, suggestion),
                            )
                            .with_suggestion(
                                format!("Consider renaming to '{}' to follow project conventions", suggestion),
                            ),
                        );
                    }

                    // Check for problematic characters in names
                    if name_str.contains(' ') {
                        findings.push(
                            Finding::new(
                                "naming_checker",
                                "spaces_in_name",
                                Severity::Low,
                                file_path.to_path_buf(),
                                1,
                                format!("Spaces found in name '{}'", name_str),
                            )
                            .with_description(
                                "Names with spaces can cause issues with scripts, URLs, and some tools".to_string(),
                            )
                            .with_suggestion(
                                "Use hyphens or underscores instead of spaces (e.g., 'my-file' or 'my_file')".to_string(),
                            ),
                        );
                    }

                    // Check for mixed naming conventions in the same path
                    let convention = self.detect_naming_convention(name_str);
                    if convention != "unknown" {
                        // This is a simple check - in a real implementation, you might want to
                        // track conventions across the entire path or project
                        if name_str.contains('-') && name_str.contains('_') {
                            findings.push(
                                Finding::new(
                                    "naming_checker",
                                    "mixed_conventions",
                                    Severity::Low,
                                    file_path.to_path_buf(),
                                    1,
                                    format!("Mixed naming conventions in '{}'", name_str),
                                )
                                .with_description(
                                    "Name contains both hyphens and underscores, mixing kebab-case and snake_case".to_string(),
                                )
                                .with_suggestion(
                                    "Choose one naming convention and stick to it (preferably kebab-case for folders, snake_case for files)".to_string(),
                                ),
                            );
                        }
                    }
                }
            }
        }

        // Check file extension appropriateness
        if let Some(extension_issue) = self.check_file_extension(file_path, content) {
            findings.push(
                Finding::new(
                    "naming_checker",
                    "incorrect_extension",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    1,
                    "File extension doesn't match content".to_string(),
                )
                .with_description(extension_issue)
                .with_suggestion(
                    "Ensure the file extension matches the actual content type".to_string(),
                ),
            );
        }

        // Check for files with no extension (except for specific cases)
        if file_path.extension().is_none() {
            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                let allowed_no_extension = [
                    "Dockerfile",
                    "dockerfile",
                    "Makefile",
                    "makefile",
                    "CMakeLists.txt",
                    "Cargo.toml",
                    "package.json",
                    "tsconfig.json",
                    "webpack.config.js",
                    "rollup.config.js",
                    "vite.config.js",
                    "next.config.js",
                    "nuxt.config.js",
                ];

                if !allowed_no_extension.contains(&file_name) {
                    findings.push(
                        Finding::new(
                            "naming_checker",
                            "no_extension",
                            Severity::Info,
                            file_path.to_path_buf(),
                            1,
                            format!("File '{}' has no extension", file_name),
                        )
                        .with_description(
                            "Files without extensions can be harder to identify and may cause issues with some tools".to_string(),
                        )
                        .with_suggestion(
                            "Consider adding an appropriate file extension".to_string(),
                        ),
                    );
                }
            }
        }

        findings
    }
}

impl Analyzer for NamingChecker {
    fn name(&self) -> &str {
        "naming_checker"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        Ok(self.analyze_path(file_path, content))
    }

    fn supports_file(&self, _file_path: &Path) -> bool {
        // The naming checker analyzes file and folder names, so it supports all files
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_reserved_name_detection() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("src/CON.txt");
        let findings = checker.analyze(&path, b"test content").unwrap();

        assert!(findings.iter().any(|f| f.rule == "reserved_name"));
    }

    #[test]
    fn test_folder_typo_detection() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("documantation/README.md");
        let findings = checker.analyze(&path, b"# Documentation").unwrap();

        assert!(findings.iter().any(|f| f.rule == "folder_name_typo"));
    }

    #[test]
    fn test_spaces_in_name() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("src/my file.rs");
        let findings = checker.analyze(&path, b"fn main() {}").unwrap();

        assert!(findings.iter().any(|f| f.rule == "spaces_in_name"));
    }

    #[test]
    fn test_mixed_conventions() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("src/my-file_name.rs");
        let findings = checker.analyze(&path, b"fn main() {}").unwrap();

        assert!(findings.iter().any(|f| f.rule == "mixed_conventions"));
    }

    #[test]
    fn test_incorrect_extension() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("src/test.py");
        let content = b"not python code";
        let findings = checker.analyze(&path, content).unwrap();

        assert!(findings.iter().any(|f| f.rule == "incorrect_extension"));
    }

    #[test]
    fn test_no_extension() {
        let checker = NamingChecker::new();
        let path = PathBuf::from("src/README");
        let findings = checker.analyze(&path, b"This is a readme").unwrap();

        assert!(findings.iter().any(|f| f.rule == "no_extension"));
    }

    #[test]
    fn test_naming_convention_detection() {
        let checker = NamingChecker::new();

        assert_eq!(checker.detect_naming_convention("kebab-case"), "kebab-case");
        assert_eq!(checker.detect_naming_convention("snake_case"), "snake_case");
        assert_eq!(checker.detect_naming_convention("camelCase"), "camelCase");
        assert_eq!(checker.detect_naming_convention("PascalCase"), "PascalCase");
        assert_eq!(checker.detect_naming_convention("unknown"), "unknown");
    }

    #[test]
    fn test_edit_distance() {
        let checker = NamingChecker::new();

        assert_eq!(checker.edit_distance("test", "test"), 0);
        assert_eq!(checker.edit_distance("test", "tset"), 2);
        assert_eq!(checker.edit_distance("kitten", "sitting"), 3);
    }
}
