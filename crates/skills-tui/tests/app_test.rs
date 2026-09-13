use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use skills_core::dashboard::{DashboardSummary, HealthScore, TargetDistribution};
use skills_core::installer::TargetEnvironment;
use skills_core::models::{LintIssue, LintSeverity, Skill, SkillCategory, SkillFrontmatter};
use skills_tui::app::{ActiveView, App};
use skills_tui::ui::draw;

fn create_mock_skill(name: &str, category: SkillCategory, description: &str) -> Skill {
    let frontmatter = SkillFrontmatter::builder()
        .name(name)
        .description(description)
        .build();

    Skill::builder()
        .path(PathBuf::from(format!("skills/{name}/SKILL.md")))
        .dir_name(name)
        .category(category)
        .promoted(false)
        .frontmatter(frontmatter)
        .content(format!("#{name}\nInstructions"))
        .raw(format!("---\nname: {name}\n---\n#{name}"))
        .build()
}

fn create_mock_skill_with_content(name: &str, category: SkillCategory, content: &str) -> Skill {
    let frontmatter = SkillFrontmatter::builder()
        .name(name)
        .description(format!("{name} description"))
        .build();

    Skill::builder()
        .path(PathBuf::from(format!("skills/{name}/SKILL.md")))
        .dir_name(name)
        .category(category)
        .promoted(false)
        .frontmatter(frontmatter)
        .content(content.to_string())
        .raw(format!("---\nname: {name}\n---\n{content}"))
        .build()
}

fn create_mock_lint_issue(
    file: PathBuf,
    rule: &str,
    message: &str,
    severity: LintSeverity,
    line: Option<usize>,
    column: Option<usize>,
) -> LintIssue {
    LintIssue::builder()
        .file(file)
        .rule(rule)
        .message(message)
        .severity(severity)
        .line(line)
        .column(column)
        .build()
}

fn create_mock_summary() -> DashboardSummary {
    DashboardSummary::builder()
        .total_skills(10)
        .active_skills(8)
        .inactive_skills(2)
        .promoted_skills(3)
        .lifecycle_skills(2)
        .user_invoked_skills(1)
        .model_invoked_skills(7)
        .forked_skills(2)
        .total_tokens(15000)
        .avg_prompt_tokens(1500)
        .min_prompt_tokens(500)
        .max_prompt_tokens(3000)
        .health(
            HealthScore::builder()
                .score(85.0)
                .total_errors(0)
                .total_warnings(3)
                .clean_skills(7)
                .clean_percentage(70.0)
                .build(),
        )
        .categories(Vec::new())
        .targets(
            vec![TargetEnvironment::ClaudeDesktop, TargetEnvironment::Cursor]
                .into_iter()
                .map(|env| {
                    TargetDistribution::builder()
                        .environment(env)
                        .installed_count(5)
                        .active_count(4)
                        .percentage(50.0)
                        .build()
                })
                .collect(),
        )
        .skill_metrics(Vec::new())
        .build()
}

#[test]
fn test_app_default_initialization() {
    let app = App::new();
    assert_eq!(app.active_view, ActiveView::Explorer);
    assert!(app.skills.is_empty());
    assert!(app.summary.is_none());
    assert!(app.search_filter.is_empty());
    assert!(app.notifications.is_empty());
    assert!(app.running);
    assert_eq!(app.selected_index, 0);
    assert!(app.is_running());
}

#[test]
fn test_app_tab_cycling_via_key_events() {
    let mut app = App::new();
    assert_eq!(app.active_view, ActiveView::Explorer);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.active_view, ActiveView::Inspector);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.active_view, ActiveView::Linter);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.active_view, ActiveView::Runner);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.active_view, ActiveView::Explorer);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::BackTab)));
    assert_eq!(app.active_view, ActiveView::Runner);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::BackTab)));
    assert_eq!(app.active_view, ActiveView::Linter);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::BackTab)));
    assert_eq!(app.active_view, ActiveView::Inspector);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::BackTab)));
    assert_eq!(app.active_view, ActiveView::Explorer);
}

#[test]
fn test_app_direct_tab_selection_via_key_events() {
    let mut app = App::new();

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('1'))));
    assert_eq!(app.active_view, ActiveView::Explorer);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('2'))));
    assert_eq!(app.active_view, ActiveView::Inspector);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('3'))));
    assert_eq!(app.active_view, ActiveView::Linter);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('4'))));
    assert_eq!(app.active_view, ActiveView::Runner);
}

