//! Unit and mock integration tests for `skills-core` remote downloader and registry client.
// cspell:words agentskills dfinity reserialized spacetimedb

use skills_core::downloader::SkillDownloader;
use skills_core::models::{
    ResourceKind, Skill, SkillCategory, SkillFrontmatter, SkillsDiscoveryEntry,
    SkillsDiscoveryIndex,
};
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

#[test]
fn test_is_non_text_content_type() {
    assert!(SkillDownloader::is_non_text_content_type("image/png"));
    assert!(SkillDownloader::is_non_text_content_type("IMAGE/JPEG"));
    assert!(SkillDownloader::is_non_text_content_type("audio/mp3"));
    assert!(SkillDownloader::is_non_text_content_type("video/mp4"));
    assert!(SkillDownloader::is_non_text_content_type("font/woff2"));
    assert!(SkillDownloader::is_non_text_content_type("application/pdf"));
    assert!(SkillDownloader::is_non_text_content_type("application/zip"));
    assert!(SkillDownloader::is_non_text_content_type(
        "application/octet-stream"
    ));
    assert!(SkillDownloader::is_non_text_content_type(
        "application/x-tar"
    ));
    assert!(SkillDownloader::is_non_text_content_type(
        "application/gzip"
    ));

    assert!(!SkillDownloader::is_non_text_content_type("text/plain"));
    assert!(!SkillDownloader::is_non_text_content_type("text/markdown"));
    assert!(!SkillDownloader::is_non_text_content_type("text/html"));
    assert!(!SkillDownloader::is_non_text_content_type(
        "application/json"
    ));
    assert!(!SkillDownloader::is_non_text_content_type(
        "application/yaml"
    ));
}

