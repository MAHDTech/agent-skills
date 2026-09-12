//! Skill workspace telemetry, catalog metrics, and dashboard aggregation engine.
//!
//! Provides the primary coordinator [`DashboardEngine`] and associated domain models
//! for aggregating skill catalog metrics, context token footprints, static analysis
//! health scores, category distributions, and agent target installation statuses.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::error::Result;
use crate::installer::{EnvironmentResolver, InstalledSkillsRegistry, TargetEnvironment};
use crate::lint::SkillLinter;
use crate::models::{LintIssue, LintReport, LintSeverity, Skill, SkillCategory};
use crate::parser::SkillParser;

// -----------------------------------------------------------------------------
// Domain Models
// -----------------------------------------------------------------------------

/// Comprehensive telemetry summary across the entire skill catalog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct DashboardSummary {
    /// Total number of skills discovered in the catalog.
    pub total_skills: usize,
    /// Number of active skills excluding deprecated items.
    pub active_skills: usize,
    /// Number of inactive skills belonging to the deprecated category.
    pub inactive_skills: usize,
    /// Number of skills promoted to top-level indexes.
    pub promoted_skills: usize,
    /// Number of skills in lifecycle categories: in-progress or deprecated.
    pub lifecycle_skills: usize,
    /// Number of skills requiring direct user invocation.
    pub user_invoked_skills: usize,
    /// Number of skills enabling autonomous model invocation.
    pub model_invoked_skills: usize,
    /// Number of skills executed within an isolated subagent fork.
    pub forked_skills: usize,
    /// Aggregate token count across all skills and attached resources.
    pub total_tokens: usize,
    /// Arithmetic mean of prompt tokens across all skills.
    pub avg_prompt_tokens: usize,
    /// Minimum prompt token count across all skills.
    pub min_prompt_tokens: usize,
    /// Maximum prompt token count across all skills.
    pub max_prompt_tokens: usize,
    /// Diagnostic health score and static analysis quality summary.
    pub health: HealthScore,
    /// Skill counts and token distributions grouped by category.
    pub categories: Vec<CategoryDistribution>,
    /// Installation metrics across supported agent execution environments.
    pub targets: Vec<TargetDistribution>,
    /// Granular metric records for each individual skill.
    pub skill_metrics: Vec<SkillMetric>,
}

/// Detailed telemetry and static analysis metrics for an individual skill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct SkillMetric {
    /// Unique identifier matching the skill directory name.
    #[builder(setter(into))]
    pub id: String,
    /// Human-readable display name extracted from frontmatter.
    #[builder(setter(into))]
    pub name: String,
    /// Resolved catalog category.
    pub category: SkillCategory,
    /// Whether this skill is promoted to top-level indexes.
    pub promoted: bool,
    /// Whether invocation requires direct user execution.
    pub user_invoked: bool,
    /// Whether execution forks into an isolated subagent context.
    pub forked: bool,
    /// Combined character count of frontmatter description and instruction markdown.
    pub char_count: usize,
    /// Word count across frontmatter description and instruction markdown.
    pub word_count: usize,
    /// Estimated context tokens consumed by prompt instructions and description.
    pub prompt_tokens: usize,
    /// Aggregate tokens consumed by prompt instructions plus attached resource assets.
    pub total_tokens: usize,
    /// Number of attached resource assets.
    pub resource_count: usize,
    /// Total byte size accumulated across all attached resource assets.
    pub resource_bytes: u64,
    /// Number of static analysis errors associated with this skill.
    pub error_count: usize,
    /// Number of static analysis warnings associated with this skill.
    pub warning_count: usize,
    /// Diagnostic quality score bounded between 0.0 and 100.0.
    pub health_score: f64,
}

