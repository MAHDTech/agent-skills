//! Main entry point for the `ask` CLI binary.

pub mod cli;

use clap::{CommandFactory, Parser};
use std::io::IsTerminal;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;

use crate::cli::{Cli, Commands, DashboardArgs, OutputFormat, SkillsArgs, TuiArgs};
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

/// Main command dispatcher evaluating options and routing execution.
pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        None => {
            if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
                run_tui(TuiArgs {
                    tick_rate: Some(250),
                    start_view: None,
                })
                .await
            } else {
                let mut cmd = Cli::command();
                cmd.print_help()?;
                println!();
                Ok(())
            }
        }
        Some(Commands::Tui(tui_args)) => run_tui(tui_args).await,
        Some(Commands::Skills(skills_args)) => run_skills(skills_args, cli.format).await,
        Some(Commands::Dashboard(dashboard_args)) => {
            run_dashboard(dashboard_args, cli.format).await
        }
    }
}

/// Launches the interactive terminal user interface.
#[allow(clippy::unused_async)]
async fn run_tui(args: TuiArgs) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("Launching TUI with args: {:?}", args);
    Ok(())
}

/// Dispatches catalog operations to skills-core.
#[allow(clippy::unused_async)]
async fn run_skills(
    args: SkillsArgs,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!(
        "Dispatching skills subcommand: {:?}, format: {:?}",
        args.command,
        format
    );
    Ok(())
}

/// Dispatches static documentation operations.
#[allow(clippy::unused_async)]
async fn run_dashboard(
    args: DashboardArgs,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!(
        "Dispatching dashboard subcommand: {:?}, format: {:?}",
        args.command,
        format
    );
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = init_logging(cli.verbose, cli.quiet) {
        eprintln!("Failed to initialize logging subsystem: {err}");
        std::process::exit(1);
    }

    if let Err(err) = run(cli).await {
        if let Some(skill_err) = err.downcast_ref::<SkillError>() {
            eprintln!("Error: {skill_err}");
            std::process::exit(error_to_exit_code(skill_err));
        }
        eprintln!("Error: {err}");
        std::process::exit(1);
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
}
