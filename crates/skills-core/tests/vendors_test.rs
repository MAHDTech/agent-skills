// cspell:words Syncer
use std::fs;
use std::path::Path;
use std::process::Command;

use skills_core::vendors::update_vendors;
use tempfile::tempdir;

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "commit.gpgsign=false",
        ])
        .args([
            "-c",
            "user.name=Vendor Tests",
            "-c",
            "user.email=tests@example.com",
        ])
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn skill(root: &Path, name: &str, body: &str) {
    let path = root.join("skills").join(name);
    fs::create_dir_all(&path).unwrap();
    fs::write(
        path.join("SKILL.md"),
        format!("---\nname: {name}\ndescription: Vendor test skill\n---\n\n{body}\n"),
    )
    .unwrap();
}

fn repository(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-b", "main"]);
    fs::write(root.join(".gitignore"), ".cache/\n").unwrap();
}

fn commit(root: &Path) {
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "fixture"]);
}

fn manifest(root: &Path, sources: &[(&str, &Path)]) {
    let mut yaml = "version: 1\nsources:\n".to_string();
    for (id, path) in sources {
        yaml.push_str(&format!(
            "  - id: {id}\n    repository: '{}'\n    ref: main\n    path: skills\n",
            path.display()
        ));
    }
    fs::write(root.join("vendor-skills.yaml"), yaml).unwrap();
}

#[test]
fn vendor_refresh_preserves_support_files_and_mirrors_changes() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    skill(&upstream, "alpha", "# Upstream alpha");
    let support = upstream.join("skills/alpha/references");
    fs::create_dir_all(&support).unwrap();
    fs::write(support.join("guide.md"), "Original support text").unwrap();
    fs::write(upstream.join("LICENSE"), "Vendor license text").unwrap();
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);

    update_vendors(&root, None).unwrap();
    let alpha = root.join("skills/vendor/example/alpha");
    assert_eq!(
        fs::read(alpha.join("SKILL.md")).unwrap(),
        fs::read(upstream.join("skills/alpha/SKILL.md")).unwrap()
    );
    assert_eq!(
        fs::read_to_string(alpha.join("references/guide.md")).unwrap(),
        "Original support text"
    );
    assert!(alpha.join("LICENSE").is_file());
    fs::write(alpha.join("SKILL.md"), "local edits").unwrap();
    update_vendors(&root, None).unwrap();
    assert_eq!(
        fs::read(alpha.join("SKILL.md")).unwrap(),
        fs::read(upstream.join("skills/alpha/SKILL.md")).unwrap()
    );

    fs::remove_dir_all(upstream.join("skills/alpha")).unwrap();
    skill(&upstream, "beta", "# Upstream beta");
    commit(&upstream);
    update_vendors(&root, None).unwrap();
    assert!(!alpha.exists());
    assert!(root.join("skills/vendor/example/beta/SKILL.md").is_file());
}

#[test]
fn vendor_exclusions_resolve_collisions_and_can_be_cleared() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    skill(&upstream, "alpha", "# Vendor alpha");
    skill(&upstream, "beta", "# Vendor beta");
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    update_vendors(&root, None).unwrap();
    let declaration = fs::read_to_string(root.join("vendor-skills.yaml")).unwrap();
    skill(&root, "alpha", "# Custom alpha");
    fs::write(
        root.join("vendor-skills.yaml"),
        format!("{declaration}    excludes:\n      - alpha\n"),
    )
    .unwrap();

    let report = update_vendors(&root, None).unwrap();
    assert!(report.fetched.contains_key("example"));
    assert!(!root.join("skills/vendor/example/alpha").exists());
    assert!(root.join("skills/vendor/example/beta/SKILL.md").is_file());
    assert!(root.join("skills/alpha/SKILL.md").is_file());

    fs::remove_dir_all(root.join("skills/alpha")).unwrap();
    fs::write(
        root.join("vendor-skills.yaml"),
        format!("{declaration}    excludes: []\n"),
    )
    .unwrap();
    update_vendors(&root, None).unwrap();
    assert!(root.join("skills/vendor/example/alpha/SKILL.md").is_file());
    assert!(root.join("skills/vendor/example/beta/SKILL.md").is_file());
}

