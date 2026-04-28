//! Datasource clients for fetching available package versions.
//!
//! Each sub-module implements a registry-specific version lookup. The common
//! output is a list of available version strings that the update-planner then
//! compares against the current constraint.

pub mod azure_bicep;
pub mod azure_pipelines_tasks;
pub mod bazel;
pub mod bitrise;
pub mod cdnjs;
pub mod cocoapods;
pub mod conan;
pub mod cpan;
pub mod crates_io;
pub mod devbox;
pub mod docker_hub;
pub mod github_releases;
pub mod github_runners;
pub mod github_tags;
pub mod gitlab_tags;
pub mod gomod;
pub mod gradle_version;
pub mod hackage;
pub mod helm;
pub mod hex;
pub mod jenkins_plugins;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod orb;
pub mod packagist;
pub mod pub_dev;
pub mod puppet_forge;
pub mod pypi;
pub mod rubygems;
pub mod terraform;
pub mod typst;
pub mod unity3d;