/// Aggregate metric distribution for a single skill category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct CategoryDistribution {
    /// Target category descriptor.
    pub category: SkillCategory,
    /// Total skills belonging to this category.
    pub count: usize,
    /// Proportion of catalog skills in this category bounded between 0.0 and 100.0.
    pub percentage: f64,
    /// Aggregate total tokens consumed by skills in this category.
    pub total_tokens: usize,
    /// Number of promoted skills in this category.
    pub promoted_count: usize,
    /// Number of lifecycle skills in this category.
    pub lifecycle_count: usize,
}

/// Overall workspace static analysis health assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct HealthScore {
    /// Mean quality score across all evaluated skills bounded between 0.0 and 100.0.
    pub score: f64,
    /// Aggregate static analysis errors across all evaluated skills.
    pub total_errors: usize,
    /// Aggregate static analysis warnings across all evaluated skills.
    pub total_warnings: usize,
    /// Number of skills free of both errors and warnings.
    pub clean_skills: usize,
    /// Proportion of clean skills bounded between 0.0 and 100.0.
    pub clean_percentage: f64,
}

/// Installation metrics for a specific agent execution environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct TargetDistribution {
    /// Agent execution environment descriptor.
    pub environment: TargetEnvironment,
    /// Number of skills registered in the environment.
    pub installed_count: usize,
    /// Number of actively enabled skills in the environment.
    pub active_count: usize,
    /// Ratio of installed skills to total catalog skills bounded between 0.0 and 100.0.
    pub percentage: f64,
}

/// Stateless telemetry engine that aggregates catalog metrics, diagnostics, and environment state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DashboardEngine;

// -----------------------------------------------------------------------------
// Engine Coordinator Implementation
// -----------------------------------------------------------------------------

#[allow(clippy::trivially_copy_pass_by_ref, clippy::unused_self)]
impl DashboardEngine {
    /// Creates a new dashboard engine instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Evaluates an in-memory slice of skills without pre-existing lint diagnostics or target registries.
    #[must_use]
    pub fn analyze(&self, skills: &[Skill]) -> DashboardSummary {
        self.analyze_full(skills, None, None)
    }

    /// Evaluates an in-memory slice of skills using a pre-computed lint report.
    #[must_use]
    pub fn analyze_with_lint(&self, skills: &[Skill], report: &LintReport) -> DashboardSummary {
        self.analyze_full(skills, Some(report), None)
    }

    /// Evaluates an in-memory slice of skills and inspects the specified target environments.
    #[must_use]
    pub fn analyze_with_targets(
        &self,
        skills: &[Skill],
        targets: &[TargetEnvironment],
    ) -> DashboardSummary {
        self.analyze_full(skills, None, Some(targets))
    }

    /// Fully analyzes in-memory skills with optional lint diagnostics and target environments.
    #[must_use]
    pub fn analyze_full(
        &self,
        skills: &[Skill],
        report: Option<&LintReport>,
        targets: Option<&[TargetEnvironment]>,
    ) -> DashboardSummary {
        Self::analyze_full_with_root(skills, report, targets, None)
    }

    /// Discovers all skills in a repository root, lints them, inspects standard target environments, and builds summary.
    pub fn analyze_repository(&self, root: impl AsRef<Path>) -> Result<DashboardSummary> {
        let root_path = root.as_ref();
        let skills = SkillParser::discover_skills(root_path)?;
        let report = SkillLinter::new().lint_all(&skills);
        let targets = TargetEnvironment::all_standard();
        Ok(self.analyze_repository_with_overrides(&skills, &report, &targets, Some(root_path)))
    }

    /// Internal evaluation pipeline accepting an optional root override for registry path resolution.
    #[must_use]
    pub fn analyze_repository_with_overrides(
        &self,
        skills: &[Skill],
        report: &LintReport,
        targets: &[TargetEnvironment],
        root_override: Option<&Path>,
    ) -> DashboardSummary {
        Self::analyze_full_with_root(skills, Some(report), Some(targets), root_override)
    }

