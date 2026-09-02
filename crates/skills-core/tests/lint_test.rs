//! Integration and unit tests for `skills-core` static analysis and linting engine.

use skills_core::lint::SkillLinter;
use skills_core::models::{LintSeverity, Skill, SkillCategory, SkillFrontmatter};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

#[allow(clippy::needless_pass_by_value)]
fn build_test_skill(
    path: &str,
    dir_name: &str,
    category: SkillCategory,
    name: &str,
    description: &str,
    content: &str,
) -> Skill {
    let fm = SkillFrontmatter::builder()
        .name(name)
        .description(description)
        .build();

    let raw = format!("---\nname: {name}\ndescription: {description}\n---\n{content}");

    Skill::builder()
        .path(PathBuf::from(path))
        .dir_name(dir_name.to_string())
        .category(category.clone())
        .promoted(category.is_promoted())
        .frontmatter(fm)
        .content(content.to_string())
        .raw(raw)
        .resources(Vec::new())
        .build()
}

#[test]
fn test_lint_clean_compliant_skill() {
    let skill = build_test_skill(
        "skills/engineering/domain-modeling/SKILL.md",
        "domain-modeling",
        SkillCategory::Engineering,
        "domain-modeling",
        "Build and sharpen domain models.",
        "# Domain Modeling\n\n```rust\nfn main() {}\n```\n\nClean content.",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);

    assert!(!report.has_errors());
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warning_count(), 0);
}

#[test]
fn test_lint_no_em_dashes_in_body_and_resources() {
    let skill = build_test_skill(
        "skills/engineering/bad-dashes/SKILL.md",
        "bad-dashes",
        SkillCategory::Engineering,
        "bad-dashes",
        "Testing em-dashes.",
        "# Header\n\nLine with an em\u{2014}dash here.",
    );

    let linter = SkillLinter::new();
    let report = linter.lint_skill(&skill);

    assert!(report.has_errors());
    let em_issue = report
        .issues
        .iter()
        .find(|i| i.rule == "no-em-dashes")
        .expect("no-em-dashes rule should be triggered");
    assert_eq!(em_issue.line, Some(7)); // Line 7 in raw content
    assert_eq!(em_issue.severity, LintSeverity::Error);
}

#[test]
fn test_lint_naming_invariants() {
    let linter = SkillLinter::new();

    // Directory mismatch
    let mismatch = build_test_skill(
        "skills/engineering/folder-a/SKILL.md",
        "folder-a",
        SkillCategory::Engineering,
        "folder-b",
        "Description here.",
        "# Content",
    );
    let rep = linter.lint_skill(&mismatch);
    assert!(rep.issues.iter().any(|i| i.rule == "naming-dir-mismatch"));

    // Non-kebab-case
    let invalid_kebab = build_test_skill(
        "skills/engineering/MySkill/SKILL.md",
        "MySkill",
        SkillCategory::Engineering,
        "MySkill",
        "Description here.",
        "# Content",
    );
    let rep = linter.lint_skill(&invalid_kebab);
    assert!(rep.issues.iter().any(|i| i.rule == "naming-kebab-case"));

    // Reserved brand keywords
    let reserved = build_test_skill(
        "skills/engineering/claude-skill/SKILL.md",
        "claude-skill",
        SkillCategory::Engineering,
        "claude-skill",
        "Description here.",
        "# Content",
    );
    let rep = linter.lint_skill(&reserved);
    assert!(rep.issues.iter().any(|i| i.rule == "naming-no-reserved"));
}

#[test]
fn test_lint_frontmatter_limits_and_missing() {
    let linter = SkillLinter::new();

    // Empty fields
    let empty_fields = build_test_skill(
        "skills/engineering/empty/SKILL.md",
        "empty",
        SkillCategory::Engineering,
        "",
        "",
        "# Content",
    );
    let rep = linter.lint_skill(&empty_fields);
    assert!(rep.issues.iter().any(|i| i.rule == "frontmatter-required"));

    // Exceeding length limits
    let long_name = "a".repeat(65);
    let long_desc = "b".repeat(1025);
    let long_fields = build_test_skill(
        "skills/engineering/long/SKILL.md",
        "long",
        SkillCategory::Engineering,
        &long_name,
        &long_desc,
        "# Content",
    );
    let rep = linter.lint_skill(&long_fields);
    assert_eq!(
        rep.issues
            .iter()
            .filter(|i| i.rule == "frontmatter-limits")
            .count(),
        2
    );
}

