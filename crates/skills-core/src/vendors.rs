//! Declarative vendor skill snapshots acquired through Git.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Output};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::error::{Result, SkillError};

use crate::parser::SkillParser;

const MANIFEST: &str = "vendor-skills.yaml";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    sources: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    id: String,
    repository: String,
    #[serde(rename = "ref", default)]
    reference: Option<String>,
    #[serde(default = "default_skill_path")]
    path: PathBuf,
    #[serde(default, rename = "license-files")]
    license_files: Vec<PathBuf>,
}

fn default_skill_path() -> PathBuf {
    PathBuf::from("skills")
}

#[derive(Debug, Default, Serialize)]
pub struct VendorReport {
    pub fetched: BTreeMap<String, VendorSnapshot>,
    pub skipped: BTreeMap<String, String>,
    pub removed: Vec<String>,
    pub retained_pending: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct VendorSnapshot {
    pub repository: String,
    pub commit: String,
}

/// Regenerates successful sources, retaining existing snapshots when acquisition fails.
pub fn update_vendors(root: &Path, pending_ref: Option<&str>) -> Result<VendorReport> {
    let manifest_path = root.join(MANIFEST);
    if !manifest_path.exists() {
        return Ok(VendorReport::default());
    }
    let manifest: Manifest = serde_yaml::from_slice(&fs::read(&manifest_path)?)
        .map_err(|e| SkillError::yaml(&manifest_path, e))?;
    validate_manifest(&manifest, &manifest_path)?;
    let vendor_root = root.join("skills/vendor");
    ensure_directory(&root.join("skills"))?;
    ensure_directory(&vendor_root)?;
    for source in &manifest.sources {
        let destination = vendor_root.join(&source.id);
        if destination
            .symlink_metadata()
            .is_ok_and(|m| !m.is_dir() || m.file_type().is_symlink())
        {
            return Err(SkillError::validation(
                destination,
                "Expected a real vendor directory",
            ));
        }
    }
    let cache = root.join(".cache");
    ensure_directory(&cache)?;
    let staging = tempfile::tempdir_in(&cache)?;
    let mut prepared = BTreeMap::new();
    let mut report = VendorReport::default();
    for source in &manifest.sources {
        match prepare_source(source, staging.path()) {
            Ok((path, commit)) => {
                report.fetched.insert(
                    source.id.clone(),
                    VendorSnapshot {
                        repository: source.repository.clone(),
                        commit,
                    },
                );
                prepared.insert(source.id.clone(), path);
            }
            Err(error) => {
                report.skipped.insert(source.id.clone(), error.to_string());
                if let Some(reference) = pending_ref {
                    match pending_snapshot(root, reference, source, staging.path()) {
                        Ok(Some(path)) => {
                            prepared.insert(source.id.clone(), path);
                            report.retained_pending.push(source.id.clone());
                        }
                        Ok(None) => {}
                        Err(error) => {
                            if let Some(message) = report.skipped.get_mut(&source.id) {
                                message
                                    .push_str(&format!("; pending snapshot unavailable: {error}"));
                            }
                        }
                    }
                }
            }
        }
    }
    validate_catalog(root, &prepared, &manifest)?;
    for (id, path) in prepared {
        replace_directory(&path, &vendor_root.join(id), staging.path())?;
    }
    let declared: HashSet<_> = manifest.sources.iter().map(|s| s.id.as_str()).collect();
    for entry in fs::read_dir(&vendor_root)? {
        let entry = entry?;
        let id = entry.file_name().to_string_lossy().into_owned();
        if id.starts_with('.') || declared.contains(id.as_str()) {
            continue;
        }
        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(entry.path())?;
            report.removed.push(id);
        }
    }
    report.removed.sort();
    Ok(report)
}

fn pending_snapshot(
    root: &Path,
    reference: &str,
    source: &Source,
    staging: &Path,
) -> Result<Option<PathBuf>> {
    if reference.starts_with('-') {
        return Err(SkillError::validation(root, "Invalid pending Git ref"));
    }
    let previous_manifest = match git(root, &["show", &format!("{reference}:{MANIFEST}")]) {
        Ok(output) => serde_yaml::from_slice::<Manifest>(&output.stdout)?,
        Err(_) => return Ok(None),
    };
    if !previous_manifest.sources.iter().any(|s| s == source) {
        return Ok(None);
    }
    let merge_base =
        String::from_utf8_lossy(&git(root, &["merge-base", "HEAD", reference])?.stdout)
            .trim()
            .to_string();
    let directory = format!("skills/vendor/{}", source.id);
    // A base-branch edit takes precedence over stale bot-branch content.
    if git(
        root,
        &["diff", "--quiet", &merge_base, "HEAD", "--", &directory],
    )
    .is_err()
    {
        return Ok(None);
    }
    let listing = git(
        root,
        &["ls-tree", "-rz", "--full-tree", reference, "--", &directory],
    )?;
    if listing.stdout.is_empty() {
        return Ok(None);
    }
    let destination = staging.join("pending/skills/vendor").join(&source.id);
    fs::create_dir_all(&destination)?;
    for record in listing.stdout.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let record =
            std::str::from_utf8(record).map_err(|e| SkillError::validation(root, e.to_string()))?;
        let (metadata, path) = record
            .split_once('\t')
            .ok_or_else(|| SkillError::validation(root, "Invalid Git tree entry"))?;
        let fields: Vec<_> = metadata.split_whitespace().collect();
        if fields.len() != 3 || fields[1] != "blob" || !matches!(fields[0], "100644" | "100755") {
            return Err(SkillError::validation(
                path,
                "Unsupported pending snapshot entry",
            ));
        }
        let relative = Path::new(path)
            .strip_prefix(&directory)
            .map_err(|e| SkillError::validation(path, e.to_string()))?;
        validate_relative_path(relative)?;
        let target = destination.join(relative);
        fs::create_dir_all(target.parent().unwrap_or(&destination))?;
        fs::write(&target, git(root, &["cat-file", "blob", fields[2]])?.stdout)?;
        #[cfg(unix)]
        if fields[0] == "100755" {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target, fs::Permissions::from_mode(0o755))?;
        }
    }
    Ok(Some(destination))
}

