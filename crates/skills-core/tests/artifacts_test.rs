#![allow(clippy::similar_names)]
// cspell:words bunx
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use skills_core::artifacts::{
    WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR, WELL_KNOWN_STATIC_DIR,
    select_discovery_skills,
};
use skills_core::{
    ArtifactsEngine, ArtifactsOptions, SKILLS_DISCOVERY_SCHEMA_V0_2_0, Skill, SkillCategory,
    SkillDownloader, SkillFrontmatter, SkillParser, SkillTree, escape_zola_shortcodes,
    rewrite_skill_links, strip_legacy_raw_wrapper, sync_resources,
};

/// Test harness providing an isolated mock workspace directory structure.
#[derive(Debug)]
pub struct ArtifactsTestHarness {
    pub temp_dir: TempDir,
    pub workspace_root: PathBuf,
}

impl ArtifactsTestHarness {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let workspace_root = temp_dir.path().to_path_buf();
        fs::create_dir_all(workspace_root.join("skills")).unwrap();
        fs::create_dir_all(workspace_root.join("skills-archive")).unwrap();
        fs::create_dir_all(workspace_root.join("agents")).unwrap();
        fs::create_dir_all(workspace_root.join("docs")).unwrap();
        fs::create_dir_all(workspace_root.join("dashboard/content/skills")).unwrap();
        Self {
            temp_dir,
            workspace_root,
        }
    }

    pub fn create_live_skill(
        &self,
        category: &str,
        name: &str,
        description: &str,
        content: &str,
    ) -> PathBuf {
        let skill_dir = self.workspace_root.join("skills").join(category).join(name);
        fs::create_dir_all(&skill_dir).unwrap();
        let skill_md =
            format!("---\nname: {name}\ndescription: {description}\n---\n# {name}\n\n{content}\n");
        fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
        skill_dir
    }

    pub fn create_vendor_skill(&self, vendor: &str, name: &str, description: &str) -> PathBuf {
        let skill_dir = self
            .workspace_root
            .join("skills")
            .join("vendor")
            .join(vendor)
            .join(name);
        fs::create_dir_all(&skill_dir).unwrap();
        let skill_md = format!(
            "---\nname: {name}\ndescription: {description}\n---\n# {name}\n\nVendor body.\n"
        );
        fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
        skill_dir
    }

    pub fn create_archived_skill(
        &self,
        category: &str,
        name: &str,
        description: &str,
        archived_date: &str,
        replaced_by: Option<&str>,
        content: &str,
    ) -> PathBuf {
        let skill_dir = self
            .workspace_root
            .join("skills-archive")
            .join(category)
            .join(name);
        fs::create_dir_all(&skill_dir).unwrap();
        let rep_yaml = if let Some(rep) = replaced_by {
            format!("  replaced-by: \"{rep}\"\n")
        } else {
            String::new()
        };
        let skill_md = format!(
            "---\nname: {name}\ndescription: {description}\nmetadata:\n  archived: \"{archived_date}\"\n{rep_yaml}---\n# {name}\n\n{content}\n"
        );
        fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
        skill_dir
    }

    pub fn create_doc(&self, name: &str, content: &str) -> PathBuf {
        let doc_file = self.workspace_root.join("docs").join(name);
        if let Some(parent) = doc_file.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&doc_file, content).unwrap();
        doc_file
    }
}

impl Default for ArtifactsTestHarness {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_agents_md_preserves_frontmatter_and_sorts_skills() {
    let harness = ArtifactsTestHarness::new();
    let agents_file = harness.workspace_root.join("agents/AGENTS.md");
    let initial_content =
        "---\ncustom_title: Test Agents\nversion: 1.0.0\n---\n# Legacy Skills\n- old\n";
    fs::write(&agents_file, initial_content).unwrap();

