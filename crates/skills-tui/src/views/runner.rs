//! Execution Runner view: parameter input form, live prompt preview, validation status, and exports.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use skills_core::models::Skill;
use skills_core::parser::TemplatePlaceholder;

use crate::app::App;

/// Primary entry point rendering the execution runner interface.
pub fn render_runner(frame: &mut Frame, app: &App, area: Rect) {
    let Some(skill) = app.selected_skill() else {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Execution Runner ");
        let empty_msg =
            Paragraph::new("No skill selected for execution. Select a skill in the Explorer view.")
                .style(Style::default().fg(Color::DarkGray))
                .block(block);
        frame.render_widget(empty_msg, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_runner_form(frame, app, skill, chunks[0]);
    render_prompt_preview(frame, app, skill, chunks[1]);
}

/// Renders the parameter form input panel including metadata and validation status.
pub fn render_runner_form(frame: &mut Frame, app: &App, skill: &Skill, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    render_skill_metadata(frame, skill, chunks[0]);
    let placeholders = app.runner_placeholders();
    render_parameter_inputs(frame, app, &placeholders, chunks[1]);
    render_validation_card(frame, app, chunks[2]);
}

/// Renders the skill target header displaying name, path, and category.
pub fn render_skill_metadata(frame: &mut Frame, skill: &Skill, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Target Skill ");

    let lines = vec![
        Line::from(vec![
            Span::styled("Skill: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                skill.name(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Path: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                skill.path.display().to_string(),
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(skill.category.as_str(), Style::default().fg(Color::Green)),
        ]),
    ];

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

/// Renders the list of interactive parameter form input fields.
pub fn render_parameter_inputs(
    frame: &mut Frame,
    app: &App,
    placeholders: &[TemplatePlaceholder],
    area: Rect,
) {
    let block = Block::default().borders(Borders::ALL).title(" Parameters ");

    if placeholders.is_empty() {
        let p = Paragraph::new("Skill instructions do not define any {{template}} parameters.")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(p, area);
        return;
    }

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let constraints: Vec<Constraint> = placeholders.iter().map(|_| Constraint::Length(3)).collect();

    let field_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (idx, p) in placeholders.iter().enumerate() {
        if idx >= field_chunks.len() {
            break;
        }

        let is_selected = idx == app.runner_selected_field;
        let border_color = if is_selected {
            Color::Cyan
        } else {
            Color::DarkGray
        };

        let field_title = if let Some(ref def) = p.default_value {
            format!(" {} (default: {}) ", p.name, def)
        } else {
            format!(" {}* ", p.name)
        };

        let field_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(field_title);

        let user_val = app.get_runner_param(&p.name);
        let val_text = match (user_val, &p.default_value) {
            (Some(v), _) => v.to_string(),
            (None, Some(def)) => def.clone(),
            (None, None) => String::new(),
        };

        let content_line = if is_selected && app.runner_input_active {
            Line::from(vec![
                Span::raw(val_text),
                Span::styled("█", Style::default().fg(Color::Cyan)),
            ])
        } else if val_text.is_empty() && p.default_value.is_none() {
            Line::from(Span::styled(
                "<required>",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            ))
        } else {
            Line::from(val_text)
        };

        let input_p = Paragraph::new(content_line).block(field_block);
        frame.render_widget(input_p, field_chunks[idx]);
    }
}

/// Renders the parameter validation status banner card.
pub fn render_validation_card(frame: &mut Frame, app: &App, area: Rect) {
    match app.validate_runner_params() {
        Ok(()) => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Validation ");
            let p = Paragraph::new(Line::from(vec![
                Span::styled(
                    "✔ ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Ready to run: All parameters provided",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]))
            .block(block);
            frame.render_widget(p, area);
        }
        Err(missing) => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Validation ");
            let p = Paragraph::new(Line::from(vec![
                Span::styled(
                    "✖ ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("Missing required: {}", missing.join(", ")),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]))
            .block(block);
            frame.render_widget(p, area);
        }
    }
}

/// Renders the syntax-styled live prompt preview pane.
pub fn render_prompt_preview(frame: &mut Frame, app: &App, skill: &Skill, area: Rect) {
    let title = format!(" Live Prompt Preview ({}) ", skill.name());
    let block = Block::default().borders(Borders::ALL).title(title);

    let prompt = app.render_runner_prompt();
    let lines: Vec<Line<'static>> = prompt.lines().map(style_prompt_line).collect();

    #[allow(clippy::cast_possible_truncation)]
    let p = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.runner_scroll as u16, 0));

    frame.render_widget(p, area);
}

/// Parses markdown structures in prompt lines and applies visual syntax styling.
#[must_use]
pub fn style_prompt_line(line: &str) -> Line<'static> {
    if line.starts_with("# ") {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if line.starts_with("## ") {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if line.starts_with("### ") || line.starts_with("#### ") {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if line.starts_with("```") {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(Color::LightGreen).bg(Color::DarkGray),
        ));
    }
    if let Some(stripped) = line.strip_prefix('>') {
        return Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                stripped.trim_start().to_string(),
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]);
    }

    if line.contains("[REQUIRED:") {
        let mut spans = Vec::new();
        let mut rem = line;
        while let Some(start_pos) = rem.find("[REQUIRED:") {
            if start_pos > 0 {
                spans.push(Span::raw(rem[..start_pos].to_string()));
            }
            let sub = &rem[start_pos..];
            if let Some(end_pos) = sub.find(']') {
                let token = &sub[..=end_pos];
                spans.push(Span::styled(
                    token.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ));
                rem = &sub[end_pos + 1..];
            } else {
                spans.push(Span::styled(
                    sub.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ));
                rem = "";
                break;
            }
        }
        if !rem.is_empty() {
            spans.push(Span::raw(rem.to_string()));
        }
        return Line::from(spans);
    }

    Line::from(line.to_string())
}
