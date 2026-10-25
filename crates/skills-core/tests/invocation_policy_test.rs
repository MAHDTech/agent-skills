use skills_core::SkillParser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[test]
fn promoted_custom_skills_have_matching_explicit_invocation_policies() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("skills-core must be inside the repository crates directory");
    let skills_root = repo_root.join("skills");
    let mut failures = Vec::new();
    let mut checked = 0;

    for entry in WalkDir::new(&skills_root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            entry.path() != skills_root.join("vendor")
                && entry.path() != skills_root.join("in-progress")
        })
    {
        let entry = entry.expect("custom skill catalog must be readable");
        if entry.file_name() != "SKILL.md" {
            continue;
        }
        let skill = match SkillParser::parse_file(entry.path()) {
            Ok(skill) => skill,
            Err(error) => {
                failures.push(format!("{}: {error}", entry.path().display()));
                continue;
            }
        };
        if !skill.promoted || skill.is_archived() {
            continue;
        }
        checked += 1;
        let sidecar = entry.path().with_file_name("agents").join("openai.yaml");
        let label = format!(
            "{} ({})",
            skill.name(),
            sidecar.strip_prefix(repo_root).unwrap().display()
        );
        let policy = match fs::read_to_string(&sidecar) {
            Ok(content) => match serde_yaml::from_str::<serde_yaml::Value>(&content) {
                Ok(policy) => policy,
                Err(error) => {
                    failures.push(format!("{label}: invalid YAML: {error}"));
                    continue;
                }
            },
            Err(error) => {
                failures.push(format!("{label}: cannot read sidecar: {error}"));
                continue;
            }
        };
        let Some(implicit) = policy["policy"]["allow_implicit_invocation"].as_bool() else {
            failures.push(format!(
                "{label}: policy.allow_implicit_invocation must be an explicit boolean"
            ));
            continue;
        };
        let expected = !skill.frontmatter.disable_model_invocation.unwrap_or(false);
        if implicit != expected {
            failures.push(format!(
                "{label}: allow_implicit_invocation is {implicit}, expected {expected} from SKILL.md"
            ));
        }
    }

    assert!(checked > 0, "no promoted custom skills were checked");
    assert!(
        failures.is_empty(),
        "{} invocation policy failures across {checked} promoted custom skills:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