    harness.create_live_skill("engineering", "skill-b", "Description B", "Content B");
    harness.create_live_skill("engineering", "skill-a", "Description A", "Content A");
    harness.create_live_skill("planning", "plan-skill", "Plan Description", "Plan Content");
    harness.create_archived_skill(
        "engineering",
        "archived-skill",
        "Old Description",
        "2024-01-01",
        None,
        "Old Content",
    );
    harness.create_live_skill(
        "in-progress",
        "draft-skill",
        "Draft Description",
        "Draft Content",
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let updated = engine.generate_agents_md(&live).unwrap();
    assert!(updated);

    let output = fs::read_to_string(&agents_file).unwrap();
    assert!(output.starts_with("---\ncustom_title: Test Agents\nversion: 1.0.0\n---\n"));
    assert!(output.contains("\n# Available Skills\n"));

    let eng_pos = output
        .find("## Engineering")
        .expect("Engineering section missing");
    let plan_pos = output
        .find("## Planning")
        .expect("Planning section missing");
    assert!(
        eng_pos < plan_pos,
        "Engineering must precede Planning in standard order"
    );

    let skill_a_pos = output
        .find("- **skill-a**: Description A")
        .expect("skill-a missing");
    let skill_b_pos = output
        .find("- **skill-b**: Description B")
        .expect("skill-b missing");
    assert!(
        skill_a_pos < skill_b_pos,
        "skill-a must precede skill-b alphabetically"
    );

    assert!(
        !output.contains("archived-skill"),
        "Archived skills must be excluded"
    );
    assert!(
        !output.contains("draft-skill"),
        "In-progress skills must be excluded"
    );
}

#[test]
fn test_readme_contains_badges_install_docs_and_catalog() {
    let harness = ArtifactsTestHarness::new();
    harness.create_doc("architecture.md", "# Architecture Guide\n");
    harness.create_doc("contributing.md", "# Contributing Guide\n");

    harness.create_live_skill(
        "engineering",
        "debug-helper",
        "Helps debug issues",
        "Debug content",
    );
    harness.create_live_skill(
        "planning",
        "spec-writer",
        "Writes specifications",
        "Spec content",
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let updated = engine.generate_readme(&live).unwrap();
    assert!(updated);

    let readme_file = harness.workspace_root.join("README.md");
    let output = fs::read_to_string(&readme_file).unwrap();

    assert!(output.contains("https://skills.sh/b/MAHDTech/agent-skills"));
    assert!(output.contains("## Status"));
    assert!(output.contains("[![CI][badge-ci]][workflow-ci]"));
    assert!(output.contains("npx skills add MAHDTech/agent-skills"));
    assert!(output.contains("bunx skills add MAHDTech/agent-skills"));
    assert!(output.contains("[architecture](docs/architecture.md)"));
    assert!(output.contains("[contributing](docs/contributing.md)"));
    assert!(output.contains("### Engineering"));
    assert!(output.contains("### Planning"));
    assert!(output.contains(
        "- **[debug-helper](skills/engineering/debug-helper/SKILL.md)** - Helps debug issues"
    ));
    assert!(output.contains(
        "- **[spec-writer](skills/planning/spec-writer/SKILL.md)** - Writes specifications"
    ));
}

#[test]
fn test_skills_sh_json_matches_schema_and_groupings() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "skill-two", "Desc 2", "Body 2");
    harness.create_live_skill("engineering", "skill-one", "Desc 1", "Body 1");
    harness.create_live_skill("tooling", "cli-tool", "Desc CLI", "Body CLI");
    harness.create_archived_skill(
        "engineering",
        "old-skill",
        "Old Desc",
        "2024-01-01",
        None,
        "Old Body",
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let updated = engine.generate_skills_sh_json(&live).unwrap();
    assert!(updated);

    let manifest_file = harness.workspace_root.join("skills.sh.json");
    let content = fs::read_to_string(&manifest_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).expect("Valid JSON");

    assert_eq!(
        json["$schema"].as_str(),
        Some("https://skills.sh/schemas/skills.sh.schema.json")
    );
    assert_eq!(json["notGrouped"].as_str(), Some("bottom"));

    let groupings = json["groupings"]
        .as_array()
        .expect("Groupings must be array");
    assert_eq!(groupings.len(), 2);

    let eng_group = &groupings[0];
    assert_eq!(eng_group["title"].as_str(), Some("Engineering"));
    let eng_skills = eng_group["skills"].as_array().expect("Skills array");
    assert_eq!(eng_skills.len(), 2);
    assert_eq!(eng_skills[0].as_str(), Some("skill-one"));
    assert_eq!(eng_skills[1].as_str(), Some("skill-two"));

    let tool_group = &groupings[1];
    assert_eq!(tool_group["title"].as_str(), Some("Tooling"));
    let tool_skills = tool_group["skills"].as_array().expect("Skills array");
    assert_eq!(tool_skills.len(), 1);
    assert_eq!(tool_skills[0].as_str(), Some("cli-tool"));

    assert!(
        !content.contains("old-skill"),
        "Archived skills must be excluded from skills.sh.json"
    );
}

#[test]
fn test_dashboard_root_and_live_categories_assigned_sequential_weights() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "eng-skill", "Eng desc", "Body");
    harness.create_live_skill("planning", "plan-skill", "Plan desc", "Body");

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let updated = engine.generate_dashboard_content(&live, &[]).unwrap();
    assert!(updated);

    let root_index = harness
        .workspace_root
        .join("dashboard/content/skills/_index.md");
    let root_meta = fs::read_to_string(&root_index).unwrap();
    assert!(root_meta.contains("weight = 1"));
    assert!(root_meta.contains("template = \"section.html\""));

    let eng_index = harness
        .workspace_root
        .join("dashboard/content/skills/engineering/_index.md");
    let eng_meta = fs::read_to_string(&eng_index).unwrap();
    assert!(eng_meta.contains("weight = 1"));
    assert!(eng_meta.contains("template = \"section.html\""));

    let plan_index = harness
        .workspace_root
        .join("dashboard/content/skills/planning/_index.md");
    let plan_meta = fs::read_to_string(&plan_index).unwrap();
    assert!(plan_meta.contains("weight = 2"));
    assert!(plan_meta.contains("template = \"section.html\""));

    let skill_index = harness
        .workspace_root
        .join("dashboard/content/skills/engineering/eng-skill/_index.md");
    let skill_meta = fs::read_to_string(&skill_index).unwrap();
    assert!(skill_meta.contains("skill = true"));
    assert!(skill_meta.contains("category = \"engineering\""));
    assert!(skill_meta.contains("template = \"skill.html\""));
}

