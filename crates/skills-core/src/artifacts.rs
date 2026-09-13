#![allow(
    clippy::too_many_lines,
    clippy::format_push_string,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::unnecessary_debug_formatting,
    clippy::manual_strip,
    clippy::struct_excessive_bools,
    clippy::items_after_statements
)]

//! Generated-artifact synchronization engine and markdown transformation primitives.
//!
//! Synchronizes repository-level manifests (`agents/AGENTS.md`, `README.md`,
//! `skills.sh.json`) and generates the Zola documentation dashboard content tree
//! including live categories, sibling reference pages, resources, and archived skills.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{Result, SkillError};
use crate::models::{Skill, SkillCategory, SkillTree};
use crate::parser::SkillParser;

/// Options controlling artifact synchronization behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactsOptions {
    /// Skip generating Zola dashboard content.
    pub skip_dashboard: bool,
    /// Restrict sync to repository artifacts only.
    pub repo_only: bool,
    /// Do not stage modified files in git.
    pub no_stage: bool,
    /// Filter to a single category name.
    pub category_filter: Option<String>,
    /// Filter to a single skill name or "category/name".
    pub skill_filter: Option<String>,
    /// GitHub repository source identifier (default: "MAHDTech/agent-skills").
    pub github_source: String,
    /// Git default branch for web links (default: "main").
    pub git_branch: String,
}

impl Default for ArtifactsOptions {
    fn default() -> Self {
        Self {
            skip_dashboard: false,
            repo_only: false,
            no_stage: false,
            category_filter: None,
            skill_filter: None,
            github_source: "MAHDTech/agent-skills".to_string(),
            git_branch: "main".to_string(),
        }
    }
}

/// Summary report returned after artifact synchronization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtifactsSummary {
    pub agents_updated: bool,
    pub readme_updated: bool,
    pub skills_sh_updated: bool,
    pub dashboard_generated: bool,
    pub staged_files: Vec<PathBuf>,
    pub live_skills_count: usize,
    pub archived_skills_count: usize,
}

/// Schema model for `skills.sh.json` manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillsManifest {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "notGrouped")]
    pub not_grouped: String,
    pub groupings: Vec<SkillGrouping>,
}

/// Category grouping entry within `skills.sh.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillGrouping {
    pub title: String,
    pub description: String,
    pub skills: Vec<String>,
}

/// Orchestrator for repository artifact generation.
#[derive(Debug, Clone)]
pub struct ArtifactsEngine {
    workspace_root: PathBuf,
    options: ArtifactsOptions,
}

/// Backward-compatible alias for the artifacts orchestrator.
pub type ArtifactsGenerator = ArtifactsEngine;

