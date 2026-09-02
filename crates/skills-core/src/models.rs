//! Core domain models and data structures for Agent Skills.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use typed_builder::TypedBuilder;

/// All supported skill categories, matching standard directory structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkillCategory {
    Engineering,
    GameDevelopment,
    Planning,
    Review,
    Github,
    Reflection,
    Writing,
    Authoring,
    Tooling,
    InProgress,
    Deprecated,
    #[serde(untagged)]
    Custom(String),
}

impl SkillCategory {
    /// Returns the kebab-case folder name matching the category.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Engineering => "engineering",
            Self::GameDevelopment => "game-development",
            Self::Planning => "planning",
            Self::Review => "review",
            Self::Github => "github",
            Self::Reflection => "reflection",
            Self::Writing => "writing",
            Self::Authoring => "authoring",
            Self::Tooling => "tooling",
            Self::InProgress => "in-progress",
            Self::Deprecated => "deprecated",
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Returns the human-readable display title for this category.
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Self::Engineering => "Engineering",
            Self::GameDevelopment => "Game Development",
            Self::Planning => "Planning",
            Self::Review => "Review",
            Self::Github => "GitHub",
            Self::Reflection => "Reflection",
            Self::Writing => "Writing",
            Self::Authoring => "Authoring",
            Self::Tooling => "Tooling",
            Self::InProgress => "In Progress",
            Self::Deprecated => "Deprecated",
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Returns the descriptive summary of what skills in this category do.
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Engineering => "The core build, debug, and delivery loop.",
            Self::GameDevelopment => "Game engines and game-development workflows.",
            Self::Planning => "Turn ideas into specs, tickets, and multi-session plans.",
            Self::Review => "Review diffs, pull requests, and test plans.",
            Self::Github => "GitHub and git workflows via the gh CLI.",
            Self::Reflection => "Self-critique and review of your own work.",
            Self::Writing => "Proofreading and documentation polish.",
            Self::Authoring => "Create and maintain the skills themselves.",
            Self::Tooling => "Environments, CLIs, and agent conventions.",
            Self::InProgress => "Drafts not yet promoted.",
            Self::Deprecated => "Retired skills kept for reference.",
            Self::Custom(_) => "Custom user-defined category.",
        }
    }

    /// Returns `true` if this category represents a lifecycle bucket (`in-progress` or `deprecated`).
    #[must_use]
    pub fn is_lifecycle(&self) -> bool {
        matches!(self, Self::InProgress | Self::Deprecated)
    }

    /// Returns `true` if skills in this category are promoted to public indexes.
    #[must_use]
    pub fn is_promoted(&self) -> bool {
        !self.is_lifecycle()
    }

    /// Returns an iterator over all standard promoted categories.
    #[must_use]
    pub fn standard_promoted() -> &'static [Self] {
        &[
            Self::Engineering,
            Self::GameDevelopment,
            Self::Planning,
            Self::Review,
            Self::Github,
            Self::Reflection,
            Self::Writing,
            Self::Authoring,
            Self::Tooling,
        ]
    }

    /// Returns an iterator over all standard lifecycle categories.
    #[must_use]
    pub fn standard_lifecycle() -> &'static [Self] {
        &[Self::InProgress, Self::Deprecated]
    }
}

impl fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for SkillCategory {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "engineering" => Self::Engineering,
            "game-development" => Self::GameDevelopment,
            "planning" => Self::Planning,
            "review" => Self::Review,
            "github" => Self::Github,
            "reflection" => Self::Reflection,
            "writing" => Self::Writing,
            "authoring" => Self::Authoring,
            "tooling" => Self::Tooling,
            "in-progress" => Self::InProgress,
            "deprecated" => Self::Deprecated,
            other => Self::Custom(other.to_string()),
        })
    }
}

