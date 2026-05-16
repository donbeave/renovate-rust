# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/elm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/elm/index.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/elm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 5 | not-applicable | — | — | Renovate's Elm versioning scheme is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 23 | not-applicable | — | — | Renovate's Elm version/range validation is not implemented as a Rust versioning API. |
| isSingleVersion("$input") === $expected | 43 | not-applicable | — | — | Renovate's Elm single-version classifier is not implemented as a Rust versioning API. |
| isStable("$input") === $expected | 55 | not-applicable | — | — | Renovate's Elm stability classifier is not implemented as a Rust versioning API. |
| returns false for invalid version | 65 | not-applicable | — | — | Renovate's Elm stability classifier is not implemented as a Rust versioning API. |
| isCompatible("$input") === $expected | 71 | not-applicable | — | — | Renovate's Elm compatibility helper is not implemented as a Rust versioning API. |
| extracts version components | 81 | not-applicable | — | — | Renovate's Elm component parser is not implemented as a Rust versioning API. |
| equals("$a", "$b") === $expected | 89 | not-applicable | — | — | Renovate's Elm comparator is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 100 | not-applicable | — | — | Renovate's Elm comparator is not implemented as a Rust versioning API. |
| sorts versions correctly | 112 | not-applicable | — | — | Renovate's Elm sorting comparator is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 120 | not-applicable | — | — | Renovate's Elm range matcher is not implemented as a Rust versioning API. |
| returns false for invalid version | 139 | not-applicable | — | — | Renovate's Elm range matcher is not implemented as a Rust versioning API. |
| returns false for invalid range | 143 | not-applicable | — | — | Renovate's Elm range matcher is not implemented as a Rust versioning API. |
| returns false for malformed range where lower > upper | 147 | not-applicable | — | — | Renovate's Elm range matcher is not implemented as a Rust versioning API. |
| isLessThanRange("$version", "$range") === $expected | 153 | not-applicable | — | — | Renovate's Elm range comparison helper is not implemented as a Rust versioning API. |
| returns false for invalid version | 170 | not-applicable | — | — | Renovate's Elm range comparison helper is not implemented as a Rust versioning API. |
| returns false for invalid range | 176 | not-applicable | — | — | Renovate's Elm range comparison helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 182 | not-applicable | — | — | Renovate's Elm satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 199 | not-applicable | — | — | Renovate's Elm satisfying-version helper is not implemented as a Rust versioning API. |
| replaces exact version with new version | 215 | not-applicable | — | — | Renovate's Elm update-value helper is not implemented as a Rust versioning API. |
| handles bump strategy for exact version | 225 | not-applicable | — | — | Renovate's Elm update-value helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$newVersion") === "$expected" | 237 | not-applicable | — | — | Renovate's Elm range update-value helper is not implemented as a Rust versioning API. |
| returns null for invalid new version | 266 | not-applicable | — | — | Renovate's Elm update-value helper is not implemented as a Rust versioning API. |
| returns null for invalid current value | 276 | not-applicable | — | — | Renovate's Elm update-value helper is not implemented as a Rust versioning API. |
| returns null for unknown range strategy | 286 | not-applicable | — | — | Renovate's Elm update-value helper is not implemented as a Rust versioning API. |
| handles widen when newVersion equals upper bound exactly | 296 | not-applicable | — | — | Renovate's Elm range update-value helper is not implemented as a Rust versioning API. |
| widens elm-version range for new compiler release | 307 | not-applicable | — | — | Renovate's Elm compiler range update-value helper is not implemented as a Rust versioning API. |
| keeps elm-version range unchanged when version is already satisfied | 318 | not-applicable | — | — | Renovate's Elm compiler range update-value helper is not implemented as a Rust versioning API. |
| replaces elm-version range when explicitly requested | 328 | not-applicable | — | — | Renovate's Elm compiler range update-value helper is not implemented as a Rust versioning API. |
| finds highest satisfying version for elm-version range | 341 | not-applicable | — | — | Renovate's Elm compiler satisfying-version helper is not implemented as a Rust versioning API. |
| returns null when no compiler version satisfies range | 355 | not-applicable | — | — | Renovate's Elm compiler satisfying-version helper is not implemented as a Rust versioning API. |

---