impl ArtifactsEngine {
    /// Creates a new engine instance with default options.
    #[must_use]
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self::with_options(workspace_root, ArtifactsOptions::default())
    }

    /// Creates a new engine instance with explicit options.
    #[must_use]
    pub fn with_options(workspace_root: impl Into<PathBuf>, options: ArtifactsOptions) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            options,
        }
    }

    /// Creates a new engine instance with options derived from environment variables.
    #[must_use]
    pub fn from_env(workspace_root: impl Into<PathBuf>) -> Self {
        let mut options = ArtifactsOptions::default();
        if std::env::var("SKILLS_SKIP_DASHBOARD").is_ok() {
            options.skip_dashboard = true;
        }
        if std::env::var("SKILLS_REPO_ONLY").is_ok()
            || std::env::var("PRE_COMMIT").is_ok()
            || std::env::var("CI").is_ok()
        {
            options.repo_only = true;
        }
        if std::env::var("SKILLS_NO_STAGE").is_ok() {
            options.no_stage = true;
        }
        Self::with_options(workspace_root, options)
    }

    /// Returns a reference to the active options.
    #[must_use]
    pub fn options(&self) -> &ArtifactsOptions {
        &self.options
    }

    /// Returns a reference to the workspace root path.
    #[must_use]
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Runs all artifact generation tasks and git staging.
    pub fn generate_all(&self) -> Result<ArtifactsSummary> {
        let all_skills = SkillParser::discover_skills(&self.workspace_root)?;
        let mut live_skills = Vec::new();
        let mut archived_skills = Vec::new();

        for skill in all_skills {
            if skill.tree == SkillTree::Archive {
                archived_skills.push(skill);
            } else {
                live_skills.push(skill);
            }
        }

        let mut summary = ArtifactsSummary {
            live_skills_count: live_skills.len(),
            archived_skills_count: archived_skills.len(),
            ..Default::default()
        };

        let mut files_to_stage = Vec::new();

        if self.generate_agents_md(&live_skills)? {
            summary.agents_updated = true;
            files_to_stage.push(self.workspace_root.join("agents").join("AGENTS.md"));
        }

        if self.generate_readme(&live_skills)? {
            summary.readme_updated = true;
            files_to_stage.push(self.workspace_root.join("README.md"));
        }

        if self.generate_skills_sh_json(&live_skills)? {
            summary.skills_sh_updated = true;
            files_to_stage.push(self.workspace_root.join("skills.sh.json"));
        }

        if !self.options.skip_dashboard
            && !self.options.repo_only
            && self.generate_dashboard_content(&live_skills, &archived_skills)?
        {
            summary.dashboard_generated = true;
            let dashboard_dir = self
                .workspace_root
                .join("dashboard")
                .join("content")
                .join("skills");
            files_to_stage.push(dashboard_dir);
        }

        if !self.options.no_stage {
            summary.staged_files = self.stage_files(&files_to_stage)?;
        }

        Ok(summary)
    }

    /// Generates `agents/AGENTS.md` index.
    pub fn generate_agents_md(&self, live_skills: &[Skill]) -> Result<bool> {
        if self.options.category_filter.is_some() || self.options.skill_filter.is_some() {
            return Ok(false);
        }

        let agents_file = self.workspace_root.join("agents").join("AGENTS.md");
        let frontmatter = if agents_file.is_file() {
            let existing =
                fs::read_to_string(&agents_file).map_err(|e| SkillError::io(&agents_file, e))?;
            let normalized = normalize_crlf(&existing);
            extract_yaml_frontmatter(&normalized)
                .unwrap_or("")
                .to_string()
        } else {
            String::new()
        };

        let mut body = String::from("\n# Available Skills\n");
        for category in SkillCategory::standard_promoted() {
            let mut matching: Vec<&Skill> = live_skills
                .iter()
                .filter(|s| s.tree == SkillTree::Live && s.promoted && s.category == *category)
                .collect();

            if matching.is_empty() {
                continue;
            }

            matching.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

            body.push_str(&format!("\n## {}\n\n", category.title()));
            for s in matching {
                let desc = s.frontmatter.description.trim();
                body.push_str(&format!("- **{}**: {}\n", s.dir_name, desc));
            }
        }

        let full_content = format!("{frontmatter}{body}");
        atomic_write(&agents_file, &full_content)?;
        Ok(true)
    }

    /// Generates `README.md` catalog.
    pub fn generate_readme(&self, live_skills: &[Skill]) -> Result<bool> {
        if self.options.category_filter.is_some() || self.options.skill_filter.is_some() {
            return Ok(false);
        }

        let readme_file = self.workspace_root.join("README.md");
        let docs_dir = self.workspace_root.join("docs");

        let mut docs_list = Vec::new();
        if docs_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&docs_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.ends_with(".md") {
                                docs_list.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
        docs_list.sort();

        let docs_content = if docs_list.is_empty() {
            String::new()
        } else {
            let lines: Vec<String> = docs_list
                .iter()
                .map(|doc| {
                    let stem = doc.strip_suffix(".md").unwrap_or(doc);
                    format!("- [{stem}](docs/{doc})")
                })
                .collect();
            format!("## Documentation\n\n{}\n\n", lines.join("\n"))
        };

        let mut category_blocks = Vec::new();
        for category in SkillCategory::standard_promoted() {
            let mut matching: Vec<&Skill> = live_skills
                .iter()
                .filter(|s| s.tree == SkillTree::Live && s.promoted && s.category == *category)
                .collect();

            if matching.is_empty() {
                continue;
            }

            matching.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

            let skills_list: Vec<String> = matching
                .iter()
                .map(|s| {
                    let desc = s.frontmatter.description.trim();
                    let posix_path = to_posix_path(&s.path);
                    format!("- **[{}]({})** - {}", s.dir_name, posix_path, desc)
                })
                .collect();

            category_blocks.push(format!(
                "### {}\n\n{}\n\n{}",
                category.title(),
                category.description(),
                skills_list.join("\n")
            ));
        }

        let catalog_content = category_blocks.join("\n\n");
        let source = &self.options.github_source;

        let readme_content = format!(
            "# Agent Skills\n\n\
            [![skills.sh](https://skills.sh/b/{source})](https://skills.sh/{source})\n\n\
            Working on my _skill issues_.\n\n\
            ![skill issues](./docs/images/skill-issues.png)\n\n\
            These are my personal agent skills and attempt to be cross-compatible with Antigravity, Claude Code, Goose and OpenCode.\n\n\
            ## Install\n\n\
            ```bash\n\
            # Using npm\n\
            npx skills add {source}\n\n\
            # Using Bun\n\
            bunx skills add {source}\n\
            ```\n\n\
            {docs_content}## Available Skills\n\n\
            {catalog_content}\n"
        );

        let final_readme = format!("{}\n", readme_content.trim_end());
        atomic_write(&readme_file, &final_readme)?;
        Ok(true)
    }

    /// Generates `skills.sh.json` manifest.
    pub fn generate_skills_sh_json(&self, live_skills: &[Skill]) -> Result<bool> {
        if self.options.category_filter.is_some() || self.options.skill_filter.is_some() {
            return Ok(false);
        }

        let manifest_file = self.workspace_root.join("skills.sh.json");
        let mut groupings = Vec::new();

        for category in SkillCategory::standard_promoted() {
            let mut matching: Vec<&Skill> = live_skills
                .iter()
                .filter(|s| s.tree == SkillTree::Live && s.promoted && s.category == *category)
                .collect();

            if matching.is_empty() {
                continue;
            }

            matching.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

            let skills: Vec<String> = matching.iter().map(|s| s.dir_name.clone()).collect();
            groupings.push(SkillGrouping {
                title: category.title().to_string(),
                description: category.description().to_string(),
                skills,
            });
        }

        let manifest = SkillsManifest {
            schema: "https://skills.sh/schemas/skills.sh.schema.json".to_string(),
            not_grouped: "bottom".to_string(),
            groupings,
        };

        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        manifest
            .serialize(&mut ser)
            .map_err(SkillError::GeneralJson)?;

        let mut json_str = String::from_utf8(buf)
            .map_err(|e| SkillError::validation(&manifest_file, e.to_string()))?;
        json_str.push('\n');

        atomic_write(&manifest_file, &json_str)?;
        Ok(true)
    }

    /// Generates dashboard content tree under `dashboard/content/skills/`.
    #[allow(clippy::too_many_lines)]
    pub fn generate_dashboard_content(
        &self,
        live_skills: &[Skill],
        archived_skills: &[Skill],
    ) -> Result<bool> {
        if self.options.skip_dashboard || self.options.repo_only {
            return Ok(false);
        }

        let dashboard_content_dir = self
            .workspace_root
            .join("dashboard")
            .join("content")
            .join("skills");
        let category_filter = self.options.category_filter.as_deref();
        let skill_filter = self.options.skill_filter.as_deref();
        let has_filter = category_filter.is_some() || skill_filter.is_some();

        if has_filter {
            fs::create_dir_all(&dashboard_content_dir)
                .map_err(|e| SkillError::io(&dashboard_content_dir, e))?;
            let root_index = dashboard_content_dir.join("_index.md");
            if !root_index.exists() {
                atomic_write(
                    &root_index,
                    "+++\ntitle = \"Skills Catalog\"\nsort_by = \"title\"\ntemplate = \"section.html\"\nweight = 1\n+++\n\nWelcome to the agent skills catalog.\n",
                )?;
            }
        } else {
            if dashboard_content_dir.exists() {
                let _ = fs::remove_dir_all(&dashboard_content_dir);
            }
            fs::create_dir_all(&dashboard_content_dir)
                .map_err(|e| SkillError::io(&dashboard_content_dir, e))?;
            let root_index = dashboard_content_dir.join("_index.md");
            atomic_write(
                &root_index,
                "+++\ntitle = \"Skills Catalog\"\nsort_by = \"title\"\ntemplate = \"section.html\"\nweight = 1\n+++\n\nWelcome to the agent skills catalog.\n",
            )?;
        }

        let skills_root = self.workspace_root.join("skills");
        let archive_root = self.workspace_root.join("skills-archive");

        let mut weight = 1usize;
        for category in SkillCategory::standard_promoted() {
            let mut matching: Vec<&Skill> = live_skills
                .iter()
                .filter(|s| {
                    s.tree == SkillTree::Live
                        && s.category == *category
                        && should_include_skill(s, category_filter, skill_filter)
                })
                .collect();

            if matching.is_empty() {
                continue;
            }

            let cat_dir = dashboard_content_dir.join(category.as_str());
            fs::create_dir_all(&cat_dir).map_err(|e| SkillError::io(&cat_dir, e))?;

            let cat_index = cat_dir.join("_index.md");
            let cat_meta = format!(
                "+++\ntitle = {}\ndescription = {}\nsort_by = \"title\"\ntemplate = \"section.html\"\nweight = {}\n+++\n",
                to_toml_string(category.title()),
                to_toml_string(category.description()),
                weight
            );
            atomic_write(&cat_index, &cat_meta)?;
            weight += 1;

            matching.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

            for s in matching {
                let skill_src_dir = if s.path.is_absolute() {
                    s.path
                        .parent()
                        .unwrap_or(&self.workspace_root)
                        .to_path_buf()
                } else {
                    let candidate = self.workspace_root.join(&s.path);
                    candidate
                        .parent()
                        .unwrap_or(&self.workspace_root)
                        .to_path_buf()
                };

                let content_base = format!("skills/{}/{}", category.as_str(), s.dir_name);
                let out_dir = cat_dir.join(&s.dir_name);
                fs::create_dir_all(&out_dir).map_err(|e| SkillError::io(&out_dir, e))?;

                let skill_body = rewrite_skill_links(
                    &s.content,
                    &content_base,
                    &skill_src_dir,
                    &skills_root,
                    Some(&archive_root),
                );
                let skill_mermaid = contains_mermaid(&skill_body);
                let safe_body = escape_zola_shortcodes(&skill_body);

                let skill_index = out_dir.join("_index.md");
                let skill_content = format!(
                    "+++\ntitle = {}\ndescription = {}\nsort_by = \"title\"\ntemplate = \"skill.html\"\n[extra]\nskill = true\ncategory = {}\nmermaid = {}\n+++\n\n{}\n",
                    to_toml_string(&s.dir_name),
                    to_toml_string(s.frontmatter.description.trim()),
                    to_toml_string(category.as_str()),
                    skill_mermaid,
                    safe_body
                );
                atomic_write(&skill_index, &skill_content)?;

                // Process sibling markdown files
                if skill_src_dir.is_dir() {
                    let mut siblings = Vec::new();
                    if let Ok(entries) = fs::read_dir(&skill_src_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                                    if fname.ends_with(".md")
                                        && !fname.eq_ignore_ascii_case("SKILL.md")
                                    {
                                        siblings.push(fname.to_string());
                                    }
                                }
                            }
                        }
                    }
                    siblings.sort();

                    for file in siblings {
                        let sib_path = skill_src_dir.join(&file);
                        let raw = fs::read_to_string(&sib_path)
                            .map_err(|e| SkillError::io(&sib_path, e))?;
                        let normalized = normalize_crlf(&raw);
                        let stripped = strip_legacy_raw_wrapper(&normalized);
                        let body = rewrite_skill_links(
                            &stripped,
                            &content_base,
                            &skill_src_dir,
                            &skills_root,
                            Some(&archive_root),
                        );
                        let sib_mermaid = contains_mermaid(&body);
                        let safe_sib_body = escape_zola_shortcodes(&body);
                        let sib_title = file.strip_suffix(".md").unwrap_or(&file);

                        let dest_sib = out_dir.join(&file);
                        let sib_content = format!(
                            "+++\ntitle = {}\n[extra]\nskill = false\ncategory = {}\nmermaid = {}\nskill_name = {}\n+++\n\n{}\n",
                            to_toml_string(sib_title),
                            to_toml_string(category.as_str()),
                            sib_mermaid,
                            to_toml_string(&s.dir_name),
                            safe_sib_body
                        );
                        atomic_write(&dest_sib, &sib_content)?;
                    }

                    // Mirror resources if present
                    let resources_src = skill_src_dir.join("resources");
                    if resources_src.is_dir() {
                        let resources_dest = out_dir.join("resources");
                        let res_content_base = format!("{content_base}/resources");
                        sync_resources(
                            &resources_src,
                            &resources_dest,
                            &res_content_base,
                            category.as_str(),
                            &s.dir_name,
                            false,
                            None,
                        )?;
                    }
                }
            }
        }

        // Archive section generation
        let matching_archived: Vec<&Skill> = archived_skills
            .iter()
            .filter(|s| {
                s.tree == SkillTree::Archive
                    && should_include_skill(s, category_filter, skill_filter)
            })
            .collect();

        if !matching_archived.is_empty() || (!has_filter && !archived_skills.is_empty()) {
            let archive_dir = dashboard_content_dir.join("archive");
            fs::create_dir_all(&archive_dir).map_err(|e| SkillError::io(&archive_dir, e))?;

            let archive_root_index = archive_dir.join("_index.md");
            let archive_root_meta = format!(
                "+++\ntitle = \"Archive\"\ndescription = \"Retired skills kept for reference.\"\nsort_by = \"title\"\ntemplate = \"section.html\"\nweight = {weight}\n+++\n"
            );
            atomic_write(&archive_root_index, &archive_root_meta)?;

            // Discover distinct categories in archived skills preserving standard order
            let mut seen_categories = HashSet::new();
            let mut ordered_categories = Vec::new();

            for cat in SkillCategory::standard_promoted() {
                if matching_archived.iter().any(|s| s.category == *cat) {
                    seen_categories.insert(cat.as_str().to_string());
                    ordered_categories.push(cat.clone());
                }
            }

            for s in &matching_archived {
                let cat_str = s.category.as_str().to_string();
                if !seen_categories.contains(&cat_str) {
                    seen_categories.insert(cat_str);
                    ordered_categories.push(s.category.clone());
                }
            }

            for category in ordered_categories {
                let mut cat_skills: Vec<&Skill> = matching_archived
                    .iter()
                    .filter(|s| s.category == category)
                    .copied()
                    .collect();

                if cat_skills.is_empty() {
                    continue;
                }

                let cat_dir = archive_dir.join(category.as_str());
                fs::create_dir_all(&cat_dir).map_err(|e| SkillError::io(&cat_dir, e))?;

                let cat_index = cat_dir.join("_index.md");
                let cat_meta = format!(
                    "+++\ntitle = {}\ndescription = {}\nsort_by = \"title\"\ntemplate = \"section.html\"\n+++\n",
                    to_toml_string(category.title()),
                    to_toml_string(category.description())
                );
                atomic_write(&cat_index, &cat_meta)?;

                cat_skills.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

                for s in cat_skills {
                    let skill_src_dir = if s.path.is_absolute() {
                        s.path
                            .parent()
                            .unwrap_or(&self.workspace_root)
                            .to_path_buf()
                    } else {
                        let candidate = self.workspace_root.join(&s.path);
                        candidate
                            .parent()
                            .unwrap_or(&self.workspace_root)
                            .to_path_buf()
                    };

                    let content_base =
                        format!("skills/archive/{}/{}", category.as_str(), s.dir_name);
                    let out_dir = cat_dir.join(&s.dir_name);
                    fs::create_dir_all(&out_dir).map_err(|e| SkillError::io(&out_dir, e))?;

                    let archived_date =
                        s.archived
                            .as_deref()
                            .or_else(|| {
                                s.frontmatter.metadata.as_ref().and_then(|m| {
                                    m.get("archived").map(std::string::String::as_str)
                                })
                            })
                            .unwrap_or("");

                    let replaced_by = s.replaced_by.as_deref().or_else(|| {
                        s.frontmatter
                            .metadata
                            .as_ref()
                            .and_then(|m| m.get("replaced-by").map(std::string::String::as_str))
                    });

                    let source_url = format!(
                        "https://github.com/{}/tree/{}/skills-archive/{}/{}",
                        self.options.github_source,
                        self.options.git_branch,
                        category.as_str(),
                        s.dir_name
                    );

                    let skill_body = rewrite_skill_links(
                        &s.content,
                        &content_base,
                        &skill_src_dir,
                        &skills_root,
                        Some(&archive_root),
                    );
                    let skill_mermaid = contains_mermaid(&skill_body);
                    let safe_body = escape_zola_shortcodes(&skill_body);

                    let replaced_by_toml = if let Some(rep) = replaced_by {
                        if rep.is_empty() {
                            String::new()
                        } else {
                            format!("replaced_by = {}\n", to_toml_string(rep))
                        }
                    } else {
                        String::new()
                    };

                    let skill_index = out_dir.join("_index.md");
                    let skill_content = format!(
                        "+++\ntitle = {}\ndescription = {}\nsort_by = \"title\"\ntemplate = \"skill.html\"\n[extra]\nskill = true\ncategory = {}\nmermaid = {}\narchived = {}\n{}source_url = {}\n+++\n\n{}\n",
                        to_toml_string(&s.dir_name),
                        to_toml_string(s.frontmatter.description.trim()),
                        to_toml_string(category.as_str()),
                        skill_mermaid,
                        to_toml_string(archived_date),
                        replaced_by_toml,
                        to_toml_string(&source_url),
                        safe_body
                    );
                    atomic_write(&skill_index, &skill_content)?;

                    // Sibling files for archived skill
                    if skill_src_dir.is_dir() {
                        let mut siblings = Vec::new();
                        if let Ok(entries) = fs::read_dir(&skill_src_dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                                        if fname.ends_with(".md")
                                            && !fname.eq_ignore_ascii_case("SKILL.md")
                                        {
                                            siblings.push(fname.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        siblings.sort();

                        for file in siblings {
                            let sib_path = skill_src_dir.join(&file);
                            let raw = fs::read_to_string(&sib_path)
                                .map_err(|e| SkillError::io(&sib_path, e))?;
                            let normalized = normalize_crlf(&raw);
                            let stripped = strip_legacy_raw_wrapper(&normalized);
                            let body = rewrite_skill_links(
                                &stripped,
                                &content_base,
                                &skill_src_dir,
                                &skills_root,
                                Some(&archive_root),
                            );
                            let sib_mermaid = contains_mermaid(&body);
                            let safe_sib_body = escape_zola_shortcodes(&body);
                            let sib_title = file.strip_suffix(".md").unwrap_or(&file);

                            let dest_sib = out_dir.join(&file);
                            let sib_content = format!(
                                "+++\ntitle = {}\n[extra]\nskill = false\ncategory = {}\nmermaid = {}\nskill_name = {}\narchived = {}\n{}source_url = {}\n+++\n\n{}\n",
                                to_toml_string(sib_title),
                                to_toml_string(category.as_str()),
                                sib_mermaid,
                                to_toml_string(&s.dir_name),
                                to_toml_string(archived_date),
                                replaced_by_toml,
                                to_toml_string(&source_url),
                                safe_sib_body
                            );
                            atomic_write(&dest_sib, &sib_content)?;
                        }

                        // Mirror resources with archive metadata
                        let resources_src = skill_src_dir.join("resources");
                        if resources_src.is_dir() {
                            let resources_dest = out_dir.join("resources");
                            let res_content_base = format!("{content_base}/resources");
                            let meta_tuple = (archived_date, replaced_by, source_url.as_str());
                            sync_resources(
                                &resources_src,
                                &resources_dest,
                                &res_content_base,
                                category.as_str(),
                                &s.dir_name,
                                true,
                                Some(meta_tuple),
                            )?;
                        }
                    }
                }
            }
        }

        Ok(true)
    }

    /// Stages updated files in the git index.
    pub fn stage_files(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>> {
        if self.options.no_stage {
            return Ok(Vec::new());
        }

        let existing_files: Vec<PathBuf> = files.iter().filter(|f| f.exists()).cloned().collect();
        if existing_files.is_empty() {
            return Ok(Vec::new());
        }

        let status = Command::new("git")
            .arg("add")
            .args(&existing_files)
            .current_dir(&self.workspace_root)
            .status();

        match status {
            Ok(s) if s.success() => Ok(existing_files),
            Ok(_) | Err(_) => {
                eprintln!("Warning: git add failed; proceeding without staging.");
                Ok(Vec::new())
            }
        }
    }
}

/// Strips legacy `{% raw %}` outer wrapper blocks when present.
#[must_use]
pub fn strip_legacy_raw_wrapper(content: &str) -> String {
    let trimmed = content.trim();
    if let Some(stripped_start) = trimmed.strip_prefix("{% raw %}") {
        if stripped_start.starts_with('\n') || stripped_start.starts_with("\r\n") {
            let inner_candidate = if stripped_start.starts_with("\r\n") {
                &stripped_start[2..]
            } else {
                &stripped_start[1..]
            };
            if let Some(inner) = inner_candidate.strip_suffix("{% endraw %}") {
                if inner.ends_with('\n') || inner.ends_with("\r\n") {
                    let final_inner = if inner.ends_with("\r\n") {
                        &inner[..inner.len() - 2]
                    } else if inner.ends_with('\n') {
                        &inner[..inner.len() - 1]
                    } else {
                        inner
                    };
                    return final_inner.to_string();
                }
            }
        }
    }
    content.to_string()
}

/// Word joiner character used to split shortcode delimiters.
const WORD_JOINER: char = '\u{2060}';

/// Escapes or neutralizes Zola shortcodes in markdown body text.
#[must_use]
pub fn escape_zola_shortcodes(content: &str) -> String {
    // Step 1: Escape inline shortcode calls `{{-? name(args) -?}}`
    let mut inline_escaped = String::with_capacity(content.len());
    let mut cursor = 0;

    while cursor < content.len() {
        if let Some(open_rel) = content[cursor..].find("{{") {
            let open_idx = cursor + open_rel;
            inline_escaped.push_str(&content[cursor..open_idx]);

            let has_dash = content[open_idx..].starts_with("{{-");
            let tag_content_start = if has_dash { open_idx + 3 } else { open_idx + 2 };

            if let Some(close_rel) = content[tag_content_start..].find("}}") {
                let close_idx = tag_content_start + close_rel;
                let candidate = &content[tag_content_start..close_idx];
                let inner = if candidate.ends_with('-') {
                    &candidate[..candidate.len() - 1]
                } else {
                    candidate
                };

                if is_inline_shortcode_call(inner) && !inner.contains("{{") {
                    inline_escaped.push_str("{{/*");
                    inline_escaped.push_str(inner);
                    inline_escaped.push_str("*/}}");
                    cursor = close_idx + 2;
                    continue;
                }
            }

            inline_escaped.push_str("{{");
            cursor = open_idx + 2;
        } else {
            inline_escaped.push_str(&content[cursor..]);
            break;
        }
    }

    // Step 2: Process body shortcodes `{%-? ... -?%}`
    #[derive(Debug, PartialEq, Eq)]
    enum TagKind {
        Open,
        End,
    }

    struct TagToken {
        start: usize,
        end: usize,
        kind: TagKind,
        inner: String,
    }

    let mut tokens = Vec::new();
    let mut search_cursor = 0;

    while search_cursor < inline_escaped.len() {
        if let Some(open_rel) = inline_escaped[search_cursor..].find("{%") {
            let open_idx = search_cursor + open_rel;
            let has_dash = inline_escaped[open_idx..].starts_with("{%-");
            let tag_content_start = if has_dash { open_idx + 3 } else { open_idx + 2 };

            if let Some(close_rel) = inline_escaped[tag_content_start..].find("%}") {
                let close_idx = tag_content_start + close_rel;
                let candidate = &inline_escaped[tag_content_start..close_idx];
                let inner = if candidate.ends_with('-') {
                    &candidate[..candidate.len() - 1]
                } else {
                    candidate
                };

                if !inner.contains("{%") {
                    let trimmed = inner.trim();
                    if trimmed == "end" {
                        tokens.push(TagToken {
                            start: open_idx,
                            end: close_idx + 2,
                            kind: TagKind::End,
                            inner: inner.to_string(),
                        });
                        search_cursor = close_idx + 2;
                        continue;
                    } else if is_body_open_shortcode(inner) {
                        tokens.push(TagToken {
                            start: open_idx,
                            end: close_idx + 2,
                            kind: TagKind::Open,
                            inner: inner.to_string(),
                        });
                        search_cursor = close_idx + 2;
                        continue;
                    }
                }
            }

            search_cursor = open_idx + 2;
        } else {
            break;
        }
    }

    if tokens.is_empty() {
        return inline_escaped;
    }

    tokens.sort_by_key(|t| t.start);

    let mut depth = 0isize;
    let mut balanced = true;
    for t in &tokens {
        match t.kind {
            TagKind::Open => depth += 1,
            TagKind::End => depth -= 1,
        }
        if !(0..=1).contains(&depth) {
            balanced = false;
            break;
        }
    }
    if depth != 0 {
        balanced = false;
    }

    let mut final_result = String::with_capacity(inline_escaped.len() + 32);
    let mut last_end = 0;

    for t in &tokens {
        final_result.push_str(&inline_escaped[last_end..t.start]);
        if balanced {
            final_result.push_str("{%/*");
            final_result.push_str(&t.inner);
            final_result.push_str("*/%}");
        } else {
            final_result.push('{');
            final_result.push(WORD_JOINER);
            final_result.push_str(&inline_escaped[t.start + 1..t.end]);
        }
        last_end = t.end;
    }
    final_result.push_str(&inline_escaped[last_end..]);

    final_result
}

fn is_inline_shortcode_call(inner: &str) -> bool {
    let trimmed = inner.trim();
    if let Some(paren_idx) = trimmed.find('(') {
        let ident = trimmed[..paren_idx].trim();
        if is_valid_shortcode_ident(ident) && trimmed.ends_with(')') {
            return true;
        }
    }
    false
}

fn is_body_open_shortcode(inner: &str) -> bool {
    let trimmed = inner.trim();
    if let Some(paren_idx) = trimmed.find('(') {
        let ident = trimmed[..paren_idx].trim();
        if is_valid_shortcode_ident(ident) && trimmed.ends_with(')') {
            return true;
        }
    }
    false
}

fn is_valid_shortcode_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Rewrites relative `.md` links pointing to existing files into Zola internal links.
#[must_use]
pub fn rewrite_skill_links(
    content: &str,
    content_base: &str,
    src_dir: &Path,
    skills_root: &Path,
    archive_root: Option<&Path>,
) -> String {
    let mut placeholders: Vec<String> = Vec::new();
    let placeholder_prefix = "\x00_CODE_SPAN_PLACEHOLDER_";

    // Step 1: Protect code blocks and spans
    let mut protected_content = String::with_capacity(content.len());
    let mut cursor = 0;
    let bytes = content.as_bytes();

    while cursor < bytes.len() {
        if content[cursor..].starts_with("```") {
            let start = cursor;
            cursor += 3;
            if let Some(pos) = content[cursor..].find("```") {
                let end = cursor + pos + 3;
                let id = placeholders.len();
                placeholders.push(content[start..end].to_string());
                protected_content.push_str(&format!("{placeholder_prefix}{id}\x00"));
                cursor = end;
            } else {
                protected_content.push_str(&content[start..]);
                break;
            }
        } else if content[cursor..].starts_with("``") {
            let start = cursor;
            cursor += 2;
            if let Some(pos) = content[cursor..].find("``") {
                let end = cursor + pos + 2;
                let id = placeholders.len();
                placeholders.push(content[start..end].to_string());
                protected_content.push_str(&format!("{placeholder_prefix}{id}\x00"));
                cursor = end;
            } else {
                protected_content.push_str(&content[start..]);
                break;
            }
        } else if content[cursor..].starts_with('`') {
            let start = cursor;
            cursor += 1;
            let line_end = content[cursor..]
                .find(['\n', '\r'])
                .unwrap_or(content.len() - cursor);
            if let Some(pos) = content[cursor..cursor + line_end].find('`') {
                let end = cursor + pos + 1;
                let id = placeholders.len();
                placeholders.push(content[start..end].to_string());
                protected_content.push_str(&format!("{placeholder_prefix}{id}\x00"));
                cursor = end;
            } else {
                protected_content.push('`');
            }
        } else {
            let ch = content[cursor..].chars().next().unwrap();
            protected_content.push(ch);
            cursor += ch.len_utf8();
        }
    }

    // Step 2: Rewrite markdown links
    let mut output = String::with_capacity(protected_content.len());
    let mut search_cursor = 0;

    let canonical_skills = skills_root.canonicalize().ok();
    let canonical_archive = archive_root.and_then(|a| a.canonicalize().ok());

    while let Some(start_idx) = protected_content[search_cursor..].find("](") {
        let link_open = search_cursor + start_idx;
        output.push_str(&protected_content[search_cursor..link_open + 2]);
        let target_start = link_open + 2;

        let after_target_open = &protected_content[target_start..];
        let has_excluded_prefix = after_target_open.starts_with("http://")
            || after_target_open.starts_with("https://")
            || after_target_open.starts_with("@/")
            || after_target_open.starts_with('#')
            || after_target_open.starts_with("mailto:")
            || after_target_open.starts_with('/');

        if has_excluded_prefix {
            search_cursor = target_start;
            continue;
        }

        let link_end = after_target_open
            .find(')')
            .filter(|&pos| !after_target_open[..pos].contains([' ', '\n', '\r', '\t']));

        if let Some(end_offset) = link_end {
            let raw_target = &after_target_open[..end_offset];
            let (rel_path, anchor) = if let Some(hash_pos) = raw_target.find('#') {
                (&raw_target[..hash_pos], Some(&raw_target[hash_pos..]))
            } else {
                (raw_target, None)
            };

            if rel_path.ends_with(".md") {
                let target_full_path = src_dir.join(rel_path);
                if target_full_path.exists() {
                    if let Ok(canonical_target) = target_full_path.canonicalize() {
                        let mut in_tree = false;
                        let mut is_lifecycle = false;

                        if let Some(sr) = &canonical_skills {
                            if let Ok(rel) = canonical_target.strip_prefix(sr) {
                                in_tree = true;
                                let cat = rel.iter().next().and_then(|c| c.to_str()).unwrap_or("");
                                if cat == "in-progress" {
                                    is_lifecycle = true;
                                }
                            }
                        }

                        if !in_tree {
                            if let Some(ar) = &canonical_archive {
                                if let Ok(rel) = canonical_target.strip_prefix(ar) {
                                    in_tree = true;
                                    let cat =
                                        rel.iter().next().and_then(|c| c.to_str()).unwrap_or("");
                                    if cat == "in-progress" {
                                        is_lifecycle = true;
                                    }
                                }
                            }
                        }

                        if in_tree && !is_lifecycle {
                            let combined = format!("{content_base}/{rel_path}");
                            let normalized = posix_normalize(&combined);
                            let zola_path = if normalized.ends_with("/SKILL.md") {
                                format!("{}/_index.md", &normalized[..normalized.len() - 9])
                            } else if normalized == "SKILL.md" {
                                "_index.md".to_string()
                            } else {
                                normalized
                            };

                            let anchor_str = anchor.unwrap_or("");
                            output.push_str(&format!("@/{zola_path}{anchor_str}"));
                            search_cursor = target_start + end_offset;
                            continue;
                        }
                    }
                }
            }

            output.push_str(raw_target);
            search_cursor = target_start + end_offset;
        } else {
            search_cursor = target_start;
        }
    }
    output.push_str(&protected_content[search_cursor..]);

    // Step 3: Restore code spans
    let mut restored = output;
    for (id, span) in placeholders.iter().enumerate() {
        let placeholder = format!("{placeholder_prefix}{id}\x00");
        restored = restored.replace(&placeholder, span);
    }

    restored
}

/// Recursively mirrors skill resource assets into dashboard directory.
pub fn sync_resources(
    src: &Path,
    dest: &Path,
    content_base: &str,
    category: &str,
    skill_name: &str,
    is_archive: bool,
    archive_metadata: Option<(&str, Option<&str>, &str)>,
) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }

    fs::create_dir_all(dest).map_err(|e| SkillError::io(dest, e))?;

    let index_marker = dest.join("_index.md");
    atomic_write(&index_marker, "+++\nrender = false\n+++\n")?;

    // Infer skills_root and archive_root from src hierarchy
    let mut skills_root = PathBuf::from("skills");
    let mut archive_root = PathBuf::from("skills-archive");
    let mut current = src;
    while let Some(parent) = current.parent() {
        if parent.ends_with("skills") {
            skills_root = parent.to_path_buf();
            if let Some(ws) = parent.parent() {
                archive_root = ws.join("skills-archive");
            }
            break;
        } else if parent.ends_with("skills-archive") {
            archive_root = parent.to_path_buf();
            if let Some(ws) = parent.parent() {
                skills_root = ws.join("skills");
            }
            break;
        }
        current = parent;
    }

    let entries = fs::read_dir(src).map_err(|e| SkillError::io(src, e))?;
    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name == "_index.md" {
            continue;
        }

        let src_path = entry.path();
        let dest_path = dest.join(&file_name);

        let file_type = entry
            .file_type()
            .map_err(|e| SkillError::io(&src_path, e))?;
        let mut real_src_path = src_path.clone();
        let mut link_src_dir = src.to_path_buf();
        let mut is_dir = file_type.is_dir();
        let mut is_file = file_type.is_file();

        if file_type.is_symlink() {
            match fs::canonicalize(&src_path) {
                Ok(canonical) => match fs::metadata(&canonical) {
                    Ok(meta) => {
                        is_dir = meta.is_dir();
                        is_file = meta.is_file();
                        if is_file {
                            if let Some(p) = canonical.parent() {
                                link_src_dir = p.to_path_buf();
                            }
                        }
                        real_src_path = canonical;
                    }
                    Err(err) => {
                        eprintln!("Warning: Broken symlink {src_path:?}: {err}");
                        continue;
                    }
                },
                Err(err) => {
                    eprintln!("Warning: Broken symlink {src_path:?}: {err}");
                    continue;
                }
            }
        }

        if is_dir {
            let next_content_base = format!("{content_base}/{file_name}");
            sync_resources(
                &real_src_path,
                &dest_path,
                &next_content_base,
                category,
                skill_name,
                is_archive,
                archive_metadata,
            )?;
        } else if is_file {
            let output_name = if file_name.eq_ignore_ascii_case("index.md") {
                "index-page.md"
            } else {
                &file_name
            };
            let final_dest_path = dest.join(output_name);

            if file_name.ends_with(".md") {
                let raw = fs::read_to_string(&real_src_path)
                    .map_err(|e| SkillError::io(&real_src_path, e))?;
                let normalized = normalize_crlf(&raw);
                let unwrapped = strip_legacy_raw_wrapper(&normalized);
                let body = rewrite_skill_links(
                    &unwrapped,
                    content_base,
                    &link_src_dir,
                    &skills_root,
                    Some(&archive_root),
                );
                let sib_mermaid = contains_mermaid(&body);
                let safe_body = escape_zola_shortcodes(&body);
                let title = output_name.strip_suffix(".md").unwrap_or(output_name);

                let content = if is_archive {
                    let (archived_date, replaced_by, source_url) =
                        archive_metadata.unwrap_or(("", None, ""));
                    let rep_line = if let Some(rep) = replaced_by {
                        if rep.is_empty() {
                            String::new()
                        } else {
                            format!("replaced_by = {}\n", to_toml_string(rep))
                        }
                    } else {
                        String::new()
                    };
                    format!(
                        "+++\ntitle = {}\n[extra]\nskill = false\ncategory = {}\nmermaid = {}\nskill_name = {}\narchived = {}\n{}source_url = {}\n+++\n\n{}\n",
                        to_toml_string(title),
                        to_toml_string(category),
                        sib_mermaid,
                        to_toml_string(skill_name),
                        to_toml_string(archived_date),
                        rep_line,
                        to_toml_string(source_url),
                        safe_body
                    )
                } else {
                    format!(
                        "+++\ntitle = {}\n[extra]\nskill = false\ncategory = {}\nmermaid = {}\nskill_name = {}\n+++\n\n{}\n",
                        to_toml_string(title),
                        to_toml_string(category),
                        sib_mermaid,
                        to_toml_string(skill_name),
                        safe_body
                    )
                };
                atomic_write(&final_dest_path, &content)?;
            } else {
                fs::copy(&real_src_path, &final_dest_path)
                    .map_err(|e| SkillError::io(&final_dest_path, e))?;
            }
        }
    }

    Ok(())
}

