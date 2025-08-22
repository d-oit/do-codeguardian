use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

// Lazy static regex patterns for optimal performance
static OUTDATED_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""([^"]+)":\s*"([^"]+)""#).unwrap());

static DEV_DEPENDENCY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"devDependencies|dev-dependencies").unwrap());

/// Analyzer for dependency-related issues and security vulnerabilities
pub struct DependencyAnalyzer {
    // Known vulnerable package patterns
    vulnerable_packages: HashMap<String, VulnerabilityInfo>,
    // Use static references to avoid recompilation overhead
    #[allow(dead_code)]
    outdated_pattern: &'static Regex,
    #[allow(dead_code)]
    dev_dependency_pattern: &'static Regex,
}

#[derive(Debug, Clone)]
struct VulnerabilityInfo {
    severity: Severity,
    description: String,
    fixed_version: Option<String>,
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        let mut vulnerable_packages = HashMap::new();

        // Add some known vulnerable packages (this would typically be loaded from a database)
        vulnerable_packages.insert(
            "lodash".to_string(),
            VulnerabilityInfo {
                severity: Severity::High,
                description: "Prototype pollution vulnerability in older versions".to_string(),
                fixed_version: Some("4.17.21".to_string()),
            },
        );

        vulnerable_packages.insert(
            "serialize-javascript".to_string(),
            VulnerabilityInfo {
                severity: Severity::Critical,
                description: "XSS vulnerability in serialize-javascript".to_string(),
                fixed_version: Some("3.1.0".to_string()),
            },
        );

