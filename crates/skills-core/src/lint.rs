//! Skill static analysis and linting engine.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;
use walkdir::WalkDir;

use crate::error::Result;
use crate::models::{LintIssue, LintReport, LintSeverity, Skill, SkillCategory};
use crate::parser::SkillParser;

/// Configurable static analysis and linting engine for Agent Skills.
#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct SkillLinter {
    /// Base repository root for relative path resolution and link target validation.
    #[builder(default, setter(into))]
    pub base_path: Option<PathBuf>,

    /// Whether to check relative file links against disk (default: true).
    #[builder(default = true)]
    pub check_relative_links: bool,

    /// Whether to check resources directory layout (default: true).
    #[builder(default = true)]
    pub check_resources: bool,
}

impl SkillLinter {
    /// Creates a new `SkillLinter` with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a linter configured with a base repository directory.
    #[must_use]
    pub fn with_base_path(base_path: impl Into<PathBuf>) -> Self {
        Self::builder().base_path(Some(base_path.into())).build()
    }

    /// Lints a single in-memory [`Skill`] instance.
    #[must_use]
    pub fn lint_skill(&self, skill: &Skill) -> LintReport {
        let mut report = LintReport::new();

        Self::check_yaml_errors(skill, &mut report);
        Self::check_frontmatter(skill, &mut report);
        Self::check_naming(skill, &mut report);
        Self::check_category(skill, &mut report);
        Self::check_em_dashes(&skill.raw, &skill.path, &mut report);
        self.check_links(skill, &mut report);
        Self::check_code_blocks(skill, &mut report);

        if self.check_resources {
            if let Some(parent) = skill.path.parent() {
                self.check_resources_layout(parent, &skill.path, &mut report);
            }
        }

        report
    }

    /// Parses and lints a skill file from disk at the specified path.
    pub fn lint_file(&self, path: impl AsRef<Path>) -> Result<LintReport> {
        let path_ref = path.as_ref();
        let parser = SkillParser::builder()
            .lenient(true)
            .load_resource_contents(true)
            .build();
        let skill = parser.parse_file_with_options(path_ref)?;
        Ok(self.lint_skill(&skill))
    }

    /// Lints a collection of skills and runs inter-skill cross-checks (such as duplicate name detection).
    #[must_use]
    pub fn lint_all(&self, skills: &[Skill]) -> LintReport {
        let mut aggregated = LintReport::new();

        for skill in skills {
            let report = self.lint_skill(skill);
            for issue in report.issues {
                aggregated.add(issue);
            }
        }

        Self::check_duplicate_names(skills, &mut aggregated);
        aggregated
    }

    /// Traverses a repository root directory, discovers all skills, and lints them.
    pub fn lint_repository(&self, root: impl AsRef<Path>) -> Result<LintReport> {
        let root_path = root.as_ref();
        let skills = SkillParser::discover_skills(root_path)?;
        let linter = Self::builder()
            .base_path(Some(root_path.to_path_buf()))
            .build();
        Ok(linter.lint_all(&skills))
    }

    // ------------------------------------------------------------------------
    // Rule Checkers
    // ------------------------------------------------------------------------

    fn check_yaml_errors(skill: &Skill, report: &mut LintReport) {
        if let Some(ref err) = skill.yaml_error {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("frontmatter-syntax")
                    .message(format!("Frontmatter YAML syntax error: {err}"))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }
    }

