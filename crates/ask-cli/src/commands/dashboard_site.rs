//! Native command implementations for static dashboard generation and lifecycle operations.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

use skills_core::artifacts::{ArtifactsEngine, ArtifactsOptions};
use skills_core::error::SkillError;
use tokio::time::interval;

use crate::commands::{resolve_root, CliError};

/// Formats actionable diagnostic text when an external CLI binary is missing.
pub(crate) fn format_tool_not_found_error(tool: &str, action_context: &str) -> String {
    format!(
        "Error: `{tool}` not found - run inside the devenv shell (devenv --no-tui shell -- dashboard --action {action_context})"
    )
}

/// Executes an external CLI tool synchronously from the workspace root.
pub(crate) fn exec_tool(
    root: &Path,
    tool: &str,
    args: &[&str],
    action_context: &str,
) -> Result<(), CliError> {
    let mut cmd = Command::new(tool);
    cmd.args(args).current_dir(root);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(CliError::Subprocess {
                    command: format!("{tool} {}", args.join(" ")),
                    code: status.code(),
                })
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("{}", format_tool_not_found_error(tool, action_context));
            Err(CliError::Io(err))
        }
        Err(err) => Err(CliError::Io(err)),
    }
}

/// Executes an external CLI tool synchronously and captures its standard output.
pub(crate) fn exec_tool_capture(
    root: &Path,
    tool: &str,
    args: &[&str],
    action_context: &str,
) -> Result<String, CliError> {
    let mut cmd = Command::new(tool);
    cmd.args(args).current_dir(root);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(CliError::Subprocess {
                    command: format!("{tool} {}", args.join(" ")),
                    code: output.status.code(),
                })
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("{}", format_tool_not_found_error(tool, action_context));
            Err(CliError::Io(err))
        }
        Err(err) => Err(CliError::Io(err)),
    }
}

/// Scans both active skills and archived skills directories and writes a git-derived freshness metadata TOML sidecar.
pub(crate) fn write_skill_dates(root: &Path) -> Result<(), CliError> {
    let mut dates: BTreeMap<String, String> = BTreeMap::new();
    let mut git_warned = false;
    let base_dirs = ["skills", "skills-archive"];

    for base_dir in base_dirs {
        let base_path = root.join(base_dir);
        if !base_path.is_dir() {
            continue;
        }

        let Ok(categories) = std::fs::read_dir(&base_path) else {
            continue;
        };

        for cat_entry in categories.flatten() {
            let cat_path = cat_entry.path();
            if !cat_path.is_dir() {
                continue;
            }
            let Some(cat_name) = cat_path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let Ok(skills) = std::fs::read_dir(&cat_path) else {
                continue;
            };

            for skill_entry in skills.flatten() {
                let skill_path = skill_entry.path();
                if !skill_path.is_dir() || !skill_path.join("SKILL.md").exists() {
                    continue;
                }
                let Some(skill_name) = skill_path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };

                let skill_rel = format!("{base_dir}/{cat_name}/{skill_name}");
                let mut cmd = Command::new("git");
                cmd.args([
                    "log",
                    "-1",
                    "--date=short",
                    "--format=%cd",
                    "--",
                    &skill_rel,
                ])
                .current_dir(root);

                match cmd.output() {
                    Ok(output) if output.status.success() => {
                        let date_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !date_str.is_empty() {
                            dates.insert(format!("{cat_name}/{skill_name}"), date_str);
                        }
                    }
                    Ok(_) | Err(_) => {
                        if !git_warned {
                            tracing::warn!("Failed to retrieve git history for skills");
                            git_warned = true;
                        }
                    }
                }
            }
        }
    }

    if dates.is_empty() {
        tracing::warn!(
            "No last-modified dates were retrieved from git; freshness dates will be empty"
        );
    }

    let mut content = String::from(
        "# Generated by `dashboard --action build` from git - do not commit.\n[dates]\n",
    );
    for (key, val) in &dates {
        let _ = writeln!(content, "\"{key}\" = \"{val}\"");
    }

    let out_file = root.join("dashboard").join("skill_dates.toml");
    if let Some(parent) = out_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out_file, content).map_err(|e| SkillError::io(out_file, e))?;

    Ok(())
}

