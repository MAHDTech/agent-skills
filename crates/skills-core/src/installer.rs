//! Skill installation, environment bridge, and lifecycle management.
//!
//! Provides enterprise-grade lifecycle management for skills across agent environments such as
//! Claude Desktop, Cursor, Antigravity, and custom workspace paths.
//! Supports automatic environment resolution, symlink mode (development), file copy mode (production),
//! atomic state tracking via `installed-skills.json`, scoped advisory locking, transactional directory swaps,
//! cryptographic integrity verification, and safe uninstallation with backups.

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::error::SkillError;

// -----------------------------------------------------------------------------
// Target Environments & Enums
// -----------------------------------------------------------------------------

/// Target AI agent execution environments supported by the installer bridge.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetEnvironment {
    /// Claude Desktop application config and skills directory.
    ClaudeDesktop,
    /// Cursor IDE application config and skills directory.
    Cursor,
    /// Antigravity CLI global or workspace agent runtime environment.
    Antigravity {
        /// Optional workspace root override; if None, uses default system location.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        workspace_root: Option<PathBuf>,
    },
    /// Arbitrary user-defined filesystem destination directory.
    Custom(PathBuf),
}

impl TargetEnvironment {
    /// Returns the machine-readable identifier for this target environment.
    #[must_use]
    pub fn identifier(&self) -> &str {
        match self {
            Self::ClaudeDesktop => "claude-desktop",
            Self::Cursor => "cursor",
            Self::Antigravity { .. } => "antigravity",
            Self::Custom(_) => "custom",
        }
    }

    /// Alias for `identifier`.
    #[must_use]
    pub fn name(&self) -> &str {
        self.identifier()
    }

    /// Returns human-readable display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeDesktop => "Claude Desktop",
            Self::Cursor => "Cursor",
            Self::Antigravity { .. } => "Antigravity",
            Self::Custom(_) => "Custom Workspace",
        }
    }

    /// Returns a list of all standard built-in environments.
    #[must_use]
    pub fn all_standard() -> Vec<Self> {
        vec![
            Self::ClaudeDesktop,
            Self::Cursor,
            Self::Antigravity {
                workspace_root: None,
            },
        ]
    }

    /// Resolves the default skills directory relative to a user's home directory.
    #[must_use]
    pub fn default_skills_dir(&self, home_dir: Option<&Path>) -> PathBuf {
        let default_home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let home = home_dir.unwrap_or(&default_home);

        match self {
            Self::ClaudeDesktop => {
                #[cfg(target_os = "macos")]
                {
                    home.join("Library")
                        .join("Application Support")
                        .join("Claude")
                        .join("skills")
                }
                #[cfg(not(target_os = "macos"))]
                {
                    home.join(".claude").join("skills")
                }
            }
            Self::Cursor => home.join(".cursor").join("skills"),
            Self::Antigravity { workspace_root } => {
                if let Some(ws) = workspace_root {
                    ws.join(".agents").join("skills")
                } else {
                    home.join(".agents").join("skills")
                }
            }
            Self::Custom(p) => {
                if p.is_absolute() {
                    p.clone()
                } else {
                    home.join(p)
                }
            }
        }
    }
}

/// Strategy used when placing skill files into a target agent environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum InstallMode {
    /// Automatically attempts Symlink mode; falls back to Copy if symlinks unsupported.
    #[default]
    Auto,
    /// Creates filesystem symbolic links pointing to the original source working directory.
    Symlink,
    /// Recursively copies all skill files and vendored assets into the target environment.
    Copy,
}

// -----------------------------------------------------------------------------
// Options & Configuration Structs
// -----------------------------------------------------------------------------

/// Configuration parameters controlling installation behaviors and collision resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct InstallOptions {
    /// Desired installation strategy (Auto, Symlink, or Copy).
    #[builder(default)]
    pub mode: InstallMode,
    /// If true, overwrites pre-existing skill installations at the target path.
    #[builder(default = false)]
    pub force: bool,
    /// If true, creates a timestamped snapshot backup before overwriting or updating.
    #[builder(default = true)]
    pub create_backup: bool,
    /// Maximum duration in milliseconds to wait for advisory file lock acquisition.
    #[builder(default = 10_000)]
    pub lock_timeout_ms: u64,
    /// Glob patterns or filenames excluded during copy-mode installation.
    #[builder(default = default_exclude_patterns())]
    pub exclude_patterns: Vec<String>,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            mode: InstallMode::Auto,
            force: false,
            create_backup: true,
            lock_timeout_ms: 10_000,
            exclude_patterns: default_exclude_patterns(),
        }
    }
}

/// Default exclude patterns when copying skill directory contents.
#[must_use]
pub fn default_exclude_patterns() -> Vec<String> {
    vec![
        ".git".to_string(),
        ".git/**".to_string(),
        "node_modules".to_string(),
        "node_modules/**".to_string(),
        "target".to_string(),
        "target/**".to_string(),
        ".DS_Store".to_string(),
        "*.lock".to_string(),
        "*.tmp".to_string(),
    ]
}

/// Configuration parameters controlling uninstallation behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct UninstallOptions {
    /// If true, archives the skill directory to a backup location before deletion.
    #[builder(default = false)]
    pub create_backup: bool,
    /// If true, permanently purges registry records instead of marking active=false.
    #[builder(default = true)]
    pub purge_state: bool,
    /// Maximum duration in milliseconds to wait for advisory file lock acquisition.
    #[builder(default = 10_000)]
    pub lock_timeout_ms: u64,
}

impl Default for UninstallOptions {
    fn default() -> Self {
        Self {
            create_backup: false,
            purge_state: true,
            lock_timeout_ms: 10_000,
        }
    }
}

// -----------------------------------------------------------------------------
// Domain Records & Metadata
// -----------------------------------------------------------------------------

/// Metadata record for an individual file installed within a skill bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SkillFileEntry {
    /// Relative path inside skill root directory (normalized with forward slashes).
    #[builder(setter(into))]
    pub path: String,
    /// SHA-256 hexadecimal digest of the file contents.
    #[builder(setter(into))]
    pub sha256: String,
    /// File size in bytes.
    pub size_bytes: u64,
    /// Unix permissions mode (or standard file attribute mask on Windows).
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions_mode: Option<u32>,
}

/// Comprehensive lifecycle metadata record for an installed skill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct InstalledSkill {
    /// Unique identifier / sanitized slug for the skill (e.g. "rust-analyzer-helper").
    #[builder(setter(into))]
    pub id: String,
    /// Human-readable display name.
    #[builder(setter(into))]
    pub name: String,
    /// Semantic version string (e.g. "1.0.0").
    #[builder(setter(into))]
    pub version: String,
    /// Original filesystem source path from which the skill was installed.
    pub source_path: PathBuf,
    /// Final absolute target installation path where files reside.
    pub target_path: PathBuf,
    /// Effective installation mode utilized (Symlink or Copy).
    pub mode: InstallMode,
    /// ISO-8601 UTC timestamp recording when the skill was first installed.
    #[builder(setter(into))]
    pub installed_at: String,
    /// ISO-8601 UTC timestamp recording the most recent update.
    #[builder(setter(into))]
    pub updated_at: String,
    /// Composite SHA-256 digest calculated over all tracked skill file entries.
    #[builder(setter(into))]
    pub checksum: String,
    /// Target execution environment.
    pub environment: TargetEnvironment,
    /// Detailed list of tracked file entries and their individual digests.
    #[builder(default, setter(into))]
    #[serde(default)]
    pub files: Vec<SkillFileEntry>,
    /// Active status flag (false indicates soft-uninstalled / disabled).
    #[builder(default = true)]
    pub active: bool,
    /// Extensible key-value metadata bag.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

