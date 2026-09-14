//! End-to-end integration tests for the `ask` CLI binary.

use assert_cmd::Command;
use predicates::prelude::*;
use skills_core::error::SkillError;
use std::path::PathBuf;

fn error_to_exit_code(err: &SkillError) -> i32 {
    match err {
        SkillError::Lint {
            count: _,
            details: _,
        }
        | SkillError::FrontmatterValidation {
            path: _,
            message: _,
        } => 2,
        _ => 1,
    }
}

#[test]
fn test_cli_help_flag() {
    for flag in ["--help", "-h"] {
        Command::cargo_bin("ask")
            .unwrap()
            .arg(flag)
            .assert()
            .success()
            .code(0)
            .stdout(predicate::str::contains(
                "Agent Skills CLI and catalog manager",
            ))
            .stdout(predicate::str::contains("Usage:"))
            .stdout(predicate::str::contains("Commands:"))
            .stdout(predicate::str::contains("Options:"))
            .stdout(predicate::str::contains("--verbose"))
            .stdout(predicate::str::contains("--quiet"))
            .stdout(predicate::str::contains("--format"))
            .stdout(predicate::str::contains("--config"))
            .stderr(predicate::str::is_empty());
    }
}

#[test]
fn test_cli_version_flag() {
    for flag in ["--version", "-V"] {
        Command::cargo_bin("ask")
            .unwrap()
            .arg(flag)
            .assert()
            .success()
            .code(0)
            .stdout(predicate::str::contains("ask"))
            .stdout(predicate::str::contains("0.1.0"))
            .stderr(predicate::str::is_empty());
    }
}

#[test]
fn test_cli_subcommands_help() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("install"))
        .stdout(predicate::str::contains("uninstall"))
        .stdout(predicate::str::contains("lint"))
        .stdout(predicate::str::contains("sync"))
        .stdout(predicate::str::contains("download-resources"))
        .stdout(predicate::str::contains("clean-resources"));

    Command::cargo_bin("ask")
        .unwrap()
        .args(["dashboard", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("summary"))
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("serve"))
        .stdout(predicate::str::contains("css"))
        .stdout(predicate::str::contains("lint"));

    Command::cargo_bin("ask")
        .unwrap()
        .args(["tui", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("--tick-rate"))
        .stdout(predicate::str::contains("--start-view"));
}

#[test]
fn test_cli_invalid_subcommand_and_format() {
    Command::cargo_bin("ask")
        .unwrap()
        .arg("non-existent-command")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "unrecognized subcommand 'non-existent-command'",
        ));

    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "list", "--format", "unsupported-format"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'unsupported-format'",
        ))
        .stderr(predicate::str::contains(
            "[possible values: plain, json, yaml, table]",
        ));
}

#[test]
fn test_cli_global_options_parsing() {
    Command::cargo_bin("ask")
        .unwrap()
        .args([
            "-v",
            "-q",
            "-f",
            "json",
            "-c",
            "/tmp/config.toml",
            "skills",
            "list",
        ])
        .assert()
        .success()
        .code(0);

    Command::cargo_bin("ask")
        .unwrap()
        .args([
            "skills",
            "list",
            "-v",
            "-q",
            "-f",
            "yaml",
            "-c",
            "/tmp/config.toml",
        ])
        .assert()
        .success()
        .code(0);

    Command::cargo_bin("ask")
        .unwrap()
        .args([
            "--verbose",
            "--quiet",
            "--format",
            "table",
            "--config",
            "/tmp/config.toml",
            "dashboard",
            "summary",
        ])
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_cli_logging_stderr_routing() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["-v", "skills", "list"])
        .assert()
        .success()
        .code(0)
        .stderr(predicate::str::contains("Dispatching skills subcommand"))
        .stdout(predicate::str::contains("Dispatching skills subcommand").not());

    Command::cargo_bin("ask")
        .unwrap()
        .args(["-q", "skills", "list"])
        .assert()
        .success()
        .code(0)
        .stderr(predicate::str::contains("Dispatching skills subcommand").not());

    Command::cargo_bin("ask")
        .unwrap()
        .args(["-v", "-q", "skills", "list"])
        .assert()
        .success()
        .code(0)
        .stderr(predicate::str::contains("Dispatching skills subcommand").not());
}

#[test]
fn test_cli_non_interactive_no_args() {
    Command::cargo_bin("ask")
        .unwrap()
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(
            "Agent Skills CLI and catalog manager",
        ))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_exit_code_mapping() {
    assert_eq!(
        error_to_exit_code(&SkillError::Lint {
            count: 1,
            details: "rule violation".into(),
        }),
        2
    );
    assert_eq!(
        error_to_exit_code(&SkillError::FrontmatterValidation {
            path: PathBuf::from("test"),
            message: "invalid".into(),
        }),
        2
    );
    assert_eq!(
        error_to_exit_code(&SkillError::GeneralIo(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "missing",
        ))),
        1
    );
    assert_eq!(
        error_to_exit_code(&SkillError::LockTimeout {
            path: PathBuf::from("test"),
            timeout_secs: 5,
        }),
        1
    );
    assert_eq!(
        error_to_exit_code(&SkillError::Network {
            url: "https://example.com".into(),
            message: "timeout".into(),
        }),
        1
    );

    Command::cargo_bin("ask")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .code(0);

    Command::cargo_bin("ask")
        .unwrap()
        .arg("invalid-subcommand-xyz")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn test_cli_action_flag() {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    Command::cargo_bin("ask")
        .unwrap()
        .current_dir(&repo_root)
        .args(["skills", "--action", "lint"])
        .assert()
        .success()
        .code(0);

    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("--action"));

    Command::cargo_bin("ask")
        .unwrap()
        .args(["dashboard", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("--action"));
}

