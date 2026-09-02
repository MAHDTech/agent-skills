//! Skill markdown and YAML frontmatter parsing engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use typed_builder::TypedBuilder;
use walkdir::WalkDir;

use crate::error::{Result, SkillError};
use crate::models::{ResourceFile, ResourceKind, Skill, SkillCategory, SkillFrontmatter};

/// Represents a structured heading section in a skill markdown body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct MarkdownSection {
    /// Heading level (1 for `#`, 2 for `##`, ..., 6 for `######`). 0 indicates preamble before first heading.
    #[builder(default = 0)]
    pub level: usize,

    /// Sanitized section title text without leading `#` or trailing whitespace. Empty for preamble.
    #[builder(default, setter(into))]
    pub title: String,

    /// Markdown body content under this heading.
    #[builder(default, setter(into))]
    pub content: String,

    /// 1-indexed line number where the heading begins.
    #[builder(default = 1)]
    pub line: usize,
}

/// Represents a template placeholder token found within skill markdown content (e.g. `{{target_branch}}`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TypedBuilder)]
pub struct TemplatePlaceholder {
    /// Normalized parameter identifier without curly braces or whitespace (e.g. `"target_branch"`).
    #[builder(setter(into))]
    pub name: String,

    /// Optional default fallback value if provided via `{{name:-default}}` or `{{name:default}}`.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,

    /// Full raw matched token text (e.g. `"{{ target_branch }}"`).
    #[builder(setter(into))]
    pub raw: String,

    /// 1-indexed line number where the placeholder begins.
    #[builder(default = 1)]
    pub line: usize,

    /// 1-indexed column number (character offset) where the placeholder begins.
    #[builder(default = 1)]
    pub column: usize,

    /// 0-indexed byte offset in source text where the placeholder starts.
    #[builder(default = 0)]
    pub start: usize,

    /// 0-indexed byte offset in source text where the placeholder ends.
    #[builder(default = 0)]
    pub end: usize,
}

/// Fully parsed and analyzed skill document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct ParsedSkill {
    /// Deserialized YAML frontmatter header.
    pub frontmatter: SkillFrontmatter,

    /// Raw unparsed YAML frontmatter string (excluding `---` delimiters).
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_frontmatter: Option<String>,

    /// Markdown instruction body without YAML frontmatter.
    #[builder(setter(into))]
    pub body: String,

    /// Structured heading sections extracted from the markdown body.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<MarkdownSection>,

    /// Discovered template placeholders in the body.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub placeholders: Vec<TemplatePlaceholder>,

    /// Attached resources discovered on disk.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceFile>,
}

impl ParsedSkill {
    /// Returns the skill name declared in frontmatter.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.frontmatter.name
    }

    /// Returns the skill description declared in frontmatter.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.frontmatter.description
    }

    /// Returns the title text of the first level 1 heading (`# Title`) if present.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.sections
            .iter()
            .find(|s| s.level == 1)
            .map(|s| s.title.as_str())
    }

    /// Finds a section by exact case-insensitive title match.
    #[must_use]
    pub fn find_section(&self, title: &str) -> Option<&MarkdownSection> {
        self.sections
            .iter()
            .find(|s| s.title.eq_ignore_ascii_case(title))
    }

    /// Returns content of the `## Rules` section if present.
    #[must_use]
    pub fn rules(&self) -> Option<&str> {
        self.find_section("Rules").map(|s| s.content.trim())
    }

    /// Returns content of the `## Context` section if present.
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.find_section("Context").map(|s| s.content.trim())
    }

    /// Returns content of the `## Examples` section if present.
    #[must_use]
    pub fn examples(&self) -> Option<&str> {
        self.find_section("Examples").map(|s| s.content.trim())
    }

    /// Returns content of the `## Execution Prompts` or `## Prompts` section if present.
    #[must_use]
    pub fn execution_prompts(&self) -> Option<&str> {
        self.find_section("Execution Prompts")
            .or_else(|| self.find_section("Prompts"))
            .map(|s| s.content.trim())
    }

    /// Renders the instruction body by substituting variables into all `{{param}}` placeholders.
    pub fn render_prompt(&self, variables: &HashMap<String, String>) -> Result<String> {
        SkillParser::render_template(&self.body, variables)
    }

    /// Converts the parsed skill into a domain `Skill` entity with path and category context.
    #[must_use]
    pub fn into_skill(self, path: impl Into<PathBuf>, category: SkillCategory) -> Skill {
        let path = path.into();
        let dir_name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or(self.frontmatter.name.as_str())
            .to_string();
        let promoted = category.is_promoted();

        let raw = format!(
            "---\n{}---\n{}",
            self.raw_frontmatter.as_deref().unwrap_or(""),
            self.body
        );

        Skill::builder()
            .path(path)
            .dir_name(dir_name)
            .category(category)
            .promoted(promoted)
            .frontmatter(self.frontmatter)
            .content(self.body)
            .raw(raw)
            .resources(self.resources)
            .build()
    }
}