    fn analyze_full_with_root(
        skills: &[Skill],
        report: Option<&LintReport>,
        targets: Option<&[TargetEnvironment]>,
        root_override: Option<&Path>,
    ) -> DashboardSummary {
        let total_skills = skills.len();

        let skill_metrics: Vec<SkillMetric> = skills
            .iter()
            .map(|skill| {
                let skill_issues: Vec<&LintIssue> = match report {
                    Some(r) => r
                        .issues
                        .iter()
                        .filter(|issue| issue_matches_skill(issue, skill))
                        .collect(),
                    None => Vec::new(),
                };
                calculate_skill_metric(skill, &skill_issues)
            })
            .collect();

        let active_skills = skills
            .iter()
            .filter(|s| s.category != SkillCategory::Deprecated)
            .count();
        let inactive_skills = skills
            .iter()
            .filter(|s| s.category == SkillCategory::Deprecated)
            .count();
        let promoted_skills = skills
            .iter()
            .filter(|s| s.promoted || s.category.is_promoted())
            .count();
        let lifecycle_skills = skills.iter().filter(|s| s.category.is_lifecycle()).count();
        let user_invoked_skills = skills.iter().filter(|s| s.is_user_invoked()).count();
        let model_invoked_skills = skills.iter().filter(|s| !s.is_user_invoked()).count();
        let forked_skills = skills.iter().filter(|s| s.is_forked()).count();

        let total_tokens = skill_metrics
            .iter()
            .map(|m| m.total_tokens)
            .fold(0, usize::saturating_add);

        let sum_prompt_tokens: usize = skill_metrics
            .iter()
            .map(|m| m.prompt_tokens)
            .fold(0, usize::saturating_add);
        let avg_prompt_tokens = sum_prompt_tokens.checked_div(total_skills).unwrap_or(0);
        let (min_prompt_tokens, max_prompt_tokens) = if total_skills == 0 {
            (0, 0)
        } else {
            let min_prompt = skill_metrics
                .iter()
                .map(|m| m.prompt_tokens)
                .min()
                .unwrap_or(0);
            let max_prompt = skill_metrics
                .iter()
                .map(|m| m.prompt_tokens)
                .max()
                .unwrap_or(0);
            (min_prompt, max_prompt)
        };

        let health = calculate_health_score(&skill_metrics);
        let categories = calculate_category_distributions(&skill_metrics);
        let targets = targets.map_or_else(Vec::new, |t| {
            calculate_target_distributions_with_root(t, total_skills, root_override)
        });

        DashboardSummary::builder()
            .total_skills(total_skills)
            .active_skills(active_skills)
            .inactive_skills(inactive_skills)
            .promoted_skills(promoted_skills)
            .lifecycle_skills(lifecycle_skills)
            .user_invoked_skills(user_invoked_skills)
            .model_invoked_skills(model_invoked_skills)
            .forked_skills(forked_skills)
            .total_tokens(total_tokens)
            .avg_prompt_tokens(avg_prompt_tokens)
            .min_prompt_tokens(min_prompt_tokens)
            .max_prompt_tokens(max_prompt_tokens)
            .health(health)
            .categories(categories)
            .targets(targets)
            .skill_metrics(skill_metrics)
            .build()
    }
}

// -----------------------------------------------------------------------------
// Standalone Metric Calculation Functions
// -----------------------------------------------------------------------------

/// Estimates prompt context tokens from text content using the standard ceiling heuristic.
///
/// Formula: `(char_count + 3) / 4`, returning 0 for empty content.
#[must_use]
pub fn calculate_prompt_tokens(content: &str) -> usize {
    let char_count = content.chars().count();
    if char_count == 0 {
        0
    } else {
        char_count.saturating_add(3) / 4
    }
}

/// Calculates aggregate token count across skill instructions, description, and attached resources.
#[must_use]
pub fn calculate_total_tokens(skill: &Skill) -> usize {
    let char_count = skill
        .description()
        .chars()
        .count()
        .saturating_add(skill.content.chars().count());
    let prompt_tokens = if char_count == 0 {
        0
    } else {
        char_count.saturating_add(3) / 4
    };
    let resource_tokens: usize = skill
        .resources
        .iter()
        .map(|res| {
            res.content.as_deref().map_or_else(
                || {
                    fs::read_to_string(&res.absolute_path)
                        .map_or(0, |c| calculate_prompt_tokens(&c))
                },
                calculate_prompt_tokens,
            )
        })
        .fold(0, usize::saturating_add);
    prompt_tokens.saturating_add(resource_tokens)
}

