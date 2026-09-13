//! Command-line interface definitions and argument parsing models.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Supported output formatting styles for structured and unstructured command results.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Standard human-readable plain text output.
    #[default]
    #[value(name = "plain", help = "Human-readable plain text")]
    Plain,

    /// Structured JSON data format.
    #[value(name = "json", help = "Structured JSON")]
    Json,

    /// Structured YAML data format.
    #[value(name = "yaml", help = "Structured YAML")]
    Yaml,

    /// Formatted ASCII or Unicode bordered table.
    #[value(name = "table", help = "Formatted bordered table")]
    Table,
}

impl OutputFormat {
    /// Returns the static string representation of the output format.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Table => "table",
        }
    }
}

/// Unified command-line interface for the Agent Skills ecosystem.
#[derive(Parser, Debug, Clone)]
#[command(
    name = "ask",
    author,
    version,
    about = "Agent Skills CLI and catalog manager",
    propagate_version = true
)]
pub struct Cli {
    /// Increase logging verbosity (-v: DEBUG, -vv: TRACE).
    #[arg(
        short = 'v',
        long = "verbose",
        action = clap::ArgAction::Count,
        global = true,
        help = "Increase logging verbosity (-v: DEBUG, -vv: TRACE)"
    )]
    pub verbose: u8,

    /// Silence all diagnostic and logging output.
    #[arg(
        short = 'q',
        long = "quiet",
        global = true,
        help = "Silence all diagnostic and logging output"
    )]
    pub quiet: bool,

    /// Output formatting style.
    #[arg(
        short = 'f',
        long = "format",
        value_enum,
        default_value_t = OutputFormat::Plain,
        global = true,
        help = "Output formatting style"
    )]
    pub format: OutputFormat,

    /// Custom configuration file path.
    #[arg(
        short = 'c',
        long = "config",
        value_name = "FILE",
        global = true,
        help = "Custom configuration file path"
    )]
    pub config: Option<PathBuf>,

    /// Optional subcommand to execute; defaults to interactive TUI when omitted on a TTY.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Top-level subcommands available within the `ask` CLI.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// Manage, inspect, and validate the skills catalog.
    #[command(about = "Manage, inspect, and validate the skills catalog")]
    Skills(SkillsArgs),

    /// Static documentation and catalog telemetry dashboard.
    #[command(about = "Static documentation and catalog telemetry dashboard")]
    Dashboard(DashboardArgs),

    /// Launch interactive terminal user interface.
    #[command(about = "Launch interactive terminal user interface")]
    Tui(TuiArgs),
}

/// Arguments and subcommands for skill catalog operations.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct SkillsArgs {
    /// Specific skills catalog operation to perform.
    #[command(subcommand)]
    pub command: SkillsCommands,
}

/// Catalog operations supported under the `skills` subcommand.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum SkillsCommands {
    /// List discovered skills in the catalog.
    #[command(about = "List discovered skills in the catalog")]
    List {
        /// Filter skills by category.
        #[arg(short = 'C', long, help = "Filter skills by category")]
        category: Option<String>,

        /// Output catalog as raw JSON.
        #[arg(short, long, help = "Output catalog as raw JSON")]
        json: bool,
    },

    /// Display details of a specific skill.
    #[command(about = "Display details of a specific skill")]
    Show {
        /// Unique identifier or directory name of the skill.
        #[arg(help = "Unique identifier or directory name of the skill")]
        skill: String,
    },

    /// Install skill symlinks into agent configuration directories.
    #[command(about = "Install skill symlinks into agent configuration directories")]
    Install {
        /// Target execution environment (e.g. gemini, claude).
        #[arg(short, long, help = "Target execution environment")]
        target: Option<String>,
    },

    /// Remove skill symlinks from agent configuration directories.
    #[command(about = "Remove skill symlinks from agent configuration directories")]
    Uninstall {
        /// Target execution environment to remove symlinks from.
        #[arg(
            short,
            long,
            help = "Target execution environment to remove symlinks from"
        )]
        target: Option<String>,
    },

    /// Validate skill frontmatter schemas and markdown formatting rules.
    #[command(about = "Validate skill frontmatter schemas and markdown formatting rules")]
    Lint {
        /// Automatically apply safe fixes for detected lint violations.
        #[arg(
            long,
            help = "Automatically apply safe fixes for detected lint violations"
        )]
        fix: bool,
    },

    /// Reconcile catalog index and target environment symlinks.
    #[command(about = "Reconcile catalog index and target environment symlinks")]
    Sync {
        /// Simulate synchronization actions without writing changes to disk.
        #[arg(
            long,
            help = "Simulate synchronization actions without writing changes to disk"
        )]
        dry_run: bool,
    },

    /// Download external resource assets for skills requiring remote files.
    #[command(about = "Download external resource assets for skills requiring remote files")]
    DownloadResources {
        /// Force re-download of existing cached resources.
        #[arg(long, help = "Force re-download of existing cached resources")]
        force: bool,
    },

    /// Remove downloaded skill resources and clear cache.
    #[command(about = "Remove downloaded skill resources and clear cache")]
    CleanResources,
}

/// Arguments and subcommands for static documentation and dashboard operations.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct DashboardArgs {
    /// Specific dashboard operation to perform.
    #[command(subcommand)]
    pub command: DashboardCommands,
}

/// Operations supported under the `dashboard` subcommand.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum DashboardCommands {
    /// Print catalog summary metrics and static analysis health scores.
    #[command(about = "Print catalog summary metrics and static analysis health scores")]
    Summary,

    /// Generate static HTML documentation and asset bundles.
    #[command(about = "Generate static HTML documentation and asset bundles")]
    Build {
        /// Output directory for compiled static assets.
        #[arg(short, long, help = "Output directory for compiled static assets")]
        output: Option<PathBuf>,
    },

    /// Run local preview web server hosting the generated dashboard.
    #[command(about = "Run local preview web server hosting the generated dashboard")]
    Serve {
        /// Port to bind local preview web server.
        #[arg(
            short = 'p',
            long,
            default_value_t = 3000,
            help = "Port to bind local preview web server"
        )]
        port: u16,
    },

    /// Recompile static style sheet assets via Tailwind CSS.
    #[command(about = "Recompile static style sheet assets via Tailwind CSS")]
    Css,

    /// Verify generated documentation markup and anchor link integrity.
    #[command(about = "Verify generated documentation markup and anchor link integrity")]
    Lint,
}

/// Arguments for launching the interactive terminal user interface.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct TuiArgs {
    /// Event polling tick rate in milliseconds.
    #[arg(
        short = 't',
        long = "tick-rate",
        help = "Event polling tick rate in milliseconds (default: 250)"
    )]
    pub tick_rate: Option<u64>,

    /// Initial active view tab (explorer, inspector, linter, runner).
    #[arg(
        short = 's',
        long = "start-view",
        help = "Initial active view tab (explorer, inspector, linter, runner)"
    )]
    pub start_view: Option<String>,
}
