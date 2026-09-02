use skills_core::error::SkillError;
use skills_core::models::{ResourceKind, SkillCategory};
use skills_core::parser::SkillParser;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_extract_frontmatter_standard() {
    let content = r"---
name: domain-modeling
description: Build and sharpen domain models.
metadata:
  source: mattpocock/skills
---
# Domain Modeling

Detailed content here.
";

    let (yaml, body) = SkillParser::extract_frontmatter(content).unwrap();
    assert!(yaml.contains("name: domain-modeling"));
    assert!(yaml.contains("description: Build and sharpen domain models."));
    assert!(body.contains("# Domain Modeling"));
    assert!(body.contains("Detailed content here."));
}

#[test]
fn test_extract_frontmatter_crlf() {
    let content = "---\r\nname: acp\r\ndescription: Agent Client Protocol.\r\n---\r\n# Title\r\nBody text.\r\n";
    let (yaml, body) = SkillParser::extract_frontmatter(content).unwrap();
    assert!(yaml.contains("name: acp"));
    assert!(body.contains("# Title"));
}

#[test]
fn test_extract_frontmatter_errors() {
    let no_opening = "name: acp\ndescription: foo\n---\n# Body";
    assert!(matches!(
        SkillParser::extract_frontmatter(no_opening),
        Err(SkillError::FrontmatterValidation { .. })
    ));

    let no_closing = "---\nname: acp\ndescription: foo\n# Body";
    assert!(matches!(
        SkillParser::extract_frontmatter(no_closing),
        Err(SkillError::FrontmatterValidation { .. })
    ));
}

#[test]
fn test_validate_skill_name_rules() {
    // Valid kebab-case names
    assert!(SkillParser::validate_skill_name("domain-modeling", "domain-modeling").is_ok());
    assert!(SkillParser::validate_skill_name("acp", "acp").is_ok());
    assert!(SkillParser::validate_skill_name("bevy-2d-games", "").is_ok());

    // Invalid naming
    assert!(matches!(
        SkillParser::validate_skill_name("", ""),
        Err(SkillError::InvalidSkillName { .. })
    ));
    assert!(matches!(
        SkillParser::validate_skill_name("Domain-Modeling", "Domain-Modeling"),
        Err(SkillError::InvalidSkillName { .. })
    ));
    assert!(matches!(
        SkillParser::validate_skill_name("domain_modeling", "domain_modeling"),
        Err(SkillError::InvalidSkillName { .. })
    ));
    assert!(matches!(
        SkillParser::validate_skill_name("claude-helper", "claude-helper"),
        Err(SkillError::InvalidSkillName { .. })
    ));
    assert!(matches!(
        SkillParser::validate_skill_name("anthropic-agent", "anthropic-agent"),
        Err(SkillError::InvalidSkillName { .. })
    ));

    // Directory mismatch
    assert!(matches!(
        SkillParser::validate_skill_name("skill-a", "skill-b"),
        Err(SkillError::InvalidSkillName { .. })
    ));
}

#[test]
fn test_extract_sections_and_code_block_isolation() {
    let body = r"# Main Title

Intro text before sections.

## Rules
Rule 1: Never guess.
Rule 2: Keep changes surgical.

```bash
# This is a comment inside a code block, NOT a heading!
echo 'hello'
```

## Context
Background context here.

### Nested Subsection
Detailed point.
";

    let sections = SkillParser::extract_sections(body);
    assert_eq!(sections.len(), 4);

    assert_eq!(sections[0].level, 1);
    assert_eq!(sections[0].title, "Main Title");
    assert!(sections[0].content.contains("Intro text before sections."));

    assert_eq!(sections[1].level, 2);
    assert_eq!(sections[1].title, "Rules");
    assert!(sections[1].content.contains("Rule 1: Never guess."));
    assert!(sections[1]
        .content
        .contains("# This is a comment inside a code block"));

    assert_eq!(sections[2].level, 2);
    assert_eq!(sections[2].title, "Context");
    assert!(sections[2].content.contains("Background context here."));

    assert_eq!(sections[3].level, 3);
    assert_eq!(sections[3].title, "Nested Subsection");
    assert!(sections[3].content.contains("Detailed point."));
}