/// Computes detailed metric statistics for an individual skill and its associated lint issues.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_skill_metric(skill: &Skill, issues: &[&LintIssue]) -> SkillMetric {
    let char_count = skill
        .description()
        .chars()
        .count()
        .saturating_add(skill.content.chars().count());
    let word_count = skill
        .description()
        .split_whitespace()
        .count()
        .saturating_add(skill.content.split_whitespace().count());
    let prompt_tokens = if char_count == 0 {
        0
    } else {
        char_count.saturating_add(3) / 4
    };
    let total_tokens = calculate_total_tokens(skill);

    let resource_count = skill.resources.len();
    let mut resource_bytes: u64 = 0;
    for res in &skill.resources {
        let bytes = if let Ok(metadata) = fs::metadata(&res.absolute_path) {
            metadata.len()
        } else if let Some(ref c) = res.content {
            c.len() as u64
        } else {
            0
        };
        resource_bytes = resource_bytes.saturating_add(bytes);
    }

    let error_count = issues
        .iter()
        .filter(|i| i.severity == LintSeverity::Error)
        .count();
    let warning_count = issues
        .iter()
        .filter(|i| i.severity == LintSeverity::Warning)
        .count();

    let penalty = (error_count as f64 * 20.0) + (warning_count as f64 * 5.0);
    let health_score = sanitize_score(100.0 - penalty);

    SkillMetric::builder()
        .id(skill.dir_name.clone())
        .name(skill.name().to_string())
        .category(skill.category.clone())
        .promoted(skill.promoted || skill.category.is_promoted())
        .user_invoked(skill.is_user_invoked())
        .forked(skill.is_forked())
        .char_count(char_count)
        .word_count(word_count)
        .prompt_tokens(prompt_tokens)
        .total_tokens(total_tokens)
        .resource_count(resource_count)
        .resource_bytes(resource_bytes)
        .error_count(error_count)
        .warning_count(warning_count)
        .health_score(health_score)
        .build()
}

/// Computes aggregate workspace health score and clean skill metrics from individual skill metrics.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_health_score(metrics: &[SkillMetric]) -> HealthScore {
    if metrics.is_empty() {
        return HealthScore::builder()
            .score(100.0)
            .total_errors(0)
            .total_warnings(0)
            .clean_skills(0)
            .clean_percentage(100.0)
            .build();
    }

    let total_errors: usize = metrics.iter().map(|m| m.error_count).sum();
    let total_warnings: usize = metrics.iter().map(|m| m.warning_count).sum();
    let clean_skills = metrics
        .iter()
        .filter(|m| m.error_count == 0 && m.warning_count == 0)
        .count();
    let clean_percentage =
        sanitize_percentage((clean_skills as f64 / metrics.len() as f64) * 100.0);
    let sum_scores: f64 = metrics.iter().map(|m| m.health_score).sum();
    let score = sanitize_score(sum_scores / metrics.len() as f64);

    HealthScore::builder()
        .score(score)
        .total_errors(total_errors)
        .total_warnings(total_warnings)
        .clean_skills(clean_skills)
        .clean_percentage(clean_percentage)
        .build()
}

