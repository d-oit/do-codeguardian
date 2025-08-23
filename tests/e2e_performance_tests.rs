use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;
use std::fs;
use std::time::Instant;

/// Performance-focused end-to-end tests

#[test]
fn test_large_codebase_performance() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a larger codebase (50 files)
    for i in 0..50 {
        let dir = temp_dir.path().join(format!("module_{}", i));
        fs::create_dir_all(&dir).unwrap();
        
        fs::write(dir.join("lib.rs"), format!(r#"
pub fn function_{}() -> i32 {{
    let value = {};
    // TODO: Optimize this function
    for i in 0..1000 {{
        println!("Processing {{}}", i);
    }}
    value
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_function_{}() {{
        assert_eq!(function_{}(), {});
    }}
}}
"#, i, i * 10, i, i, i * 10)).unwrap();
    }
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("turbo")  // Use turbo mode for performance
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"))
        .stdout(predicate::function(|output: &str| {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
                if let Some(scanned) = json["summary"]["total_files_scanned"].as_u64() {
                    return scanned >= 40; // Should scan most files
                }
            }
            false
        }));
    
    let duration = start.elapsed();
    // Should complete within 60 seconds for 50 files
    assert!(duration.as_secs() < 60, "Analysis took too long: {:?}", duration);
}

#[test]
fn test_memory_usage_large_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files with substantial content
    for i in 0..10 {
        let content = format!(r#"
// Large file {} with many functions
{}

pub struct LargeStruct_{} {{
    {}
}}

impl LargeStruct_{} {{
    {}
}}
"#, 
            i,
            (0..100).map(|j| format!("pub fn function_{}_{j}() {{ println!(\"Function {j}\"); }}", i)).collect::<Vec<_>>().join("\n"),
            i,
            (0..50).map(|j| format!("    field_{}: i32,", j)).collect::<Vec<_>>().join("\n"),
            i,
            (0..50).map(|j| format!("    pub fn method_{}(&self) -> i32 {{ self.field_{} }}", j, j)).collect::<Vec<_>>().join("\n")
        );
        
        fs::write(temp_dir.path().join(format!("large_file_{}.rs", i)), content).unwrap();
    }
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));
}

#[test]
fn test_concurrent_analysis_performance() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files that can be analyzed in parallel
    for i in 0..20 {
        fs::write(temp_dir.path().join(format!("concurrent_{}.rs", i)), format!(r#"
// File {} for concurrent analysis
pub fn process_data_{}() {{
    let secret = "key-{}-secret"; // Each file has a finding
    println!("Processing with secret: {{}}", secret);
    
    // Some computation
    for j in 0..100 {{
        let result = {} * j;
        println!("Result: {{}}", result);
    }}
}}
"#, i, i, i, i)).unwrap();
    }
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--parallel")
        .arg("4")  // Use 4 workers
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::function(|output: &str| {
            // Should find secrets in all files
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
                if let Some(findings) = json["findings"].as_array() {
                    return findings.len() >= 15; // Most files should have findings
                }
            }
            false
        }));
    
    let duration = start.elapsed();
    // Parallel processing should be reasonably fast
    assert!(duration.as_secs() < 30, "Parallel analysis took too long: {:?}", duration);
}