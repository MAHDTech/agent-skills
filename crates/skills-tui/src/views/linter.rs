//! Linter view: diagnostic table, severity badges, remediation guidance, and workspace health card.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table, Wrap};
use ratatui::Frame;
use skills_core::models::{LintIssue, LintSeverity};

use crate::app::App;

/// Primary entry point rendering the full Linter view layout.
pub fn render_linter(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_diagnostics_table(frame, app, chunks[0]);
    render_remediation_pane(frame, app, chunks[1]);
}

/// Renders the scrollable diagnostics table or clean workspace empty state.
pub fn render_diagnostics_table(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(" Diagnostics ({} findings) ", app.linter_diagnostics.len());
    let block = Block::default().borders(Borders::ALL).title(title);

    if app.linter_diagnostics.is_empty() {
        let empty_msg = Paragraph::new(Line::from(vec![
            Span::styled(
                "✔ ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "No static analysis issues detected across workspace skills.",
                Style::default().fg(Color::Green),
            ),
        ]))
        .block(block);
        frame.render_widget(empty_msg, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Severity"),
        Cell::from("Rule"),
        Cell::from("Skill"),
        Cell::from("Location"),
        Cell::from("Summary"),
    ])
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .linter_diagnostics
        .iter()
        .enumerate()
        .map(|(idx, issue)| {
            let skill_name = app
                .skills
                .iter()
                .find(|s| {
                    s.path == issue.file
                        || issue.file.ends_with(&s.path)
                        || s.path.ends_with(&issue.file)
                        || (s.path.parent().is_some()
                            && issue.file.starts_with(s.path.parent().unwrap()))
                        || s.dir_name == issue.file.to_string_lossy()
                })
                .map_or_else(
                    || {
                        issue.file.parent().and_then(|p| p.file_name()).map_or_else(
                            || issue.file.display().to_string(),
                            |n| n.to_string_lossy().to_string(),
                        )
                    },
                    |s| s.name().to_string(),
                );

            let cells = vec![
                Cell::from(Line::from(severity_badge(issue.severity))),
                Cell::from(issue.rule.clone()),
                Cell::from(skill_name),
                Cell::from(format_location(issue)),
                Cell::from(issue.message.clone()),
            ];

            let row = Row::new(cells);
            if idx == app.linter_selected_index {
                row.style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                row
            }
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(11),
            Constraint::Length(22),
            Constraint::Length(20),
            Constraint::Length(12),
            Constraint::Min(20),
        ],
    )
    .header(header)
    .block(block);

    frame.render_widget(table, area);
}

/// Renders the remediation guidance panel and workspace health card.
pub fn render_remediation_pane(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    render_guidance_pane(frame, app, app.selected_linter_issue(), chunks[0]);
    render_health_card(frame, app, chunks[1]);
}

/// Renders prescriptive remediation instructions for the active diagnostic finding.
pub fn render_guidance_pane(frame: &mut Frame, app: &App, issue: Option<&LintIssue>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Diagnostic Remediation ");

    let Some(issue) = issue else {
        let p = Paragraph::new("No diagnostic selected.").block(block);
        frame.render_widget(p, area);
        return;
    };

    let remediation_text = remediation_for_rule(&issue.rule);

    let lines = vec![
        Line::from(vec![
            Span::styled("Rule: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&issue.rule, Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            severity_badge(issue.severity),
        ]),
        Line::from(vec![
            Span::styled("File: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                issue.file.display().to_string(),
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Location: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format_location(issue)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Diagnostic Message:",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("  {}", issue.message)),
        Line::from(""),
        Line::from(Span::styled(
            "Recommended Remediation:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("  {remediation_text}")),
    ];

    #[allow(clippy::cast_possible_truncation)]
    let p = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((app.linter_scroll as u16, 0));

    frame.render_widget(p, area);
}

/// Renders workspace health score gauge, clean skills count, and violation counters.
#[allow(clippy::cast_precision_loss)]
pub fn render_health_card(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Workspace Health ");

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(block.inner(area));

    frame.render_widget(block, area);

    let (score, clean_skills, total_errors, total_warnings) = if let Some(summary) = &app.summary {
        (
            summary.health.score.clamp(0.0, 100.0),
            summary.health.clean_skills,
            summary.health.total_errors,
            summary.health.total_warnings,
        )
    } else {
        let total_skills = app.skills.len();
        let errors = app
            .linter_diagnostics
            .iter()
            .filter(|i| i.severity == LintSeverity::Error)
            .count();
        let warnings = app
            .linter_diagnostics
            .iter()
            .filter(|i| i.severity == LintSeverity::Warning)
            .count();

        let skills_with_issues: std::collections::HashSet<_> =
            app.linter_diagnostics.iter().map(|i| &i.file).collect();
        let clean = total_skills.saturating_sub(skills_with_issues.len());
        let calc_score = if total_skills > 0 {
            (clean as f64 / total_skills as f64) * 100.0
        } else {
            100.0
        };
        (calc_score, clean, errors, warnings)
    };

    let gauge_color = if score >= 80.0 {
        Color::Green
    } else if score >= 50.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let percent = score.round() as u16;

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(gauge_color))
        .percent(percent)
        .label(format!("{score:.1}%"));

    frame.render_widget(gauge, inner_chunks[0]);

    let details = vec![
        Line::from(vec![
            Span::styled(
                "Clean Skills: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(clean_skills.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Diagnostic Errors: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(total_errors.to_string(), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled(
                "Diagnostic Warnings: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                total_warnings.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let p = Paragraph::new(details);
    frame.render_widget(p, inner_chunks[1]);
}

/// Maps static analysis rule identifiers to concrete remediation instructions.
#[must_use]
pub fn remediation_for_rule(rule: &str) -> &'static str {
    match rule {
        "frontmatter-required" => {
            "Add mandatory 'name' and 'description' fields to YAML frontmatter block in SKILL.md."
        }
        "frontmatter-syntax" => "Fix invalid YAML syntax within frontmatter delimiters.",
        "frontmatter-limits" => {
            "Ensure skill name is 64 characters or fewer and description is concise."
        }
        "naming-kebab-case" => {
            "Rename skill directory and frontmatter name to lowercase alphanumeric words separated by hyphens."
        }
        "category-invalid" => {
            "Set skill category to a supported variant (e.g. engineering, tooling, review)."
        }
        "no-em-dashes" => {
            "Replace unicode em-dashes with standard hyphens, colons, or clean punctuation."
        }
        "broken-links" => "Verify all relative markdown links target existing files on disk.",
        "duplicate-names" => {
            "Disambiguate duplicate skill names to maintain unique catalog routing."
        }
        "unsupported-target" => {
            "Update target agent compatibility tags in frontmatter metadata."
        }
        _ => "Inspect the source file at the indicated coordinates and address the diagnostic condition.",
    }
}

/// Formats source file line and column coordinates into a compact string representation.
fn format_location(issue: &LintIssue) -> String {
    match (issue.line, issue.column) {
        (Some(l), Some(c)) => format!("L{l}:{c}"),
        (Some(l), None) => format!("L{l}"),
        _ => "-".to_string(),
    }
}

/// Constructs a colored severity badge widget Span for a diagnostic finding.
fn severity_badge(severity: LintSeverity) -> Span<'static> {
    match severity {
        LintSeverity::Error => Span::styled(
            "[ ERROR ]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        LintSeverity::Warning => Span::styled(
            "[ WARN  ]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    }
}