/// Type aliases for compatibility.
pub type SkillRecord = InstalledSkill;
pub type InstalledSkillEntry = InstalledSkill;

/// Schema-versioned state registry tracking all installed skills within an environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct InstalledSkillsRegistry {
    /// Registry schema version number (currently 2).
    #[builder(default = 2)]
    pub schema_version: u32,
    /// ISO-8601 UTC timestamp of last registry modification.
    #[builder(default = timestamp_now_iso())]
    pub updated_at: String,
    /// Map of skill ID to `InstalledSkill` record.
    #[builder(default)]
    pub skills: HashMap<String, InstalledSkill>,
}

impl Default for InstalledSkillsRegistry {
    fn default() -> Self {
        Self {
            schema_version: 2,
            updated_at: timestamp_now_iso(),
            skills: HashMap::new(),
        }
    }
}

impl InstalledSkillsRegistry {
    /// Creates a new empty registry instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the registry from disk, applying automatic schema migration from v1 to v2 if needed.
    pub fn load(path: &Path) -> Result<Self, InstallerError> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path).map_err(|e| InstallerError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        if content.trim().is_empty() {
            return Ok(Self::new());
        }

        let raw_val: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| InstallerError::RegistrySerialization {
                path: path.to_path_buf(),
                source: e,
            })?;

        let schema_ver = raw_val
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1);

        if schema_ver > 2 {
            return Err(InstallerError::UnsupportedSchemaVersion {
                found: u32::try_from(schema_ver).unwrap_or(u32::MAX),
                supported: 2,
            });
        }

        if schema_ver == 1 {
            let mut reg = Self::default();
            if let Some(skills_obj) = raw_val.get("skills").and_then(serde_json::Value::as_object) {
                for (k, v) in skills_obj {
                    if let Ok(skill) = serde_json::from_value::<InstalledSkill>(v.clone()) {
                        reg.skills.insert(k.clone(), skill);
                    } else {
                        let name = v
                            .get("name")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or(k)
                            .to_string();
                        let version = v
                            .get("version")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("0.1.0")
                            .to_string();
                        let source_path = v
                            .get("source_path")
                            .and_then(serde_json::Value::as_str)
                            .map(PathBuf::from)
                            .unwrap_or_default();
                        let target_path = v
                            .get("target_path")
                            .and_then(serde_json::Value::as_str)
                            .map(PathBuf::from)
                            .unwrap_or_default();
                        let mode = v
                            .get("mode")
                            .and_then(|m| serde_json::from_value::<InstallMode>(m.clone()).ok())
                            .unwrap_or(InstallMode::Copy);
                        let installed_at = v
                            .get("installed_at")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or(&reg.updated_at)
                            .to_string();
                        let active = v
                            .get("active")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(true);

                        let skill = InstalledSkill {
                            id: k.clone(),
                            name,
                            version,
                            source_path,
                            target_path,
                            mode,
                            installed_at,
                            updated_at: reg.updated_at.clone(),
                            checksum: String::new(),
                            environment: TargetEnvironment::Custom(PathBuf::new()),
                            files: Vec::new(),
                            active,
                            metadata: HashMap::new(),
                        };
                        reg.skills.insert(k.clone(), skill);
                    }
                }
            }
            return Ok(reg);
        }

        let registry: InstalledSkillsRegistry =
            serde_json::from_value(raw_val).map_err(|e| InstallerError::RegistrySerialization {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(registry)
    }

    /// Atomically persists `InstalledSkillsRegistry` to disk via tempfile flush and rename.
    pub fn save(&self, path: &Path) -> Result<(), InstallerError> {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| {
            InstallerError::RegistrySerialization {
                path: path.to_path_buf(),
                source: e,
            }
        })?;

        let nonce = generate_nonce();
        let tmp_path = path.with_extension(format!("tmp.{nonce}"));
        {
            let mut file = File::create(&tmp_path).map_err(|e| InstallerError::Io {
                path: tmp_path.clone(),
                source: e,
            })?;
            file.write_all(content.as_bytes())
                .map_err(|e| InstallerError::Io {
                    path: tmp_path.clone(),
                    source: e,
                })?;
            file.flush().map_err(|e| InstallerError::Io {
                path: tmp_path.clone(),
                source: e,
            })?;
        }

        fs::rename(&tmp_path, path).map_err(|e| {
            let _ = fs::remove_file(&tmp_path);
            InstallerError::Io {
                path: path.to_path_buf(),
                source: e,
            }
        })?;

        Ok(())
    }

    /// Returns a reference to an installed skill by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&InstalledSkill> {
        self.skills.get(id)
    }

    /// Returns a mutable reference to an installed skill by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut InstalledSkill> {
        self.skills.get_mut(id)
    }

    /// Inserts or updates an installed skill record.
    pub fn insert(&mut self, skill: InstalledSkill) {
        self.updated_at = timestamp_now_iso();
        self.skills.insert(skill.id.clone(), skill);
    }

    /// Removes a skill record by ID.
    pub fn remove(&mut self, id: &str) -> Option<InstalledSkill> {
        self.updated_at = timestamp_now_iso();
        self.skills.remove(id)
    }
}

// -----------------------------------------------------------------------------
// Operation Results & Verification
// -----------------------------------------------------------------------------

/// Result payload returned upon successful skill installation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallResult {
    /// The installed skill record.
    pub skill: InstalledSkill,
    /// The effective installation mode chosen and executed.
    pub installed_mode: InstallMode,
    /// Whether an existing installation was replaced.
    pub replaced_existing: bool,
    /// Path to created backup directory, if backup was enabled during overwrite.
    pub backup_path: Option<PathBuf>,
}

/// Result payload returned upon skill uninstallation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UninstallResult {
    /// ID of the uninstalled skill.
    pub skill_id: String,
    /// Target path from which the skill was removed.
    pub target_path: PathBuf,
    /// Path to backup directory if backup was requested.
    pub backup_path: Option<PathBuf>,
}

/// Result payload returned upon skill update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateResult {
    /// The updated skill record.
    pub skill: InstalledSkill,
    /// Previous version string before update.
    pub previous_version: String,
    /// New version string after update.
    pub new_version: String,
    /// Path to backup directory created during update.
    pub backup_path: Option<PathBuf>,
}

/// Integrity verification status for an installed skill bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrityStatus {
    /// All files and digests match registry specifications.
    Valid,
    /// Cumulative or individual file checksum mismatch.
    ChecksumMismatch { expected: String, actual: String },
    /// A required tracked file is missing on disk.
    MissingFile { path: String },
    /// An unexpected or untracked file was found in target directory.
    ExtraFile { path: String },
}

// -----------------------------------------------------------------------------
// Installer Errors
// -----------------------------------------------------------------------------

/// Comprehensive error taxonomy for skill installation and lifecycle management.
#[derive(Error, Debug)]
pub enum InstallerError {
    #[error("I/O error at {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Path traversal detected: {path:?} escapes bounded root {boundary:?}")]
    PathTraversal { path: PathBuf, boundary: PathBuf },

    #[error("Invalid skill ID '{id}': must contain only alphanumeric ASCII, hyphens, and underscores (1-64 chars)")]
    InvalidSkillId { id: String },

    #[error("Skill '{id}' is already installed at {existing_path:?}. Use force=true to overwrite")]
    SkillAlreadyInstalled { id: String, existing_path: PathBuf },

    #[error("Skill '{id}' not found in target environment registry")]
    SkillNotFound { id: String },

    #[error("Target environment '{0:?}' could not be resolved on current platform")]
    EnvironmentNotResolvable(TargetEnvironment),