/// Skill markdown parser and directory discovery engine.
#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct SkillParser {
    /// Whether to eagerly load text contents of discovered resource files.
    #[builder(default = false)]
    pub load_resource_contents: bool,

    /// Whether to suppress frontmatter YAML errors and record them into `Skill.yaml_error`.
    #[builder(default = false)]
    pub lenient: bool,
}

impl SkillParser {
    /// Creates a new `SkillParser` with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a skill markdown string into a validated `ParsedSkill`.
    ///
    /// # Errors
    /// Returns `SkillError::FrontmatterValidation` or `SkillError::Yaml` if frontmatter
    /// is missing, malformed, or invalid.
    pub fn parse_str(content: &str) -> Result<ParsedSkill> {
        Self::default().parse_str_internal(content, None)
    }

    /// Parses a skill markdown string with explicit path context.
    pub fn parse_str_with_path(content: &str, path: impl AsRef<Path>) -> Result<ParsedSkill> {
        Self::default().parse_str_internal(content, Some(path.as_ref()))
    }

    /// Parses a skill file (`SKILL.md`) from disk, discovering associated resources.
    ///
    /// # Errors
    /// Returns `SkillError::Io` on read failure, or `SkillError` on parse/validation failure.
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Skill> {
        Self::default().parse_file_internal(path.as_ref())
    }

    /// Instance method parsing a skill file from disk using configured settings.
    pub fn parse_file_with_options(&self, path: impl AsRef<Path>) -> Result<Skill> {
        self.parse_file_internal(path.as_ref())
    }

    /// Extracts frontmatter YAML and markdown body slices between `---` delimiters.
    ///
    /// # Errors
    /// Returns `SkillError::FrontmatterValidation` if opening or closing delimiters are missing.
    pub fn extract_frontmatter(content: &str) -> Result<(&str, &str)> {
        let bom = char::from_u32(0xFEFF).unwrap_or('\0');
        let trimmed_start = content.strip_prefix(bom).unwrap_or(content);

        // Check opening delimiter
        if !trimmed_start.starts_with("---") {
            return Err(SkillError::FrontmatterValidation {
                path: PathBuf::new(),
                message: "Missing opening '---' frontmatter delimiter".to_string(),
            });
        }

        let after_opening = &trimmed_start[3..];
        let yaml_start = if let Some(stripped) = after_opening.strip_prefix("\r\n") {
            stripped
        } else if let Some(stripped) = after_opening.strip_prefix('\n') {
            stripped
        } else if let Some(newline_pos) = after_opening.find('\n') {
            let first_line = after_opening[..newline_pos].trim();
            if !first_line.is_empty() {
                return Err(SkillError::FrontmatterValidation {
                    path: PathBuf::new(),
                    message: "Unexpected characters after opening '---' delimiter".to_string(),
                });
            }
            &after_opening[newline_pos + 1..]
        } else {
            return Err(SkillError::FrontmatterValidation {
                path: PathBuf::new(),
                message: "Unterminated frontmatter: missing closing '---' delimiter".to_string(),
            });
        };

        // Scan line-by-line for closing delimiter
        let mut offset = 0;
        let mut found = false;
        let mut yaml_end = 0;
        let mut body_start = 0;

        for line in yaml_start.split('\n') {
            let line_trimmed = line.trim_end_matches('\r').trim();
            if line_trimmed == "---" || line_trimmed == "..." {
                found = true;
                yaml_end = offset;
                let line_len = line.len() + 1; // + 1 for \n
                body_start = (offset + line_len).min(yaml_start.len());
                break;
            }
            offset += line.len() + 1; // + 1 for \n
        }

        if !found {
            return Err(SkillError::FrontmatterValidation {
                path: PathBuf::new(),
                message: "Unterminated frontmatter: missing closing '---' delimiter".to_string(),
            });
        }

        let yaml_slice = &yaml_start[..yaml_end];
        let body_slice = if body_start < yaml_start.len() {
            &yaml_start[body_start..]
        } else {
            ""
        };

        Ok((yaml_slice, body_slice))
    }

