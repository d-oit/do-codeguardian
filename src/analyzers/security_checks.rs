use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;

/// Additional language-specific security checks
pub struct SecurityChecks {
    pub javascript: JavaScriptSecurity,
    pub python: PythonSecurity,
    pub php: PhpSecurity,
    pub java: JavaSecurity,
    pub rust: RustSecurity,
}

impl Default for SecurityChecks {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityChecks {
    pub fn new() -> Self {
        Self {
            javascript: JavaScriptSecurity,
            python: PythonSecurity,
            php: PhpSecurity,
            java: JavaSecurity,
            rust: RustSecurity,
        }
    }
}

pub struct JavaScriptSecurity;
impl JavaScriptSecurity {
    pub fn check(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe eval usage
        if line.contains("eval(") {
            findings.push(Finding::new(
                "security",
                "unsafe_eval",
                Severity::Critical,
                file_path.to_path_buf(),
                line_number,
                "Unsafe eval() usage detected".to_string(),
            ));
        }

        // Check for unsafe innerHTML with user data
        if line.contains("innerHTML")
            && (line.contains("req.") || line.contains("input") || line.contains("params"))
        {
            findings.push(Finding::new(
                "security",
                "unsafe_innerhtml",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Unsafe innerHTML with user data".to_string(),
            ));
        }

        Ok(findings)
    }
}

pub struct PythonSecurity;
impl PythonSecurity {
    pub fn check(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for pickle usage
        if line.contains("pickle.load") || line.contains("pickle.loads") {
            findings.push(Finding::new(
                "security",
                "unsafe_deserialization",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Unsafe pickle deserialization".to_string(),
            ));
        }

        Ok(findings)
    }
}

pub struct PhpSecurity;
impl PhpSecurity {
    pub fn check(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe include/require
        if (line.contains("include") || line.contains("require")) && line.contains("$_") {
            findings.push(Finding::new(
                "security",
                "file_inclusion",
                Severity::Critical,
                file_path.to_path_buf(),
                line_number,
                "Potential file inclusion vulnerability".to_string(),
            ));
        }

        Ok(findings)
    }
}

pub struct JavaSecurity;
impl JavaSecurity {
    pub fn check(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe deserialization
        if line.contains("ObjectInputStream") && line.contains("readObject") {
            findings.push(Finding::new(
                "security",
                "unsafe_deserialization",
                Severity::High,
                file_path.to_path_buf(),
                line_number,
                "Unsafe object deserialization".to_string(),
            ));
        }

        Ok(findings)
    }
}

pub struct RustSecurity;
impl RustSecurity {
    pub fn check(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unsafe blocks
        if line.contains("unsafe") && !line.trim_start().starts_with("//") {
            findings.push(Finding::new(
                "security",
                "unsafe_block",
                Severity::Medium,
                file_path.to_path_buf(),
                line_number,
                "Unsafe block detected".to_string(),
            ));
        }

        Ok(findings)
    }
}
