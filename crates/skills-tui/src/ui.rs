//! Layout geometry and active view rendering pipeline.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs};
use ratatui::Frame;

use crate::app::{ActiveView, App};

#[path = "views/explorer.rs"]
pub mod explorer;

#[path = "views/inspector.rs"]
pub mod inspector;

/// Primary rendering entry point drawing the full application UI onto the terminal frame.
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_tab_bar(frame, app, chunks[0]);
    render_viewport(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);
}

/// Renders the top navigation tab bar widget.
pub fn render_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = ActiveView::all()
        .iter()
        .map(|v| Line::from(v.title()))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Agent Skills "),
        )
        .select(app.active_view.index())
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

/// Dispatches active viewport rendering based on `app.active_view`.
pub fn render_viewport(frame: &mut Frame, app: &App, area: Rect) {
    match app.active_view {
        ActiveView::Explorer => render_explorer_view(frame, app, area),
        ActiveView::Inspector => render_inspector_view(frame, app, area),
        ActiveView::Linter => render_linter_view(frame, app, area),
        ActiveView::Runner => render_runner_view(frame, app, area),
    }
}

/// Renders the bottom keybinding shortcuts and status metadata footer.
pub fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(area);

    let shortcuts_text = if app.search_active {
        "Esc: Cancel | Enter: Done | Backspace: Erase | Down/Up: Select | Tab: Switch View | q: Quit"
    } else {
        match app.active_view {
            ActiveView::Explorer => {
                "Tab: Switch View | 1-4: Select | /: Search | Enter: Inspect | j/k: Select | q: Quit"
            }
            ActiveView::Inspector => {
                "Tab: Switch View | 1-4: Select | j/k: Scroll Preview | q: Quit"
            }
            ActiveView::Linter | ActiveView::Runner => {
                "Tab: Switch View | 1-4: Select | /: Search | Esc: Clear | q: Quit"
            }
        }
    };

    let shortcuts = Paragraph::new(shortcuts_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Keybindings "),
        )
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(shortcuts, footer_chunks[0]);

    let total = app.skills.len();
    let filtered = app.filtered_skills().len();
    let status_text = if let Some(notification) = app.latest_notification() {
        format!("Total: {total} | Filtered: {filtered} | Alert: {notification}")
    } else {
        format!("Total: {total} | Filtered: {filtered}")
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(status, footer_chunks[1]);
}

fn render_explorer_view(frame: &mut Frame, app: &App, area: Rect) {
    explorer::render_explorer(frame, app, area);
}

fn render_inspector_view(frame: &mut Frame, app: &App, area: Rect) {
    inspector::render_inspector(frame, app, area);
}

fn render_linter_view(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Linter & Diagnostics ");

    let Some(summary) = &app.summary else {
        let p = Paragraph::new("Telemetry and diagnostic metrics not loaded").block(block);
        frame.render_widget(p, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    let health_score = summary.health.score.clamp(0.0, 100.0);
    let gauge_color = if health_score >= 80.0 {
        Color::Green
    } else if health_score >= 50.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let percent = health_score.round() as u16;

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Overall Health Score "),
        )
        .gauge_style(Style::default().fg(gauge_color))
        .percent(percent)
        .label(format!("{health_score:.1}%"));
    frame.render_widget(gauge, chunks[0]);

    let details = vec![
        Line::from(vec![
            Span::styled(
                "Total Skills: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.total_skills.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Active Skills: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.active_skills.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Inactive Skills: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.inactive_skills.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Clean Skills: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.health.clean_skills.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Diagnostic Errors: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                summary.health.total_errors.to_string(),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Diagnostic Warnings: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                summary.health.total_warnings.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let p = Paragraph::new(details).block(block);
    frame.render_widget(p, chunks[1]);
}

fn render_runner_view(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Execution Runner ");

    let Some(summary) = &app.summary else {
        let p = Paragraph::new("Runner target metrics not loaded").block(block);
        frame.render_widget(p, area);
        return;
    };

    let header = Row::new(vec!["Environment", "Installed", "Active", "Coverage"]).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = summary
        .targets
        .iter()
        .map(|target| {
            Row::new(vec![
                target.environment.identifier().to_string(),
                target.installed_count.to_string(),
                target.active_count.to_string(),
                format!("{:.1}%", target.percentage),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
        ],
    )
    .header(header)
    .block(block);

    frame.render_widget(table, area);
}