    #[error(
        "Failed to acquire advisory file lock at {lock_path:?} within {timeout_ms}ms: {reason}"
    )]
    LockTimeout {
        lock_path: PathBuf,
        timeout_ms: u64,
        reason: String,
    },

    #[error("Registry serialization/deserialization failed for {path:?}: {source}")]
    RegistrySerialization {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("Unsupported registry schema version {found}, maximum supported is {supported}")]
    UnsupportedSchemaVersion { found: u32, supported: u32 },

    #[error("Integrity check failed for skill '{id}': expected SHA-256 {expected}, got {actual}")]
    IntegrityMismatch {
        id: String,
        expected: String,
        actual: String,
    },

    #[error("Symlink creation failed: developer mode / unprivileged symlinks not available: {0}")]
    WindowsSymlinkPrivilegeRequired(String),

    #[error("Atomic directory swap failed from {staging:?} to {target:?}: {reason}. Rollback status: {rollback_status}")]
    AtomicSwapFailed {
        staging: PathBuf,
        target: PathBuf,
        reason: String,
        rollback_status: String,
    },

    #[error("Source skill path '{path:?}' is invalid: {reason}")]
    InvalidSourcePath { path: PathBuf, reason: String },
}

impl From<InstallerError> for SkillError {
    fn from(err: InstallerError) -> Self {
        match err {
            InstallerError::Io { path, source } => SkillError::Io { path, source },
            InstallerError::PathTraversal { path, boundary } => SkillError::PathTraversal {
                path,
                base: boundary,
            },
            InstallerError::InvalidSkillId { id } => SkillError::InvalidSkillName {
                name: id,
                reason: "Invalid skill ID characters or format".to_string(),
            },
            InstallerError::SkillAlreadyInstalled { id, existing_path } => {
                SkillError::DuplicateSkill {
                    name: id,
                    paths: vec![existing_path],
                }
            }
            InstallerError::SkillNotFound { id } => SkillError::NotFound { query: id },
            InstallerError::LockTimeout {
                lock_path,
                timeout_ms,
                ..
            } => SkillError::LockTimeout {
                path: lock_path,
                timeout_secs: (timeout_ms / 1000).max(1),
            },
            InstallerError::RegistrySerialization { path, source } => {
                SkillError::Json { path, source }
            }
            other => SkillError::validation(PathBuf::new(), other.to_string()),
        }
    }
}

// -----------------------------------------------------------------------------
// Internal Engines & Helpers
// -----------------------------------------------------------------------------

/// Cross-platform path resolver for agent runtime environments.
#[derive(Debug, Clone, Copy, Default)]
pub struct EnvironmentResolver;

impl EnvironmentResolver {
    /// Resolves the absolute root directory containing installed skills for the given environment.
    pub fn resolve_skills_dir(env: &TargetEnvironment) -> Result<PathBuf, InstallerError> {
        Self::resolve_skills_dir_with_root(env, None)
    }

    /// Resolves target directory with an optional base root override.
    pub fn resolve_skills_dir_with_root(
        env: &TargetEnvironment,
        root_override: Option<&Path>,
    ) -> Result<PathBuf, InstallerError> {
        if let Some(root) = root_override {
            return match env {
                TargetEnvironment::ClaudeDesktop => Ok(root.join(".claude").join("skills")),
                TargetEnvironment::Cursor => Ok(root.join(".cursor").join("skills")),
                TargetEnvironment::Antigravity { workspace_root } => {
                    if let Some(ws) = workspace_root {
                        Ok(ws.join(".agents").join("skills"))
                    } else {
                        Ok(root.join(".agents").join("skills"))
                    }
                }
                TargetEnvironment::Custom(p) => {
                    if p.is_absolute() {
                        Ok(p.clone())
                    } else {
                        Ok(root.join(p))
                    }
                }
            };
        }

        match env {
            TargetEnvironment::ClaudeDesktop => {
                #[cfg(target_os = "macos")]
                {
                    if let Some(home) = dirs::home_dir() {
                        Ok(home
                            .join("Library")
                            .join("Application Support")
                            .join("Claude")
                            .join("skills"))
                    } else {
                        Err(InstallerError::EnvironmentNotResolvable(env.clone()))
                    }
                }
                #[cfg(not(target_os = "macos"))]
                {
                    if let Some(config) = dirs::config_dir() {
                        Ok(config.join("claude").join("skills"))
                    } else if let Some(home) = dirs::home_dir() {
                        Ok(home.join(".claude").join("skills"))
                    } else {
                        Err(InstallerError::EnvironmentNotResolvable(env.clone()))
                    }
                }
            }
            TargetEnvironment::Cursor => {
                if let Some(home) = dirs::home_dir() {
                    Ok(home.join(".cursor").join("skills"))
                } else {
                    Err(InstallerError::EnvironmentNotResolvable(env.clone()))
                }
            }
            TargetEnvironment::Antigravity { workspace_root } => {
                if let Some(ws) = workspace_root {
                    Ok(ws.join(".agents").join("skills"))
                } else if let Some(home) = dirs::home_dir() {
                    Ok(home.join(".agents").join("skills"))
                } else {
                    Err(InstallerError::EnvironmentNotResolvable(env.clone()))
                }
            }
            TargetEnvironment::Custom(p) => {
                if p.is_absolute() {
                    Ok(p.clone())
                } else if let Some(home) = dirs::home_dir() {
                    Ok(home.join(p))
                } else {
                    Ok(p.clone())
                }
            }
        }
    }

    /// Resolves the canonical path to `installed-skills.json`.
    pub fn resolve_registry_path(env: &TargetEnvironment) -> Result<PathBuf, InstallerError> {
        Self::resolve_registry_path_with_root(env, None)
    }

    /// Resolves registry path with an optional base root override.
    pub fn resolve_registry_path_with_root(
        env: &TargetEnvironment,
        root_override: Option<&Path>,
    ) -> Result<PathBuf, InstallerError> {
        let skills_dir = Self::resolve_skills_dir_with_root(env, root_override)?;
        Ok(skills_dir.join("installed-skills.json"))
    }

    /// Resolves the canonical path to `installed-skills.lock`.
    pub fn resolve_lock_path(env: &TargetEnvironment) -> Result<PathBuf, InstallerError> {
        Self::resolve_lock_path_with_root(env, None)
    }

    /// Resolves lock path with an optional base root override.
    pub fn resolve_lock_path_with_root(
        env: &TargetEnvironment,
        root_override: Option<&Path>,
    ) -> Result<PathBuf, InstallerError> {
        let skills_dir = Self::resolve_skills_dir_with_root(env, root_override)?;
        Ok(skills_dir.join("installed-skills.lock"))
    }
}

/// Security validator guarding against path traversal and malicious identifier injections.
#[derive(Debug, Clone, Copy, Default)]
pub struct PathValidator;

impl PathValidator {
    /// Validates skill ID against strict ASCII alphanumeric + hyphen + underscore whitelist.
    pub fn validate_skill_id(id: &str) -> Result<(), InstallerError> {
        if id.is_empty() || id.len() > 64 {
            return Err(InstallerError::InvalidSkillId { id: id.to_string() });
        }

        // Check for reserved device names on Windows
        let upper = id.to_ascii_uppercase();
        let reserved = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];
        if reserved.contains(&upper.as_str()) {
            return Err(InstallerError::InvalidSkillId { id: id.to_string() });
        }

        // Check valid characters
        let is_valid = id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !is_valid || id.starts_with('.') || id.contains("..") {
            return Err(InstallerError::InvalidSkillId { id: id.to_string() });
        }

        Ok(())
    }

    /// Ensures that `path` is strictly contained within `boundary` without directory escape.
    pub fn ensure_within_boundary(path: &Path, boundary: &Path) -> Result<(), InstallerError> {
        let norm_path = normalize_path(path);
        let norm_boundary = normalize_path(boundary);

        if norm_path.starts_with(&norm_boundary) {
            Ok(())
        } else {
            Err(InstallerError::PathTraversal {
                path: path.to_path_buf(),
                boundary: boundary.to_path_buf(),
            })
        }
    }
}

