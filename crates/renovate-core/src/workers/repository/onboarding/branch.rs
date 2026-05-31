//! Onboarding branch module.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/`.

pub mod check;
pub mod commit_message;
pub mod config;
pub mod create;
pub mod index;

pub use index::{OnboardingBranchConfig, OnboardingResult};