        Self {
            vulnerable_packages,
            outdated_pattern: &OUTDATED_PATTERN,
            dev_dependency_pattern: &DEV_DEPENDENCY_PATTERN,
        }
    }

    fn analyze_package_json(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Parse package.json for dependency analysis
        if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(content) {
            // Check dependencies
            if let Some(deps) = package_json.get("dependencies").and_then(|d| d.as_object()) {
                findings.extend(self.check_dependencies(file_path, deps, false)?);
            }

            // Check devDependencies
            if let Some(dev_deps) = package_json
                .get("devDependencies")
                .and_then(|d| d.as_object())
            {
                findings.extend(self.check_dependencies(file_path, dev_deps, true)?);
            }

            // Check for missing security-related fields
            findings.extend(self.check_security_fields(file_path, &package_json)?);
        }

        Ok(findings)
    }

    fn analyze_cargo_toml(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Parse Cargo.toml for dependency analysis
        if let Ok(cargo_toml) = toml::from_str::<toml::Value>(content) {
            // Check dependencies
            if let Some(deps) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
                findings.extend(self.check_rust_dependencies(file_path, deps)?);
            }

            // Check for insecure dependency specifications
            findings.extend(self.check_rust_security_practices(file_path, &cargo_toml)?);
        }

        Ok(findings)
    }

    fn check_dependencies(
        &self,
        file_path: &Path,
        deps: &serde_json::Map<String, serde_json::Value>,
        is_dev: bool,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for (package_name, version) in deps {
            // Check for known vulnerabilities
            if let Some(vuln_info) = self.vulnerable_packages.get(package_name) {
                let severity = if is_dev && vuln_info.severity == Severity::Critical {
                    Severity::High // Downgrade dev dependency criticals to high
                } else {
                    vuln_info.severity.clone()
                };

                let mut finding = Finding::new(
                    "dependency",
                    "vulnerable_package",
                    severity,
                    file_path.to_path_buf(),
                    1,
                    format!("Vulnerable package detected: {}", package_name),
                )
                .with_description(format!(
                    "Package '{}' has known security vulnerabilities: {}",
                    package_name, vuln_info.description
                ));

                if let Some(fixed_version) = &vuln_info.fixed_version {
                    finding = finding
                        .with_suggestion(format!("Update to version {} or later", fixed_version));
                }

                findings.push(finding);
            }

            // Check for wildcard or overly permissive version ranges
            if let Some(version_str) = version.as_str() {
                if version_str.contains("*") || version_str.starts_with(">=") {
                    findings.push(
                        Finding::new(
                            "dependency",
                            "permissive_version",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            1,
                            format!("Overly permissive version specification for {}", package_name),
                        )
                        .with_description("Wildcard or overly broad version ranges can introduce breaking changes".to_string())
                        .with_suggestion("Use more specific version ranges or exact versions for stability".to_string())
                    );
                }

                // Check for pre-release versions in production dependencies
                if !is_dev
                    && (version_str.contains("alpha")
                        || version_str.contains("beta")
                        || version_str.contains("rc"))
                {
                    findings.push(
                        Finding::new(
                            "dependency",
                            "prerelease_dependency",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            1,
                            format!("Pre-release dependency in production: {}", package_name),
                        )
                        .with_description(
                            "Pre-release versions may be unstable for production use".to_string(),
                        )
                        .with_suggestion(
                            "Consider using stable releases for production dependencies"
                                .to_string(),
                        ),
                    );
                }
            }
        }

        Ok(findings)
    }

    fn check_rust_dependencies(
        &self,
        file_path: &Path,
        deps: &toml::map::Map<String, toml::Value>,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for (package_name, version_spec) in deps {
            // Check for git dependencies without specific commits
            if let Some(table) = version_spec.as_table() {
                if table.contains_key("git")
                    && !table.contains_key("rev")
                    && !table.contains_key("tag")
                {
                    findings.push(
                        Finding::new(
                            "dependency",
                            "git_dependency_no_pin",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            1,
                            format!("Git dependency without pinned revision: {}", package_name),
                        )
                        .with_description(
                            "Git dependencies should be pinned to specific commits or tags"
                                .to_string(),
                        )
                        .with_suggestion(
                            "Add 'rev' or 'tag' field to pin the dependency to a specific version"
                                .to_string(),
                        ),
                    );
                }

                // Check for path dependencies in production
                if table.contains_key("path") {
                    findings.push(
                        Finding::new(
                            "dependency",
                            "path_dependency",
                            Severity::Low,
                            file_path.to_path_buf(),
                            1,
                            format!("Path dependency detected: {}", package_name),
                        )
                        .with_description(
                            "Path dependencies may not be available in all environments"
                                .to_string(),
                        )
                        .with_suggestion(
                            "Consider publishing to crates.io or use git dependencies".to_string(),
                        ),
                    );
                }
            }
        }

        Ok(findings)
    }

    fn check_security_fields(
        &self,
        file_path: &Path,
        package_json: &serde_json::Value,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for missing repository field
        if package_json.get("repository").is_none() {
            findings.push(
                Finding::new(
                    "dependency",
                    "missing_repository",
                    Severity::Low,
                    file_path.to_path_buf(),
                    1,
                    "Missing repository field in package.json".to_string(),
                )
                .with_description(
                    "Repository field helps with security auditing and transparency".to_string(),
                )
                .with_suggestion(
                    "Add repository field with your project's source code location".to_string(),
                ),
            );
        }

        // Check for missing license field
        if package_json.get("license").is_none() {
            findings.push(
                Finding::new(
                    "dependency",
                    "missing_license",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    1,
                    "Missing license field in package.json".to_string(),
                )
                .with_description("License field is important for legal compliance".to_string())
                .with_suggestion("Add license field specifying your project's license".to_string()),
            );
        }

        Ok(findings)
    }

    fn check_rust_security_practices(
        &self,
        file_path: &Path,
        cargo_toml: &toml::Value,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for missing metadata
        if let Some(package) = cargo_toml.get("package").and_then(|p| p.as_table()) {
            if package.get("license").is_none() && package.get("license-file").is_none() {
                findings.push(
                    Finding::new(
                        "dependency",
                        "missing_license",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        1,
                        "Missing license field in Cargo.toml".to_string(),
                    )
                    .with_description(
                        "License field is required for publishing and legal compliance".to_string(),
                    )
                    .with_suggestion(
                        "Add license field or license-file to [package] section".to_string(),
                    ),
                );
            }

            if package.get("repository").is_none() {
                findings.push(
                    Finding::new(
                        "dependency",
                        "missing_repository",
                        Severity::Low,
                        file_path.to_path_buf(),
                        1,
                        "Missing repository field in Cargo.toml".to_string(),
                    )
                    .with_description(
                        "Repository field helps with transparency and security auditing"
                            .to_string(),
                    )
                    .with_suggestion("Add repository field to [package] section".to_string()),
                );
            }
        }

        Ok(findings)
    }

    fn is_package_json(&self, file_path: &Path) -> bool {
        file_path.file_name().and_then(|n| n.to_str()) == Some("package.json")
    }

    fn is_cargo_toml(&self, file_path: &Path) -> bool {
        file_path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml")
    }
}

impl Analyzer for DependencyAnalyzer {
    fn name(&self) -> &str {
        "dependency"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);

        if self.is_package_json(file_path) {
            self.analyze_package_json(file_path, &content_str)
        } else if self.is_cargo_toml(file_path) {
            self.analyze_cargo_toml(file_path, &content_str)
        } else {
            Ok(Vec::new())
        }
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        self.is_package_json(file_path) || self.is_cargo_toml(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_vulnerable_package_detection() {
        let analyzer = DependencyAnalyzer::new();
        let package_json = r#"
        {
            "dependencies": {
                "lodash": "4.17.20",
                "express": "4.18.0"
            }
        }
        "#;

        let findings = analyzer
            .analyze_package_json(&PathBuf::from("package.json"), package_json)
            .unwrap();

        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.message.contains("lodash")));
    }

    #[test]
    fn test_permissive_version_detection() {
        let analyzer = DependencyAnalyzer::new();
        let package_json = r#"
        {
            "dependencies": {
                "express": "*",
                "react": ">=16.0.0"
            }
        }
        "#;

        let findings = analyzer
            .analyze_package_json(&PathBuf::from("package.json"), package_json)
            .unwrap();

        assert!(findings.len() >= 2);
        assert!(findings.iter().any(|f| f.rule == "permissive_version"));
    }
}
