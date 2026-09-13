//! Skill Inspector view: metadata inspection, agent target badges, template parameters, and markdown preview.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;
use skills_core::models::Skill;
use skills_core::parser::SkillParser;

use crate::app::App;

/// Top-level entry point rendering the full Inspector interface.
pub fn render_inspector(frame: &mut Frame, app: &App, area: Rect) {
    let Some(skill) = app.selected_skill() else {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Skill Inspector ");
        let p = Paragraph::new("No skill selected for inspection")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(p, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(11),
            Constraint::Length(4),
            Constraint::Min(6),
        ])
        .split(chunks[0]);

    render_metadata_card(frame, skill, left_chunks[0]);
    render_target_badges(frame, skill, left_chunks[1]);
    render_parameter_table(frame, skill, left_chunks[2]);
    render_markdown_preview(frame, skill, app.inspector_scroll, chunks[1]);
}

/// Renders the left-hand metadata summary card.
pub fn render_metadata_card(frame: &mut Frame, skill: &Skill, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Metadata: {} ", skill.name()));

    let label_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let val_style = Style::default().fg(Color::Cyan);

    let lines = vec![
        Line::from(vec![
            Span::styled("Directory: ", label_style),
            Span::styled(skill.dir_name.as_str(), val_style),
        ]),
        Line::from(vec![
            Span::styled("Path: ", label_style),
            Span::styled(
                skill.path.display().to_string(),
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Category: ", label_style),
            Span::styled(skill.category.as_str(), val_style),
        ]),
        Line::from(vec![
            Span::styled("Origin: ", label_style),
            Span::styled(
                if skill.is_archived() {
                    "Archive"
                } else {
                    "Live"
                },
                val_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Promoted: ", label_style),
            Span::styled(if skill.promoted { "yes" } else { "no" }, val_style),
        ]),
        Line::from(vec![
            Span::styled("User Invoked: ", label_style),
            Span::styled(
                if skill.is_user_invoked() { "yes" } else { "no" },
                val_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Subagent Fork: ", label_style),
            Span::styled(if skill.is_forked() { "yes" } else { "no" }, val_style),
        ]),
        Line::from(vec![
            Span::styled("Resources: ", label_style),
            Span::styled(format!("{} attached", skill.resources.len()), val_style),
        ]),
    ];

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

/// Renders visual pill badges for supported agent targets.
pub fn render_target_badges(frame: &mut Frame, skill: &Skill, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Agent Target Support ");

    let targets = [
        ("claude-desktop", "[ Claude Desktop ]"),
        ("cursor", "[ Cursor ]"),
        ("antigravity", "[ Antigravity ]"),
    ];

    let mut spans = Vec::new();
    for (idx, (id, label)) in targets.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::raw(" "));
        }
        let supported = is_target_supported(skill, id);
        let style = if supported {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(*label, style));
    }

    let p = Paragraph::new(Line::from(spans)).block(block);
    frame.render_widget(p, area);
}

/// Renders the template parameter table dynamically parsed from skill content.
pub fn render_parameter_table(frame: &mut Frame, skill: &Skill, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Template Parameters ");

    let placeholders = SkillParser::extract_placeholders(&skill.content);
    if placeholders.is_empty() {
        let p = Paragraph::new("No template parameters defined in skill instructions.")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(p, area);
        return;
    }

    let header = Row::new(vec!["Parameter", "Default", "Line"]).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = placeholders
        .iter()
        .map(|param| {
            Row::new(vec![
                Cell::from(Span::styled(
                    param.name.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    param.default_value.as_deref().unwrap_or("-").to_string(),
                    Style::default().fg(Color::Yellow),
                )),
                Cell::from(Span::styled(
                    format!("L{}", param.line),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(45),
            Constraint::Percentage(40),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(block);

    frame.render_widget(table, area);
}

/// Renders the syntax-styled markdown instruction preview pane.
pub fn render_markdown_preview(frame: &mut Frame, skill: &Skill, scroll_offset: usize, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Instructions Preview ({}) ", skill.name()));

    let lines = parse_markdown_lines(&skill.content);
    #[allow(clippy::cast_possible_truncation)]
    let scroll_y = scroll_offset.min(u16::MAX as usize) as u16;

    let paragraph = Paragraph::new(lines).block(block).scroll((scroll_y, 0));

    frame.render_widget(paragraph, area);
}

/// Checks whether a given target agent environment is supported for a skill.
#[must_use]
pub fn is_target_supported(skill: &Skill, target_id: &str) -> bool {
    if skill.is_archived() {
        return false;
    }

    if let Some(meta) = &skill.frontmatter.metadata {
        if let Some(targets) = meta.get("targets") {
            return targets.contains(target_id);
        }
    }

    if let Some(extra_val) = skill.frontmatter.extra.get("targets") {
        if let Some(seq) = extra_val.as_sequence() {
            return seq
                .iter()
                .any(|v| v.as_str().is_some_and(|s| s == target_id));
        }
        if let Some(s) = extra_val.as_str() {
            return s.contains(target_id);
        }
    }

    true
}

/// Parses markdown text lines into styled Ratatui Line structs.
#[must_use]
pub fn parse_markdown_lines(content: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    let code_style = Style::default()
        .fg(Color::LightGreen)
        .bg(Color::Rgb(30, 30, 30));
    let fence_style = Style::default().fg(Color::DarkGray);

    for raw_line in content.lines() {
        let trimmed = raw_line.trim_start();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            lines.push(Line::from(Span::styled(raw_line.to_string(), fence_style)));
            continue;
        }

        if in_code_block {
            lines.push(Line::from(Span::styled(raw_line.to_string(), code_style)));
            continue;
        }

        if raw_line.trim().is_empty() {
            lines.push(Line::from(""));
            continue;
        }

        if let Some(rest) = raw_line.strip_prefix("# ") {
            let style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
            lines.push(Line::from(parse_inline_spans(rest, style)));
        } else if let Some(rest) = raw_line.strip_prefix("## ") {
            let style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            lines.push(Line::from(parse_inline_spans(rest, style)));
        } else if let Some(rest) = raw_line.strip_prefix("### ") {
            let style = Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD);
            lines.push(Line::from(parse_inline_spans(rest, style)));
        } else if trimmed.starts_with("####") {
            let heading_text = trimmed.trim_start_matches('#').trim_start();
            let style = Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD);
            lines.push(Line::from(parse_inline_spans(heading_text, style)));
        } else if let Some(rest) = trimmed.strip_prefix("> ") {
            let mut spans = vec![Span::styled("│ ", Style::default().fg(Color::DarkGray))];
            let body_style = Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC);
            spans.extend(parse_inline_spans(rest, body_style));
            lines.push(Line::from(spans));
        } else if trimmed == ">" {
            lines.push(Line::from(Span::styled(
                "│ ",
                Style::default().fg(Color::DarkGray),
            )));
        } else if let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            let mut spans = vec![Span::styled("  • ", Style::default().fg(Color::Cyan))];
            let body_style = Style::default().fg(Color::White);
            spans.extend(parse_inline_spans(rest, body_style));
            lines.push(Line::from(spans));
        } else {
            let body_style = Style::default().fg(Color::White);
            lines.push(Line::from(parse_inline_spans(raw_line, body_style)));
        }
    }

    lines
}

/// Tokenizes text for backtick delimited inline code spans, applying distinct styles.
#[must_use]
pub fn parse_inline_spans(text: &str, base_style: Style) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let code_style = Style::default()
        .fg(Color::LightYellow)
        .bg(Color::Rgb(40, 40, 40))
        .add_modifier(Modifier::BOLD);

    for (idx, segment) in text.split('`').enumerate() {
        if segment.is_empty() {
            continue;
        }
        if idx % 2 == 1 {
            spans.push(Span::styled(segment.to_string(), code_style));
        } else {
            spans.push(Span::styled(segment.to_string(), base_style));
        }
    }

    spans
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use skills_core::models::{Skill, SkillCategory, SkillFrontmatter, SkillTree};

    use super::*;

    fn create_test_skill(name: &str, content: &str) -> Skill {
        let frontmatter = SkillFrontmatter::builder()
            .name(name)
            .description("Inspector test description")
            .build();

        Skill::builder()
            .path(PathBuf::from(format!("skills/{name}/SKILL.md")))
            .dir_name(name)
            .category(SkillCategory::Engineering)
            .promoted(true)
            .frontmatter(frontmatter)
            .content(content.to_string())
            .raw(format!("---\nname: {name}\n---\n{content}"))
            .build()
    }

    #[test]
    fn test_template_parameters_extraction() {
        let markdown = "Run command on branch {{branch:-main}} with author {{author}}.";
        let skill = create_test_skill("demo", markdown);

        let placeholders = SkillParser::extract_placeholders(&skill.content);
        assert_eq!(placeholders.len(), 2);
        assert_eq!(placeholders[0].name, "branch");
        assert_eq!(placeholders[0].default_value.as_deref(), Some("main"));
        assert_eq!(placeholders[0].line, 1);
        assert_eq!(placeholders[1].name, "author");
        assert_eq!(placeholders[1].default_value, None);
        assert_eq!(placeholders[1].line, 1);
    }

    #[test]
    fn test_markdown_preview_styling() {
        let content = "\
# Heading One
## Heading Two
### Heading Three
#### Heading Four
> Blockquote text
- List item with `code`
```rust
fn main() {}
```
Normal paragraph";

        let lines = parse_markdown_lines(content);
        assert_eq!(lines.len(), 10);

        // Heading 1 style check (bold Yellow)
        assert_eq!(lines[0].spans[0].content, "Heading One");
        assert_eq!(lines[0].spans[0].style.fg, Some(Color::Yellow));

        // Heading 2 style check (bold Cyan)
        assert_eq!(lines[1].spans[0].content, "Heading Two");
        assert_eq!(lines[1].spans[0].style.fg, Some(Color::Cyan));

        // Heading 3 style check (bold White)
        assert_eq!(lines[2].spans[0].content, "Heading Three");
        assert_eq!(lines[2].spans[0].style.fg, Some(Color::White));

        // Heading 4 style check (bold White)
        assert_eq!(lines[3].spans[0].content, "Heading Four");
        assert_eq!(lines[3].spans[0].style.fg, Some(Color::White));

        // Blockquote check
        assert_eq!(lines[4].spans[0].content, "│ ");
        assert_eq!(lines[4].spans[1].content, "Blockquote text");

        // List item with inline code check
        assert_eq!(lines[5].spans[0].content, "  • ");
        assert_eq!(lines[5].spans[1].content, "List item with ");
        assert_eq!(lines[5].spans[2].content, "code");
        assert_eq!(lines[5].spans[2].style.fg, Some(Color::LightYellow));

        // Code fence check
        assert_eq!(lines[6].spans[0].content, "```rust");
        assert_eq!(lines[6].spans[0].style.fg, Some(Color::DarkGray));

        // Code block body check
        assert_eq!(lines[7].spans[0].content, "fn main() {}");
        assert_eq!(lines[7].spans[0].style.fg, Some(Color::LightGreen));

        // Closing fence
        assert_eq!(lines[8].spans[0].content, "```");
        assert_eq!(lines[8].spans[0].style.fg, Some(Color::DarkGray));

        // Normal paragraph check
        assert_eq!(lines[9].spans[0].content, "Normal paragraph");
        assert_eq!(lines[9].spans[0].style.fg, Some(Color::White));
    }

    #[test]
    fn test_is_target_supported() {
        let mut skill = create_test_skill("demo", "Content");
        assert!(is_target_supported(&skill, "claude-desktop"));
        assert!(is_target_supported(&skill, "cursor"));
        assert!(is_target_supported(&skill, "antigravity"));

        skill.tree = SkillTree::Archive;
        assert!(!is_target_supported(&skill, "claude-desktop"));

        skill.tree = SkillTree::Live;
        let mut meta = HashMap::new();
        meta.insert("targets".to_string(), "cursor, antigravity".to_string());
        skill.frontmatter.metadata = Some(meta);

        assert!(!is_target_supported(&skill, "claude-desktop"));
        assert!(is_target_supported(&skill, "cursor"));
        assert!(is_target_supported(&skill, "antigravity"));
    }

    #[test]
    fn test_inspector_render_headless() {
        let backend = TestBackend::new(120, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new();
        terminal
            .draw(|f| render_inspector(f, &app, f.area()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let content: String = buffer
            .content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect();
        assert!(content.contains("No skill selected for inspection"));

        let skill = create_test_skill("demo", "# Instructions\nUse parameter {{var}}.");
        app.skills = vec![skill];
        terminal
            .draw(|f| render_inspector(f, &app, f.area()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let content: String = buffer
            .content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect();
        assert!(content.contains("Metadata: demo"));
        assert!(content.contains("Agent Target Support"));
        assert!(content.contains("Template Parameters"));
        assert!(content.contains("Instructions Preview (demo)"));
    }
}