#[test]
fn test_extract_and_render_template_placeholders() {
    let template =
        "Hello {{ user }}, welcome to {{ repo:-agent-skills }}! Target branch: {{ branch:main }}.";
    let placeholders = SkillParser::extract_placeholders(template);

    assert_eq!(placeholders.len(), 3);
    assert_eq!(placeholders[0].name, "user");
    assert_eq!(placeholders[0].default_value, None);

    assert_eq!(placeholders[1].name, "repo");
    assert_eq!(
        placeholders[1].default_value.as_deref(),
        Some("agent-skills")
    );

    assert_eq!(placeholders[2].name, "branch");
    assert_eq!(placeholders[2].default_value.as_deref(), Some("main"));

    let mut vars = HashMap::new();
    vars.insert("user".to_string(), "Alice".to_string());
    vars.insert("branch".to_string(), "develop".to_string());

    let rendered = SkillParser::render_template(template, &vars).unwrap();
    assert_eq!(
        rendered,
        "Hello Alice, welcome to agent-skills! Target branch: develop."
    );

    // Missing required variable without default
    let empty_vars = HashMap::new();
    let err = SkillParser::render_template(template, &empty_vars);
    assert!(matches!(err, Err(SkillError::FrontmatterValidation { .. })));
}

#[test]
fn test_parsed_skill_accessors() {
    let doc = r"---
name: my-skill
description: Skill description.
---
# My Skill

## Rules
Rule A

## Context
Context B

## Examples
Example C

## Execution Prompts
Prompt D
";

    let parsed = SkillParser::parse_str(doc).unwrap();
    assert_eq!(parsed.name(), "my-skill");
    assert_eq!(parsed.description(), "Skill description.");
    assert_eq!(parsed.title(), Some("My Skill"));
    assert_eq!(parsed.rules(), Some("Rule A"));
    assert_eq!(parsed.context(), Some("Context B"));
    assert_eq!(parsed.examples(), Some("Example C"));
    assert_eq!(parsed.execution_prompts(), Some("Prompt D"));
}

#[test]
fn test_discover_resources_and_file_parsing() {
    let dir = tempdir().unwrap();
    let skill_dir = dir.path().join("skills/engineering/test-skill");
    let auto_dir = skill_dir.join("resources/auto");
    let manual_dir = skill_dir.join("resources/manual");

    fs::create_dir_all(&auto_dir).unwrap();
    fs::create_dir_all(&manual_dir).unwrap();

    let skill_file_path = skill_dir.join("SKILL.md");
    let mut skill_file = File::create(&skill_file_path).unwrap();
    write!(
        skill_file,
        r"---
name: test-skill
description: Comprehensive testing skill.
---
# Test Skill

Instructions.
"
    )
    .unwrap();

    let auto_file_path = auto_dir.join("llms.txt");
    let mut auto_file = File::create(&auto_file_path).unwrap();
    write!(auto_file, "LLM documentation").unwrap();

    let manual_file_path = manual_dir.join("GUIDE.md");
    let mut manual_file = File::create(&manual_file_path).unwrap();
    write!(manual_file, "Manual user guide").unwrap();

    // Parse single skill file
    let parser = SkillParser::builder().load_resource_contents(true).build();
    let skill = parser.parse_file_with_options(&skill_file_path).unwrap();

    assert_eq!(skill.name(), "test-skill");
    assert_eq!(skill.category, SkillCategory::Engineering);
    assert!(skill.promoted);
    assert_eq!(skill.resources.len(), 2);

    assert_eq!(skill.resources[0].kind, ResourceKind::Auto);
    assert_eq!(
        skill.resources[0].relative_path,
        PathBuf::from("resources/auto/llms.txt")
    );
    assert_eq!(
        skill.resources[0].content.as_deref(),
        Some("LLM documentation")
    );

    assert_eq!(skill.resources[1].kind, ResourceKind::Manual);
    assert_eq!(
        skill.resources[1].relative_path,
        PathBuf::from("resources/manual/GUIDE.md")
    );
    assert_eq!(
        skill.resources[1].content.as_deref(),
        Some("Manual user guide")
    );

    // Recursive discovery across directory tree
    let discovered = SkillParser::discover_skills(dir.path()).unwrap();
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].name(), "test-skill");
}

#[test]
fn test_unicode_and_emoji_handling() {
    let doc = r"---
name: unicode-skill
description: Emojis and multi-byte characters 🚀 💡.
---
# 🚀 Unicode Skill (日本語, 🎯)

## Rules
- ルール 1: 正確に実行すること。
";

    let parsed = SkillParser::parse_str(doc).unwrap();
    assert_eq!(parsed.name(), "unicode-skill");
    assert_eq!(
        parsed.description(),
        "Emojis and multi-byte characters 🚀 💡."
    );
    assert_eq!(parsed.title(), Some("🚀 Unicode Skill (日本語, 🎯)"));
    assert!(parsed
        .rules()
        .unwrap()
        .contains("ルール 1: 正確に実行すること。"));
}