/// Normalizes a path by resolving relative `.` and `..` components lexically.
#[must_use]
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            c => {
                normalized.push(c.as_os_str());
            }
        }
    }
    normalized
}

/// Scoped advisory file lock protecting shared environment registry files.
#[derive(Debug)]
pub struct FileLockGuard {
    lock_file_path: PathBuf,
    active: bool,
}

impl FileLockGuard {
    /// Acquires an exclusive advisory lock on `lock_path` with timeout and stale-lock recovery.
    pub fn acquire(lock_path: &Path, timeout: Duration) -> Result<Self, InstallerError> {
        let start = Instant::now();
        if let Some(parent) = lock_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let pid = std::process::id();
        let timeout_ms = u64::try_from(timeout.as_millis()).unwrap_or(u64::MAX);

        loop {
            let res = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(lock_path);

            match res {
                Ok(mut file) => {
                    let now_ts = timestamp_now_iso();
                    let lock_data = format!("pid={pid}\ntimestamp={now_ts}\n");
                    let _ = file.write_all(lock_data.as_bytes());
                    let _ = file.flush();
                    return Ok(Self {
                        lock_file_path: lock_path.to_path_buf(),
                        active: true,
                    });
                }
                Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                    if try_recover_stale_lock(lock_path) {
                        continue;
                    }

                    if start.elapsed() >= timeout {
                        return Err(InstallerError::LockTimeout {
                            lock_path: lock_path.to_path_buf(),
                            timeout_ms,
                            reason: "Timed out waiting for existing lock holder to release"
                                .to_string(),
                        });
                    }

                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(err) => {
                    return Err(InstallerError::Io {
                        path: lock_path.to_path_buf(),
                        source: err,
                    });
                }
            }
        }
    }

    /// Manually releases the lock.
    pub fn release(mut self) -> Result<(), InstallerError> {
        if self.active {
            self.active = false;
            if self.lock_file_path.exists() {
                fs::remove_file(&self.lock_file_path).map_err(|e| InstallerError::Io {
                    path: self.lock_file_path.clone(),
                    source: e,
                })?;
            }
        }
        Ok(())
    }
}

impl Drop for FileLockGuard {
    fn drop(&mut self) {
        if self.active && self.lock_file_path.exists() {
            let _ = fs::remove_file(&self.lock_file_path);
            self.active = false;
        }
    }
}

pub(crate) fn try_recover_stale_lock(lock_path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(lock_path) {
        let mut pid_opt: Option<u32> = None;
        let mut ts_secs_opt: Option<u64> = None;

        for line in content.lines() {
            if let Some(pid_str) = line.strip_prefix("pid=") {
                pid_opt = pid_str.trim().parse().ok();
            } else if let Some(ts_str) = line.strip_prefix("timestamp=") {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts_str.trim()) {
                    let now = chrono::Utc::now();
                    if let Ok(duration) = now
                        .signed_duration_since(dt.with_timezone(&chrono::Utc))
                        .to_std()
                    {
                        ts_secs_opt = Some(duration.as_secs());
                    }
                }
            }
        }

        let stale_timestamp = ts_secs_opt.is_some_and(|s| s > 30);

        let is_stale_by_pid = if let Some(pid) = pid_opt {
            #[cfg(target_os = "linux")]
            {
                !Path::new(&format!("/proc/{pid}")).exists()
            }
            #[cfg(unix)]
            #[cfg(not(target_os = "linux"))]
            {
                unsafe { libc::kill(pid as i32, 0) != 0 }
            }
            #[cfg(not(unix))]
            {
                false
            }
        } else {
            false
        };

        let stale_modified = if let Ok(meta) = fs::metadata(lock_path) {
            if let Ok(modified) = meta.modified() {
                SystemTime::now()
                    .duration_since(modified)
                    .is_ok_and(|elapsed| elapsed > Duration::from_secs(30))
            } else {
                false
            }
        } else {
            false
        };

        if stale_timestamp || is_stale_by_pid || stale_modified {
            let _ = fs::remove_file(lock_path);
            return true;
        }
    }
    false
}

/// Transactional directory swapper providing atomic directory swaps and rollback on failure.
#[derive(Debug)]
pub struct AtomicSwapCoordinator<'a> {
    pub skills_dir: &'a Path,
    pub skill_id: &'a str,
    pub target_path: PathBuf,
    pub staging_path: PathBuf,
    pub backup_path: Option<PathBuf>,
    pub swapped: bool,
}

impl<'a> AtomicSwapCoordinator<'a> {
    /// Creates a new swap coordinator for placing `staging_path` into `target_path`.
    pub fn new(skills_dir: &'a Path, skill_id: &'a str, staging_path: PathBuf) -> Self {
        let target_path = skills_dir.join(skill_id);
        Self {
            skills_dir,
            skill_id,
            target_path,
            staging_path,
            backup_path: None,
            swapped: false,
        }
    }

    /// Executes the directory swap, preserving the previous installation into `.backups/` if requested.
    pub fn execute_swap(&mut self, create_backup: bool) -> Result<(), InstallerError> {
        let target_exists =
            self.target_path.exists() || self.target_path.symlink_metadata().is_ok();

        if target_exists {
            let backup_dir = self.skills_dir.join(".backups");
            let _ = fs::create_dir_all(&backup_dir);
            let nonce = generate_nonce();
            let backup_dest = backup_dir.join(format!("{}_{nonce}", self.skill_id));

            if create_backup {
                if is_symlink(&self.target_path) {
                    let link_target =
                        fs::read_link(&self.target_path).map_err(|e| InstallerError::Io {
                            path: self.target_path.clone(),
                            source: e,
                        })?;
                    let meta_file = backup_dest.with_extension("symlink_info");
                    fs::write(&meta_file, link_target.to_string_lossy().as_bytes()).map_err(
                        |e| InstallerError::Io {
                            path: meta_file,
                            source: e,
                        },
                    )?;
                } else if self.target_path.is_dir() {
                    let _ = copy_dir_all(&self.target_path, &backup_dest, &[]);
                }
                self.backup_path = Some(backup_dest);
            }

            remove_dir_or_symlink_all(&self.target_path)?;
        }

        if let Err(err) = fs::rename(&self.staging_path, &self.target_path) {
            let rollback_status = if let Some(ref bk) = self.backup_path {
                if bk.is_dir() {
                    let _ = fs::rename(bk, &self.target_path);
                    "restored_from_backup"
                } else {
                    "backup_restoration_failed"
                }
            } else {
                "no_backup"
            };

            return Err(InstallerError::AtomicSwapFailed {
                staging: self.staging_path.clone(),
                target: self.target_path.clone(),
                reason: err.to_string(),
                rollback_status: rollback_status.to_string(),
            });
        }

        self.swapped = true;
        Ok(())
    }

    /// Commits the swap transaction.
    pub fn commit(mut self) {
        self.swapped = false; // Prevent rollback on drop
        if self.staging_path.exists() {
            let _ = remove_dir_or_symlink_all(&self.staging_path);
        }
    }

    /// Rolls back the swap, restoring previous target if backed up.
    pub fn rollback(mut self) {
        if self.swapped {
            let _ = remove_dir_or_symlink_all(&self.target_path);
            if let Some(ref bk) = self.backup_path {
                if bk.is_dir() {
                    let _ = fs::rename(bk, &self.target_path);
                }
            }
            self.swapped = false;
        }
        if self.staging_path.exists() {
            let _ = remove_dir_or_symlink_all(&self.staging_path);
        }
    }

    /// Returns the backup directory path if one was created.
    #[must_use]
    pub fn backup_path(&self) -> Option<&Path> {
        self.backup_path.as_deref()
    }
}

