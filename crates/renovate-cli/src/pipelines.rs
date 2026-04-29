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
//! - [`pre_commit`]    — pre-commit
//! - [`git`]           — Git submodules
//! - [`puppet`]        — Puppet
//! - [`conan`]         — Conan C++ package manager
//! - [`haskell`]       — Haskell Cabal
//! - [`jenkins`]       — Jenkins plugins + OCB (OpenTelemetry Collector Builder)
//! - [`homebrew`]      — Homebrew formula
//! - [`typst`]         — Typst
//! - [`cpanfile`]      — cpanfile (Perl)
//! - [`vendir`]        — Vendir
//! - [`cnb`]           — Cloud Native Buildpacks
//! - [`copier`]        — Copier template manager
//! - [`batect`]        — Batect build tool
//! - [`heroku`]        — Heroku/Render runtime.txt
//! - [`misc`]          — Renovate config extends presets, Hermit

mod ansible;
mod batect;
mod bazel;
mod ci;
mod cnb;
mod conan;
mod copier;
mod cpanfile;
mod custom_managers;
mod dart;
mod docker;
mod dotnet;
mod git;
mod go;
mod haskell;
mod helm;
mod heroku;
mod homebrew;
mod javascript;
mod jenkins;
mod jvm;
mod kubernetes;
mod misc;
mod mobile;
mod nix;
mod php;
mod pre_commit;
mod puppet;
mod python;
mod ruby;
mod rust;
mod terraform;
mod typst;
mod vendir;
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
    pre_commit::process(ctx).await;
    git::process(ctx).await;
    puppet::process(ctx).await;
    conan::process(ctx).await;
    haskell::process(ctx).await;
    jenkins::process(ctx).await;
    homebrew::process(ctx).await;
    typst::process(ctx).await;
    cpanfile::process(ctx).await;
    vendir::process(ctx).await;
    cnb::process(ctx).await;
    copier::process(ctx).await;
    batect::process(ctx).await;
    heroku::process(ctx).await;
    misc::process(ctx).await;
    // customManagers runs last so it can pick up files not matched by built-in managers.
    custom_managers::process(ctx).await;
}
