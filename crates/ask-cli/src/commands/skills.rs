// cspell:words Syncer syncer
//! Command handlers for the `ask skills` subcommands.

use std::path::{Path, PathBuf};

use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Table};

use crate::cli::{OutputFormat, SkillsArgs, SkillsCommands};
use crate::commands::resolve_root;
use skills_core::downloader::SkillDownloader;
use skills_core::error::SkillError;
use skills_core::installer::{InstallOptions, Installer, TargetEnvironment, UninstallOptions};
use skills_core::lint::SkillLinter;
use skills_core::models::LintSeverity;
use skills_core::parser::SkillParser;
use skills_core::sync::SkillSyncer;

/// Dispatches catalog operations to skills-core domain engines.
pub async fn run(args: SkillsArgs, format: OutputFormat) -> Result<(), SkillError> {
    if let Some(ref action) = args.action {
        return run_action_script(action);
    }

    if let Some(ref cmd) = args.command {
        tracing::debug!(
            "Dispatching skills subcommand: {:?}, format: {:?}",
            cmd,
            format
        );

        match cmd {
            SkillsCommands::List { category, json } => {
                run_list(category.as_deref(), *json, format).await
            }
            SkillsCommands::Show { skill } => run_show(skill, format).await,
            SkillsCommands::Install { source, target } => {
                run_install(source.as_deref(), target.as_deref()).await
            }
            SkillsCommands::Uninstall { skill, target } => {
                run_uninstall(skill.as_deref(), target.as_deref()).await
            }
            SkillsCommands::Lint { path, fix } => run_lint(path.as_deref(), *fix, format).await,
            SkillsCommands::Sync { dry_run } => run_sync(*dry_run).await,
            SkillsCommands::Download { url } => run_download(url).await,
            SkillsCommands::DownloadResources { force } => run_download_resources(*force).await,
            SkillsCommands::CleanResources => run_clean_resources().await,
        }
    } else {
        run_list(None, false, format).await
    }
}

fn run_action_script(action: &str) -> Result<(), SkillError> {
    let status = std::process::Command::new("bun")
        .arg("run")
        .arg("bin/skills/index.ts")
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
async fn run_list(
    category: Option<&str>,
    json_flag: bool,
    format: OutputFormat,
) -> Result<(), SkillError> {
    let root = resolve_root()?;
    let mut skills = SkillParser::discover_skills(&root)?;
    skills.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));

    if let Some(cat) = category {
        skills.retain(|s| s.category.as_str().eq_ignore_ascii_case(cat));
    }

    if json_flag || format == OutputFormat::Json {
        let json = serde_json::to_string_pretty(&skills).map_err(|e| SkillError::Json {
            path: root.clone(),
            source: e,
        })?;
        println!("{json}");
    } else if format == OutputFormat::Yaml {
        let yaml = serde_yaml::to_string(&skills).map_err(|e| SkillError::yaml(&root, e))?;
        println!("{yaml}");
    } else if format == OutputFormat::Plain {
        for skill in &skills {
            println!(
                "{}\t{}\t{}\t{}",
                skill.dir_name,
                skill.category,
                skill.name(),
                skill.description()
            );
        }
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            Cell::new("ID / Directory").add_attribute(Attribute::Bold),
            Cell::new("Name").add_attribute(Attribute::Bold),
            Cell::new("Category").add_attribute(Attribute::Bold),
            Cell::new("Tree").add_attribute(Attribute::Bold),
            Cell::new("Promoted").add_attribute(Attribute::Bold),
            Cell::new("Description").add_attribute(Attribute::Bold),
        ]);

        for skill in &skills {
            table.add_row(vec![
                Cell::new(&skill.dir_name),
                Cell::new(skill.name()),
                Cell::new(skill.category.as_str()),
                Cell::new(skill.tree.as_str()),
                Cell::new(if skill.promoted { "yes" } else { "no" }),
                Cell::new(skill.description()),
            ]);
        }
        println!("{table}");
    }

    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_show(query: &str, format: OutputFormat) -> Result<(), SkillError> {
    let root = resolve_root()?;
    let skills = SkillParser::discover_skills(&root)?;
    let skill = skills
        .into_iter()
        .find(|s| {
            s.dir_name.eq_ignore_ascii_case(query)
                || s.name().eq_ignore_ascii_case(query)
                || s.path.to_string_lossy().contains(query)
        })
        .ok_or_else(|| SkillError::NotFound {
            query: query.to_string(),
        })?;

    if format == OutputFormat::Json {
        let json = serde_json::to_string_pretty(&skill).map_err(|e| SkillError::Json {
            path: root.clone(),
            source: e,
        })?;
        println!("{json}");
    } else if format == OutputFormat::Yaml {
        let yaml = serde_yaml::to_string(&skill).map_err(|e| SkillError::yaml(&root, e))?;
        println!("{yaml}");
    } else {
        println!("Name:        {}", skill.name());
        println!("Identifier:  {}", skill.dir_name);
        println!("Category:    {}", skill.category.as_str());
        println!("Origin Tree: {}", skill.tree.as_str());
        println!("Promoted:    {}", if skill.promoted { "yes" } else { "no" });
        println!("Description: {}", skill.description());
        println!();
        println!(
            "Context:     {}",
            skill.frontmatter.context.as_deref().unwrap_or("inline")
        );
        println!(
            "Target Agent:{}",
            skill.frontmatter.agent.as_deref().unwrap_or("none")
        );
        println!(
            "User Invoked:{}",
            if skill.is_user_invoked() { "yes" } else { "no" }
        );
        println!(
            "Argument Hint:{}",
            skill.frontmatter.argument_hint.as_deref().unwrap_or("none")
        );
        println!();
        println!("Resources ({}):", skill.resources.len());
        for res in &skill.resources {
            println!("  - {} ({:?})", res.relative_path.display(), res.kind);
        }
        println!();
        println!("--- Content ---");
        println!("{}", skill.content);
    }

    Ok(())
}