/// Skill frontmatter parsed from YAML header in `SKILL.md`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SkillFrontmatter {
    /// Skill identifier (must match directory name).
    #[builder(setter(into))]
    pub name: String,

    /// Invocation trigger description and model-facing behavior.
    #[builder(setter(into))]
    pub description: String,

    /// Optional explicit category override (usually inferred from directory path).
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<SkillCategory>,

    /// Source URLs for vendored documentation resources.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,

    /// When true, strips invocation from autonomous agent model context.
    #[builder(default, setter(into))]
    #[serde(
        default,
        rename = "disable-model-invocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_model_invocation: Option<bool>,

    /// Command-line usage hint for skills accepting arguments.
    #[builder(default, setter(into))]
    #[serde(
        default,
        rename = "argument-hint",
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_hint: Option<String>,

    /// Execution context strategy, e.g. `"fork"`.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Target subagent type when forked, e.g. `"general-purpose"`.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// Provenance metadata key-value map (e.g. source, license).
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Arbitrary additional frontmatter fields preserved across round-trips.
    #[builder(default, setter(into))]
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// Alias for backwards compatibility with legacy TypeScript typing.
pub type SkillMetadata = SkillFrontmatter;

/// Resource origin kind within a skill's `resources/` folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceKind {
    /// Automatically fetched and managed by downloader from `resources:` URLs.
    Auto,
    /// Hand-authored static documentation, scripts, and ADRs.
    Manual,
}

/// A discovered resource file within a skill directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct ResourceFile {
    /// Relative path inside the skill directory (e.g. `resources/manual/CONTEXT.md`).
    #[builder(setter(into))]
    pub relative_path: PathBuf,

    /// Full filesystem path on disk.
    #[builder(setter(into))]
    pub absolute_path: PathBuf,

    /// Resource classification (Auto vs Manual).
    pub kind: ResourceKind,

    /// Optional cached text content.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Comprehensive representation of an Agent Skill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Skill {
    /// Relative path from repository root (e.g. `skills/engineering/domain-modeling/SKILL.md`).
    #[builder(setter(into))]
    pub path: PathBuf,

    /// Basename of the directory containing the skill (e.g. `domain-modeling`).
    #[builder(setter(into))]
    pub dir_name: String,

    /// Inferred or explicit category.
    pub category: SkillCategory,

    /// Whether this skill is promoted to top-level catalogs.
    pub promoted: bool,

    /// Parsed frontmatter structure.
    pub frontmatter: SkillFrontmatter,

    /// Markdown body content after the YAML frontmatter delimiter.
    #[builder(setter(into))]
    pub content: String,

    /// Raw unparsed file content including frontmatter.
    #[builder(setter(into))]
    pub raw: String,

    /// Attached or discovered resources.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceFile>,

    /// Recorded YAML error string if parsed in lenient/recovery mode.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yaml_error: Option<String>,
}

impl Skill {
    /// Returns the skill name from frontmatter.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.frontmatter.name
    }

    /// Returns the skill description from frontmatter.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.frontmatter.description
    }

    /// Returns `true` if model invocation is disabled (user-invoked only).
    #[must_use]
    pub fn is_user_invoked(&self) -> bool {
        self.frontmatter.disable_model_invocation.unwrap_or(false)
    }

    /// Returns `true` if this skill requires execution in a subagent fork.
    #[must_use]
    pub fn is_forked(&self) -> bool {
        self.frontmatter.context.as_deref() == Some("fork")
    }
}

/// Grouping item within `skills.sh.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SkillGrouping {
    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub description: String,

    #[builder(default, setter(into))]
    #[serde(default)]
    pub skills: Vec<String>,
}

/// Representation of the `skills.sh.json` manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct SkillsManifest {
    #[builder(default = "https://skills.sh/schemas/skills.sh.schema.json".to_string(), setter(into))]
    #[serde(rename = "$schema")]
    pub schema: String,

    #[builder(default, setter(into))]
    #[serde(
        default,
        rename = "notGrouped",
        skip_serializing_if = "Option::is_none"
    )]
    pub not_grouped: Option<String>,

    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groupings: Vec<SkillGrouping>,
}