fn validate_catalog(
    root: &Path,
    prepared: &BTreeMap<String, PathBuf>,
    manifest: &Manifest,
) -> Result<()> {
    let mut names = BTreeMap::new();
    let mut skills = Vec::new();
    let vendor_root = root.join("skills/vendor");
    for entry in WalkDir::new(root.join("skills"))
        .into_iter()
        .filter_entry(|e| e.path() != vendor_root)
    {
        let entry = entry.map_err(|e| SkillError::validation(root, e.to_string()))?;
        if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
            skills.push(SkillParser::parse_file(entry.path())?);
        }
    }
    for source in &manifest.sources {
        let existing = vendor_root.join(&source.id);
        if !prepared.contains_key(&source.id) && existing.is_dir() {
            skills.extend(SkillParser::discover_skills(&existing)?);
        }
    }
    for directory in prepared.values() {
        for entry in fs::read_dir(directory)? {
            skills.push(SkillParser::parse_file(entry?.path().join("SKILL.md"))?);
        }
    }
    for skill in skills {
        if !skill.promoted || skill.is_archived() {
            continue;
        }
        if let Some(previous) = names.insert(skill.name().to_string(), skill.path.clone()) {
            return Err(SkillError::DuplicateSkill {
                name: skill.name().to_string(),
                paths: vec![previous, skill.path],
            });
        }
    }
    Ok(())
}

fn validate_manifest(manifest: &Manifest, path: &Path) -> Result<()> {
    if manifest.version != 1 {
        return Err(SkillError::validation(
            path,
            "Unsupported vendor manifest version",
        ));
    }
    let mut ids = HashSet::new();
    for source in &manifest.sources {
        validate_identifier(&source.id)?;
        if !ids.insert(&source.id) {
            return Err(SkillError::validation(
                path,
                format!("Duplicate vendor id: {}", source.id),
            ));
        }
        validate_relative_path(&source.path)?;
        for license in &source.license_files {
            validate_relative_path(license)?;
        }
        let repository = &source.repository;
        if !(repository.starts_with("https://")
            || repository.starts_with("ssh://")
            || repository.starts_with("git@")
            || Path::new(repository).is_absolute())
        {
            return Err(SkillError::validation(
                path,
                "Expected an HTTPS/SSH Git URL or absolute local repository path",
            ));
        }
        if source
            .reference
            .as_ref()
            .is_some_and(|r| r.is_empty() || r.starts_with('-'))
        {
            return Err(SkillError::validation(path, "Invalid vendor Git ref"));
        }
    }
    Ok(())
}

fn validate_identifier(id: &str) -> Result<()> {
    if id.is_empty()
        || id.len() > 64
        || id.starts_with('-')
        || id.ends_with('-')
        || !id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    {
        return Err(SkillError::validation(
            id,
            "Expected a lowercase kebab-case identifier",
        ));
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|p| !matches!(p, Component::Normal(_) | Component::CurDir))
    {
        return Err(SkillError::validation(
            path,
            "Expected a contained relative path",
        ));
    }
    Ok(())
}