    /// Validates skill frontmatter attributes against domain constraints.
    pub fn validate_frontmatter(fm: &SkillFrontmatter, path: Option<&Path>) -> Result<()> {
        let dummy_path = PathBuf::new();
        let target_path = path.unwrap_or(&dummy_path);

        if fm.name.trim().is_empty() {
            return Err(SkillError::validation(
                target_path,
                "Missing required frontmatter field 'name'",
            ));
        }

        if fm.description.trim().is_empty() {
            return Err(SkillError::validation(
                target_path,
                "Missing required frontmatter field 'description'",
            ));
        }

        let dir_name = path
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        Self::validate_skill_name(&fm.name, dir_name)?;

        Ok(())
    }

    /// Validates skill name conformity against kebab-case rules and directory name equality.
    pub fn validate_skill_name(name: &str, dir_name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(SkillError::InvalidSkillName {
                name: name.to_string(),
                reason: "Skill name cannot be empty".to_string(),
            });
        }

        if name.len() > 64 {
            return Err(SkillError::InvalidSkillName {
                name: name.to_string(),
                reason: format!(
                    "Skill name length ({}) exceeds maximum 64 characters",
                    name.len()
                ),
            });
        }

        let lower = name.to_ascii_lowercase();
        if lower.contains("anthropic") || lower.contains("claude") {
            return Err(SkillError::InvalidSkillName {
                name: name.to_string(),
                reason: "Skill name cannot contain reserved words 'anthropic' or 'claude'"
                    .to_string(),
            });
        }

        // Kebab-case validation: ^[a-z0-9]+(-[a-z0-9]+)*$
        let is_valid_kebab = !name.starts_with('-')
            && !name.ends_with('-')
            && !name.contains("--")
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');

        if !is_valid_kebab {
            return Err(SkillError::InvalidSkillName {
                name: name.to_string(),
                reason: "Skill name must be lowercase kebab-case (e.g. 'my-skill-name')"
                    .to_string(),
            });
        }

        if !dir_name.is_empty() && dir_name != "." && dir_name != "skills" && name != dir_name {
            return Err(SkillError::InvalidSkillName {
                name: name.to_string(),
                reason: format!(
                    "Skill name '{name}' does not match enclosing directory name '{dir_name}'"
                ),
            });
        }

