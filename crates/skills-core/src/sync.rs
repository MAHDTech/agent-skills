//! Multi-target skill synchronization engine.
//!
//! Provides two-phase reconciliation between local skill catalogs (source of truth)
//! and multiple target agent execution environments (Claude Desktop, Cursor, Antigravity, Custom).
//! Supports side-effect-free drift planning, SemVer-aware upgrade/conflict detection,
//! orphan pruning, atomic rollouts with automated backups, and dry-run guarantees.
// cspell:words Syncer syncer

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::SkillError;
use crate::installer::{
    collect_file_entries, compute_dir_checksum, default_exclude_patterns, is_excluded,
    timestamp_now_iso, InstallMode, InstallOptions, InstalledSkill, Installer, InstallerError,
    IntegrityStatus, PathValidator, TargetEnvironment, UninstallOptions,
};
use crate::models::SkillFrontmatter;
use crate::parser::SkillParser;

// -----------------------------------------------------------------------------
// Domain Types & Enums
// -----------------------------------------------------------------------------

/// Discrete action classification determined by catalog-target drift analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SyncActionKind {
    /// Skill is present in catalog but missing from the target environment.
    Install,
    /// Skill exists in target, but source catalog has a newer version or modified checksum.
    Update,
    /// Skill is present in target but absent from catalog (orphan pruning candidate).
    Delete,
    /// Discrepancy detected requiring explicit resolution (downgrade hazard or target tampering).
    Conflict,
    /// Source and target are identical in version, checksum, and structure (no action needed).
    NoOp,
}

/// Proposed reconciliation action for a specific skill in a target environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncAction {
    /// Unique identifier of the skill (e.g. "github-pr-reviewer").
    pub skill_id: String,

    /// Target execution environment where the action will be applied.
    pub target_env: TargetEnvironment,

    /// Type of synchronization action to execute.
    pub kind: SyncActionKind,

    /// Version present in local catalog (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,

    /// Version currently installed in target environment (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,

    /// Composite SHA-256 checksum of source catalog files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_checksum: Option<String>,

    /// Composite SHA-256 checksum of installed target files recorded in registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_checksum: Option<String>,

    /// Diagnostic explanation detailing why this action was selected.
    pub reason: String,
}

/// A queryable batch of planned reconciliation actions across one or more targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPlan {
    /// Ordered list of planned reconciliation actions.
    pub actions: Vec<SyncAction>,

    /// ISO-8601 UTC timestamp of plan generation.
    pub created_at: String,
}

impl SyncPlan {
    /// Creates a new synchronization plan.
    #[must_use]
    pub fn new(actions: Vec<SyncAction>) -> Self {
        Self {
            actions,
            created_at: timestamp_now_iso(),
        }
    }

    /// Returns an iterator over all `Install` actions.
    pub fn installs(&self) -> impl Iterator<Item = &SyncAction> {
        self.actions
            .iter()
            .filter(|a| a.kind == SyncActionKind::Install)
    }

    /// Returns an iterator over all `Update` actions.
    pub fn updates(&self) -> impl Iterator<Item = &SyncAction> {
        self.actions
            .iter()
            .filter(|a| a.kind == SyncActionKind::Update)
    }

    /// Returns an iterator over all `Delete` actions.
    pub fn deletes(&self) -> impl Iterator<Item = &SyncAction> {
        self.actions
            .iter()
            .filter(|a| a.kind == SyncActionKind::Delete)
    }

    /// Returns an iterator over all `Conflict` actions.
    pub fn conflicts(&self) -> impl Iterator<Item = &SyncAction> {
        self.actions
            .iter()
            .filter(|a| a.kind == SyncActionKind::Conflict)
    }

    /// Returns an iterator over all `NoOp` actions.
    pub fn no_ops(&self) -> impl Iterator<Item = &SyncAction> {
        self.actions
            .iter()
            .filter(|a| a.kind == SyncActionKind::NoOp)
    }