fn ensure_directory(path: &Path) -> Result<()> {
    if path
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        return Err(SkillError::validation(
            path,
            "Managed vendor directories cannot be symlinks",
        ));
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn git(root: &Path, args: &[&str]) -> Result<Output> {
    let mut command = Command::new("git");
    command
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "http.lowSpeedLimit=1",
            "-c",
            "http.lowSpeedTime=30",
        ])
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .current_dir(root);
    let configured_ssh = std::env::var_os("GIT_SSH_COMMAND").is_some()
        || std::env::var_os("GIT_SSH").is_some()
        || Command::new("git")
            .args(["config", "--get", "core.sshCommand"])
            .current_dir(root)
            .output()?
            .status
            .success();
    if !configured_ssh {
        command.env(
            "GIT_SSH_COMMAND",
            "ssh -o BatchMode=yes -o ConnectTimeout=20",
        );
    }
    let output = command.output()?;
    if !output.status.success() {
        return Err(SkillError::validation(
            root,
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    Ok(output)
}

fn prepare_source(source: &Source, staging: &Path) -> Result<(PathBuf, String)> {
    let checkout = staging.join(format!("{}-checkout", source.id));
    let mut args = vec![
        "clone",
        "--quiet",
        "--depth=1",
        "--filter=blob:none",
        "--sparse",
    ];
    if let Some(reference) = source.reference.as_deref() {
        args.extend(["--branch", reference]);
    }
    args.extend(["--", &source.repository]);
    let checkout_str = checkout.to_string_lossy();
    args.push(&checkout_str);
    git(staging, &args)?;
    if source.path != Path::new(".") {
        git(
            &checkout,
            &[
                "sparse-checkout",
                "set",
                "--",
                &source.path.to_string_lossy(),
            ],
        )?;
    } else {
        git(&checkout, &["sparse-checkout", "disable"])?;
    }
    let collection = checkout.join(&source.path);
    let canonical = collection.canonicalize()?;
    if !canonical.starts_with(checkout.canonicalize()?) {
        return Err(SkillError::validation(
            &collection,
            "Skill path escapes checkout",
        ));
    }
    let destination = staging.join("skills/vendor").join(&source.id);
    fs::create_dir_all(&destination)?;
    let mut count = 0;
    for entry in fs::read_dir(&collection)? {
        let entry = entry?;
        let skill = entry.path();
        if entry.file_name() == ".git" {
            continue;
        }
        if entry.file_type()?.is_symlink() {
            return Err(SkillError::validation(
                &skill,
                "Vendor symlinks are unsupported",
            ));
        }
        if !entry.file_type()?.is_dir() {
            continue;
        }
        if !skill.join("SKILL.md").is_file() {
            for nested in WalkDir::new(&skill).follow_links(false) {
                let nested = nested.map_err(|e| SkillError::validation(&skill, e.to_string()))?;
                if nested.file_name() == "SKILL.md" {
                    return Err(SkillError::validation(
                        nested.path(),
                        "Nested skill collections are unsupported; declare a flat skill collection path",
                    ));
                }
            }
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        validate_identifier(&name)?;
        let target = destination.join(&name);
        copy_tree(&skill, &target)?;
        SkillParser::parse_file(target.join("SKILL.md"))?;
        copy_licenses(source, &checkout, &target)?;
        count += 1;
    }
    if count == 0 {
        return Err(SkillError::validation(
            &collection,
            "No skill directories found",
        ));
    }
    let commit = String::from_utf8_lossy(&git(&checkout, &["rev-parse", "HEAD"])?.stdout)
        .trim()
        .to_string();
    Ok((destination, commit))
}

fn copy_tree(source: &Path, target: &Path) -> Result<()> {
    for entry in WalkDir::new(source).follow_links(false) {
        let entry = entry.map_err(|e| SkillError::validation(source, e.to_string()))?;
        let relative = entry
            .path()
            .strip_prefix(source)
            .map_err(|e| SkillError::validation(source, e.to_string()))?;
        let destination = target.join(relative);
        if entry.file_type().is_symlink() {
            return Err(SkillError::validation(
                entry.path(),
                "Vendor symlinks are unsupported",
            ));
        }
        if entry.file_type().is_dir() {
            fs::create_dir_all(&destination)?;
        } else if entry.file_type().is_file() {
            fs::copy(entry.path(), destination)?;
        }
    }
    Ok(())
}

fn copy_licenses(source: &Source, checkout: &Path, target: &Path) -> Result<()> {
    let licenses = if source.license_files.is_empty() {
        fs::read_dir(checkout)?
            .filter_map(std::result::Result::ok)
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                (name.starts_with("LICENSE")
                    || name.starts_with("NOTICE")
                    || name.starts_with("COPYING"))
                .then_some(PathBuf::from(name))
            })
            .collect()
    } else {
        source.license_files.clone()
    };
    for license in licenses {
        let name = license
            .file_name()
            .ok_or_else(|| SkillError::validation(&license, "Missing license filename"))?;
        let destination = target.join(name);
        if !destination.exists() {
            let text = git(checkout, &["show", &format!("HEAD:{}", license.display())])?.stdout;
            fs::write(destination, text)?;
        }
    }
    Ok(())
}

fn replace_directory(source: &Path, target: &Path, staging: &Path) -> Result<()> {
    if target
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        return Err(SkillError::validation(
            target,
            "Vendor destination cannot be a symlink",
        ));
    }
    let backup = staging.join("backup");
    let existed = target.exists();
    if existed {
        fs::rename(target, &backup)?;
    }
    if let Err(error) = fs::rename(source, target) {
        if existed {
            fs::rename(&backup, target)?;
        }
        return Err(SkillError::io(target, error));
    }
    if existed {
        fs::remove_dir_all(backup)?;
    }
    Ok(())
}