impl Drop for AtomicSwapCoordinator<'_> {
    fn drop(&mut self) {
        if self.staging_path.exists() {
            let _ = remove_dir_or_symlink_all(&self.staging_path);
        }
    }
}

// -----------------------------------------------------------------------------
// Core Engine: Installer & InstallerBuilder
// -----------------------------------------------------------------------------

/// Builder for constructing configured `Installer` instances.
#[derive(Debug, Clone, Default)]
pub struct InstallerBuilder {
    custom_root: Option<PathBuf>,
    default_options: Option<InstallOptions>,
}

impl InstallerBuilder {
    /// Creates a new builder instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets custom root override directory.
    #[must_use]
    pub fn custom_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.custom_root = Some(root.into());
        self
    }

    /// Sets default installation options.
    #[must_use]
    pub fn default_options(mut self, options: InstallOptions) -> Self {
        self.default_options = Some(options);
        self
    }

    /// Builds the `Installer`.
    #[must_use]
    pub fn build(self) -> Installer {
        Installer {
            custom_root: self.custom_root,
            default_options: self.default_options.unwrap_or_default(),
        }
    }
}

/// Primary orchestrator for skill installation and lifecycle management across agent environments.
#[derive(Debug, Clone)]
pub struct Installer {
    custom_root: Option<PathBuf>,
    default_options: InstallOptions,
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}

impl Installer {
    /// Returns a new builder instance.
    #[must_use]
    pub fn builder() -> InstallerBuilder {
        InstallerBuilder::new()
    }

