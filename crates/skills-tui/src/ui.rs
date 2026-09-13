//! Layout geometry and active view rendering pipeline.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{ActiveView, App};

#[path = "views/explorer.rs"]
pub mod explorer;

#[path = "views/inspector.rs"]
pub mod inspector;

#[path = "views/linter.rs"]
pub mod linter;

#[path = "views/runner.rs"]
pub mod runner;

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
        ActiveView::Linter => linter::render_linter(frame, app, area),
        ActiveView::Runner => runner::render_runner(frame, app, area),
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
            ActiveView::Linter => {
                "Tab: Switch View | 1-4: Select | j/k: Select Issue | Enter: Jump to Skill | r: Rescan | q: Quit"
            }
            ActiveView::Runner => {
                if app.runner_input_active {
                    "Enter/Esc: Done | Backspace: Erase | Tab: Next Field"
                } else {
                    "Tab: Field | Enter/i: Edit | c: Copy Prompt | e: Export File | j/k: Scroll | r: Reset | q: Quit"
                }
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
