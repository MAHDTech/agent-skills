use std::path::PathBuf;

use ratatui::backend::TestBackend;
use ratatui::Terminal;
use skills_core::dashboard::{DashboardSummary, HealthScore, TargetDistribution};
use skills_core::installer::TargetEnvironment;
use skills_core::models::{Skill, SkillCategory, SkillFrontmatter};
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
fn test_app_tab_cycling() {
    let mut app = App::new();
    assert_eq!(app.active_view, ActiveView::Explorer);

    app.next_tab();
    assert_eq!(app.active_view, ActiveView::Inspector);

    app.next_tab();
    assert_eq!(app.active_view, ActiveView::Linter);

    app.next_tab();
    assert_eq!(app.active_view, ActiveView::Runner);

    app.next_tab();
    assert_eq!(app.active_view, ActiveView::Explorer);

    app.prev_tab();
    assert_eq!(app.active_view, ActiveView::Runner);

    app.prev_tab();
    assert_eq!(app.active_view, ActiveView::Linter);

    app.prev_tab();
    assert_eq!(app.active_view, ActiveView::Inspector);

    app.prev_tab();
    assert_eq!(app.active_view, ActiveView::Explorer);
}

#[test]
fn test_app_direct_tab_selection() {
    let mut app = App::new();
    for view in ActiveView::all() {
        app.set_tab(*view);
        assert_eq!(app.active_view, *view);
    }
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
fn test_app_selection_bounds() {
    let mut empty_app = App::new();
    assert!(empty_app.selected_skill().is_none());
    empty_app.select_next();
    assert_eq!(empty_app.selected_index, 0);
    empty_app.select_prev();
    assert_eq!(empty_app.selected_index, 0);

    let skills = vec![
        create_mock_skill("skill-a", SkillCategory::Engineering, "Skill A description"),
        create_mock_skill("skill-b", SkillCategory::Engineering, "Skill B description"),
    ];
    let mut app = App::with_skills(skills);
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-a");

    app.select_next();
    assert_eq!(app.selected_index, 1);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-b");

    app.select_next();
    assert_eq!(app.selected_index, 1);

    app.select_prev();
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.selected_skill().unwrap().name(), "skill-a");

    app.select_prev();
    assert_eq!(app.selected_index, 0);
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
fn test_app_quit_state() {
    let mut app = App::new();
    assert!(app.is_running());
    app.quit();
    assert!(!app.is_running());
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