    /// Creates an installer with default platform configurations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            custom_root: None,
            default_options: InstallOptions::default(),
        }
    }

    /// Creates an installer with an explicit root directory override.
    #[must_use]
    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self {
            custom_root: Some(root.into()),
            default_options: InstallOptions::default(),
        }
    }

    /// Resolves target directory for an environment, respecting builder root overrides.
    pub fn resolve_target_dir(&self, env: &TargetEnvironment) -> Result<PathBuf, InstallerError> {
        EnvironmentResolver::resolve_skills_dir_with_root(env, self.custom_root.as_deref())
    }

    /// Resolves registry path for an environment.
    pub fn resolve_registry_path(
        &self,
        env: &TargetEnvironment,
    ) -> Result<PathBuf, InstallerError> {
        EnvironmentResolver::resolve_registry_path_with_root(env, self.custom_root.as_deref())
    }

    /// Resolves lock path for an environment.
    pub fn resolve_lock_path(&self, env: &TargetEnvironment) -> Result<PathBuf, InstallerError> {
        EnvironmentResolver::resolve_lock_path_with_root(env, self.custom_root.as_deref())
    }

    /// Reads and deserializes `installed-skills.json` for the given environment.
    pub fn read_registry(
        &self,
        env: &TargetEnvironment,
    ) -> Result<InstalledSkillsRegistry, InstallerError> {
        let reg_path = self.resolve_registry_path(env)?;
        InstalledSkillsRegistry::load(&reg_path)
    }

    /// Atomically persists `InstalledSkillsRegistry` to disk.
    pub fn write_registry_atomic(
        &self,
        env: &TargetEnvironment,
        registry: &InstalledSkillsRegistry,
    ) -> Result<(), InstallerError> {
        let reg_path = self.resolve_registry_path(env)?;
        registry.save(&reg_path)
    }

    /// Installs a skill from `source_path` into `env` according to `options`.
    #[allow(clippy::too_many_lines)]
    pub fn install(
        &self,
        source_path: &Path,
        env: &TargetEnvironment,
        options: &InstallOptions,
    ) -> Result<InstallResult, InstallerError> {
        if !source_path.exists() {
            return Err(InstallerError::InvalidSourcePath {
                path: source_path.to_path_buf(),
                reason: "Source path does not exist".to_string(),
            });
        }
        if !source_path.is_dir() {
            return Err(InstallerError::InvalidSourcePath {
                path: source_path.to_path_buf(),
                reason: "Source path must be a directory".to_string(),
            });
        }

        // Verify that source contains SKILL.md
        let skill_md_path = source_path.join("SKILL.md");
        if !skill_md_path.exists() {
            return Err(InstallerError::InvalidSourcePath {
                path: source_path.to_path_buf(),
                reason: "Source directory does not contain SKILL.md".to_string(),
            });
        }

        // Extract metadata from SKILL.md
        let (skill_id, skill_name, skill_version) = extract_skill_metadata(source_path);
        PathValidator::validate_skill_id(&skill_id)?;

        let skills_dir = self.resolve_target_dir(env)?;
        fs::create_dir_all(&skills_dir).map_err(|e| InstallerError::Io {
            path: skills_dir.clone(),
            source: e,
        })?;

        let lock_path = self.resolve_lock_path(env)?;
        let _lock =
            FileLockGuard::acquire(&lock_path, Duration::from_millis(options.lock_timeout_ms))?;

        let target_path = skills_dir.join(&skill_id);
        PathValidator::ensure_within_boundary(&target_path, &skills_dir)?;

        let target_exists = target_path.exists() || target_path.symlink_metadata().is_ok();
        if target_exists && !options.force {
            return Err(InstallerError::SkillAlreadyInstalled {
                id: skill_id.clone(),
                existing_path: target_path,
            });
        }

        let canonical_source = source_path
            .canonicalize()
            .unwrap_or_else(|_| source_path.to_path_buf());

        // Determine effective mode
        let effective_mode = match options.mode {
            InstallMode::Auto => {
                #[cfg(unix)]
                {
                    InstallMode::Symlink
                }
                #[cfg(windows)]
                {
                    let nonce = generate_nonce();
                    let test_symlink = skills_dir.join(format!(".test_symlink_{nonce}"));
                    let symlink_ok = create_symlink(&canonical_source, &test_symlink).is_ok();
                    let _ = remove_dir_or_symlink_all(&test_symlink);
                    if symlink_ok {
                        InstallMode::Symlink
                    } else {
                        InstallMode::Copy
                    }
                }
                #[cfg(not(any(unix, windows)))]
                {
                    InstallMode::Copy
                }
            }
            mode => mode,
        };

        let mut backup_path_opt = None;
        let files: Vec<SkillFileEntry>;

        match effective_mode {
            InstallMode::Symlink => {
                if target_exists {
                    if options.create_backup {
                        let backup_dir = skills_dir.join(".backups");
                        let _ = fs::create_dir_all(&backup_dir);
                        let nonce = generate_nonce();
                        let backup_dest = backup_dir.join(format!("{skill_id}_{nonce}"));

                        if is_symlink(&target_path) {
                            let link_target =
                                fs::read_link(&target_path).map_err(|e| InstallerError::Io {
                                    path: target_path.clone(),
                                    source: e,
                                })?;
                            let meta_file = backup_dest.with_extension("symlink_info");
                            fs::write(&meta_file, link_target.to_string_lossy().as_bytes())
                                .map_err(|e| InstallerError::Io {
                                    path: meta_file,
                                    source: e,
                                })?;
                        } else if target_path.is_dir() {
                            let _ = copy_dir_all(&target_path, &backup_dest, &[]);
                        }
                        backup_path_opt = Some(backup_dest);
                    }
                    remove_dir_or_symlink_all(&target_path)?;
                }

                create_symlink(&canonical_source, &target_path)?;
                files = collect_file_entries(&canonical_source, &options.exclude_patterns)?;
            }
            InstallMode::Copy | InstallMode::Auto => {
                let nonce = generate_nonce();
                let staging_dir = skills_dir
                    .join(".staging")
                    .join(format!("{skill_id}_{nonce}"));

                files = copy_dir_all(&canonical_source, &staging_dir, &options.exclude_patterns)?;

                let mut coordinator =
                    AtomicSwapCoordinator::new(&skills_dir, &skill_id, staging_dir);
                coordinator.execute_swap(options.create_backup)?;
                backup_path_opt = coordinator.backup_path().map(Path::to_path_buf);
                coordinator.commit();
            }
        }

        let checksum = compute_dir_checksum(&files);
        let now_ts = timestamp_now_iso();

        let installed_skill = InstalledSkill {
            id: skill_id.clone(),
            name: skill_name,
            version: skill_version,
            source_path: canonical_source,
            target_path: target_path.clone(),
            mode: effective_mode,
            installed_at: now_ts.clone(),
            updated_at: now_ts,
            checksum,
            environment: env.clone(),
            files,
            active: true,
            metadata: HashMap::new(),
        };

        let mut registry = self.read_registry(env)?;
        registry.insert(installed_skill.clone());
        self.write_registry_atomic(env, &registry)?;

        Ok(InstallResult {
            skill: installed_skill,
            installed_mode: effective_mode,
            replaced_existing: target_exists,
            backup_path: backup_path_opt,
        })
    }

    /// Uninstalls a skill identified by `skill_id` from `env`.
    pub fn uninstall(
        &self,
        skill_id: &str,
        env: &TargetEnvironment,
        options: &UninstallOptions,
    ) -> Result<UninstallResult, InstallerError> {
        PathValidator::validate_skill_id(skill_id)?;

        let skills_dir = self.resolve_target_dir(env)?;
        let lock_path = self.resolve_lock_path(env)?;
        let _lock =
            FileLockGuard::acquire(&lock_path, Duration::from_millis(options.lock_timeout_ms))?;

        let mut registry = self.read_registry(env)?;
        let target_path = skills_dir.join(skill_id);
        let target_exists = target_path.exists() || target_path.symlink_metadata().is_ok();
        let in_registry = registry.get(skill_id).is_some();

        if !target_exists && !in_registry {
            return Err(InstallerError::SkillNotFound {
                id: skill_id.to_string(),
            });
        }

        let mut backup_path_opt = None;

        if target_exists {
            if options.create_backup {
                let backup_dir = skills_dir.join(".backups");
                let _ = fs::create_dir_all(&backup_dir);
                let nonce = generate_nonce();
                let backup_dest = backup_dir.join(format!("{skill_id}_uninstall_{nonce}"));

                if is_symlink(&target_path) {
                    let link_target =
                        fs::read_link(&target_path).map_err(|e| InstallerError::Io {
                            path: target_path.clone(),
                            source: e,
                        })?;
                    let meta_file = backup_dest.with_extension("symlink_info");
                    fs::write(&meta_file, link_target.to_string_lossy().as_bytes()).map_err(
                        |e| InstallerError::Io {
                            path: meta_file,
                            source: e,
                        },
                    )?;
                } else if target_path.is_dir() {
                    let _ = copy_dir_all(&target_path, &backup_dest, &[]);
                }
                backup_path_opt = Some(backup_dest);
            }

            remove_dir_or_symlink_all(&target_path)?;
        }

        if options.purge_state {
            registry.remove(skill_id);
        } else if let Some(skill) = registry.get_mut(skill_id) {
            skill.active = false;
            skill.updated_at = timestamp_now_iso();
        }

        self.write_registry_atomic(env, &registry)?;

        Ok(UninstallResult {
            skill_id: skill_id.to_string(),
            target_path,
            backup_path: backup_path_opt,
        })
    }

    /// Updates an existing installed skill from new `source_path` contents.
    pub fn update(
        &self,
        source_path: &Path,
        env: &TargetEnvironment,
        options: &InstallOptions,
    ) -> Result<UpdateResult, InstallerError> {
        let (skill_id, _, _) = extract_skill_metadata(source_path);
        PathValidator::validate_skill_id(&skill_id)?;

        let previous =
            self.get_skill(&skill_id, env)?
                .ok_or_else(|| InstallerError::SkillNotFound {
                    id: skill_id.clone(),
                })?;

        let mut update_opts = options.clone();
        update_opts.force = true;

        let install_res = self.install(source_path, env, &update_opts)?;

        Ok(UpdateResult {
            previous_version: previous.version,
            new_version: install_res.skill.version.clone(),
            skill: install_res.skill,
            backup_path: install_res.backup_path,
        })
    }

    /// Retrieves an installed skill record by ID from the environment registry.
    pub fn get_skill(
        &self,
        skill_id: &str,
        env: &TargetEnvironment,
    ) -> Result<Option<InstalledSkill>, InstallerError> {
        let registry = self.read_registry(env)?;
        Ok(registry.get(skill_id).cloned())
    }

    /// Lists all installed skills registered within the target environment.
    pub fn list_skills(
        &self,
        env: &TargetEnvironment,
    ) -> Result<Vec<InstalledSkill>, InstallerError> {
        let registry = self.read_registry(env)?;
        Ok(registry.skills.into_values().collect())
    }

    /// Sets the active status flag for an installed skill.
    pub fn set_active(
        &self,
        skill_id: &str,
        env: &TargetEnvironment,
        active: bool,
    ) -> Result<InstalledSkill, InstallerError> {
        let lock_path = self.resolve_lock_path(env)?;
        let _lock = FileLockGuard::acquire(
            &lock_path,
            Duration::from_millis(self.default_options.lock_timeout_ms),
        )?;

        let mut registry = self.read_registry(env)?;
        if let Some(skill) = registry.get_mut(skill_id) {
            skill.active = active;
            skill.updated_at = timestamp_now_iso();
            let updated = skill.clone();
            self.write_registry_atomic(env, &registry)?;
            Ok(updated)
        } else {
            Err(InstallerError::SkillNotFound {
                id: skill_id.to_string(),
            })
        }
    }

    /// Verifies the structural and cryptographic integrity of an installed skill bundle.
    pub fn verify(
        &self,
        skill_id: &str,
        env: &TargetEnvironment,
    ) -> Result<IntegrityStatus, InstallerError> {
        let skill =
            self.get_skill(skill_id, env)?
                .ok_or_else(|| InstallerError::SkillNotFound {
                    id: skill_id.to_string(),
                })?;

        if !skill.target_path.exists() && skill.target_path.symlink_metadata().is_err() {
            return Ok(IntegrityStatus::MissingFile {
                path: skill.target_path.to_string_lossy().to_string(),
            });
        }

        let mut expected_files = HashMap::new();
        for file_entry in &skill.files {
            expected_files.insert(file_entry.path.clone(), file_entry.sha256.clone());
            let full_path = skill.target_path.join(&file_entry.path);

            if !full_path.exists() {
                return Ok(IntegrityStatus::MissingFile {
                    path: file_entry.path.clone(),
                });
            }

            let actual_hash = compute_file_sha256(&full_path)?;
            if actual_hash != file_entry.sha256 {
                return Ok(IntegrityStatus::ChecksumMismatch {
                    expected: file_entry.sha256.clone(),
                    actual: actual_hash,
                });
            }
        }

        if skill.target_path.is_dir() {
            for e in walkdir::WalkDir::new(&skill.target_path)
                .follow_links(false)
                .into_iter()
                .flatten()
            {
                if e.file_type().is_file() {
                    if let Ok(rel) = e.path().strip_prefix(&skill.target_path) {
                        let rel_str = rel.to_string_lossy().replace('\\', "/");
                        if !expected_files.contains_key(&rel_str)
                            && !is_excluded(&rel_str, &default_exclude_patterns())
                        {
                            return Ok(IntegrityStatus::ExtraFile { path: rel_str });
                        }
                    }
                }
            }
        }

        Ok(IntegrityStatus::Valid)
    }

    /// Scans the target skills directory for unrecorded folders or broken symlinks and removes them.
    pub fn clean_orphans(&self, env: &TargetEnvironment) -> Result<Vec<PathBuf>, InstallerError> {
        let skills_dir = self.resolve_target_dir(env)?;
        if !skills_dir.exists() {
            return Ok(Vec::new());
        }

        let lock_path = self.resolve_lock_path(env)?;
        let _lock = FileLockGuard::acquire(
            &lock_path,
            Duration::from_millis(self.default_options.lock_timeout_ms),
        )?;

        let registry = self.read_registry(env)?;
        let mut cleaned = Vec::new();

        for entry in fs::read_dir(&skills_dir).map_err(|e| InstallerError::Io {
            path: skills_dir.clone(),
            source: e,
        })? {
            let entry = entry.map_err(|e| InstallerError::Io {
                path: skills_dir.clone(),
                source: e,
            })?;
            let p = entry.path();
            let fname = entry.file_name();
            let fname_str = fname.to_string_lossy();

            if fname_str == "installed-skills.json"
                || fname_str == "installed-skills.lock"
                || fname_str == ".backups"
                || fname_str == ".staging"
                || fname_str.starts_with('.')
            {
                continue;
            }

            if !registry.skills.contains_key(fname_str.as_ref()) {
                remove_dir_or_symlink_all(&p)?;
                cleaned.push(p);
            }
        }

        Ok(cleaned)
    }
}