    /// Returns true if the plan contains any conflicts requiring resolution.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        self.conflicts().next().is_some()
    }

    /// Returns true if all actions in the plan are `NoOp`.
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.actions.iter().all(|a| a.kind == SyncActionKind::NoOp)
    }

    /// Total count of planned actions across all targets.
    #[must_use]
    pub fn total_actions(&self) -> usize {
        self.actions.len()
    }

    /// Returns actions scoped to a specific target environment.
    pub fn actions_for_target<'a>(
        &'a self,
        target: &'a TargetEnvironment,
    ) -> impl Iterator<Item = &'a SyncAction> {
        self.actions.iter().filter(move |a| &a.target_env == target)
    }
}

/// Conflict resolution strategies for handling divergent skill versions and content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictStrategy {
    /// Local catalog state forcefully overwrites target state with automated backup creation.
    LocalWins,
    /// Target state is preserved; local catalog changes are skipped.
    RemoteWins,
    /// Requires user prompt / approval; in non-interactive/headless execution, aborts with error.
    #[default]
    PromptUser,
}

/// Summary of executed synchronization actions and metrics.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSummary {
    /// Number of new skills successfully installed.
    pub installed: usize,

    /// Number of existing skills updated.
    pub updated: usize,

    /// Number of orphaned skills removed.
    pub deleted: usize,

    /// Number of detected conflicts successfully resolved.
    pub conflicts_resolved: usize,

    /// Number of skills already up to date (no action required).
    pub no_ops: usize,

    /// Indicates whether execution was simulated without disk mutations.
    pub dry_run: bool,

    /// List of skill IDs that were affected (installed, updated, or deleted).
    pub affected_skills: Vec<String>,
}

impl SyncSummary {
    /// Generates a summary for a dry-run execution directly from a plan.
    #[must_use]
    pub fn from_plan_dry_run(plan: &SyncPlan) -> Self {
        let mut affected = Vec::new();
        for action in &plan.actions {
            if matches!(
                action.kind,
                SyncActionKind::Install | SyncActionKind::Update | SyncActionKind::Delete
            ) && !affected.contains(&action.skill_id)
            {
                affected.push(action.skill_id.clone());
            }
        }
        Self {
            installed: plan.installs().count(),
            updated: plan.updates().count(),
            deleted: plan.deletes().count(),
            conflicts_resolved: 0,
            no_ops: plan.no_ops().count(),
            dry_run: true,
            affected_skills: affected,
        }
    }
}

/// Comprehensive error taxonomy for skill synchronization failures.
#[derive(Debug, Error)]
pub enum SyncError {
    #[error("I/O error at {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Installer error: {0}")]
    Installer(#[from] InstallerError),

    #[error("Unresolved conflict for skill '{skill_id}' in target '{target:?}': {reason}")]
    UnresolvedConflict {
        skill_id: String,
        target: TargetEnvironment,
        reason: String,
    },

    #[error("Target environment '{0:?}' is unreachable or cannot be resolved")]
    TargetUnreachable(TargetEnvironment),

    #[error("Invalid skill source catalog at {path:?}: {reason}")]
    InvalidCatalog { path: PathBuf, reason: String },
}

impl From<SyncError> for SkillError {
    fn from(err: SyncError) -> Self {
        match err {
            SyncError::Io { path, source } => SkillError::Io { path, source },
            SyncError::Installer(e) => e.into(),
            SyncError::UnresolvedConflict {
                skill_id, reason, ..
            } => SkillError::validation(
                PathBuf::new(),
                format!("Conflict on '{skill_id}': {reason}"),
            ),
            SyncError::TargetUnreachable(env) => {
                SkillError::validation(PathBuf::new(), format!("Target unreachable: {env:?}"))
            }
            SyncError::InvalidCatalog { path, reason } => SkillError::validation(path, reason),
        }
    }
}

// -----------------------------------------------------------------------------
// Internal SemVer & Metadata Helpers
// -----------------------------------------------------------------------------

/// Parsed `SemVer` components for version comparison without external dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedVersion<'a> {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_release: Option<&'a str>,
}

