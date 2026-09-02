use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use tempfile::TempDir;

use skills_core::{
    EnvironmentResolver, InstallMode, InstallOptions, InstallResult, InstalledSkillsRegistry,
    Installer, InstallerError, IntegrityStatus, PathValidator, SkillInstaller, TargetEnvironment,
    UninstallOptions, UninstallResult, UpdateResult,
};

/// Helper to create a complete mock skill folder with `SKILL.md` and resources.
fn create_test_skill_fixture(
    root: &Path,
    id: &str,
    version: &str,
    extra_files: &[(&str, &str)],
) -> PathBuf {
    let skill_dir = root.join(id);
    fs::create_dir_all(skill_dir.join("resources/manual")).unwrap();
    fs::create_dir_all(skill_dir.join("resources/auto")).unwrap();

    let skill_md = format!(
        "---\nname: {id}\ndescription: Test skill {id}\nmetadata:\n  version: \"{version}\"\n---\n# {id}\nSkill body content.\n"
    );
    fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
    fs::write(
        skill_dir.join("resources/manual/guide.txt"),
        "Manual guide content\n",
    )
    .unwrap();
    fs::write(
        skill_dir.join("resources/auto/data.json"),
        "{\"status\":\"ok\"}\n",
    )
    .unwrap();

    for (rel_path, content) in extra_files {
        let full = skill_dir.join(rel_path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full, content).unwrap();
    }

    skill_dir
}

// -----------------------------------------------------------------------------
// 1. Environment Resolution Tests
// -----------------------------------------------------------------------------

#[test]
fn test_environment_resolution_custom_and_workspace() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let custom_env = TargetEnvironment::Custom(PathBuf::from("my-custom-skills"));
    let skills_dir =
        EnvironmentResolver::resolve_skills_dir_with_root(&custom_env, Some(root)).unwrap();
    assert_eq!(skills_dir, root.join("my-custom-skills"));

    let ws_root = root.join("my-workspace");
    let antigravity_env = TargetEnvironment::Antigravity {
        workspace_root: Some(ws_root.clone()),
    };
    let ag_dir =
        EnvironmentResolver::resolve_skills_dir_with_root(&antigravity_env, Some(root)).unwrap();
    assert_eq!(ag_dir, ws_root.join(".agents/skills"));

    let claude_env = TargetEnvironment::ClaudeDesktop;
    let claude_dir =
        EnvironmentResolver::resolve_skills_dir_with_root(&claude_env, Some(root)).unwrap();
    assert_eq!(claude_dir, root.join(".claude/skills"));

    let cursor_env = TargetEnvironment::Cursor;
    let cursor_dir =
        EnvironmentResolver::resolve_skills_dir_with_root(&cursor_env, Some(root)).unwrap();
    assert_eq!(cursor_dir, root.join(".cursor/skills"));
}

// -----------------------------------------------------------------------------
// 2. Copy Mode Installation Lifecycle
// -----------------------------------------------------------------------------

#[test]
fn test_install_copy_mode_full_lifecycle() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source =
        create_test_skill_fixture(root, "copy-skill", "1.0.0", &[("script.sh", "#!/bin/sh\n")]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let options = InstallOptions::builder()
        .mode(InstallMode::Copy)
        .force(false)
        .create_backup(true)
        .build();

    let res: InstallResult = installer.install(&source, &env, &options).unwrap();
    assert_eq!(res.skill.id, "copy-skill");
    assert_eq!(res.skill.version, "1.0.0");
    assert_eq!(res.installed_mode, InstallMode::Copy);
    assert!(!res.replaced_existing);
    assert!(res.backup_path.is_none());

    // Verify files copied to target
    let target_dir = root.join("installed/copy-skill");
    assert!(target_dir.join("SKILL.md").exists());
    assert!(target_dir.join("resources/manual/guide.txt").exists());
    assert!(target_dir.join("resources/auto/data.json").exists());
    assert!(target_dir.join("script.sh").exists());

    // Verify registry
    let registry = installer.read_registry(&env).unwrap();
    assert_eq!(registry.skills.len(), 1);
    let reg_skill = registry.get("copy-skill").unwrap();
    assert!(reg_skill.active);
    assert_eq!(reg_skill.version, "1.0.0");

    // Verify integrity
    let status = installer.verify("copy-skill", &env).unwrap();
    assert_eq!(status, IntegrityStatus::Valid);
}

// -----------------------------------------------------------------------------
// 3. Symlink Mode Installation Lifecycle
// -----------------------------------------------------------------------------