// -----------------------------------------------------------------------------
// SkillInstaller Convenience Wrapper
// -----------------------------------------------------------------------------

/// Convenience installer bound to a specific target environment and directory configuration.
#[derive(Debug, Clone, TypedBuilder)]
pub struct SkillInstaller {
    #[builder(setter(into))]
    pub target: TargetEnvironment,
    #[builder(default)]
    pub mode: InstallMode,
    pub target_dir: PathBuf,
    pub registry_path: PathBuf,
    #[builder(default, setter(into))]
    pub backup_dir: Option<PathBuf>,
    #[builder(default = false)]
    pub overwrite: bool,
}

impl SkillInstaller {
    /// Creates a new `SkillInstaller` for a given target environment.
    pub fn new(target: TargetEnvironment) -> Result<Self, InstallerError> {
        let installer = Installer::new();
        let target_dir = installer.resolve_target_dir(&target)?;
        let registry_path = installer.resolve_registry_path(&target)?;
        Ok(Self {
            target,
            mode: InstallMode::Auto,
            target_dir,
            registry_path,
            backup_dir: None,
            overwrite: false,
        })
    }

    /// Creates a `SkillInstaller` anchored to a custom root base directory.
    #[must_use]
    pub fn with_root(target: TargetEnvironment, root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let installer = Installer::with_root(&root);
        let target_dir = installer
            .resolve_target_dir(&target)
            .unwrap_or_else(|_| root.join("skills"));
        let registry_path = target_dir.join("installed-skills.json");
        Self {
            target,
            mode: InstallMode::Auto,
            target_dir,
            registry_path,
            backup_dir: None,
            overwrite: false,
        }
    }

    /// Configures the installation mode.
    #[must_use]
    pub fn with_mode(mut self, mode: InstallMode) -> Self {
        self.mode = mode;
        self
    }

    /// Configures the target directory.
    #[must_use]
    pub fn with_target_dir(mut self, target_dir: impl Into<PathBuf>) -> Self {
        let td = target_dir.into();
        self.registry_path = td.join("installed-skills.json");
        self.target_dir = td;
        self
    }

    /// Configures the registry path.
    #[must_use]
    pub fn with_registry_path(mut self, registry_path: impl Into<PathBuf>) -> Self {
        self.registry_path = registry_path.into();
        self
    }

    /// Configures the backup directory.
    #[must_use]
    pub fn with_backup_dir(mut self, backup_dir: impl Into<PathBuf>) -> Self {
        self.backup_dir = Some(backup_dir.into());
        self
    }

    /// Configures whether to overwrite existing installations.
    #[must_use]
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Installs a skill from `skill_source`.
    pub fn install_skill(&self, skill_source: &Path) -> Result<InstalledSkill, InstallerError> {
        let installer = self.to_underlying_installer();
        let options = InstallOptions::builder()
            .mode(self.mode)
            .force(self.overwrite)
            .create_backup(self.backup_dir.is_some())
            .build();
        let res = installer.install(skill_source, &self.target, &options)?;
        Ok(res.skill)
    }

    /// Installs multiple skills in batch.
    pub fn install_many(
        &self,
        skill_sources: &[PathBuf],
    ) -> Result<Vec<InstalledSkill>, InstallerError> {
        let mut results = Vec::with_capacity(skill_sources.len());
        for src in skill_sources {
            results.push(self.install_skill(src)?);
        }
        Ok(results)
    }