#[test]
fn test_app_selection_bounds_via_key_events() {
    let mut empty_app = App::new();
    assert!(empty_app.selected_skill().is_none());
    assert!(empty_app.handle_key_event(KeyEvent::from(KeyCode::Down)));
    assert_eq!(empty_app.selected_index, 0);
    assert!(empty_app.handle_key_event(KeyEvent::from(KeyCode::Up)));
    assert_eq!(empty_app.selected_index, 0);

    let skills = vec![
        create_mock_skill("skill-a", SkillCategory::Engineering, "Skill A description"),
        create_mock_skill("skill-b", SkillCategory::Engineering, "Skill B description"),
    ];
    let mut app = App::with_skills(skills);
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-a");

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('j'))));
    assert_eq!(app.selected_index, 1);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-b");

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Down)));
    assert_eq!(app.selected_index, 1);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('k'))));
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-a");

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Up)));
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_app_quit_via_key_events() {
    let mut app = App::new();
    assert!(app.is_running());
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('q'))));
    assert!(!app.is_running());

    let mut app2 = App::new();
    assert!(app2.is_running());
    assert!(app2.handle_key_event(KeyEvent::from(KeyCode::Esc)));
    assert!(!app2.is_running());
}

#[test]
fn test_app_ignore_release_key_events() {
    let mut app = App::new();
    let release_event = KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Release,
        state: KeyEventState::NONE,
    };
    assert!(!app.handle_key_event(release_event));
    assert!(app.is_running());
}

#[test]
fn test_app_unhandled_key_events() {
    let mut app = App::new();
    assert!(!app.handle_key_event(KeyEvent::from(KeyCode::Char('z'))));
    assert_eq!(app.active_view, ActiveView::Explorer);
    assert!(app.is_running());
}

#[test]
fn test_app_search_filter() {
    let skills = vec![
        create_mock_skill(
            "git-commit",
            SkillCategory::Tooling,
            "Create conventional commits",
        ),
        create_mock_skill(
            "code-review",
            SkillCategory::Review,
            "Review code changes against standards",
        ),
        create_mock_skill(
            "bevy-dev",
            SkillCategory::GameDevelopment,
            "Bevy engine game development",
        ),
    ];

    let mut app = App::with_skills(skills);
    assert_eq!(app.filtered_skills().len(), 3);

    app.set_search_filter("git");
    assert_eq!(app.filtered_skills().len(), 1);
    assert_eq!(app.filtered_skills()[0].name(), "git-commit");

    app.set_search_filter("review");
    assert_eq!(app.filtered_skills().len(), 1);
    assert_eq!(app.filtered_skills()[0].name(), "code-review");

    app.clear_search_filter();
    assert_eq!(app.filtered_skills().len(), 3);
}

#[test]
fn test_app_notification_stack() {
    let mut app = App::new();
    assert!(app.latest_notification().is_none());
    assert!(app.dismiss_notification().is_none());

    app.add_notification("First notification");
    app.add_notification("Second notification");

    assert_eq!(app.latest_notification(), Some("Second notification"));
    assert_eq!(
        app.dismiss_notification(),
        Some("First notification".to_string())
    );
    assert_eq!(app.latest_notification(), Some("Second notification"));
    assert_eq!(
        app.dismiss_notification(),
        Some("Second notification".to_string())
    );
    assert!(app.dismiss_notification().is_none());
}

#[test]
fn test_ui_headless_render_all_views() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    let skills = vec![
        create_mock_skill(
            "rust-analyzer",
            SkillCategory::Tooling,
            "Rust language server",
        ),
        create_mock_skill("cargo-lint", SkillCategory::Tooling, "Cargo linter wrapper"),
    ];
    let mut app = App::with_skills(skills).with_summary(create_mock_summary());

    for view in ActiveView::all() {
        app.set_tab(*view);
        terminal
            .draw(|frame| draw(frame, &app))
            .expect("Render should succeed for all views");
    }
}

