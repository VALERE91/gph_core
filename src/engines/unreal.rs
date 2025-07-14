use crate::config::ProjectConfig;
use crate::engine::{GameEngine, ProjectInfo};
use crate::error::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub struct UnrealEngine {
    pub uat_path: PathBuf, // Path to RunUAT.bat or RunUAT.sh
}

impl UnrealEngine {
    pub fn new(path: &PathBuf) -> Self {
        let uat_path = if cfg!(windows) {
            path.join("Engine/Build/BatchFiles/RunUAT.bat")
        } else {
            path.join("Engine/Build/BatchFiles/RunUAT.sh")
        };
        
        Self {
            uat_path
        }
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
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            projects.push(ProjectInfo {
                name,
                path,
                engine_type: "Unreal".to_string(),
            });
        }
        Ok(projects)
    }

    fn build_project(&self, project_info: &ProjectInfo, _project_config: &ProjectConfig) -> Result<()> {
        println!("Building Unreal project: {}", &project_info.name);

        let uproject_path = &project_info.path;

        let platfrom = if cfg!(windows) {
            "-platform=Win64"
        } else {
            "-platform=Mac"
        };
        
        let status = Command::new(&self.uat_path)
            .arg("BuildCookRun")
            .arg(format!("-project={}", uproject_path.to_str().unwrap()))
            .arg("-build")
            .arg("-clientconfig=Development") // Or Shipping
            .arg(platfrom)
            .arg("-nocompileeditor")
            .arg("-unattended")
            .arg("-stdout")
            .arg("-utf8output")
            .status()
            .map_err(|e| Error::CommandExecution { engine: self.name().to_string(), source: e })?;

        if !status.success() {
            return Err(Error::BuildFailed { project_path: project_info.name.clone() });
        }

        Ok(())
    }

    fn package_project(
        &self,
        project_info: &ProjectInfo,
        _project_config: &ProjectConfig,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        println!("Packaging Unreal project: {}", &project_info.name);

        let uproject_path = &project_info.path;

        let status = Command::new(&self.uat_path)
            .arg("BuildCookRun")
            .arg(format!("-project={}", uproject_path.to_str().unwrap()))
            .arg("-archive")
            .arg(format!("-archivedirectory={}", output_dir.to_str().unwrap()))
            .arg("-package")
            .arg("-clientconfig=Development") // Or Shipping
            .arg("-platform=Win64") // Or Mac, Linux
            .arg("-nocompileeditor")
            .arg("-unattended")
            .arg("-stdout")
            .arg("-utf8output")
            .status()
            .map_err(|e| Error::CommandExecution { engine: self.name().to_string(), source: e })?;

        if !status.success() {
            return Err(Error::BuildFailed { project_path: project_info.name.clone() });
        }

        // The actual output path might be a subdirectory, but for now we return the target dir
        Ok(output_dir.to_path_buf())
    }
}