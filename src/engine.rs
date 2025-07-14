use crate::config::ProjectConfig;
use crate::error::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub path: PathBuf,
    pub engine_type: String, // e.g., "Unreal", "Unity", "Godot"
}

/// A trait defining the common interface for all supported game engines.
pub trait GameEngine: Send + Sync {
    /// Returns the display name of the engine.
    fn name(&self) -> &str;

    /// Detects all valid projects for this engine in a given directory.
    fn detect_projects(&self, search_dir: &Path) -> Result<Vec<ProjectInfo>>;

    /// Executes the build command for a given project.
    fn build_project(&self, project_info: &ProjectInfo, project_config: &ProjectConfig) -> Result<()>;

    /// Packages the project for distribution.
    fn package_project(
        &self,
        project_info: &ProjectInfo,
        project_config: &ProjectConfig,
        output_dir: &Path,
    ) -> Result<PathBuf>;
}