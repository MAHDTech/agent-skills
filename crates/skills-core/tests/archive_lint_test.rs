// cspell:words unpromoted
//! Comprehensive integration test suite for skills-archive modeling, metadata lint, and cross-reference policy.

use skills_core::error::SkillError;
use skills_core::lint::SkillLinter;
use skills_core::models::{LintSeverity, Skill, SkillCategory, SkillFrontmatter, SkillTree};
use skills_core::parser::SkillParser;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn build_test_skill(
    path: &str,
    dir_name: &str,
    category: SkillCategory,
    name: &str,
    tree: SkillTree,
    group: Option<&str>,
    archived: Option<&str>,
    replaced_by: Option<&str>,
    content: &str,
) -> Skill {
    let mut meta = HashMap::new();
    if let Some(g) = group {
        meta.insert("group".to_string(), g.to_string());
    }
    if let Some(a) = archived {
        meta.insert("archived".to_string(), a.to_string());
    }
    if let Some(r) = replaced_by {
        meta.insert("replaced-by".to_string(), r.to_string());
    }

    let fm = SkillFrontmatter::builder()
        .name(name)
        .description("Test skill description.")
        .metadata(if meta.is_empty() { None } else { Some(meta) })
        .build();

    let raw = format!("---\nname: {name}\ndescription: Test skill description.\n---\n{content}");
    let promoted = if tree == SkillTree::Archive {
        false
    } else {
        category.is_promoted()
    };

    Skill::builder()
        .path(PathBuf::from(path))
        .dir_name(dir_name.to_string())
        .category(category)
        .promoted(promoted)
        .tree(tree)
        .group(group.map(ToString::to_string))
        .archived(archived.map(ToString::to_string))
        .replaced_by(replaced_by.map(ToString::to_string))
        .frontmatter(fm)
        .content(content.to_string())
        .raw(raw)
        .resources(Vec::new())
        .build()
}

#[test]
fn test_archive_missing_metadata_archived() {
    let skill = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        None,
        None,
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-metadata")
        .expect("archived-metadata error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue
        .message
        .contains("Archived skill must declare 'metadata.archived'"));
}