#[test]
fn test_ui_headless_buffer_contents() {
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let skills = vec![create_mock_skill(
        "demo-skill",
        SkillCategory::Authoring,
        "Demo skill",
    )];
    let app = App::with_skills(skills);

    terminal
        .draw(|frame| draw(frame, &app))
        .expect("Headless draw must succeed");

    let buffer = terminal.backend().buffer();
    let content: String = buffer
        .content()
        .iter()
        .map(ratatui::buffer::Cell::symbol)
        .collect();

    assert!(
        content.contains("Agent Skills"),
        "Buffer must contain header title"
    );
    assert!(
        content.contains("Explorer"),
        "Buffer must contain Explorer tab"
    );
    assert!(
        content.contains("Inspector"),
        "Buffer must contain Inspector tab"
    );
    assert!(content.contains("Linter"), "Buffer must contain Linter tab");
    assert!(content.contains("Runner"), "Buffer must contain Runner tab");
    assert!(
        content.contains("Keybindings"),
        "Buffer must contain Keybindings title"
    );
    assert!(
        content.contains("Tab: Switch View"),
        "Buffer must contain navigation hint"
    );
    assert!(
        content.contains("q: Quit"),
        "Buffer must contain quit shortcut"
    );
}

#[test]
fn test_linter_navigation_bounds() {
    let skills = vec![create_mock_skill(
        "lint-test",
        SkillCategory::Tooling,
        "Lint test skill",
    )];
    let mut app = App::with_skills(skills);
    app.set_tab(ActiveView::Linter);

    app.linter_diagnostics = vec![
        create_mock_lint_issue(
            PathBuf::from("skills/lint-test/SKILL.md"),
            "naming-kebab-case",
            "Name must be kebab-case",
            LintSeverity::Error,
            Some(1),
            Some(1),
        ),
        create_mock_lint_issue(
            PathBuf::from("skills/lint-test/SKILL.md"),
            "no-em-dashes",
            "Found unicode em-dash",
            LintSeverity::Warning,
            Some(5),
            Some(12),
        ),
        create_mock_lint_issue(
            PathBuf::from("skills/lint-test/SKILL.md"),
            "broken-links",
            "Relative link not found",
            LintSeverity::Warning,
            None,
            None,
        ),
    ];

    assert_eq!(app.linter_selected_index, 0);

    // Navigate down with 'j' and Down arrow
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('j'))));
    assert_eq!(app.linter_selected_index, 1);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Down)));
    assert_eq!(app.linter_selected_index, 2);

    // Clamping at len - 1
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Down)));
    assert_eq!(app.linter_selected_index, 2);

    // Navigate up with 'k' and Up arrow
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('k'))));
    assert_eq!(app.linter_selected_index, 1);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Up)));
    assert_eq!(app.linter_selected_index, 0);

    // Clamping at 0
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Up)));
    assert_eq!(app.linter_selected_index, 0);
}

#[test]
fn test_linter_selection_jump_success() {
    let skills = vec![
        create_mock_skill("skill-a", SkillCategory::Tooling, "First skill"),
        create_mock_skill("skill-b", SkillCategory::Engineering, "Target skill"),
    ];
    let mut app = App::with_skills(skills);
    app.set_tab(ActiveView::Linter);

    app.linter_diagnostics = vec![create_mock_lint_issue(
        PathBuf::from("skills/skill-b/SKILL.md"),
        "frontmatter-required",
        "Missing description field",
        LintSeverity::Error,
        Some(1),
        None,
    )];
    app.linter_selected_index = 0;

    // Dispatches Enter to jump
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Enter)));
    assert_eq!(app.active_view, ActiveView::Inspector);
    assert_eq!(app.selected_index, 1);
    assert_eq!(app.selected_skill().map(|s| s.name()), Some("skill-b"));
    assert!(app
        .latest_notification()
        .unwrap_or_default()
        .contains("Jumped to skill-b from diagnostic frontmatter-required"));
}

#[test]
fn test_linter_selection_jump_unmatched_path() {
    let skills = vec![create_mock_skill(
        "skill-a",
        SkillCategory::Tooling,
        "First skill",
    )];
    let mut app = App::with_skills(skills);
    app.set_tab(ActiveView::Linter);

    app.linter_diagnostics = vec![create_mock_lint_issue(
        PathBuf::from("nonexistent/skill/SKILL.md"),
        "frontmatter-syntax",
        "Syntax error",
        LintSeverity::Error,
        None,
        None,
    )];
    app.linter_selected_index = 0;

    // Dispatches Enter - should fail jump gracefully
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Enter)));
    assert_eq!(app.active_view, ActiveView::Linter);
    assert!(app
        .latest_notification()
        .unwrap_or_default()
        .contains("No matching skill found for diagnostic"));
}

#[test]
fn test_linter_refresh_diagnostics() {
    let skills = vec![create_mock_skill(
        "bad_naming",
        SkillCategory::Tooling,
        "Bad name",
    )];
    let mut app = App::with_skills(skills);
    app.set_tab(ActiveView::Linter);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('r'))));
    assert!(app
        .latest_notification()
        .unwrap_or_default()
        .contains("Diagnostics refreshed"));
}

