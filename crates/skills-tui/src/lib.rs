//! Terminal user interface for Agent Skills.
//!
//! Provides an asynchronous event loop, resilient terminal lifecycle management,
//! centralized state management, and a multi-view visual layout built on Ratatui.

pub mod app;
pub mod event;
pub mod ui;

pub use app::{init_terminal, install_panic_hook, restore_terminal, ActiveView, App};
pub use event::{Event, EventHandler};
pub use ui::draw;

/// Returns the current crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