#[test]
fn test_dashboard_archive_section_weighted_last() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "eng-skill", "Eng desc", "Body");
    harness.create_live_skill("planning", "plan-skill", "Plan desc", "Body");
    harness.create_archived_skill(
        "engineering",
        "arch-skill",
        "Arch desc",
        "2024-01-01",
        None,
        "Body",
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let mut live = Vec::new();
    let mut archived = Vec::new();
    for s in skills {
        if s.tree.is_archive() {
            archived.push(s);
        } else {
            live.push(s);
        }
    }

    let updated = engine.generate_dashboard_content(&live, &archived).unwrap();
    assert!(updated);

    let archive_index = harness
        .workspace_root
        .join("dashboard/content/skills/archive/_index.md");
    let archive_meta = fs::read_to_string(&archive_index).unwrap();

    assert!(archive_meta.contains("weight = 3"));
    assert!(archive_meta.contains("template = \"section.html\""));
    assert!(archive_meta.contains("title = \"Archive\""));
}

#[test]
fn test_dashboard_archive_subsections_and_pages() {
    let harness = ArtifactsTestHarness::new();
    let skill_dir = harness.create_archived_skill(
        "engineering",
        "retired-engine",
        "Retired desc",
        "2024-06-01",
        Some("new-engine"),
        "Archive skill body",
    );
    fs::write(
        skill_dir.join("details.md"),
        "# Additional Details\nSibling markdown content.\n",
    )
    .unwrap();

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let archived: Vec<_> = skills.into_iter().filter(|s| s.tree.is_archive()).collect();

    let updated = engine.generate_dashboard_content(&[], &archived).unwrap();
    assert!(updated);

    let cat_index = harness
        .workspace_root
        .join("dashboard/content/skills/archive/engineering/_index.md");
    assert!(cat_index.is_file());
    let cat_meta = fs::read_to_string(&cat_index).unwrap();
    assert!(cat_meta.contains("title = \"Engineering\""));
    assert!(cat_meta.contains("template = \"section.html\""));

    let skill_index = harness
        .workspace_root
        .join("dashboard/content/skills/archive/engineering/retired-engine/_index.md");
    assert!(skill_index.is_file());
    let skill_content = fs::read_to_string(&skill_index).unwrap();
    assert!(skill_content.contains("skill = true"));
    assert!(skill_content.contains("category = \"engineering\""));
    assert!(skill_content.contains("archived = \"2024-06-01\""));
    assert!(skill_content.contains("replaced_by = \"new-engine\""));
    assert!(skill_content.contains(concat!(
        "source_url = \"",
        "https://",
        "github.com",
        "/MAHDTech/agent-skills/tree/main/skills-archive/engineering/retired-engine\""
    )));
    assert!(skill_content.contains("Archive skill body"));

    let sibling_file = harness
        .workspace_root
        .join("dashboard/content/skills/archive/engineering/retired-engine/details.md");
    assert!(sibling_file.is_file());
    let sib_content = fs::read_to_string(&sibling_file).unwrap();
    assert!(sib_content.contains("skill = false"));
    assert!(sib_content.contains("skill_name = \"retired-engine\""));
    assert!(sib_content.contains("archived = \"2024-06-01\""));
    assert!(sib_content.contains("replaced_by = \"new-engine\""));
    assert!(sib_content.contains(concat!(
        "source_url = \"",
        "https://",
        "github.com",
        "/MAHDTech/agent-skills/tree/main/skills-archive/engineering/retired-engine\""
    )));
    assert!(sib_content.contains("Sibling markdown content."));
}

#[test]
fn test_markdown_transformations_code_block_protection() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "target-skill", "Target", "Target content");

    let source_dir = harness
        .workspace_root
        .join("skills/engineering/source-skill");
    fs::create_dir_all(&source_dir).unwrap();
    let skills_root = harness.workspace_root.join("skills");

    let text_with_code = "Single: `[link](../target-skill/SKILL.md)`\nDouble: ``[link](../target-skill/SKILL.md)``\nFence:\n```markdown\n[link](../target-skill/SKILL.md)\n```\nOutside: [link](../target-skill/SKILL.md)\n";

    let rewritten = rewrite_skill_links(
        text_with_code,
        "skills/engineering/source-skill",
        &source_dir,
        &skills_root,
        None,
    );

    assert!(rewritten.contains("Single: `[link](../target-skill/SKILL.md)`"));
    assert!(rewritten.contains("Double: ``[link](../target-skill/SKILL.md)``"));
    assert!(rewritten.contains("```markdown\n[link](../target-skill/SKILL.md)\n```"));
    assert!(rewritten.contains("Outside: [link](@/skills/engineering/target-skill/_index.md)"));

    let raw_wrapped = "{% raw %}\ncode block inside raw\n{% endraw %}\n";
    let stripped = strip_legacy_raw_wrapper(raw_wrapped);
    assert_eq!(stripped, "code block inside raw");
}

