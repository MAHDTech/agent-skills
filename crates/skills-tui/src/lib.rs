//! Terminal user interface for Agent Skills.

/// Returns the current crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
