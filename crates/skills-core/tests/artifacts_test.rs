#![allow(clippy::similar_names)]
// cspell:words bunx
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use skills_core::{
    escape_zola_shortcodes, rewrite_skill_links, strip_legacy_raw_wrapper, sync_resources,
    ArtifactsEngine, ArtifactsOptions, SkillParser,
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
    assert!(fs::read_to_string(&root_index)
        .unwrap()
        .contains("render = false"));

    let child_index = dest.join("manual/_index.md");
    assert!(child_index.is_file());
    assert!(fs::read_to_string(&child_index)
        .unwrap()
        .contains("render = false"));

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
#[allow(warnings)]
fn test_environment_flags_respected() {
    let harness = ArtifactsTestHarness::new();
    harness.create_live_skill("engineering", "skill-env", "Env desc", "Env body");

    // Test SKILLS_SKIP_DASHBOARD env flag
    std::env::set_var("SKILLS_SKIP_DASHBOARD", "1");
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().skip_dashboard);
    std::env::remove_var("SKILLS_SKIP_DASHBOARD");

    // Test SKILLS_NO_STAGE env flag
    std::env::set_var("SKILLS_NO_STAGE", "1");
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().no_stage);
    std::env::remove_var("SKILLS_NO_STAGE");

    // Test SKILLS_REPO_ONLY env flag
    std::env::set_var("SKILLS_REPO_ONLY", "1");
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().repo_only);
    std::env::remove_var("SKILLS_REPO_ONLY");

    // Test CI env flag
    std::env::set_var("CI", "1");
    let engine = ArtifactsEngine::from_env(&harness.workspace_root);
    assert!(engine.options().repo_only);
    std::env::remove_var("CI");

    // Behavioral test: skip_dashboard skips generating dashboard content
    let skip_options = ArtifactsOptions {
        skip_dashboard: true,
        ..Default::default()
    };
    let engine_skip = ArtifactsEngine::with_options(&harness.workspace_root, skip_options);
    let generated = engine_skip.generate_dashboard_content(&[], &[]).unwrap();
    assert!(!generated);
    assert!(!harness
        .workspace_root
        .join("dashboard/content/skills/engineering")
        .exists());

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