        Ok(())
    }

    /// Parses markdown body into hierarchical `MarkdownSection` items.
    ///
    /// Headings inside fenced code blocks are ignored.
    #[must_use]
    pub fn extract_sections(body: &str) -> Vec<MarkdownSection> {
        let mut sections = Vec::new();
        let mut current_section: Option<MarkdownSection> = None;
        let mut in_code_fence = false;
        let mut line_num = 1;

        for line in body.split('\n') {
            let trimmed = line.trim();

            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_fence = !in_code_fence;
            }

            if !in_code_fence && trimmed.starts_with('#') {
                let mut level = 0;
                for c in trimmed.chars() {
                    if c == '#' {
                        level += 1;
                    } else {
                        break;
                    }
                }

                if level <= 6 && trimmed[level..].starts_with(' ') {
                    if let Some(sec) = current_section.take() {
                        sections.push(sec);
                    }

                    let title = trimmed[level..]
                        .trim()
                        .trim_end_matches('#')
                        .trim()
                        .to_string();
                    current_section = Some(MarkdownSection {
                        level,
                        title,
                        content: String::new(),
                        line: line_num,
                    });
                    line_num += 1;
                    continue;
                }
            }

            if let Some(ref mut sec) = current_section {
                if !sec.content.is_empty() {
                    sec.content.push('\n');
                }
                sec.content.push_str(line.trim_end_matches('\r'));
            } else if !line.trim().is_empty() {
                // Preamble before any heading
                current_section = Some(MarkdownSection {
                    level: 0,
                    title: String::new(),
                    content: line.trim_end_matches('\r').to_string(),
                    line: line_num,
                });
            }

            line_num += 1;
        }

        if let Some(sec) = current_section {
            sections.push(sec);
        }

        sections
    }

    /// Extracts all template placeholders (`{{param}}` or `{{param:-default}}`) with line and column coordinates.
    #[must_use]
    pub fn extract_placeholders(text: &str) -> Vec<TemplatePlaceholder> {
        let mut placeholders = Vec::new();
        let mut line = 1;
        let mut col = 1;
        let mut i = 0;
        let bytes = text.as_bytes();
        let len = bytes.len();

        while i + 1 < len {
            if bytes[i] == b'\n' {
                line += 1;
                col = 1;
                i += 1;
                continue;
            }

            if bytes[i] == b'{' && bytes[i + 1] == b'{' {
                let start_idx = i;
                let start_line = line;
                let start_col = col;

                // Scan for closing `}}`
                let mut j = i + 2;
                let mut found_close = false;
                while j + 1 < len && bytes[j] != b'\n' {
                    if bytes[j] == b'}' && bytes[j + 1] == b'}' {
                        found_close = true;
                        break;
                    }
                    j += 1;
                }

                if found_close {
                    let token_bytes = &bytes[start_idx..j + 2];
                    let raw_token = String::from_utf8_lossy(token_bytes).to_string();
                    let inner = &raw_token[2..raw_token.len() - 2].trim();

                    // Ignore comments or handlebars blocks: {{/* */}}, {{#if}}, {{/if}}
                    if !inner.starts_with("/*")
                        && !inner.starts_with('#')
                        && !inner.starts_with('/')
                        && !inner.starts_with('@')
                        && !inner.is_empty()
                    {
                        let (name, default_value) = if let Some(pos) = inner.find(":-") {
                            (
                                inner[..pos].trim().to_string(),
                                Some(inner[pos + 2..].trim().to_string()),
                            )
                        } else if let Some(pos) = inner.find(':') {
                            (
                                inner[..pos].trim().to_string(),
                                Some(inner[pos + 1..].trim().to_string()),
                            )
                        } else {
                            ((*inner).to_string(), None)
                        };

                        placeholders.push(TemplatePlaceholder {
                            name,
                            default_value,
                            raw: raw_token,
                            line: start_line,
                            column: start_col,
                            start: start_idx,
                            end: j + 2,
                        });
                    }

                    col += j + 2 - start_idx;
                    i = j + 2;
                    continue;
                }
            }

            col += 1;
            i += 1;
        }

        placeholders
    }

    /// Renders a template string by replacing `{{var}}` placeholders with values from `variables`.
    pub fn render_template(template: &str, variables: &HashMap<String, String>) -> Result<String> {
        let placeholders = Self::extract_placeholders(template);
        let mut result = String::with_capacity(template.len());
        let mut last_idx = 0;

        for p in placeholders {
            result.push_str(&template[last_idx..p.start]);
            if let Some(val) = variables.get(&p.name) {
                result.push_str(val);
            } else if let Some(ref def) = p.default_value {
                result.push_str(def);
            } else {
                return Err(SkillError::FrontmatterValidation {
                    path: PathBuf::new(),
                    message: format!("Missing required template variable '{}'", p.name),
                });
            }
            last_idx = p.end;
        }

        result.push_str(&template[last_idx..]);
        Ok(result)
    }

    /// Discovers auxiliary resource files located in `resources/auto/` and `resources/manual/`.
    pub fn discover_resources(skill_dir: impl AsRef<Path>) -> Result<Vec<ResourceFile>> {
        Self::default().discover_resources_internal(skill_dir.as_ref())
    }

    /// Traverses a root directory discovering and cataloging all `SKILL.md` skill definitions.
    pub fn discover_skills(root: impl AsRef<Path>) -> Result<Vec<Skill>> {
        Self::default().discover_skills_internal(root.as_ref())
    }

    // ------------------------------------------------------------------------
    // Internal Helper Methods
    // ------------------------------------------------------------------------

    fn parse_str_internal(&self, content: &str, path: Option<&Path>) -> Result<ParsedSkill> {
        let (yaml_raw, body) = match Self::extract_frontmatter(content) {
            Ok((y, b)) => (y, b),
            Err(e) => {
                if self.lenient {
                    return Ok(ParsedSkill {
                        frontmatter: SkillFrontmatter::builder()
                            .name("unknown")
                            .description("Lenient parse fallback")
                            .build(),
                        raw_frontmatter: None,
                        body: content.to_string(),
                        sections: Self::extract_sections(content),
                        placeholders: Self::extract_placeholders(content),
                        resources: Vec::new(),
                    });
                }
                return Err(e);
            }
        };

        let frontmatter: SkillFrontmatter = match serde_yaml::from_str(yaml_raw) {
            Ok(fm) => fm,
            Err(err) => {
                if self.lenient {
                    SkillFrontmatter::builder()
                        .name("unknown")
                        .description("Lenient YAML error")
                        .build()
                } else {
                    let target_path = path.unwrap_or_else(|| Path::new("SKILL.md"));
                    return Err(SkillError::yaml(target_path, err));
                }
            }
        };

        if !self.lenient {
            Self::validate_frontmatter(&frontmatter, path)?;
        }

        let sections = Self::extract_sections(body);
        let placeholders = Self::extract_placeholders(body);

        Ok(ParsedSkill {
            frontmatter,
            raw_frontmatter: Some(yaml_raw.to_string()),
            body: body.to_string(),
            sections,
            placeholders,
            resources: Vec::new(),
        })
    }

    fn parse_file_internal(&self, path: &Path) -> Result<Skill> {
        let content = fs::read_to_string(path).map_err(|e| SkillError::io(path, e))?;
        let parsed = self.parse_str_internal(&content, Some(path))?;

        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let dir_name = parent
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| parsed.name())
            .to_string();

        let category = if let Some(ref cat) = parsed.frontmatter.category {
            cat.clone()
        } else {
            Self::infer_category_from_path(path)
        };

        let resources = self.discover_resources_internal(parent)?;

        Ok(Skill::builder()
            .path(path.to_path_buf())
            .dir_name(dir_name)
            .category(category.clone())
            .promoted(category.is_promoted())
            .frontmatter(parsed.frontmatter)
            .content(parsed.body)
            .raw(content)
            .resources(resources)
            .build())
    }

    /// Infers skill category from directory layout (e.g. `skills/<category>/<name>/SKILL.md`).
    #[must_use]
    pub fn infer_category_from_path(path: &Path) -> SkillCategory {
        let components: Vec<_> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        // Check for `skills/<category>/<name>/SKILL.md`
        for i in 0..components.len() {
            if components[i] == "skills" && i + 1 < components.len() {
                let cat_str = &components[i + 1];
                return SkillCategory::from_str(cat_str).unwrap();
            }
        }

        // Fallback: grandparent directory name
        if components.len() >= 3 {
            let cat_str = &components[components.len() - 3];
            return SkillCategory::from_str(cat_str).unwrap();
        }

        SkillCategory::Engineering
    }

    fn discover_resources_internal(&self, skill_dir: &Path) -> Result<Vec<ResourceFile>> {
        let resources_dir = skill_dir.join("resources");
        if !resources_dir.exists() || !resources_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut resources = Vec::new();

        for entry in WalkDir::new(&resources_dir).follow_links(false) {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => return Err(SkillError::io(skill_dir, err.into())),
            };

            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = entry.file_name().to_string_lossy();

                // Skip hidden files
                if file_name.starts_with('.') {
                    continue;
                }

                let relative_path = file_path
                    .strip_prefix(skill_dir)
                    .unwrap_or(file_path)
                    .to_path_buf();

                let kind = if relative_path.starts_with("resources/auto")
                    || relative_path.starts_with("auto")
                {
                    ResourceKind::Auto
                } else {
                    ResourceKind::Manual
                };

                let content = if self.load_resource_contents {
                    fs::read_to_string(file_path).ok()
                } else {
                    None
                };

                resources.push(ResourceFile {
                    relative_path,
                    absolute_path: file_path.to_path_buf(),
                    kind,
                    content,
                });
            }
        }

        resources.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(resources)
    }

    fn discover_skills_internal(&self, root: &Path) -> Result<Vec<Skill>> {
        let search_dir = if root.join("skills").is_dir() {
            root.join("skills")
        } else {
            root.to_path_buf()
        };

        let mut skills = Vec::new();

        for entry in WalkDir::new(&search_dir).follow_links(false) {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => return Err(SkillError::io(root, err.into())),
            };

            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy();
                if name.eq_ignore_ascii_case("SKILL.md") {
                    let path = entry.path();
                    match self.parse_file_internal(path) {
                        Ok(mut skill) => {
                            // Relativize path against root
                            if let Ok(rel) = path.strip_prefix(root) {
                                skill.path = rel.to_path_buf();
                            }
                            skills.push(skill);
                        }
                        Err(e) => {
                            if !self.lenient {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }

        skills.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(skills)
    }
}
