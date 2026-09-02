//! Core domain engine for Agent Skills.
//!
//! Provides the foundational domain models, category definitions, frontmatter
//! parsing, linting primitives, error types, and synchronization models.

pub mod error;
pub mod models;

pub use error::{Result, SkillError};
pub use models::*;

/// Returns the current crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
