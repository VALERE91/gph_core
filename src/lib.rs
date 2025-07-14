pub mod config;
pub mod engine;
pub mod engines;
pub mod error;
pub mod uploader;

use config::{GlobalConfig, ProjectConfig};
use engine::{GameEngine, ProjectInfo};
use error::{Error, Result};
use std::fs;
use std::path::Path;

/// The main entry point for managing projects.
/// Holds instances of all supported engine handlers.
pub struct ProjectManager {
    engines: Vec<Box<dyn GameEngine>>,
    global_config: GlobalConfig,
}

impl ProjectManager {
    /// Creates a new ProjectManager, initializing engine handlers based on config.
    pub fn new(global_config: GlobalConfig) -> Self {
        let mut engines: Vec<Box<dyn GameEngine>> = Vec::new();

        if let Some(path) = &global_config.engine_paths.unreal {
            if path.exists() {
                engines.push(Box::new(engines::unreal::UnrealEngine::new(
                    path.clone(),
                )));
            }
        }
        if let Some(path) = &global_config.engine_paths.unity {
            if path.exists() {
                engines.push(Box::new(engines::unity::UnityEngine::new(path.clone())));
            }
        }
        if let Some(path) = &global_config.engine_paths.godot {
            if path.exists() {
                engines.push(Box::new(engines::godot::GodotEngine::new(path.clone())));
            }
        }

        Self {
            engines,
            global_config,
        }
    }

    /// Initializes a project directory by creating a `.gph/config.toml` file.
    /// This corresponds to the `gph init` command.
    pub fn init_project(&self, project_dir: &Path) -> Result<ProjectConfig> {
        let gph_dir = project_dir.join(".gph");
        if gph_dir.exists() {
            return Err(Error::AlreadyInitialized {
                path: gph_dir,
            });
        }
        fs::create_dir(&gph_dir)?;

        // Try to auto-detect the engine to give a better default config
        let projects = self.find_all_projects(project_dir)?;
        let engine_type = projects.first().map(|p| p.engine_type.clone());

        let new_config = ProjectConfig {
            engine_type,
            ..Default::default()
        };

        new_config.save(project_dir)?;
        println!("Project initialized at {}", project_dir.display());
        Ok(new_config)
    }

    /// Detects all projects from all configured engines in a given directory.
    pub fn find_all_projects(&self, search_dir: &Path) -> Result<Vec<ProjectInfo>> {
        let mut all_projects = Vec::new();
        println!("Detecting projects in: {}", search_dir.display());
        for engine in &self.engines {
            println!("Checking for {} projects...", engine.name());
            match engine.detect_projects(search_dir) {
                Ok(mut projects) => {
                    if !projects.is_empty() {
                        println!("...found {} {} project(s).", projects.len(), engine.name());
                        all_projects.append(&mut projects);
                    }
                }
                Err(e) => {
                    // Log the error but don't stop detection for other engines
                    eprintln!("Could not detect {} projects: {}", engine.name(), e);
                }
            }
        }
        Ok(all_projects)
    }
}