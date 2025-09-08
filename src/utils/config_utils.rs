//! Common configuration utilities for consistent config handling

use crate::config::Config;
use anyhow;

/// Enable all broken files detection features
pub fn enable_broken_files_detection(config: &mut Config) {
    config.analyzers.broken_files.enabled = true;
    config.analyzers.broken_files.detect_merge_conflicts = true;
    config.analyzers.broken_files.detect_ai_placeholders = true;
    config.analyzers.broken_files.detect_duplicates = true;
}

/// Enable specific broken files detection feature
pub fn enable_broken_files_feature(config: &mut Config, feature: BrokenFilesFeature) {
    config.analyzers.broken_files.enabled = true;

    match feature {
        BrokenFilesFeature::MergeConflicts => {
            config.analyzers.broken_files.detect_merge_conflicts = true;
        }
        BrokenFilesFeature::Placeholders => {
            config.analyzers.broken_files.detect_ai_placeholders = true;
        }
        BrokenFilesFeature::Duplicates => {
            config.analyzers.broken_files.detect_duplicates = true;
        }
        BrokenFilesFeature::All => {
            enable_broken_files_detection(config);
        }
    }
}

/// Set fail on conflicts configuration
pub fn set_fail_on_conflicts(config: &mut Config, fail: bool) {
    config.analyzers.broken_files.conflicts.fail_on_conflicts = fail;
}

/// Set parallel processing configuration
pub fn set_parallel_workers(config: &mut Config, workers: usize) {
    if workers > 0 {
        config.analysis.max_workers = workers as u32;
    }
}

/// Set ML threshold configuration
pub fn set_ml_threshold(config: &mut Config, threshold: f64) -> anyhow::Result<()> {
    if !(0.0..=1.0).contains(&threshold) {
        return Err(anyhow::anyhow!(
            "ML threshold must be between 0.0 and 1.0, got: {}",
            threshold
        ));
    }

    config.analysis.ml_threshold = Some(threshold);
    Ok(())
}

/// Set baseline file configuration
pub fn set_baseline_file(config: &mut Config, baseline_path: &std::path::Path) {
    config.analysis.baseline_file = Some(baseline_path.to_path_buf());
}

#[derive(Debug, Clone, Copy)]
pub enum BrokenFilesFeature {
    MergeConflicts,
    Placeholders,
    Duplicates,
    All,
}
