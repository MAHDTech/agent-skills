//! Terminal lifecycle management, navigation routing, and central application state.

use std::io::{stdout, Stdout};
use std::panic::{set_hook, take_hook};

use crossterm::cursor::Show;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use skills_core::dashboard::DashboardSummary;
use skills_core::models::Skill;

/// Configures stdout in raw alternate screen mode with mouse tracking and panic protection.
pub fn init_terminal() -> color_eyre::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout_handle = stdout();
    execute!(stdout_handle, EnterAlternateScreen, EnableMouseCapture)?;
    install_panic_hook();
    let backend = CrosstermBackend::new(stdout_handle);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

/// Restores the terminal to canonical line-buffered state and re-enables cursor display.
pub fn restore_terminal() -> color_eyre::Result<()> {
    let mut stdout_handle = stdout();
    execute!(
        stdout_handle,
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    disable_raw_mode()?;
    Ok(())
}

/// Installs a panic hook that restores canonical terminal state before printing the panic diagnostic.
pub fn install_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}

/// Primary navigation tabs available within the terminal user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveView {
    /// Skill catalog overview with search query filtering.
    #[default]
    Explorer,
    /// Detailed skill metadata, frontmatter, and attached resource inspector.
    Inspector,
    /// Static analysis diagnostics, quality score gauge, and lint issues.
    Linter,
    /// Execution environment target distribution and installation sync tracker.
    Runner,
}

impl ActiveView {
    /// Returns the human-readable identifier for the view.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Explorer => "Explorer",
            Self::Inspector => "Inspector",
            Self::Linter => "Linter",
            Self::Runner => "Runner",
        }
    }

    /// Returns the tab title including its numeric hotkey label.
    #[must_use]
    pub fn title(&self) -> &'static str {
        match self {
            Self::Explorer => "[1] Explorer",
            Self::Inspector => "[2] Inspector",
            Self::Linter => "[3] Linter",
            Self::Runner => "[4] Runner",
        }
    }

    /// Returns the zero-based numeric index of the view.
    #[must_use]
    pub fn index(&self) -> usize {
        match self {
            Self::Explorer => 0,
            Self::Inspector => 1,
            Self::Linter => 2,
            Self::Runner => 3,
        }
    }

    /// Constructs an `ActiveView` from a numeric index with circular modulo wrapping.
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index % 4 {
            0 => Self::Explorer,
            1 => Self::Inspector,
            2 => Self::Linter,
            _ => Self::Runner,
        }
    }

    /// Advances circularly to the next view tab.
    #[must_use]
    pub fn next(&self) -> Self {
        Self::from_index(self.index() + 1)
    }

    /// Retreats circularly to the previous view tab.
    #[must_use]
    pub fn prev(&self) -> Self {
        Self::from_index(self.index() + 3)
    }

    /// Returns a static slice of all four views in canonical order.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::Explorer, Self::Inspector, Self::Linter, Self::Runner]
    }
}

/// Central application state container managing navigation, loaded catalog, and user selections.
#[derive(Debug, Clone)]
pub struct App {
    /// Currently active navigation tab.
    pub active_view: ActiveView,
    /// Complete catalog of loaded skills.
    pub skills: Vec<Skill>,
    /// Optional workspace telemetry and static analysis summary.
    pub summary: Option<DashboardSummary>,
    /// Active search filter query string.
    pub search_filter: String,
    /// FIFO queue of transient notification messages.
    pub notifications: Vec<String>,
    /// Execution control flag for the application event loop.
    pub running: bool,
    /// Cursor index of the selected item in lists or tables.
    pub selected_index: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates default application state with empty collections and Explorer view.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::default(),
            skills: Vec::new(),
            summary: None,
            search_filter: String::new(),
            notifications: Vec::new(),
            running: true,
            selected_index: 0,
        }
    }

    /// Creates application state pre-populated with a skill catalog.
    #[must_use]
    pub fn with_skills(skills: Vec<Skill>) -> Self {
        Self {
            skills,
            ..Self::new()
        }
    }

    /// Configures the telemetry dashboard summary on the state container.
    #[must_use]
    pub fn with_summary(mut self, summary: DashboardSummary) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Advances circularly to the next view tab.
    pub fn next_tab(&mut self) {
        self.active_view = self.active_view.next();
    }

    /// Retreats circularly to the previous view tab.
    pub fn prev_tab(&mut self) {
        self.active_view = self.active_view.prev();
    }

    /// Sets the active view tab directly.
    pub fn set_tab(&mut self, view: ActiveView) {
        self.active_view = view;
    }

    /// Handles keyboard events, updating navigation tab, selection cursor, or termination flag.
    ///
    /// Discards release events and unmapped keys, returning `false`.
    /// Returns `true` if the key event produced a state transition.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        if key.kind == KeyEventKind::Release {
            return false;
        }

        match key.code {
            KeyCode::Tab => {
                self.next_tab();
                true
            }
            KeyCode::BackTab => {
                self.prev_tab();
                true
            }
            KeyCode::Char('1') => {
                self.set_tab(ActiveView::Explorer);
                true
            }
            KeyCode::Char('2') => {
                self.set_tab(ActiveView::Inspector);
                true
            }
            KeyCode::Char('3') => {
                self.set_tab(ActiveView::Linter);
                true
            }
            KeyCode::Char('4') => {
                self.set_tab(ActiveView::Runner);
                true
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.quit();
                true
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                true
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_prev();
                true
            }
            _ => false,
        }
    }

    /// Sets the search filter query and resets selected index to 0.
    pub fn set_search_filter(&mut self, filter: impl Into<String>) {
        self.search_filter = filter.into();
        self.selected_index = 0;
    }

    /// Clears the search filter query and resets selected index to 0.
    pub fn clear_search_filter(&mut self) {
        self.search_filter.clear();
        self.selected_index = 0;
    }

    /// Returns references to all skills matching the search filter query across name, dir, category, or description.
    #[must_use]
    pub fn filtered_skills(&self) -> Vec<&Skill> {
        let query = self.search_filter.trim().to_lowercase();
        if query.is_empty() {
            return self.skills.iter().collect();
        }

        self.skills
            .iter()
            .filter(|skill| {
                skill.name().to_lowercase().contains(&query)
                    || skill.dir_name.to_lowercase().contains(&query)
                    || skill.category.as_str().to_lowercase().contains(&query)
                    || skill.description().to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Returns a reference to the currently selected skill in the filtered list, or None if empty.
    #[must_use]
    pub fn selected_skill(&self) -> Option<&Skill> {
        let filtered = self.filtered_skills();
        if filtered.is_empty() {
            None
        } else {
            filtered.get(self.selected_index).copied()
        }
    }

    /// Moves selection down to the next skill, bounded by the filtered collection size.
    pub fn select_next(&mut self) {
        let count = self.filtered_skills().len();
        if count > 0 && self.selected_index + 1 < count {
            self.selected_index += 1;
        }
    }

    /// Moves selection up to the previous skill, bounded by index 0.
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Enqueues a transient status notification message.
    pub fn add_notification(&mut self, message: impl Into<String>) {
        self.notifications.push(message.into());
    }

    /// Dequeues and returns the oldest status notification in FIFO order.
    pub fn dismiss_notification(&mut self) -> Option<String> {
        if self.notifications.is_empty() {
            None
        } else {
            Some(self.notifications.remove(0))
        }
    }

    /// Returns a reference to the latest notification message, if any.
    #[must_use]
    pub fn latest_notification(&self) -> Option<&str> {
        self.notifications.last().map(String::as_str)
    }

    /// Sets the running flag to false to stop the event loop.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Returns true if the application event loop is running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }
}