#[test]
fn vendor_exclusions_can_omit_multiple_or_all_skills() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    for name in ["alpha", "beta", "gamma"] {
        skill(&upstream, name, "# Vendor skill");
    }
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    update_vendors(&root, None).unwrap();
    let declaration = fs::read_to_string(root.join("vendor-skills.yaml")).unwrap();
    fs::write(
        root.join("vendor-skills.yaml"),
        format!("{declaration}    excludes: [alpha, beta, unknown]\n"),
    )
    .unwrap();
    let report = update_vendors(&root, None).unwrap();
    assert!(report.fetched.contains_key("example"));
    assert!(!root.join("skills/vendor/example/alpha").exists());
    assert!(!root.join("skills/vendor/example/beta").exists());
    assert!(root.join("skills/vendor/example/gamma/SKILL.md").is_file());

    fs::write(
        root.join("vendor-skills.yaml"),
        format!("{declaration}    excludes: [alpha, beta, gamma]\n"),
    )
    .unwrap();
    let report = update_vendors(&root, None).unwrap();
    assert!(report.fetched.contains_key("example"));
    assert!(!root.join("skills/vendor/example/gamma").exists());
}

#[test]
fn vendor_collisions_do_not_replace_the_existing_catalog() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    skill(&upstream, "alpha", "# Original");
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    update_vendors(&root, None).unwrap();
    let previous = fs::read(root.join("skills/vendor/example/alpha/SKILL.md")).unwrap();
    skill(&root, "collision", "# Custom skill");
    skill(&upstream, "collision", "# Vendor collision");
    skill(&upstream, "alpha", "# Updated");
    commit(&upstream);
    let error = update_vendors(&root, None).unwrap_err();
    assert!(error.to_string().contains("collision"));
    assert_eq!(
        previous,
        fs::read(root.join("skills/vendor/example/alpha/SKILL.md")).unwrap()
    );
    assert!(!root.join("skills/vendor/example/collision").exists());
}

#[test]
fn vendor_failure_retains_the_successful_snapshot_pending_in_a_pr() {
    let temp = tempdir().unwrap();
    let first = temp.path().join("first");
    let second = temp.path().join("second");
    let root = temp.path().join("catalog");
    for path in [&first, &second, &root] {
        repository(path);
    }
    skill(&first, "alpha", "# Base version");
    skill(&second, "beta", "# Base version");
    commit(&first);
    commit(&second);
    manifest(&root, &[("first", &first), ("second", &second)]);
    update_vendors(&root, None).unwrap();
    commit(&root);
    git(&root, &["checkout", "-b", "automation/vendor-skills"]);
    skill(&first, "alpha", "# Pending PR version");
    commit(&first);
    update_vendors(&root, None).unwrap();
    commit(&root);
    git(&root, &["checkout", "main"]);
    fs::rename(&first, temp.path().join("offline")).unwrap();
    skill(&second, "beta", "# Today's version");
    commit(&second);

    let report = update_vendors(&root, Some("automation/vendor-skills")).unwrap();
    assert!(report.skipped.contains_key("first"));
    assert!(
        fs::read_to_string(root.join("skills/vendor/first/alpha/SKILL.md"))
            .unwrap()
            .contains("Pending PR version")
    );
    assert!(
        fs::read_to_string(root.join("skills/vendor/second/beta/SKILL.md"))
            .unwrap()
            .contains("Today's version")
    );
}

#[test]
fn vendor_catalog_discovers_installs_and_renders_upstream_support_files() {
    use skills_core::artifacts::{ArtifactsEngine, ArtifactsOptions};
    use skills_core::downloader::SkillDownloader;
    use skills_core::lint::SkillLinter;
    use skills_core::parser::SkillParser;
    use skills_core::sync::SkillSyncer;

    let temp = tempdir().unwrap();
    let root = temp.path();
    let vendor = root.join("skills/vendor/example/alpha");
    fs::create_dir_all(vendor.join("references")).unwrap();
    fs::create_dir_all(vendor.join("resources/auto")).unwrap();
    fs::write(vendor.join("SKILL.md"), format!("---\nname: alpha\ndescription: Vendor\n---\n\n# Alpha\n\nVendor punctuation {} unchanged.\n\n[Guide](references/guide.md)\n", '\u{2014}')).unwrap();
    fs::write(vendor.join("references/guide.md"), "# Vendor guide\n").unwrap();
    fs::write(
        vendor.join("resources/auto/keep.md"),
        "Upstream-owned content",
    )
    .unwrap();
    let skills = SkillParser::discover_skills(root).unwrap();
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].vendor_source(), Some("example"));
    assert!(!SkillLinter::new().lint_all(&skills).has_errors());
    SkillDownloader::new().clean_all_resources(&skills).unwrap();
    assert!(vendor.join("resources/auto/keep.md").is_file());
    let discovered = SkillSyncer::new(root.join("skills"))
        .discover_catalog_skills()
        .unwrap();
    assert_eq!(discovered.get("alpha"), Some(&vendor));
    ArtifactsEngine::with_options(
        root,
        ArtifactsOptions {
            no_stage: true,
            ..Default::default()
        },
    )
    .generate_all()
    .unwrap();
    assert!(
        fs::read_to_string(root.join("README.md"))
            .unwrap()
            .contains("Vendor Skills")
    );
    assert!(
        fs::read_to_string(root.join("agents/AGENTS.md"))
            .unwrap()
            .contains("example")
    );
    assert!(
        fs::read_to_string(root.join("skills.sh.json"))
            .unwrap()
            .contains("Vendor Skills / example")
    );
    assert!(
        root.join("dashboard/content/skills/vendor/example/alpha/_index.md")
            .is_file()
    );
    assert!(
        root.join("dashboard/content/skills/vendor/example/alpha/references/guide.md")
            .is_file()
    );
}