/// Cleans development Pagefind cache and target output directory with retry backoff.
pub(crate) fn clean_output_dir(root: &Path, output_dir: &Path) {
    let dev_pagefind = root.join("dashboard/static/pagefind");
    if dev_pagefind.exists() {
        if let Err(err) = std::fs::remove_dir_all(&dev_pagefind) {
            tracing::warn!("Failed to clean dev pagefind directory: {err}");
        }
    }

    if !output_dir.exists() {
        return;
    }

    let max_retries = 5;
    let delay = Duration::from_millis(150);

    for attempt in 1..=max_retries {
        match std::fs::remove_dir_all(output_dir) {
            Ok(()) => return,
            Err(err) if attempt == max_retries => {
                tracing::warn!(
                    "Failed to clean directory {}: {}. Proceeding anyway...",
                    output_dir.display(),
                    err
                );
                return;
            }
            Err(_) => {
                std::thread::sleep(delay);
            }
        }
    }
}

/// Executes full site compilation pipeline for production builds or preview serving.
pub(crate) fn build_site(
    root: &Path,
    output: Option<&Path>,
    serve_mode: bool,
) -> Result<(), CliError> {
    let options = ArtifactsOptions {
        skip_dashboard: false,
        repo_only: false,
        no_stage: false,
        ..Default::default()
    };
    ArtifactsEngine::with_options(root, options).generate_all()?;

    exec_tool(
        root,
        "tailwindcss",
        &[
            "-i",
            "dashboard/css/input.css",
            "-o",
            "dashboard/static/build/css/generated.css",
        ],
        "build",
    )?;

    write_skill_dates(root)?;

    let public_dir = output.map_or_else(|| root.join("dashboard/public"), Path::to_path_buf);
    clean_output_dir(root, &public_dir);

    let mut zola_args = vec!["--root", "dashboard", "build"];
    let out_str;
    if let Some(out) = output {
        out_str = out.to_string_lossy().to_string();
        zola_args.push("-o");
        zola_args.push(&out_str);
    }
    exec_tool(root, "zola", &zola_args, "build")?;

    let public_str = public_dir.to_string_lossy().to_string();
    if serve_mode {
        exec_tool(
            root,
            "pagefind",
            &[
                "--site",
                &public_str,
                "--output-path",
                "dashboard/static/pagefind",
            ],
            "build",
        )?;
    } else {
        exec_tool(
            root,
            "pagefind",
            &["--site", &public_str, "--output-subdir", "pagefind"],
            "build",
        )?;
    }

    Ok(())
}

/// Compiles static documentation site and Pagefind search index.
#[allow(clippy::unused_async)]
pub(crate) async fn run_build(output: Option<&Path>) -> Result<(), CliError> {
    let root = resolve_root()?;
    build_site(&root, output, false)?;
    let dest = output.map_or("dashboard/public", |p| {
        p.to_str().unwrap_or("dashboard/public")
    });
    println!("Done! Site built to {dest}.");
    Ok(())
}

/// Compiles standalone Tailwind CSS stylesheets.
#[allow(clippy::unused_async)]
pub(crate) async fn run_css() -> Result<(), CliError> {
    let root = resolve_root()?;
    exec_tool(
        &root,
        "tailwindcss",
        &[
            "-i",
            "dashboard/css/input.css",
            "-o",
            "dashboard/static/build/css/generated.css",
        ],
        "css",
    )?;
    println!("Done! CSS written to dashboard/static/build/css/generated.css.");
    Ok(())
}