#[test]
fn test_markdown_transformations_relative_link_rewriting() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "target", "Target", "Target");
    harness.create_live_skill("in-progress", "draft", "Draft", "Draft");

    let source_dir = harness.workspace_root.join("skills/engineering/source");
    fs::create_dir_all(&source_dir).unwrap();
    let skills_root = harness.workspace_root.join("skills");

    // 1. Valid relative link to existing skill
    let input1 = "[Target](../target/SKILL.md)";
    let out1 = rewrite_skill_links(
        input1,
        "skills/engineering/source",
        &source_dir,
        &skills_root,
        None,
    );
    assert_eq!(out1, "[Target](@/skills/engineering/target/_index.md)");

    // 2. Relative link with anchor
    let input2 = "[Target Section](../target/SKILL.md#options)";
    let out2 = rewrite_skill_links(
        input2,
        "skills/engineering/source",
        &source_dir,
        &skills_root,
        None,
    );
    assert_eq!(
        out2,
        "[Target Section](@/skills/engineering/target/_index.md#options)"
    );

    // 3. Link to non-existent file remains untouched
    let input3 = "[Missing](../missing/SKILL.md)";
    let out3 = rewrite_skill_links(
        input3,
        "skills/engineering/source",
        &source_dir,
        &skills_root,
        None,
    );
    assert_eq!(out3, "[Missing](../missing/SKILL.md)");

    // 4. Link to lifecycle category (in-progress) is not rewritten
    let input4 = "[Draft](../../skills/in-progress/draft/SKILL.md)";
    let out4 = rewrite_skill_links(
        input4,
        "skills/engineering/source",
        &source_dir,
        &skills_root,
        None,
    );
    assert_eq!(out4, "[Draft](../../skills/in-progress/draft/SKILL.md)");

    // 5. External link remains untouched
    let input5 = "[Web](https://example.com/SKILL.md)";
    let out5 = rewrite_skill_links(
        input5,
        "skills/engineering/source",
        &source_dir,
        &skills_root,
        None,
    );
    assert_eq!(out5, "[Web](https://example.com/SKILL.md)");
}

#[test]
fn test_markdown_transformations_shortcode_escaping() {
    // 1. Inline function call escaped
    let inline_call = "Image: {{ resize_image(path=\"a.png\", width=100) }}";
    let esc1 = escape_zola_shortcodes(inline_call);
    assert_eq!(
        esc1,
        "Image: {{/* resize_image(path=\"a.png\", width=100) */}}"
    );

    // 2. Variable interpolation without call is not treated as function call
    let var_expr = "Hello {{ user.name }}!";
    let esc2 = escape_zola_shortcodes(var_expr);
    assert_eq!(esc2, "Hello {{ user.name }}!");

    // 3. Balanced body shortcode
    let balanced = "{% note(title=\"Tip\") %} Important advice {% end %}";
    let esc3 = escape_zola_shortcodes(balanced);
    assert_eq!(
        esc3,
        "{%/* note(title=\"Tip\") */%} Important advice {%/* end */%}"
    );

    // 4. Unbalanced body shortcode has opening split by Unicode Word Joiner
    let unbalanced = "{% note() %} Unclosed body text";
    let esc4 = escape_zola_shortcodes(unbalanced);
    assert!(esc4.contains("{\u{2060}%"));

    // 5. Nested body shortcode has opening split by Unicode Word Joiner
    let nested = "{% outer() %} {% inner() %} {% end %} {% end %}";
    let esc5 = escape_zola_shortcodes(nested);
    assert!(esc5.contains("{\u{2060}%"));
}

#[test]
fn test_resource_mirroring_and_suppression() {
    let harness = ArtifactsTestHarness::new();
    let skill_res = harness
        .workspace_root
        .join("skills/engineering/my-skill/resources");
    fs::create_dir_all(skill_res.join("manual")).unwrap();

    fs::write(skill_res.join("manual/data.json"), "{\"status\":\"ok\"}\n").unwrap();
    fs::write(skill_res.join("manual/guide.md"), "# Guide\nGuide body.\n").unwrap();
    fs::write(skill_res.join("manual/index.md"), "# Index\nIndex body.\n").unwrap();

    #[cfg(unix)]
    {
        let _ =
            std::os::unix::fs::symlink("nonexistent_path", skill_res.join("manual/broken_link"));
    }

    let dest = harness
        .workspace_root
        .join("dashboard/content/skills/engineering/my-skill/resources");
    let result = sync_resources(
        &skill_res,
        &dest,
        "skills/engineering/my-skill/resources",
        "engineering",
        "my-skill",
        false,
        None,
    );
    assert!(
        result.is_ok(),
        "sync_resources should succeed even with broken symlinks"
    );

    // Root and child directories must have _index.md with render = false
    let root_index = dest.join("_index.md");
    assert!(root_index.is_file());
    assert!(
        fs::read_to_string(&root_index)
            .unwrap()
            .contains("render = false")
    );

    let child_index = dest.join("manual/_index.md");
    assert!(child_index.is_file());
    assert!(
        fs::read_to_string(&child_index)
            .unwrap()
            .contains("render = false")
    );

    // Plain files copied verbatim
    let copied_json = dest.join("manual/data.json");
    assert!(copied_json.is_file());
    assert_eq!(
        fs::read_to_string(&copied_json).unwrap(),
        "{\"status\":\"ok\"}\n"
    );

    // Markdown file transformed with metadata
    let guide_file = dest.join("manual/guide.md");
    assert!(guide_file.is_file());
    let guide_content = fs::read_to_string(&guide_file).unwrap();
    assert!(guide_content.contains("title = \"guide\""));
    assert!(guide_content.contains("skill = false"));
    assert!(guide_content.contains("Guide body."));

    // index.md renamed to index-page.md to avoid collision with section _index.md
    assert!(!dest.join("manual/index.md").exists());
    let index_page = dest.join("manual/index-page.md");
    assert!(index_page.is_file());
    let page_content = fs::read_to_string(&index_page).unwrap();
    assert!(page_content.contains("title = \"index-page\""));
    assert!(page_content.contains("skill = false"));
    assert!(page_content.contains("Index body."));
}

