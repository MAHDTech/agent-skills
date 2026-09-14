//! Main entry point for the `ask` CLI binary.

pub mod cli;
pub mod commands;

use clap::{CommandFactory, Parser};
use std::io::IsTerminal;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;

use crate::cli::{Cli, Commands, TuiArgs};
use skills_core::error::SkillError;

/// Configures the tracing subscriber to output exclusively to standard error.
///
/// Log levels are controlled by verbosity count and quiet flag:
/// - quiet = true: `LevelFilter::OFF`
/// - quiet = false, verbose = 0: `LevelFilter::INFO` (or `RUST_LOG` environment variable if set)
/// - quiet = false, verbose = 1: `LevelFilter::DEBUG`
/// - quiet = false, verbose >= 2: `LevelFilter::TRACE`
pub fn init_logging(verbose: u8, quiet: bool) -> Result<(), Box<dyn std::error::Error>> {
    let filter = if quiet {
        LevelFilter::OFF
    } else {
        match verbose {
            0 => {
                if let Ok(rust_log) = std::env::var("RUST_LOG") {
                    rust_log.parse::<LevelFilter>().unwrap_or(LevelFilter::INFO)
                } else {
                    LevelFilter::INFO
                }
            }
            1 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        }
    };

    fmt()
        .with_writer(std::io::stderr)
        .with_max_level(filter)
        .try_init()
        .map_err(|e| -> Box<dyn std::error::Error> { e })?;

    Ok(())
}

/// Maps domain errors to standardized process exit codes:
/// - Code 0: Success, clean termination, help, or version display.
/// - Code 1: Runtime execution errors (I/O, network, lock contention).
/// - Code 2: Lint violations, schema validation failures, or command-line syntax parsing errors.
#[must_use]
pub fn error_to_exit_code(err: &SkillError) -> i32 {
    match err {
        SkillError::Lint { .. }
        | SkillError::FrontmatterValidation { .. }
        | SkillError::Yaml { .. } => 2,
        _ => 1,
    }
}

/// Translates general and structured CLI errors into standardized process exit codes.
#[must_use]
pub fn cli_error_to_exit_code(err: &(dyn std::error::Error + 'static)) -> i32 {
    if let Some(cli_err) = err.downcast_ref::<commands::CliError>() {
        match cli_err {
            commands::CliError::Subprocess { code, .. } => code.unwrap_or(1),
            commands::CliError::Skill(s) => error_to_exit_code(s),
            commands::CliError::Io(_) => 1,
        }
    } else if let Some(skill_err) = err.downcast_ref::<SkillError>() {
        error_to_exit_code(skill_err)
    } else {
        1
    }
}

/// Main command dispatcher evaluating options and routing execution.
pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        None => {
            if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
                commands::tui::run(TuiArgs {
                    tick_rate: Some(250),
                    start_view: None,
                })
                .await
                .map_err(Into::into)
            } else {
                let mut cmd = Cli::command();
                cmd.print_help()?;
                println!();
                Ok(())
            }
        }
        Some(Commands::Tui(tui_args)) => commands::tui::run(tui_args).await.map_err(Into::into),
        Some(Commands::Skills(skills_args)) => commands::skills::run(skills_args, cli.format)
            .await
            .map_err(Into::into),
        Some(Commands::Dashboard(dashboard_args)) => {
            commands::dashboard::run(dashboard_args, cli.format)
                .await
                .map_err(Into::into)
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = init_logging(cli.verbose, cli.quiet) {
        eprintln!("Failed to initialize logging subsystem: {err}");
        std::process::exit(1);
    }

    if let Err(err) = run(cli).await {
        eprintln!("Error: {err}");
        std::process::exit(cli_error_to_exit_code(&*err));
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_to_exit_code() {
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
        let yaml_err = serde_yaml::from_str::<serde_yaml::Value>(":").unwrap_err();
        assert_eq!(
            error_to_exit_code(&SkillError::Yaml {
                path: PathBuf::from("test"),
                source: yaml_err,
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
    }
    #[test]
    fn test_cli_error_to_exit_code() {
        let sub_err = commands::CliError::Subprocess {
            command: "test".into(),
            code: Some(42),
        };
        assert_eq!(cli_error_to_exit_code(&sub_err), 42);

        let sub_err_none = commands::CliError::Subprocess {
            command: "test".into(),
            code: None,
        };
        assert_eq!(cli_error_to_exit_code(&sub_err_none), 1);

        let lint_err = commands::CliError::Skill(SkillError::Lint {
            count: 1,
            details: "lint".into(),
        });
        assert_eq!(cli_error_to_exit_code(&lint_err), 2);

        let io_err = commands::CliError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "denied",
        ));
        assert_eq!(cli_error_to_exit_code(&io_err), 1);
    }
}