/// Verifies committed documentation content matches freshly generated output without drift.
#[allow(clippy::unused_async)]
pub(crate) async fn run_lint() -> Result<(), CliError> {
    let root = resolve_root()?;

    let options = ArtifactsOptions {
        skip_dashboard: false,
        repo_only: false,
        no_stage: true,
        ..Default::default()
    };
    ArtifactsEngine::with_options(&root, options).generate_all()?;

    let status = exec_tool_capture(
        &root,
        "git",
        &["status", "--porcelain", "--", "dashboard/content"],
        "lint",
    )?;

    let drifted: Vec<&str> = status
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .filter(|line| line.starts_with("??") || (line.len() >= 2 && line.as_bytes()[1] != b' '))
        .collect();

    let _ = Command::new("git")
        .args([
            "checkout",
            "--",
            "README.md",
            "agents/AGENTS.md",
            "skills.sh.json",
            "dashboard/content",
        ])
        .current_dir(&root)
        .status();
    let _ = Command::new("git")
        .args(["clean", "-fdq", "dashboard/content"])
        .current_dir(&root)
        .status();

    if !drifted.is_empty() {
        eprintln!(
            "dashboard/content is out of date with the skills. Run `dashboard --action build`, then stage and commit the regenerated dashboard files."
        );
        eprintln!("Drifted paths:\n{}", drifted.join("\n"));
        return Err(CliError::Subprocess {
            command: "git status --porcelain -- dashboard/content".to_string(),
            code: Some(1),
        });
    }

    println!("Done! Dashboard content is committed and up to date.");
    Ok(())
}

/// Spawns live development server with Tailwind watch and Zola serve.
pub(crate) async fn run_serve(port: u16) -> Result<(), CliError> {
    let root = resolve_root()?;
    build_site(&root, None, true)?;

    let port_str = port.to_string();

    let mut tailwind_cmd = tokio::process::Command::new("tailwindcss");
    tailwind_cmd
        .args([
            "-i",
            "dashboard/css/input.css",
            "-o",
            "dashboard/static/build/css/generated.css",
            "--watch",
        ])
        .current_dir(&root);

    let mut tailwind_child = match tailwind_cmd.spawn() {
        Ok(c) => c,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("{}", format_tool_not_found_error("tailwindcss", "serve"));
            return Err(CliError::Io(err));
        }
        Err(err) => return Err(CliError::Io(err)),
    };

    let mut zola_cmd = tokio::process::Command::new("zola");
    zola_cmd
        .args(["--root", "dashboard", "serve", "-p", &port_str])
        .current_dir(&root);

    let zola_child = match zola_cmd.spawn() {
        Ok(c) => c,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let _ = tailwind_child.kill().await;
            eprintln!("{}", format_tool_not_found_error("zola", "serve"));
            return Err(CliError::Io(err));
        }
        Err(err) => {
            let _ = tailwind_child.kill().await;
            return Err(CliError::Io(err));
        }
    };

    println!("Serving dashboard at http://127.0.0.1:{port} with live reload...");

    drop(tailwind_cmd);
    drop(zola_cmd);
    let res = serve_event_loop(&root, tailwind_child, zola_child).await;
    res
}

#[cfg(unix)]
async fn wait_for_terminate() {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        Ok(mut sig) => {
            sig.recv().await;
        }
        Err(_) => {
            std::future::pending::<()>().await;
        }
    }
}

#[cfg(not(unix))]
async fn wait_for_terminate() {
    std::future::pending::<()>().await;
}

pub(crate) async fn serve_event_loop(
    root: &Path,
    mut tailwind_child: tokio::process::Child,
    mut zola_child: tokio::process::Child,
) -> Result<(), CliError> {
    let mut last_snapshot = snapshot_watch_dirs(root);
    let mut ticker = interval(Duration::from_millis(300));
    let mut pending_debounce: Option<tokio::time::Instant> = None;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\nReceived SIGINT. Shutting down...");
                break;
            }
            () = wait_for_terminate() => {
                println!("\nReceived SIGTERM. Shutting down...");
                break;
            }
            status = tailwind_child.wait() => {
                eprintln!("Tailwind watch process exited unexpectedly: {status:?}");
                break;
            }
            status = zola_child.wait() => {
                eprintln!("Zola serve process exited unexpectedly: {status:?}");
                break;
            }
            _ = ticker.tick() => {
                let current_snapshot = snapshot_watch_dirs(root);
                if current_snapshot != last_snapshot {
                    last_snapshot = current_snapshot;
                    pending_debounce = Some(tokio::time::Instant::now() + Duration::from_millis(300));
                }

                if let Some(deadline) = pending_debounce {
                    if tokio::time::Instant::now() >= deadline {
                        pending_debounce = None;
                        println!("Content changed. Rebuilding search indexes...");
                        rebuild_search_indexes(root);
                    }
                }
            }
        }
    }

    let _ = tailwind_child.kill().await;
    let _ = zola_child.kill().await;
    println!("Done!");
    Ok(())
}

