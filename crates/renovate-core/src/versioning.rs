//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.

pub mod cargo;
pub mod maven;
pub mod npm;
pub mod pep440;