impl<'a> ParsedVersion<'a> {
    pub(crate) fn parse(s: &'a str) -> Option<Self> {
        let trimmed = s.trim().trim_start_matches(['v', 'V']);
        let (release, pre_release) = match trimmed.split_once('-') {
            Some((r, p)) => (r, Some(p)),
            None => (trimmed, None),
        };

        let mut parts = release.split('.');
        let major = parts.next()?.parse::<u64>().ok()?;
        let minor = parts.next().unwrap_or("0").parse::<u64>().ok()?;
        let patch = parts.next().unwrap_or("0").parse::<u64>().ok()?;

        Some(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }
}

impl Ord for ParsedVersion<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)) {
            Ordering::Equal => match (self.pre_release, other.pre_release) {
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Less, // 1.0.0-beta < 1.0.0
                (None, Some(_)) => Ordering::Greater,
                (Some(a), Some(b)) => a.cmp(b),
            },
            ord => ord,
        }
    }
}

impl PartialOrd for ParsedVersion<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares two version strings according to `SemVer` rules with lexical fallback.
pub(crate) fn compare_versions(src: &str, tgt: &str) -> Ordering {
    match (ParsedVersion::parse(src), ParsedVersion::parse(tgt)) {
        (Some(s), Some(t)) => s.cmp(&t),
        _ => src.cmp(tgt),
    }
}

/// Extracts skill ID, human name, and version from a skill source directory.
pub(crate) fn extract_skill_info(source_dir: &Path) -> (String, String, String) {
    let skill_md = source_dir.join("SKILL.md");
    if skill_md.exists() {
        if let Ok(content) = fs::read_to_string(&skill_md) {
            if let Ok((yaml_str, _)) = SkillParser::extract_frontmatter(&content) {
                if let Ok(fm) = serde_yaml::from_str::<SkillFrontmatter>(yaml_str) {
                    let id = if fm.name.is_empty() {
                        source_dir.file_name().map_or_else(
                            || "unnamed-skill".to_string(),
                            |n| n.to_string_lossy().to_string(),
                        )
                    } else {
                        fm.name.clone()
                    };
                    let name = if fm.name.is_empty() {
                        id.clone()
                    } else {
                        fm.name
                    };
                    let version = fm
                        .metadata
                        .as_ref()
                        .and_then(|m| m.get("version"))
                        .cloned()
                        .or_else(|| {
                            fm.extra.get("version").and_then(|v| match v {
                                serde_yaml::Value::String(s) => Some(s.clone()),
                                serde_yaml::Value::Number(n) => Some(n.to_string()),
                                _ => None,
                            })
                        })
                        .unwrap_or_else(|| "0.1.0".to_string());
                    return (id, name, version);
                }
            }
        }
    }

    let fallback = source_dir.file_name().map_or_else(
        || "unnamed-skill".to_string(),
        |n| n.to_string_lossy().to_string(),
    );
    (fallback.clone(), fallback, "0.1.0".to_string())
}

// -----------------------------------------------------------------------------
// SkillSyncer Coordinator
// -----------------------------------------------------------------------------

/// Multi-target skill synchronization coordinator.
#[derive(Debug, Clone)]
pub struct SkillSyncer {
    /// Root path of the local skill catalog (e.g. `repo_root/skills`).
    pub catalog_dir: PathBuf,

    /// Target agent execution environments to synchronize.
    pub targets: Vec<TargetEnvironment>,

    /// Underlying installer engine for executing updates, installs, and uninstalls.
    pub installer: Installer,

    /// Preferred installation mode (Auto, Symlink, or Copy).
    pub mode: InstallMode,

    /// Conflict resolution policy.
    pub conflict_strategy: ConflictStrategy,

    /// If true, removes installed skills that no longer exist in the local catalog.
    pub prune_orphans: bool,

    /// If true, executes analysis only without modifying disk state or acquiring locks.
    pub dry_run: bool,

    /// Glob patterns and filenames excluded during checksum computation and file transfers.
    pub exclude_patterns: Vec<String>,
}

impl SkillSyncer {
    /// Creates a new syncer anchored to a local catalog directory with default settings.
    #[must_use]
    pub fn new(catalog_dir: impl Into<PathBuf>) -> Self {
        Self {
            catalog_dir: catalog_dir.into(),
            targets: Vec::new(),
            installer: Installer::new(),
            mode: InstallMode::Auto,
            conflict_strategy: ConflictStrategy::PromptUser,
            prune_orphans: false,
            dry_run: false,
            exclude_patterns: default_exclude_patterns(),
        }
    }

    /// Adds a target environment to synchronize.
    #[must_use]
    pub fn with_target(mut self, target: TargetEnvironment) -> Self {
        self.targets.push(target);
        self
    }

