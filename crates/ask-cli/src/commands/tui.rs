//! Command handler for the `ask tui` subcommand.

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::time::Duration;

use crate::cli::TuiArgs;
use crate::commands::{resolve_root, CliError};
use skills_core::dashboard::DashboardEngine;
use skills_core::error::SkillError;
use skills_core::parser::SkillParser;
use skills_tui::app::{init_terminal, restore_terminal, ActiveView, App};
use skills_tui::event::{Event, EventHandler};
use skills_tui::ui::draw;

fn parse_active_view(view_str: &str) -> Option<ActiveView> {
    match view_str.to_lowercase().as_str() {
        "explorer" => Some(ActiveView::Explorer),
        "inspector" => Some(ActiveView::Inspector),
        "linter" => Some(ActiveView::Linter),
        "runner" => Some(ActiveView::Runner),
        _ => None,
    }
}

/// Launches the interactive terminal user interface.
pub async fn run(args: TuiArgs) -> Result<(), CliError> {
    let root = resolve_root()?;
    let skills = SkillParser::discover_skills(&root)?;
    let summary = DashboardEngine::new().analyze_repository(&root)?;

    let mut app = App::with_skills(skills).with_summary(summary);

    if let Some(ref view_name) = args.start_view {
        if let Some(view) = parse_active_view(view_name) {
            app.set_tab(view);
        }
    }

    let tick_rate = Duration::from_millis(args.tick_rate.unwrap_or(250));
    let mut terminal =
        init_terminal().map_err(|e| SkillError::GeneralIo(std::io::Error::other(e.to_string())))?;
    let mut events = EventHandler::new(tick_rate);

    let loop_result = run_event_loop(&mut terminal, &mut app, &mut events).await;
    let restore_result =
        restore_terminal().map_err(|e| SkillError::GeneralIo(std::io::Error::other(e.to_string())));

    loop_result.and(restore_result)?;
    Ok(())
}

async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    events: &mut EventHandler,
) -> Result<(), SkillError> {
    while app.is_running() {
        terminal
            .draw(|frame| draw(frame, app))
            .map_err(|e| SkillError::GeneralIo(std::io::Error::other(e.to_string())))?;

        let event = events.next().await.map_err(|e| {
            SkillError::GeneralIo(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                e.to_string(),
            ))
        })?;

        match event {
            Event::Key(key) => {
                app.handle_key_event(key);
            }
            Event::Quit => {
                app.quit();
            }
            Event::Tick | Event::Resize(..) | Event::Mouse(..) => {}
        }
    }

    Ok(())
}
