use do_codeguardian::analyzers::security_analyzer::SecurityAnalyzer;
use std::path::PathBuf;
use tempfile::tempdir;

fn main() {
    let analyzer = SecurityAnalyzer::new();
    let temp_dir = tempdir().unwrap();

    // Test with a simple hardcoded secret
    let test_content = r#"let api_key = "sk-1234567890abcdef";"#;
    let file_path = temp_dir.path().join("test.rs");
    std::fs::write(&file_path, test_content).unwrap();

    let findings = analyzer
        .analyze(&file_path, test_content.as_bytes())
        .unwrap();
    println!("Found {} findings", findings.len());
    for finding in &findings {
        println!("  - {}: {}", finding.rule, finding.message);
    }
}