    /// Adds multiple target environments to synchronize.
    #[must_use]
    pub fn with_targets(mut self, targets: impl IntoIterator<Item = TargetEnvironment>) -> Self {
        self.targets.extend(targets);
        self
    }

    /// Sets the conflict resolution strategy.
    #[must_use]
    pub fn with_conflict_strategy(mut self, strategy: ConflictStrategy) -> Self {
        self.conflict_strategy = strategy;
        self
    }

    /// Sets the orphan pruning policy.
    #[must_use]
    pub fn with_prune_orphans(mut self, prune: bool) -> Self {
        self.prune_orphans = prune;
        self
    }

    /// Sets dry-run simulation mode.
    #[must_use]
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the underlying installer instance.
    #[must_use]
    pub fn with_installer(mut self, installer: Installer) -> Self {
        self.installer = installer;
        self
    }

    /// Sets the preferred install mode (Auto, Symlink, Copy).
    #[must_use]
    pub fn with_mode(mut self, mode: InstallMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets exclusion patterns for checksumming and copying.
    #[must_use]
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Discovers skills in the catalog directory, returning a mapping of skill ID to directory path.
    #[allow(tail_expr_drop_order)]
    pub fn discover_catalog_skills(&self) -> Result<HashMap<String, PathBuf>, SyncError> {
        if !self.catalog_dir.exists() || !self.catalog_dir.is_dir() {
            return Err(SyncError::InvalidCatalog {
                path: self.catalog_dir.clone(),
                reason: "Catalog path does not exist or is not a directory".to_string(),
            });
        }

        let mut skills = HashMap::new();
        let entries = fs::read_dir(&self.catalog_dir).map_err(|e| SyncError::Io {
            path: self.catalog_dir.clone(),
            source: e,
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| SyncError::Io {
                path: self.catalog_dir.clone(),
                source: e,
            })?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let name_lossy = path.file_name().unwrap_or_default().to_string_lossy();
            if is_excluded(&name_lossy, &self.exclude_patterns) {
                continue;
            }

            // Check if flat layout: <catalog>/<skill_id>/SKILL.md
            if path.join("SKILL.md").is_file() {
                let (skill_id, _, _) = extract_skill_info(&path);
                PathValidator::validate_skill_id(&skill_id).map_err(|e| {
                    SyncError::InvalidCatalog {
                        path: path.clone(),
                        reason: e.to_string(),
                    }
                })?;
                skills.insert(skill_id, path);
            } else {
                // Check categorized layout: <catalog>/<category>/<skill_id>/SKILL.md
                let sub_entries_res = fs::read_dir(&path);
                if let Ok(sub_entries) = sub_entries_res {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if !sub_path.is_dir() {
                            continue;
                        }
                        let sub_name = sub_path.file_name().unwrap_or_default().to_string_lossy();
                        if is_excluded(&sub_name, &self.exclude_patterns) {
                            continue;
                        }
                        if sub_path.join("SKILL.md").is_file() {
                            let (skill_id, _, _) = extract_skill_info(&sub_path);
                            PathValidator::validate_skill_id(&skill_id).map_err(|e| {
                                SyncError::InvalidCatalog {
                                    path: sub_path.clone(),
                                    reason: e.to_string(),
                                }
                            })?;
                            skills.insert(skill_id, sub_path);
                        }
                    }
                }
            }
        }

        Ok(skills)
    }

