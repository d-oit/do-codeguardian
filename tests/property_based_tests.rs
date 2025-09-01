use do_codeguardian::analyzers::{
    performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer, Analyzer,
};
use proptest::prelude::*;
use std::path::PathBuf;

proptest! {
    #[test]
    fn test_security_analyzer_never_panics(
        content in ".*{0,1000}",
        extension in "(rs|js|py|go|txt|md|json|toml|yaml)"
    ) {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from(format!("test.{}", extension));

        // This should never panic, regardless of input
        let result = std::panic::catch_unwind(|| {
            analyzer.analyze(&file_path, content.as_bytes())
        });

        prop_assert!(result.is_ok(), "SecurityAnalyzer panicked on input");

        if let Ok(analysis_result) = result {
            prop_assert!(analysis_result.is_ok(), "SecurityAnalyzer returned error");
        }
    }

    #[test]
    fn test_performance_analyzer_never_panics(
        content in ".*{0,1000}",
        extension in "(rs|js|py|go)"
    ) {
        let analyzer = PerformanceAnalyzer::new();
        let file_path = PathBuf::from(format!("test.{}", extension));

        let result = std::panic::catch_unwind(|| {
            analyzer.analyze(&file_path, content.as_bytes())
        });

        prop_assert!(result.is_ok(), "PerformanceAnalyzer panicked on input");
    }

    #[test]
    fn test_analyzers_handle_unicode(
        content in "[\\p{Any}]*{0,500}"
    ) {
        let security_analyzer = SecurityAnalyzer::new();
        let performance_analyzer = PerformanceAnalyzer::new();
        let file_path = PathBuf::from("test.rs");

        // Test both analyzers with unicode content
        let security_result = std::panic::catch_unwind(|| {
            security_analyzer.analyze(&file_path, content.as_bytes())
        });

        let performance_result = std::panic::catch_unwind(|| {
            performance_analyzer.analyze(&file_path, content.as_bytes())
        });

        prop_assert!(security_result.is_ok(), "SecurityAnalyzer panicked on unicode");
        prop_assert!(performance_result.is_ok(), "PerformanceAnalyzer panicked on unicode");
    }

    #[test]
    fn test_analyzers_handle_large_lines(
        line_size in 1..10000usize
    ) {
        let content = "x".repeat(line_size);
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("test.rs");

        let result = analyzer.analyze(&file_path, content.as_bytes());
        prop_assert!(result.is_ok(), "Analyzer failed on large line");
    }
}

#[cfg(test)]
mod deterministic_edge_cases {
    use super::*;

    #[test]
    fn test_empty_file() {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("empty.rs");

        let result = analyzer.analyze(&file_path, b"");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_binary_data() {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("binary.rs");

        // Test with binary data (all byte values)
        let binary_data: Vec<u8> = (0..=255).collect();
        let result = analyzer.analyze(&file_path, &binary_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_very_long_single_line() {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("long_line.rs");

        // 1MB single line
        let long_line = "x".repeat(1024 * 1024);
        let result = analyzer.analyze(&file_path, long_line.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn test_many_short_lines() {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("many_lines.rs");

        // 100k short lines
        let many_lines = "x\n".repeat(100_000);
        let result = analyzer.analyze(&file_path, many_lines.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn test_null_bytes() {
        let analyzer = SecurityAnalyzer::new();
        let file_path = PathBuf::from("null_bytes.rs");

        let content_with_nulls = b"let x = \"test\x00null\x00bytes\";";
        let result = analyzer.analyze(&file_path, content_with_nulls);
        assert!(result.is_ok());
    }
}