/// Alias for `skills.sh.json` manifest.
pub type SkillsShJson = SkillsManifest;

/// Severity level for lint findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LintSeverity {
    Warning,
    Error,
}

impl fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Structured lint violation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct LintIssue {
    /// Relative or absolute path to the file with the violation.
    #[builder(setter(into))]
    pub file: PathBuf,

    /// Line number (1-indexed), if applicable.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// Column number (1-indexed), if applicable.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,

    /// Machine-readable rule identifier (e.g. `"naming-kebab-case"`, `"no-em-dashes"`).
    #[builder(setter(into))]
    pub rule: String,

    /// Human-readable explanation and remediation hint.
    #[builder(setter(into))]
    pub message: String,

    /// Severity level.
    pub severity: LintSeverity,
}

/// Aggregated report of lint violations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LintReport {
    pub issues: Vec<LintIssue>,
}

impl LintReport {
    /// Creates an empty lint report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a lint issue to the report.
    pub fn add(&mut self, issue: LintIssue) {
        self.issues.push(issue);
    }

    /// Adds an error issue.
    pub fn add_error(
        &mut self,
        file: impl Into<PathBuf>,
        rule: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(
            LintIssue::builder()
                .file(file)
                .rule(rule)
                .message(message)
                .severity(LintSeverity::Error)
                .build(),
        );
    }

    /// Adds a warning issue.
    pub fn add_warning(
        &mut self,
        file: impl Into<PathBuf>,
        rule: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(
            LintIssue::builder()
                .file(file)
                .rule(rule)
                .message(message)
                .severity(LintSeverity::Warning)
                .build(),
        );
    }

    /// Returns `true` if any error-level issues are present.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|i| i.severity == LintSeverity::Error)
    }

    /// Total count of error-level issues.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == LintSeverity::Error)
            .count()
    }

    /// Total count of warning-level issues.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == LintSeverity::Warning)
            .count()
    }
}

/// Options controlling the synchronization workflow.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SyncOptions {
    /// Filter to sync only skills within a specific category.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Filter to sync only a specific skill (`<name>` or `<category>/<name>`).
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,

    /// When true, skips host tool machine symlinking and only updates repo docs.
    #[builder(default)]
    #[serde(default)]
    pub repo_only: bool,

    /// When true, skips regenerating Zola dashboard content.
    #[builder(default)]
    #[serde(default)]
    pub skip_dashboard: bool,

    /// When true, skips staging generated files in Git.
    #[builder(default)]
    #[serde(default)]
    pub no_stage: bool,
}

/// Summary report returned after a synchronization run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SyncResult {
    /// Total number of active skills processed.
    #[builder(default)]
    pub skills_count: usize,

    /// Whether `agents/AGENTS.md` index was updated.
    #[builder(default)]
    pub agents_updated: bool,

    /// Whether `README.md` catalog was updated.
    #[builder(default)]
    pub readme_updated: bool,

    /// Whether `skills.sh.json` manifest was updated.
    #[builder(default)]
    pub manifest_updated: bool,

    /// Count of Zola markdown pages generated for the dashboard.
    #[builder(default)]
    pub dashboard_pages_generated: usize,

    /// Files staged into Git index.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub staged_files: Vec<PathBuf>,
}

/// Detected agent tooling on the host machine.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[allow(clippy::struct_excessive_bools)]
pub struct DetectedTools {
    pub claude: bool,
    pub opencode: bool,
    pub goose: bool,
    pub antigravity: bool,
}

/// Summary report returned after linking skills into host agent environments.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct LinkResult {
    pub skills_count: usize,
    pub linked: usize,
    pub removed: usize,
    pub conflicts: usize,
    pub tools: DetectedTools,
    #[builder(default, setter(into))]
    pub targets: Vec<PathBuf>,
}

/// Summary report returned after unlinking skills from host agent environments.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct UnlinkResult {
    pub removed: usize,
}
