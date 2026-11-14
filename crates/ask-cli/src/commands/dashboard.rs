//! Command handlers for the `ask dashboard` subcommands.

#[path = "dashboard_site.rs"]
mod dashboard_site;

use std::fmt::Write as _;

use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Table};

use crate::cli::{DashboardArgs, DashboardCommands, OutputFormat};
use crate::commands::{CliError, resolve_root};
use skills_core::dashboard::{DashboardEngine, DashboardSummary};
use skills_core::error::SkillError;

/// Dispatches dashboard telemetry and static documentation operations.
pub async fn run(args: DashboardArgs, format: OutputFormat) -> Result<(), CliError> {
    if let Some(ref action) = args.action {
        return match action.as_str() {
            "summary" => run_summary(format).await,
            "build" => dashboard_site::run_build(None).await,
            "serve" => dashboard_site::run_serve(3000).await,
            "css" => dashboard_site::run_css().await,
            "lint" => dashboard_site::run_lint().await,
            "index" => dashboard_site::run_index().await,
            other => Err(CliError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown dashboard action: '{other}'"),
            ))),
        };
    }

    if let Some(ref cmd) = args.command {
        tracing::debug!(
            "Dispatching dashboard subcommand: {:?}, format: {:?}",
            cmd,
            format
        );

        match cmd {
            DashboardCommands::Summary => run_summary(format).await,
            DashboardCommands::Build { output } => {
                dashboard_site::run_build(output.as_deref()).await
            }
            DashboardCommands::Serve { port } => dashboard_site::run_serve(*port).await,
            DashboardCommands::Css => dashboard_site::run_css().await,
            DashboardCommands::Lint => dashboard_site::run_lint().await,
            DashboardCommands::Index => dashboard_site::run_index().await,
        }
    } else {
        run_summary(format).await
    }
}

