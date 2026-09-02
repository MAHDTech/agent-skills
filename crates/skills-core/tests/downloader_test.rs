//! Unit and mock integration tests for `skills-core` remote downloader and registry client.

use skills_core::downloader::SkillDownloader;
use skills_core::models::{ResourceKind, Skill, SkillCategory, SkillFrontmatter};
use std::fs::{self, File};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_slugify_variations() {
    let docs_url = format!("https://{}/serde/latest/serde/index.html", "docs.rs");
    let raw_url = format!(
        "https://{}/MAHDTech/agent-skills/raw/main/guide.txt",
        "github.com"
    );
    let api_url = format!("https://{}/api/v1/reference", "example.com");
    let root_url = format!("https://{}/", "example.com");

    assert_eq!(
        SkillDownloader::smart_slugify(&docs_url, None),
        "serde-latest-serde-index.md"
    );
    assert_eq!(
        SkillDownloader::smart_slugify(&raw_url, None),
        "mahdtech-agent-skills-raw-main-guide.txt"
    );
    assert_eq!(
        SkillDownloader::smart_slugify(&api_url, None),
        "api-v1-reference.md"
    );
    assert_eq!(
        SkillDownloader::smart_slugify(&root_url, None),
        "overview-index.md"
    );
}

#[test]
fn test_domain_prefix_extraction() {
    let bun_url = format!("https://{}/docs", "bun.sh");
    let custom_url = format!("https://{}/v1", "docs.example.co.uk");
    let raw_gh_url = format!("https://{}/owner/repo", "raw.githubusercontent.com");

    assert_eq!(SkillDownloader::get_domain_prefix(&bun_url), "bun");
    assert_eq!(SkillDownloader::get_domain_prefix(&custom_url), "example");
    assert_eq!(
        SkillDownloader::get_domain_prefix(&raw_gh_url),
        "githubusercontent"
    );
}

#[test]
fn test_path_traversal_detection() {
    let dir = tempdir().unwrap();
    let base = dir.path();

    // Valid path
    let ok_path = SkillDownloader::check_path_traversal(base, "guide.md");
    assert!(ok_path.is_ok());

    // Path traversal attempt with parent dir
    let bad_path = SkillDownloader::check_path_traversal(base, "../secret.txt");
    assert!(bad_path.is_err());

    // Path traversal with subfolder escaping
    let bad_sub = SkillDownloader::check_path_traversal(base, "sub/../../secret.txt");
    assert!(bad_sub.is_err());
}

#[test]
fn test_markdown_sanitization() {
    let raw = "Title: Scraped Doc\nMarkdown Content:\n# Header &amp; Title\n\nGeneric type: Box<T, U> in text.\nNull byte: \x00 here.\nEmpty link: [text]()\n\n```rust\nlet x: Vec<T> = vec![];\n```\n";
    let sanitized = SkillDownloader::sanitize_markdown(raw);

    assert!(!sanitized.contains("Title: Scraped Doc"));
    assert!(sanitized.contains("# Header & Title"));
    assert!(sanitized.contains(r"Box\<T, U\>"));
    assert!(sanitized.contains("let x: Vec<T> = vec![];")); // Unescaped inside code block
    assert!(!sanitized.contains('\x00'));
}

#[test]
fn test_llms_txt_parser() {
    let index = r"- [Getting Started](https://example.com/docs/start.md): Intro guide
- [API Reference](/docs/api.md): Core endpoints
- [Ignore Image](https://example.com/logo.png): Logo
";

    let links = SkillDownloader::parse_llms_txt_links(index, "https://example.com/docs/llms.txt");
    assert_eq!(links.len(), 2);
    assert_eq!(links[0], "https://example.com/docs/start.md");
    assert_eq!(links[1], "https://example.com/docs/api.md");
}

#[test]
fn test_sha256_verification() {
    let bytes = b"hello world";
    let hash = SkillDownloader::compute_sha256(bytes);
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
    assert!(SkillDownloader::verify_sha256(bytes, &hash));
    assert!(!SkillDownloader::verify_sha256(bytes, "invalid_hash"));
}

#[test]
fn test_auto_resources_clean_and_manual_isolation() {
    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/test-skill");
    let auto_dir = skill_dir.join("resources/auto");
    let manual_dir = skill_dir.join("resources/manual");

    fs::create_dir_all(&auto_dir).unwrap();
    fs::create_dir_all(&manual_dir).unwrap();

    let auto_file = auto_dir.join("downloaded.md");
    let manual_file = manual_dir.join("GUIDE.md");

    File::create(&auto_file).unwrap();
    File::create(&manual_file).unwrap();

    let downloader = SkillDownloader::new();
    let cleaned = downloader.clean_auto_resources(&skill_dir).unwrap();

    assert_eq!(cleaned, 1);
    assert!(!auto_dir.exists());
    assert!(manual_dir.exists());
    assert!(manual_file.exists());
}

#[tokio::test]
async fn test_fetch_url_and_registry_mock() {
    let mock_server = MockServer::start().await;

    let manifest_json = r#"{
        "$schema": "https://skills.sh/schemas/skills.sh.schema.json",
        "groupings": [
            {
                "title": "Engineering",
                "description": "Core engineering skills.",
                "skills": ["domain-modeling"]
            }
        ]
    }"#;

    Mock::given(method("GET"))
        .and(path("/skills.sh.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(manifest_json))
        .mount(&mock_server)
        .await;

    let downloader = SkillDownloader::new();
    let registry_url = format!("{}/skills.sh.json", mock_server.uri());
    let manifest = downloader.fetch_registry(&registry_url).await.unwrap();

    assert_eq!(
        manifest.schema,
        "https://skills.sh/schemas/skills.sh.schema.json"
    );
    assert_eq!(manifest.groupings.len(), 1);
    assert_eq!(manifest.groupings[0].title, "Engineering");
    assert_eq!(manifest.groupings[0].skills[0], "domain-modeling");
}

#[tokio::test]
async fn test_download_skill_resources_end_to_end() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/docs/guide.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Mock Guide Content"))
        .mount(&mock_server)
        .await;

    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/my-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    let skill_path = skill_dir.join("SKILL.md");

    let resource_url = format!("{}/docs/guide.md", mock_server.uri());

    let fm = SkillFrontmatter::builder()
        .name("my-skill")
        .description("Test skill")
        .resources(vec![resource_url])
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("my-skill".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Content".to_string())
        .raw("---\nname: my-skill\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::new();
    let files = downloader.download_skill_resources(&skill).await.unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].kind, ResourceKind::Auto);
    assert!(files[0].absolute_path.exists());

    let content = fs::read_to_string(&files[0].absolute_path).unwrap();
    assert_eq!(content, "# Mock Guide Content");
}
