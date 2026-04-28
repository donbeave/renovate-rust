//! Core domain types for the Rust reimplementation of Renovate.

pub mod config;

/// Library version string, sourced from the workspace package version.
///
/// Exposed so the CLI can render a single canonical version string regardless
/// of which crate it queries.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
