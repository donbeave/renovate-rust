//! Onboarding PR module.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/`.

pub mod config_description;
pub mod index;
pub mod pr_list; // for the config-description.ts port (getConfigDesc/getScheduleDesc)

pub use index::OnboardingPrConfig;
