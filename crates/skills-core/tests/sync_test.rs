// cspell:words Syncer syncer
use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use skills_core::{
    ConflictStrategy, InstallMode, InstallOptions, Installer, SkillSyncer, SyncActionKind,
    SyncError, TargetEnvironment,
};

/// Helper to generate a mock skill directory with `SKILL.md` and sample resource files.
fn create_mock_skill(root: &Path, id: &str, version: &str, body: &str) -> PathBuf {
    let skill_dir = root.join(id);
    fs::create_dir_all(skill_dir.join("resources/manual")).unwrap();
    fs::create_dir_all(skill_dir.join("resources/auto")).unwrap();

    let skill_md = format!(
        "---\nname: {id}\ndescription: Test skill {id}\nmetadata:\n  version: \"{version}\"\n---\n# {id}\n{body}\n"
    );
    fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
    fs::write(
        skill_dir.join("resources/manual/test.txt"),
        "test manual content\n",
    )
    .unwrap();
    skill_dir
}

/// Helper to generate a mock catalog with a list of (id, version, body) tuples.
#[allow(dead_code)]
fn create_mock_catalog(root: &Path, skills: &[(&str, &str, &str)]) -> PathBuf {
    let catalog_dir = root.join("catalog");
    for (id, version, body) in skills {
        create_mock_skill(&catalog_dir, id, version, body);
    }
    catalog_dir
}

/// Test harness providing isolated temporary directories for catalog and agent targets.
struct TestHarness {
    temp: TempDir,
    catalog_dir: PathBuf,
    agent_dir: PathBuf,
    target: TargetEnvironment,
}

impl TestHarness {
    fn new() -> Self {
        let temp = TempDir::new().unwrap();
        let catalog_dir = temp.path().join("catalog");
        let agent_dir = temp.path().join("agent_skills");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::create_dir_all(&agent_dir).unwrap();
        let target = TargetEnvironment::Custom(agent_dir.clone());
        Self {
            temp,
            catalog_dir,
            agent_dir,
            target,
        }
    }
}

#[test]
fn test_plan_fresh_install() {
    let h = TestHarness::new();
    create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body A");
    create_mock_skill(&h.catalog_dir, "skill-b", "1.0.0", "Body B");

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert_eq!(plan.installs().count(), 2);
    assert_eq!(plan.updates().count(), 0);
    assert_eq!(plan.deletes().count(), 0);
    assert!(!plan.is_noop());
    assert!(!plan.has_conflicts());
}

#[test]
fn test_plan_version_upgrade() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body A");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Bump catalog version to 1.1.0
    create_mock_skill(&h.catalog_dir, "skill-a", "1.1.0", "Body A Updated");

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert_eq!(plan.updates().count(), 1);
    let update = plan.updates().next().unwrap();
    assert_eq!(update.skill_id, "skill-a");
    assert_eq!(update.source_version.as_deref(), Some("1.1.0"));
    assert_eq!(update.target_version.as_deref(), Some("1.0.0"));
    assert!(update.reason.contains("1.1.0 > 1.0.0"));
}

#[test]
fn test_plan_content_drift_same_version() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Original Body");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Modify file content while keeping version 1.0.0
    fs::write(
        skill_dir.join("resources/manual/test.txt"),
        "modified content\n",
    )
    .unwrap();

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert_eq!(plan.updates().count(), 1);
    let update = plan.updates().next().unwrap();
    assert_eq!(update.source_version.as_deref(), Some("1.0.0"));
    assert_eq!(update.target_version.as_deref(), Some("1.0.0"));
    assert_ne!(update.source_checksum, update.target_checksum);
    assert!(update.reason.contains("Content drift detected"));
}

#[test]
fn test_plan_noop_when_identical() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body A");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert!(plan.is_noop());
    assert_eq!(plan.no_ops().count(), 1);
    assert_eq!(plan.actions[0].kind, SyncActionKind::NoOp);
}

#[test]
fn test_plan_downgrade_conflict() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-a", "1.2.0", "Body v1.2");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Downgrade catalog to 1.0.0
    create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body v1.0");

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert!(plan.has_conflicts());
    assert_eq!(plan.conflicts().count(), 1);
    let conflict = plan.conflicts().next().unwrap();
    assert!(conflict.reason.contains("downgrade hazard"));
}

