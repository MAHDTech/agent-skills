//! Remote skill downloader, registry client, resource sanitizer, and directory manager.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::{Client, StatusCode};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use typed_builder::TypedBuilder;

use crate::error::{Result, SkillError};
use crate::models::{ResourceFile, ResourceKind, Skill, SkillsManifest};

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

    /// Downloads and synchronizes all configured `resources` for a single skill.
    ///
    /// Saves downloaded assets exclusively to `<skill_dir>/resources/auto/` and prunes
    /// any unreferenced orphaned files in `resources/auto/`.
    /// Never modifies or deletes any files in `resources/manual/`.
    pub async fn download_skill_resources(&self, skill: &Skill) -> Result<Vec<ResourceFile>> {
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

        let mut all_urls_to_fetch: Vec<String> = Vec::new();
        let mut downloaded_files: Vec<ResourceFile> = Vec::new();
        let mut saved_filenames: HashSet<String> = HashSet::new();

        // 1. Expand llms.txt index files if present
        for url in resource_urls {
            let clean_url = Self::strip_reader_proxy(url);
            if Self::is_non_text_url(clean_url) {
                continue;
            }

            if Self::is_llms_index_url(clean_url) {
                if let Ok(index_content) = self.fetch_url(clean_url).await {
                    let child_links = Self::parse_llms_txt_links(&index_content, clean_url);
                    all_urls_to_fetch.push(clean_url.to_string());
                    for child in child_links {
                        if !all_urls_to_fetch.contains(&child) {
                            all_urls_to_fetch.push(child);
                        }
                    }
                }
            } else {
                all_urls_to_fetch.push(clean_url.to_string());
            }
        }

        // 2. Download and write each URL
        for url in &all_urls_to_fetch {
            let content = match self.fetch_url(url).await {
                Ok(c) => c,
                Err(err) => {
                    // Skip errors on dynamically discovered child links
                    if all_urls_to_fetch.len() > 1 && !resource_urls.contains(url) {
                        continue;
                    }
                    return Err(err);
                }
            };

            let sanitized = Self::sanitize_markdown(&content);
            let raw_slug = Self::smart_slugify(url, None);
            let domain_prefix = Self::get_domain_prefix(url);
            let final_filename = if saved_filenames.contains(&raw_slug) {
                format!("{domain_prefix}-{raw_slug}")
            } else {
                raw_slug
            };

            let dest_path = Self::check_path_traversal(&auto_dir, &final_filename)?;
            let mut file = File::create(&dest_path).map_err(|e| SkillError::io(&dest_path, e))?;
            file.write_all(sanitized.as_bytes())
                .map_err(|e| SkillError::io(&dest_path, e))?;

            saved_filenames.insert(final_filename.clone());

            let rel_path = PathBuf::from("resources/auto").join(&final_filename);
            downloaded_files.push(
                ResourceFile::builder()
                    .relative_path(rel_path)
                    .absolute_path(dest_path)
                    .kind(ResourceKind::Auto)
                    .content(Some(sanitized))
                    .build(),
            );
        }

        // 3. Prune orphaned files in auto directory
        if let Ok(entries) = fs::read_dir(&auto_dir) {
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

        Ok(downloaded_files)
    }

    /// Downloads resources across a collection of skills.
    pub async fn download_all_resources(
        &self,
        skills: &[Skill],
    ) -> Result<HashMap<String, Vec<ResourceFile>>> {
        let mut results = HashMap::new();
        for skill in skills {
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
        if content.starts_with("Title:") {
            if let Some(idx) = content.find("\nMarkdown Content:\n") {
                return content[idx + "\nMarkdown Content:\n".len()..].trim_start();
            }
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
        let lower = content_type.to_ascii_lowercase();
        lower.starts_with("image/")
            || lower.starts_with("audio/")
            || lower.starts_with("video/")
            || lower.starts_with("font/")
            || lower.contains("application/pdf")
            || lower.contains("application/zip")
            || lower.contains("application/octet-stream")
            || lower.contains("application/x-tar")
            || lower.contains("application/gzip")
    }

    /// Checks if a URL targets an `llms.txt` or `llms-full.txt` index file.
    #[must_use]
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_llms_index_url(url_str: &str) -> bool {
        let clean = url_str.split('?').next().unwrap_or(url_str);
        let lower = clean.to_ascii_lowercase();
        lower.ends_with("/llms.txt")
            || lower.ends_with("/_llms.txt")
            || lower.ends_with("/llms-full.txt")
            || (lower.contains("llms") && (lower.ends_with(".txt") || lower.ends_with(".md")))
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
        if trimmed_path.is_empty() || trimmed_path == "index" || trimmed_path == "overview" {
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

        let final_slug = slug.trim_matches('-');
        if final_slug.is_empty() || final_slug == "index" {
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

    /// Parses links from an `llms.txt` document.
    #[must_use]
    pub fn parse_llms_txt_links(content: &str, base_url: &str) -> Vec<String> {
        let mut links = Vec::new();
        let base_domain = Self::get_domain_prefix(base_url);

        for line in content.split('\n') {
            let trimmed = line.trim();
            if let Some(start_bracket) = trimmed.find('[') {
                let after = &trimmed[start_bracket + 1..];
                if let Some(close_bracket) = after.find(']') {
                    let after_close = &after[close_bracket + 1..];
                    if after_close.starts_with('(') {
                        if let Some(close_paren) = after_close.find(')') {
                            let raw_link = after_close[1..close_paren].trim();
                            let clean_link = raw_link
                                .trim_matches('<')
                                .trim_matches('>')
                                .trim_matches('"')
                                .trim_matches('\'');

                            if !clean_link.is_empty() && !Self::is_non_text_url(clean_link) {
                                let resolved = if clean_link.starts_with("http://")
                                    || clean_link.starts_with("https://")
                                {
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
                                };

                                let link_domain = Self::get_domain_prefix(&resolved);
                                if link_domain == base_domain
                                    && resolved != base_url
                                    && !links.contains(&resolved)
                                {
                                    links.push(resolved);
                                }
                            }
                        }
                    }
                }
            }
        }

        links
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
        format!("{:x}", hasher.finalize())
    }

    /// Validates SHA-256 digest of input bytes.
    #[must_use]
    pub fn verify_sha256(data: &[u8], expected_hex: &str) -> bool {
        let actual = Self::compute_sha256(data);
        actual.eq_ignore_ascii_case(expected_hex.trim())
    }
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
