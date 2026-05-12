//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.

pub mod aws_machine_image;
pub mod cargo;
pub mod exact;
pub mod git;
pub mod hashicorp;
pub mod helm;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod pep440;
pub mod pvp;
pub mod redhat;
pub mod rpm;
pub mod semver_coerced;
pub mod semver_generic;
pub mod ubuntu;
pub mod unity3d;
