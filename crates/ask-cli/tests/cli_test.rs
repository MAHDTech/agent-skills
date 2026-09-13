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
