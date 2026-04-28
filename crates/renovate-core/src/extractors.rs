//! Dependency extractors for each supported package manager.
//!
//! Each sub-module parses a specific manifest format and returns a list of
//! [`ExtractedDep`] records suitable for datasource version lookups.

pub mod bundler;
pub mod cargo;
pub mod composer;
pub mod docker_compose;
pub mod dockerfile;
pub mod github_actions;
pub mod gomod;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod pep621;
pub mod pip;
pub mod poetry;
pub mod pubspec;
