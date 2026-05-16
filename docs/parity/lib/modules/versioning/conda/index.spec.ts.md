# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/conda/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/conda/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/conda/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 4 | not-applicable | — | — | Renovate's Conda versioning scheme is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 26 | not-applicable | — | — | Renovate's Conda versioning validation is not implemented as a Rust versioning API. |
| isStable("$input") === $expected | 47 | not-applicable | — | — | Renovate's Conda stability classifier is not implemented as a Rust versioning API. |
| equals("$a", "$b") === $expected | 57 | not-applicable | — | — | Renovate's Conda version comparator is not implemented as a Rust versioning API. |
| matches("$a", "$b") === $expected | 69 | not-applicable | — | — | Renovate's Conda matcher is not implemented as a Rust versioning API. |
| getMajor("$a") === $expected | 82 | not-applicable | — | — | Renovate's Conda version component parser is not implemented as a Rust versioning API. |
| getMinor($a) === $expected | 93 | not-applicable | — | — | Renovate's Conda version component parser is not implemented as a Rust versioning API. |
| getPatch("$a") === $expected | 105 | not-applicable | — | — | Renovate's Conda version component parser is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $isSingle | 116 | not-applicable | — | — | Renovate's Conda single-version classifier is not implemented as a Rust versioning API. |
| always compatible | 131 | not-applicable | — | — | Renovate's Conda compatibility helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 146 | not-applicable | — | — | Renovate's Conda satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 157 | not-applicable | — | — | Renovate's Conda satisfying-version helper is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $result | 168 | not-applicable | — | — | Renovate's Conda version comparator is not implemented as a Rust versioning API. |
| returns a pinned value | 176 | not-applicable | — | — | Renovate's Conda pinned-value helper is not implemented as a Rust versioning API. |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion) === $expected | 180 | not-applicable | — | — | Renovate's Conda update-value helper is not implemented as a Rust versioning API. |

---