#[test]
fn test_runner_placeholder_extraction_and_focus_cycling() {
    let content = "Run on {{branch:-main}} with {{flags}} and {{branch}}.";
    let skill = create_mock_skill_with_content("git-run", SkillCategory::Tooling, content);
    let mut app = App::with_skills(vec![skill]);
    app.set_tab(ActiveView::Runner);

    let placeholders = app.runner_placeholders();
    assert_eq!(placeholders.len(), 2);
    assert_eq!(placeholders[0].name, "branch");
    assert_eq!(placeholders[0].default_value.as_deref(), Some("main"));
    assert_eq!(placeholders[1].name, "flags");
    assert_eq!(placeholders[1].default_value, None);

    assert_eq!(app.runner_selected_field, 0);

    // Circular focus next with Tab and Down
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.runner_selected_field, 1);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Tab)));
    assert_eq!(app.runner_selected_field, 0);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Down)));
    assert_eq!(app.runner_selected_field, 1);

    // Circular focus prev with BackTab and Up
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::BackTab)));
    assert_eq!(app.runner_selected_field, 0);

    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Up)));
    assert_eq!(app.runner_selected_field, 1);
}

#[test]
fn test_runner_interactive_text_editing() {
    let content = "Commit with message {{message}}.";
    let skill = create_mock_skill_with_content("git-commit", SkillCategory::Tooling, content);
    let mut app = App::with_skills(vec![skill]);
    app.set_tab(ActiveView::Runner);

    assert!(!app.runner_input_active);

    // Enter editing mode via Enter
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Enter)));
    assert!(app.runner_input_active);

    // Type "feat"
    for c in ['f', 'e', 'a', 't'] {
        assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char(c))));
    }
    assert_eq!(app.get_runner_param("message"), Some("feat"));

    // Backspace to erase 't' -> "fea"
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Backspace)));
    assert_eq!(app.get_runner_param("message"), Some("fea"));

    // Exit editing mode via Esc
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Esc)));
    assert!(!app.runner_input_active);

    // Enter editing mode via 'i'
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('i'))));
    assert!(app.runner_input_active);

    // Exit editing mode via Enter
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Enter)));
    assert!(!app.runner_input_active);
}

#[test]
fn test_runner_live_prompt_substitution() {
    let content = "Deploy {{service}} to {{env:-staging}} with {{replicas}}.";
    let skill = create_mock_skill_with_content("deploy-svc", SkillCategory::Engineering, content);
    let mut app = App::with_skills(vec![skill]);
    app.set_tab(ActiveView::Runner);

    // Initial prompt: service missing, env defaults to staging, replicas missing
    let initial = app.render_runner_prompt();
    assert_eq!(
        initial,
        "Deploy [REQUIRED: service] to staging with [REQUIRED: replicas]."
    );

    // Set service
    app.set_runner_param("service", "api-gateway");
    let after_svc = app.render_runner_prompt();
    assert_eq!(
        after_svc,
        "Deploy api-gateway to staging with [REQUIRED: replicas]."
    );

    // Set replicas
    app.set_runner_param("replicas", "3");
    let complete = app.render_runner_prompt();
    assert_eq!(complete, "Deploy api-gateway to staging with 3.");
}

#[test]
fn test_runner_validation_status() {
    let content = "Deploy {{service}} to {{env:-staging}} with {{replicas}}.";
    let skill = create_mock_skill_with_content("deploy-svc", SkillCategory::Engineering, content);
    let mut app = App::with_skills(vec![skill]);
    app.set_tab(ActiveView::Runner);

    let status = app.validate_runner_params();
    assert!(status.is_err());
    let missing = status.unwrap_err();
    assert_eq!(missing, vec!["service".to_string(), "replicas".to_string()]);

    app.set_runner_param("service", "billing");
    app.set_runner_param("replicas", "2");

    assert!(app.validate_runner_params().is_ok());

    // Reset parameters
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('r'))));
    assert!(app.validate_runner_params().is_err());
    assert!(app
        .latest_notification()
        .unwrap_or_default()
        .contains("Parameters reset to defaults"));
}

