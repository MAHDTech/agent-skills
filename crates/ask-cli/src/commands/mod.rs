//! Subcommand handler modules and shared CLI utilities.

pub mod dashboard;
pub mod skills;
pub mod tui;

use skills_core::error::SkillError;
use std::path::PathBuf;

/// Structured error type for CLI operations, bridging domain, subprocess, and I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// Domain error originating from `skills-core`.
    #[error(transparent)]
    Skill(#[from] SkillError),

    /// External subprocess command failure.
    #[error("Subprocess command '{command}' failed with exit code {code:?}")]
    Subprocess {
        /// Executed command description.
        command: String,
        /// Child process exit code if available.
        code: Option<i32>,
    },

    /// Filesystem or standard I/O failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Resolves the repository root path via `AGENT_SKILLS_HOME` or ancestor traversal.
pub fn resolve_root() -> Result<PathBuf, SkillError> {
    if let Ok(env_path) = std::env::var("AGENT_SKILLS_HOME") {
        let path = PathBuf::from(env_path);
        if path.is_dir() {
            return Ok(path);
        }
    }

    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for ancestor in current.ancestors() {
        if ancestor.join("skills").is_dir() {
            return Ok(ancestor.to_path_buf());
        }
        let cargo_path = ancestor.join("Cargo.toml");
        if cargo_path.is_file() {
            if let Ok(manifest) = std::fs::read_to_string(&cargo_path) {
                if manifest.contains("[workspace]") {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }
    }

    Ok(PathBuf::from("."))
}