#[test]
fn test_install_symlink_mode_and_live_updates() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "sym-skill", "1.2.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let options = InstallOptions::builder()
        .mode(InstallMode::Symlink)
        .force(false)
        .build();

    let res = installer.install(&source, &env, &options).unwrap();
    assert_eq!(res.skill.id, "sym-skill");
    assert_eq!(res.installed_mode, InstallMode::Symlink);

    let target_dir = root.join("installed/sym-skill");
    assert!(target_dir
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());

    // Live update test: add a file in source, verify visible at destination
    fs::write(source.join("new_file.txt"), "live update content\n").unwrap();
    assert!(target_dir.join("new_file.txt").exists());
}

// -----------------------------------------------------------------------------
// 4. Auto Mode Installation
// -----------------------------------------------------------------------------

#[test]
fn test_install_auto_mode_selection() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "auto-skill", "0.5.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let options = InstallOptions::builder().mode(InstallMode::Auto).build();
    let res = installer.install(&source, &env, &options).unwrap();
    assert_eq!(res.skill.id, "auto-skill");
    assert!(res.skill.target_path.exists());
}

// -----------------------------------------------------------------------------
// 5. Conflict Without Force Fails
// -----------------------------------------------------------------------------

#[test]
fn test_install_conflict_without_force_fails() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "conflict-skill", "1.0.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let options = InstallOptions::builder().force(false).build();
    installer.install(&source, &env, &options).unwrap();

    // Second installation attempt should fail
    let err = installer.install(&source, &env, &options).unwrap_err();
    match err {
        InstallerError::SkillAlreadyInstalled { id, .. } => {
            assert_eq!(id, "conflict-skill");
        }
        other => panic!("Expected SkillAlreadyInstalled, got {other:?}"),
    }
}

// -----------------------------------------------------------------------------
// 6. Force Overwrite with Backup
// -----------------------------------------------------------------------------

#[test]
fn test_install_force_overwrite_with_backup() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source_v1 = create_test_skill_fixture(root, "backup-skill", "1.0.0", &[("v1.txt", "v1")]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let opts1 = InstallOptions::builder().mode(InstallMode::Copy).build();
    installer.install(&source_v1, &env, &opts1).unwrap();

    // Modify source for v2
    fs::write(
        source_v1.join("SKILL.md"),
        "---\nname: backup-skill\ndescription: Test\nmetadata:\n  version: \"2.0.0\"\n---\n# backup-skill\n",
    )
    .unwrap();
    fs::write(source_v1.join("v2.txt"), "v2").unwrap();

    let opts2 = InstallOptions::builder()
        .mode(InstallMode::Copy)
        .force(true)
        .create_backup(true)
        .build();

    let res2 = installer.install(&source_v1, &env, &opts2).unwrap();
    assert_eq!(res2.skill.version, "2.0.0");
    assert!(res2.replaced_existing);
    assert!(res2.backup_path.is_some());

    let backup_dir = res2.backup_path.unwrap();
    assert!(backup_dir.exists());
    assert!(backup_dir.join("v1.txt").exists());
}

// -----------------------------------------------------------------------------
// 7. Clean Uninstallation with Purge
// -----------------------------------------------------------------------------

#[test]
fn test_uninstall_clean_removal_and_purge() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "purge-skill", "1.0.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    installer
        .install(&source, &env, &InstallOptions::default())
        .unwrap();
    assert!(root.join("installed/purge-skill").exists());

    let un_opts = UninstallOptions::builder()
        .purge_state(true)
        .create_backup(false)
        .build();

    let un_res: UninstallResult = installer.uninstall("purge-skill", &env, &un_opts).unwrap();
    assert_eq!(un_res.skill_id, "purge-skill");
    assert!(!root.join("installed/purge-skill").exists());

    let registry = installer.read_registry(&env).unwrap();
    assert!(!registry.skills.contains_key("purge-skill"));
}

// -----------------------------------------------------------------------------
// 8. Soft Uninstallation (Active = False)
// -----------------------------------------------------------------------------

#[test]
fn test_uninstall_soft_deactivation() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "soft-skill", "1.0.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    installer
        .install(&source, &env, &InstallOptions::default())
        .unwrap();

    let un_opts = UninstallOptions::builder()
        .purge_state(false)
        .create_backup(false)
        .build();

    installer.uninstall("soft-skill", &env, &un_opts).unwrap();
    assert!(!root.join("installed/soft-skill").exists());

    let registry = installer.read_registry(&env).unwrap();
    let record = registry.get("soft-skill").unwrap();
    assert!(!record.active);
}

// -----------------------------------------------------------------------------
// 9. Skill Update Lifecycle
// -----------------------------------------------------------------------------

