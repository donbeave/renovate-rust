//! Core domain types for the Rust reimplementation of Renovate.

pub mod config;
pub mod http;
pub mod platform;

/// Library version string, sourced from the workspace package version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
