//! Native command implementations for static dashboard generation and lifecycle operations.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use skills_core::artifacts::{
    ArtifactsEngine, ArtifactsOptions, DISCOVERY_SKILL_FILE, WELL_KNOWN_AGENT_SKILLS_DIR,
    WELL_KNOWN_SKILLS_DIR, WELL_KNOWN_STATIC_DIR, select_discovery_skills,
};
use skills_core::error::SkillError;
use skills_core::parser::SkillParser;
use tokio::time::interval;

use crate::commands::{CliError, resolve_root};

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

/// Resolves the dashboard output directory, anchoring relative paths to the workspace root.
pub(crate) fn resolve_output_dir(root: &Path, output: Option<&Path>) -> PathBuf {
    match output {
        None => root.join("dashboard/public"),
        Some(path) if path.is_relative() => root.join(path),
        Some(path) => path.to_path_buf(),
    }
}

/// Copies raw SKILL.md bytes for all discovery-eligible skills into `<output_dir>/.well-known/`.
pub(crate) fn materialize_well_known(root: &Path, output_dir: &Path) -> Result<usize, CliError> {
    let skills = SkillParser::discover_skills(root)?;
    let selected = select_discovery_skills(&skills);
    let count = selected.len();

    for skill in selected {
        let src = if skill.path.is_absolute() {
            skill.path.clone()
        } else {
            root.join(&skill.path)
        };
        for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
            let dest_dir = output_dir.join(rel_dir).join(skill.name());
            std::fs::create_dir_all(&dest_dir).map_err(|err| SkillError::io(&dest_dir, err))?;
            let dest_file = dest_dir.join(DISCOVERY_SKILL_FILE);
            std::fs::copy(&src, &dest_file).map_err(|err| SkillError::io(&dest_file, err))?;
        }
    }

    Ok(count)
}