pub(crate) fn snapshot_watch_dirs(root: &Path) -> HashMap<PathBuf, SystemTime> {
    let mut snapshot = HashMap::new();
    let dirs = [
        root.join("skills"),
        root.join("dashboard/content"),
        root.join("dashboard/themes"),
    ];

    for base in &dirs {
        if !base.is_dir() {
            continue;
        }
        collect_snapshot_recursive(base, root, &mut snapshot);
    }

    snapshot
}

pub(crate) fn collect_snapshot_recursive(
    dir: &Path,
    root: &Path,
    map: &mut HashMap<PathBuf, SystemTime>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if name.starts_with('.')
            || name == "node_modules"
            || name == "public"
            || name == "skill_dates.toml"
        {
            continue;
        }

        if let Ok(rel) = path.strip_prefix(root) {
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if rel_str.starts_with("dashboard/content/skills")
                || rel_str.contains("static/pagefind")
            {
                continue;
            }
        }

        if path.is_dir() {
            collect_snapshot_recursive(&path, root, map);
        } else if let Ok(meta) = path.metadata() {
            if let Ok(mtime) = meta.modified() {
                map.insert(path, mtime);
            }
        }
    }
}

pub(crate) fn rebuild_search_indexes(root: &Path) {
    let options = ArtifactsOptions {
        skip_dashboard: false,
        repo_only: false,
        no_stage: false,
        ..Default::default()
    };
    if let Err(err) = ArtifactsEngine::with_options(root, options).generate_all() {
        tracing::warn!("Failed to sync artifacts during live reload: {err}");
        return;
    }
    if let Err(err) = write_skill_dates(root) {
        tracing::warn!("Failed to write skill dates during live reload: {err}");
        return;
    }
    if let Err(err) = exec_tool(root, "zola", &["--root", "dashboard", "build"], "serve") {
        tracing::warn!("Failed to build Zola during live reload: {err}");
        return;
    }
    if let Err(err) = exec_tool(
        root,
        "pagefind",
        &[
            "--site",
            "dashboard/public",
            "--output-path",
            "dashboard/static/pagefind",
        ],
        "serve",
    ) {
        tracing::warn!("Failed to index search assets during live reload: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_format_tool_not_found_error() {
        let msg = format_tool_not_found_error("tailwindcss", "css");
        assert!(msg.contains("Error: `tailwindcss` not found"));
        assert!(msg.contains("devenv --no-tui shell -- dashboard --action css"));
    }

    #[test]
    fn test_write_skill_dates_empty_fallback() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        write_skill_dates(root).expect("write_skill_dates should succeed");

        let sidecar = root.join("dashboard/skill_dates.toml");
        assert!(sidecar.is_file());
        let content = std::fs::read_to_string(&sidecar).expect("failed to read sidecar");
        assert!(
            content.contains("# Generated by `dashboard --action build` from git - do not commit.")
        );
        assert!(content.contains("[dates]"));
    }

    #[test]
    fn test_write_skill_dates_scans_skills_and_archive() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let active_skill = root.join("skills/analysis/code-review");
        std::fs::create_dir_all(&active_skill).expect("failed to create active skill dir");
        std::fs::write(active_skill.join("SKILL.md"), "# Active Skill")
            .expect("failed to write SKILL.md");

        let archive_skill = root.join("skills-archive/planning/plan-before-coding");
        std::fs::create_dir_all(&archive_skill).expect("failed to create archive skill dir");
        std::fs::write(archive_skill.join("SKILL.md"), "# Archive Skill")
            .expect("failed to write SKILL.md");

        write_skill_dates(root).expect("write_skill_dates should succeed");

        let sidecar = root.join("dashboard/skill_dates.toml");
        assert!(sidecar.is_file());
        let content = std::fs::read_to_string(&sidecar).expect("failed to read sidecar");
        assert!(content.contains("[dates]"));
    }

    #[test]
    fn test_clean_output_dir_removes_paths() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();
        let dev_pagefind = root.join("dashboard/static/pagefind");
        std::fs::create_dir_all(&dev_pagefind).expect("failed to create dev pagefind");
        std::fs::write(dev_pagefind.join("test.txt"), "dummy").expect("failed to write dummy file");

        let out_dir = root.join("dashboard/public");
        std::fs::create_dir_all(&out_dir).expect("failed to create public dir");
        std::fs::write(out_dir.join("index.html"), "<html></html>")
            .expect("failed to write html file");

        clean_output_dir(root, &out_dir);

        assert!(!dev_pagefind.exists());
        assert!(!out_dir.exists());
    }

    #[test]
    fn test_clean_output_dir_nonexistent_target() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();
        let out_dir = root.join("dashboard/does_not_exist");

        clean_output_dir(root, &out_dir);
        assert!(!out_dir.exists());
    }

    #[test]
    fn test_exec_tool_not_found_returns_error() {
        let temp = tempdir().expect("failed to create temp dir");
        let result = exec_tool(temp.path(), "nonexistent_binary_xyz_12345", &[], "test");
        assert!(result.is_err());
        match result {
            Err(CliError::Io(err)) => assert_eq!(err.kind(), std::io::ErrorKind::NotFound),
            other => panic!("expected CliError::Io(NotFound), got: {other:?}"),
        }
    }

    #[test]
    fn test_exec_tool_capture_not_found_returns_error() {
        let temp = tempdir().expect("failed to create temp dir");
        let result = exec_tool_capture(temp.path(), "nonexistent_binary_xyz_12345", &[], "test");
        assert!(result.is_err());
        match result {
            Err(CliError::Io(err)) => assert_eq!(err.kind(), std::io::ErrorKind::NotFound),
            other => panic!("expected CliError::Io(NotFound), got: {other:?}"),
        }
    }

    #[test]
    fn test_exec_tool_subprocess_failure() {
        let temp = tempdir().expect("failed to create temp dir");
        let result = exec_tool(temp.path(), "git", &["invalid-subcommand-xyz"], "test");
        assert!(result.is_err());
        match result {
            Err(CliError::Subprocess { code, .. }) => assert_ne!(code, Some(0)),
            other => panic!("expected CliError::Subprocess, got: {other:?}"),
        }
    }

    #[test]
    fn test_porcelain_drift_filtering_includes_archive() {
        let sample = " M dashboard/content/_index.md\n?? dashboard/content/new.md\n?? dashboard/content/skills/archive/planning/_index.md\nMM dashboard/content/both.md\nM  dashboard/content/staged.md\nA  dashboard/content/staged_new.md\n";
        let drifted: Vec<&str> = sample
            .lines()
            .map(str::trim_end)
            .filter(|line| !line.is_empty())
            .filter(|line| {
                line.starts_with("??") || (line.len() >= 2 && line.as_bytes()[1] != b' ')
            })
            .collect();

        assert_eq!(drifted.len(), 4);
        assert!(drifted.contains(&" M dashboard/content/_index.md"));
        assert!(drifted.contains(&"?? dashboard/content/new.md"));
        assert!(drifted.contains(&"?? dashboard/content/skills/archive/planning/_index.md"));
        assert!(drifted.contains(&"MM dashboard/content/both.md"));
        assert!(!drifted.contains(&"M  dashboard/content/staged.md"));
        assert!(!drifted.contains(&"A  dashboard/content/staged_new.md"));
    }
}