#[test]
#[allow(unsafe_code)]
#[allow(warnings)]
fn test_environment_flags_respected() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "skill-env", "Env desc", "Env body");

    // Test SKILLS_SKIP_DASHBOARD env flag
    unsafe {
        std::env::set_var("SKILLS_SKIP_DASHBOARD", "1");
    }
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().skip_dashboard);
    unsafe {
        std::env::remove_var("SKILLS_SKIP_DASHBOARD");
    }

    // Test SKILLS_NO_STAGE env flag
    unsafe {
        std::env::set_var("SKILLS_NO_STAGE", "1");
    }
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().no_stage);
    unsafe {
        std::env::remove_var("SKILLS_NO_STAGE");
    }

    // Test SKILLS_REPO_ONLY env flag
    unsafe {
        std::env::set_var("SKILLS_REPO_ONLY", "1");
    }
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().repo_only);
    unsafe {
        std::env::remove_var("SKILLS_REPO_ONLY");
    }

    // Test CI env flag
    unsafe {
        std::env::set_var("CI", "1");
    }
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().repo_only);
    unsafe {
        std::env::remove_var("CI");
    }

    // Behavioral test: skip_dashboard skips generating dashboard content
    let skip_options = ArtifactsOptions {
        skip_dashboard: true,
        ..Default::default()
    };
    let engine_skip = ArtifactsEngine::with_options(&harness.workspace_root, skip_options);
    let generated = engine_skip.generate_dashboard_content(&[], &[]).unwrap();
    assert!(!generated);
    assert!(
        !harness
            .workspace_root
            .join("dashboard/content/skills/engineering")
            .exists()
    );

    // Behavioral test: no_stage skips git staging
    let no_stage_options = ArtifactsOptions {
        no_stage: true,
        ..Default::default()
    };
    let engine_no_stage = ArtifactsEngine::with_options(&harness.workspace_root, no_stage_options);
    let staged = engine_no_stage
        .stage_files(&[harness.workspace_root.join("README.md")])
        .unwrap();
    assert!(staged.is_empty());
}

#[test]
fn test_dashboard_live_skill_with_siblings_and_mermaid() {
    let harness = ArtifactsTestHarness::new();
    let skill_dir = harness.create_live_skill(
        "engineering",
        "diagram-skill",
        "Diagram skill desc",
        "```mermaid\ngraph TD;\nA-->B;\n```\nContent with mermaid.",
    );
    fs::write(
        skill_dir.join("guide.md"),
        "# Sibling Guide\nThis is a sibling guide.\n",
    )
    .unwrap();

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let updated = engine.generate_dashboard_content(&live, &[]).unwrap();
    assert!(updated);

    let skill_index = harness
        .workspace_root
        .join("dashboard/content/skills/engineering/diagram-skill/_index.md");
    assert!(skill_index.is_file());
    let skill_content = fs::read_to_string(&skill_index).unwrap();
    assert!(skill_content.contains("mermaid = true"));
    assert!(skill_content.contains("skill = true"));

    let sibling_file = harness
        .workspace_root
        .join("dashboard/content/skills/engineering/diagram-skill/guide.md");
    assert!(sibling_file.is_file());
    let sib_content = fs::read_to_string(&sibling_file).unwrap();
    assert!(sib_content.contains("skill = false"));
    assert!(sib_content.contains("skill_name = \"diagram-skill\""));
    assert!(sib_content.contains("Sibling Guide"));
}

#[test]
fn test_dashboard_content_with_category_and_skill_filter() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "eng-one", "Desc", "Body");
    harness.create_live_skill("engineering", "eng-two", "Desc", "Body");
    harness.create_live_skill("planning", "plan-one", "Desc", "Body");

    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    // Test with category filter
    let cat_options = ArtifactsOptions {
        category_filter: Some("engineering".to_string()),
        ..Default::default()
    };
    let engine_cat = ArtifactsEngine::with_options(&harness.workspace_root, cat_options);
    let updated = engine_cat.generate_dashboard_content(&live, &[]).unwrap();
    assert!(updated);

    assert!(
        harness
            .workspace_root
            .join("dashboard/content/skills/engineering/eng-one/_index.md")
            .exists()
    );
    assert!(
        !harness
            .workspace_root
            .join("dashboard/content/skills/planning/plan-one/_index.md")
            .exists()
    );

    // Test with skill filter
    let skill_options = ArtifactsOptions {
        skill_filter: Some("eng-two".to_string()),
        ..Default::default()
    };
    let engine_skill = ArtifactsEngine::with_options(&harness.workspace_root, skill_options);
    let updated_skill = engine_skill.generate_dashboard_content(&live, &[]).unwrap();
    assert!(updated_skill);

    assert!(
        harness
            .workspace_root
            .join("dashboard/content/skills/engineering/eng-two/_index.md")
            .exists()
    );
}

