use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Global configuration for the user (e.g., in ~/.config/gph/config.toml)
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GlobalConfig {
    pub engine_paths: EnginePaths,
    pub auth_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EnginePaths {
    pub unreal: Option<PathBuf>,
    pub unity: Option<PathBuf>,
    pub godot: Option<PathBuf>,
}

impl GlobalConfig {
    /// Loads the global config from a specific path.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| crate::error::Error::Config(e.to_string()))
    }

    /// Saves the global config to a specific path, creating parent dirs if needed.
    pub fn save(&self, path: &Path) -> crate::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| crate::error::Error::Config(e.to_string()))?;
        fs::write(path, content)?;
        Ok(())
    }
}

/// Project-specific configuration (in <PROJECT_ROOT>/.gph/config.toml)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectConfig {
    pub project_id: Option<String>,
    pub team_id: Option<String>,
    pub engine_type: Option<String>,
    // Allow for overriding commands
    pub build_command: Option<String>,
    pub package_command: Option<String>,
}

impl ProjectConfig {
    pub fn load(project_root: &Path) -> crate::Result<Self> {
        let config_path = project_root.join(".gph").join("config.toml");
        let content = fs::read_to_string(config_path)?;
        toml::from_str(&content).map_err(|e| crate::error::Error::Config(e.to_string()))
    }

    pub fn save(&self, project_root: &Path) -> crate::Result<()> {
        let config_path = project_root.join(".gph").join("config.toml");
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| crate::error::Error::Config(e.to_string()))?;
        fs::write(config_path, content)?;
        Ok(())
    }
}