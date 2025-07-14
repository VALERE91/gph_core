use crate::config::ProjectConfig;
use crate::engine::{GameEngine, ProjectInfo};
use crate::error::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub struct UnityEngine {
    pub executable_path: PathBuf,
}

impl UnityEngine {
    pub fn new(executable_path: PathBuf) -> Self {
        Self { executable_path }
    }
}

impl GameEngine for UnityEngine {
    fn name(&self) -> &str {
        "Unity"
    }

    fn detect_projects(&self, search_dir: &Path) -> Result<Vec<ProjectInfo>> {
        let mut projects = Vec::new();
        // Unity projects are directories containing an 'Assets' subdirectory.
        for entry in WalkDir::new(search_dir).min_depth(1).max_depth(3).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() && entry.path().join("Assets").is_dir() && entry.path().join("ProjectSettings").is_dir() {
                let path = entry.path().to_path_buf();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                projects.push(ProjectInfo {
                    name,
                    path,
                    engine_type: "Unity".to_string(),
                });
            }
        }
        Ok(projects)
    }

    fn build_project(&self, project_info: &ProjectInfo, _project_config: &ProjectConfig) -> Result<()> {
        println!("Building Unity project: {}", &project_info.name);
        // TODO: Use project_config to allow for custom build commands.
        Ok(())
    }

    fn package_project(
        &self,
        project_info: &ProjectInfo,
        _project_config: &ProjectConfig,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        println!("Packaging Unity project: {}", &project_info.name);
        // TODO: Use project_config to allow for custom package commands.
        let packaged_path = output_dir.join(&project_info.name);
        Ok(packaged_path)
    }
}