#[test]
fn test_skills_list_table_output() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "list"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("ID / Directory"))
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Category"))
        .stdout(predicate::str::contains("Description"));
}

#[test]
fn test_skills_list_json_format() {
    let assert = Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "list", "--format", "json"])
        .assert()
        .success()
        .code(0);

    let stdout_bytes = assert.get_output().stdout.clone();
    let json_val: serde_json::Value = serde_json::from_slice(&stdout_bytes)
        .expect("skills list --format json should return valid JSON");
    assert!(json_val.is_array());
}

#[test]
fn test_skills_list_yaml_format() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "list", "--format", "yaml"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("dir_name:"));
}

#[test]
fn test_skills_list_category_filter() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "list", "--category", "engineering"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("engineering"));
}

#[test]
fn test_skills_show_valid_skill() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "show", "domain-modeling"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Identifier:  domain-modeling"))
        .stdout(predicate::str::contains("Category:    engineering"));
}

#[test]
fn test_skills_show_not_found() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "show", "non-existent-skill-xyz"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Skill not found for query 'non-existent-skill-xyz'",
        ));
}

#[test]
fn test_skills_lint_workspace() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "lint"])
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_skills_sync_dry_run() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "sync", "--dry-run"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Synchronization Plan"))
        .stdout(predicate::str::contains("dry_run: true"));
}

#[test]
fn test_dashboard_summary_table() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["dashboard", "summary"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Catalog Overview Metric"))
        .stdout(predicate::str::contains("Diagnostic Quality Metric"))
        .stdout(predicate::str::contains("Category"))
        .stdout(predicate::str::contains("Target Environment"));
}

#[test]
fn test_dashboard_default_summary() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["dashboard"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Catalog Overview Metric"))
        .stdout(predicate::str::contains("Diagnostic Quality Metric"));
}

#[test]
fn test_dashboard_summary_json() {
    let assert = Command::cargo_bin("ask")
        .unwrap()
        .args(["dashboard", "summary", "--format", "json"])
        .assert()
        .success()
        .code(0);

    let stdout_bytes = assert.get_output().stdout.clone();
    let json_val: serde_json::Value = serde_json::from_slice(&stdout_bytes)
        .expect("dashboard summary --format json should return valid JSON");
    assert!(json_val.get("total_skills").is_some());
    assert!(json_val.get("health").is_some());
}

#[test]
fn test_tui_help_flags() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["tui", "--help"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("--tick-rate"))
        .stdout(predicate::str::contains("--start-view"));
}

#[test]
fn test_skills_lint_invalid_skill_exit_code() {
    let temp = tempfile::tempdir().unwrap();
    let skill_dir = temp.path().join("invalid-skill");
    std::fs::create_dir_all(&skill_dir).unwrap();
    std::fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: Missing name field\n---\n# Content\n",
    )
    .unwrap();

    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "lint", skill_dir.to_str().unwrap()])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn test_skills_lint_fix() {
    let temp = tempfile::tempdir().unwrap();
    let skill_dir = temp.path().join("fixable-skill");
    std::fs::create_dir_all(&skill_dir).unwrap();
    let content_with_em_dash =
        "---\nname: fixable-skill\ndescription: A valid description\n---\n# Content \u{2014} with em dash\n";
    let skill_file = skill_dir.join("SKILL.md");
    std::fs::write(&skill_file, content_with_em_dash).unwrap();

    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "lint", "--fix", skill_dir.to_str().unwrap()])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(
            "Fixed 1 issue(s) across 1 file(s).",
        ));

    let fixed_content = std::fs::read_to_string(&skill_file).unwrap();
    assert!(!fixed_content.contains('\u{2014}'));
    assert!(fixed_content.contains('-'));
}

#[test]
fn test_skills_install_missing_argument_exit_code_2() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "install"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("<SOURCE>"));
}

#[test]
fn test_skills_uninstall_missing_argument_exit_code_2() {
    Command::cargo_bin("ask")
        .unwrap()
        .args(["skills", "uninstall"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("<SKILL>"));
}

#[test]
fn test_skills_download_caching() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            use std::io::{Read, Write};
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 18\r\n\r\n# Downloaded Skill";
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });

    let temp = tempfile::tempdir().unwrap();
    let url = format!("http://127.0.0.1:{port}/sample-skill.md");

    Command::cargo_bin("ask")
        .unwrap()
        .env("AGENT_SKILLS_HOME", temp.path())
        .args(["skills", "download", &url])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Cached remote skill to"))
        .stdout(predicate::str::contains("# Downloaded Skill"));

    let slug = skills_core::downloader::SkillDownloader::smart_slugify(&url, None);
    let cached_file = temp.path().join(".cache").join("skills").join(slug);
    assert!(cached_file.exists());
    let cached_content = std::fs::read_to_string(cached_file).unwrap();
    assert_eq!(cached_content, "# Downloaded Skill");
}
