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

    fn build_project(&self, project_info: &ProjectInfo) -> Result<()> {
        println!("Building Unity project: {}", &project_info.name);
        // TODO: Implement command execution with Unity's command line arguments.
        // Example:
        // Command::new(&self.executable_path)
        //     .arg("-batchmode")
        //     .arg("-quit")
        //     .arg("-projectPath")
        //     .arg(project_info.path.to_str().unwrap())
        //     .arg("-executeMethod")
        //     .arg("MyEditorScript.PerformBuild") // You need a C# script in Unity to trigger the build
        //     .status()?;
        Ok(())
    }

    fn package_project(&self, project_info: &ProjectInfo, output_dir: &Path) -> Result<PathBuf> {
        println!("Packaging Unity project: {}", &project_info.name);
        // Packaging in Unity is typically handled by the same build script.
        // This function might just confirm the output exists.
        let packaged_path = output_dir.join(&project_info.name);
        Ok(packaged_path)
    }
}