#[allow(clippy::too_many_lines, clippy::unused_async)]
async fn run_summary(format: OutputFormat) -> Result<(), CliError> {
    let root = resolve_root()?;
    let summary = DashboardEngine::new().analyze_repository(&root)?;

    if format == OutputFormat::Json {
        let json = serde_json::to_string_pretty(&summary).map_err(|e| SkillError::Json {
            path: root.clone(),
            source: e,
        })?;
        println!("{json}");
    } else if format == OutputFormat::Yaml {
        let yaml = serde_yaml::to_string(&summary).map_err(|e| SkillError::yaml(&root, e))?;
        println!("{yaml}");
    } else if format == OutputFormat::Plain {
        print!("{}", format_summary_plain(&summary));
    } else {
        // Table 1: Catalog Overview
        let mut overview_table = Table::new();
        overview_table.load_style(UTF8_FULL.with_rounded_corners());
        overview_table.set_header(vec![
            Cell::new("Catalog Overview Metric").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);
        overview_table.add_row(vec![
            Cell::new("Total Skills"),
            Cell::new(summary.total_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Active Skills"),
            Cell::new(summary.active_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Inactive Skills"),
            Cell::new(summary.inactive_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Promoted Skills"),
            Cell::new(summary.promoted_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Lifecycle Skills"),
            Cell::new(summary.lifecycle_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("User Invoked Skills"),
            Cell::new(summary.user_invoked_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Model Invoked Skills"),
            Cell::new(summary.model_invoked_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Forked Skills"),
            Cell::new(summary.forked_skills).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Total Tokens"),
            Cell::new(summary.total_tokens).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Average Prompt Tokens"),
            Cell::new(summary.avg_prompt_tokens).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Min Prompt Tokens"),
            Cell::new(summary.min_prompt_tokens).set_alignment(CellAlignment::Right),
        ]);
        overview_table.add_row(vec![
            Cell::new("Max Prompt Tokens"),
            Cell::new(summary.max_prompt_tokens).set_alignment(CellAlignment::Right),
        ]);
        println!("{overview_table}");
        println!();

        // Table 2: Health and Diagnostic Quality
        let mut health_table = Table::new();
        health_table.load_style(UTF8_FULL.with_rounded_corners());
        health_table.set_header(vec![
            Cell::new("Diagnostic Quality Metric").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);
        health_table.add_row(vec![
            Cell::new("Overall Health Score"),
            Cell::new(format!("{:.1}%", summary.health.score)).set_alignment(CellAlignment::Right),
        ]);
        health_table.add_row(vec![
            Cell::new("Clean Skills"),
            Cell::new(format!(
                "{} ({:.1}%)",
                summary.health.clean_skills, summary.health.clean_percentage
            ))
            .set_alignment(CellAlignment::Right),
        ]);
        health_table.add_row(vec![
            Cell::new("Static Analysis Errors"),
            Cell::new(summary.health.total_errors).set_alignment(CellAlignment::Right),
        ]);
        health_table.add_row(vec![
            Cell::new("Static Analysis Warnings"),
            Cell::new(summary.health.total_warnings).set_alignment(CellAlignment::Right),
        ]);
        println!("{health_table}");
        println!();

        // Table 3: Category Distribution
        let mut category_table = Table::new();
        category_table.load_style(UTF8_FULL.with_rounded_corners());
        category_table.set_header(vec![
            Cell::new("Category").add_attribute(Attribute::Bold),
            Cell::new("Skills").add_attribute(Attribute::Bold),
            Cell::new("Catalog %").add_attribute(Attribute::Bold),
            Cell::new("Total Tokens").add_attribute(Attribute::Bold),
            Cell::new("Promoted").add_attribute(Attribute::Bold),
            Cell::new("Lifecycle").add_attribute(Attribute::Bold),
        ]);
        for cat in &summary.categories {
            category_table.add_row(vec![
                Cell::new(cat.category.title()),
                Cell::new(cat.count).set_alignment(CellAlignment::Right),
                Cell::new(format!("{:.1}%", cat.percentage)).set_alignment(CellAlignment::Right),
                Cell::new(cat.total_tokens).set_alignment(CellAlignment::Right),
                Cell::new(cat.promoted_count).set_alignment(CellAlignment::Right),
                Cell::new(cat.lifecycle_count).set_alignment(CellAlignment::Right),
            ]);
        }
        println!("{category_table}");
        println!();

        // Table 4: Agent Target Environments
        let mut target_table = Table::new();
        target_table.load_style(UTF8_FULL.with_rounded_corners());
        target_table.set_header(vec![
            Cell::new("Target Environment").add_attribute(Attribute::Bold),
            Cell::new("Installed").add_attribute(Attribute::Bold),
            Cell::new("Active").add_attribute(Attribute::Bold),
            Cell::new("Adoption %").add_attribute(Attribute::Bold),
        ]);
        for target in &summary.targets {
            target_table.add_row(vec![
                Cell::new(target.environment.display_name()),
                Cell::new(target.installed_count).set_alignment(CellAlignment::Right),
                Cell::new(target.active_count).set_alignment(CellAlignment::Right),
                Cell::new(format!("{:.1}%", target.percentage)).set_alignment(CellAlignment::Right),
            ]);
        }
        println!("{target_table}");
    }

    Ok(())
}

fn format_summary_plain(summary: &DashboardSummary) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "Total Skills: {}", summary.total_skills);
    let _ = writeln!(out, "Active Skills: {}", summary.active_skills);
    let _ = writeln!(out, "Inactive Skills: {}", summary.inactive_skills);
    let _ = writeln!(out, "Promoted Skills: {}", summary.promoted_skills);
    let _ = writeln!(out, "Lifecycle Skills: {}", summary.lifecycle_skills);
    let _ = writeln!(out, "User Invoked Skills: {}", summary.user_invoked_skills);
    let _ = writeln!(
        out,
        "Model Invoked Skills: {}",
        summary.model_invoked_skills
    );
    let _ = writeln!(out, "Forked Skills: {}", summary.forked_skills);
    let _ = writeln!(out, "Total Tokens: {}", summary.total_tokens);
    let _ = writeln!(out, "Average Prompt Tokens: {}", summary.avg_prompt_tokens);
    let _ = writeln!(out, "Min Prompt Tokens: {}", summary.min_prompt_tokens);
    let _ = writeln!(out, "Max Prompt Tokens: {}", summary.max_prompt_tokens);

    out.push('\n');
    let _ = writeln!(out, "Overall Health Score: {:.1}%", summary.health.score);
    let _ = writeln!(
        out,
        "Clean Skills: {} ({:.1}%)",
        summary.health.clean_skills, summary.health.clean_percentage
    );
    let _ = writeln!(
        out,
        "Static Analysis Errors: {}",
        summary.health.total_errors
    );
    let _ = writeln!(
        out,
        "Static Analysis Warnings: {}",
        summary.health.total_warnings
    );

    out.push('\n');
    for cat in &summary.categories {
        let _ = writeln!(
            out,
            "{}\t{}\t{:.1}%\t{}\t{}\t{}",
            cat.category.title(),
            cat.count,
            cat.percentage,
            cat.total_tokens,
            cat.promoted_count,
            cat.lifecycle_count
        );
    }

    out.push('\n');
    for target in &summary.targets {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{:.1}%",
            target.environment.display_name(),
            target.installed_count,
            target.active_count,
            target.percentage
        );
    }

    out
}
