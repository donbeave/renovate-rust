//! Core domain types for the Rust reimplementation of Renovate.
//!
//! This crate is intentionally thin during initial scaffolding. Modules will
//! be added as parity slices (config, managers, datasources, versioning,
//! workers) are implemented.

/// Library version string, sourced from the workspace package version.
///
/// Exposed so the CLI can render a single canonical version string regardless
/// of which crate it queries.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
