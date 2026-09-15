// cspell:words Syncer syncer
//! Command handlers for the `ask skills` subcommands.

use std::path::{Path, PathBuf};

use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Table};

use crate::cli::{OutputFormat, SkillsArgs, SkillsCommands};
use crate::commands::{resolve_root, CliError};
use skills_core::artifacts::ArtifactsEngine;
use skills_core::downloader::SkillDownloader;
use skills_core::error::SkillError;
use skills_core::installer::{InstallOptions, Installer, TargetEnvironment, UninstallOptions};
use skills_core::lint::SkillLinter;
use skills_core::models::LintSeverity;
use skills_core::parser::SkillParser;
use skills_core::sync::SkillSyncer;

/// Dispatches catalog operations to skills-core domain engines.
pub async fn run(args: SkillsArgs, format: OutputFormat) -> Result<(), CliError> {
    if let Some(ref action) = args.action {
        return run_action(action, format).await;
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
                run_install(source, target.as_deref()).await
            }
            SkillsCommands::Uninstall { skill, target } => {
                run_uninstall(skill, target.as_deref()).await
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

async fn run_action(action: &str, format: OutputFormat) -> Result<(), CliError> {
    match action {
        "list" => run_list(None, false, format).await,
        "show" => Err(CliError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Action 'show' requires a skill name argument; use 'ask skills show <name>' instead",
        ))),
        "install" => Err(CliError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Action 'install' requires a skill target argument; use 'ask skills install <name>' instead",
        ))),
        "uninstall" => Err(CliError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Action 'uninstall' requires a skill name argument; use 'ask skills uninstall <name>' instead",
        ))),
        "lint" => run_lint(None, false, format).await,
        "sync" => run_sync(false).await,
        "download-resources" => run_download_resources(false).await,
        "clean-resources" => run_clean_resources().await,
        other => Err(CliError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unknown skills action: '{other}'"),
        ))),
    }
}

#[allow(clippy::unused_async)]
async fn run_list(
    category: Option<&str>,
    json_flag: bool,
    format: OutputFormat,
) -> Result<(), CliError> {
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
async fn run_show(query: &str, format: OutputFormat) -> Result<(), CliError> {
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
async fn run_install(source: &str, target: Option<&str>) -> Result<(), CliError> {
    let root = resolve_root()?;
    let source_path = if Path::new(source).join("SKILL.md").exists() {
        PathBuf::from(source)
    } else {
        let skills = SkillParser::discover_skills(&root)?;
        let skill = skills
            .into_iter()
            .find(|s| {
                s.dir_name.eq_ignore_ascii_case(source) || s.name().eq_ignore_ascii_case(source)
            })
            .ok_or_else(|| SkillError::NotFound {
                query: source.to_string(),
            })?;
        root.join(skill.path.parent().unwrap_or_else(|| Path::new("")))
    };

    let target_env = resolve_target_environment(target);
    let options = InstallOptions::default();
    let result = Installer::new()
        .install(&source_path, &target_env, &options)
        .map_err(SkillError::from)?;

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
async fn run_uninstall(skill: &str, target: Option<&str>) -> Result<(), CliError> {
    let target_env = resolve_target_environment(target);
    let options = UninstallOptions::default();
    let result = Installer::new()
        .uninstall(skill, &target_env, &options)
        .map_err(SkillError::from)?;

    println!(
        "Successfully uninstalled skill '{}' from {}",
        result.skill_id,
        target_env.display_name()
    );

    Ok(())
}

fn collect_markdown_files(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            files.push(path.to_path_buf());
        }
    } else if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    collect_markdown_files(&entry_path, files);
                } else if entry_path.extension().and_then(|s| s.to_str()) == Some("md") {
                    files.push(entry_path);
                }
            }
        }
    }
}

