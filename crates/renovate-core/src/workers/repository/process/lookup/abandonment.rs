//! Package abandonment detection for lookup updates.
//!
//! Mirrors `lib/workers/repository/process/lookup/abandonment.ts`.

//! @parity `lib/workers/repository/process/lookup/abandonment.ts` partial — calculateAbandonment (delegates to util::calculate_abandonment which implements the timestamp + threshold logic from mostRecentTimestamp + abandonmentThreshold, sets isAbandoned on result, writes AbandonedPackageStats if abandoned); single test ported. The core calculate_abandonment (with existing Ported tests) lives in util.rs; this provides the dedicated lookup/abandonment module for source-mapping.
// Re-export the core for the lookup step (the TS fn signature is adapted to Rust types in util).
pub use crate::util::calculate_abandonment;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_original_release_result_when_no_abandonment_threshold_is_provided() {
        // Ported: "returns the original release result when no abandonment threshold is provided" — lib/workers/repository/process/lookup/abandonment.spec.ts line 27
        // (the util returns Option<bool> for isAbandoned; None/ false means "original" i.e. no abandonment applied. Full tests for core live in util with Ported; this verifies the unit surface.)
        let result = calculate_abandonment(Some("2022-01-01T00:00:00.000Z"), None, 1672531200000); // mock now from util test
        assert!(result.is_none() || result == Some(false));
    }
}