#[test]
fn test_sync_resources_archived_metadata() {
    let harness = ArtifactsTestHarness::new();
    let skill_res = harness
        .workspace_root
        .join("skills-archive/engineering/old-skill/resources");
    fs::create_dir_all(skill_res.join("manual")).unwrap();
    fs::write(
        skill_res.join("manual/ref.md"),
        "# Reference\nReference body.\n",
    )
    .unwrap();

    let dest = harness
        .workspace_root
        .join("dashboard/content/skills/archive/engineering/old-skill/resources");
    let meta = Some((
        "2024-05-01",
        Some("new-skill"),
        "https://github.com/MAHDTech/agent-skills",
    ));
    let result = sync_resources(
        &skill_res,
        &dest,
        "skills/archive/engineering/old-skill/resources",
        "engineering",
        "old-skill",
        true,
        meta,
    );
    assert!(result.is_ok());

    let ref_file = dest.join("manual/ref.md");
    assert!(ref_file.is_file());
    let ref_content = fs::read_to_string(&ref_file).unwrap();
    assert!(ref_content.contains("archived = \"2024-05-01\""));
    assert!(ref_content.contains("replaced_by = \"new-skill\""));
    assert!(ref_content.contains("source_url = \"https://github.com/MAHDTech/agent-skills\""));
    assert!(ref_content.contains("skill = false"));
}

#[test]
fn test_markdown_transformations_archive_link_rewriting() {
    let harness = ArtifactsTestHarness::new();
    harness.create_archived_skill(
        "engineering",
        "legacy-tool",
        "Legacy",
        "2024-01-01",
        None,
        "Legacy content",
    );

    let source_dir = harness
        .workspace_root
        .join("skills/engineering/modern-tool");
    fs::create_dir_all(&source_dir).unwrap();
    let skills_root = harness.workspace_root.join("skills");
    let archive_root = harness.workspace_root.join("skills-archive");

    let input = "[Legacy Tool](../../../skills-archive/engineering/legacy-tool/SKILL.md)";
    let out = rewrite_skill_links(
        input,
        "skills/engineering/modern-tool",
        &source_dir,
        &skills_root,
        Some(&archive_root),
    );
    assert_eq!(
        out,
        "[Legacy Tool](@/skills-archive/engineering/legacy-tool/_index.md)"
    );
}

#[test]
fn test_discovery_index_exact_bytes() {
    let harness = ArtifactsTestHarness::new();
    let skill_dir = harness.create_live_skill(
        "engineering",
        "alpha-tool",
        "Alpha description.",
        "Alpha body.",
    );
    let raw_bytes = fs::read(skill_dir.join("SKILL.md")).unwrap();
    let expected_hex = SkillDownloader::compute_sha256(&raw_bytes);

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    assert!(engine.generate_discovery_index(&live).unwrap());

    let v020_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_AGENT_SKILLS_DIR)
        .join("index.json");
    let v010_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_SKILLS_DIR)
        .join("index.json");

    let v020_content = fs::read_to_string(&v020_path).unwrap();
    let expected_v020 = format!(
        "{{\n  \"$schema\": \"{SKILLS_DISCOVERY_SCHEMA_V0_2_0}\",\n  \"skills\": [\n    {{\n      \"name\": \"alpha-tool\",\n      \"type\": \"skill-md\",\n      \"description\": \"Alpha description.\",\n      \"url\": \"/.well-known/agent-skills/alpha-tool/SKILL.md\",\n      \"digest\": \"sha256:{expected_hex}\"\n    }}\n  ]\n}}\n"
    );
    assert_eq!(v020_content, expected_v020);
    assert!(!v020_content.ends_with("\n\n"));

    let v010_content = fs::read_to_string(&v010_path).unwrap();
    let expected_v010 = "{\n  \"skills\": [\n    {\n      \"name\": \"alpha-tool\",\n      \"description\": \"Alpha description.\",\n      \"files\": [\n        \"SKILL.md\"\n      ]\n    }\n  ]\n}\n";
    assert_eq!(v010_content, expected_v010);
    assert!(!v010_content.ends_with("\n\n"));
}

#[test]
fn test_discovery_index_sorted_by_name_across_categories() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("writing", "zebra-guide", "Zebra desc", "Body");
    harness.create_live_skill("tooling", "alpha-tool", "Alpha desc", "Body");
    harness.create_live_skill("engineering", "middle-build", "Middle desc", "Body");

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    assert!(engine.generate_discovery_index(&live).unwrap());

    for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
        let index_path = harness
            .workspace_root
            .join("dashboard/static")
            .join(rel_dir)
            .join("index.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(index_path).unwrap()).unwrap();
        let names: Vec<&str> = parsed["skills"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["alpha-tool", "middle-build", "zebra-guide"]);
    }
}

#[test]
fn test_discovery_index_digest_matches_raw_on_disk_bytes() {
    let harness = ArtifactsTestHarness::new();
    let skill_dir = harness.workspace_root.join("skills/engineering/raw-check");
    fs::create_dir_all(&skill_dir).unwrap();
    let skill_md_path = skill_dir.join("SKILL.md");

    let raw_bytes =
        b"---   \r\nname: raw-check\r\ndescription: Raw bytes test\r\n...\r\n# raw-check\r\n\r\nBody line.\r\n";
    fs::write(&skill_md_path, raw_bytes).unwrap();

    let raw_str = std::str::from_utf8(raw_bytes).unwrap();
    let reconstructed_skill = SkillParser::parse_str_with_path(raw_str, &skill_md_path)
        .unwrap()
        .into_skill(
            "skills/engineering/raw-check/SKILL.md",
            SkillCategory::Engineering,
        );
    assert_ne!(
        reconstructed_skill.raw.as_bytes(),
        raw_bytes,
        "ParsedSkill::into_skill reconstruction must differ from raw on-disk bytes for this test"
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    assert!(
        engine
            .generate_discovery_index(&[reconstructed_skill])
            .unwrap()
    );

    let v020_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_AGENT_SKILLS_DIR)
        .join("index.json");
    let v020_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(v020_path).unwrap()).unwrap();

    let expected_digest = format!("sha256:{}", SkillDownloader::compute_sha256(raw_bytes));
    assert_eq!(
        v020_json["skills"][0]["digest"].as_str(),
        Some(expected_digest.as_str())
    );
}