/// Computes grouped, sorted category distributions from individual skill metrics.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_category_distributions(metrics: &[SkillMetric]) -> Vec<CategoryDistribution> {
    if metrics.is_empty() {
        return Vec::new();
    }

    let mut category_map: HashMap<SkillCategory, (usize, usize, usize, usize)> = HashMap::new();
    for m in metrics {
        let entry = category_map
            .entry(m.category.clone())
            .or_insert((0, 0, 0, 0));
        entry.0 += 1; // count
        entry.1 = entry.1.saturating_add(m.total_tokens); // total_tokens
        if m.promoted {
            entry.2 += 1; // promoted_count
        }
        if m.category.is_lifecycle() {
            entry.3 += 1; // lifecycle_count
        }
    }

    let total = metrics.len() as f64;
    let mut distributions: Vec<CategoryDistribution> = category_map
        .into_iter()
        .map(
            |(cat, (count, total_tokens, promoted_count, lifecycle_count))| {
                let percentage = sanitize_percentage((count as f64 / total) * 100.0);
                CategoryDistribution::builder()
                    .category(cat)
                    .count(count)
                    .percentage(percentage)
                    .total_tokens(total_tokens)
                    .promoted_count(promoted_count)
                    .lifecycle_count(lifecycle_count)
                    .build()
            },
        )
        .collect();

    distributions.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.category.title().cmp(b.category.title()))
            .then_with(|| a.category.as_str().cmp(b.category.as_str()))
    });

    distributions
}

/// Inspects target environments and computes installation distributions relative to total skills.
#[must_use]
pub fn calculate_target_distributions(
    targets: &[TargetEnvironment],
    total_skills: usize,
) -> Vec<TargetDistribution> {
    calculate_target_distributions_with_root(targets, total_skills, None)
}