    /// Computes a side-effect-free reconciliation plan across all configured targets.
    #[allow(clippy::too_many_lines)]
    pub fn create_plan(&self) -> Result<SyncPlan, SyncError> {
        let catalog_skills = self.discover_catalog_skills()?;
        let mut sorted_catalog_skills: Vec<(&String, &PathBuf)> = catalog_skills.iter().collect();
        sorted_catalog_skills.sort_by_key(|(id, _)| *id);

        let mut actions = Vec::new();

        for target in &self.targets {
            let registry = self.installer.read_registry(target)?;
            let skills_dir = self.installer.resolve_target_dir(target)?;

            // 1. Process catalog skills against target (in deterministic alphabetical order)
            for (skill_id, source_path) in &sorted_catalog_skills {
                let (_, _, source_version) = extract_skill_info(source_path);
                let entries = collect_file_entries(source_path, &self.exclude_patterns)?;
                let source_checksum = compute_dir_checksum(&entries);

                if let Some(installed_skill) = registry.get(skill_id) {
                    let target_path = skills_dir.join(skill_id);
                    let is_symlink = target_path.symlink_metadata().is_ok_and(|m| m.is_symlink())
                        || installed_skill.mode == InstallMode::Symlink;

                    let integrity = self.installer.verify(skill_id, target)?;

                    let is_tampered = match integrity {
                        IntegrityStatus::MissingFile { .. } => true,
                        IntegrityStatus::ChecksumMismatch { .. } => !is_symlink,
                        IntegrityStatus::Valid | IntegrityStatus::ExtraFile { .. } => false,
                    };

                    if is_tampered {
                        actions.push(SyncAction {
                            skill_id: (*skill_id).clone(),
                            target_env: target.clone(),
                            kind: SyncActionKind::Conflict,
                            source_version: Some(source_version),
                            target_version: Some(installed_skill.version.clone()),
                            source_checksum: Some(source_checksum),
                            target_checksum: Some(installed_skill.checksum.clone()),
                            reason: format!(
                                "Target integrity verification failed for '{skill_id}': files modified or missing out-of-band"
                            ),
                        });
                    } else {
                        let version_cmp =
                            compare_versions(&source_version, &installed_skill.version);

                        match version_cmp {
                            Ordering::Greater => {
                                actions.push(SyncAction {
                                    skill_id: (*skill_id).clone(),
                                    target_env: target.clone(),
                                    kind: SyncActionKind::Update,
                                    source_version: Some(source_version.clone()),
                                    target_version: Some(installed_skill.version.clone()),
                                    source_checksum: Some(source_checksum),
                                    target_checksum: Some(installed_skill.checksum.clone()),
                                    reason: format!(
                                        "Source has newer version ({source_version} > {})",
                                        installed_skill.version
                                    ),
                                });
                            }
                            Ordering::Less => {
                                actions.push(SyncAction {
                                        skill_id: (*skill_id).clone(),
                                        target_env: target.clone(),
                                        kind: SyncActionKind::Conflict,
                                        source_version: Some(source_version.clone()),
                                        target_version: Some(installed_skill.version.clone()),
                                        source_checksum: Some(source_checksum),
                                        target_checksum: Some(installed_skill.checksum.clone()),
                                        reason: format!(
                                            "Local catalog version is older than target ({source_version} < {}) - downgrade hazard",
                                            installed_skill.version
                                        ),
                                    });
                            }
                            Ordering::Equal => {
                                if source_checksum == installed_skill.checksum {
                                    actions.push(SyncAction {
                                        skill_id: (*skill_id).clone(),
                                        target_env: target.clone(),
                                        kind: SyncActionKind::NoOp,
                                        source_version: Some(source_version),
                                        target_version: Some(installed_skill.version.clone()),
                                        source_checksum: Some(source_checksum),
                                        target_checksum: Some(installed_skill.checksum.clone()),
                                        reason: "Skill is up to date (version and checksum match)"
                                            .to_string(),
                                    });
                                } else {
                                    actions.push(SyncAction {
                                            skill_id: (*skill_id).clone(),
                                            target_env: target.clone(),
                                            kind: SyncActionKind::Update,
                                            source_version: Some(source_version.clone()),
                                            target_version: Some(installed_skill.version.clone()),
                                            source_checksum: Some(source_checksum),
                                            target_checksum: Some(installed_skill.checksum.clone()),
                                            reason: format!(
                                                "Content drift detected at version {source_version} (checksum mismatch)"
                                            ),
                                        });
                                }
                            }
                        }
                    }
                } else {
                    // Not in target registry
                    let target_path = skills_dir.join(skill_id);
                    let target_exists =
                        target_path.exists() || target_path.symlink_metadata().is_ok();
                    let reason = if target_exists {
                        "Untracked target directory exists on disk, requires adoption".to_string()
                    } else {
                        "Skill not installed in target environment".to_string()
                    };

                    actions.push(SyncAction {
                        skill_id: (*skill_id).clone(),
                        target_env: target.clone(),
                        kind: SyncActionKind::Install,
                        source_version: Some(source_version),
                        target_version: None,
                        source_checksum: Some(source_checksum),
                        target_checksum: None,
                        reason,
                    });
                }
            }

            // 2. Process orphaned installed skills in deterministic alphabetical order
            let mut sorted_registry_skills: Vec<(&String, &InstalledSkill)> =
                registry.skills.iter().collect();
            sorted_registry_skills.sort_by_key(|(id, _)| *id);

            for (installed_id, installed_skill) in sorted_registry_skills {
                if !catalog_skills.contains_key(installed_id) {
                    if self.prune_orphans {
                        actions.push(SyncAction {
                            skill_id: installed_id.clone(),
                            target_env: target.clone(),
                            kind: SyncActionKind::Delete,
                            source_version: None,
                            target_version: Some(installed_skill.version.clone()),
                            source_checksum: None,
                            target_checksum: Some(installed_skill.checksum.clone()),
                            reason: "Skill removed from catalog (orphan pruning enabled)"
                                .to_string(),
                        });
                    } else {
                        actions.push(SyncAction {
                            skill_id: installed_id.clone(),
                            target_env: target.clone(),
                            kind: SyncActionKind::NoOp,
                            source_version: None,
                            target_version: Some(installed_skill.version.clone()),
                            source_checksum: None,
                            target_checksum: Some(installed_skill.checksum.clone()),
                            reason: "Orphan skill in target retained (prune_orphans = false)"
                                .to_string(),
                        });
                    }
                }
            }
        }

        Ok(SyncPlan::new(actions))
    }

