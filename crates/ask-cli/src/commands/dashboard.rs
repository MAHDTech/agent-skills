//! Command handlers for the `ask dashboard` subcommands.

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use std::path::Path;

use crate::cli::{DashboardArgs, DashboardCommands, OutputFormat};
use crate::commands::resolve_root;
use skills_core::dashboard::DashboardEngine;
use skills_core::error::SkillError;

/// Dispatches dashboard telemetry and static documentation operations.
pub async fn run(args: DashboardArgs, format: OutputFormat) -> Result<(), SkillError> {
    if let Some(ref action) = args.action {
        return run_action_script(action);
    }

    if let Some(ref cmd) = args.command {
        tracing::debug!(
            "Dispatching dashboard subcommand: {:?}, format: {:?}",
            cmd,
            format
        );

        match cmd {
            DashboardCommands::Summary => run_summary(format).await,
            DashboardCommands::Build { output } => run_build(output.as_deref()).await,
            DashboardCommands::Serve { port } => run_serve(*port).await,
            DashboardCommands::Css => run_css().await,
            DashboardCommands::Lint => run_lint().await,
        }
    } else {
        run_summary(format).await
    }
}

fn run_action_script(action: &str) -> Result<(), SkillError> {
    let status = std::process::Command::new("bun")
        .arg("run")
        .arg("bin/dashboard/index.ts")
        .arg("--action")
        .arg(action)
        .status()
        .map_err(SkillError::GeneralIo)?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_build(output: Option<&Path>) -> Result<(), SkillError> {
    let mut cmd = std::process::Command::new("bun");
    cmd.arg("run")
        .arg("bin/dashboard/index.ts")
        .arg("--action")
        .arg("build");
    if let Some(out) = output {
        cmd.arg("--output").arg(out);
    }
    let status = cmd.status().map_err(SkillError::GeneralIo)?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_serve(port: u16) -> Result<(), SkillError> {
    let status = std::process::Command::new("bun")
        .arg("run")
        .arg("bin/dashboard/index.ts")
        .arg("--action")
        .arg("serve")
        .arg("--port")
        .arg(port.to_string())
        .status()
        .map_err(SkillError::GeneralIo)?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_css() -> Result<(), SkillError> {
    let status = std::process::Command::new("bun")
        .arg("run")
        .arg("bin/dashboard/index.ts")
        .arg("--action")
        .arg("css")
        .status()
        .map_err(SkillError::GeneralIo)?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_lint() -> Result<(), SkillError> {
    let status = std::process::Command::new("bun")
        .arg("run")
        .arg("bin/dashboard/index.ts")
        .arg("--action")
        .arg("lint")
        .status()
        .map_err(SkillError::GeneralIo)?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[allow(clippy::too_many_lines, clippy::unused_async)]
async fn run_summary(format: OutputFormat) -> Result<(), SkillError> {
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
    } else {
        // Table 1: Catalog Overview
        let mut overview_table = Table::new();
        overview_table.load_preset(UTF8_FULL);
        overview_table.apply_modifier(UTF8_ROUND_CORNERS);
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
        health_table.load_preset(UTF8_FULL);
        health_table.apply_modifier(UTF8_ROUND_CORNERS);
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
        category_table.load_preset(UTF8_FULL);
        category_table.apply_modifier(UTF8_ROUND_CORNERS);
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
        target_table.load_preset(UTF8_FULL);
        target_table.apply_modifier(UTF8_ROUND_CORNERS);
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
