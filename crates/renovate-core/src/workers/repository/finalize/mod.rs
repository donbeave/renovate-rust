//! Finalize module.
//!
//! Mirrors `lib/workers/repository/finalize/`.

pub mod index;
pub mod prune;
pub mod repository_statistics;

pub use index::FinalizeResult;