    /// Executes a pre-computed synchronization plan, applying conflict policies and mutations.
    #[allow(clippy::too_many_lines)]
    pub fn execute_plan(&self, plan: &SyncPlan) -> Result<SyncSummary, SyncError> {
        if self.dry_run {
            return Ok(SyncSummary::from_plan_dry_run(plan));
        }

        let catalog_skills = self.discover_catalog_skills()?;
        let mut summary = SyncSummary::default();

        for action in &plan.actions {
            match action.kind {
                SyncActionKind::Install => {
                    let source_path = catalog_skills.get(&action.skill_id).ok_or_else(|| {
                        SyncError::InvalidCatalog {
                            path: self.catalog_dir.clone(),
                            reason: format!(
                                "Source path not found in catalog for skill '{}'",
                                action.skill_id
                            ),
                        }
                    })?;

                    let install_opts = InstallOptions {
                        mode: self.mode,
                        exclude_patterns: self.exclude_patterns.clone(),
                        force: true,
                        create_backup: true,
                        ..Default::default()
                    };

                    self.installer
                        .install(source_path, &action.target_env, &install_opts)?;
                    summary.installed += 1;
                    if !summary.affected_skills.contains(&action.skill_id) {
                        summary.affected_skills.push(action.skill_id.clone());
                    }
                }
                SyncActionKind::Update => {
                    let source_path = catalog_skills.get(&action.skill_id).ok_or_else(|| {
                        SyncError::InvalidCatalog {
                            path: self.catalog_dir.clone(),
                            reason: format!(
                                "Source path not found in catalog for skill '{}'",
                                action.skill_id
                            ),
                        }
                    })?;

                    let update_opts = InstallOptions {
                        mode: self.mode,
                        exclude_patterns: self.exclude_patterns.clone(),
                        force: true,
                        create_backup: true,
                        ..Default::default()
                    };

                    self.installer
                        .update(source_path, &action.target_env, &update_opts)?;
                    summary.updated += 1;
                    if !summary.affected_skills.contains(&action.skill_id) {
                        summary.affected_skills.push(action.skill_id.clone());
                    }
                }
                SyncActionKind::Delete => {
                    let uninstall_opts = UninstallOptions {
                        purge_state: true,
                        create_backup: true,
                        ..Default::default()
                    };

                    self.installer.uninstall(
                        &action.skill_id,
                        &action.target_env,
                        &uninstall_opts,
                    )?;
                    summary.deleted += 1;
                    if !summary.affected_skills.contains(&action.skill_id) {
                        summary.affected_skills.push(action.skill_id.clone());
                    }
                }
                SyncActionKind::NoOp => {
                    summary.no_ops += 1;
                }
                SyncActionKind::Conflict => match self.conflict_strategy {
                    ConflictStrategy::LocalWins => {
                        let source_path =
                            catalog_skills.get(&action.skill_id).ok_or_else(|| {
                                SyncError::InvalidCatalog {
                                    path: self.catalog_dir.clone(),
                                    reason: format!(
                                        "Source path not found in catalog for skill '{}'",
                                        action.skill_id
                                    ),
                                }
                            })?;

                        let opts = InstallOptions {
                            mode: self.mode,
                            exclude_patterns: self.exclude_patterns.clone(),
                            force: true,
                            create_backup: true,
                            ..Default::default()
                        };

                        let target_skill = self
                            .installer
                            .get_skill(&action.skill_id, &action.target_env)?;
                        if target_skill.is_some() {
                            self.installer
                                .update(source_path, &action.target_env, &opts)?;
                        } else {
                            self.installer
                                .install(source_path, &action.target_env, &opts)?;
                        }

                        summary.conflicts_resolved += 1;
                        summary.updated += 1;
                        if !summary.affected_skills.contains(&action.skill_id) {
                            summary.affected_skills.push(action.skill_id.clone());
                        }
                    }
                    ConflictStrategy::RemoteWins => {
                        summary.conflicts_resolved += 1;
                        summary.no_ops += 1;
                    }
                    ConflictStrategy::PromptUser => {
                        return Err(SyncError::UnresolvedConflict {
                            skill_id: action.skill_id.clone(),
                            target: action.target_env.clone(),
                            reason: action.reason.clone(),
                        });
                    }
                },
            }
        }

        Ok(summary)
    }

