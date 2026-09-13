//! Skill Explorer view: interactive catalog table, search input, and status badges.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;
use skills_core::models::Skill;

use crate::app::App;

/// Top-level entry point rendering the full Explorer interface.
pub fn render_explorer(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    render_search_bar(frame, app, chunks[0]);
    render_skills_table(frame, app, chunks[1]);
}

/// Renders the interactive search query input container.
pub fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let (block, line) = if app.search_active {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Search [Active] ");
        let line = Line::from(vec![
            Span::styled(
                "❯ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &app.search_filter,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(Color::Cyan)),
        ]);
        (block, line)
    } else {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Search [/] ");
        let line = if app.search_filter.is_empty() {
            Line::from(vec![
                Span::styled("❯ ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Press '/' to search skills...",
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled("❯ ", Style::default().fg(Color::DarkGray)),
                Span::styled(&app.search_filter, Style::default().fg(Color::White)),
            ])
        };
        (block, line)
    };

    let p = Paragraph::new(line).block(block);
    frame.render_widget(p, area);
}

/// Renders the skills catalog table or delegates to empty state if filtered list is empty.
pub fn render_skills_table(frame: &mut Frame, app: &App, area: Rect) {
    let skills = app.filtered_skills();
    if skills.is_empty() {
        render_empty_state(frame, app, area);
        return;
    }

    let header = Row::new(vec![
        "Status",
        "Skill Name",
        "Version",
        "Author",
        "Category",
        "Description",
    ])
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = skills
        .iter()
        .enumerate()
        .map(|(idx, skill)| {
            let is_selected = idx == app.selected_index;
            let row_style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(format_status_badges(skill)),
                Cell::from(skill.name()),
                Cell::from(extract_version(skill)),
                Cell::from(extract_author(skill)),
                Cell::from(skill.category.as_str()),
                Cell::from(skill.description()),
            ])
            .style(row_style)
        })
        .collect();

    let title = format!(" Skills ({}) ", skills.len());
    let table = Table::new(
        rows,
        [
            Constraint::Length(9),
            Constraint::Length(24),
            Constraint::Length(9),
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Min(20),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(table, area);
}

/// Renders a centered diagnostic fallback when no skills match the search query.
pub fn render_empty_state(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Skill Explorer ");
    let message = if app.search_filter.is_empty() {
        "No skills loaded in catalog".to_string()
    } else {
        format!("No skills found matching filter: {}", app.search_filter)
    };
    let paragraph = Paragraph::new(message)
        .block(block)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}

/// Formats compact status indicators as styled spans inside a Line.
#[must_use]
pub fn format_status_badges(skill: &Skill) -> Line<'static> {
    let mut spans = Vec::new();

    if skill.promoted {
        spans.push(Span::styled(
            "[P]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if skill.is_user_invoked() {
        spans.push(Span::styled(
            "[U]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if skill.is_forked() {
        spans.push(Span::styled(
            "[F]",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if skill.is_archived() {
        spans.push(Span::styled("[A]", Style::default().fg(Color::DarkGray)));
    }
    if skill.yaml_error.is_some() {
        spans.push(Span::styled(
            "[E]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    }

    if spans.is_empty() {
        spans.push(Span::styled("[-]", Style::default().fg(Color::DarkGray)));
    }

    Line::from(spans)
}

/// Extracts version metadata from frontmatter metadata or extra attributes.
#[must_use]
pub fn extract_version(skill: &Skill) -> String {
    if let Some(meta) = &skill.frontmatter.metadata {
        if let Some(v) = meta.get("version") {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    if let Some(val) = skill.frontmatter.extra.get("version") {
        if let Some(s) = val.as_str() {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        } else if let Some(i) = val.as_i64() {
            return i.to_string();
        } else if let Some(f) = val.as_f64() {
            return f.to_string();
        }
    }

    "-".to_string()
}

/// Extracts author metadata from frontmatter metadata or extra attributes.
#[must_use]
pub fn extract_author(skill: &Skill) -> String {
    if let Some(meta) = &skill.frontmatter.metadata {
        if let Some(a) = meta.get("author") {
            let trimmed = a.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    if let Some(val) = skill.frontmatter.extra.get("author") {
        if let Some(s) = val.as_str() {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    "-".to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use skills_core::models::{Skill, SkillCategory, SkillFrontmatter, SkillTree};

    use super::*;

    fn create_test_skill(name: &str, category: SkillCategory) -> Skill {
        let frontmatter = SkillFrontmatter::builder()
            .name(name)
            .description("Test skill description")
            .build();

        Skill::builder()
            .path(PathBuf::from(format!("skills/{name}/SKILL.md")))
            .dir_name(name)
            .category(category)
            .promoted(false)
            .frontmatter(frontmatter)
            .content("Test content".to_string())
            .raw(format!("---\nname: {name}\n---\nTest content"))
            .build()
    }

    #[test]
    fn test_extract_version_and_author() {
        let mut skill = create_test_skill("demo", SkillCategory::Engineering);
        assert_eq!(extract_version(&skill), "-");
        assert_eq!(extract_author(&skill), "-");

        let mut meta = HashMap::new();
        meta.insert("version".to_string(), "1.2.0".to_string());
        meta.insert("author".to_string(), "Alice".to_string());
        skill.frontmatter.metadata = Some(meta);

        assert_eq!(extract_version(&skill), "1.2.0");
        assert_eq!(extract_author(&skill), "Alice");
    }

    #[test]
    fn test_format_status_badges() {
        let mut skill = create_test_skill("demo", SkillCategory::Engineering);
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[-]");

        skill.promoted = true;
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[P]");

        skill.frontmatter.disable_model_invocation = Some(true);
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[P][U]");

        skill.frontmatter.context = Some("fork".to_string());
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[P][U][F]");

        skill.tree = SkillTree::Archive;
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[P][U][F][A]");

        skill.yaml_error = Some("Invalid syntax".to_string());
        let badges = format_status_badges(&skill);
        let line_text: String = badges.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(line_text, "[P][U][F][A][E]");

        // Individual flags
        let mut skill_u = create_test_skill("u", SkillCategory::Engineering);
        skill_u.frontmatter.disable_model_invocation = Some(true);
        assert_eq!(format_status_badges(&skill_u).spans[0].content, "[U]");

        let mut skill_f = create_test_skill("f", SkillCategory::Engineering);
        skill_f.frontmatter.context = Some("fork".to_string());
        assert_eq!(format_status_badges(&skill_f).spans[0].content, "[F]");

        let mut skill_a = create_test_skill("a", SkillCategory::Engineering);
        skill_a.tree = SkillTree::Archive;
        assert_eq!(format_status_badges(&skill_a).spans[0].content, "[A]");

        let mut skill_e = create_test_skill("e", SkillCategory::Engineering);
        skill_e.yaml_error = Some("error".to_string());
        assert_eq!(format_status_badges(&skill_e).spans[0].content, "[E]");
    }

    #[test]
    fn test_explorer_render_headless() {
        let backend = TestBackend::new(120, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new();
        terminal
            .draw(|f| render_explorer(f, &app, f.area()))
            .unwrap();

        let skill = create_test_skill("alpha", SkillCategory::Tooling);
        app.skills = vec![skill];
        app.activate_search();
        app.push_search_char('a');
        terminal
            .draw(|f| render_explorer(f, &app, f.area()))
            .unwrap();

        let buffer = terminal.backend().buffer();
        let content: String = buffer
            .content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect();
        assert!(content.contains("Search [Active]"));
        assert!(content.contains("❯"));
        assert!(content.contains("█"));
        assert!(content.contains("alpha"));
        assert!(content.contains("Status"));
        assert!(content.contains("Skill Name"));
        assert!(content.contains("Version"));
        assert!(content.contains("Author"));
        assert!(content.contains("Category"));
        assert!(content.contains("Description"));
    }
}