#[test]
fn test_plan_target_tampering_conflict() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body A");

    let opts = InstallOptions {
        mode: InstallMode::Copy,
        ..Default::default()
    };
    let installer = Installer::new();
    installer.install(&skill_dir, &h.target, &opts).unwrap();

    // Tamper with target file on disk out-of-band
    let target_skill_md = h.agent_dir.join("skill-a/SKILL.md");
    fs::write(target_skill_md, "tampered content").unwrap();

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();

    assert!(plan.has_conflicts());
    let conflict = plan.conflicts().next().unwrap();
    assert!(conflict.reason.contains("integrity verification failed"));
}

#[test]
fn test_orphan_prune_disabled_produces_noop() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-orphan", "1.0.0", "Body Orphan");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Delete skill from catalog
    fs::remove_dir_all(&skill_dir).unwrap();

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_prune_orphans(false);
    let plan = syncer.create_plan().unwrap();

    assert_eq!(plan.deletes().count(), 0);
    assert_eq!(plan.no_ops().count(), 1);
    assert!(plan.actions[0].reason.contains("prune_orphans = false"));
}

#[test]
fn test_orphan_prune_enabled_produces_delete() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-orphan", "1.0.0", "Body Orphan");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Delete skill from catalog
    fs::remove_dir_all(&skill_dir).unwrap();

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_prune_orphans(true);
    let plan = syncer.create_plan().unwrap();

    assert_eq!(plan.deletes().count(), 1);
    assert_eq!(plan.actions[0].kind, SyncActionKind::Delete);
    assert_eq!(plan.actions[0].skill_id, "skill-orphan");
}

#[test]
fn test_dry_run_guarantee_zero_disk_mutations() {
    let h = TestHarness::new();
    create_mock_skill(&h.catalog_dir, "skill-dry", "1.0.0", "Dry Body");

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_dry_run(true);

    let summary = syncer.sync().unwrap();

    assert!(summary.dry_run);
    assert_eq!(summary.installed, 1);
    assert_eq!(summary.affected_skills, vec!["skill-dry"]);

    // Target directory must remain completely empty
    let entries = fs::read_dir(&h.agent_dir).unwrap().count();
    assert_eq!(entries, 0);

    // Registry file must not exist
    let reg_path = h.agent_dir.join("installed-skills.json");
    assert!(!reg_path.exists());
}

#[test]
fn test_execute_plan_installs_and_updates() {
    let h = TestHarness::new();
    let skill_a = create_mock_skill(&h.catalog_dir, "skill-a", "1.0.0", "Body A");
    create_mock_skill(&h.catalog_dir, "skill-b", "1.0.0", "Body B");

    // Pre-install skill-a at v1.0.0
    let installer = Installer::new();
    installer
        .install(&skill_a, &h.target, &InstallOptions::default())
        .unwrap();

    // Bump skill-a to v1.1.0 in catalog
    create_mock_skill(&h.catalog_dir, "skill-a", "1.1.0", "Body A Updated");

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let plan = syncer.create_plan().unwrap();
    let summary = syncer.execute_plan(&plan).unwrap();

    assert!(!summary.dry_run);
    assert_eq!(summary.installed, 1);
    assert_eq!(summary.updated, 1);

    let reg = installer.read_registry(&h.target).unwrap();
    assert_eq!(reg.skills.len(), 2);
    assert_eq!(reg.get("skill-a").unwrap().version, "1.1.0");
    assert_eq!(reg.get("skill-b").unwrap().version, "1.0.0");
}

#[test]
fn test_conflict_resolution_local_wins() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-conflict", "1.5.0", "Target v1.5");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Catalog has older version (downgrade conflict)
    create_mock_skill(&h.catalog_dir, "skill-conflict", "1.0.0", "Catalog v1.0");

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_conflict_strategy(ConflictStrategy::LocalWins);

    let summary = syncer.sync().unwrap();

    assert_eq!(summary.conflicts_resolved, 1);
    assert_eq!(summary.updated, 1);

    let reg = installer.read_registry(&h.target).unwrap();
    assert_eq!(reg.get("skill-conflict").unwrap().version, "1.0.0");

    // Pre-swap backup must exist
    let backups_dir = h.agent_dir.join(".backups");
    assert!(backups_dir.exists());
}