    fn check_frontmatter(skill: &Skill, report: &mut LintReport) {
        let fm = &skill.frontmatter;

        if fm.name.trim().is_empty() {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("frontmatter-required")
                    .message("Missing or empty mandatory frontmatter field 'name'")
                    .severity(LintSeverity::Error)
                    .build(),
            );
        } else if fm.name.len() > 64 {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("frontmatter-limits")
                    .message(format!(
                        "Skill name length ({}) exceeds maximum 64 characters",
                        fm.name.len()
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }

        if fm.description.trim().is_empty() {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("frontmatter-required")
                    .message("Missing or empty mandatory frontmatter field 'description'")
                    .severity(LintSeverity::Error)
                    .build(),
            );
        } else if fm.description.len() > 1024 {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("frontmatter-limits")
                    .message(format!(
                        "Skill description length ({}) exceeds maximum 1024 characters",
                        fm.description.len()
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }
    }

    fn check_naming(skill: &Skill, report: &mut LintReport) {
        let name = &skill.frontmatter.name;

        // Check directory matching
        if !skill.dir_name.is_empty() && skill.dir_name != "." && name != &skill.dir_name {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("naming-dir-mismatch")
                    .message(format!(
                        "Skill name '{name}' does not match enclosing directory name '{}'",
                        skill.dir_name
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }

        // Check kebab-case pattern
        let is_valid_kebab = !name.is_empty()
            && !name.starts_with('-')
            && !name.ends_with('-')
            && !name.contains("--")
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');

        if !is_valid_kebab && !name.is_empty() {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("naming-kebab-case")
                    .message(format!(
                        "Skill name '{name}' must be lowercase kebab-case (e.g. 'my-skill-name')"
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }

        // Check reserved words
        let lower = name.to_ascii_lowercase();
        if lower.contains("anthropic") || lower.contains("claude") {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("naming-no-reserved")
                    .message(format!(
                        "Skill name '{name}' cannot contain reserved words 'anthropic' or 'claude'"
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
        }
    }

    fn check_category(skill: &Skill, report: &mut LintReport) {
        if let SkillCategory::Custom(ref cat) = skill.category {
            report.add(
                LintIssue::builder()
                    .file(skill.path.clone())
                    .rule("valid-category")
                    .message(format!("Unrecognized or custom skill category '{cat}'"))
                    .severity(LintSeverity::Warning)
                    .build(),
            );
        }
    }

    /// Checks text content for prohibited Unicode U+2014 em-dash characters.
    pub fn check_em_dashes(content: &str, file: &Path, report: &mut LintReport) {
        for (line_num, line) in (1..).zip(content.split('\n')) {
            if line.contains('\u{2014}') {
                for (col_num, c) in (1..).zip(line.chars()) {
                    if c == '\u{2014}' {
                        report.add(
                            LintIssue::builder()
                                .file(file.to_path_buf())
                                .line(Some(line_num))
                                .column(Some(col_num))
                                .rule("no-em-dashes")
                                .message(
                                    "Contains em-dash (Unicode U+2014). Never use em-dashes. Use hyphens ('-'), colons, commas, or parentheses instead."
                                )
                                .severity(LintSeverity::Error)
                                .build(),
                        );
                    }
                }
            }
        }
    }

    fn check_links(&self, skill: &Skill, report: &mut LintReport) {
        let stripped = Self::strip_code_blocks(&skill.content);

        for (line_num, line) in (1..).zip(stripped.split('\n')) {
            // Check absolute file links: file:/// or absolute paths
            if line.contains("file:///") || line.contains("file://") {
                report.add(
                    LintIssue::builder()
                        .file(skill.path.clone())
                        .line(Some(line_num))
                        .rule("no-absolute-file-links")
                        .message(
                            "Absolute 'file://' links are prohibited. Use relative repository paths instead."
                        )
                        .severity(LintSeverity::Error)
                        .build(),
                );
            }

            // Check relative links if disk checking is enabled
            if self.check_relative_links {
                if let Some(parent) = skill.path.parent() {
                    let base_dir = if let Some(ref base) = self.base_path {
                        base.join(parent)
                    } else {
                        parent.to_path_buf()
                    };

                    Self::scan_relative_links(line, line_num, &skill.path, &base_dir, report);
                }
            }
        }
    }

    fn scan_relative_links(
        line: &str,
        line_num: usize,
        skill_file: &Path,
        base_dir: &Path,
        report: &mut LintReport,
    ) {
        let mut remaining = line;

        while let Some(start_bracket) = remaining.find('[') {
            let after_bracket = &remaining[start_bracket + 1..];
            if let Some(close_bracket) = after_bracket.find(']') {
                let after_close = &after_bracket[close_bracket + 1..];
                if after_close.starts_with('(') {
                    if let Some(close_paren) = after_close.find(')') {
                        let link_target = after_close[1..close_paren].trim();

                        if !link_target.is_empty()
                            && !link_target.starts_with("http://")
                            && !link_target.starts_with("https://")
                            && !link_target.starts_with("mailto:")
                            && !link_target.starts_with('#')
                            && !link_target.starts_with('/')
                            && !link_target.starts_with("@/")
                        {
                            let clean_target = link_target.split('#').next().unwrap_or(link_target);
                            if !clean_target.is_empty() {
                                let target_path = base_dir.join(clean_target);
                                if !target_path.exists() {
                                    report.add(
                                        LintIssue::builder()
                                            .file(skill_file.to_path_buf())
                                            .line(Some(line_num))
                                            .rule("broken-relative-links")
                                            .message(format!(
                                                "Broken relative link: referenced file '{clean_target}' does not exist on disk"
                                            ))
                                            .severity(LintSeverity::Error)
                                            .build(),
                                    );
                                }
                            }
                        }

                        remaining = &after_close[close_paren + 1..];
                        continue;
                    }
                }
            }
            remaining = after_bracket;
        }
    }

    fn check_code_blocks(skill: &Skill, report: &mut LintReport) {
        let mut in_fence = false;

        for (line_num, line) in (1..).zip(skill.content.split('\n')) {
            let trimmed = line.trim();

            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                let fence_marker = &trimmed[..3];
                let rest = trimmed[3..].trim();

                if in_fence {
                    // Closing fence
                    in_fence = false;
                } else {
                    // Opening fence
                    if rest.is_empty() {
                        report.add(
                            LintIssue::builder()
                                .file(skill.path.clone())
                                .line(Some(line_num))
                                .rule("fenced-code-blocks")
                                .message(format!(
                                    "Fenced code block opening with '{fence_marker}' is missing a language identifier"
                                ))
                                .severity(LintSeverity::Warning)
                                .build(),
                        );
                    }
                    in_fence = true;
                }
            }
        }
    }

    fn check_resources_layout(&self, skill_dir: &Path, skill_path: &Path, report: &mut LintReport) {
        let full_skill_dir = if let Some(ref base) = self.base_path {
            base.join(skill_dir)
        } else {
            skill_dir.to_path_buf()
        };

        let resources_dir = full_skill_dir.join("resources");
        if !resources_dir.exists() || !resources_dir.is_dir() {
            return;
        }

        // Scan direct entries inside `resources/`
        let Ok(entries) = fs::read_dir(&resources_dir) else {
            return;
        };

        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().to_string();

            // Ignore hidden files like .gitkeep
            if name.starts_with('.') {
                continue;
            }

            if file_type.is_file() {
                report.add(
                    LintIssue::builder()
                        .file(skill_path.to_path_buf())
                        .rule("resources-folder-structure")
                        .message(format!(
                            "Loose file '{name}' found directly in 'resources/'. All resource files must be placed in 'resources/auto/' or 'resources/manual/'."
                        ))
                        .severity(LintSeverity::Error)
                        .build(),
                );
            } else if file_type.is_dir() && name != "auto" && name != "manual" {
                report.add(
                    LintIssue::builder()
                        .file(skill_path.to_path_buf())
                        .rule("resources-folder-structure")
                        .message(format!(
                            "Disallowed directory 'resources/{name}/'. Only 'resources/auto/' and 'resources/manual/' are permitted."
                        ))
                        .severity(LintSeverity::Error)
                        .build(),
                );
            }
        }

        // Walk entire resources tree to check for index.md and scan manual files for em-dashes
        for entry in WalkDir::new(&resources_dir).follow_links(false) {
            let Ok(entry) = entry else { continue };

            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy();
                let lower_name = file_name.to_ascii_lowercase();

                if lower_name == "index.md" || lower_name == "_index.md" {
                    report.add(
                        LintIssue::builder()
                            .file(skill_path.to_path_buf())
                            .rule("resources-folder-structure")
                            .message(format!(
                                "Resource file '{file_name}' is forbidden to prevent site indexing collisions."
                            ))
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                }

                // Check manual resources for em-dashes
                let rel_path = entry
                    .path()
                    .strip_prefix(&full_skill_dir)
                    .unwrap_or(entry.path());
                let rel_str = rel_path.to_string_lossy();
                if rel_str.contains("resources/manual") || rel_str.contains("manual") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        Self::check_em_dashes(&content, entry.path(), report);
                    }
                }
            }
        }
    }

    fn check_duplicate_names(skills: &[Skill], report: &mut LintReport) {
        let mut names_map: HashMap<&str, Vec<&Skill>> = HashMap::new();

        for skill in skills {
            names_map.entry(&skill.dir_name).or_default().push(skill);
        }

        for (name, skill_group) in names_map {
            if skill_group.len() > 1 {
                let promoted_count = skill_group.iter().filter(|s| s.promoted).count();
                let paths_str = skill_group
                    .iter()
                    .map(|s| s.path.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                if promoted_count > 1 {
                    report.add(
                        LintIssue::builder()
                            .file(skill_group[0].path.clone())
                            .rule("duplicate-skill-names")
                            .message(format!(
                                "Duplicate skill name '{name}' detected across promoted categories: [{paths_str}]"
                            ))
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                } else {
                    report.add(
                        LintIssue::builder()
                            .file(skill_group[0].path.clone())
                            .rule("duplicate-skill-names")
                            .message(format!(
                                "Duplicate skill name '{name}' detected between promoted and lifecycle categories: [{paths_str}]"
                            ))
                            .severity(LintSeverity::Warning)
                            .build(),
                    );
                }
            }
        }
    }

    fn strip_code_blocks(content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_fence = false;

        for line in content.split('\n') {
            let trimmed = line.trim();
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_fence = !in_fence;
                result.push('\n');
                continue;
            }

            if in_fence {
                result.push('\n');
                continue;
            }

            // Strip inline backtick code spans
            let mut stripped_line = String::with_capacity(line.len());
            let mut in_inline = false;
            for c in line.chars() {
                if c == '`' {
                    in_inline = !in_inline;
                    stripped_line.push(' ');
                } else if in_inline {
                    stripped_line.push(' ');
                } else {
                    stripped_line.push(c);
                }
            }

            result.push_str(&stripped_line);
            result.push('\n');
        }

        result
    }
}