#[test]
fn test_archive_invalid_date_format() {
    let skill = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026/09/04"),
        None,
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-metadata")
        .expect("archived-metadata error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue.message.contains("Invalid archived date '2026/09/04'"));
}

#[test]
fn test_archive_valid_date() {
    let skill = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        None,
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_live_skill_with_archived_metadata() {
    let skill = build_test_skill(
        "skills/engineering/active-tool/SKILL.md",
        "active-tool",
        SkillCategory::Engineering,
        "active-tool",
        SkillTree::Live,
        None,
        Some("2026-09-04"),
        None,
        "# Active Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-metadata")
        .expect("archived-metadata error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue
        .message
        .contains("Live skill must not declare 'metadata.archived'"));
}

#[test]
fn test_archived_replaced_by_valid() {
    let live = build_test_skill(
        "skills/engineering/new-tool/SKILL.md",
        "new-tool",
        SkillCategory::Engineering,
        "new-tool",
        SkillTree::Live,
        None,
        None,
        None,
        "# New Tool",
    );
    let archive = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        Some("new-tool"),
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[live, archive]);
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_archived_replaced_by_unknown() {
    let archive = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        Some("does-not-exist"),
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[archive]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-replaced-by")
        .expect("archived-replaced-by error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue.message.contains("unknown skill 'does-not-exist'"));
}

#[test]
fn test_archived_replaced_by_archived() {
    let target_archive = build_test_skill(
        "skills-archive/engineering/target-tool/SKILL.md",
        "target-tool",
        SkillCategory::Engineering,
        "target-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-01"),
        None,
        "# Target Tool",
    );
    let archive = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        Some("target-tool"),
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[target_archive, archive]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-replaced-by")
        .expect("archived-replaced-by error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue
        .message
        .contains("replaces with archived skill 'target-tool'"));
}

#[test]
fn test_archived_replaced_by_unpromoted() {
    let unpromoted = build_test_skill(
        "skills/in-progress/draft-tool/SKILL.md",
        "draft-tool",
        SkillCategory::InProgress,
        "draft-tool",
        SkillTree::Live,
        None,
        None,
        None,
        "# Draft Tool",
    );
    let archive = build_test_skill(
        "skills-archive/engineering/old-tool/SKILL.md",
        "old-tool",
        SkillCategory::Engineering,
        "old-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        Some("draft-tool"),
        "# Old Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[unpromoted, archive]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "archived-replaced-by")
        .expect("archived-replaced-by error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue
        .message
        .contains("replaces with unpromoted skill 'draft-tool'"));
}

#[test]
fn test_duplicate_name_across_live_and_archive() {
    let live = build_test_skill(
        "skills/engineering/my-tool/SKILL.md",
        "my-tool",
        SkillCategory::Engineering,
        "my-tool",
        SkillTree::Live,
        None,
        None,
        None,
        "# Live Tool",
    );
    let archive = build_test_skill(
        "skills-archive/engineering/my-tool/SKILL.md",
        "my-tool",
        SkillCategory::Engineering,
        "my-tool",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        None,
        "# Archive Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[live, archive]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "duplicate-skill-names")
        .expect("duplicate-skill-names error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue.message.contains("across trees (live and archive)"));
}

#[test]
fn test_cross_ref_valid_same_group() {
    let s1 = build_test_skill(
        "skills/authoring/skill-a/SKILL.md",
        "skill-a",
        SkillCategory::Authoring,
        "skill-a",
        SkillTree::Live,
        Some("authoring"),
        None,
        None,
        "# Skill A\n\nCalls /skill-b and [Skill B](skills/authoring/skill-b/SKILL.md).",
    );
    let s2 = build_test_skill(
        "skills/authoring/skill-b/SKILL.md",
        "skill-b",
        SkillCategory::Authoring,
        "skill-b",
        SkillTree::Live,
        Some("authoring"),
        None,
        None,
        "# Skill B",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[s1, s2]);
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_cross_ref_forbidden_different_group() {
    let s1 = build_test_skill(
        "skills/github/git-tool/SKILL.md",
        "git-tool",
        SkillCategory::Github,
        "git-tool",
        SkillTree::Live,
        Some("github"),
        None,
        None,
        "# Git Tool\n\nRuns /code-review for review passes.",
    );
    let s2 = build_test_skill(
        "skills/review/code-review/SKILL.md",
        "code-review",
        SkillCategory::Review,
        "code-review",
        SkillTree::Live,
        Some("review"),
        None,
        None,
        "# Code Review",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[s1, s2]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "cross-reference-group")
        .expect("cross-reference-group error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue.message.contains("Forbidden cross-reference"));
    assert!(issue.message.contains("group: github"));
    assert!(issue.message.contains("group: review"));
}

#[test]
fn test_cross_ref_forbidden_no_group() {
    let s1 = build_test_skill(
        "skills/engineering/solo-a/SKILL.md",
        "solo-a",
        SkillCategory::Engineering,
        "solo-a",
        SkillTree::Live,
        None,
        None,
        None,
        "# Solo A\n\nUses /solo-b here.",
    );
    let s2 = build_test_skill(
        "skills/engineering/solo-b/SKILL.md",
        "solo-b",
        SkillCategory::Engineering,
        "solo-b",
        SkillTree::Live,
        None,
        None,
        None,
        "# Solo B",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[s1, s2]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "cross-reference-group")
        .expect("cross-reference-group error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue.message.contains("group: none"));
}

#[test]
fn test_cross_ref_forbidden_to_archived() {
    let s1 = build_test_skill(
        "skills/engineering/modern/SKILL.md",
        "modern",
        SkillCategory::Engineering,
        "modern",
        SkillTree::Live,
        Some("authoring"),
        None,
        None,
        "# Modern\n\nSee /old-archived-tool for history.",
    );
    let s2 = build_test_skill(
        "skills-archive/engineering/old-archived-tool/SKILL.md",
        "old-archived-tool",
        SkillCategory::Engineering,
        "old-archived-tool",
        SkillTree::Archive,
        Some("authoring"),
        Some("2026-09-04"),
        None,
        "# Old Archived Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[s1, s2]);

    assert!(report.has_errors());
    let issue = report
        .issues
        .iter()
        .find(|i| i.rule == "cross-reference-group")
        .expect("cross-reference-group error expected");
    assert_eq!(issue.severity, LintSeverity::Error);
    assert!(issue
        .message
        .contains("Forbidden cross-reference to archived skill 'old-archived-tool'"));
}

#[test]
fn test_skill_router_exempt_as_source() {
    let router = build_test_skill(
        "skills/authoring/skill-router/SKILL.md",
        "skill-router",
        SkillCategory::Authoring,
        "skill-router",
        SkillTree::Live,
        Some("authoring"),
        None,
        None,
        "# Skill Router\n\nRoutes to /git-tool and /code-review universally.",
    );
    let s1 = build_test_skill(
        "skills/github/git-tool/SKILL.md",
        "git-tool",
        SkillCategory::Github,
        "git-tool",
        SkillTree::Live,
        Some("github"),
        None,
        None,
        "# Git Tool",
    );
    let s2 = build_test_skill(
        "skills/review/code-review/SKILL.md",
        "code-review",
        SkillCategory::Review,
        "code-review",
        SkillTree::Live,
        Some("review"),
        None,
        None,
        "# Code Review",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[router, s1, s2]);
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_archived_skill_exempt_as_source() {
    let archived = build_test_skill(
        "skills-archive/engineering/retired/SKILL.md",
        "retired",
        SkillCategory::Engineering,
        "retired",
        SkillTree::Archive,
        None,
        Some("2026-09-04"),
        None,
        "# Retired\n\nHistorically called /git-tool and /code-review.",
    );
    let s1 = build_test_skill(
        "skills/github/git-tool/SKILL.md",
        "git-tool",
        SkillCategory::Github,
        "git-tool",
        SkillTree::Live,
        Some("github"),
        None,
        None,
        "# Git Tool",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_all(&[archived, s1]);
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_parser_bare_archive_dir_fails() {
    let dir = tempdir().unwrap();
    let bare_skill_dir = dir.path().join("skills-archive/my-skill");
    fs::create_dir_all(&bare_skill_dir).unwrap();
    let file_path = bare_skill_dir.join("SKILL.md");

    let mut f = File::create(&file_path).unwrap();
    writeln!(
        f,
        "---\nname: my-skill\ndescription: Bare archive.\n---\n# Content"
    )
    .unwrap();

    let result = SkillParser::parse_file(&file_path);

    assert!(result.is_err());
    match result.unwrap_err() {
        SkillError::FrontmatterValidation {
            path: _,
            message: msg,
        } => {
            assert!(msg.contains("Bare skills-archive/<name>/ is forbidden"));
        }
        other => panic!("Expected FrontmatterValidation error, got: {other:?}"),
    }
}

#[test]
fn test_parser_discovers_both_trees() {
    let dir = tempdir().unwrap();
    let live_dir = dir.path().join("skills/engineering/active-one");
    let archive_dir = dir.path().join("skills-archive/planning/archived-one");
    fs::create_dir_all(&live_dir).unwrap();
    fs::create_dir_all(&archive_dir).unwrap();

    let live_file = live_dir.join("SKILL.md");
    let mut f1 = File::create(&live_file).unwrap();
    writeln!(
        f1,
        "---\nname: active-one\ndescription: Live skill.\n---\n# Active"
    )
    .unwrap();

    let archive_file = archive_dir.join("SKILL.md");
    let mut f2 = File::create(&archive_file).unwrap();
    writeln!(
        f2,
        "---\nname: archived-one\ndescription: Archived skill.\nmetadata:\n  archived: '2026-09-04'\n---\n# Archived"
    )
    .unwrap();

    let skills = SkillParser::discover_skills(dir.path()).unwrap();
    assert_eq!(skills.len(), 2);

    let active = skills.iter().find(|s| s.name() == "active-one").unwrap();
    assert_eq!(active.tree, SkillTree::Live);
    assert!(active.promoted);

    let retired = skills.iter().find(|s| s.name() == "archived-one").unwrap();
    assert_eq!(retired.tree, SkillTree::Archive);
    assert!(!retired.promoted);
    assert_eq!(retired.archived_date(), Some("2026-09-04"));
}
