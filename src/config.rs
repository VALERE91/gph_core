use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
pub struct EnginePaths {
    pub unreal: Option<PathBuf>,
    pub unity: Option<PathBuf>,
    pub godot: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub engine_paths: EnginePaths,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}