#[test]
fn test_runner_export_actions() {
    let content = "Execute {{task}} on {{target}}.";
    let skill = create_mock_skill_with_content("runner-exec", SkillCategory::Tooling, content);
    let mut app = App::with_skills(vec![skill]);
    app.set_tab(ActiveView::Runner);
    app.set_runner_param("task", "build");
    app.set_runner_param("target", "prod");

    // File export to custom destination
    let temp_dir = std::env::temp_dir();
    let export_path = temp_dir.join("test-runner-export-prompt.txt");
    let exported = app.export_runner_prompt_file(Some(&export_path));
    assert!(exported.is_ok());
    let written = std::fs::read_to_string(&export_path).expect("File must be written");
    assert_eq!(written, "Execute build on prod.");
    let _ = std::fs::remove_file(export_path);

    // Clipboard export
    let clip_res = app.export_runner_prompt_clipboard();
    assert!(clip_res.is_ok());
    assert!(app
        .latest_notification()
        .unwrap_or_default()
        .contains("Prompt copied to clipboard via OSC 52"));

    // Key event triggers 'c', 'e'
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('c'))));
    assert!(app.handle_key_event(KeyEvent::from(KeyCode::Char('e'))));
    let default_export = PathBuf::from("./runner-exec-prompt.txt");
    if default_export.exists() {
        let _ = std::fs::remove_file(default_export);
    }
}

#[test]
fn test_linter_headless_buffer_rendering() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    let skills = vec![create_mock_skill(
        "broken-skill",
        SkillCategory::Tooling,
        "Broken skill",
    )];
    let mut app = App::with_skills(skills).with_summary(create_mock_summary());
    app.set_tab(ActiveView::Linter);

    app.linter_diagnostics = vec![
        create_mock_lint_issue(
            PathBuf::from("skills/broken-skill/SKILL.md"),
            "frontmatter-required",
            "Missing mandatory name and description fields",
            LintSeverity::Error,
            Some(1),
            Some(1),
        ),
        create_mock_lint_issue(
            PathBuf::from("skills/broken-skill/SKILL.md"),
            "no-em-dashes",
            "Found unicode em-dash in body",
            LintSeverity::Warning,
            Some(10),
            Some(4),
        ),
    ];

    terminal
        .draw(|frame| draw(frame, &app))
        .expect("Linter headless draw must succeed");

    let buffer = terminal.backend().buffer();
    let content: String = buffer
        .content()
        .iter()
        .map(ratatui::buffer::Cell::symbol)
        .collect();

    assert!(content.contains("Diagnostics (2 findings)"));
    assert!(content.contains("Severity"));
    assert!(content.contains("Rule"));
    assert!(content.contains("Skill"));
    assert!(content.contains("Location"));
    assert!(content.contains("Summary"));
    assert!(content.contains("[ ERROR ]"));
    assert!(content.contains("frontmatter-required"));
    assert!(content.contains("Diagnostic Remediation"));
    assert!(content.contains("Add mandatory 'name' and 'description'"));
    assert!(content.contains("Workspace Health"));
}

#[test]
fn test_runner_headless_buffer_rendering() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    let content = "# Header 1\n## Subheader\n> A quote\nDeploy {{service}} to {{env:-staging}}.";
    let skill = create_mock_skill_with_content("deploy-skill", SkillCategory::Engineering, content);
    let mut app = App::with_skills(vec![skill]).with_summary(create_mock_summary());
    app.set_tab(ActiveView::Runner);

    terminal
        .draw(|frame| draw(frame, &app))
        .expect("Runner headless draw must succeed");

    let buffer = terminal.backend().buffer();
    let content_str: String = buffer
        .content()
        .iter()
        .map(ratatui::buffer::Cell::symbol)
        .collect();

    assert!(content_str.contains("Target Skill"));
    assert!(content_str.contains("deploy-skill"));
    assert!(content_str.contains("Parameters"));
    assert!(content_str.contains("service*"));
    assert!(content_str.contains("env (default: staging)"));
    assert!(content_str.contains("Validation"));
    assert!(content_str.contains("Missing required: service"));
    assert!(content_str.contains("Live Prompt Preview (deploy-skill)"));
    assert!(content_str.contains("[REQUIRED: service]"));

    // Activate input mode and verify cursor glyph renders
    app.activate_runner_input();
    terminal
        .draw(|frame| draw(frame, &app))
        .expect("Runner headless draw in active mode must succeed");

    let active_buffer = terminal.backend().buffer();
    let active_content: String = active_buffer
        .content()
        .iter()
        .map(ratatui::buffer::Cell::symbol)
        .collect();
    assert!(active_content.contains('█'));
}
