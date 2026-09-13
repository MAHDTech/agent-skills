//! Terminal lifecycle management, navigation routing, and central application state.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{stdout, Stdout, Write};
use std::panic::{set_hook, take_hook};
use std::path::{Path, PathBuf};

use crossterm::cursor::Show;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use skills_core::dashboard::DashboardSummary;
use skills_core::lint::SkillLinter;
use skills_core::models::{LintIssue, Skill};
use skills_core::parser::{SkillParser, TemplatePlaceholder};

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
    /// State flag indicating whether the interactive search prompt is active.
    pub search_active: bool,
    /// Vertical line scroll offset in the inspector markdown preview.
    pub inspector_scroll: usize,
    /// FIFO queue of transient notification messages.
    pub notifications: Vec<String>,
    /// Execution control flag for the application event loop.
    pub running: bool,
    /// Cursor index of the selected item in lists or tables.
    pub selected_index: usize,

    // Linter state fields
    /// Cursor index of the selected static analysis issue in the Linter view.
    pub linter_selected_index: usize,
    /// Vertical line scroll offset in the diagnostic remediation details panel.
    pub linter_scroll: usize,
    /// Cached collection of static analysis issues detected across workspace skills.
    pub linter_diagnostics: Vec<LintIssue>,

    // Runner state fields
    /// Cursor index of the focused parameter input field in the Runner view.
    pub runner_selected_field: usize,
    /// State flag indicating whether the interactive runner parameter input mode is active.
    pub runner_input_active: bool,
    /// Map of user-supplied parameter values keyed by placeholder identifier.
    pub runner_params: HashMap<String, String>,
    /// Vertical line scroll offset in the rendered prompt preview pane.
    pub runner_scroll: usize,
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
            search_active: false,
            inspector_scroll: 0,
            notifications: Vec::new(),
            running: true,
            selected_index: 0,
            linter_selected_index: 0,
            linter_scroll: 0,
            linter_diagnostics: Vec::new(),
            runner_selected_field: 0,
            runner_input_active: false,
            runner_params: HashMap::new(),
            runner_scroll: 0,
        }
    }

    /// Creates application state pre-populated with a skill catalog.
    #[must_use]
    pub fn with_skills(skills: Vec<Skill>) -> Self {
        let linter_diagnostics = SkillLinter::new().lint_all(&skills).issues;
        Self {
            skills,
            linter_diagnostics,
            ..Self::new()
        }
    }

    /// Configures the telemetry dashboard summary on the state container.
    #[must_use]
    pub fn with_summary(mut self, summary: DashboardSummary) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Activates search mode so typing directs input into the search filter.
    pub fn activate_search(&mut self) {
        self.search_active = true;
    }

    /// Deactivates search mode, returning keyboard focus to navigation.
    pub fn deactivate_search(&mut self) {
        self.search_active = false;
    }

    /// Appends a character to the search filter and resets the selection cursor to 0.
    pub fn push_search_char(&mut self, c: char) {
        self.search_filter.push(c);
        self.selected_index = 0;
    }

    /// Removes the last character from the search filter and resets the selection cursor to 0.
    pub fn pop_search_char(&mut self) {
        self.search_filter.pop();
        self.selected_index = 0;
    }

    /// Decrements the inspector preview scroll offset with lower bound 0.
    pub fn scroll_inspector_up(&mut self) {
        self.inspector_scroll = self.inspector_scroll.saturating_sub(1);
    }

    /// Increments the inspector preview scroll offset.
    pub fn scroll_inspector_down(&mut self) {
        self.inspector_scroll = self.inspector_scroll.saturating_add(1);
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
    #[allow(clippy::too_many_lines)]
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        if key.kind == KeyEventKind::Release {
            return false;
        }

        if self.search_active {
            return match key.code {
                KeyCode::Esc => {
                    self.clear_search_filter();
                    self.deactivate_search();
                    true
                }
                KeyCode::Enter => {
                    self.deactivate_search();
                    true
                }
                KeyCode::Backspace => {
                    self.pop_search_char();
                    true
                }
                KeyCode::Char(c) => {
                    self.push_search_char(c);
                    true
                }
                KeyCode::Down => {
                    self.select_next();
                    true
                }
                KeyCode::Up => {
                    self.select_prev();
                    true
                }
                _ => false,
            };
        }

        if self.active_view == ActiveView::Runner && self.runner_input_active {
            return match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.deactivate_runner_input();
                    true
                }
                KeyCode::Backspace => {
                    self.pop_runner_char();
                    true
                }
                KeyCode::Char(c) => {
                    self.push_runner_char(c);
                    true
                }
                KeyCode::Tab => {
                    self.select_runner_field_next();
                    true
                }
                KeyCode::BackTab => {
                    self.select_runner_field_prev();
                    true
                }
                _ => false,
            };
        }

        if self.active_view == ActiveView::Linter {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.select_linter_next();
                    return true;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.select_linter_prev();
                    return true;
                }
                KeyCode::Enter => {
                    self.jump_to_selected_linter_skill();
                    return true;
                }
                KeyCode::Char('r') => {
                    self.refresh_linter_diagnostics();
                    return true;
                }
                _ => {}
            }
        }

        if self.active_view == ActiveView::Runner {
            match key.code {
                KeyCode::Tab if !self.runner_placeholders().is_empty() => {
                    self.select_runner_field_next();
                    return true;
                }
                KeyCode::Down => {
                    self.select_runner_field_next();
                    return true;
                }
                KeyCode::BackTab if !self.runner_placeholders().is_empty() => {
                    self.select_runner_field_prev();
                    return true;
                }
                KeyCode::Up => {
                    self.select_runner_field_prev();
                    return true;
                }
                KeyCode::Enter | KeyCode::Char('i') => {
                    self.activate_runner_input();
                    return true;
                }
                KeyCode::Char('j') => {
                    self.scroll_runner_down();
                    return true;
                }
                KeyCode::Char('k') => {
                    self.scroll_runner_up();
                    return true;
                }
                KeyCode::Char('c') => {
                    let _ = self.export_runner_prompt_clipboard();
                    return true;
                }
                KeyCode::Char('e') => {
                    let _ = self.export_runner_prompt_file(None);
                    return true;
                }
                KeyCode::Char('r') => {
                    self.reset_runner_params();
                    return true;
                }
                _ => {}
            }
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
            KeyCode::Char('/') if self.active_view == ActiveView::Explorer => {
                self.activate_search();
                true
            }
            KeyCode::Enter if self.active_view == ActiveView::Explorer => {
                if self.selected_skill().is_some() {
                    self.set_tab(ActiveView::Inspector);
                    true
                } else {
                    false
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.active_view == ActiveView::Inspector {
                    self.scroll_inspector_down();
                } else {
                    self.select_next();
                }
                true
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.active_view == ActiveView::Inspector {
                    self.scroll_inspector_up();
                } else {
                    self.select_prev();
                }
                true
            }
            _ => false,
        }
    }

    /// Sets the search filter query, resets selected index to 0, and resets inspector scroll to 0.
    pub fn set_search_filter(&mut self, filter: impl Into<String>) {
        self.search_filter = filter.into();
        self.selected_index = 0;
        self.inspector_scroll = 0;
    }

    /// Clears the search filter query, resets selected index to 0, and resets inspector scroll to 0.
    pub fn clear_search_filter(&mut self) {
        self.search_filter.clear();
        self.selected_index = 0;
        self.inspector_scroll = 0;
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

    /// Moves selection down to the next skill, bounded by the filtered collection size, and resets inspector scroll to 0.
    pub fn select_next(&mut self) {
        let count = self.filtered_skills().len();
        if count > 0 && self.selected_index + 1 < count {
            self.selected_index += 1;
        }
        self.inspector_scroll = 0;
    }

    /// Moves selection up to the previous skill, bounded by index 0, and resets inspector scroll to 0.
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
        self.inspector_scroll = 0;
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

    // ------------------------------------------------------------------------
    // Linter View State Manipulation
    // ------------------------------------------------------------------------

    /// Moves the linter selection cursor down to the next diagnostic issue.
    pub fn select_linter_next(&mut self) {
        if !self.linter_diagnostics.is_empty()
            && self.linter_selected_index + 1 < self.linter_diagnostics.len()
        {
            self.linter_selected_index += 1;
        }
        self.linter_scroll = 0;
    }

    /// Moves the linter selection cursor up to the previous diagnostic issue.
    pub fn select_linter_prev(&mut self) {
        if self.linter_selected_index > 0 {
            self.linter_selected_index -= 1;
        }
        self.linter_scroll = 0;
    }

    /// Returns a reference to the currently selected diagnostic issue, if any.
    #[must_use]
    pub fn selected_linter_issue(&self) -> Option<&LintIssue> {
        self.linter_diagnostics.get(self.linter_selected_index)
    }

    /// Jumps from the selected diagnostic finding to the corresponding skill in the Inspector view.
    /// Returns true if a matching skill was found and navigation succeeded, false otherwise.
    pub fn jump_to_selected_linter_skill(&mut self) -> bool {
        let Some(issue) = self.selected_linter_issue() else {
            return false;
        };

        let issue_file = issue.file.clone();
        let issue_rule = issue.rule.clone();

        let matching_idx = self.skills.iter().position(|s| {
            s.path == issue_file
                || issue_file.ends_with(&s.path)
                || s.path.ends_with(&issue_file)
                || (s.path.parent().is_some() && issue_file.starts_with(s.path.parent().unwrap()))
                || s.dir_name == issue_file.to_string_lossy()
        });

        if let Some(skill_idx) = matching_idx {
            let skill_name = self.skills[skill_idx].name().to_string();
            let skill_path = self.skills[skill_idx].path.clone();

            self.clear_search_filter();
            if let Some(pos) = self
                .filtered_skills()
                .iter()
                .position(|s| s.path == skill_path)
            {
                self.selected_index = pos;
            } else {
                self.selected_index = skill_idx;
            }

            self.active_view = ActiveView::Inspector;
            self.add_notification(format!(
                "Jumped to {skill_name} from diagnostic {issue_rule}"
            ));
            true
        } else {
            self.add_notification(format!(
                "No matching skill found for diagnostic {}",
                issue_file.display()
            ));
            false
        }
    }

    /// Re-evaluates static analysis across all workspace skills and refreshes cached diagnostics.
    pub fn refresh_linter_diagnostics(&mut self) {
        self.linter_diagnostics = SkillLinter::new().lint_all(&self.skills).issues;
        if self.linter_diagnostics.is_empty() {
            self.linter_selected_index = 0;
        } else if self.linter_selected_index >= self.linter_diagnostics.len() {
            self.linter_selected_index = self.linter_diagnostics.len() - 1;
        }
        self.add_notification(format!(
            "Diagnostics refreshed ({} findings)",
            self.linter_diagnostics.len()
        ));
    }

    // ------------------------------------------------------------------------
    // Runner View State Manipulation and Prompt Generation
    // ------------------------------------------------------------------------

    /// Extracts unique template placeholders from the currently selected skill's instructions.
    #[must_use]
    pub fn runner_placeholders(&self) -> Vec<TemplatePlaceholder> {
        let Some(skill) = self.selected_skill() else {
            return Vec::new();
        };
        let placeholders = SkillParser::extract_placeholders(&skill.content);
        let mut seen = std::collections::HashSet::new();
        let mut deduped = Vec::new();
        for p in placeholders {
            if seen.insert(p.name.clone()) {
                deduped.push(p);
            }
        }
        deduped
    }

    /// Advances parameter form focus circularly to the next placeholder field.
    pub fn select_runner_field_next(&mut self) {
        let count = self.runner_placeholders().len();
        if count > 0 {
            self.runner_selected_field = (self.runner_selected_field + 1) % count;
        }
    }

    /// Retreats parameter form focus circularly to the previous placeholder field.
    pub fn select_runner_field_prev(&mut self) {
        let count = self.runner_placeholders().len();
        if count > 0 {
            self.runner_selected_field = if self.runner_selected_field == 0 {
                count - 1
            } else {
                self.runner_selected_field - 1
            };
        }
    }

    /// Activates interactive parameter text editing mode if placeholders exist.
    pub fn activate_runner_input(&mut self) {
        if !self.runner_placeholders().is_empty() {
            self.runner_input_active = true;
        }
    }

    /// Deactivates interactive parameter text editing mode.
    pub fn deactivate_runner_input(&mut self) {
        self.runner_input_active = false;
    }

    /// Appends a typed character to the currently focused parameter value buffer.
    pub fn push_runner_char(&mut self, c: char) {
        let placeholders = self.runner_placeholders();
        if let Some(p) = placeholders.get(self.runner_selected_field) {
            let entry = self.runner_params.entry(p.name.clone()).or_default();
            entry.push(c);
        }
    }

    /// Removes the last character from the currently focused parameter value buffer.
    pub fn pop_runner_char(&mut self) {
        let placeholders = self.runner_placeholders();
        if let Some(p) = placeholders.get(self.runner_selected_field) {
            if let Some(entry) = self.runner_params.get_mut(&p.name) {
                entry.pop();
            }
        }
    }

    /// Sets an explicit parameter value in the runner parameter map.
    pub fn set_runner_param(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.runner_params.insert(key.into(), value.into());
    }

    /// Retrieves the current value of a parameter by key from the runner parameter map.
    #[must_use]
    pub fn get_runner_param(&self, key: &str) -> Option<&str> {
        self.runner_params.get(key).map(String::as_str)
    }

    /// Clears all user-entered runner parameter values and restores template defaults.
    pub fn reset_runner_params(&mut self) {
        self.runner_params.clear();
        self.add_notification("Parameters reset to defaults");
    }

    /// Decrements the prompt preview vertical scroll offset.
    pub fn scroll_runner_up(&mut self) {
        self.runner_scroll = self.runner_scroll.saturating_sub(1);
    }

    /// Increments the prompt preview vertical scroll offset.
    pub fn scroll_runner_down(&mut self) {
        self.runner_scroll = self.runner_scroll.saturating_add(1);
    }

    /// Validates whether all required template parameters without defaults have been populated.
    /// Returns Ok(()) on success, or `Err(missing_names)` listing unsatisfied parameters.
    pub fn validate_runner_params(&self) -> Result<(), Vec<String>> {
        let placeholders = self.runner_placeholders();
        let mut missing = Vec::new();
        for p in placeholders {
            let has_value = self
                .runner_params
                .get(&p.name)
                .is_some_and(|v| !v.trim().is_empty());
            if !has_value && p.default_value.is_none() {
                missing.push(p.name);
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }

    /// Renders the complete prompt text with variable substitutions and fallback tokens applied.
    #[must_use]
    pub fn render_runner_prompt(&self) -> String {
        let Some(skill) = self.selected_skill() else {
            return String::new();
        };
        let placeholders = SkillParser::extract_placeholders(&skill.content);
        let mut rendered = String::with_capacity(skill.content.len());
        let mut last_idx = 0;

        for p in placeholders {
            if p.start >= last_idx && p.end <= skill.content.len() {
                rendered.push_str(&skill.content[last_idx..p.start]);
                if let Some(val) = self.runner_params.get(&p.name).filter(|v| !v.is_empty()) {
                    rendered.push_str(val);
                } else if let Some(ref default_val) = p.default_value {
                    rendered.push_str(default_val);
                } else {
                    let _ = write!(rendered, "[REQUIRED: {}]", p.name);
                }
                last_idx = p.end;
            }
        }

        if last_idx < skill.content.len() {
            rendered.push_str(&skill.content[last_idx..]);
        }
        rendered
    }

    /// Encodes rendered prompt text to base64 and emits an OSC 52 clipboard escape sequence to stdout.
    pub fn export_runner_prompt_clipboard(&mut self) -> color_eyre::Result<()> {
        let prompt = self.render_runner_prompt();
        let b64 = base64_encode(prompt.as_bytes());
        let osc52 = format!("\x1b]52;c;{b64}\x07");
        let mut stdout_handle = stdout();
        stdout_handle.write_all(osc52.as_bytes())?;
        stdout_handle.flush()?;
        self.add_notification("Prompt copied to clipboard via OSC 52");
        Ok(())
    }

    /// Writes the rendered prompt text to a file on disk.
    pub fn export_runner_prompt_file(
        &mut self,
        dest: Option<&Path>,
    ) -> color_eyre::Result<PathBuf> {
        let dest_path = if let Some(p) = dest {
            p.to_path_buf()
        } else if let Some(skill) = self.selected_skill() {
            PathBuf::from(format!("./{}-prompt.txt", skill.dir_name))
        } else {
            PathBuf::from("./prompt.txt")
        };

        let prompt = self.render_runner_prompt();
        std::fs::write(&dest_path, prompt)?;
        self.add_notification(format!("Prompt exported to {}", dest_path.display()));
        Ok(dest_path)
    }
}

/// Encodes raw bytes into an RFC 4648 standard base64 string without external dependencies.
fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);

        let n = (u32::from(b0) << 16) | (u32::from(b1) << 8) | u32::from(b2);
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use skills_core::models::{Skill, SkillCategory, SkillFrontmatter};

    use super::*;

    fn create_test_skill(name: &str) -> Skill {
        let frontmatter = SkillFrontmatter::builder()
            .name(name)
            .description(format!("{name} description"))
            .build();

        Skill::builder()
            .path(PathBuf::from(format!("skills/{name}/SKILL.md")))
            .dir_name(name)
            .category(SkillCategory::Engineering)
            .promoted(false)
            .frontmatter(frontmatter)
            .content(format!("#{name}\nInstructions"))
            .raw(format!("---\nname: {name}\n---\n#{name}"))
            .build()
    }

    #[test]
    fn test_app_search_active_toggle() {
        let mut app = App::new();
        assert!(!app.search_active);

        app.activate_search();
        assert!(app.search_active);

        app.deactivate_search();
        assert!(!app.search_active);
    }

    #[test]
    fn test_app_search_char_push_and_pop() {
        let mut app = App::new();
        app.selected_index = 5;

        app.push_search_char('f');
        assert_eq!(app.search_filter, "f");
        assert_eq!(app.selected_index, 0);

        app.selected_index = 3;
        app.push_search_char('o');
        assert_eq!(app.search_filter, "fo");
        assert_eq!(app.selected_index, 0);

        app.selected_index = 2;
        app.pop_search_char();
        assert_eq!(app.search_filter, "f");
        assert_eq!(app.selected_index, 0);

        app.pop_search_char();
        assert_eq!(app.search_filter, "");
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_inspector_scroll_bounds() {
        let mut app = App::new();
        assert_eq!(app.inspector_scroll, 0);

        app.scroll_inspector_up();
        assert_eq!(app.inspector_scroll, 0);

        app.scroll_inspector_down();
        assert_eq!(app.inspector_scroll, 1);

        app.scroll_inspector_down();
        assert_eq!(app.inspector_scroll, 2);

        app.scroll_inspector_up();
        assert_eq!(app.inspector_scroll, 1);
    }

    #[test]
    fn test_app_selection_change_resets_inspector_scroll() {
        let skills = vec![create_test_skill("skill-1"), create_test_skill("skill-2")];
        let mut app = App::with_skills(skills);

        app.scroll_inspector_down();
        app.scroll_inspector_down();
        assert_eq!(app.inspector_scroll, 2);

        app.select_next();
        assert_eq!(app.inspector_scroll, 0);

        app.scroll_inspector_down();
        assert_eq!(app.inspector_scroll, 1);

        app.select_prev();
        assert_eq!(app.inspector_scroll, 0);

        app.scroll_inspector_down();
        app.set_search_filter("skill");
        assert_eq!(app.inspector_scroll, 0);

        app.scroll_inspector_down();
        app.clear_search_filter();
        assert_eq!(app.inspector_scroll, 0);
    }

    #[test]
    fn test_search_mode_key_events() {
        let mut app = App::new();
        app.activate_search();

        let char_event = KeyEvent {
            code: KeyCode::Char('t'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(char_event));
        assert_eq!(app.search_filter, "t");
        assert!(app.search_active);

        let backspace_event = KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(backspace_event));
        assert_eq!(app.search_filter, "");
        assert!(app.search_active);

        let enter_event = KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(enter_event));
        assert!(!app.search_active);

        app.activate_search();
        app.push_search_char('x');
        let esc_event = KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(esc_event));
        assert_eq!(app.search_filter, "");
        assert!(!app.search_active);
        assert!(app.is_running());
    }

    #[test]
    fn test_explorer_enter_navigates_to_inspector() {
        let mut app = App::new();
        assert_eq!(app.active_view, ActiveView::Explorer);

        let enter_event = KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        // Empty catalog -> returns false
        assert!(!app.handle_key_event(enter_event));
        assert_eq!(app.active_view, ActiveView::Explorer);

        app.skills = vec![create_test_skill("demo")];
        assert!(app.handle_key_event(enter_event));
        assert_eq!(app.active_view, ActiveView::Inspector);
    }

    #[test]
    fn test_inspector_scroll_key_events() {
        let mut app = App::with_skills(vec![
            create_test_skill("demo-1"),
            create_test_skill("demo-2"),
        ]);
        app.set_tab(ActiveView::Inspector);

        let j_event = KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(j_event));
        assert_eq!(app.inspector_scroll, 1);
        assert_eq!(app.selected_index, 0);

        let down_event = KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(down_event));
        assert_eq!(app.inspector_scroll, 2);
        assert_eq!(app.selected_index, 0);

        let k_event = KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(k_event));
        assert_eq!(app.inspector_scroll, 1);
        assert_eq!(app.selected_index, 0);

        let up_event = KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        assert!(app.handle_key_event(up_event));
        assert_eq!(app.inspector_scroll, 0);
        assert_eq!(app.selected_index, 0);
    }
}