#[allow(clippy::unused_async)]
async fn run_lint(path: Option<&Path>, fix: bool, format: OutputFormat) -> Result<(), CliError> {
    let root = resolve_root()?;
    let target = match path {
        Some(p) => p.to_path_buf(),
        None => root.clone(),
    };

    if fix {
        let files_to_scan = if target.is_file() {
            vec![target.clone()]
        } else {
            let scan_dir = if path.is_none() && root.join("skills").is_dir() {
                root.join("skills")
            } else {
                target.clone()
            };
            let mut files = Vec::new();
            collect_markdown_files(&scan_dir, &mut files);
            files
        };

        let mut total_fixes = 0;
        let mut files_modified = 0;

        for file in files_to_scan {
            if let Ok(content) = std::fs::read_to_string(&file) {
                if content.contains('\u{2014}') {
                    let count = content.matches('\u{2014}').count();
                    let sanitized = content.replace('\u{2014}', "-");
                    if std::fs::write(&file, sanitized).is_ok() {
                        total_fixes += count;
                        files_modified += 1;
                    }
                }
            }
        }

        if files_modified > 0 && format != OutputFormat::Json {
            println!("Fixed {total_fixes} issue(s) across {files_modified} file(s).");
        }
    }

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
        }
        .into());
    }

    Ok(())
}

#[allow(clippy::unused_async)]
async fn run_sync(dry_run: bool) -> Result<(), CliError> {
    let root = resolve_root()?;
    let catalog_dir = if root.join("skills").is_dir() {
        root.join("skills")
    } else {
        root.clone()
    };

    let artifacts = if !dry_run {
        Some(ArtifactsEngine::from_env(&root).generate_all()?)
    } else {
        None
    };

    let repo_only = !dry_run
        && (std::env::var("SKILLS_REPO_ONLY").is_ok()
            || std::env::var("PRE_COMMIT").is_ok()
            || std::env::var("CI").is_ok());

    if !repo_only {
        let syncer = SkillSyncer::new(catalog_dir)
            .with_targets(TargetEnvironment::all_standard())
            .with_dry_run(dry_run);

        let plan = syncer.create_plan().map_err(SkillError::from)?;

        println!("Synchronization Plan (actions: {}):", plan.actions.len());
        for action in &plan.actions {
            println!(
                "  [{:?}] {} for {}",
                action.kind,
                action.skill_id,
                action.target_env.display_name()
            );
        }

        let summary = syncer.execute_plan(&plan).map_err(SkillError::from)?;

        println!(
            "Sync complete (dry_run: {}): {} installed, {} updated, {} deleted, {} up-to-date.",
            summary.dry_run, summary.installed, summary.updated, summary.deleted, summary.no_ops
        );
    } else {
        println!("Repository-only sync active: skipping machine target synchronization.");
    }

    if let Some(art) = artifacts {
        let mut repo_files = 0;
        if art.readme_updated {
            repo_files += 1;
        }
        if art.agents_updated {
            repo_files += 1;
        }
        if art.skills_sh_updated {
            repo_files += 1;
        }
        let dashboard_pages = if art.dashboard_generated {
            art.live_skills_count + art.archived_skills_count
        } else {
            0
        };
        println!(
            "Artifacts synchronized: {} repository files updated, {} dashboard pages generated.",
            repo_files, dashboard_pages
        );
    }

    Ok(())
}

async fn run_download(url: &str) -> Result<(), CliError> {
    let content = SkillDownloader::new().fetch_url(url).await?;
    let root = resolve_root()?;
    let cache_dir = root.join(".cache").join("skills");
    std::fs::create_dir_all(&cache_dir)?;

    let filename = SkillDownloader::smart_slugify(url, None);
    let destination = cache_dir.join(&filename);
    std::fs::write(&destination, &content)?;

    println!("Cached remote skill to {}", destination.display());
    println!("{content}");
    Ok(())
}

async fn run_download_resources(_force: bool) -> Result<(), CliError> {
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
async fn run_clean_resources() -> Result<(), CliError> {
    let root = resolve_root()?;
    let skills = SkillParser::discover_skills(&root)?;
    let count = SkillDownloader::new().clean_all_resources(&skills)?;
    println!("Cleaned auto resources across {count} skill directories.");
    Ok(())
}