#[tokio::test]
async fn test_download_skill_resources_empty_resources() {
    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/empty-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    let skill_path = skill_dir.join("SKILL.md");

    let fm = SkillFrontmatter::builder()
        .name("empty-skill")
        .description("Empty resources test skill")
        .resources(None)
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("empty-skill".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Content".to_string())
        .raw("---\nname: empty-skill\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::new();
    let files = downloader.download_skill_resources(&skill).await.unwrap();
    assert_eq!(files, [] as [skills_core::ResourceFile; 0]);
}

#[tokio::test]
async fn test_fetch_url_nonexistent_fails() {
    let downloader = SkillDownloader::new();
    let res = downloader.fetch_url("http://127.0.0.1:9/nonexistent").await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_download_skill_resources_llms_expansion_collision_and_orphan_pruning() {
    let mock_server = MockServer::start().await;
    let base_uri = mock_server.uri();

    let llms_body = format!(
        "- [Guide]({base_uri}/docs/guide.md)\n- [Duplicate Guide]({base_uri}/docs/guide.md)\n- [Broken Child]({base_uri}/docs/missing.md)\n"
    );

    Mock::given(method("GET"))
        .and(path("/llms.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string(llms_body))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/docs/guide.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Child Guide"))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/docs/missing.md"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/llms-skill");
    let auto_dir = skill_dir.join("resources/auto");
    fs::create_dir_all(&auto_dir).unwrap();

    // Seed an orphan file (should be pruned) and a hidden file (should be preserved)
    let orphan_path = auto_dir.join("stale-orphan.md");
    let hidden_path = auto_dir.join(".keep");
    fs::write(&orphan_path, "old content").unwrap();
    fs::write(&hidden_path, "keep").unwrap();

    let skill_path = skill_dir.join("SKILL.md");
    let fm = SkillFrontmatter::builder()
        .name("llms-skill")
        .description("Test llms expansion and pruning")
        .resources(vec![
            format!("{base_uri}/llms.txt"),
            format!("{base_uri}/docs/guide.md?duplicate=1"),
            format!("{base_uri}/assets/diagram.png"),
        ])
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("llms-skill".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Content".to_string())
        .raw("---\nname: llms-skill\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::builder().max_retries(0).build();
    let files = downloader.download_skill_resources(&skill).await.unwrap();

    // Expects llms.txt, child guide.md, and duplicate guide.md with domain prefix
    assert_eq!(files.len(), 3);
    assert!(!orphan_path.exists());
    assert!(hidden_path.exists());
}

#[tokio::test]
async fn test_download_skill_resources_skips_vendor_skill() {
    let skill_path = std::path::PathBuf::from("skills/vendor/acme/vendor-skill/SKILL.md");

    let fm = SkillFrontmatter::builder()
        .name("vendor-skill")
        .description("Vendor skill")
        .resources(vec!["http://127.0.0.1:9/should-not-fetch.md".to_string()])
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("vendor-skill".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Vendor".to_string())
        .raw("---\nname: vendor-skill\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::new();
    let files = downloader.download_skill_resources(&skill).await.unwrap();
    assert!(files.is_empty());
}

#[test]
fn test_skills_discovery_index_serde_v010_and_v020() {
    let v010_json = r#"{
        "skills": [
            {
                "name": "icp-cli",
                "description": "Build and deploy canisters.",
                "files": ["SKILL.md", "references/config.md"]
            }
        ]
    }"#;

    let parsed_v010: SkillsDiscoveryIndex = serde_json::from_str(v010_json).unwrap();
    assert_eq!(parsed_v010.schema, None);
    assert_eq!(parsed_v010.skills.len(), 1);
    assert_eq!(parsed_v010.skills[0].name, "icp-cli");
    assert_eq!(parsed_v010.skills[0].entry_type, None);
    assert_eq!(parsed_v010.skills[0].url, None);
    assert_eq!(parsed_v010.skills[0].digest, None);
    assert_eq!(
        parsed_v010.skills[0].files,
        vec!["SKILL.md", "references/config.md"]
    );

    let reserialized_v010 = serde_json::to_value(&parsed_v010).unwrap();
    assert!(reserialized_v010.get("$schema").is_none());
    assert!(reserialized_v010["skills"][0].get("type").is_none());
    assert!(reserialized_v010["skills"][0].get("url").is_none());
    assert!(reserialized_v010["skills"][0].get("digest").is_none());

    let schema_uri = format!(
        "https://{}/discovery/0.2.0/schema.json",
        "schemas.agentskills.io"
    );
    let built_v020 = SkillsDiscoveryIndex::builder()
        .schema(Some(schema_uri.clone()))
        .skills(vec![
            SkillsDiscoveryEntry::builder()
                .name("cli")
                .entry_type(Some("skill-md".to_string()))
                .description("SpacetimeDB CLI reference")
                .url(Some("/.well-known/agent-skills/cli/SKILL.md".to_string()))
                .digest(Some("sha256:abc123".to_string()))
                .build(),
        ])
        .build();

    let v020_str = serde_json::to_string(&built_v020).unwrap();
    let round_trip_v020: SkillsDiscoveryIndex = serde_json::from_str(&v020_str).unwrap();
    assert_eq!(round_trip_v020, built_v020);
    let v020_val = serde_json::to_value(&round_trip_v020).unwrap();
    assert_eq!(v020_val["$schema"], schema_uri);
    assert_eq!(v020_val["skills"][0]["type"], "skill-md");
    assert!(v020_val["skills"][0].get("files").is_none());
}

#[test]
fn test_is_skills_discovery_index_url_and_dotfile_safe_slugs() {
    let v010_url = format!("https://{}/.well-known/skills/index.json", "dfinity.org");
    let v020_url = format!(
        "https://{}/.well-known/agent-skills/index.json?v=2#main",
        "spacetimedb.com"
    );
    let registry_url = format!("https://{}/skills.sh.json", "skills.sh");

    assert!(SkillDownloader::is_skills_discovery_index_url(&v010_url));
    assert!(SkillDownloader::is_skills_discovery_index_url(&v020_url));
    assert!(SkillDownloader::is_skills_discovery_index_url(
        "/.well-known/agent-skills/index.json"
    ));
    assert!(!SkillDownloader::is_skills_discovery_index_url(
        &registry_url
    ));

    let skill_md_url = format!(
        "https://{}/.well-known/agent-skills/cli/SKILL.md",
        "spacetimedb.com"
    );
    let slug = SkillDownloader::smart_slugify(&skill_md_url, None);
    assert!(!slug.starts_with('.'));
    assert_eq!(slug, "well-known-agent-skills-cli-skill.md");

    let underscore_index = format!("https://{}/_index.md", "example.com");
    assert_eq!(
        SkillDownloader::smart_slugify(&underscore_index, None),
        "overview-index.md"
    );
}

#[test]
fn test_parse_llms_txt_links_multiple_and_bare_urls() {
    let host = "example.com";
    let other_host = "other.org";
    let base_url = format!("https://{host}/docs/llms.txt");
    let content = format!(
        "- [First](https://{host}/docs/one.md) and [Second](/docs/two.md)\n\
         - [Tag] [Third](relative-three.md)\n\
         Discovery manifest: https://{host}/.well-known/agent-skills/index.json.\n\
         Also see (https://{host}/docs/bare.md), and ignore https://{other_host}/skip.md\n"
    );

    let links = SkillDownloader::parse_llms_txt_links(&content, &base_url);
    assert_eq!(
        links,
        vec![
            format!("https://{host}/docs/one.md"),
            format!("https://{host}/docs/two.md"),
            format!("https://{host}/docs/relative-three.md"),
            format!("https://{host}/.well-known/agent-skills/index.json"),
            format!("https://{host}/docs/bare.md"),
        ]
    );
}

#[test]
fn test_verify_sha256_with_prefix() {
    let payload = b"raw skill markdown with <T> generic";
    let hex = SkillDownloader::compute_sha256(payload);
    let prefixed_lower = format!("sha256:{hex}");
    let prefixed_upper = format!("SHA256:{}", hex.to_ascii_uppercase());

    assert!(SkillDownloader::verify_sha256(payload, &hex));
    assert!(SkillDownloader::verify_sha256(payload, &prefixed_lower));
    assert!(SkillDownloader::verify_sha256(payload, &prefixed_upper));
    assert!(!SkillDownloader::verify_sha256(payload, "sha256:deadbeef"));
}

#[tokio::test]
async fn test_download_skill_resources_well_known_v010_and_v020_via_llms_txt() {
    let mock_server = MockServer::start().await;
    let base_uri = mock_server.uri();

    let cli_skill_raw = "# CLI Skill\n\nGeneric type Vec<T> outside code fence.\n";
    let cli_digest = format!(
        "sha256:{}",
        SkillDownloader::compute_sha256(cli_skill_raw.as_bytes())
    );

    let sdk_skill_raw = "# SDK Skill\n\nClient SDK reference.\n";
    let sdk_digest = format!(
        "sha256:{}",
        SkillDownloader::compute_sha256(sdk_skill_raw.as_bytes())
    );

    let schema_uri = format!(
        "https://{}/discovery/0.2.0/schema.json",
        "schemas.agentskills.io"
    );
    let v020_index = serde_json::json!({
        "$schema": schema_uri,
        "skills": [
            {
                "name": "cli",
                "type": "skill-md",
                "description": "CLI skill",
                "url": "/.well-known/agent-skills/cli/SKILL.md",
                "digest": cli_digest
            },
            {
                "name": "rust-sdk",
                "type": "skill-md",
                "description": "Rust SDK skill at shared filename",
                "url": "/shared/SKILL.md",
                "digest": sdk_digest
            },
            {
                "name": "bundled-archive",
                "type": "archive",
                "description": "Archive entry should be skipped",
                "url": "/.well-known/agent-skills/bundled-archive/skill.tar.gz"
            }
        ]
    });

    let v010_index = serde_json::json!({
        "skills": [
            {
                "name": "canister-dev",
                "description": "Canister development skill",
                "files": ["SKILL.md", "references/guide.md"]
            }
        ]
    });

    let llms_body = format!(
        "# Documentation Index\n\n\
         Agent skills discovery: {base_uri}/.well-known/agent-skills/index.json\n"
    );

    Mock::given(method("GET"))
        .and(path("/llms.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string(llms_body))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/agent-skills/index.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(v020_index.to_string()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/agent-skills/cli/SKILL.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string(cli_skill_raw))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/shared/SKILL.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string(sdk_skill_raw))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/skills/index.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(v010_index.to_string()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/skills/canister-dev/SKILL.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Canister Dev Skill"))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/skills/canister-dev/references/guide.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Canister Reference Guide"))
        .mount(&mock_server)
        .await;

    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/discovery-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    let skill_path = skill_dir.join("SKILL.md");

    let fm = SkillFrontmatter::builder()
        .name("discovery-skill")
        .description("Test .well-known discovery expansion")
        .resources(vec![
            format!("{base_uri}/llms.txt"),
            format!("{base_uri}/.well-known/skills/index.json"),
        ])
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("discovery-skill".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Discovery Skill".to_string())
        .raw("---\nname: discovery-skill\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::builder().max_retries(0).build();
    let files = downloader.download_skill_resources(&skill).await.unwrap();

    // Expects:
    // 1. llms.txt
    // 2. well-known-agent-skills-cli-skill.md (v0.2.0 via llms.txt, raw bytes verified before <T> escaping)
    // 3. rust-sdk-shared-skill.md (v0.2.0 disambiguated with skill_prefix)
    // 4. well-known-skills-canister-dev-skill.md (v0.1.0 direct)
    // 5. well-known-skills-canister-dev-references-guide.md (v0.1.0 direct)
    assert_eq!(files.len(), 5);

    let filenames: Vec<String> = files
        .iter()
        .map(|f| {
            f.absolute_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();

    for name in &filenames {
        assert!(!name.starts_with('.'), "Unexpected dotfile: {name}");
        assert!(
            !name.contains("index.json"),
            "Raw discovery index.json should not be saved: {name}"
        );
    }

    assert!(filenames.contains(&"llms.txt".to_string()));
    assert!(filenames.contains(&"well-known-agent-skills-cli-skill.md".to_string()));
    assert!(filenames.contains(&"rust-sdk-shared-skill.md".to_string()));
    assert!(filenames.contains(&"well-known-skills-canister-dev-skill.md".to_string()));
    assert!(filenames.contains(&"well-known-skills-canister-dev-references-guide.md".to_string()));

    let cli_file = files
        .iter()
        .find(|f| {
            f.absolute_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .as_ref()
                == "well-known-agent-skills-cli-skill.md"
        })
        .unwrap();
    let saved_cli_content = fs::read_to_string(&cli_file.absolute_path).unwrap();
    assert!(saved_cli_content.contains(r"Vec\<T\>"));
}

#[tokio::test]
async fn test_download_skill_resources_digest_mismatch_returns_error() {
    let mock_server = MockServer::start().await;
    let base_uri = mock_server.uri();

    let schema_uri = format!(
        "https://{}/discovery/0.2.0/schema.json",
        "schemas.agentskills.io"
    );
    let v020_index = serde_json::json!({
        "$schema": schema_uri,
        "skills": [
            {
                "name": "tampered-skill",
                "type": "skill-md",
                "description": "Tampered skill entry",
                "url": "/.well-known/agent-skills/tampered-skill/SKILL.md",
                "digest": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/.well-known/agent-skills/index.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(v020_index.to_string()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/agent-skills/tampered-skill/SKILL.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Tampered Content"))
        .mount(&mock_server)
        .await;

    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/tampered-test");
    fs::create_dir_all(&skill_dir).unwrap();
    let skill_path = skill_dir.join("SKILL.md");

    let fm = SkillFrontmatter::builder()
        .name("tampered-test")
        .description("Test digest mismatch failure")
        .resources(vec![format!(
            "{base_uri}/.well-known/agent-skills/index.json"
        )])
        .build();

    let skill = Skill::builder()
        .path(skill_path)
        .dir_name("tampered-test".to_string())
        .category(SkillCategory::Engineering)
        .promoted(true)
        .frontmatter(fm)
        .content("# Content".to_string())
        .raw("---\nname: tampered-test\n---".to_string())
        .resources(Vec::new())
        .build();

    let downloader = SkillDownloader::builder().max_retries(0).build();
    let result = downloader.download_skill_resources(&skill).await;
    assert!(result.is_err());

    let auto_dir = skill_dir.join("resources/auto");
    let entries: Vec<_> = fs::read_dir(&auto_dir).unwrap().flatten().collect();
    assert!(entries.is_empty());
}
