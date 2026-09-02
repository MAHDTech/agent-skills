//! Comprehensive unit and integration tests for `skills-core` domain models and error types.

use skills_core::error::SkillError;
use skills_core::models::{
    LintReport, ResourceFile, ResourceKind, Skill, SkillCategory, SkillFrontmatter, SkillGrouping,
    SkillsManifest, SyncOptions, SyncResult,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn test_skill_category_serialization_and_display() {
    let cat = SkillCategory::GameDevelopment;
    let serialized = serde_json::to_string(&cat).unwrap();
    assert_eq!(serialized, "\"game-development\"");

    let deserialized: SkillCategory = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, SkillCategory::GameDevelopment);
    assert_eq!(deserialized.as_str(), "game-development");
    assert_eq!(deserialized.title(), "Game Development");
    assert!(deserialized.is_promoted());
    assert!(!deserialized.is_lifecycle());

    // Lifecycle category
    let in_progress = SkillCategory::InProgress;
    assert_eq!(in_progress.as_str(), "in-progress");
    assert!(in_progress.is_lifecycle());
    assert!(!in_progress.is_promoted());

    // Custom category
    let custom = SkillCategory::from_str("my-custom-cat").unwrap();
    assert_eq!(custom, SkillCategory::Custom("my-custom-cat".to_string()));
    assert_eq!(custom.as_str(), "my-custom-cat");
    assert_eq!(custom.title(), "my-custom-cat");
}

#[test]
fn test_skill_frontmatter_deserialization_standard() {
    let yaml_doc = r"
name: domain-modeling
description: Build and sharpen domain models.
metadata:
  source: mattpocock/skills
  license: MIT
";

    let fm: SkillFrontmatter = serde_yaml::from_str(yaml_doc).unwrap();
    assert_eq!(fm.name, "domain-modeling");
    assert_eq!(fm.description, "Build and sharpen domain models.");
    assert!(fm.resources.is_none());
    assert!(fm.disable_model_invocation.is_none());

    let meta = fm.metadata.expect("metadata should be present");
    assert_eq!(
        meta.get("source").map(String::as_str),
        Some("mattpocock/skills")
    );
    assert_eq!(meta.get("license").map(String::as_str), Some("MIT"));
}

#[test]
fn test_skill_frontmatter_with_optional_fields_and_kebab_aliases() {
    let yaml_doc = r"
name: test-runner
description: Runs test suite on demand.
disable-model-invocation: true
argument-hint: '<filter>'
context: fork
agent: general-purpose
resources:
  - https://example.com/docs.md
  - https://example.com/llms.txt
";

    let fm: SkillFrontmatter = serde_yaml::from_str(yaml_doc).unwrap();
    assert_eq!(fm.name, "test-runner");
    assert_eq!(fm.disable_model_invocation, Some(true));
    assert_eq!(fm.argument_hint.as_deref(), Some("<filter>"));
    assert_eq!(fm.context.as_deref(), Some("fork"));
    assert_eq!(fm.agent.as_deref(), Some("general-purpose"));
    let res = fm.resources.unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], "https://example.com/docs.md");
}

#[test]
fn test_skill_frontmatter_builder() {
    let mut meta = HashMap::new();
    meta.insert("author".to_string(), "MAHDTech".to_string());

    let fm = SkillFrontmatter::builder()
        .name("sculpt-code")
        .description("Refactor cleanly.")
        .metadata(Some(meta))
        .build();

    assert_eq!(fm.name, "sculpt-code");
    assert_eq!(fm.description, "Refactor cleanly.");
    assert_eq!(fm.metadata.unwrap().get("author").unwrap(), "MAHDTech");
}

