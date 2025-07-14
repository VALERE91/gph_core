use crate::engine::{GameEngine, ProjectInfo};
use crate::error::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub struct UnrealEngine {
    pub uat_path: PathBuf, // Path to RunUAT.bat or RunUAT.sh
}

impl UnrealEngine {
    pub fn new(uat_path: PathBuf) -> Self {
        Self { uat_path }
    }
}

impl GameEngine for UnrealEngine {
    fn name(&self) -> &str {
        "Unreal Engine"
    }

    fn detect_projects(&self, search_dir: &Path) -> Result<Vec<ProjectInfo>> {
        let mut projects = Vec::new();
        for entry in WalkDir::new(search_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".uproject"))
        {
            let path = entry.path().to_path_buf();
            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            projects.push(ProjectInfo {
                name,
                path,
                engine_type: "Unreal".to_string(),
            });
        }
        Ok(projects)
    }

    fn build_project(&self, project_info: &ProjectInfo) -> Result<()> {
        println!("Building Unreal project: {}", &project_info.name);
        // TODO: Implement the command execution logic.
        // Example:
        // let status = Command::new(&self.uat_path)
        //     .arg("BuildCookRun")
        //     .arg(format!("-project={}", project_info.path.to_str().unwrap()))
        //     .arg("-build")
        //     // ... other necessary args
        //     .status()
        //     .map_err(|e| Error::CommandExecution { engine: self.name().to_string(), source: e })?;
        //
        // if !status.success() {
        //     return Err(Error::BuildFailed { project_path: project_info.name.clone() });
        // }
        Ok(())
    }

    fn package_project(&self, project_info: &ProjectInfo, output_dir: &Path) -> Result<PathBuf> {
        println!("Packaging Unreal project: {}", &project_info.name);
        // TODO: Implement the command execution logic for packaging.
        // This is often part of the BuildCookRun command.
        let packaged_path = output_dir.join(&project_info.name);
        Ok(packaged_path)
    }
}
