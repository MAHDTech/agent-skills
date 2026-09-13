//! Core domain engine for Agent Skills.
// cspell:words Syncer
//!
//! Provides the foundational domain models, category definitions, frontmatter
//! parsing, linting primitives, error types, remote skill downloader, installer engine,
//! multi-target skill synchronization engine, repository artifact generation, and dashboard telemetry aggregation.

pub mod artifacts;
pub mod dashboard;
pub mod downloader;
pub mod error;
pub mod installer;
pub mod lint;
pub mod models;
pub mod parser;
pub mod sync;

pub use artifacts::{
    escape_zola_shortcodes, rewrite_skill_links, strip_legacy_raw_wrapper, sync_resources,
    ArtifactsEngine, ArtifactsGenerator, ArtifactsOptions, ArtifactsSummary,
};
pub use dashboard::{
    calculate_category_distributions, calculate_health_score, calculate_prompt_tokens,
    calculate_skill_metric, calculate_target_distributions,
    calculate_target_distributions_with_root, calculate_total_tokens, CategoryDistribution,
    DashboardEngine, DashboardSummary, HealthScore, SkillMetric, TargetDistribution,
};
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