fn resolve_target_environment(target: Option<&str>) -> TargetEnvironment {
    match target {
        None | Some("antigravity" | "agy") => TargetEnvironment::Antigravity {
            workspace_root: None,
        },
        Some("claude" | "claude-desktop") => TargetEnvironment::ClaudeDesktop,
        Some("cursor") => TargetEnvironment::Cursor,
        Some(custom) => TargetEnvironment::Custom(PathBuf::from(custom)),
    }
}

#[allow(clippy::unused_async)]
async fn run_install(source: Option<&str>, target: Option<&str>) -> Result<(), SkillError> {
    let source_str = source.ok_or_else(|| SkillError::NotFound {
        query: "source argument is required for install".to_string(),
    })?;

    let root = resolve_root()?;
    let source_path = if Path::new(source_str).join("SKILL.md").exists() {
        PathBuf::from(source_str)
    } else {
        let skills = SkillParser::discover_skills(&root)?;
        let skill = skills
            .into_iter()
            .find(|s| {
                s.dir_name.eq_ignore_ascii_case(source_str)
                    || s.name().eq_ignore_ascii_case(source_str)
            })
            .ok_or_else(|| SkillError::NotFound {
                query: source_str.to_string(),
            })?;
        root.join(skill.path.parent().unwrap_or_else(|| Path::new("")))
    };

    let target_env = resolve_target_environment(target);
    let options = InstallOptions::default();
    let result = Installer::new().install(&source_path, &target_env, &options)?;

    println!(
        "Successfully installed skill '{}' (version {}) into {} via {:?}",
        result.skill.name,
        result.skill.version,
        target_env.display_name(),
        result.installed_mode
    );

    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_uninstall(skill: Option<&str>, target: Option<&str>) -> Result<(), SkillError> {
    let skill_id = skill.ok_or_else(|| SkillError::NotFound {
        query: "skill identifier is required for uninstall".to_string(),
    })?;

    let target_env = resolve_target_environment(target);
    let options = UninstallOptions::default();
    let result = Installer::new().uninstall(skill_id, &target_env, &options)?;

    println!(
        "Successfully uninstalled skill '{}' from {}",
        result.skill_id,
        target_env.display_name()
    );

    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_lint(path: Option<&Path>, _fix: bool, format: OutputFormat) -> Result<(), SkillError> {
    let root = resolve_root()?;
    let target = match path {
        Some(p) => p.to_path_buf(),
        None => root.clone(),
    };

    let linter = SkillLinter::new();
    let report = if target.is_file() {
        linter.lint_file(&target)?
    } else {
        linter.lint_repository(&target)?
    };

    if format == OutputFormat::Json {
        let json = serde_json::to_string_pretty(&report.issues).map_err(|e| SkillError::Json {
            path: root.clone(),
            source: e,
        })?;
        println!("{json}");
    } else {
        for issue in &report.issues {
            let tag = match issue.severity {
                LintSeverity::Error => "[ERROR]".red().bold(),
                LintSeverity::Warning => "[WARN]".yellow().bold(),
            };
            println!(
                "{} {}:{}:{} - [{}] {}",
                tag,
                issue.file.display(),
                issue.line.unwrap_or(1),
                issue.column.unwrap_or(1),
                issue.rule,
                issue.message
            );
        }
        println!(
            "Found {} error(s) and {} warning(s).",
            report.error_count(),
            report.warning_count()
        );
    }

    if report.has_errors() {
        return Err(SkillError::Lint {
            count: report.error_count(),
            details: format!("Found {} lint error(s)", report.error_count()),
        });
    }

    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_sync(dry_run: bool) -> Result<(), SkillError> {
    let root = resolve_root()?;
    let catalog_dir = if root.join("skills").is_dir() {
        root.join("skills")
    } else {
        root
    };

    let syncer = SkillSyncer::new(catalog_dir)
        .with_targets(TargetEnvironment::all_standard())
        .with_dry_run(dry_run);

    let plan = syncer.create_plan()?;

    println!("Synchronization Plan (actions: {}):", plan.actions.len());
    for action in &plan.actions {
        println!(
            "  [{:?}] {} for {}",
            action.kind,
            action.skill_id,
            action.target_env.display_name()
        );
    }

    let summary = syncer.execute_plan(&plan)?;

    println!(
        "Sync complete (dry_run: {}): {} installed, {} updated, {} deleted, {} up-to-date.",
        summary.dry_run, summary.installed, summary.updated, summary.deleted, summary.no_ops
    );

    Ok(())
}

async fn run_download(url: &str) -> Result<(), SkillError> {
    let content = SkillDownloader::new().fetch_url(url).await?;
    println!("{content}");
    Ok(())
}

async fn run_download_resources(_force: bool) -> Result<(), SkillError> {
    let root = resolve_root()?;
    let skills = SkillParser::discover_skills(&root)?;
    let downloader = SkillDownloader::new();
    let results = downloader.download_all_resources(&skills).await?;
    let total_downloaded: usize = results.values().map(Vec::len).sum();
    println!(
        "Downloaded {total_downloaded} resource files across {} skills.",
        results.len()
    );
    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_clean_resources() -> Result<(), SkillError> {
    let root = resolve_root()?;
    let skills = SkillParser::discover_skills(&root)?;
    let count = SkillDownloader::new().clean_all_resources(&skills)?;
    println!("Cleaned auto resources across {count} skill directories.");
    Ok(())
}