#[test]
fn vendor_refresh_is_unchanged_when_only_unrelated_upstream_files_change() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    repository(&root);
    skill(&upstream, "alpha", "# Stable content");
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    update_vendors(&root, None).unwrap();
    commit(&root);
    fs::write(upstream.join("README.md"), "Unrelated change").unwrap();
    commit(&upstream);
    update_vendors(&root, None).unwrap();
    git(&root, &["diff", "--exit-code", "HEAD"]);
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(status.stdout.is_empty());
}

#[test]
fn vendor_download_failure_keeps_existing_files_and_advances_other_sources() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let unavailable = temp.path().join("unavailable");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(root.join("skills/vendor/old/alpha")).unwrap();
    fs::write(
        root.join("skills/vendor/old/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: Previous snapshot\n---\n\n# Keep\n",
    )
    .unwrap();
    skill(&upstream, "beta", "# New vendor");
    commit(&upstream);
    manifest(&root, &[("old", &unavailable), ("new", &upstream)]);
    let previous = fs::read(root.join("skills/vendor/old/alpha/SKILL.md")).unwrap();
    let report = update_vendors(&root, None).unwrap();
    assert!(report.skipped.contains_key("old"));
    assert_eq!(
        previous,
        fs::read(root.join("skills/vendor/old/alpha/SKILL.md")).unwrap()
    );
    assert!(root.join("skills/vendor/new/beta/SKILL.md").is_file());
}

#[test]
fn vendor_license_override_uses_the_skill_license_instead_of_the_repository_license() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    skill(&upstream, "alpha", "# Alpha");
    fs::write(upstream.join("LICENSE"), "Different root license").unwrap();
    fs::create_dir(upstream.join("licenses")).unwrap();
    fs::write(
        upstream.join("licenses/LICENSE"),
        "Applicable skill license",
    )
    .unwrap();
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    let declaration = fs::read_to_string(root.join("vendor-skills.yaml")).unwrap();
    fs::write(
        root.join("vendor-skills.yaml"),
        format!("{declaration}    license-files: [licenses/LICENSE]\n"),
    )
    .unwrap();
    update_vendors(&root, None).unwrap();
    assert_eq!(
        fs::read_to_string(root.join("skills/vendor/example/alpha/LICENSE")).unwrap(),
        "Applicable skill license"
    );
}

#[test]
fn vendor_manifest_rejects_paths_outside_the_managed_tree() {
    let temp = tempdir().unwrap();
    let outside = temp.path().join("outside");
    let root = temp.path().join("catalog");
    fs::create_dir_all(&root).unwrap();
    fs::write(&outside, "Keep this file").unwrap();
    fs::write(root.join("vendor-skills.yaml"), "version: 1\nsources:\n  - id: ../outside\n    repository: https://example.com/skills.git\n").unwrap();
    assert!(update_vendors(&root, None).is_err());
    assert_eq!(fs::read_to_string(&outside).unwrap(), "Keep this file");
}

#[test]
fn vendor_layout_changes_do_not_silently_drop_nested_skills() {
    let temp = tempdir().unwrap();
    let upstream = temp.path().join("upstream");
    let root = temp.path().join("catalog");
    repository(&upstream);
    fs::create_dir_all(&root).unwrap();
    skill(&upstream, "alpha", "# Previous version");
    commit(&upstream);
    manifest(&root, &[("example", &upstream)]);
    update_vendors(&root, None).unwrap();
    skill(&upstream, "alpha", "# New version");
    skill(
        &upstream.join("skills/category"),
        "nested",
        "# Nested skill",
    );
    commit(&upstream);
    let report = update_vendors(&root, None).unwrap();
    assert!(report.skipped.contains_key("example"));
    assert!(
        fs::read_to_string(root.join("skills/vendor/example/alpha/SKILL.md"))
            .unwrap()
            .contains("Previous version")
    );
}
