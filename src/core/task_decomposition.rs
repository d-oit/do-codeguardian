//! Task decomposition logic for breaking down complex analysis tasks into parallel subtasks

use crate::core::swarm_types::{Priority, SwarmConfig, SwarmError, SwarmTask};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Strategy for decomposing analysis tasks
#[derive(Debug, Clone)]
pub enum DecompositionStrategy {
    /// Decompose by file type and size
    FileBased,
    /// Decompose by directory structure
    DirectoryBased,
    /// Decompose by analysis type (security, performance, quality)
    AnalysisTypeBased,
    /// Decompose by code complexity metrics
    ComplexityBased,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Task decomposer for breaking down complex analysis tasks
pub struct TaskDecomposer {
    config: SwarmConfig,
    strategy: DecompositionStrategy,
    agent_capabilities: HashMap<String, Vec<String>>, // agent_type -> supported_file_types
}

impl TaskDecomposer {
    pub fn new(config: SwarmConfig, strategy: DecompositionStrategy) -> Self {
        Self {
            config,
            strategy,
            agent_capabilities: Self::load_agent_capabilities(),
        }
    }

    /// Load agent capabilities from configuration or registry
    fn load_agent_capabilities() -> HashMap<String, Vec<String>> {
        // This would typically be loaded from a configuration file or agent registry
        // For now, we'll define some common agent types and their capabilities
        let mut capabilities = HashMap::new();

        capabilities.insert(
            "security_analyzer".to_string(),
            vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "java".to_string(),
                "go".to_string(),
                "cpp".to_string(),
                "c".to_string(),
            ],
        );

        capabilities.insert(
            "performance_analyzer".to_string(),
            vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "java".to_string(),
                "go".to_string(),
            ],
        );