#[test]
fn test_update_skill_version_and_checksum_refresh() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "update-skill", "1.0.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    let inst_res = installer
        .install(&source, &env, &InstallOptions::default())
        .unwrap();
    let old_checksum = inst_res.skill.checksum;

    // Bump version and change content
    fs::write(
        source.join("SKILL.md"),
        "---\nname: update-skill\ndescription: Updated\nmetadata:\n  version: \"1.1.0\"\n---\n# update-skill\nNew content.\n",
    )
    .unwrap();

    let update_res: UpdateResult = installer
        .update(&source, &env, &InstallOptions::default())
        .unwrap();
    assert_eq!(update_res.previous_version, "1.0.0");
    assert_eq!(update_res.new_version, "1.1.0");
    assert_ne!(update_res.skill.checksum, old_checksum);

    let fetched = installer.get_skill("update-skill", &env).unwrap().unwrap();
    assert_eq!(fetched.version, "1.1.0");
}

// -----------------------------------------------------------------------------
// 10. Path Traversal & Invalid ID Validation
// -----------------------------------------------------------------------------

#[test]
fn test_path_traversal_validation_and_invalid_ids() {
    assert!(PathValidator::validate_skill_id("valid-id_123").is_ok());
    assert!(PathValidator::validate_skill_id("").is_err());
    assert!(PathValidator::validate_skill_id("../escaped").is_err());
    assert!(PathValidator::validate_skill_id("foo/bar").is_err());
    assert!(PathValidator::validate_skill_id("CON").is_err());
    assert!(PathValidator::validate_skill_id("AUX").is_err());
    assert!(PathValidator::validate_skill_id(&"a".repeat(70)).is_err());
}

// -----------------------------------------------------------------------------
// 11. Atomic Swap & Rollback
// -----------------------------------------------------------------------------

#[test]
fn test_atomic_swap_and_rollback_on_injection_error() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let skills_dir = root.join("skills");
    fs::create_dir_all(&skills_dir).unwrap();

    let staging = skills_dir.join(".staging/test_staging");
    fs::create_dir_all(&staging).unwrap();
    fs::write(staging.join("payload.txt"), "hello").unwrap();

    let mut coordinator = skills_core::installer::AtomicSwapCoordinator::new(
        &skills_dir,
        "test-skill",
        staging.clone(),
    );

    coordinator.execute_swap(true).unwrap();
    assert!(skills_dir.join("test-skill/payload.txt").exists());

    // Rollback should revert
    coordinator.rollback();
    assert!(!skills_dir.join("test-skill").exists());
}

// -----------------------------------------------------------------------------
// 12. Registry Migration v1 to v2
// -----------------------------------------------------------------------------

#[test]
fn test_registry_serialization_and_migration_v1_to_v2() {
    let temp = TempDir::new().unwrap();
    let reg_path = temp.path().join("installed-skills.json");

    let v1_json = r#"{
        "schema_version": 1,
        "updated_at": "2026-01-01T00:00:00Z",
        "skills": {
            "legacy-skill": {
                "name": "legacy-skill",
                "version": "0.9.0",
                "source_path": "/tmp/source",
                "target_path": "/tmp/target",
                "mode": "copy",
                "installed_at": "2026-01-01T00:00:00Z",
                "active": true
            }
        }
    }"#;
    fs::write(&reg_path, v1_json).unwrap();

    let loaded = InstalledSkillsRegistry::load(&reg_path).unwrap();
    assert_eq!(loaded.schema_version, 2);
    assert!(loaded.skills.contains_key("legacy-skill"));
    assert_eq!(loaded.skills["legacy-skill"].version, "0.9.0");

    loaded.save(&reg_path).unwrap();
    let reloaded = InstalledSkillsRegistry::load(&reg_path).unwrap();
    assert_eq!(reloaded.schema_version, 2);
}

// -----------------------------------------------------------------------------
// 13. Concurrency & Advisory Locking
// -----------------------------------------------------------------------------

