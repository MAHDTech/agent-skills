//! Remote skill downloader, registry client, resource sanitizer, and directory manager.
// cspell:words agentskills rsplit

use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, StatusCode};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use typed_builder::TypedBuilder;

use crate::error::{Result, SkillError};
use crate::models::{
    ResourceFile, ResourceKind, Skill, SkillTree, SkillsDiscoveryEntry, SkillsDiscoveryIndex,
    SkillsManifest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpandedResourceTarget {
    url: String,
    expected_digest: Option<String>,
    skill_prefix: Option<String>,
    is_top_level: bool,
}

/// Default request timeout for downloads in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 60;
/// Default maximum retry attempts for transient HTTP errors.
pub const DEFAULT_MAX_RETRIES: usize = 3;
/// Default User-Agent header for HTTP requests.
pub const DEFAULT_USER_AGENT: &str = "agent-skills-downloader/0.1.0 (Rust; Linux)";

/// Asynchronous client and directory manager for remote skill resources.
#[derive(Debug, Clone, TypedBuilder)]
pub struct SkillDownloader {
    /// Underlying HTTP client.
    #[builder(default = default_client())]
    pub client: Client,

    /// Request timeout duration.
    #[builder(default = Duration::from_secs(DEFAULT_TIMEOUT_SECS))]
    pub timeout: Duration,

    /// Maximum retry attempts for transient server errors (429, 500, 502, 503, 504).
    #[builder(default = DEFAULT_MAX_RETRIES)]
    pub max_retries: usize,

    /// Optional local cache directory path.
    #[builder(default, setter(into))]
    pub cache_dir: Option<PathBuf>,
}

fn default_client() -> Client {
    let mut headers = HeaderMap::new();
    if let Ok(val) = HeaderValue::from_str(DEFAULT_USER_AGENT) {
        headers.insert(USER_AGENT, val);
    }
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/markdown, text/plain, text/html, application/json, */*"),
    );

    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        .build()
        .unwrap_or_default()
}

impl Default for SkillDownloader {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl SkillDownloader {
    /// Creates a new `SkillDownloader` with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetches the raw string content of a URL with exponential backoff retries.
    #[allow(tail_expr_drop_order)]
    pub async fn fetch_url(&self, url: &str) -> Result<String> {
        let mut attempts = 0;
        let mut delay = Duration::from_millis(500);

        loop {
            attempts += 1;
            match self.client.get(url).send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        let text = response.text().await.map_err(|e| SkillError::Network {
                            url: url.to_string(),
                            message: format!("Failed to read response body: {e}"),
                        })?;
                        return Ok(text);
                    }

                    if (status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error())
                        && attempts <= self.max_retries
                    {
                        tokio::time::sleep(delay).await;
                        delay *= 2;
                        continue;
                    }

                    return Err(SkillError::Network {
                        url: url.to_string(),
                        message: format!("HTTP error: status {status}"),
                    });
                }
                Err(err) => {
                    if attempts <= self.max_retries {
                        tokio::time::sleep(delay).await;
                        delay *= 2;
                        continue;
                    }
                    return Err(SkillError::Network {
                        url: url.to_string(),
                        message: format!("Request failed after {attempts} attempts: {err}"),
                    });
                }
            }
        }
    }

    /// Fetches and deserializes a remote `skills.sh.json` registry manifest.
    pub async fn fetch_registry(&self, registry_url: &str) -> Result<SkillsManifest> {
        let content = self.fetch_url(registry_url).await?;
        serde_json::from_str::<SkillsManifest>(&content).map_err(SkillError::GeneralJson)
    }

    /// Fetches and deserializes a remote `.well-known/skills/index.json` or
    /// `.well-known/agent-skills/index.json` discovery manifest.
    pub async fn fetch_discovery_index(&self, index_url: &str) -> Result<SkillsDiscoveryIndex> {
        let content = self.fetch_url(index_url).await?;
        serde_json::from_str::<SkillsDiscoveryIndex>(&content).map_err(SkillError::GeneralJson)
    }

    /// Downloads and synchronizes all configured `resources` for a single skill.
    ///
    /// Saves downloaded assets exclusively to `<skill_dir>/resources/auto/` and prunes
    /// any unreferenced orphaned files in `resources/auto/`.
    /// Never modifies or deletes any files in `resources/manual/`.
    pub async fn download_skill_resources(&self, skill: &Skill) -> Result<Vec<ResourceFile>> {
        if !Self::is_managed_resource_skill(skill) {
            return Ok(Vec::new());
        }
        let skill_dir = skill.path.parent().unwrap_or_else(|| Path::new("."));
        let auto_dir = skill_dir.join("resources").join("auto");

        let resource_urls = match skill.frontmatter.resources {
            Some(ref list) if !list.is_empty() => list,
            _ => {
                // If skill has no resources configured, self-heal by cleaning auto resources
                let _ = self.clean_auto_resources(skill_dir);
                return Ok(Vec::new());
            }
        };

        fs::create_dir_all(&auto_dir).map_err(|e| SkillError::io(&auto_dir, e))?;

        let all_targets = self.expand_resource_targets(resource_urls).await?;
        let mut downloaded_files: Vec<ResourceFile> = Vec::new();
        let mut saved_filenames: HashSet<String> = HashSet::new();

        for target in &all_targets {
            let content = match self.fetch_url(&target.url).await {
                Ok(c) => c,
                Err(_) if all_targets.len() > 1 && !target.is_top_level => continue,
                Err(err) => return Err(err),
            };
            if let Some(ref expected_digest) = target.expected_digest
                && !Self::verify_sha256(content.as_bytes(), expected_digest)
            {
                return Err(SkillError::Network {
                    url: target.url.clone(),
                    message: format!(
                        "SHA-256 digest mismatch for '{}': expected {expected_digest}",
                        target.url
                    ),
                });
            }
            let resource_file = Self::save_downloaded_resource(
                &target.url,
                &content,
                &auto_dir,
                &mut saved_filenames,
                target.skill_prefix.as_deref(),
            )?;
            downloaded_files.push(resource_file);
        }

        Self::prune_orphaned_auto_files(&auto_dir, &saved_filenames);

        Ok(downloaded_files)
    }

    async fn expand_resource_targets(
        &self,
        resource_urls: &[String],
    ) -> Result<Vec<ExpandedResourceTarget>> {
        let mut all_targets: Vec<ExpandedResourceTarget> = Vec::new();
        for url in resource_urls {
            let clean_url = Self::strip_reader_proxy(url);
            if Self::is_non_text_url(clean_url) {
                continue;
            }

            if Self::is_skills_discovery_index_url(clean_url) {
                let index = self.fetch_discovery_index(clean_url).await?;
                for entry_target in Self::expand_discovery_index_entries(&index, clean_url) {
                    if !all_targets.iter().any(|t| t.url == entry_target.url) {
                        all_targets.push(entry_target);
                    }
                }
            } else if Self::is_llms_index_url(clean_url) {
                if let Ok(index_content) = self.fetch_url(clean_url).await {
                    Self::upsert_top_level_target(&mut all_targets, clean_url);
                    for child in Self::parse_llms_txt_links(&index_content, clean_url) {
                        if Self::is_skills_discovery_index_url(&child) {
                            if let Ok(index) = self.fetch_discovery_index(&child).await {
                                for entry_target in
                                    Self::expand_discovery_index_entries(&index, &child)
                                {
                                    if !all_targets.iter().any(|t| t.url == entry_target.url) {
                                        all_targets.push(entry_target);
                                    }
                                }
                            }
                        } else if !all_targets.iter().any(|t| t.url == child) {
                            all_targets.push(ExpandedResourceTarget {
                                url: child,
                                expected_digest: None,
                                skill_prefix: None,
                                is_top_level: false,
                            });
                        }
                    }
                }
            } else {
                Self::upsert_top_level_target(&mut all_targets, clean_url);
            }
        }
        Ok(all_targets)
    }

    fn is_managed_resource_skill(skill: &Skill) -> bool {
        skill.tree == SkillTree::Live && skill.vendor_source().is_none()
    }

    fn upsert_top_level_target(targets: &mut Vec<ExpandedResourceTarget>, url: &str) {
        if let Some(existing) = targets.iter_mut().find(|t| t.url == url) {
            existing.is_top_level = true;
        } else {
            targets.push(ExpandedResourceTarget {
                url: url.to_string(),
                expected_digest: None,
                skill_prefix: None,
                is_top_level: true,
            });
        }
    }

    fn expand_discovery_index_entries(
        index: &SkillsDiscoveryIndex,
        index_url: &str,
    ) -> Vec<ExpandedResourceTarget> {
        let no_query = index_url.split('?').next().unwrap_or(index_url);
        let clean_index = no_query.split('#').next().unwrap_or(no_query);
        let discovery_root = clean_index
            .rsplit_once('/')
            .map_or(clean_index, |(parent, _)| parent);

        let mut targets = Vec::new();
        for entry in &index.skills {
            targets.extend(Self::expand_discovery_entry(
                entry,
                clean_index,
                discovery_root,
            ));
        }
        targets
    }

    fn expand_discovery_entry(
        entry: &SkillsDiscoveryEntry,
        clean_index: &str,
        discovery_root: &str,
    ) -> Vec<ExpandedResourceTarget> {
        if let Some(ref entry_type) = entry.entry_type
            && !entry_type.trim().eq_ignore_ascii_case("skill-md")
        {
            return Vec::new();
        }

        let skill_name = entry.name.trim();
        if skill_name.is_empty()
            || skill_name.contains('/')
            || skill_name.contains('\\')
            || skill_name.contains("..")
        {
            return Vec::new();
        }

        let mut targets = Vec::new();
        let mut entry_urls = Vec::new();

        if let Some(ref raw_url) = entry.url {
            let trimmed_url = raw_url.trim();
            if !trimmed_url.is_empty() && !has_parent_dir_segment(trimmed_url) {
                let resolved_url = Self::resolve_relative_url(clean_index, trimmed_url);
                if !Self::is_non_text_url(&resolved_url) {
                    entry_urls.push(resolved_url.clone());
                    targets.push(ExpandedResourceTarget {
                        url: resolved_url,
                        expected_digest: entry.digest.clone(),
                        skill_prefix: Some(skill_name.to_string()),
                        is_top_level: false,
                    });
                }
            }
        }

        let entry_base = format!("{discovery_root}/{skill_name}");
        for file_rel in &entry.files {
            let clean_rel = file_rel.trim().trim_start_matches('/');
            if clean_rel.is_empty() || has_parent_dir_segment(clean_rel) {
                continue;
            }
            let file_url = format!("{entry_base}/{clean_rel}");
            if Self::is_non_text_url(&file_url) || entry_urls.contains(&file_url) {
                continue;
            }
            let expected_digest = if entry.url.is_none()
                && (entry.files.len() == 1 || clean_rel.eq_ignore_ascii_case("SKILL.md"))
            {
                entry.digest.clone()
            } else {
                None
            };
            entry_urls.push(file_url.clone());
            targets.push(ExpandedResourceTarget {
                url: file_url,
                expected_digest,
                skill_prefix: Some(skill_name.to_string()),
                is_top_level: false,
            });
        }

        if entry.url.is_none() && entry.files.is_empty() {
            let fallback_url = format!("{entry_base}/SKILL.md");
            if !Self::is_non_text_url(&fallback_url) && !entry_urls.contains(&fallback_url) {
                targets.push(ExpandedResourceTarget {
                    url: fallback_url,
                    expected_digest: entry.digest.clone(),
                    skill_prefix: Some(skill_name.to_string()),
                    is_top_level: false,
                });
            }
        }

        targets
    }

    fn save_downloaded_resource(
        url: &str,
        content: &str,
        auto_dir: &Path,
        saved_filenames: &mut HashSet<String>,
        skill_prefix: Option<&str>,
    ) -> Result<ResourceFile> {
        let sanitized = Self::sanitize_markdown(content);
        let mut raw_slug = Self::smart_slugify(url, None);
        if let Some(skill_name) = skill_prefix {
            let clean_skill: String = skill_name
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .map(|c| c.to_ascii_lowercase())
                .collect();
            let clean_skill = clean_skill.trim_matches(['-', '.']);
            if !clean_skill.is_empty() && !raw_slug.contains(clean_skill) {
                raw_slug = format!("{clean_skill}-{raw_slug}");
            }
        }
        let final_filename = if saved_filenames.contains(&raw_slug) {
            let domain_candidate = format!("{}-{raw_slug}", Self::get_domain_prefix(url));
            if !saved_filenames.contains(&domain_candidate) {
                domain_candidate
            } else {
                let (stem, ext) = domain_candidate
                    .rsplit_once('.')
                    .map_or((domain_candidate.as_str(), ""), |(s, e)| (s, e));
                let mut counter = 2usize;
                loop {
                    let candidate = if ext.is_empty() {
                        format!("{stem}-{counter}")
                    } else {
                        format!("{stem}-{counter}.{ext}")
                    };
                    if !saved_filenames.contains(&candidate) {
                        break candidate;
                    }
                    counter += 1;
                }
            }
        } else {
            raw_slug
        };

        let dest_path = Self::check_path_traversal(auto_dir, &final_filename)?;
        let mut file = File::create(&dest_path).map_err(|e| SkillError::io(&dest_path, e))?;
        file.write_all(sanitized.as_bytes())
            .map_err(|e| SkillError::io(&dest_path, e))?;

        saved_filenames.insert(final_filename.clone());

        let rel_path = PathBuf::from("resources/auto").join(&final_filename);
        Ok(ResourceFile::builder()
            .relative_path(rel_path)
            .absolute_path(dest_path)
            .kind(ResourceKind::Auto)
            .content(Some(sanitized))
            .build())
    }

    fn prune_orphaned_auto_files(auto_dir: &Path, saved_filenames: &HashSet<String>) {
        let Ok(entries) = fs::read_dir(auto_dir) else {
            return;
        };
        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let filename = entry.file_name().to_string_lossy().to_string();

            if file_type.is_file()
                && !filename.starts_with('.')
                && !saved_filenames.contains(&filename)
            {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    /// Downloads resources across a collection of skills.
    pub async fn download_all_resources(
        &self,
        skills: &[Skill],
    ) -> Result<HashMap<String, Vec<ResourceFile>>> {
        let mut results = HashMap::new();
        for skill in skills {
            if !Self::is_managed_resource_skill(skill) {
                continue;
            }
            let files = self.download_skill_resources(skill).await?;
            results.insert(skill.name().to_string(), files);
        }
        Ok(results)
    }

    /// Cleans `<skill_dir>/resources/auto/` directory.
    ///
    /// Never touches `<skill_dir>/resources/manual/`.
    /// Returns 1 if directory was removed, 0 if it did not exist.
    pub fn clean_auto_resources(&self, skill_dir: &Path) -> Result<usize> {
        let auto_dir = skill_dir.join("resources").join("auto");
        if auto_dir.exists() {
            fs::remove_dir_all(&auto_dir).map_err(|e| SkillError::io(&auto_dir, e))?;
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Cleans auto resources across all given skills.
    pub fn clean_all_resources(&self, skills: &[Skill]) -> Result<usize> {
        let mut count = 0;
        for skill in skills {
            if !Self::is_managed_resource_skill(skill) {
                continue;
            }
            if let Some(parent) = skill.path.parent() {
                count += self.clean_auto_resources(parent)?;
            }
        }
        Ok(count)
    }

    // ------------------------------------------------------------------------
    // Algorithms & Utility Functions
    // ------------------------------------------------------------------------

    /// Strips reader proxy prefixes such as `https://r.jina.ai/`.
    #[must_use]
    pub fn strip_reader_proxy(url_str: &str) -> &str {
        if let Some(stripped) = url_str.strip_prefix("https://r.jina.ai/") {
            stripped
        } else if let Some(stripped) = url_str.strip_prefix("http://r.jina.ai/") {
            stripped
        } else {
            url_str
        }
    }

    /// Strips Jina Reader metadata header preambles.
    #[must_use]
    pub fn strip_reader_metadata(content: &str) -> &str {
        if content.starts_with("Title:")
            && let Some(idx) = content.find("\nMarkdown Content:\n")
        {
            return content[idx + "\nMarkdown Content:\n".len()..].trim_start();
        }
        content
    }

    /// Checks if a URL points to a non-text binary or media file.
    #[must_use]
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_non_text_url(url_str: &str) -> bool {
        let clean = url_str.split('?').next().unwrap_or(url_str);
        let lower = clean.to_ascii_lowercase();

        let binary_extensions = [
            ".zip", ".pdf", ".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg", ".ico", ".bmp",
            ".avif", ".tar", ".gz", ".tgz", ".bz2", ".xz", ".exe", ".dmg", ".iso", ".bin", ".mp4",
            ".mp3", ".wav", ".ogg", ".webm", ".mov", ".avi", ".flv", ".m4a", ".m4v", ".woff",
            ".woff2", ".ttf", ".eot", ".otf",
        ];

        binary_extensions.iter().any(|ext| lower.ends_with(ext))
    }

    /// Checks if a MIME content-type is non-text.
    #[must_use]
    pub fn is_non_text_content_type(content_type: &str) -> bool {
        const PREFIXES: &[&str] = &["image/", "audio/", "video/", "font/"];
        const CONTAINS: &[&str] = &[
            "application/pdf",
            "application/zip",
            "application/octet-stream",
            "application/x-tar",
            "application/gzip",
        ];
        let lower = content_type.to_ascii_lowercase();
        PREFIXES.iter().any(|p| lower.starts_with(p)) || CONTAINS.iter().any(|c| lower.contains(c))
    }

    /// Checks if a URL targets an `llms.txt` link-index file (excluding consolidated `llms-full` bundles).
    #[must_use]
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_llms_index_url(url_str: &str) -> bool {
        let no_query = url_str.split('?').next().unwrap_or(url_str);
        let clean = no_query.split('#').next().unwrap_or(no_query);
        let lower = clean.to_ascii_lowercase();
        if lower.contains("llms-full") || lower.contains("llms_full") {
            return false;
        }
        lower.ends_with("/llms.txt")
            || lower.ends_with("/_llms.txt")
            || (lower.contains("llms") && (lower.ends_with(".txt") || lower.ends_with(".md")))
    }

    /// Checks if a URL targets an Agent Skills Discovery index (`.well-known/skills/index.json`
    /// or `.well-known/agent-skills/index.json`).
    #[must_use]
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_skills_discovery_index_url(url_str: &str) -> bool {
        let no_query = url_str.split('?').next().unwrap_or(url_str);
        let clean = no_query.split('#').next().unwrap_or(no_query);
        let lower = clean.to_ascii_lowercase();
        lower.ends_with("/.well-known/skills/index.json")
            || lower.ends_with("/.well-known/agent-skills/index.json")
            || lower == ".well-known/skills/index.json"
            || lower == ".well-known/agent-skills/index.json"
    }

    /// Extracts a domain brand prefix from a URL.
    #[must_use]
    pub fn get_domain_prefix(url_str: &str) -> String {
        let clean = Self::strip_reader_proxy(url_str);
        let host = clean
            .split("://")
            .nth(1)
            .unwrap_or(clean)
            .split('/')
            .next()
            .unwrap_or("")
            .split(':')
            .next()
            .unwrap_or("");

        if host.is_empty() {
            return "doc".to_string();
        }

        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() <= 1 {
            return host.to_string();
        }

        // Handle common multi-part TLDs (co.uk, com.au, org.nz, etc.)
        if parts.len() >= 3 && (parts[parts.len() - 2] == "co" || parts[parts.len() - 2] == "com") {
            parts[parts.len() - 3].to_string()
        } else {
            parts[parts.len() - 2].to_string()
        }
    }

    /// Converts an arbitrary URL into a clean, safe markdown file slug.
    #[must_use]
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn smart_slugify(url_str: &str, common_prefix: Option<&str>) -> String {
        let clean = Self::strip_reader_proxy(url_str);
        let no_query = clean.split('?').next().unwrap_or(clean);
        let no_hash = no_query.split('#').next().unwrap_or(no_query);

        let path_part = if let Some(prefix) = common_prefix {
            no_hash.strip_prefix(prefix).unwrap_or(no_hash)
        } else if let Some(idx) = no_hash.find("://") {
            let after_scheme = &no_hash[idx + 3..];
            after_scheme.split_once('/').map_or("", |(_, p)| p)
        } else {
            no_hash
        };

        let trimmed_path = path_part.trim_matches('/');
        if trimmed_path.is_empty()
            || trimmed_path == "index"
            || trimmed_path == "_index"
            || trimmed_path == "overview"
        {
            return "overview-index.md".to_string();
        }

        let (base, ext) = if let Some(stripped) = trimmed_path.strip_suffix(".txt") {
            (stripped, ".txt")
        } else if let Some(stripped) = trimmed_path.strip_suffix(".md") {
            (stripped, ".md")
        } else if let Some(stripped) = trimmed_path.strip_suffix(".html") {
            (stripped, ".md")
        } else {
            (trimmed_path, ".md")
        };

        let mut slug = String::with_capacity(base.len() + 4);
        for c in base.chars() {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                slug.push(c.to_ascii_lowercase());
            } else if c == '/' {
                slug.push('-');
            }
        }

        let final_slug = slug.trim_matches(['-', '.']);
        if final_slug.is_empty() || final_slug == "index" || final_slug == "_index" {
            format!("overview-index{ext}")
        } else {
            format!("{final_slug}{ext}")
        }
    }

    /// Resolves and validates target path to prevent directory traversal outside base directory.
    pub fn check_path_traversal(base_dir: &Path, filename: &str) -> Result<PathBuf> {
        let combined = base_dir.join(filename);
        let mut normalized = PathBuf::new();

        for comp in combined.components() {
            match comp {
                Component::Normal(c) => normalized.push(c),
                Component::RootDir => normalized.push(std::path::MAIN_SEPARATOR.to_string()),
                Component::Prefix(p) => normalized.push(p.as_os_str()),
                Component::CurDir => {}
                Component::ParentDir => {
                    if !normalized.pop() {
                        return Err(SkillError::PathTraversal {
                            path: combined,
                            base: base_dir.to_path_buf(),
                        });
                    }
                }
            }
        }

        if !normalized.starts_with(base_dir) {
            return Err(SkillError::PathTraversal {
                path: combined,
                base: base_dir.to_path_buf(),
            });
        }

        Ok(normalized)
    }

    /// Parses links from an `llms.txt` document, extracting both markdown `[text](url)` links
    /// and bare `http://` / `https://` URLs per line.
    #[must_use]
    pub fn parse_llms_txt_links(content: &str, base_url: &str) -> Vec<String> {
        let mut links = Vec::new();
        let base_domain = Self::get_domain_prefix(base_url);

        for line in content.split('\n') {
            let mut remaining = line.trim();
            let mut non_markdown_text = String::with_capacity(remaining.len());

            while let Some(start_bracket) = remaining.find('[') {
                non_markdown_text.push_str(&remaining[..start_bracket]);
                let after_open = &remaining[start_bracket + 1..];
                if let Some(close_bracket) = after_open.find(']') {
                    let after_close = &after_open[close_bracket + 1..];
                    if let Some(after_paren_open) = after_close.strip_prefix('(')
                        && let Some(close_paren) = after_paren_open.find(')')
                    {
                        let raw_link = after_paren_open[..close_paren].trim();
                        Self::push_resolved_llms_link(&mut links, raw_link, base_url, &base_domain);
                        non_markdown_text.push(' ');
                        remaining = &after_paren_open[close_paren + 1..];
                        continue;
                    }
                    non_markdown_text.push(' ');
                    remaining = after_open;
                } else {
                    non_markdown_text.push_str(remaining);
                    remaining = "";
                }
            }
            non_markdown_text.push_str(remaining);

            for token in non_markdown_text.split_whitespace() {
                let Some(idx) = token.find("https://").or_else(|| token.find("http://")) else {
                    continue;
                };
                let maybe_url = &token[idx..];
                let end_idx = maybe_url
                    .find([')', ']', '>', '"', '\''])
                    .unwrap_or(maybe_url.len());
                let candidate = maybe_url[..end_idx].trim_end_matches(['.', ',', ';', ':']);
                Self::push_resolved_llms_link(&mut links, candidate, base_url, &base_domain);
            }
        }

        links
    }

    fn push_resolved_llms_link(
        links: &mut Vec<String>,
        raw_link: &str,
        base_url: &str,
        base_domain: &str,
    ) {
        let clean_link = raw_link
            .trim()
            .trim_matches('<')
            .trim_matches('>')
            .trim_matches('"')
            .trim_matches('\'');

        if clean_link.is_empty() || clean_link.starts_with('#') || Self::is_non_text_url(clean_link)
        {
            return;
        }

        let resolved_raw = Self::resolve_relative_url(base_url, clean_link);
        let resolved = resolved_raw
            .split('#')
            .next()
            .unwrap_or(&resolved_raw)
            .to_string();
        let clean_base = base_url.split('#').next().unwrap_or(base_url);
        let link_domain = Self::get_domain_prefix(&resolved);
        if !resolved.is_empty()
            && link_domain == base_domain
            && resolved != clean_base
            && !links.contains(&resolved)
        {
            links.push(resolved);
        }
    }

    fn resolve_relative_url(base_url: &str, clean_link: &str) -> String {
        if clean_link.starts_with("http://") || clean_link.starts_with("https://") {
            clean_link.to_string()
        } else if clean_link.starts_with('/') {
            let origin = base_url
                .split("://")
                .nth(1)
                .unwrap_or(base_url)
                .split('/')
                .next()
                .unwrap_or("");
            let scheme = if base_url.starts_with("http://") {
                "http://"
            } else {
                "https://"
            };
            format!("{scheme}{origin}{clean_link}")
        } else {
            let url_parts = base_url.split('/').collect::<Vec<_>>();
            let parent = if url_parts.len() > 1 {
                url_parts[..url_parts.len() - 1].join("/")
            } else {
                base_url.to_string()
            };
            format!("{parent}/{clean_link}")
        }
    }

    /// Strips non-printable control characters and Unicode replacement chars.
    #[must_use]
    pub fn sanitize_control_characters(content: &str) -> String {
        content
            .chars()
            .filter(|&c| {
                if c == '\n' || c == '\r' || c == '\t' {
                    true
                } else {
                    let u = c as u32;
                    u >= 0x20 && c != char::REPLACEMENT_CHARACTER
                }
            })
            .collect()
    }

    /// Decodes common HTML entities into plain characters.
    #[must_use]
    pub fn decode_html_entities(content: &str) -> String {
        content
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&#x20;", " ")
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
    }

    /// Escapes orphan generic type brackets outside code blocks (e.g. `<T>` -> `\<T\>`).
    #[must_use]
    pub fn escape_orphan_type_brackets(content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_fence = false;

        for line in content.split('\n') {
            let trimmed = line.trim();
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_fence = !in_fence;
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if in_fence {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            let mut remaining = line;
            while let Some(open_idx) = remaining.find('<') {
                result.push_str(&remaining[..open_idx]);
                let after_open = &remaining[open_idx + 1..];

                if let Some(close_idx) = after_open.find('>') {
                    let inner = &after_open[..close_idx];
                    let is_standard_tag = inner == "div"
                        || inner == "/div"
                        || inner == "p"
                        || inner == "/p"
                        || inner == "a"
                        || inner == "/a"
                        || inner == "span"
                        || inner == "/span"
                        || inner == "code"
                        || inner == "/code"
                        || inner == "pre"
                        || inner == "/pre"
                        || inner.starts_with("http://")
                        || inner.starts_with("https://");

                    let is_type_param = !is_standard_param_tag(inner)
                        && !is_standard_tag
                        && inner.chars().all(|c| {
                            c.is_ascii_alphanumeric()
                                || c == ','
                                || c == ' '
                                || c == '_'
                                || c == '\''
                                || c == ':'
                        });

                    if is_type_param && !inner.is_empty() {
                        result.push_str("\\<");
                        result.push_str(inner);
                        result.push_str("\\>");
                    } else {
                        result.push('<');
                        result.push_str(inner);
                        result.push('>');
                    }
                    remaining = &after_open[close_idx + 1..];
                } else {
                    result.push('<');
                    remaining = after_open;
                }
            }
            result.push_str(remaining);
            result.push('\n');
        }

        if !content.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        result
    }

    /// Normalizes empty markdown link placeholders `[text]()` -> `text`.
    #[must_use]
    pub fn normalize_markdown_formatting(content: &str) -> String {
        content
            .replace("[]()", "")
            .replace("[]( # )", "")
            .replace("[](#)", "")
    }

    /// Full sanitization pipeline for downloaded markdown.
    #[must_use]
    pub fn sanitize_markdown(content: &str) -> String {
        let no_jina = Self::strip_reader_metadata(content);
        let decoded = Self::decode_html_entities(no_jina);
        let no_ctrl = Self::sanitize_control_characters(&decoded);
        let escaped = Self::escape_orphan_type_brackets(&no_ctrl);
        Self::normalize_markdown_formatting(&escaped)
    }

    /// Computes hexadecimal SHA-256 digest of input bytes.
    #[must_use]
    pub fn compute_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        crate::hashing::encode_hex(&hasher.finalize())
    }

    /// Validates SHA-256 digest of input bytes, accepting bare hex or `sha256:<hex>`.
    #[must_use]
    pub fn verify_sha256(data: &[u8], expected_hex: &str) -> bool {
        let trimmed = expected_hex.trim();
        let expected = if trimmed.len() >= 7 && trimmed[..7].eq_ignore_ascii_case("sha256:") {
            trimmed[7..].trim()
        } else {
            trimmed
        };
        let actual = Self::compute_sha256(data);
        actual.eq_ignore_ascii_case(expected)
    }
}

fn has_parent_dir_segment(raw: &str) -> bool {
    let no_query = raw.split('?').next().unwrap_or(raw);
    let path_part = no_query.split('#').next().unwrap_or(no_query);
    path_part.split(['/', '\\']).any(|seg| seg == "..")
}

fn is_standard_param_tag(tag: &str) -> bool {
    let lower = tag.to_ascii_lowercase();
    lower.starts_with("img ")
        || lower.starts_with("a ")
        || lower.starts_with("div ")
        || lower.starts_with("span ")
        || lower.starts_with("table")
        || lower.starts_with("tr")
        || lower.starts_with("td")
        || lower.starts_with("th")
        || lower.starts_with('b')
        || lower.starts_with('i')
        || lower.starts_with("em")
        || lower.starts_with("strong")
}