        capabilities.insert(
            "quality_analyzer".to_string(),
            vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "java".to_string(),
                "go".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "md".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "json".to_string(),
            ],
        );

        capabilities.insert(
            "dependency_analyzer".to_string(),
            vec![
                "toml".to_string(),
                "json".to_string(),
                "xml".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
            ],
        );

        capabilities
    }

    /// Decompose an analysis task into multiple parallel subtasks
    pub async fn decompose_task(
        &self,
        analysis_request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        match self.strategy {
            DecompositionStrategy::FileBased => self.decompose_by_files(analysis_request).await,
            DecompositionStrategy::DirectoryBased => {
                self.decompose_by_directories(analysis_request).await
            }
            DecompositionStrategy::AnalysisTypeBased => {
                self.decompose_by_analysis_type(analysis_request).await
            }
            DecompositionStrategy::ComplexityBased => {
                self.decompose_by_complexity(analysis_request).await
            }
            DecompositionStrategy::Hybrid => self.decompose_hybrid(analysis_request).await,
        }
    }

    /// Decompose task by grouping files of similar types and sizes
    async fn decompose_by_files(
        &self,
        request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        let mut tasks = Vec::new();
        let mut file_groups = HashMap::new();

        // Group files by extension
        for file_path in &request.target_files {
            if let Some(ext) = Path::new(file_path).extension() {
                let ext_str = ext.to_string_lossy().to_string();
                file_groups
                    .entry(ext_str)
                    .or_insert_with(Vec::new)
                    .push(file_path.clone());
            }
        }

        // Create tasks for each file group
        for (file_type, files) in file_groups {
            if files.len() <= self.config.max_concurrent_tasks {
                // Small group - create one task per file
                for file in files {
                    let task = self.create_file_task(&file_type, vec![file], request)?;
                    tasks.push(task);
                }
            } else {
                // Large group - split into chunks
                let chunks = self.chunk_files(files, self.config.max_concurrent_tasks);
                for chunk in chunks {
                    let task = self.create_file_task(&file_type, chunk, request)?;
                    tasks.push(task);
                }
            }
        }

        Ok(tasks)
    }

    /// Decompose task by directory structure
    async fn decompose_by_directories(
        &self,
        request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        let mut tasks = Vec::new();
        let mut dir_groups = HashMap::new();

        // Group files by their parent directory
        for file_path in &request.target_files {
            let path = Path::new(file_path);
            if let Some(parent) = path.parent() {
                let dir_str = parent.to_string_lossy().to_string();
                dir_groups
                    .entry(dir_str)
                    .or_insert_with(Vec::new)
                    .push(file_path.clone());
            }
        }

        // Create tasks for each directory
        for (dir, files) in dir_groups {
            let task = self.create_directory_task(&dir, files, request)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Decompose task by analysis type (security, performance, quality)
    async fn decompose_by_analysis_type(
        &self,
        request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        let mut tasks = Vec::new();

        // Create separate tasks for each analysis type
        let analysis_types = vec![
            "security_analyzer",
            "performance_analyzer",
            "quality_analyzer",
            "dependency_analyzer",
        ];

        for analysis_type in analysis_types {
            if self.should_create_task_for_type(analysis_type, &request.target_files) {
                let task = self.create_analysis_type_task(analysis_type, request)?;
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    /// Decompose task based on code complexity metrics
    async fn decompose_by_complexity(
        &self,
        request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        let mut simple_files = Vec::new();
        let mut complex_files = Vec::new();

        // Analyze file complexity (simplified - in practice, this would use actual metrics)
        for file_path in &request.target_files {
            let complexity = self.estimate_complexity(file_path).await?;
            if complexity > 100 {
                // Arbitrary threshold
                complex_files.push(file_path.clone());
            } else {
                simple_files.push(file_path.clone());
            }
        }

        let mut tasks = Vec::new();

        // Create separate tasks for simple and complex files
        if !simple_files.is_empty() {
            let task = self.create_complexity_task("simple", simple_files, request)?;
            tasks.push(task);
        }

        if !complex_files.is_empty() {
            let task = self.create_complexity_task("complex", complex_files, request)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Hybrid decomposition combining multiple strategies
    async fn decompose_hybrid(
        &self,
        request: &AnalysisRequest,
    ) -> Result<Vec<SwarmTask>, SwarmError> {
        // Start with file-based decomposition
        let file_tasks = self.decompose_by_files(request).await?;

        // Then apply analysis type decomposition
        let mut hybrid_tasks = Vec::new();

        for file_task in file_tasks {
            // For each file-based task, create sub-tasks for different analysis types
            let analysis_types = self.get_relevant_analysis_types(&file_task.target_files);

            for analysis_type in analysis_types {
                let mut analysis_task = file_task.clone();
                analysis_task.agent_type = analysis_type.to_string();
                analysis_task.id = format!("{}_{}", file_task.id, analysis_type);
                hybrid_tasks.push(analysis_task);
            }
        }

        Ok(hybrid_tasks)
    }

    /// Create a task for processing specific files
    fn create_file_task(
        &self,
        file_type: &str,
        files: Vec<String>,
        request: &AnalysisRequest,
    ) -> Result<SwarmTask, SwarmError> {
        let agent_type = self.get_agent_for_file_type(file_type)?;

        Ok(SwarmTask {
            id: format!(
                "file_task_{}_{}",
                file_type,
                uuid::Uuid::new_v4().as_simple()
            ),
            agent_type: agent_type.to_string(),
            priority: self.calculate_priority(&files),
            target_files: files,
            parameters: request.parameters.clone(),
            timeout: self.config.task_timeout,
            dependencies: Vec::new(),
        })
    }

    /// Create a task for processing a directory
    fn create_directory_task(
        &self,
        dir: &str,
        files: Vec<String>,
        request: &AnalysisRequest,
    ) -> Result<SwarmTask, SwarmError> {
        Ok(SwarmTask {
            id: format!(
                "dir_task_{}_{}",
                dir.replace('/', "_"),
                uuid::Uuid::new_v4().as_simple()
            ),
            agent_type: "multi_analyzer".to_string(), // Generic agent for directory analysis
            priority: Priority::Medium,
            target_files: files,
            parameters: request.parameters.clone(),
            timeout: self.config.task_timeout,
            dependencies: Vec::new(),
        })
    }

    /// Create a task for a specific analysis type
    fn create_analysis_type_task(
        &self,
        analysis_type: &str,
        request: &AnalysisRequest,
    ) -> Result<SwarmTask, SwarmError> {
        // Filter files that this analysis type can handle
        let supported_files = self.filter_supported_files(analysis_type, &request.target_files);

        Ok(SwarmTask {
            id: format!(
                "{}_task_{}",
                analysis_type,
                uuid::Uuid::new_v4().as_simple()
            ),
            agent_type: analysis_type.to_string(),
            priority: self.get_analysis_type_priority(analysis_type),
            target_files: supported_files,
            parameters: request.parameters.clone(),
            timeout: self.config.task_timeout,
            dependencies: Vec::new(),
        })
    }

    /// Create a task based on complexity
    fn create_complexity_task(
        &self,
        complexity_level: &str,
        files: Vec<String>,
        request: &AnalysisRequest,
    ) -> Result<SwarmTask, SwarmError> {
        Ok(SwarmTask {
            id: format!(
                "{}_task_{}",
                complexity_level,
                uuid::Uuid::new_v4().as_simple()
            ),
            agent_type: "complexity_analyzer".to_string(),
            priority: if complexity_level == "complex" {
                Priority::High
            } else {
                Priority::Medium
            },
            target_files: files,
            parameters: request.parameters.clone(),
            timeout: self.config.task_timeout,
            dependencies: Vec::new(),
        })
    }

    /// Get the appropriate agent for a file type
    fn get_agent_for_file_type(&self, file_type: &str) -> Result<&str, SwarmError> {
        // Find the first agent that supports this file type
        for (agent_type, supported_types) in &self.agent_capabilities {
            if supported_types.contains(&file_type.to_string()) {
                return Ok(agent_type);
            }
        }

        Err(SwarmError::ConfigurationError(format!(
            "No agent found for file type: {}",
            file_type
        )))
    }

    /// Calculate priority based on file characteristics
    fn calculate_priority(&self, files: &[String]) -> Priority {
        // Simple priority calculation based on file count and types
        if files.len() > 100 {
            Priority::High
        } else if files.len() > 50 {
            Priority::Medium
        } else {
            Priority::Low
        }
    }

    /// Get priority for analysis type
    fn get_analysis_type_priority(&self, analysis_type: &str) -> Priority {
        match analysis_type {
            "security_analyzer" => Priority::Critical,
            "performance_analyzer" => Priority::High,
            "quality_analyzer" => Priority::Medium,
            "dependency_analyzer" => Priority::Medium,
            _ => Priority::Low,
        }
    }

    /// Check if we should create a task for this analysis type
    fn should_create_task_for_type(&self, analysis_type: &str, files: &[String]) -> bool {
        let supported_files = self.filter_supported_files(analysis_type, files);
        !supported_files.is_empty()
    }

    /// Filter files that are supported by the given analysis type
    fn filter_supported_files(&self, analysis_type: &str, files: &[String]) -> Vec<String> {
        if let Some(supported_types) = self.agent_capabilities.get(analysis_type) {
            files
                .iter()
                .filter(|file| {
                    if let Some(ext) = Path::new(file).extension() {
                        supported_types.contains(&ext.to_string_lossy().to_string())
                    } else {
                        false
                    }
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get relevant analysis types for a set of files
    fn get_relevant_analysis_types(&self, files: &[String]) -> Vec<String> {
        let mut relevant_types = HashSet::new();

        for file in files {
            if let Some(ext) = Path::new(file).extension() {
                let ext_str = ext.to_string_lossy().to_string();
                for (agent_type, supported_types) in &self.agent_capabilities {
                    if supported_types.contains(&ext_str) {
                        relevant_types.insert(agent_type.clone());
                    }
                }
            }
        }

        relevant_types.into_iter().collect()
    }

    /// Split files into chunks for parallel processing
    fn chunk_files(&self, files: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
        files
            .chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Estimate complexity of a file (simplified implementation)
    async fn estimate_complexity(&self, file_path: &str) -> Result<usize, SwarmError> {
        // In a real implementation, this would analyze the file for:
        // - Lines of code
        // - Cyclomatic complexity
        // - Number of functions/methods
        // - Code duplication metrics
        // For now, we'll use a simple heuristic based on file size

        match tokio::fs::metadata(file_path).await {
            Ok(metadata) => {
                let size_kb = metadata.len() / 1024;
                Ok(size_kb as usize) // Simple complexity estimate
            }
            Err(e) => Err(SwarmError::InternalError(format!(
                "Failed to read file metadata for {}: {}",
                file_path, e
            ))),
        }
    }
}

/// Request for analysis task decomposition
#[derive(Debug, Clone)]
pub struct AnalysisRequest {
    pub target_files: Vec<String>,
    pub parameters: HashMap<String, String>,
    pub analysis_types: Vec<String>,
    pub priority: Priority,
}
