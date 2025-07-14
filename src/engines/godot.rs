use crate::config::ProjectConfig;
use crate::engine::{GameEngine, ProjectInfo};
use crate::error::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct GodotEngine {
    pub executable_path: PathBuf,
}

impl GodotEngine {
    pub fn new(executable_path: PathBuf) -> Self {
        Self { executable_path }
    }
}

impl GameEngine for GodotEngine {
    fn name(&self) -> &str {
        "Godot"
    }

    fn detect_projects(&self, search_dir: &Path) -> Result<Vec<ProjectInfo>> {
        let mut projects = Vec::new();
        for entry in WalkDir::new(search_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy() == "project.godot")
        {
            let path = entry.path().parent().unwrap().to_path_buf();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            projects.push(ProjectInfo {
                name,
                path,
                engine_type: "Godot".to_string(),
            });
        }
        Ok(projects)
    }

    fn build_project(&self, project_info: &ProjectInfo, _project_config: &ProjectConfig) -> Result<()> {
        println!("Building Godot project: {}", &project_info.name);
        Ok(())
    }

    fn package_project(
        &self,
        project_info: &ProjectInfo,
        _project_config: &ProjectConfig,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        println!("Packaging (exporting) Godot project: {}", &project_info.name);
        // TODO: Use project_config to allow for custom package commands.
        let packaged_path = output_dir.join(&project_info.name);
        Ok(packaged_path)
    }
}