#[test]
fn test_discovery_index_digest_falls_back_to_raw_when_missing_on_disk() {
    let harness = ArtifactsTestHarness::new();
    let raw = "---\nname: in-memory-fallback\ndescription: In-memory fallback test\n---\n# in-memory-fallback\n\nFallback body.\n";
    let missing_rel = PathBuf::from("skills/engineering/in-memory-fallback/SKILL.md");
    assert!(!harness.workspace_root.join(&missing_rel).exists());

    let skill = Skill::builder()
        .path(missing_rel)
        .dir_name("in-memory-fallback")
        .category(SkillCategory::Engineering)
        .promoted(true)
        .tree(SkillTree::Live)
        .frontmatter(
            SkillFrontmatter::builder()
                .name("in-memory-fallback")
                .description("In-memory fallback test")
                .build(),
        )
        .content("# in-memory-fallback\n")
        .raw(raw)
        .build();

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    assert!(engine.generate_discovery_index(&[skill]).unwrap());

    let v020_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_AGENT_SKILLS_DIR)
        .join("index.json");
    let v020_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(v020_path).unwrap()).unwrap();

    let expected_digest = format!("sha256:{}", SkillDownloader::compute_sha256(raw.as_bytes()));
    assert_eq!(
        v020_json["skills"][0]["digest"].as_str(),
        Some(expected_digest.as_str())
    );
}

#[test]
fn test_discovery_index_v010_lists_only_skill_md() {
    let harness = ArtifactsTestHarness::new();
    let skill_dir = harness.create_live_skill(
        "engineering",
        "resource-rich",
        "Skill with resources",
        "Body",
    );
    fs::create_dir_all(skill_dir.join("resources/manual")).unwrap();
    fs::write(skill_dir.join("resources/manual/guide.md"), "# Guide\n").unwrap();
    fs::create_dir_all(skill_dir.join("agents")).unwrap();
    fs::write(
        skill_dir.join("agents/openai.yaml"),
        "policy:\n  allow_implicit_invocation: true\n",
    )
    .unwrap();

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();
    assert_eq!(live[0].resources.len(), 1);

    assert!(engine.generate_discovery_index(&live).unwrap());

    let v010_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_SKILLS_DIR)
        .join("index.json");
    let v010_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(v010_path).unwrap()).unwrap();
    assert_eq!(
        v010_json["skills"][0]["files"],
        serde_json::json!(["SKILL.md"])
    );

    let v020_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_AGENT_SKILLS_DIR)
        .join("index.json");
    let v020_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(v020_path).unwrap()).unwrap();
    assert!(v020_json["skills"][0].get("files").is_none());
}

#[test]
fn test_discovery_index_excludes_vendor_archive_and_in_progress() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "custom-live", "Custom live skill", "Body");
    harness.create_vendor_skill("cloudflare", "vendor-skill", "Vendor snapshot");
    harness.create_archived_skill(
        "engineering",
        "retired-skill",
        "Retired skill",
        "2025-01-01",
        None,
        "Old body",
    );
    harness.create_live_skill("in-progress", "draft-work", "Draft skill", "Draft body");

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    assert!(engine.generate_discovery_index(&live).unwrap());

    for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
        let index_path = harness
            .workspace_root
            .join("dashboard/static")
            .join(rel_dir)
            .join("index.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(index_path).unwrap()).unwrap();
        let entries = parsed["skills"].as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["name"].as_str(), Some("custom-live"));
    }
}

#[test]
fn test_discovery_index_excludes_unsafe_names() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "safe-name-1", "Safe skill", "Body");

    let mut skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let unsafe_names = [
        "Upper-Case",
        "-leading",
        "trailing-",
        "double--dash",
        "../escape",
        "nested/name",
        "",
    ];

    for bad_name in unsafe_names {
        skills.push(
            Skill::builder()
                .path(PathBuf::from("skills/engineering/bad/SKILL.md"))
                .dir_name("bad")
                .category(SkillCategory::Engineering)
                .promoted(true)
                .tree(SkillTree::Live)
                .frontmatter(
                    SkillFrontmatter::builder()
                        .name(bad_name)
                        .description("Unsafe name fixture")
                        .build(),
                )
                .content("# Bad\n")
                .raw("---\n---\n# Bad\n")
                .build(),
        );
    }

    let selected = select_discovery_skills(&skills);
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name(), "safe-name-1");

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    assert!(engine.generate_discovery_index(&skills).unwrap());

    for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
        let index_path = harness
            .workspace_root
            .join("dashboard/static")
            .join(rel_dir)
            .join("index.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(index_path).unwrap()).unwrap();
        let entries = parsed["skills"].as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["name"].as_str(), Some("safe-name-1"));
    }
}

