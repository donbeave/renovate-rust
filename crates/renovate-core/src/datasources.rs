//! Datasource clients for fetching available package versions.
//!
//! Each sub-module implements a registry-specific version lookup. The common
//! output is a list of available version strings that the update-planner then
//! compares against the current constraint.

pub mod crates_io;
pub mod docker_hub;
pub mod github_tags;
pub mod gitlab_tags;
pub mod gomod;
pub mod helm;
pub mod hex;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod packagist;
pub mod pub_dev;
pub mod pypi;
pub mod rubygems;
pub mod terraform;
