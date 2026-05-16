# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pep440/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pep440/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/pep440/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | not-applicable | — | — | Renovate's full PEP 440 validation contract is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| isStable("$input") === $expected | 25 | not-applicable | — | — | Renovate's PEP 440 stability classifier is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| equals($a, $b) === $expected | 34 | not-applicable | — | — | Renovate's PEP 440 comparator is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| matches($a, $b) === $expected | 42 | not-applicable | — | — | Renovate's PEP 440 range matcher is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| isSingleVersion("$version") === $isSingle | 53 | not-applicable | — | — | Renovate's PEP 440 single-version classifier is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| getSatisfyingVersion($versions, "$range") === $expected | 78 | not-applicable | — | — | Renovate's PEP 440 satisfying-version helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| minSatisfyingVersion($versions, "$range") === $expected | 89 | not-applicable | — | — | Renovate's PEP 440 satisfying-version helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 100 | not-applicable | — | — | Renovate's PEP 440 update-value helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 190 | not-applicable | — | — | Renovate's PEP 440 replacement update-value helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |
| isLessThanRange("$version", "$range") === "$expected" | 307 | not-applicable | — | — | Renovate's PEP 440 range comparison helper is not implemented as a Rust API; Rust currently exposes narrower exact-pin update-summary logic. |

---