/// Cleans development Pagefind cache and target output directory with retry backoff.
pub(crate) fn clean_output_dir(root: &Path, output_dir: &Path) {
    let dev_pagefind = root.join("dashboard/static/pagefind");
    if dev_pagefind.exists()
        && let Err(err) = std::fs::remove_dir_all(&dev_pagefind)
    {
        tracing::warn!("Failed to clean dev pagefind directory: {err}");
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

    let public_dir = resolve_output_dir(root, output);
    clean_output_dir(root, &public_dir);

    let mut zola_args = vec!["--root", "dashboard", "build"];
    let out_str;
    if let Some(out) = output {
        out_str = out.to_string_lossy().to_string();
        zola_args.push("-o");
        zola_args.push(&out_str);
    }
    exec_tool(root, "zola", &zola_args, "build")?;
    materialize_well_known(root, &public_dir)?;

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

/// Extracts unstaged and untracked lines from `git status --porcelain` output.
pub(crate) fn porcelain_drift(status: &str) -> Vec<&str> {
    status
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .filter(|line| line.starts_with("??") || (line.len() >= 2 && line.as_bytes()[1] != b' '))
        .collect()
}

static STAGING_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Scoped temporary directory used for non-destructive artifact staging during lint.
struct StagingDir {
    path: PathBuf,
}

impl StagingDir {
    fn new(prefix: &str) -> Result<Self, CliError> {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let seq = STAGING_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("{prefix}-{pid}-{nanos}-{seq}"));
        std::fs::create_dir_all(&path).map_err(|err| SkillError::io(&path, err))?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for StagingDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Recursively collects regular file paths under `dir`, formatted as POSIX paths relative to `base`.
fn collect_relative_files(
    base: &Path,
    dir: &Path,
    out: &mut BTreeSet<String>,
) -> Result<(), CliError> {
    if !dir.is_dir() {
        return Ok(());
    }
    let entries = std::fs::read_dir(dir).map_err(|err| SkillError::io(dir, err))?;
    for entry in entries {
        let entry = entry.map_err(|err| SkillError::io(dir, err))?;
        let path = entry.path();
        if path.is_dir() {
            collect_relative_files(base, &path, out)?;
        } else if path.is_file()
            && let Ok(rel) = path.strip_prefix(base)
        {
            out.insert(rel.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(())
}

/// Compares generated dashboard trees between `staged_root` and `workspace_root` without modifying `workspace_root`.
pub(crate) fn compare_generated_trees(
    staged_root: &Path,
    workspace_root: &Path,
) -> Result<Vec<String>, CliError> {
    let mut drifted = Vec::new();
    for rel_tree in ["dashboard/content/skills", WELL_KNOWN_STATIC_DIR] {
        let mut rel_files = BTreeSet::new();
        collect_relative_files(staged_root, &staged_root.join(rel_tree), &mut rel_files)?;
        collect_relative_files(
            workspace_root,
            &workspace_root.join(rel_tree),
            &mut rel_files,
        )?;

        for rel_path in rel_files {
            let staged_file = staged_root.join(&rel_path);
            let workspace_file = workspace_root.join(&rel_path);
            if !workspace_file.is_file() {
                drifted.push(format!(" D {rel_path}"));
            } else if !staged_file.is_file() {
                drifted.push(format!("?? {rel_path}"));
            } else {
                let staged_bytes =
                    std::fs::read(&staged_file).map_err(|err| SkillError::io(&staged_file, err))?;
                let workspace_bytes = std::fs::read(&workspace_file)
                    .map_err(|err| SkillError::io(&workspace_file, err))?;
                if staged_bytes != workspace_bytes {
                    drifted.push(format!(" M {rel_path}"));
                }
            }
        }
    }
    Ok(drifted)
}

/// Verifies committed documentation content at `root` matches freshly staged output without mutating `root`.
pub(crate) fn check_lint_at(root: &Path) -> Result<(), CliError> {
    let staging = StagingDir::new("ask-dashboard-lint")?;

    let options = ArtifactsOptions {
        skip_dashboard: false,
        repo_only: false,
        no_stage: true,
        output_root: Some(staging.path().to_path_buf()),
        ..Default::default()
    };
    ArtifactsEngine::with_options(root, options).generate_all()?;

    let mut drifted = compare_generated_trees(staging.path(), root)?;

    let status = exec_tool_capture(
        root,
        "git",
        &[
            "status",
            "--porcelain",
            "--",
            "dashboard/content",
            WELL_KNOWN_STATIC_DIR,
        ],
        "lint",
    )?;

    for line in porcelain_drift(&status) {
        let rel_path = line.get(3..).unwrap_or(line).trim();
        let already_recorded = drifted
            .iter()
            .any(|entry| entry.get(3..).unwrap_or(entry).trim() == rel_path);
        if !already_recorded {
            drifted.push(line.to_string());
        }
    }

    if !drifted.is_empty() {
        eprintln!(
            "dashboard/content or {WELL_KNOWN_STATIC_DIR} is out of date with the skills. Run `dashboard --action build`, then stage and commit the regenerated dashboard files."
        );
        eprintln!("Drifted paths:\n{}", drifted.join("\n"));
        return Err(CliError::Subprocess {
            command: format!("git status --porcelain -- dashboard/content {WELL_KNOWN_STATIC_DIR}"),
            code: Some(1),
        });
    }

    println!("Done! Dashboard content is committed and up to date.");
    Ok(())
}

/// Verifies committed documentation content matches freshly generated output without drift.
#[allow(clippy::unused_async)]
pub(crate) async fn run_lint() -> Result<(), CliError> {
    let root = resolve_root()?;
    check_lint_at(&root)
}

/// Spawns the Tailwind CSS watcher subprocess.
pub(crate) fn spawn_tailwind(root: &Path) -> Result<tokio::process::Child, CliError> {
    let mut tailwind_cmd = tokio::process::Command::new("tailwindcss");
    tailwind_cmd
        .args([
            "-i",
            "dashboard/css/input.css",
            "-o",
            "dashboard/static/build/css/generated.css",
            "--watch",
        ])
        .current_dir(root);

    tailwind_cmd.spawn().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            eprintln!("{}", format_tool_not_found_error("tailwindcss", "serve"));
        }
        CliError::Io(err)
    })
}

/// Spawns the Zola development server subprocess.
pub(crate) fn spawn_zola(root: &Path, port: &str) -> Result<tokio::process::Child, CliError> {
    let mut zola_cmd = tokio::process::Command::new("zola");
    zola_cmd
        .args(["--root", "dashboard", "serve", "-p", port])
        .current_dir(root);

    zola_cmd.spawn().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            eprintln!("{}", format_tool_not_found_error("zola", "serve"));
        }
        CliError::Io(err)
    })
}

