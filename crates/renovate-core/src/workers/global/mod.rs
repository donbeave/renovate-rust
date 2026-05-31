//! Global worker module.
//!
//! Mirrors `lib/workers/global/`.

pub mod autodiscover;
pub mod config;
pub mod index;
pub mod initialize;

pub use index::GlobalWorkerConfig;
