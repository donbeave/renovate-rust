# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/current-age.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/current-age.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `util/package-rules/current-age › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if release is older | 18 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_is_older` | — |
| returns false if release is younger | 30 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_is_younger` | — |
| returns null if release invalid | 42 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_invalid` | Rust matcher is boolean-only, so invalid dates are treated as a non-match |
| returns false if release undefined | 54 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_undefined` | — |
| returns true if age matches | 66 | ported | `package_rule.rs` | `current_age_matcher_returns_true_if_age_matches` | — |

---