/// Spawns live development server with Tailwind watch and Zola serve.
pub(crate) async fn run_serve(port: u16) -> Result<(), CliError> {
    let root = resolve_root()?;
    build_site(&root, None, true)?;

    let mut tailwind_child = spawn_tailwind(&root)?;
    let port_str = port.to_string();
    let zola_child = match spawn_zola(&root, &port_str) {
        Ok(c) => c,
        Err(err) => {
            let _ = tailwind_child.kill().await;
            return Err(err);
        }
    };

    println!("Serving dashboard at http://127.0.0.1:{port} with live reload...");
    serve_event_loop(&root, tailwind_child, zola_child).await
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

                if let Some(deadline) = pending_debounce
                    && tokio::time::Instant::now() >= deadline {
                        pending_debounce = None;
                        println!("Content changed. Rebuilding search indexes...");
                        rebuild_search_indexes(root);
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

/// Checks whether a filesystem entry should be skipped during dashboard watch snapshotting.
pub(crate) fn is_ignored_snapshot_entry(name: &str, path: &Path, root: &Path) -> bool {
    const IGNORED_NAMES: &[&str] = &["node_modules", "public", "skill_dates.toml"];
    if name.starts_with('.') || IGNORED_NAMES.contains(&name) {
        return true;
    }

    if let Ok(rel) = path.strip_prefix(root) {
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        if rel_str.starts_with("dashboard/content/skills") || rel_str.contains("static/pagefind") {
            return true;
        }
    }

    false
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

        if is_ignored_snapshot_entry(name, &path, root) {
            continue;
        }

        if path.is_dir() {
            collect_snapshot_recursive(&path, root, map);
        } else if let Ok(meta) = path.metadata()
            && let Ok(mtime) = meta.modified()
        {
            map.insert(path, mtime);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn run_live_reload_step<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> bool {
    match result {
        Ok(_) => true,
        Err(err) => {
            tracing::warn!("Failed to {context} during live reload: {err}");
            false
        }
    }
}

fn sync_live_reload_inputs(root: &Path) -> bool {
    let options = ArtifactsOptions {
        skip_dashboard: false,
        repo_only: false,
        no_stage: false,
        ..Default::default()
    };
    run_live_reload_step(
        ArtifactsEngine::with_options(root, options).generate_all(),
        "sync artifacts",
    ) && run_live_reload_step(write_skill_dates(root), "write skill dates")
}

fn prepare_live_reload_site(root: &Path) -> bool {
    sync_live_reload_inputs(root)
        && run_live_reload_step(
            exec_tool(root, "zola", &["--root", "dashboard", "build"], "serve"),
            "build Zola",
        )
}

pub(crate) fn rebuild_search_indexes(root: &Path) {
    if !prepare_live_reload_site(root) {
        return;
    }
    let _ = run_live_reload_step(
        materialize_well_known(root, &root.join("dashboard/public")),
        "materialize .well-known skills",
    );
    let _ = run_live_reload_step(
        exec_tool(
            root,
            "pagefind",
            &[
                "--site",
                "dashboard/public",
                "--output-path",
                "dashboard/static/pagefind",
            ],
            "serve",
        ),
        "index search assets",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
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
        let sample = concat!(
            " M dashboard/content/_index.md\n",
            "?? dashboard/content/new.md\n",
            "?? dashboard/content/skills/archive/planning/_index.md\n",
            "MM dashboard/content/both.md\n",
            " M dashboard/static/.well-known/agent-skills/index.json\n",
            "?? dashboard/static/.well-known/skills/index.json\n",
            "M  dashboard/content/staged.md\n",
            "A  dashboard/content/staged_new.md\n",
            "A  dashboard/static/.well-known/agent-skills/index.json\n",
            "M  dashboard/static/.well-known/skills/index.json\n",
        );
        let drifted = porcelain_drift(sample);

        assert_eq!(drifted.len(), 6);
        assert!(drifted.contains(&" M dashboard/content/_index.md"));
        assert!(drifted.contains(&"?? dashboard/content/new.md"));
        assert!(drifted.contains(&"?? dashboard/content/skills/archive/planning/_index.md"));
        assert!(drifted.contains(&"MM dashboard/content/both.md"));
        assert!(drifted.contains(&" M dashboard/static/.well-known/agent-skills/index.json"));
        assert!(drifted.contains(&"?? dashboard/static/.well-known/skills/index.json"));
        assert!(!drifted.contains(&"M  dashboard/content/staged.md"));
        assert!(!drifted.contains(&"A  dashboard/content/staged_new.md"));
        assert!(!drifted.contains(&"A  dashboard/static/.well-known/agent-skills/index.json"));
        assert!(!drifted.contains(&"M  dashboard/static/.well-known/skills/index.json"));
    }

    #[test]
    fn test_resolve_output_dir_handles_default_relative_and_absolute() {
        let root = Path::new("/workspace/repo");

        assert_eq!(
            resolve_output_dir(root, None),
            PathBuf::from("/workspace/repo/dashboard/public")
        );
        assert_eq!(
            resolve_output_dir(root, Some(Path::new("dist/site"))),
            PathBuf::from("/workspace/repo/dist/site")
        );
        assert_eq!(
            resolve_output_dir(root, Some(Path::new("/tmp/custom-site"))),
            PathBuf::from("/tmp/custom-site")
        );
    }

    #[test]
    fn test_materialize_well_known_copies_raw_bytes_to_both_trees() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let skill_dir = root.join("skills/engineering/raw-copy");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let raw_bytes =
            b"---   \r\nname: raw-copy\r\ndescription: Raw copy check\r\n...\r\n# raw-copy\r\n\r\nExact bytes.\r\n";
        std::fs::write(skill_dir.join("SKILL.md"), raw_bytes).unwrap();

        let out_dir = root.join("dashboard/public");
        let count = materialize_well_known(root, &out_dir).unwrap();
        assert_eq!(count, 1);

        let v020_copy = out_dir
            .join(WELL_KNOWN_AGENT_SKILLS_DIR)
            .join("raw-copy")
            .join(DISCOVERY_SKILL_FILE);
        let v010_copy = out_dir
            .join(WELL_KNOWN_SKILLS_DIR)
            .join("raw-copy")
            .join(DISCOVERY_SKILL_FILE);

        assert_eq!(std::fs::read(v020_copy).unwrap(), raw_bytes);
        assert_eq!(std::fs::read(v010_copy).unwrap(), raw_bytes);
    }

    #[test]
    fn test_materialize_well_known_skips_vendor_archive_and_drafts() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let live_dir = root.join("skills/engineering/live-tool");
        std::fs::create_dir_all(&live_dir).unwrap();
        std::fs::write(
            live_dir.join("SKILL.md"),
            "---\nname: live-tool\ndescription: Live skill\n---\n# live-tool\n",
        )
        .unwrap();

        let vendor_dir = root.join("skills/vendor/cloudflare/vendor-tool");
        std::fs::create_dir_all(&vendor_dir).unwrap();
        std::fs::write(
            vendor_dir.join("SKILL.md"),
            "---\nname: vendor-tool\ndescription: Vendor skill\n---\n# vendor-tool\n",
        )
        .unwrap();

        let archive_dir = root.join("skills-archive/engineering/retired-tool");
        std::fs::create_dir_all(&archive_dir).unwrap();
        std::fs::write(
            archive_dir.join("SKILL.md"),
            "---\nname: retired-tool\ndescription: Retired skill\nmetadata:\n  archived: \"2025-01-01\"\n---\n# retired-tool\n",
        )
        .unwrap();

        let draft_dir = root.join("skills/in-progress/draft-tool");
        std::fs::create_dir_all(&draft_dir).unwrap();
        std::fs::write(
            draft_dir.join("SKILL.md"),
            "---\nname: draft-tool\ndescription: Draft skill\n---\n# draft-tool\n",
        )
        .unwrap();

        let out_dir = root.join("dashboard/public");
        let count = materialize_well_known(root, &out_dir).unwrap();
        assert_eq!(count, 1);

        for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
            let tree_dir = out_dir.join(rel_dir);
            assert!(tree_dir.join("live-tool/SKILL.md").is_file());
            assert!(!tree_dir.join("vendor-tool").exists());
            assert!(!tree_dir.join("retired-tool").exists());
            assert!(!tree_dir.join("draft-tool").exists());
        }
    }

    #[test]
    fn test_materialize_well_known_matches_generated_index() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        for (cat, name) in [("writing", "zeta-doc"), ("engineering", "alpha-build")] {
            let dir = root.join("skills").join(cat).join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                dir.join("SKILL.md"),
                format!("---\nname: {name}\ndescription: Desc for {name}\n---\n# {name}\n"),
            )
            .unwrap();
        }
        let vendor_dir = root.join("skills/vendor/cloudflare/ignored-vendor");
        std::fs::create_dir_all(&vendor_dir).unwrap();
        std::fs::write(
            vendor_dir.join("SKILL.md"),
            "---\nname: ignored-vendor\ndescription: Vendor\n---\n# ignored-vendor\n",
        )
        .unwrap();

        let engine = ArtifactsEngine::with_options(
            root,
            ArtifactsOptions {
                no_stage: true,
                ..Default::default()
            },
        );
        engine.generate_all().unwrap();

        let out_dir = root.join("dashboard/public");
        let count = materialize_well_known(root, &out_dir).unwrap();
        assert_eq!(count, 2);

        let index_path = root
            .join("dashboard/static")
            .join(WELL_KNOWN_AGENT_SKILLS_DIR)
            .join("index.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(index_path).unwrap()).unwrap();
        let entries = parsed["skills"].as_array().unwrap();

        let indexed_names: BTreeSet<String> = entries
            .iter()
            .map(|e| e["name"].as_str().unwrap().to_string())
            .collect();

        for rel_dir in [WELL_KNOWN_AGENT_SKILLS_DIR, WELL_KNOWN_SKILLS_DIR] {
            let materialized_names: BTreeSet<String> = std::fs::read_dir(out_dir.join(rel_dir))
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            assert_eq!(materialized_names, indexed_names);
        }

        for entry in entries {
            let name = entry["name"].as_str().unwrap();
            let expected_digest = entry["digest"].as_str().unwrap();
            let served_bytes = std::fs::read(
                out_dir
                    .join(WELL_KNOWN_AGENT_SKILLS_DIR)
                    .join(name)
                    .join(DISCOVERY_SKILL_FILE),
            )
            .unwrap();
            let actual_digest = format!(
                "sha256:{}",
                skills_core::SkillDownloader::compute_sha256(&served_bytes)
            );
            assert_eq!(actual_digest, expected_digest);
        }
    }

    #[test]
    fn test_check_lint_at_preserves_unstaged_edits_and_untracked_files() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let git = |args: &[&str]| {
            let status = Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .expect("git command failed to spawn");
            assert!(status.success(), "git {args:?} failed");
        };

        git(&["init"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test User"]);

        let skill_dir = root.join("skills/engineering/demo-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: Demo skill\n---\n# demo-skill\n\nBody.\n",
        )
        .unwrap();
        std::fs::create_dir_all(root.join("agents")).unwrap();
        std::fs::create_dir_all(root.join("dashboard/content")).unwrap();
        std::fs::create_dir_all(root.join("dashboard/static")).unwrap();
        std::fs::write(root.join("dashboard/content/_index.md"), "committed page\n").unwrap();
        std::fs::write(root.join("dashboard/static/CNAME"), "skills.mahdtech.com\n").unwrap();

        ArtifactsEngine::with_options(
            root,
            ArtifactsOptions {
                no_stage: true,
                ..Default::default()
            },
        )
        .generate_all()
        .unwrap();

        git(&["add", "."]);
        git(&["commit", "-m", "chore: initial fixture"]);

        // Clean committed state passes lint.
        check_lint_at(root).expect("clean committed state should pass lint");

        // Unstaged edits to non-dashboard files must be ignored by dashboard lint and preserved.
        std::fs::write(root.join("README.md"), "dirty readme\n").unwrap();
        std::fs::write(root.join("agents/AGENTS.md"), "dirty agents\n").unwrap();
        std::fs::write(root.join("skills.sh.json"), "{\"dirty\":true}\n").unwrap();

        check_lint_at(root).expect("dirty repo manifests should not fail dashboard lint");
        assert_eq!(
            std::fs::read_to_string(root.join("README.md")).unwrap(),
            "dirty readme\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("agents/AGENTS.md")).unwrap(),
            "dirty agents\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("skills.sh.json")).unwrap(),
            "{\"dirty\":true}\n"
        );

        // Unstaged edit to dashboard/content/_index.md and untracked files under dashboard/content
        // and dashboard/static/.well-known must fail lint WITHOUT reverting or deleting them.
        std::fs::write(root.join("dashboard/content/_index.md"), "dirty landing\n").unwrap();
        let untracked_content = root.join("dashboard/content/untracked-draft.md");
        std::fs::write(&untracked_content, "draft content\n").unwrap();
        let untracked_well_known = root.join(WELL_KNOWN_STATIC_DIR).join("extra.json");
        std::fs::write(&untracked_well_known, "{\"extra\":true}\n").unwrap();

        let err = check_lint_at(root).expect_err("dirty dashboard files should fail lint");
        assert!(matches!(err, CliError::Subprocess { code: Some(1), .. }));

        assert_eq!(
            std::fs::read_to_string(root.join("README.md")).unwrap(),
            "dirty readme\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("agents/AGENTS.md")).unwrap(),
            "dirty agents\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("skills.sh.json")).unwrap(),
            "{\"dirty\":true}\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("dashboard/content/_index.md")).unwrap(),
            "dirty landing\n"
        );
        assert_eq!(
            std::fs::read_to_string(&untracked_content).unwrap(),
            "draft content\n"
        );
        assert_eq!(
            std::fs::read_to_string(&untracked_well_known).unwrap(),
            "{\"extra\":true}\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("dashboard/static/CNAME")).unwrap(),
            "skills.mahdtech.com\n"
        );
    }

    #[test]
    fn test_is_ignored_snapshot_entry() {
        let root = Path::new("/mock/repo");

        assert!(is_ignored_snapshot_entry(".git", &root.join(".git"), root));
        assert!(is_ignored_snapshot_entry(
            ".hidden",
            &root.join(".hidden"),
            root
        ));
        assert!(is_ignored_snapshot_entry(
            "node_modules",
            &root.join("node_modules"),
            root
        ));
        assert!(is_ignored_snapshot_entry(
            "public",
            &root.join("public"),
            root
        ));
        assert!(is_ignored_snapshot_entry(
            "skill_dates.toml",
            &root.join("skill_dates.toml"),
            root
        ));

        assert!(is_ignored_snapshot_entry(
            "file.md",
            &root.join("dashboard/content/skills/engineering/test.md"),
            root
        ));
        assert!(is_ignored_snapshot_entry(
            "pagefind.js",
            &root.join("dashboard/static/pagefind/pagefind.js"),
            root
        ));

        assert!(!is_ignored_snapshot_entry(
            "SKILL.md",
            &root.join("skills/engineering/my-skill/SKILL.md"),
            root
        ));
        assert!(!is_ignored_snapshot_entry(
            "index.html",
            &root.join("dashboard/templates/index.html"),
            root
        ));
    }

    #[test]
    fn test_collect_snapshot_recursive() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let skills_dir = root.join("skills/engineering/sample");
        std::fs::create_dir_all(&skills_dir).unwrap();
        let valid_file = skills_dir.join("SKILL.md");
        std::fs::write(&valid_file, "# Sample").unwrap();

        // Ignored dot directory
        let dot_dir = root.join(".git");
        std::fs::create_dir_all(&dot_dir).unwrap();
        std::fs::write(dot_dir.join("config"), "git config").unwrap();

        // Ignored node_modules
        let node_dir = root.join("node_modules");
        std::fs::create_dir_all(&node_dir).unwrap();
        std::fs::write(node_dir.join("package.json"), "{}").unwrap();

        let mut snapshot = HashMap::new();
        collect_snapshot_recursive(root, root, &mut snapshot);

        assert!(snapshot.contains_key(&valid_file));
        assert!(!snapshot.contains_key(&dot_dir.join("config")));
        assert!(!snapshot.contains_key(&node_dir.join("package.json")));

        // Non-existent directory handling
        let mut empty_snapshot = HashMap::new();
        collect_snapshot_recursive(&root.join("nonexistent"), root, &mut empty_snapshot);
        assert!(empty_snapshot.is_empty());
    }

    #[test]
    fn test_check_lint_at_detects_content_and_well_known_drift_without_mutating_workspace() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let git = |args: &[&str]| {
            let status = Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .expect("git command failed to spawn");
            assert!(status.success(), "git {args:?} failed");
        };

        git(&["init"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test User"]);

        let skill_dir = root.join("skills/engineering/demo-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: Initial description\n---\n# demo-skill\n\nInitial body.\n",
        )
        .unwrap();
        std::fs::create_dir_all(root.join("agents")).unwrap();
        std::fs::create_dir_all(root.join("dashboard/content")).unwrap();
        std::fs::write(root.join("dashboard/content/_index.md"), "committed page\n").unwrap();

        ArtifactsEngine::with_options(
            root,
            ArtifactsOptions {
                no_stage: true,
                ..Default::default()
            },
        )
        .generate_all()
        .unwrap();

        git(&["add", "."]);
        git(&["commit", "-m", "chore: initial fixture"]);

        let generated_skill_page =
            root.join("dashboard/content/skills/engineering/demo-skill/_index.md");
        let original_skill_page = std::fs::read_to_string(&generated_skill_page).unwrap();

        // Modify source skill without regenerating dashboard artifacts.
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: Updated description\n---\n# demo-skill\n\nUpdated body.\n",
        )
        .unwrap();

        let err = check_lint_at(root).expect_err("stale dashboard artifacts should fail lint");
        assert!(matches!(err, CliError::Subprocess { code: Some(1), .. }));

        // Workspace dashboard files must remain untouched by check_lint_at.
        assert_eq!(
            std::fs::read_to_string(&generated_skill_page).unwrap(),
            original_skill_page
        );
    }

    #[test]
    fn test_compare_generated_trees_reports_missing_extra_and_modified_files() {
        let staged_temp = tempdir().expect("failed to create staged temp dir");
        let workspace_temp = tempdir().expect("failed to create workspace temp dir");
        let staged = staged_temp.path();
        let workspace = workspace_temp.path();

        let staged_skill_dir = staged.join("dashboard/content/skills/engineering/alpha");
        let ws_skill_dir = workspace.join("dashboard/content/skills/engineering/alpha");
        std::fs::create_dir_all(&staged_skill_dir).unwrap();
        std::fs::create_dir_all(&ws_skill_dir).unwrap();

        // Missing in workspace (" D ")
        std::fs::write(staged_skill_dir.join("missing.md"), "staged only\n").unwrap();

        // Extra in workspace ("?? ")
        std::fs::write(ws_skill_dir.join("extra.md"), "workspace only\n").unwrap();

        // Modified between staged and workspace (" M ")
        std::fs::write(staged_skill_dir.join("_index.md"), "fresh content\n").unwrap();
        std::fs::write(ws_skill_dir.join("_index.md"), "stale content\n").unwrap();

        // Matching file (not reported)
        let staged_wk = staged
            .join("dashboard/static")
            .join(WELL_KNOWN_AGENT_SKILLS_DIR);
        let ws_wk = workspace
            .join("dashboard/static")
            .join(WELL_KNOWN_AGENT_SKILLS_DIR);
        std::fs::create_dir_all(&staged_wk).unwrap();
        std::fs::create_dir_all(&ws_wk).unwrap();
        std::fs::write(staged_wk.join("index.json"), "{}\n").unwrap();
        std::fs::write(ws_wk.join("index.json"), "{}\n").unwrap();

        // Hand-authored dashboard/content/_index.md outside skills/ is ignored by compare_generated_trees.
        std::fs::write(
            workspace.join("dashboard/content/_index.md"),
            "hand authored\n",
        )
        .unwrap();

        let drifted = compare_generated_trees(staged, workspace).unwrap();
        assert_eq!(
            drifted,
            vec![
                " M dashboard/content/skills/engineering/alpha/_index.md".to_string(),
                "?? dashboard/content/skills/engineering/alpha/extra.md".to_string(),
                " D dashboard/content/skills/engineering/alpha/missing.md".to_string(),
            ]
        );
    }

    #[test]
    fn test_run_live_reload_step_reports_ok_and_err() {
        assert!(run_live_reload_step::<_, &str>(Ok(1), "sync artifacts"));
        assert!(!run_live_reload_step::<(), _>(
            Err("boom"),
            "sync artifacts"
        ));
    }

    #[test]
    fn test_sync_live_reload_inputs_succeeds_and_fails_hermetically() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();

        let git = |args: &[&str]| {
            let status = Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .expect("git command failed to spawn");
            assert!(status.success(), "git {args:?} failed");
        };

        git(&["init"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test User"]);

        let skill_dir = root.join("skills/engineering/demo-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::create_dir_all(root.join("skills-archive")).unwrap();
        std::fs::create_dir_all(root.join("agents")).unwrap();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::create_dir_all(root.join("dashboard/content/skills")).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: Demo skill description\n---\n# demo-skill\n\nBody.\n",
        )
        .unwrap();

        assert!(sync_live_reload_inputs(root));
        let sidecar = root.join("dashboard/skill_dates.toml");
        assert!(sidecar.is_file());
        assert!(
            root.join("dashboard/static")
                .join(WELL_KNOWN_AGENT_SKILLS_DIR)
                .join("index.json")
                .is_file()
        );

        std::fs::remove_file(&sidecar).unwrap();
        std::fs::create_dir_all(&sidecar).unwrap();
        assert!(!sync_live_reload_inputs(root));

        let broken_temp = tempdir().expect("failed to create temp dir");
        let bad_root = broken_temp.path();
        std::fs::create_dir_all(bad_root.join("README.md")).unwrap();
        assert!(!sync_live_reload_inputs(bad_root));
        assert!(!bad_root.join("dashboard/skill_dates.toml").exists());
    }

    #[test]
    fn test_rebuild_search_indexes_short_circuits_on_sync_failure() {
        let temp = tempdir().expect("failed to create temp dir");
        let root = temp.path();
        std::fs::create_dir_all(root.join("README.md")).unwrap();

        rebuild_search_indexes(root);

        assert!(!root.join("dashboard/skill_dates.toml").exists());
        assert!(!root.join("dashboard/public").exists());
    }
}
