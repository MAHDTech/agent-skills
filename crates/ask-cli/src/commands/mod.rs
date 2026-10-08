//! Subcommand handler modules and shared CLI utilities.

pub mod dashboard;
pub mod skills;
pub mod tui;

use skills_core::error::SkillError;
use std::path::{Path, PathBuf};

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
    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let env_val = std::env::var("AGENT_SKILLS_HOME").ok();
    Ok(resolve_root_from(&current, env_val.as_deref()))
}

/// Resolves root given a start directory and optional environment variable override.
pub fn resolve_root_from(start_dir: &Path, env_home: Option<&str>) -> PathBuf {
    if let Some(env_path) = env_home {
        let path = PathBuf::from(env_path);
        if path.is_dir() {
            return path;
        }
    }

    for ancestor in start_dir.ancestors() {
        if ancestor.join("skills").is_dir() {
            return ancestor.to_path_buf();
        }
        let cargo_path = ancestor.join("Cargo.toml");
        if cargo_path.is_file() {
            if let Ok(manifest) = std::fs::read_to_string(&cargo_path) {
                if manifest.contains("[workspace]") {
                    return ancestor.to_path_buf();
                }
            }
        }
    }

    PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_root_from_env_var_valid() {
        let temp = tempdir().unwrap();
        let expected = temp.path().to_path_buf();
        let res = resolve_root_from(Path::new("/nonexistent"), Some(expected.to_str().unwrap()));
        assert_eq!(res, expected);
    }

    #[test]
    fn test_resolve_root_from_env_var_nonexistent_falls_through() {
        let temp = tempdir().unwrap();
        let skills_dir = temp.path().join("skills");
        std::fs::create_dir_all(&skills_dir).unwrap();
        let sub = temp.path().join("a/b/c");
        std::fs::create_dir_all(&sub).unwrap();

        let res = resolve_root_from(&sub, Some("/nonexistent/path/for/env"));
        assert_eq!(res, temp.path().to_path_buf());
    }

    #[test]
    fn test_resolve_root_from_ancestor_skills_dir() {
        let temp = tempdir().unwrap();
        let skills_dir = temp.path().join("skills");
        std::fs::create_dir_all(&skills_dir).unwrap();
        let sub = temp.path().join("sub/dir");
        std::fs::create_dir_all(&sub).unwrap();

        let res = resolve_root_from(&sub, None);
        assert_eq!(res, temp.path().to_path_buf());
    }

    #[test]
    fn test_resolve_root_from_ancestor_cargo_workspace() {
        let temp = tempdir().unwrap();
        let cargo_path = temp.path().join("Cargo.toml");
        std::fs::write(&cargo_path, "[workspace]\nmembers = []\n").unwrap();
        let sub = temp.path().join("crate/src");
        std::fs::create_dir_all(&sub).unwrap();

        let res = resolve_root_from(&sub, None);
        assert_eq!(res, temp.path().to_path_buf());
    }

    #[test]
    fn test_resolve_root_from_fallback() {
        let temp = tempdir().unwrap();
        let isolated = temp.path().join("empty");
        std::fs::create_dir_all(&isolated).unwrap();

        let res = resolve_root_from(&isolated, None);
        assert_eq!(res, PathBuf::from("."));
    }

    #[test]
    fn test_resolve_root_current() {
        let res = resolve_root();
        assert!(res.is_ok());
    }
}
