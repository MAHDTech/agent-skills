//! Custom error types for `skills-core`.

use std::path::PathBuf;
use thiserror::Error;

/// Crate-level Result type alias.
pub type Result<T> = std::result::Result<T, SkillError>;

/// Master error enum for all `skills-core` operations.
#[derive(Debug, Error)]
pub enum SkillError {
    /// File or directory I/O failure with path context.
    #[error("I/O error at '{path}': {source}")]
    Io {
        /// Target path where the I/O error occurred.
        path: PathBuf,
        /// Underlying `std::io::Error`.
        #[source]
        source: std::io::Error,
    },

    /// General I/O failure without explicit path context.
    #[error("General I/O error: {0}")]
    GeneralIo(#[from] std::io::Error),

    /// YAML serialization or deserialization failure with path context.
    #[error("YAML syntax error in '{path}': {source}")]
    Yaml {
        /// File path containing the invalid YAML.
        path: PathBuf,
        /// Underlying `serde_yaml::Error`.
        #[source]
        source: serde_yaml::Error,
    },

    /// General YAML error without explicit path context.
    #[error("General YAML error: {0}")]
    GeneralYaml(#[from] serde_yaml::Error),

    /// JSON serialization or deserialization failure with path context.
    #[error("JSON error in '{path}': {source}")]
    Json {
        /// File path containing the invalid JSON.
        path: PathBuf,
        /// Underlying `serde_json::Error`.
        #[source]
        source: serde_json::Error,
    },

    /// General JSON error without explicit path context.
    #[error("General JSON error: {0}")]
    GeneralJson(#[from] serde_json::Error),

    /// Frontmatter validation failure.
    #[error("Frontmatter validation error in '{path}': {message}")]
    FrontmatterValidation {
        /// File path with frontmatter failure.
        path: PathBuf,
        /// Detailed description of the validation failure.
        message: String,
    },

    /// Invalid skill name format or illegal token.
    #[error("Invalid skill name '{name}': {reason}")]
    InvalidSkillName {
        /// Invalid skill name string.
        name: String,
        /// Reason why the name is invalid.
        reason: String,
    },

    /// Unknown or unsupported skill category.
    #[error("Unknown category '{category}' for skill at '{path}'")]
    UnknownCategory {
        /// Unrecognized category string.
        category: String,
        /// File path where the category was encountered.
        path: PathBuf,
    },

    /// Markdown AST or parsing failure.
    #[error("Markdown parsing error in '{path}': {message}")]
    MarkdownParse {
        /// Path to the markdown document.
        path: PathBuf,
        /// Detailed parse failure explanation.
        message: String,
    },

    /// Lint failure aggregating rule violations.
    #[error("Lint validation failed with {count} error(s):\n{details}")]
    Lint {
        /// Number of error-level lint issues.
        count: usize,
        /// Formatted details string.
        details: String,
    },

    /// Symlink creation or resolution failure.
    #[error("Symlink error targeting '{target}' from '{destination}': {message}")]
    Symlink {
        /// Target destination pointed to by the symlink.
        target: PathBuf,
        /// Symlink file location on disk.
        destination: PathBuf,
        /// Detailed failure message.
        message: String,
    },

    /// Lock acquisition timeout.
    #[error("Lock acquisition timed out for '{path}' after {timeout_secs} seconds")]
    LockTimeout {
        /// Path to the locked resource.
        path: PathBuf,
        /// Number of seconds waited before timing out.
        timeout_secs: u64,
    },

    /// Path traversal security violation.
    #[error("Path traversal detected: '{path}' is outside base directory '{base}'")]
    PathTraversal {
        /// Attempted path escaping the base boundary.
        path: PathBuf,
        /// Root base directory.
        base: PathBuf,
    },

    /// Network error during remote fetch or download.
    #[error("Network error fetching '{url}': {message}")]
    Network {
        /// Request URL.
        url: String,
        /// Network failure explanation.
        message: String,
    },

    /// Query or skill resolution miss.
    #[error("Skill not found for query '{query}'")]
    NotFound {
        /// Search or query term.
        query: String,
    },

    /// Duplicate skill names in multiple category folders.
    #[error("Duplicate skill name '{name}' detected in categories: {paths:?}")]
    DuplicateSkill {
        /// Conflicting skill identifier.
        name: String,
        /// Discovered file paths sharing the identifier.
        paths: Vec<PathBuf>,
    },
}

impl SkillError {
    /// Constructs a [`SkillError::Io`] with path context.
    #[must_use]
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    /// Constructs a [`SkillError::Yaml`] with path context.
    #[must_use]
    pub fn yaml(path: impl Into<PathBuf>, source: serde_yaml::Error) -> Self {
        Self::Yaml {
            path: path.into(),
            source,
        }
    }

    /// Constructs a [`SkillError::FrontmatterValidation`] error.
    #[must_use]
    pub fn validation(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::FrontmatterValidation {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Constructs a [`SkillError::Lint`] error.
    #[must_use]
    pub fn lint(count: usize, details: impl Into<String>) -> Self {
        Self::Lint {
            count,
            details: details.into(),
        }
    }
}
