//! Core domain engine for Agent Skills.
// cspell:words Syncer
//!
//! Provides the foundational domain models, category definitions, frontmatter
//! parsing, linting primitives, error types, remote skill downloader, installer engine,
//! and multi-target skill synchronization engine.

pub mod downloader;
pub mod error;
pub mod installer;
pub mod lint;
pub mod models;
pub mod parser;
pub mod sync;

pub use downloader::SkillDownloader;
pub use error::{Result, SkillError};
pub use installer::{
    EnvironmentResolver, FileLockGuard, InstallMode, InstallOptions, InstallResult, InstalledSkill,
    InstalledSkillsRegistry, Installer, InstallerBuilder, InstallerError, IntegrityStatus,
    PathValidator, SkillFileEntry, SkillInstaller, SkillRecord, TargetEnvironment,
    UninstallOptions, UninstallResult, UpdateResult,
};
pub use lint::SkillLinter;
pub use models::*;
pub use parser::{MarkdownSection, ParsedSkill, SkillParser, TemplatePlaceholder};
pub use sync::{
    ConflictStrategy, SkillSyncer, SyncAction, SyncActionKind, SyncError, SyncPlan, SyncSummary,
};

/// Returns the current crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
