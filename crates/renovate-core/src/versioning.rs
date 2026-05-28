//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.

pub mod apk;
pub mod bazel_module;
pub mod aws_eks_addon;
pub mod aws_machine_image;
pub mod azure_rest_api;
pub mod cargo;
pub mod conan;
pub mod conda;
pub mod deno;
pub mod elm;
pub mod devbox;
pub mod docker;
pub mod exact;
pub mod git;
pub mod github_actions;
pub mod hex;
pub mod glasskube;
pub mod go_mod_directive;
pub mod deb;
pub mod gradle;
pub mod ivy;
pub mod hashicorp;
pub mod helm;
pub mod hermit;
pub mod kubernetes_api;
pub mod lambda_node;
pub mod loose;
pub mod maven;
pub mod nixpkgs;
pub mod node;
pub mod npm;
pub mod nuget;
pub mod pep440;
pub mod perl;
pub mod pvp;
pub mod redhat;
pub mod rez;
pub mod rpm;
pub mod rust_release_channel;
pub mod same_major;
pub mod semver_coerced;
pub mod semver_partial;
pub mod semver_generic;
pub mod semver_node;
pub mod ubuntu;
pub mod unity3d;
pub mod unity3d_packages;