#[test]
fn test_discovery_index_trims_description() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill(
        "engineering",
        "trim-target",
        "\"  Trimmed summary text.  \\n\"",
        "Body",
    );

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    assert!(engine.generate_discovery_index(&live).unwrap());

    for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
        let index_path = harness
            .workspace_root
            .join("dashboard/static")
            .join(rel_dir)
            .join("index.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(index_path).unwrap()).unwrap();
        assert_eq!(
            parsed["skills"][0]["description"].as_str(),
            Some("Trimmed summary text.")
        );
    }
}

#[test]
fn test_discovery_index_skipped_with_category_or_skill_filter() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "eng-one", "Desc", "Body");

    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let cat_engine = ArtifactsEngine::with_options(
        &harness.workspace_root,
        ArtifactsOptions {
            category_filter: Some("engineering".to_string()),
            no_stage: true,
            ..Default::default()
        },
    );
    assert!(!cat_engine.generate_discovery_index(&live).unwrap());
    let cat_summary = cat_engine.generate_all().unwrap();
    assert!(!cat_summary.discovery_index_updated);
    assert!(!harness.workspace_root.join(WELL_KNOWN_STATIC_DIR).exists());

    let skill_engine = ArtifactsEngine::with_options(
        &harness.workspace_root,
        ArtifactsOptions {
            skill_filter: Some("eng-one".to_string()),
            no_stage: true,
            ..Default::default()
        },
    );
    assert!(!skill_engine.generate_discovery_index(&live).unwrap());
    let skill_summary = skill_engine.generate_all().unwrap();
    assert!(!skill_summary.discovery_index_updated);
    assert!(!harness.workspace_root.join(WELL_KNOWN_STATIC_DIR).exists());
}

#[test]
fn test_discovery_index_skipped_with_skip_dashboard_or_repo_only() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "eng-one", "Desc", "Body");

    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    let skip_engine = ArtifactsEngine::with_options(
        &harness.workspace_root,
        ArtifactsOptions {
            skip_dashboard: true,
            no_stage: true,
            ..Default::default()
        },
    );
    assert!(!skip_engine.generate_discovery_index(&live).unwrap());
    let skip_summary = skip_engine.generate_all().unwrap();
    assert!(!skip_summary.discovery_index_updated);
    assert!(!harness.workspace_root.join(WELL_KNOWN_STATIC_DIR).exists());

    let repo_engine = ArtifactsEngine::with_options(
        &harness.workspace_root,
        ArtifactsOptions {
            repo_only: true,
            no_stage: true,
            ..Default::default()
        },
    );
    assert!(!repo_engine.generate_discovery_index(&live).unwrap());
    let repo_summary = repo_engine.generate_all().unwrap();
    assert!(!repo_summary.discovery_index_updated);
    assert!(!harness.workspace_root.join(WELL_KNOWN_STATIC_DIR).exists());
}

#[test]
fn test_discovery_index_empty_catalog_emits_empty_skills_array() {
    let harness = ArtifactsTestHarness::new();
    harness.create_vendor_skill("cloudflare", "vendor-only", "Vendor only skill");

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let skills = SkillParser::discover_skills(&harness.workspace_root).unwrap();
    let live: Vec<_> = skills.into_iter().filter(|s| s.tree.is_live()).collect();

    assert!(engine.generate_discovery_index(&live).unwrap());

    let v020_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_AGENT_SKILLS_DIR)
        .join("index.json");
    let v010_path = harness
        .workspace_root
        .join("dashboard/static")
        .join(WELL_KNOWN_SKILLS_DIR)
        .join("index.json");

    assert_eq!(
        fs::read_to_string(v020_path).unwrap(),
        format!("{{\n  \"$schema\": \"{SKILLS_DISCOVERY_SCHEMA_V0_2_0}\",\n  \"skills\": []\n}}\n")
    );
    assert_eq!(
        fs::read_to_string(v010_path).unwrap(),
        "{\n  \"skills\": []\n}\n"
    );
}

#[test]
fn test_generate_all_writes_discovery_index_and_sets_summary() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "sync-check", "Sync check desc", "Body");

    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(&harness.workspace_root)
        .output();

    let engine = ArtifactsEngine::new(&harness.workspace_root);
    let summary = engine.generate_all().unwrap();

    assert!(summary.discovery_index_updated);
    let well_known_dir = harness.workspace_root.join(WELL_KNOWN_STATIC_DIR);
    assert!(summary.staged_files.contains(&well_known_dir));
    assert!(
        harness
            .workspace_root
            .join("dashboard/static")
            .join(WELL_KNOWN_AGENT_SKILLS_DIR)
            .join("index.json")
            .is_file()
    );
    assert!(
        harness
            .workspace_root
            .join("dashboard/static")
            .join(WELL_KNOWN_SKILLS_DIR)
            .join("index.json")
            .is_file()
    );
}

#[test]
fn test_skills_sh_json_empty_catalog_keeps_groupings_key() {
    let harness = ArtifactsTestHarness::new();
    let engine = ArtifactsEngine::new(&harness.workspace_root);
    assert!(engine.generate_skills_sh_json(&[]).unwrap());

    let manifest_file = harness.workspace_root.join("skills.sh.json");
    let content = fs::read_to_string(&manifest_file).unwrap();
    assert_eq!(
        content,
        "{\n  \"$schema\": \"https://skills.sh/schemas/skills.sh.schema.json\",\n  \"notGrouped\": \"bottom\",\n  \"groupings\": []\n}\n"
    );
}
