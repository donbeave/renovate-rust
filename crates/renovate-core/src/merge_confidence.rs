//! Merge confidence level utilities.
//!
//! Mirrors:
//! - `lib/util/merge-confidence/common.ts`
//! - `lib/util/merge-confidence/index.ts` (pure functions only)

/// Ordered confidence levels from lowest to highest.
///
/// Mirrors `confidenceLevels` from `lib/util/merge-confidence/index.ts`.
const CONFIDENCE_ORDER: &[&str] = &["low", "neutral", "high", "very high"];

/// Returns true if `value` is a recognized merge confidence level.
///
/// Mirrors `isMergeConfidence` from `lib/util/merge-confidence/index.ts`.
pub fn is_merge_confidence(value: &str) -> bool {
    CONFIDENCE_ORDER.contains(&value)
}

/// Returns true if `confidence` is an active (actionable) confidence level.
///
/// "low" and non-recognized values are considered inactive.
/// Mirrors `isActiveConfidenceLevel` from `lib/util/merge-confidence/index.ts`.
pub fn is_active_confidence_level(confidence: &str) -> bool {
    is_merge_confidence(confidence) && confidence != "low"
}

/// Returns true if `confidence` is at least `minimum_confidence`.
///
/// Mirrors `satisfiesConfidenceLevel` from `lib/util/merge-confidence/index.ts`.
pub fn satisfies_confidence_level(confidence: &str, minimum_confidence: &str) -> bool {
    let conf_order = |level: &str| {
        CONFIDENCE_ORDER
            .iter()
            .position(|&l| l == level)
            .map(|i| i as i32)
            .unwrap_or(-1)
    };
    conf_order(confidence) >= conf_order(minimum_confidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns false if null" — util/merge-confidence/index.spec.ts line 22
    #[test]
    fn is_active_confidence_level_null_returns_false() {
        assert!(!is_active_confidence_level("null"));
    }

    // Ported: "returns false if low" — util/merge-confidence/index.spec.ts line 26
    #[test]
    fn is_active_confidence_level_low_returns_false() {
        assert!(!is_active_confidence_level("low"));
    }

    // Ported: "returns false if nonsense" — util/merge-confidence/index.spec.ts line 30
    #[test]
    fn is_active_confidence_level_nonsense_returns_false() {
        assert!(!is_active_confidence_level("nonsense"));
    }

    // Ported: "returns true if valid value (high)" — util/merge-confidence/index.spec.ts line 34
    #[test]
    fn is_active_confidence_level_high_returns_true() {
        assert!(is_active_confidence_level("high"));
    }

    // Ported: "returns false if less" — util/merge-confidence/index.spec.ts line 40
    #[test]
    fn satisfies_confidence_level_less_returns_false() {
        assert!(!satisfies_confidence_level("low", "high"));
    }

    // Ported: "returns true if equal" — util/merge-confidence/index.spec.ts line 44
    #[test]
    fn satisfies_confidence_level_equal_returns_true() {
        assert!(satisfies_confidence_level("high", "high"));
    }

    // Ported: "returns true if more" — util/merge-confidence/index.spec.ts line 48
    #[test]
    fn satisfies_confidence_level_more_returns_true() {
        assert!(satisfies_confidence_level("very high", "high"));
    }
}
