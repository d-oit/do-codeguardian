use crate::analyzers::Analyzer;
use crate::types::Finding;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// Build artifact duplicate analyzer for detecting and managing duplicate build outputs
#[derive(Debug)]
pub struct BuildArtifactAnalyzer {
    pub build_dirs: Vec<PathBuf>,
    pub artifact_patterns: Vec<String>,
    pub max_file_size: u64,
    hash_cache: HashMap<String, Vec<PathBuf>>,
    conflict_resolver: ConflictResolver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub path: PathBuf,
    pub size: u64,
    pub hash: String,
    pub artifact_type: ArtifactType,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArtifactType {
    Binary,
    Library,
    ObjectFile,
    Archive,
    SharedLibrary,
    Config,
    Other,
}

#[derive(Debug, Clone)]
pub struct DuplicateArtifact {
    pub hash: String,
    pub artifacts: Vec<BuildArtifact>,
    pub total_wasted_space: u64,
    pub conflict_level: ConflictLevel,
    pub cleanup_recommendation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct ConflictResolver {
    resolution_strategies: Vec<ResolutionStrategy>,
}

#[derive(Debug, Clone)]
pub enum ResolutionStrategy {
    KeepNewest,
    KeepLargest,
    KeepByPath(String),
    Interactive,
}

impl Default for BuildArtifactAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildArtifactAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            build_dirs: vec![
                PathBuf::from("target"),
                PathBuf::from("build"),
                PathBuf::from("dist"),
                PathBuf::from("out"),
            ],
            artifact_patterns: vec![
                "*.so".to_string(),
                "*.dylib".to_string(),
                "*.dll".to_string(),
                "*.a".to_string(),
                "*.lib".to_string(),
                "*.o".to_string(),
                "*.exe".to_string(),
                "*.bin".to_string(),
            ],
            max_file_size: 100 * 1024 * 1024, // 100MB
            hash_cache: HashMap::new(),
            conflict_resolver: ConflictResolver::new(),
        };
        analyzer.initialize_patterns();
        analyzer
    }

    pub fn with_build_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.build_dirs = dirs;
        self
    }

    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    fn initialize_patterns(&mut self) {
        // Add more patterns based on common build systems
        self.artifact_patterns.extend(vec![
            "Cargo.toml".to_string(), // For dependency analysis
            "package.json".to_string(),
            "requirements.txt".to_string(),
            "pom.xml".to_string(),
            "build.gradle".to_string(),
        ]);
    }

    /// Scan build directories for artifacts and detect duplicates
    pub fn scan_build_artifacts(&mut self) -> Result<Vec<DuplicateArtifact>> {
        let mut artifacts = Vec::new();

        // Scan all build directories
        for build_dir in &self.build_dirs {
            if build_dir.exists() {
                artifacts.extend(self.scan_directory(build_dir)?);
            }
        }

        // Group by hash to find duplicates
        self.group_artifacts_by_hash(artifacts)
    }

    fn scan_directory(&self, dir: &Path) -> Result<Vec<BuildArtifact>> {
        let mut artifacts = Vec::new();

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(artifact) = self.analyze_file(entry.path())? {
                    artifacts.push(artifact);
                }
            }
        }

        Ok(artifacts)
    }

    fn analyze_file(&self, path: &Path) -> Result<Option<BuildArtifact>> {
        // Check file size limit
        let metadata = fs::metadata(path)?;
        if metadata.len() > self.max_file_size {
            return Ok(None);
        }

        // Check if it's an artifact we care about
        if !self.is_build_artifact(path) {
            return Ok(None);
        }

        // Calculate hash
        let hash = self.calculate_file_hash(path)?;

        // Determine artifact type
        let artifact_type = self.determine_artifact_type(path);

        // Extract dependencies if applicable
        let dependencies = self.extract_dependencies(path)?;

        Ok(Some(BuildArtifact {
            path: path.to_path_buf(),
            size: metadata.len(),
            hash,
            artifact_type,
            dependencies,
        }))
    }

    fn is_build_artifact(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            // Check patterns
            for pattern in &self.artifact_patterns {
                if let Some(ext) = pattern.strip_prefix("*.") {
                    // Handle glob patterns like "*.so"
                    // Remove "*."
                    if file_name.ends_with(ext) {
                        return true;
                    }
                } else if pattern == file_name {
                    // Handle exact matches like "Cargo.toml"
                    return true;
                }
            }

            // Check if it's in a build directory
            if let Some(parent) = path.parent() {
                let parent_str = parent.to_string_lossy();
                if parent_str.contains("target/")
                    || parent_str.contains("build/")
                    || parent_str.contains("dist/")
                    || parent_str.contains("out/")
                {
                    return true;
                }
            }
        }
        false
    }

    fn determine_artifact_type(&self, path: &Path) -> ArtifactType {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "so" | "dylib" | "dll" => ArtifactType::SharedLibrary,
                "a" | "lib" => ArtifactType::Archive,
                "o" | "obj" => ArtifactType::ObjectFile,
                "exe" | "bin" | "out" => ArtifactType::Binary,
                "jar" | "war" | "ear" => ArtifactType::Archive, // Java archives
                "deb" | "rpm" => ArtifactType::Binary,          // Package files
                _ => ArtifactType::Other,
            }
        } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            // Handle files without extensions
            if file_name.starts_with("lib") && file_name.contains(".so") {
                ArtifactType::SharedLibrary
            } else if file_name.starts_with("lib") && file_name.ends_with(".a") {
                ArtifactType::Archive
            } else if file_name.contains("binary") || file_name.contains("executable") {
                ArtifactType::Binary
            } else {
                ArtifactType::Other
            }
        } else {
            ArtifactType::Other
        }
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let content = fs::read(path)?;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    fn extract_dependencies(&self, path: &Path) -> Result<Vec<String>> {
        let mut dependencies = Vec::new();

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            match file_name {
                "Cargo.toml" => {
                    dependencies.extend(self.extract_cargo_dependencies(path)?);
                }
                "package.json" => {
                    dependencies.extend(self.extract_npm_dependencies(path)?);
                }
                "requirements.txt" => {
                    dependencies.extend(self.extract_pip_dependencies(path)?);
                }
                _ => {
                    // For binary artifacts, try to extract dependency info
                    if matches!(
                        self.determine_artifact_type(path),
                        ArtifactType::Binary | ArtifactType::SharedLibrary
                    ) {
                        dependencies.extend(self.extract_binary_dependencies(path)?);
                    }
                }
            }
        }

        Ok(dependencies)
    }

    fn extract_cargo_dependencies(&self, path: &Path) -> Result<Vec<String>> {
        // Simple TOML parsing for dependencies
        let content = fs::read_to_string(path)?;
        let mut deps = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.contains("dependencies") {
                // Found dependencies section - parse following lines
                // This is a simplified implementation
                deps.push("cargo-dependencies".to_string());
                break;
            }
        }

        Ok(deps)
    }

    fn extract_npm_dependencies(&self, path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(path)?;
        let mut deps = Vec::new();

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps_obj) = json.get("dependencies").and_then(|d| d.as_object()) {
                deps.extend(deps_obj.keys().cloned());
            }
        }

        Ok(deps)
    }

    fn extract_pip_dependencies(&self, path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(path)?;
        let mut deps = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                // Extract package name (before version specifiers)
                if let Some(package) = line.split(&['=', '>', '<', '!'][..]).next() {
                    deps.push(package.trim().to_string());
                }
            }
        }

        Ok(deps)
    }

    fn extract_binary_dependencies(&self, path: &Path) -> Result<Vec<String>> {
        // Use system tools to extract dependencies from binaries
        let mut deps = Vec::new();

        // Try ldd on Unix-like systems
        #[cfg(unix)]
        {
            if let Ok(output) = std::process::Command::new("ldd").arg(path).output() {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines() {
                        if line.contains("=>") {
                            if let Some(lib) =
                                line.split("=>").nth(1).and_then(|s| s.split('(').next())
                            {
                                deps.push(lib.trim().to_string());
                            }
                        }
                    }
                }
            }
        }

        // Try otool on macOS
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("otool")
                .args(&["-L", &path.to_string_lossy()])
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines().skip(1) {
                        // Skip first line
                        if let Some(lib) = line.split('(').next() {
                            deps.push(lib.trim().to_string());
                        }
                    }
                }
            }
        }

        Ok(deps)
    }

    fn group_artifacts_by_hash(
        &mut self,
        artifacts: Vec<BuildArtifact>,
    ) -> Result<Vec<DuplicateArtifact>> {
        let mut hash_groups: HashMap<String, Vec<BuildArtifact>> = HashMap::new();

        // Group artifacts by hash
        for artifact in artifacts {
            hash_groups
                .entry(artifact.hash.clone())
                .or_default()
                .push(artifact.clone());
            self.hash_cache
                .entry(artifact.hash.clone())
                .or_default()
                .push(artifact.path.clone());
        }

        // Create duplicate artifacts for groups with more than one item
        let mut duplicates = Vec::new();
        for (hash, artifacts) in hash_groups {
            if artifacts.len() > 1 {
                let total_wasted_space = artifacts.iter().skip(1).map(|a| a.size).sum();
                let conflict_level = self.assess_conflict_level(&artifacts);
                let cleanup_recommendation =
                    self.generate_cleanup_recommendation(&artifacts, conflict_level.clone());

                duplicates.push(DuplicateArtifact {
                    hash,
                    artifacts,
                    total_wasted_space,
                    conflict_level,
                    cleanup_recommendation,
                });
            }
        }

        // Sort by wasted space (largest first)
        duplicates.sort_by(|a, b| b.total_wasted_space.cmp(&a.total_wasted_space));

        Ok(duplicates)
    }

    fn assess_conflict_level(&self, artifacts: &[BuildArtifact]) -> ConflictLevel {
        let mut has_different_types = false;
        let mut has_dependency_conflicts = false;

        // Check for different artifact types with same hash
        let first_type = &artifacts[0].artifact_type;
        for artifact in artifacts.iter().skip(1) {
            if artifact.artifact_type != *first_type {
                has_different_types = true;
                break;
            }
        }

        // Check for dependency conflicts
        let dep_sets: Vec<HashSet<_>> = artifacts
            .iter()
            .map(|a| a.dependencies.iter().cloned().collect())
            .collect();

        for i in 1..dep_sets.len() {
            if dep_sets[i] != dep_sets[0] {
                has_dependency_conflicts = true;
                break;
            }
        }

        if has_different_types && has_dependency_conflicts {
            ConflictLevel::Critical
        } else if has_different_types || has_dependency_conflicts {
            ConflictLevel::High
        } else if artifacts.len() > 3 {
            ConflictLevel::Medium
        } else {
            ConflictLevel::Low
        }
    }

    fn generate_cleanup_recommendation(
        &self,
        artifacts: &[BuildArtifact],
        conflict_level: ConflictLevel,
    ) -> String {
        match conflict_level {
            ConflictLevel::Critical => {
                "CRITICAL: Multiple artifact types with dependency conflicts detected. \
                 Manual review required before cleanup. Consider rebuilding from clean state."
                    .to_string()
            }
            ConflictLevel::High => {
                format!(
                    "HIGH PRIORITY: Found {} duplicate artifacts with conflicts. \
                        Recommend keeping the newest version and removing others.",
                    artifacts.len()
                )
            }
            ConflictLevel::Medium => {
                format!(
                    "MEDIUM: {} duplicate artifacts wasting {} bytes. \
                        Safe to remove all but one.",
                    artifacts.len(),
                    artifacts.iter().skip(1).map(|a| a.size).sum::<u64>()
                )
            }
            ConflictLevel::Low => {
                format!(
                    "LOW: {} duplicate artifacts. Automated cleanup recommended.",
                    artifacts.len()
                )
            }
            ConflictLevel::None => "No conflicts detected.".to_string(),
        }
    }

    /// Resolve conflicts and generate cleanup actions
    pub fn resolve_conflicts(
        &self,
        duplicates: &[DuplicateArtifact],
    ) -> Result<Vec<CleanupAction>> {
        let mut actions = Vec::new();

        for duplicate in duplicates {
            let resolution = self.conflict_resolver.resolve(duplicate)?;
            actions.extend(resolution);
        }

        Ok(actions)
    }

    /// Generate build optimization recommendations
    pub fn generate_build_recommendations(&self, duplicates: &[DuplicateArtifact]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let total_wasted = duplicates.iter().map(|d| d.total_wasted_space).sum::<u64>();
        if total_wasted > 1024 * 1024 * 100 {
            // 100MB
            recommendations.push(format!(
                "Clean {} MB of duplicate build artifacts to reduce disk usage",
                total_wasted / (1024 * 1024)
            ));
        }

        let critical_duplicates: Vec<_> = duplicates
            .iter()
            .filter(|d| matches!(d.conflict_level, ConflictLevel::Critical))
            .collect();

        if !critical_duplicates.is_empty() {
            recommendations.push(format!(
                "Address {} critical dependency conflicts in build artifacts",
                critical_duplicates.len()
            ));
        }

        // Check for build system specific optimizations
        if self.detect_cargo_build() {
            recommendations
                .push("Consider using 'cargo build --release' for optimized artifacts".to_string());
            recommendations
                .push("Enable incremental compilation with CARGO_INCREMENTAL=1".to_string());
        }

        recommendations
    }

    fn detect_cargo_build(&self) -> bool {
        Path::new("Cargo.toml").exists()
    }

    /// Execute automated cleanup
    pub fn execute_cleanup(&self, actions: &[CleanupAction]) -> Result<CleanupReport> {
        let mut report = CleanupReport::new();

        for action in actions {
            match action {
                CleanupAction::Remove { path, reason: _ } => match fs::remove_file(path) {
                    Ok(_) => {
                        report.files_removed.push(path.clone());
                        report.space_saved += self.get_file_size(path);
                    }
                    Err(e) => {
                        report
                            .errors
                            .push(format!("Failed to remove {}: {}", path.display(), e));
                    }
                },
                CleanupAction::Keep { .. } => {
                    // Nothing to do for keep actions
                }
            }
        }

        Ok(report)
    }

    fn get_file_size(&self, path: &Path) -> u64 {
        fs::metadata(path).map(|m| m.len()).unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
pub enum CleanupAction {
    Remove { path: PathBuf, reason: String },
    Keep { path: PathBuf, reason: String },
}

#[derive(Debug)]
pub struct CleanupReport {
    pub files_removed: Vec<PathBuf>,
    pub space_saved: u64,
    pub errors: Vec<String>,
}

impl CleanupReport {
    pub fn new() -> Self {
        Self {
            files_removed: Vec::new(),
            space_saved: 0,
            errors: Vec::new(),
        }
    }
}

impl Default for CleanupReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ConflictResolver {
    pub fn new() -> Self {
        Self {
            resolution_strategies: vec![
                ResolutionStrategy::KeepNewest,
                ResolutionStrategy::KeepLargest,
            ],
        }
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ConflictResolver {
    pub fn resolve(&self, duplicate: &DuplicateArtifact) -> Result<Vec<CleanupAction>> {
        let mut actions = Vec::new();

        match duplicate.conflict_level {
            ConflictLevel::Critical => {
                // For critical conflicts, keep all and flag for manual review
                for artifact in &duplicate.artifacts {
                    actions.push(CleanupAction::Keep {
                        path: artifact.path.clone(),
                        reason: "Critical conflict - manual review required".to_string(),
                    });
                }
            }
            _ => {
                // Apply resolution strategies
                let to_keep = self.select_artifacts_to_keep(&duplicate.artifacts);
                let to_remove: Vec<_> = duplicate
                    .artifacts
                    .iter()
                    .filter(|a| !to_keep.contains(&a.path))
                    .collect();

                for artifact in to_keep {
                    actions.push(CleanupAction::Keep {
                        path: artifact.clone(),
                        reason: "Selected by resolution strategy".to_string(),
                    });
                }

                for artifact in to_remove {
                    actions.push(CleanupAction::Remove {
                        path: artifact.path.clone(),
                        reason: format!(
                            "Duplicate of kept artifact, saving {} bytes",
                            artifact.size
                        ),
                    });
                }
            }
        }

        Ok(actions)
    }

    fn select_artifacts_to_keep(&self, artifacts: &[BuildArtifact]) -> Vec<PathBuf> {
        // Apply strategies in order
        for strategy in &self.resolution_strategies {
            match strategy {
                ResolutionStrategy::KeepNewest => {
                    if let Some(newest) = self.find_newest(artifacts) {
                        return vec![newest.path.clone()];
                    }
                }
                ResolutionStrategy::KeepLargest => {
                    if let Some(largest) = self.find_largest(artifacts) {
                        return vec![largest.path.clone()];
                    }
                }
                ResolutionStrategy::KeepByPath(pattern) => {
                    if let Some(matched) = artifacts
                        .iter()
                        .find(|a| a.path.to_string_lossy().contains(pattern))
                    {
                        return vec![matched.path.clone()];
                    }
                }
                ResolutionStrategy::Interactive => {
                    // For now, fall back to keep newest
                    continue;
                }
            }
        }

        // Default: keep first one
        artifacts
            .first()
            .map(|a| a.path.clone())
            .into_iter()
            .collect()
    }

    fn find_newest<'a>(&self, artifacts: &'a [BuildArtifact]) -> Option<&'a BuildArtifact> {
        artifacts.iter().max_by_key(|a| {
            a.path
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        })
    }

    fn find_largest<'a>(&self, artifacts: &'a [BuildArtifact]) -> Option<&'a BuildArtifact> {
        artifacts.iter().max_by_key(|a| a.size)
    }
}

