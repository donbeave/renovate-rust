# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pep440/range.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pep440/range.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/pep440/range`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checkRange("$rangeInput, "$newVersion"") === "$expected" | 8 | not-applicable | — | — | Renovate's PEP 440 range normalization helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| returns null without warning if new version is excluded from range | 24 | not-applicable | — | — | Renovate's PEP 440 range update-value/logging behavior is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| handles v-prefixed version as currentValue | 39 | not-applicable | — | — | Renovate's PEP 440 update-value helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| handles bare version that differs from currentVersion without v-prefix | 49 | not-applicable | — | — | Renovate's PEP 440 update-value helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |

---