    /// Convenience workflow: executes `create_plan()` followed immediately by `execute_plan()`.
    pub fn sync(&self) -> Result<SyncSummary, SyncError> {
        let plan = self.create_plan()?;
        self.execute_plan(&plan)
    }
}

// -----------------------------------------------------------------------------
// Unit Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_version_basic() {
        let v = ParsedVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.pre_release, None);

        let v_pre = ParsedVersion::parse("v2.0.0-beta.1").unwrap();
        assert_eq!(v_pre.major, 2);
        assert_eq!(v_pre.minor, 0);
        assert_eq!(v_pre.patch, 0);
        assert_eq!(v_pre.pre_release, Some("beta.1"));
    }

    #[test]
    fn test_compare_versions() {
        assert_eq!(compare_versions("1.10.0", "1.2.0"), Ordering::Greater);
        assert_eq!(compare_versions("1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.0.0-beta", "1.0.0"), Ordering::Less);
        assert_eq!(compare_versions("1.0.0", "1.0.0-alpha"), Ordering::Greater);
        assert_eq!(compare_versions("v1.2.0", "1.2.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.0.0", "1.1.0"), Ordering::Less);
    }

    #[test]
    fn test_plan_helpers() {
        let action1 = SyncAction {
            skill_id: "skill-a".to_string(),
            target_env: TargetEnvironment::ClaudeDesktop,
            kind: SyncActionKind::Install,
            source_version: Some("1.0.0".to_string()),
            target_version: None,
            source_checksum: None,
            target_checksum: None,
            reason: "new skill".to_string(),
        };
        let action2 = SyncAction {
            skill_id: "skill-b".to_string(),
            target_env: TargetEnvironment::Cursor,
            kind: SyncActionKind::NoOp,
            source_version: Some("1.0.0".to_string()),
            target_version: Some("1.0.0".to_string()),
            source_checksum: None,
            target_checksum: None,
            reason: "up to date".to_string(),
        };

        let plan = SyncPlan::new(vec![action1, action2]);
        assert_eq!(plan.total_actions(), 2);
        assert_eq!(plan.installs().count(), 1);
        assert_eq!(plan.no_ops().count(), 1);
        assert!(!plan.is_noop());
        assert!(!plan.has_conflicts());

        let summary = SyncSummary::from_plan_dry_run(&plan);
        assert!(summary.dry_run);
        assert_eq!(summary.installed, 1);
        assert_eq!(summary.no_ops, 1);
        assert_eq!(summary.affected_skills, vec!["skill-a"]);
    }
}
