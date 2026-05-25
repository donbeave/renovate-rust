//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.

pub mod aws_machine_image;
pub mod cargo;
pub mod deno;
pub mod devbox;
pub mod exact;
pub mod git;
pub mod github_actions;
pub mod glasskube;
pub mod hashicorp;
pub mod kubernetes_api;
pub mod same_major;
pub mod loose;
pub mod helm;
pub mod maven;
pub mod nixpkgs;
pub mod npm;
pub mod nuget;
pub mod pep440;
pub mod perl;
pub mod pvp;
pub mod redhat;
pub mod rpm;
pub mod rust_release_channel;
pub mod semver_coerced;
pub mod semver_generic;
pub mod semver_node;
pub mod ubuntu;
pub mod unity3d;
pub mod unity3d_packages;
