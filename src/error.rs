use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration file error: {0}")]
    Config(String),

    #[error("Failed to execute command for {engine}: {source}")]
    CommandExecution {
        engine: String,
        #[source]
        source: io::Error,
    },

    #[error("Build process failed for project: {project_path}")]
    BuildFailed { project_path: String },

    #[error("Project detection failed for engine {engine_name}")]
    DetectionFailed { engine_name: String },

    #[error("Engine executable not found at path: {path}")]
    EngineNotFound { path: PathBuf },

    #[error("Project already initialized at {path}")]
    AlreadyInitialized { path: PathBuf },
}

pub type Result<T> = anyhow::Result<T, Error>;