//! Per-manager pipeline implementations organized by ecosystem.
//!
//! Each sub-module handles one logical group of package managers.  All
//! sub-modules share state via [`RepoPipelineCtx`] and re-use imports
//! from this module via `use super::*`.
//!
//! Sub-modules:
//! - [`rust`]          — Cargo
//! - [`javascript`]    — npm, Bun, Meteor, HTML CDN, CDN URLs
//! - [`python`]        — pip, Poetry, pep621, setup.cfg, Pipfile, PEP 723, Pixi
//! - [`go`]            — Go modules
//! - [`jvm`]           — Maven, Gradle, SBT, Kotlin, Ant, Scalafmt, Clojure, Leiningen
//! - [`ci`]            — GitHub Actions, GitLab CI, CircleCI, Buildkite, Azure, Bitrise, …
//! - [`docker`]        — Dockerfile, Docker Compose, Dev Container, Quadlet
//! - [`helm`]          — Helm, Helmfile, Helmsman, Fleet
//! - [`kubernetes`]    — Kustomize, raw manifests, FluxCD, Tekton, ArgoCD, Crossplane, …
//! - [`terraform`]     — Terraform, Terragrunt, TFLint, Azure Bicep
//! - [`ruby`]          — Bundler, gemspec, CocoaPods
//! - [`mobile`]        — Swift, Mint, XcodeGen, Mix (Elixir), Gleam
//! - [`dotnet`]        — NuGet, Cake
//! - [`php`]           — Composer
//! - [`dart`]          — pub (Dart/Flutter), FVM
//! - [`version_files`] — asdf, mise, tool-version files, Devbox
//! - [`bazel`]         — Bazel Module, Bazel WORKSPACE
//! - [`ansible`]       — ansible-galaxy, ansible task files
//! - [`nix`]           — Nix flakes
//! - [`misc`]          — Typst, cpanfile, pre-commit, Conan, Haskell, Jsonnet, Puppet, Jenkins, …

mod ansible;
mod bazel;
mod ci;
mod dart;
mod docker;
mod dotnet;
mod go;
mod helm;
mod javascript;
mod jvm;
mod kubernetes;
mod misc;
mod mobile;
mod nix;
mod php;
mod python;
mod ruby;
mod rust;
mod terraform;
mod version_files;

// ── Shared imports re-exported for sub-modules via `use super::*` ──────────

pub(crate) use std::collections::HashMap;

pub(crate) use crate::context::RepoPipelineCtx;
pub(crate) use crate::output;
pub(crate) use crate::pipeline_utils::*;
pub(crate) use crate::report_builders::*;
pub(crate) use renovate_core::datasources::bitrise as bitrise_datasource;
pub(crate) use renovate_core::datasources::crates_io;
pub(crate) use renovate_core::datasources::docker_hub as docker_datasource;
pub(crate) use renovate_core::datasources::github_releases as github_releases_datasource;
pub(crate) use renovate_core::datasources::github_tags as github_tags_datasource;
pub(crate) use renovate_core::datasources::gomod as gomod_datasource;
pub(crate) use renovate_core::datasources::helm as helm_datasource;
pub(crate) use renovate_core::datasources::maven as maven_datasource;
pub(crate) use renovate_core::datasources::npm as npm_datasource;
pub(crate) use renovate_core::datasources::nuget as nuget_datasource;
pub(crate) use renovate_core::datasources::packagist as packagist_datasource;
pub(crate) use renovate_core::datasources::pub_dev as pub_datasource;
pub(crate) use renovate_core::datasources::pypi as pypi_datasource;
pub(crate) use renovate_core::datasources::rubygems as rubygems_datasource;
pub(crate) use renovate_core::datasources::terraform as terraform_datasource;
pub(crate) use renovate_core::extractors::bundler as bundler_extractor;
pub(crate) use renovate_core::extractors::cargo as cargo_extractor;
pub(crate) use renovate_core::extractors::composer as composer_extractor;
pub(crate) use renovate_core::extractors::github_actions as github_actions_extractor;
pub(crate) use renovate_core::extractors::gomod as gomod_extractor;
pub(crate) use renovate_core::extractors::gradle as gradle_extractor;
pub(crate) use renovate_core::extractors::helm as helm_extractor;
pub(crate) use renovate_core::extractors::homeassistant as homeassistant_extractor;
pub(crate) use renovate_core::extractors::homebrew as homebrew_extractor;
pub(crate) use renovate_core::extractors::maven as maven_extractor;
pub(crate) use renovate_core::extractors::npm as npm_extractor;
pub(crate) use renovate_core::extractors::nuget as nuget_extractor;
pub(crate) use renovate_core::extractors::pep621 as pep621_extractor;
pub(crate) use renovate_core::extractors::pip as pip_extractor;
pub(crate) use renovate_core::extractors::poetry as poetry_extractor;
pub(crate) use renovate_core::extractors::pubspec as pubspec_extractor;
pub(crate) use renovate_core::extractors::setup_cfg as setup_cfg_extractor;
pub(crate) use renovate_core::extractors::terraform as terraform_extractor;
pub(crate) use renovate_core::http::HttpClient;

/// Run all manager pipeline blocks, accumulating results into `ctx.report`.
pub(crate) async fn process_all_managers(ctx: &mut RepoPipelineCtx<'_>) {
    rust::process(ctx).await;
    dart::process(ctx).await;
    dotnet::process(ctx).await;
    php::process(ctx).await;
    javascript::process(ctx).await;
    python::process(ctx).await;
    go::process(ctx).await;
    jvm::process(ctx).await;
    ci::process(ctx).await;
    docker::process(ctx).await;
    helm::process(ctx).await;
    kubernetes::process(ctx).await;
    terraform::process(ctx).await;
    ruby::process(ctx).await;
    mobile::process(ctx).await;
    version_files::process(ctx).await;
    bazel::process(ctx).await;
    ansible::process(ctx).await;
    nix::process(ctx).await;
    misc::process(ctx).await;
}
