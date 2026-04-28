//! Datasource clients for fetching available package versions.
//!
//! Each sub-module implements a registry-specific version lookup. The common
//! output is a list of available version strings that the update-planner then
//! compares against the current constraint.

pub mod crates_io;
pub mod docker_hub;
pub mod github_tags;
pub mod gomod;
pub mod maven;
pub mod npm;
pub mod packagist;
pub mod pypi;
