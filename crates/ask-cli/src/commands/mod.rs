//! Subcommand handler modules and shared CLI utilities.

pub mod dashboard;
pub mod skills;
pub mod tui;

use skills_core::error::SkillError;
use std::path::PathBuf;

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
        if ancestor.join("skills").is_dir() || ancestor.join("Cargo.toml").is_file() {
            return Ok(ancestor.to_path_buf());
        }
    }

    Ok(PathBuf::from("."))
}