    /// Uninstalls a skill by name.
    pub fn uninstall_skill(
        &self,
        skill_name: &str,
        create_backup: bool,
    ) -> Result<Option<InstalledSkill>, InstallerError> {
        let installer = self.to_underlying_installer();
        let existing = installer.get_skill(skill_name, &self.target)?;
        let options = UninstallOptions::builder()
            .create_backup(create_backup)
            .purge_state(true)
            .build();
        match installer.uninstall(skill_name, &self.target, &options) {
            Ok(_) => Ok(existing),
            Err(InstallerError::SkillNotFound { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Uninstalls multiple skills in batch.
    pub fn uninstall_many(
        &self,
        skill_names: &[&str],
        create_backup: bool,
    ) -> Result<Vec<InstalledSkill>, InstallerError> {
        let mut uninstalled = Vec::new();
        for name in skill_names {
            if let Some(skill) = self.uninstall_skill(name, create_backup)? {
                uninstalled.push(skill);
            }
        }
        Ok(uninstalled)
    }

    /// Lists all installed skills.
    pub fn list_installed(&self) -> Result<Vec<InstalledSkill>, InstallerError> {
        let installer = self.to_underlying_installer();
        installer.list_skills(&self.target)
    }

    /// Retrieves an installed skill by name.
    pub fn get_installed(
        &self,
        skill_name: &str,
    ) -> Result<Option<InstalledSkill>, InstallerError> {
        let installer = self.to_underlying_installer();
        installer.get_skill(skill_name, &self.target)
    }

    /// Sets the active flag for a skill.
    pub fn set_active(
        &self,
        skill_name: &str,
        active: bool,
    ) -> Result<InstalledSkill, InstallerError> {
        let installer = self.to_underlying_installer();
        installer.set_active(skill_name, &self.target, active)
    }

    /// Verifies the structural integrity of an installed skill.
    pub fn verify_installation(&self, skill_name: &str) -> Result<bool, InstallerError> {
        let installer = self.to_underlying_installer();
        match installer.verify(skill_name, &self.target)? {
            IntegrityStatus::Valid => Ok(true),
            _ => Ok(false),
        }
    }

    /// Cleans up orphaned directories or dangling symlinks.
    pub fn clean_orphans(&self, env: &TargetEnvironment) -> Result<Vec<PathBuf>, InstallerError> {
        let installer = self.to_underlying_installer();
        installer.clean_orphans(env)
    }

    fn to_underlying_installer(&self) -> Installer {
        Installer::builder()
            .custom_root(self.target_dir.parent().unwrap_or(&self.target_dir))
            .build()
    }
}

// -----------------------------------------------------------------------------
// Standalone Helper Functions
// -----------------------------------------------------------------------------

/// Computes hex SHA-256 digest of a byte slice.
#[must_use]
pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Computes hex SHA-256 digest of a file on disk.
pub fn compute_file_sha256(path: &Path) -> Result<String, InstallerError> {
    let mut file = File::open(path).map_err(|e| InstallerError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = file.read(&mut buffer).map_err(|e| InstallerError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Computes cumulative hex SHA-256 checksum over sorted file entries.
#[must_use]
pub fn compute_dir_checksum(entries: &[SkillFileEntry]) -> String {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    let mut hasher = Sha256::new();
    for entry in sorted {
        hasher.update(entry.path.as_bytes());
        hasher.update(b":");
        hasher.update(entry.sha256.as_bytes());
        hasher.update(b"\n");
    }
    format!("{:x}", hasher.finalize())
}

/// Recursively scans directory and collects file entries with SHA-256 digests.
pub fn collect_file_entries(
    root: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<SkillFileEntry>, InstallerError> {
    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(root).follow_links(false) {
        let entry = entry.map_err(|e| InstallerError::Io {
            path: root.to_path_buf(),
            source: e.into(),
        })?;

        if entry.file_type().is_file() {
            let path = entry.path();
            if let Ok(rel) = path.strip_prefix(root) {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                if is_excluded(&rel_str, exclude_patterns) {
                    continue;
                }

                let sha256 = compute_file_sha256(path)?;
                let metadata = entry.metadata().map_err(|e| InstallerError::Io {
                    path: path.to_path_buf(),
                    source: e.into(),
                })?;

                #[cfg(unix)]
                let permissions_mode = {
                    use std::os::unix::fs::PermissionsExt;
                    Some(metadata.permissions().mode())
                };
                #[cfg(not(unix))]
                let permissions_mode = None;

                entries.push(SkillFileEntry {
                    path: rel_str,
                    sha256,
                    size_bytes: metadata.len(),
                    permissions_mode,
                });
            }
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

/// Recursively copies directory tree filtering exclusions and calculating file hashes.
pub fn copy_dir_all(
    src: &Path,
    dst: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<SkillFileEntry>, InstallerError> {
    fs::create_dir_all(dst).map_err(|e| InstallerError::Io {
        path: dst.to_path_buf(),
        source: e,
    })?;

    let mut entries = Vec::new();

    for entry in walkdir::WalkDir::new(src).follow_links(false) {
        let entry = entry.map_err(|e| InstallerError::Io {
            path: src.to_path_buf(),
            source: e.into(),
        })?;

        let rel = entry.path().strip_prefix(src).unwrap_or(entry.path());
        let rel_str = rel.to_string_lossy().replace('\\', "/");

        if rel_str.is_empty() {
            continue;
        }

        if is_excluded(&rel_str, exclude_patterns) {
            continue;
        }

        let target_file = dst.join(rel);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_file).map_err(|e| InstallerError::Io {
                path: target_file.clone(),
                source: e,
            })?;
        } else if entry.file_type().is_symlink() {
            let link_target = fs::read_link(entry.path()).map_err(|e| InstallerError::Io {
                path: entry.path().to_path_buf(),
                source: e,
            })?;
            create_symlink(&link_target, &target_file)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target_file.parent() {
                fs::create_dir_all(parent).map_err(|e| InstallerError::Io {
                    path: parent.to_path_buf(),
                    source: e,
                })?;
            }

            fs::copy(entry.path(), &target_file).map_err(|e| InstallerError::Io {
                path: target_file.clone(),
                source: e,
            })?;

            let sha256 = compute_file_sha256(&target_file)?;
            let metadata = entry.metadata().map_err(|e| InstallerError::Io {
                path: entry.path().to_path_buf(),
                source: e.into(),
            })?;

            #[cfg(unix)]
            let permissions_mode = {
                use std::os::unix::fs::PermissionsExt;
                Some(metadata.permissions().mode())
            };
            #[cfg(not(unix))]
            let permissions_mode = None;

            entries.push(SkillFileEntry {
                path: rel_str,
                sha256,
                size_bytes: metadata.len(),
                permissions_mode,
            });
        }
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

/// Creates a cross-platform directory or file symlink.
pub fn create_symlink(src: &Path, dst: &Path) -> Result<(), InstallerError> {
    if let Some(parent) = dst.parent() {
        let _ = fs::create_dir_all(parent);
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst).map_err(|e| InstallerError::Io {
            path: dst.to_path_buf(),
            source: e,
        })
    }
    #[cfg(windows)]
    {
        if src.is_dir() {
            std::os::windows::fs::symlink_dir(src, dst)
                .map_err(|e| InstallerError::WindowsSymlinkPrivilegeRequired(e.to_string()))
        } else {
            std::os::windows::fs::symlink_file(src, dst)
                .map_err(|e| InstallerError::WindowsSymlinkPrivilegeRequired(e.to_string()))
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(InstallerError::WindowsSymlinkPrivilegeRequired(
            "Symlinks not supported on current platform".into(),
        ))
    }
}

/// Checks whether path is a symbolic link.
#[must_use]
pub fn is_symlink(path: &Path) -> bool {
    path.symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
}

/// Removes a directory, symlink, or file safely and recursively.
pub fn remove_dir_or_symlink_all(path: &Path) -> Result<(), InstallerError> {
    if is_symlink(path) {
        #[cfg(unix)]
        {
            fs::remove_file(path).map_err(|e| InstallerError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
        }
        #[cfg(windows)]
        {
            if path.is_dir() {
                fs::remove_dir(path).map_err(|e| InstallerError::Io {
                    path: path.to_path_buf(),
                    source: e,
                })?;
            } else {
                fs::remove_file(path).map_err(|e| InstallerError::Io {
                    path: path.to_path_buf(),
                    source: e,
                })?;
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            fs::remove_file(path).map_err(|e| InstallerError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
        }
    } else if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| InstallerError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
    } else if path.exists() {
        fs::remove_file(path).map_err(|e| InstallerError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
    }
    Ok(())
}

/// Checks if a relative path matches any exclusion glob patterns.
#[must_use]
pub fn is_excluded(rel_path: &str, exclude_patterns: &[String]) -> bool {
    let normalized = rel_path.trim_start_matches('/');
    for pattern in exclude_patterns {
        if pattern == normalized {
            return true;
        }
        if let Some(prefix) = pattern.strip_suffix("/**") {
            if normalized == prefix || normalized.starts_with(&format!("{prefix}/")) {
                return true;
            }
        }
        if let Some(suffix) = pattern.strip_prefix('*') {
            if normalized.ends_with(suffix) {
                return true;
            }
        }
        for component in normalized.split('/') {
            if component == pattern {
                return true;
            }
        }
    }
    false
}

/// Generates current RFC 3339 / ISO-8601 UTC timestamp string without external dependencies.
#[must_use]
pub fn timestamp_now_iso() -> String {
    Utc::now().to_rfc3339()
}

/// Generates a random alphanumeric nonce string.
#[must_use]
pub fn generate_nonce() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}_{}", std::process::id(), now.as_nanos())
}

/// Extracts skill ID, name, and version from a skill source directory containing `SKILL.md`.
fn extract_skill_metadata(source_dir: &Path) -> (String, String, String) {
    let skill_md = source_dir.join("SKILL.md");
    if skill_md.exists() {
        if let Ok(content) = fs::read_to_string(&skill_md) {
            if let Ok((yaml_str, _)) = crate::parser::SkillParser::extract_frontmatter(&content) {
                if let Ok(fm) = serde_yaml::from_str::<crate::models::SkillFrontmatter>(yaml_str) {
                    let id = fm.name.clone();
                    let name = fm.name;
                    let version = fm
                        .metadata
                        .as_ref()
                        .and_then(|m| m.get("version"))
                        .cloned()
                        .unwrap_or_else(|| "0.1.0".to_string());
                    return (id, name, version);
                }
            }
        }
    }

    let fallback_name = source_dir.file_name().map_or_else(
        || "unnamed-skill".to_string(),
        |n| n.to_string_lossy().to_string(),
    );

    (fallback_name.clone(), fallback_name, "0.1.0".to_string())
}