#[test]
fn test_concurrent_file_locking_mutual_exclusion() {
    let temp = TempDir::new().unwrap();
    let lock_path = Arc::new(temp.path().join("test.lock"));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let lp = Arc::clone(&lock_path);
        let handle = thread::spawn(move || {
            let guard = skills_core::FileLockGuard::acquire(&lp, Duration::from_secs(5)).unwrap();
            thread::sleep(Duration::from_millis(20));
            guard.release().unwrap();
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

// -----------------------------------------------------------------------------
// 14. Stale Lock Recovery
// -----------------------------------------------------------------------------

#[test]
fn test_stale_lock_recovery_after_timeout() {
    let temp = TempDir::new().unwrap();
    let lock_path = temp.path().join("stale.lock");

    // Write a fake lock file with non-existent PID and past timestamp
    fs::write(&lock_path, "pid=99999999\ntimestamp=2020-01-01T00:00:00Z\n").unwrap();

    let guard = skills_core::FileLockGuard::acquire(&lock_path, Duration::from_millis(500));
    assert!(guard.is_ok());
}

// -----------------------------------------------------------------------------
// 15. Integrity Verification Detects Tampered Files
// -----------------------------------------------------------------------------

#[test]
fn test_integrity_verification_detects_tampered_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "tamper-skill", "1.0.0", &[("file1.txt", "abc")]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    installer
        .install(
            &source,
            &env,
            &InstallOptions::builder().mode(InstallMode::Copy).build(),
        )
        .unwrap();

    // Valid check
    assert_eq!(
        installer.verify("tamper-skill", &env).unwrap(),
        IntegrityStatus::Valid
    );

    // Tamper file content (ChecksumMismatch)
    let file1_path = root.join("installed/tamper-skill/file1.txt");
    fs::write(&file1_path, "modified content").unwrap();
    match installer.verify("tamper-skill", &env).unwrap() {
        IntegrityStatus::ChecksumMismatch { .. } => {}
        other => panic!("Expected ChecksumMismatch, got {other:?}"),
    }

    // Missing file
    fs::remove_file(&file1_path).unwrap();
    match installer.verify("tamper-skill", &env).unwrap() {
        IntegrityStatus::MissingFile { path } => {
            assert_eq!(path, "file1.txt");
        }
        other => panic!("Expected MissingFile, got {other:?}"),
    }

    // Restore file and add untracked file (ExtraFile)
    fs::write(&file1_path, "abc").unwrap();
    let extra_path = root.join("installed/tamper-skill/extra_untracked.txt");
    fs::write(&extra_path, "intruder").unwrap();
    match installer.verify("tamper-skill", &env).unwrap() {
        IntegrityStatus::ExtraFile { path } => {
            assert_eq!(path, "extra_untracked.txt");
        }
        other => panic!("Expected ExtraFile, got {other:?}"),
    }
}

// -----------------------------------------------------------------------------
// 16. Clean Orphans
// -----------------------------------------------------------------------------

#[test]
fn test_clean_orphans() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let source = create_test_skill_fixture(root, "valid-skill", "1.0.0", &[]);

    let installer = Installer::with_root(root);
    let env = TargetEnvironment::Custom(root.join("installed"));

    installer
        .install(&source, &env, &InstallOptions::default())
        .unwrap();

    // Create untracked orphan folder
    let orphan_dir = root.join("installed/orphan-folder");
    fs::create_dir_all(&orphan_dir).unwrap();
    fs::write(orphan_dir.join("junk.txt"), "junk").unwrap();

    let cleaned = installer.clean_orphans(&env).unwrap();
    assert_eq!(cleaned.len(), 1);
    assert_eq!(cleaned[0], orphan_dir);
    assert!(!orphan_dir.exists());
    assert!(root.join("installed/valid-skill").exists());
}

// -----------------------------------------------------------------------------
// 17. SkillInstaller Convenience API Tests
// -----------------------------------------------------------------------------

#[test]
fn test_skill_installer_convenience_wrapper() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let s1 = create_test_skill_fixture(root, "batch-1", "1.0.0", &[]);
    let s2 = create_test_skill_fixture(root, "batch-2", "2.0.0", &[]);

    let skill_installer =
        SkillInstaller::with_root(TargetEnvironment::Custom(root.join("target_skills")), root)
            .with_mode(InstallMode::Copy)
            .with_backup_dir(root.join("backups"))
            .with_overwrite(true);

    // Batch install
    let installed_list = skill_installer.install_many(&[s1, s2]).unwrap();
    assert_eq!(installed_list.len(), 2);

    // List and get
    let all = skill_installer.list_installed().unwrap();
    assert_eq!(all.len(), 2);
    assert!(skill_installer.get_installed("batch-1").unwrap().is_some());

    // Active toggle
    let updated = skill_installer.set_active("batch-1", false).unwrap();
    assert!(!updated.active);

    // Verification
    assert!(skill_installer.verify_installation("batch-2").unwrap());

    // Batch uninstall
    let uninstalled = skill_installer
        .uninstall_many(&["batch-1", "batch-2"], true)
        .unwrap();
    assert_eq!(uninstalled.len(), 2);
    assert_eq!(skill_installer.list_installed().unwrap().len(), 0);
}
