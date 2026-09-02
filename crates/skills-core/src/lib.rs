//! Core domain engine for Agent Skills.
//!
//! Provides the foundational domain models, category definitions, frontmatter
//! parsing, linting primitives, error types, remote skill downloader, and synchronization models.

pub mod downloader;
pub mod error;
pub mod lint;
pub mod models;
pub mod parser;

pub use downloader::SkillDownloader;
pub use error::{Result, SkillError};
pub use lint::SkillLinter;
pub use models::*;
pub use parser::{MarkdownSection, ParsedSkill, SkillParser, TemplatePlaceholder};

/// Returns the current crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