impl Analyzer for BuildArtifactAnalyzer {
    fn name(&self) -> &str {
        "build_artifact_duplicate"
    }

    fn analyze(&self, _file_path: &Path, _content: &[u8]) -> Result<Vec<Finding>> {
        // This analyzer works on directories, not individual files
        // The actual analysis is done via scan_build_artifacts()
        Ok(Vec::new())
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Support build-related files and directories
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            matches!(
                file_name,
                "Cargo.toml" | "package.json" | "requirements.txt" | "pom.xml" | "build.gradle"
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_artifact_detection() {
        let analyzer = BuildArtifactAnalyzer::new();

        // Test various file types
        assert!(analyzer.is_build_artifact(Path::new("target/debug/binary")));
        assert!(analyzer.is_build_artifact(Path::new("libtest.so")));
        assert!(analyzer.is_build_artifact(Path::new("Cargo.toml")));
        assert!(!analyzer.is_build_artifact(Path::new("README.md")));
    }

    #[test]
    fn test_artifact_type_detection() {
        let analyzer = BuildArtifactAnalyzer::new();

        assert_eq!(
            analyzer.determine_artifact_type(Path::new("lib.so")),
            ArtifactType::SharedLibrary
        );
        assert_eq!(
            analyzer.determine_artifact_type(Path::new("lib.a")),
            ArtifactType::Archive
        );
        assert_eq!(
            analyzer.determine_artifact_type(Path::new("main.o")),
            ArtifactType::ObjectFile
        );
        assert_eq!(
            analyzer.determine_artifact_type(Path::new("app.exe")),
            ArtifactType::Binary
        );
    }

    #[test]
    fn test_conflict_assessment() {
        let analyzer = BuildArtifactAnalyzer::new();

        let artifacts = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
        ];

        let conflict_level = analyzer.assess_conflict_level(&artifacts);
        assert_eq!(conflict_level, ConflictLevel::Low);
    }

    #[test]
    fn test_duplicate_detection() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let mut analyzer = BuildArtifactAnalyzer::new();

        // Create test files with same content in target directory (so they're recognized as artifacts)
        let target_dir = temp_dir.path().join("target");
        fs::create_dir_all(&target_dir)?;
        let file1 = target_dir.join("file1.so");
        let file2 = target_dir.join("file2.so");
        fs::write(&file1, "test content")?;
        fs::write(&file2, "test content")?;

        // Create build artifacts
        let artifacts = vec![
            analyzer
                .analyze_file(&file1)?
                .ok_or_else(|| anyhow::anyhow!("Failed to analyze file1"))?,
            analyzer
                .analyze_file(&file2)?
                .ok_or_else(|| anyhow::anyhow!("Failed to analyze file2"))?,
        ];

        let duplicates = analyzer.group_artifacts_by_hash(artifacts)?;
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].artifacts.len(), 2);

        Ok(())
    }
}
