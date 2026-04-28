//! Dependency extractors for each supported package manager.
//!
//! Each sub-module parses a specific manifest format and returns a list of
//! [`ExtractedDep`] records suitable for datasource version lookups.

pub mod ansible_galaxy;
pub mod asdf;
pub mod azure_pipelines;
pub mod bitbucket_pipelines;
pub mod buildkite;
pub mod bundler;
pub mod cargo;
pub mod circleci;
pub mod cloudbuild;
pub mod cocoapods;
pub mod composer;
pub mod devbox;
pub mod devcontainer;
pub mod docker_compose;
pub mod dockerfile;
pub mod droneci;
pub mod gemspec;
pub mod github_actions;
pub mod gitlabci;
pub mod gomod;
pub mod gradle;
pub mod gradle_wrapper;
pub mod helm;
pub mod helm_values;
pub mod helmfile;
pub mod jenkins;
pub mod kustomize;
pub mod maven;
pub mod maven_wrapper;
pub mod mise;
pub mod mix;
pub mod npm;
pub mod nuget;
pub mod pep621;
pub mod pip;
pub mod pipfile;
pub mod poetry;
pub mod pre_commit;
pub mod pubspec;
pub mod quadlet;
pub mod setup_cfg;
pub mod spm;
pub mod terraform;
pub mod velaci;
pub mod version_file;
pub mod woodpecker;
