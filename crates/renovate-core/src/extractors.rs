//! Dependency extractors for each supported package manager.
//!
//! Each sub-module parses a specific manifest format and returns a list of
//! [`ExtractedDep`] records suitable for datasource version lookups.

pub mod cargo;