#[test]
fn test_lint_absolute_and_relative_links() {
    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/links-test");
    fs::create_dir_all(&skill_dir).unwrap();

    let target_doc = skill_dir.join("guide.md");
    File::create(&target_doc).unwrap();

    let file_proto = "file://";
    let content = format!(
        "# Links Test\nHere is an absolute link: [bad]({file_proto}/home/user/test.md).\nHere is a good relative link: [good](./guide.md).\nHere is a broken relative link: [broken](./nonexistent.md).\n\n```bash\n# Code block with file:// should be ignored\ncat {file_proto}/tmp/test\n```\n"
    );

    let mut skill = build_test_skill(
        "skills/engineering/links-test/SKILL.md",
        "links-test",
        SkillCategory::Engineering,
        "links-test",
        "Link validation test.",
        &content,
    );
    skill.path = skill_dir.join("SKILL.md");

    let linter = SkillLinter::builder()
        .base_path(Some(dir.path().to_path_buf()))
        .build();
    let report = linter.lint_skill(&skill);

    assert!(report
        .issues
        .iter()
        .any(|i| i.rule == "no-absolute-file-links"));
    assert!(report
        .issues
        .iter()
        .any(|i| i.rule == "broken-relative-links"));
}

#[test]
fn test_lint_code_blocks_missing_language() {
    let linter = SkillLinter::new();

    let content = "# Code Block\n\n```\nlet x = 1;\n```\n";
    let skill = build_test_skill(
        "skills/engineering/test/SKILL.md",
        "test",
        SkillCategory::Engineering,
        "test",
        "Test code block linting.",
        content,
    );

    let report = linter.lint_skill(&skill);
    let warning = report
        .issues
        .iter()
        .find(|i| i.rule == "fenced-code-blocks")
        .expect("fenced-code-blocks warning should be emitted");
    assert_eq!(warning.severity, LintSeverity::Warning);
}

#[test]
fn test_lint_resources_layout() {
    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/res-test");
    let resources_dir = skill_dir.join("resources");
    fs::create_dir_all(&resources_dir).unwrap();

    // Create loose file in resources/
    let loose_file = resources_dir.join("notes.txt");
    File::create(&loose_file).unwrap();

    // Create disallowed directory
    let bad_dir = resources_dir.join("docs");
    fs::create_dir_all(&bad_dir).unwrap();

    // Create index.md inside auto
    let auto_dir = resources_dir.join("auto");
    fs::create_dir_all(&auto_dir).unwrap();
    let index_file = auto_dir.join("index.md");
    File::create(&index_file).unwrap();

    let mut skill = build_test_skill(
        "skills/engineering/res-test/SKILL.md",
        "res-test",
        SkillCategory::Engineering,
        "res-test",
        "Resources layout test.",
        "# Content",
    );
    skill.path = skill_dir.join("SKILL.md");

    let linter = SkillLinter::builder()
        .base_path(Some(dir.path().to_path_buf()))
        .build();
    let report = linter.lint_skill(&skill);

    let res_issues = report
        .issues
        .iter()
        .filter(|i| i.rule == "resources-folder-structure")
        .count();
    assert_eq!(res_issues, 3);
}

#[test]
fn test_lint_duplicate_skill_names_across_categories() {
    let linter = SkillLinter::new();

    let skill1 = build_test_skill(
        "skills/engineering/dup-test/SKILL.md",
        "dup-test",
        SkillCategory::Engineering,
        "dup-test",
        "Promoted skill 1",
        "# 1",
    );

    let skill2 = build_test_skill(
        "skills/planning/dup-test/SKILL.md",
        "dup-test",
        SkillCategory::Planning,
        "dup-test",
        "Promoted skill 2",
        "# 2",
    );

    // Two promoted skills with same name -> Error
    let rep = linter.lint_all(&[skill1.clone(), skill2]);
    let dup_issue = rep
        .issues
        .iter()
        .find(|i| i.rule == "duplicate-skill-names")
        .expect("duplicate-skill-names issue should be present");
    assert_eq!(dup_issue.severity, LintSeverity::Error);

    // One promoted and one lifecycle -> Warning
    let lifecycle_skill = build_test_skill(
        "skills/in-progress/dup-test/SKILL.md",
        "dup-test",
        SkillCategory::InProgress,
        "dup-test",
        "Lifecycle skill",
        "# Draft",
    );

    let rep2 = linter.lint_all(&[skill1, lifecycle_skill]);
    let dup_warning = rep2
        .issues
        .iter()
        .find(|i| i.rule == "duplicate-skill-names")
        .expect("duplicate-skill-names warning should be present");
    assert_eq!(dup_warning.severity, LintSeverity::Warning);
}

#[test]
fn test_lint_file_and_repository_end_to_end() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let skill_dir = root.join("skills/engineering/my-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let mut f = File::create(&skill_file).unwrap();
    let doc = "---\nname: my-skill\ndescription: Clean skill for repository testing.\n---\n# My Skill\n\n```rust\nfn test() {}\n```\n\nInstructions.\n";
    f.write_all(doc.as_bytes()).unwrap();

    let linter = SkillLinter::new();
    let file_report = linter.lint_file(&skill_file).unwrap();
    assert!(!file_report.has_errors());

    let repo_report = linter.lint_repository(root).unwrap();
    assert!(!repo_report.has_errors());
    assert_eq!(repo_report.error_count(), 0);
}