fn atomic_write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| SkillError::io(parent, e))?;
    }
    let nonce = generate_nonce();
    let tmp_path = path.with_extension(format!("tmp.{nonce}"));
    {
        use std::io::Write;
        let mut file = fs::File::create(&tmp_path).map_err(|e| SkillError::io(&tmp_path, e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| SkillError::io(&tmp_path, e))?;
        file.flush().map_err(|e| SkillError::io(&tmp_path, e))?;
        file.sync_all().ok();
    }
    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        SkillError::io(path, e)
    })?;
    Ok(())
}

fn generate_nonce() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}_{}", std::process::id(), now.as_nanos())
}

fn normalize_crlf(input: &str) -> String {
    input.replace("\r\n", "\n")
}

fn extract_yaml_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") {
        return None;
    }
    let after_first = &content[4..];
    if let Some(idx) = after_first.find("\n---\n") {
        let end = 4 + idx + 5;
        Some(&content[..end])
    } else {
        None
    }
}

fn contains_mermaid(content: &str) -> bool {
    content.contains("```mermaid")
}

fn matches_skill_filter(skill: &Skill, filter: &str) -> bool {
    if filter.contains('/') {
        let mut parts = filter.splitn(2, '/');
        let cat = parts.next().unwrap_or("");
        let name = parts.next().unwrap_or("");
        skill.category.as_str() == cat && (skill.dir_name == name || skill.frontmatter.name == name)
    } else {
        skill.dir_name == filter || skill.frontmatter.name == filter
    }
}

fn should_include_skill(
    skill: &Skill,
    category_filter: Option<&str>,
    skill_filter: Option<&str>,
) -> bool {
    if category_filter.is_none() && skill_filter.is_none() {
        return true;
    }
    if let Some(cat) = category_filter {
        if skill.category.as_str() != cat {
            return false;
        }
    }
    if let Some(name) = skill_filter {
        if !matches_skill_filter(skill, name) {
            return false;
        }
    }
    true
}

fn to_posix_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn posix_normalize(path: &str) -> String {
    let mut parts = Vec::new();
    for seg in path.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            if !parts.is_empty() && parts.last() != Some(&"..") {
                parts.pop();
            } else {
                parts.push("..");
            }
        } else {
            parts.push(seg);
        }
    }
    parts.join("/")
}

fn to_toml_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| format!("\"{value}\""))
}