#[test]
fn test_full_skill_domain_model() {
    let fm = SkillFrontmatter::builder()
        .name("acp")
        .description("Agent Client Protocol integration.")
        .build();

    let resource = ResourceFile::builder()
        .relative_path(PathBuf::from("resources/manual/spec.md"))
        .absolute_path(PathBuf::from(
            "/repo/skills/engineering/acp/resources/manual/spec.md",
        ))
        .kind(ResourceKind::Manual)
        .content(Some("# ACP Spec".to_string()))
        .build();

    let skill = Skill::builder()
        .path(PathBuf::from("skills/engineering/acp/SKILL.md"))
        .dir_name("acp".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# ACP\n\nContent here.".to_string())
        .raw("---\nname: acp\n---\n# ACP".to_string())
        .resources(vec![resource])
        .build();

    assert_eq!(skill.name(), "acp");
    assert_eq!(skill.description(), "Agent Client Protocol integration.");
    assert_eq!(skill.category, SkillCategory::Engineering);
    assert!(skill.promoted);
    assert!(!skill.is_user_invoked());
    assert!(!skill.is_forked());
    assert_eq!(skill.resources.len(), 1);
    assert_eq!(skill.resources[0].kind, ResourceKind::Manual);
}

#[test]
fn test_skills_manifest_serialization() {
    let manifest = SkillsManifest::builder()
        .not_grouped(Some("bottom".to_string()))
        .groupings(vec![SkillGrouping::builder()
            .title("Engineering")
            .description("The core build loop.")
            .skills(vec!["acp".to_string(), "tdd".to_string()])
            .build()])
        .build();

    let json_str = serde_json::to_string_pretty(&manifest).unwrap();
    assert!(json_str.contains("\"$schema\": \"https://skills.sh/schemas/skills.sh.schema.json\""));
    assert!(json_str.contains("\"notGrouped\": \"bottom\""));
    assert!(json_str.contains("\"title\": \"Engineering\""));

    let parsed: SkillsManifest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.groupings.len(), 1);
    assert_eq!(parsed.groupings[0].skills, vec!["acp", "tdd"]);
}

#[test]
fn test_lint_report_aggregation() {
    let mut report = LintReport::new();
    assert!(!report.has_errors());
    assert_eq!(report.error_count(), 0);

    report.add_warning(
        PathBuf::from("skills/in-progress/draft/SKILL.md"),
        "lifecycle-draft",
        "Draft skill is not promoted",
    );
    assert!(!report.has_errors());
    assert_eq!(report.warning_count(), 1);

    report.add_error(
        PathBuf::from("skills/engineering/broken/SKILL.md"),
        "no-em-dashes",
        "Contains Unicode U+2014 em-dash",
    );
    assert!(report.has_errors());
    assert_eq!(report.error_count(), 1);
    assert_eq!(report.warning_count(), 1);
}

#[test]
fn test_sync_options_and_result() {
    let opts = SyncOptions::builder()
        .category(Some("engineering".to_string()))
        .repo_only(true)
        .build();

    assert_eq!(opts.category.as_deref(), Some("engineering"));
    assert!(opts.repo_only);
    assert!(!opts.skip_dashboard);

    let res = SyncResult::builder()
        .skills_count(42)
        .agents_updated(true)
        .readme_updated(true)
        .dashboard_pages_generated(85)
        .staged_files(vec![PathBuf::from("README.md")])
        .build();

    assert_eq!(res.skills_count, 42);
    assert!(res.agents_updated);
    assert_eq!(res.staged_files.len(), 1);
}

#[test]
fn test_skill_error_formatting_and_chaining() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = SkillError::io(PathBuf::from("skills/engineering/foo/SKILL.md"), io_err);
    assert!(err
        .to_string()
        .contains("I/O error at 'skills/engineering/foo/SKILL.md'"));

    let val_err = SkillError::validation(
        PathBuf::from("skills/engineering/bad/SKILL.md"),
        "Missing mandatory field 'description'",
    );
    assert!(val_err
        .to_string()
        .contains("Missing mandatory field 'description'"));

    let lock_err = SkillError::LockTimeout {
        path: PathBuf::from("skills.json.lock"),
        timeout_secs: 10,
    };
    assert!(lock_err.to_string().contains("Lock acquisition timed out"));
}