#[test]
fn test_conflict_resolution_remote_wins() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-conflict", "1.5.0", "Target v1.5");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    // Catalog has older version (downgrade conflict)
    create_mock_skill(&h.catalog_dir, "skill-conflict", "1.0.0", "Catalog v1.0");

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_conflict_strategy(ConflictStrategy::RemoteWins);

    let summary = syncer.sync().unwrap();

    assert_eq!(summary.conflicts_resolved, 1);
    assert_eq!(summary.updated, 0);
    assert_eq!(summary.no_ops, 1);

    let reg = installer.read_registry(&h.target).unwrap();
    assert_eq!(reg.get("skill-conflict").unwrap().version, "1.5.0");
}

#[test]
fn test_conflict_resolution_prompt_user_aborts_headless() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-conflict", "1.5.0", "Target v1.5");

    let installer = Installer::new();
    installer
        .install(&skill_dir, &h.target, &InstallOptions::default())
        .unwrap();

    create_mock_skill(&h.catalog_dir, "skill-conflict", "1.0.0", "Catalog v1.0");

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_conflict_strategy(ConflictStrategy::PromptUser);

    let result = syncer.sync();

    assert!(matches!(
        result,
        Err(SyncError::UnresolvedConflict {
            skill_id,
            ..
        }) if skill_id == "skill-conflict"
    ));
}

#[test]
fn test_multi_target_synchronization() {
    let h = TestHarness::new();
    create_mock_skill(&h.catalog_dir, "skill-multi", "1.0.0", "Multi Body");

    let agent_dir_2 = h.temp.path().join("agent_skills_2");
    fs::create_dir_all(&agent_dir_2).unwrap();
    let target_2 = TargetEnvironment::Custom(agent_dir_2);

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_target(target_2.clone());

    let summary = syncer.sync().unwrap();

    assert_eq!(summary.installed, 2);

    let installer = Installer::new();
    let reg1 = installer.read_registry(&h.target).unwrap();
    let reg2 = installer.read_registry(&target_2).unwrap();

    assert!(reg1.get("skill-multi").is_some());
    assert!(reg2.get("skill-multi").is_some());
}

#[test]
fn test_symlink_mode_synchronization() {
    let h = TestHarness::new();
    let skill_dir = create_mock_skill(&h.catalog_dir, "skill-sym", "1.0.0", "Sym Body");

    let syncer = SkillSyncer::new(&h.catalog_dir)
        .with_target(h.target.clone())
        .with_mode(InstallMode::Symlink);

    let summary = syncer.sync().unwrap();
    assert_eq!(summary.installed, 1);

    let target_skill_dir = h.agent_dir.join("skill-sym");
    assert!(target_skill_dir.symlink_metadata().unwrap().is_symlink());

    // Modify source file; plan should detect drift to refresh registry checksum
    fs::write(
        skill_dir.join("resources/manual/test.txt"),
        "new symlink content\n",
    )
    .unwrap();
    let plan = syncer.create_plan().unwrap();
    assert_eq!(plan.updates().count(), 1);
}

#[test]
fn test_convenience_sync_workflow() {
    let h = TestHarness::new();
    create_mock_skill(
        &h.catalog_dir,
        "skill-convenience",
        "1.0.0",
        "Convenience Body",
    );

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let summary = syncer.sync().unwrap();

    assert_eq!(summary.installed, 1);
    assert_eq!(summary.no_ops, 0);

    // Second sync invocation immediately produces NoOp
    let summary_second = syncer.sync().unwrap();
    assert_eq!(summary_second.installed, 0);
    assert_eq!(summary_second.no_ops, 1);
}

#[test]
fn test_catalog_discovery_nested_categories() {
    let h = TestHarness::new();
    let eng_dir = h.catalog_dir.join("engineering");
    let rev_dir = h.catalog_dir.join("review");
    fs::create_dir_all(&eng_dir).unwrap();
    fs::create_dir_all(&rev_dir).unwrap();

    create_mock_skill(&eng_dir, "code-analyzer", "1.0.0", "Analyze code");
    create_mock_skill(&rev_dir, "pr-reviewer", "1.0.0", "Review PRs");

    let syncer = SkillSyncer::new(&h.catalog_dir).with_target(h.target.clone());
    let skills = syncer.discover_catalog_skills().unwrap();

    assert_eq!(skills.len(), 2);
    assert!(skills.contains_key("code-analyzer"));
    assert!(skills.contains_key("pr-reviewer"));

    let plan = syncer.create_plan().unwrap();
    assert_eq!(plan.installs().count(), 2);
}
