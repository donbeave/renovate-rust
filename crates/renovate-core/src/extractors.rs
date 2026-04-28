//! Dependency extractors for each supported package manager.
//!
//! Each sub-module parses a specific manifest format and returns a list of
//! [`ExtractedDep`] records suitable for datasource version lookups.

pub mod cargo;
pub mod docker_compose;
pub mod dockerfile;
pub mod github_actions;
pub mod maven;
pub mod npm;
pub mod pep621;
pub mod pip;
