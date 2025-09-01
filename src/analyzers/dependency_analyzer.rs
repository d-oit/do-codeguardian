use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use serde_json;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::warn;

// Use cargo-audit crate for future direct integration
// #[cfg(feature = "cargo-audit")]
// use cargo_audit::auditor::Auditor;
// #[cfg(feature = "cargo-audit")]
// use cargo_audit::config::AuditConfig;

/// Dependency analyzer for scanning Cargo.toml dependencies
pub struct DependencyAnalyzer {
    project_root: PathBuf,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer for the given project root
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Analyze dependencies for vulnerabilities, outdated packages, and license issues
    pub fn analyze_dependencies(&self) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for Cargo.toml
        let cargo_toml = self.project_root.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Ok(findings);
        }

        // Parse metadata
        let _metadata = MetadataCommand::new()
            .manifest_path(&cargo_toml)
            .exec()
            .context("Failed to parse Cargo.toml")?;

        // Analyze vulnerabilities
        findings.extend(self.analyze_vulnerabilities(&cargo_toml)?);

        // Note: Outdated packages and license compliance checks require external tools
        // and are not implemented in this basic version to avoid dependency conflicts

        Ok(findings)
    }

    /// Analyze for known vulnerabilities using cargo-audit
    fn analyze_vulnerabilities(&self, cargo_toml: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        #[cfg(feature = "cargo-audit")]
        {
            // Use cargo-audit crate directly for better integration
            match self.run_cargo_audit_direct() {
                Ok(audit_findings) => findings.extend(audit_findings),
                Err(e) => {
                    warn!(
                        "Failed to run cargo-audit directly: {}. Falling back to subprocess.",
                        e
                    );
                    findings.extend(self.run_cargo_audit_subprocess(cargo_toml)?);
                }
            }
        }

        #[cfg(not(feature = "cargo-audit"))]
        {
            // Fallback to subprocess when cargo-audit feature is not enabled
            findings.extend(self.run_cargo_audit_subprocess(cargo_toml)?);
        }

        Ok(findings)
    }

    #[cfg(feature = "cargo-audit")]
    fn run_cargo_audit_direct(&self) -> Result<Vec<Finding>> {
        // For now, fall back to subprocess approach due to API complexity
        // TODO: Implement direct cargo-audit integration when API stabilizes
        self.run_cargo_audit_subprocess(&self.project_root.join("Cargo.toml"))
    }

    fn run_cargo_audit_subprocess(&self, cargo_toml: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Run cargo audit as a subprocess (fallback method)
        let output = Command::new("cargo")
            .args(["audit", "--json"])
            .current_dir(&self.project_root)
            .output()
            .context("Failed to run cargo audit subprocess")?;

        if output.status.success() {
            // Parse the JSON output
            let json_output = String::from_utf8_lossy(&output.stdout);
            if let Ok(report) = serde_json::from_str::<serde_json::Value>(&json_output) {
                if let Some(vulnerabilities) =
                    report.get("vulnerabilities").and_then(|v| v.as_array())
                {
                    for vuln in vulnerabilities {
                        if let (Some(package), Some(advisory)) = (
                            vuln.get("package")
                                .and_then(|p| p.get("name"))
                                .and_then(|n| n.as_str()),
                            vuln.get("advisory"),
                        ) {
                            let title = advisory
                                .get("title")
                                .and_then(|t| t.as_str())
                                .unwrap_or("Unknown vulnerability");
                            let description = advisory
                                .get("description")
                                .and_then(|d| d.as_str())
                                .unwrap_or("");
                            let advisory_id =
                                advisory.get("id").and_then(|i| i.as_str()).unwrap_or("");

                            let finding = Finding::new(
                                "dependency-analyzer",
                                "vulnerability",
                                Severity::Critical,
                                cargo_toml.to_path_buf(),
                                0,
                                format!("Vulnerable dependency: {} - {}", package, title),
                            )
                            .with_description(description.to_string())
                            .with_suggestion(
                                "Run 'cargo audit' for detailed fix information".to_string(),
                            )
                            .with_metadata(
                                "advisory_id".to_string(),
                                advisory_id.to_string().into(),
                            );

                            findings.push(finding);
                        }
                    }
                }
            }
        } else {
            // Check if cargo-audit is installed
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("cargo-audit") || stderr.contains("not found") {
                warn!("cargo-audit not found. Install with: cargo install cargo-audit");
                warn!("Alternatively, enable the 'cargo-audit' feature in Cargo.toml for direct integration");
            } else {
                warn!("Failed to run vulnerability audit: {}", stderr);
            }
        }

        Ok(findings)
    }

    /// Check if a license is considered problematic for enterprise use
    pub fn is_problematic_license(&self, license: &str) -> bool {
        match license {
            // GPL licenses require derivative works to be GPL
            "GPL-2.0" | "GPL-2.0+" | "GPL-3.0" | "GPL-3.0+" => true,
            // LGPL licenses have linking restrictions
            "LGPL-2.1" | "LGPL-2.1+" | "LGPL-3.0" | "LGPL-3.0+" => true,
            // AGPL requires network service source disclosure
            "AGPL-3.0" | "AGPL-3.0+" => true,
            // Copyleft licenses
            "OSL-3.0" | "EPL-1.0" | "EPL-2.0" => true,
            // Permissive licenses are generally safe
            "MIT" | "Apache-2.0" | "BSD-2-Clause" | "BSD-3-Clause" | "ISC" => false,
            // Unknown licenses should be reviewed
            _ => true,
        }
    }
}

impl Analyzer for DependencyAnalyzer {
    fn name(&self) -> &str {
        "dependency-analyzer"
    }

    fn analyze(&self, file_path: &Path, _content: &[u8]) -> Result<Vec<Finding>> {
        // For dependency analysis, we only care about Cargo.toml files
        if file_path.file_name().and_then(|n| n.to_str()) != Some("Cargo.toml") {
            return Ok(Vec::new());
        }

        self.analyze_dependencies()
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        file_path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_dependency_analyzer_creation() {
        let temp_dir = tempdir().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        assert_eq!(analyzer.project_root, temp_dir.path());
    }

    #[test]
    fn test_analyze_dependencies_no_cargo_toml() {
        let temp_dir = tempdir().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        let findings = analyzer.analyze_dependencies().unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_is_problematic_license() {
        let temp_dir = tempdir().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());

        assert!(analyzer.is_problematic_license("GPL-3.0"));
        assert!(analyzer.is_problematic_license("LGPL-2.1"));
        assert!(!analyzer.is_problematic_license("MIT"));
        assert!(!analyzer.is_problematic_license("Apache-2.0"));
    }
}
