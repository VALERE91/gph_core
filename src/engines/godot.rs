use crate::engine::{GameEngine, ProjectInfo};
use crate::error::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
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
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            projects.push(ProjectInfo {
                name,
                path,
                engine_type: "Godot".to_string(),
            });
        }
        Ok(projects)
    }

    fn build_project(&self, project_info: &ProjectInfo) -> Result<()> {
        println!("Building Godot project: {}", &project_info.name);
        // Godot's build process is exporting. This function might not be needed
        // if packaging does it all. For now, we'll consider it a no-op.
        Ok(())
    }

    fn package_project(&self, project_info: &ProjectInfo, output_dir: &Path) -> Result<PathBuf> {
        println!("Packaging (exporting) Godot project: {}", &project_info.name);
        // TODO: Implement command execution for exporting.
        // You need an export_presets.cfg file in the project.
        // Example:
        // let output_path = output_dir.join(format!("{}.exe", &project_info.name));
        // Command::new(&self.executable_path)
        //     .arg("--path")
        //     .arg(project_info.path.to_str().unwrap())
        //     .arg("--export-release") // Or --export-debug
        //     .arg("\"Windows Desktop\"") // The name of the export preset
        //     .arg(output_path.to_str().unwrap())
        //     .arg("--headless") // Added in Godot 4
        //     .status()?;
        let packaged_path = output_dir.join(&project_info.name);
        Ok(packaged_path)
    }
}