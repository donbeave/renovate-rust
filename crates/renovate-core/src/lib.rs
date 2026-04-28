//! Core domain types for the Rust reimplementation of Renovate.

pub mod branch;
pub mod config;
pub mod datasources;
pub mod extractors;
pub mod http;
pub mod managers;
pub mod platform;
pub mod repo_config;
pub mod schedule;
pub mod versioning;

/// Library version string, sourced from the workspace package version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
