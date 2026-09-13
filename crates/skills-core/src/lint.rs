// cspell:words unpromoted
//! Skill static analysis and linting engine.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;
use walkdir::WalkDir;

use crate::error::Result;
use crate::models::{LintIssue, LintReport, LintSeverity, Skill, SkillCategory, SkillTree};
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
        Self::check_archived_metadata(skill, &mut report);
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
        Self::check_archived_replaced_by(skills, &mut aggregated);
        Self::check_cross_references(skills, &mut aggregated);
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

    fn check_archived_metadata(skill: &Skill, report: &mut LintReport) {
        match skill.tree {
            SkillTree::Archive => {
                if let Some(ref date_str) = skill.archived {
                    if chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").is_err() {
                        report.add(
                            LintIssue::builder()
                                .file(skill.path.clone())
                                .rule("archived-metadata")
                                .message(format!(
                                    "Invalid archived date '{date_str}': expected ISO format YYYY-MM-DD"
                                ))
                                .severity(LintSeverity::Error)
                                .build(),
                        );
                    }
                } else {
                    report.add(
                        LintIssue::builder()
                            .file(skill.path.clone())
                            .rule("archived-metadata")
                            .message("Archived skill must declare 'metadata.archived' ISO date string (YYYY-MM-DD)")
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                }
            }
            SkillTree::Live => {
                if skill.archived.is_some() {
                    report.add(
                        LintIssue::builder()
                            .file(skill.path.clone())
                            .rule("archived-metadata")
                            .message("Live skill must not declare 'metadata.archived'; retired skills belong under skills-archive/")
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                }
            }
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
                let is_manual = rel_path
                    .parent()
                    .is_some_and(|p| p.components().any(|c| c.as_os_str() == "manual"));
                if is_manual {
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
                let has_live = skill_group.iter().any(|s| s.tree == SkillTree::Live);
                let has_archive = skill_group.iter().any(|s| s.tree == SkillTree::Archive);
                let paths_str = skill_group
                    .iter()
                    .map(|s| s.path.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                if has_live && has_archive {
                    report.add(
                        LintIssue::builder()
                            .file(skill_group[0].path.clone())
                            .rule("duplicate-skill-names")
                            .message(format!(
                                "Duplicate skill name '{name}' detected across trees (live and archive): [{paths_str}]"
                            ))
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                } else {
                    let promoted_count = skill_group.iter().filter(|s| s.promoted).count();
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
    }

    fn check_archived_replaced_by(skills: &[Skill], report: &mut LintReport) {
        let mut catalog: HashMap<&str, &Skill> = HashMap::new();
        for skill in skills {
            catalog.insert(&skill.dir_name, skill);
            catalog.insert(skill.name(), skill);
        }

        for skill in skills {
            if skill.tree != SkillTree::Archive {
                continue;
            }

            if let Some(ref replaced_by) = skill.replaced_by {
                if let Some(target) = catalog.get(replaced_by.as_str()) {
                    if target.tree != SkillTree::Live {
                        report.add(
                            LintIssue::builder()
                                .file(skill.path.clone())
                                .rule("archived-replaced-by")
                                .message(format!(
                                    "Archived skill replaces with archived skill '{replaced_by}'; replacement must be a live promoted skill"
                                ))
                                .severity(LintSeverity::Error)
                                .build(),
                        );
                    } else if !target.promoted {
                        report.add(
                            LintIssue::builder()
                                .file(skill.path.clone())
                                .rule("archived-replaced-by")
                                .message(format!(
                                    "Archived skill replaces with unpromoted skill '{replaced_by}'; replacement must be a live promoted skill"
                                ))
                                .severity(LintSeverity::Error)
                                .build(),
                        );
                    }
                } else {
                    report.add(
                        LintIssue::builder()
                            .file(skill.path.clone())
                            .rule("archived-replaced-by")
                            .message(format!(
                                "Archived skill replaces with unknown skill '{replaced_by}'"
                            ))
                            .severity(LintSeverity::Error)
                            .build(),
                    );
                }
            }
        }
    }

    #[allow(clippy::needless_range_loop, clippy::too_many_lines)]
    fn check_cross_references(skills: &[Skill], report: &mut LintReport) {
        let mut catalog: HashMap<&str, &Skill> = HashMap::new();
        for skill in skills {
            catalog.insert(&skill.dir_name, skill);
            catalog.insert(skill.name(), skill);
        }

        for source in skills {
            // Archived skills are exempt as sources
            if source.tree == SkillTree::Archive {
                continue;
            }

            // Universal router is exempt as source
            if source.name() == "skill-router" || source.dir_name == "skill-router" {
                continue;
            }

            let mut in_fence = false;

            for (line_idx, raw_line) in source.content.lines().enumerate() {
                let line_num = line_idx + 1;
                let trimmed = raw_line.trim();

                // Skip fenced code blocks (``` or ~~~)
                if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                    in_fence = !in_fence;
                    continue;
                }
                if in_fence {
                    continue;
                }

                // 1. Scan slash commands: /<identifier>
                let char_indices: Vec<(usize, char)> = raw_line.char_indices().collect();
                for i in 0..char_indices.len() {
                    let (_byte_pos, c) = char_indices[i];
                    if c == '/' {
                        // Check preceding character boundary
                        if i > 0 {
                            let (_, prev_c) = char_indices[i - 1];
                            if prev_c.is_ascii_alphanumeric()
                                || prev_c == '_'
                                || prev_c == '.'
                                || prev_c == '/'
                                || prev_c == '-'
                            {
                                continue;
                            }
                        }

                        // Collect candidate kebab-case name
                        let mut end = i + 1;
                        while end < char_indices.len() {
                            let (_, next_c) = char_indices[end];
                            if next_c.is_ascii_lowercase()
                                || next_c.is_ascii_digit()
                                || next_c == '-'
                            {
                                end += 1;
                            } else {
                                break;
                            }
                        }

                        if end > i + 1 {
                            let start_byte = char_indices[i + 1].0;
                            let end_byte = if end < char_indices.len() {
                                char_indices[end].0
                            } else {
                                raw_line.len()
                            };
                            let candidate_name = &raw_line[start_byte..end_byte];
                            let clean_name = candidate_name.trim_end_matches('-');

                            // Check succeeding character boundary
                            if end < char_indices.len() {
                                let (_, next_c) = char_indices[end];
                                if next_c.is_ascii_alphanumeric()
                                    || next_c == '_'
                                    || next_c == '.'
                                    || next_c == '/'
                                {
                                    continue;
                                }
                            }

                            if let Some(&target) = catalog.get(clean_name) {
                                Self::validate_cross_reference(
                                    source, target, clean_name, line_num, report,
                                );
                            }
                        }
                    }
                }

                // 2. Scan Markdown relative links: [text](target)
                let mut remaining = raw_line;
                while let Some(open_bracket) = remaining.find('[') {
                    let after_open = &remaining[open_bracket + 1..];
                    if let Some(close_bracket) = after_open.find(']') {
                        let after_close = &after_open[close_bracket + 1..];
                        if after_close.starts_with('(') {
                            if let Some(close_paren) = after_close.find(')') {
                                let link_url = after_close[1..close_paren].trim();
                                if !link_url.starts_with("http://")
                                    && !link_url.starts_with("https://")
                                    && !link_url.starts_with('#')
                                    && !link_url.starts_with("mailto:")
                                {
                                    let clean_url = link_url
                                        .split('#')
                                        .next()
                                        .unwrap_or(link_url)
                                        .split('?')
                                        .next()
                                        .unwrap_or(link_url);
                                    let path_obj = Path::new(clean_url);

                                    // Check if link target is a skill file or contains a skill directory
                                    for comp in path_obj.components() {
                                        let name_str = comp.as_os_str().to_string_lossy();
                                        let clean_name = name_str.as_ref();
                                        if let Some(&target) = catalog.get(clean_name) {
                                            if target.dir_name != source.dir_name
                                                && target.name() != source.name()
                                            {
                                                Self::validate_cross_reference(
                                                    source, target, clean_name, line_num, report,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                                remaining = &after_close[close_paren + 1..];
                                continue;
                            }
                        }
                    }
                    remaining = after_open;
                }
            }
        }
    }

    fn validate_cross_reference(
        source: &Skill,
        target: &Skill,
        target_name: &str,
        line_num: usize,
        report: &mut LintReport,
    ) {
        // Self-reference is valid
        if source.dir_name == target.dir_name || source.name() == target.name() {
            return;
        }

        // Live skills may never reference archived skills
        if target.tree == SkillTree::Archive {
            report.add(
                LintIssue::builder()
                    .file(source.path.clone())
                    .line(Some(line_num))
                    .rule("cross-reference-group")
                    .message(format!(
                        "Forbidden cross-reference to archived skill '{target_name}' from live skill '{}'",
                        source.name()
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
            return;
        }

        // Must share identical metadata.group
        let src_grp = source.group();
        let tgt_grp = target.group();

        if src_grp.is_none() || tgt_grp.is_none() || src_grp != tgt_grp {
            report.add(
                LintIssue::builder()
                    .file(source.path.clone())
                    .line(Some(line_num))
                    .rule("cross-reference-group")
                    .message(format!(
                        "Forbidden cross-reference from '{}' (group: {}) to '{target_name}' (group: {}): skills are self-contained by default and may only reference skills in the same declared 'metadata.group'",
                        source.name(),
                        src_grp.unwrap_or("none"),
                        tgt_grp.unwrap_or("none"),
                    ))
                    .severity(LintSeverity::Error)
                    .build(),
            );
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