/// Inspects target environments with an optional root path override for testing.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_target_distributions_with_root(
    targets: &[TargetEnvironment],
    total_skills: usize,
    root_override: Option<&Path>,
) -> Vec<TargetDistribution> {
    targets
        .iter()
        .map(|target| {
            let registry = load_target_registry(target, root_override);
            let installed_count = registry.skills.len();
            let active_count = registry.skills.values().filter(|s| s.active).count();
            let percentage = if total_skills == 0 {
                0.0
            } else {
                sanitize_percentage((installed_count as f64 / total_skills as f64) * 100.0)
            };

            TargetDistribution::builder()
                .environment(target.clone())
                .installed_count(installed_count)
                .active_count(active_count)
                .percentage(percentage)
                .build()
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Private Helper Routines
// -----------------------------------------------------------------------------

/// Determines if a lint issue belongs to a specific skill by comparing paths.
fn issue_matches_skill(issue: &LintIssue, skill: &Skill) -> bool {
    if issue.file == skill.path {
        return true;
    }
    if let Some(parent) = skill.path.parent() {
        if !parent.as_os_str().is_empty() && issue.file.starts_with(parent) {
            return true;
        }
    }
    issue.file.ends_with(&skill.path)
}

/// Clamps and sanitizes a percentage value to guaranteed [0.0, 100.0] interval.
fn sanitize_percentage(val: f64) -> f64 {
    if val.is_nan() || val.is_infinite() {
        0.0
    } else {
        val.clamp(0.0, 100.0)
    }
}

/// Clamps and sanitizes a health score value to guaranteed [0.0, 100.0] interval.
fn sanitize_score(val: f64) -> f64 {
    if val.is_nan() || val.is_infinite() {
        100.0
    } else {
        val.clamp(0.0, 100.0)
    }
}

/// Loads the target registry safely, falling back to an empty registry on missing file or read error.
fn load_target_registry(
    target: &TargetEnvironment,
    root_override: Option<&Path>,
) -> InstalledSkillsRegistry {
    let reg_path = EnvironmentResolver::resolve_registry_path_with_root(target, root_override)
        .unwrap_or_else(|_| {
            target
                .default_skills_dir(root_override)
                .join("installed-skills.json")
        });
    InstalledSkillsRegistry::load(&reg_path).unwrap_or_default()
}

// -----------------------------------------------------------------------------
// Unit Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    use crate::installer::InstalledSkill;
    use crate::models::{LintSeverity, SkillCategory, SkillFrontmatter};

    fn create_test_skill(
        dir_name: &str,
        category: SkillCategory,
        description: &str,
        content: &str,
    ) -> Skill {
        Skill::builder()
            .path(PathBuf::from(format!("skills/{dir_name}/SKILL.md")))
            .dir_name(dir_name.to_string())
            .category(category)
            .promoted(false)
            .frontmatter(
                SkillFrontmatter::builder()
                    .name(dir_name.to_string())
                    .description(description.to_string())
                    .build(),
            )
            .content(content.to_string())
            .raw(format!(
                "---\nname: {dir_name}\ndescription: {description}\n---\n{content}"
            ))
            .build()
    }

    #[test]
    fn test_empty_collection_safety() {
        let engine = DashboardEngine::new();
        let targets = vec![TargetEnvironment::ClaudeDesktop, TargetEnvironment::Cursor];
        let summary = engine.analyze_with_targets(&[], &targets);

        assert_eq!(summary.total_skills, 0);
        assert_eq!(summary.active_skills, 0);
        assert_eq!(summary.inactive_skills, 0);
        assert_eq!(summary.promoted_skills, 0);
        assert_eq!(summary.lifecycle_skills, 0);
        assert_eq!(summary.user_invoked_skills, 0);
        assert_eq!(summary.model_invoked_skills, 0);
        assert_eq!(summary.forked_skills, 0);
        assert_eq!(summary.total_tokens, 0);
        assert_eq!(summary.avg_prompt_tokens, 0);
        assert_eq!(summary.min_prompt_tokens, 0);
        assert_eq!(summary.max_prompt_tokens, 0);

        assert_eq!(summary.health.score, 100.0);
        assert_eq!(summary.health.clean_percentage, 100.0);
        assert_eq!(summary.health.clean_skills, 0);
        assert_eq!(summary.health.total_errors, 0);
        assert_eq!(summary.health.total_warnings, 0);

        assert!(summary.categories.is_empty());
        assert_eq!(summary.targets.len(), 2);
        for target in &summary.targets {
            assert_eq!(target.percentage, 0.0);
            assert_eq!(target.installed_count, 0);
            assert_eq!(target.active_count, 0);
        }
        assert!(summary.skill_metrics.is_empty());
    }

    #[test]
    fn test_single_skill_invariants() {
        let engine = DashboardEngine::new();
        let skill = create_test_skill(
            "test-skill",
            SkillCategory::Engineering,
            "Short description",
            "This is sample skill content for testing.",
        );

        let summary = engine.analyze(&[skill]);

        assert_eq!(summary.total_skills, 1);
        assert_eq!(summary.active_skills, 1);
        assert_eq!(summary.inactive_skills, 0);
        assert_eq!(summary.promoted_skills, 1); // Engineering is standard promoted
        assert_eq!(summary.lifecycle_skills, 0);
        assert_eq!(summary.user_invoked_skills, 0);
        assert_eq!(summary.model_invoked_skills, 1);

        assert_eq!(summary.min_prompt_tokens, summary.max_prompt_tokens);
        assert_eq!(summary.avg_prompt_tokens, summary.min_prompt_tokens);
        assert!(summary.total_tokens > 0);

        assert_eq!(summary.categories.len(), 1);
        assert_eq!(summary.categories[0].category, SkillCategory::Engineering);
        assert_eq!(summary.categories[0].count, 1);
        assert_eq!(summary.categories[0].percentage, 100.0);
    }

    #[test]
    fn test_token_ceiling_arithmetic() {
        assert_eq!(calculate_prompt_tokens(""), 0);
        assert_eq!(calculate_prompt_tokens("a"), 1);
        assert_eq!(calculate_prompt_tokens("abc"), 1);
        assert_eq!(calculate_prompt_tokens("abcd"), 1);
        assert_eq!(calculate_prompt_tokens("abcde"), 2);
        assert_eq!(calculate_prompt_tokens(&"x".repeat(1000)), 250);

        let mut skill = create_test_skill(
            "resource-skill",
            SkillCategory::Tooling,
            "1234", // 4 chars
            "5678", // 4 chars -> total 8 chars = (8 + 3)/4 = 2 prompt tokens
        );

        // Without resources
        assert_eq!(calculate_total_tokens(&skill), 2);

        // With attached in-memory resource
        skill.resources.push(crate::models::ResourceFile {
            relative_path: PathBuf::from("resources/ref.md"),
            absolute_path: PathBuf::from("/nonexistent/resources/ref.md"),
            kind: crate::models::ResourceKind::Manual,
            content: Some("1234".to_string()), // 4 chars -> (4 + 3)/4 = 1 token
        });

        // 2 + 1 = 3
        assert_eq!(calculate_total_tokens(&skill), 3);
    }

    #[test]
    fn test_health_score_penalties_and_clamping() {
        let skill = create_test_skill(
            "diagnostics-skill",
            SkillCategory::Writing,
            "desc",
            "content",
        );

        let err_issue = LintIssue::builder()
            .file(PathBuf::from("skills/diagnostics-skill/SKILL.md"))
            .rule("error-rule")
            .message("fatal issue")
            .severity(LintSeverity::Error)
            .build();

        let warn_issue = LintIssue::builder()
            .file(PathBuf::from("skills/diagnostics-skill/SKILL.md"))
            .rule("warn-rule")
            .message("warning issue")
            .severity(LintSeverity::Warning)
            .build();

        // 1 error (20.0) + 1 warning (5.0) -> 100.0 - 25.0 = 75.0
        let metric = calculate_skill_metric(&skill, &[&err_issue, &warn_issue]);
        assert_eq!(metric.error_count, 1);
        assert_eq!(metric.warning_count, 1);
        assert_eq!(metric.health_score, 75.0);

        // 6 errors -> 6 * 20 = 120 -> clamped to 0.0
        let errs = vec![&err_issue; 6];
        let clamped_metric = calculate_skill_metric(&skill, &errs);
        assert_eq!(clamped_metric.health_score, 0.0);

        // Aggregate health score
        let metrics = vec![metric, clamped_metric];
        let health = calculate_health_score(&metrics);
        assert_eq!(health.total_errors, 7);
        assert_eq!(health.total_warnings, 1);
        assert_eq!(health.clean_skills, 0);
        assert_eq!(health.clean_percentage, 0.0);
        // (75.0 + 0.0) / 2 = 37.5
        assert_eq!(health.score, 37.5);
    }

    #[test]
    fn test_category_distribution_grouping_and_sorting() {
        let s1 = create_test_skill("eng-1", SkillCategory::Engineering, "d", "c");
        let s2 = create_test_skill("eng-2", SkillCategory::Engineering, "d", "c");
        let s3 = create_test_skill("eng-3", SkillCategory::Engineering, "d", "c");
        let s4 = create_test_skill("wri-1", SkillCategory::Writing, "d", "c");
        let s5 = create_test_skill("wri-2", SkillCategory::Writing, "d", "c");
        let s6 = create_test_skill("plan-1", SkillCategory::Planning, "d", "c");

        let metrics = vec![
            calculate_skill_metric(&s1, &[]),
            calculate_skill_metric(&s2, &[]),
            calculate_skill_metric(&s3, &[]),
            calculate_skill_metric(&s4, &[]),
            calculate_skill_metric(&s5, &[]),
            calculate_skill_metric(&s6, &[]),
        ];

        let categories = calculate_category_distributions(&metrics);

        assert_eq!(categories.len(), 3);
        // 3 Engineering, 2 Writing, 1 Planning
        assert_eq!(categories[0].category, SkillCategory::Engineering);
        assert_eq!(categories[0].count, 3);
        assert_eq!(categories[1].category, SkillCategory::Writing);
        assert_eq!(categories[1].count, 2);
        assert_eq!(categories[2].category, SkillCategory::Planning);
        assert_eq!(categories[2].count, 1);

        let sum_pct: f64 = categories.iter().map(|c| c.percentage).sum();
        assert!((sum_pct - 100.0).abs() < 1e-6);

        // Secondary sorting by title ascending when counts are identical
        let t1 = create_test_skill("wri", SkillCategory::Writing, "d", "c");
        let t2 = create_test_skill("auth", SkillCategory::Authoring, "d", "c");
        let equal_metrics = vec![
            calculate_skill_metric(&t1, &[]),
            calculate_skill_metric(&t2, &[]),
        ];
        let sorted_equal = calculate_category_distributions(&equal_metrics);
        assert_eq!(sorted_equal[0].category, SkillCategory::Authoring);
        assert_eq!(sorted_equal[1].category, SkillCategory::Writing);
    }

    #[test]
    fn test_summary_json_roundtrip() {
        let engine = DashboardEngine::new();
        let skill = create_test_skill(
            "sample",
            SkillCategory::Tooling,
            "Sample description",
            "Sample markdown body",
        );
        let summary = engine.analyze(&[skill]);

        let json = serde_json::to_string(&summary).expect("serialization failed");
        let deserialized: DashboardSummary =
            serde_json::from_str(&json).expect("deserialization failed");

        assert_eq!(summary, deserialized);
    }

    #[test]
    fn test_target_distributions_with_mock_registry() {
        let temp = tempdir().expect("tempdir failed");
        let target_env = TargetEnvironment::Custom(temp.path().to_path_buf());
        let registry_path = temp.path().join("installed-skills.json");

        let mut reg = InstalledSkillsRegistry::new();
        reg.skills.insert(
            "active-skill".to_string(),
            InstalledSkill {
                id: "active-skill".to_string(),
                name: "Active Skill".to_string(),
                version: "0.1.0".to_string(),
                source_path: PathBuf::new(),
                target_path: PathBuf::new(),
                mode: crate::installer::InstallMode::Copy,
                installed_at: "2026-01-01T00:00:00Z".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                checksum: String::new(),
                environment: target_env.clone(),
                files: Vec::new(),
                active: true,
                metadata: HashMap::new(),
            },
        );
        reg.skills.insert(
            "inactive-skill".to_string(),
            InstalledSkill {
                id: "inactive-skill".to_string(),
                name: "Inactive Skill".to_string(),
                version: "0.1.0".to_string(),
                source_path: PathBuf::new(),
                target_path: PathBuf::new(),
                mode: crate::installer::InstallMode::Copy,
                installed_at: "2026-01-01T00:00:00Z".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                checksum: String::new(),
                environment: target_env.clone(),
                files: Vec::new(),
                active: false,
                metadata: HashMap::new(),
            },
        );

        let data = serde_json::to_string_pretty(&reg).expect("serialize registry failed");
        fs::write(&registry_path, data).expect("write registry failed");

        let targets = vec![target_env];
        let dist = calculate_target_distributions_with_root(&targets, 4, Some(temp.path()));

        assert_eq!(dist.len(), 1);
        assert_eq!(dist[0].installed_count, 2);
        assert_eq!(dist[0].active_count, 1);
        assert_eq!(dist[0].percentage, 50.0);
    }

    #[test]
    fn test_analyze_repository_full_pipeline() {
        let temp = tempdir().expect("tempdir failed");
        let skill_dir = temp.path().join("skills/engineering/demo-skill");
        fs::create_dir_all(&skill_dir).expect("create skill dir failed");

        let skill_content = r"---
name: demo-skill
description: A demo skill for testing repository discovery.
---
# Demo Skill

Instructions and details.
";
        fs::write(skill_dir.join("SKILL.md"), skill_content).expect("write SKILL.md failed");

        let engine = DashboardEngine::new();
        let summary = engine
            .analyze_repository(temp.path())
            .expect("analyze_repository failed");

        assert_eq!(summary.total_skills, 1);
        assert_eq!(summary.active_skills, 1);
        assert_eq!(summary.skill_metrics.len(), 1);
        assert_eq!(summary.skill_metrics[0].id, "demo-skill");
        assert_eq!(
            summary.skill_metrics[0].category,
            SkillCategory::Engineering
        );
        assert!(summary.total_tokens > 0);
    }
}
