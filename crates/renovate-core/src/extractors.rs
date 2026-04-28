//! Dependency extractors for each supported package manager.
//!
//! Each sub-module parses a specific manifest format and returns a list of
//! [`ExtractedDep`] records suitable for datasource version lookups.

pub mod asdf;
pub mod bundler;
pub mod cargo;
pub mod cocoapods;
pub mod composer;
pub mod docker_compose;
pub mod dockerfile;
pub mod github_actions;
pub mod gomod;
pub mod gradle;
pub mod helm;
pub mod maven;
pub mod mix;
pub mod npm;
pub mod nuget;
pub mod pep621;
pub mod pip;
pub mod poetry;
pub mod pre_commit;
pub mod pubspec;
pub mod setup_cfg;
pub mod spm;
pub mod terraform;
