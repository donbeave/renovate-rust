# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/regex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/regex/index.spec.ts
**Total tests:** 24 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/regex/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| requires a valid configuration to be initialized | 10 | not-applicable | — | — | Renovate's configurable regex versioning scheme is not implemented as a Rust versioning API; Rust uses regexes only in extractor/config-specific code. |
| works without config | 14 | not-applicable | — | — | Renovate's configurable regex versioning scheme is not implemented as a Rust versioning API; Rust uses regexes only in extractor/config-specific code. |
| works with missing version | 19 | not-applicable | — | — | Renovate's configurable regex versioning scheme is not implemented as a Rust versioning API; Rust uses regexes only in extractor/config-specific code. |
| on invalid regex: "$regex" | 25 | not-applicable | — | — | Renovate's regex versioning configuration validation is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 35 | not-applicable | — | — | Renovate's regex versioning validation is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 58 | not-applicable | — | — | Renovate's regex versioning compatibility helper is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 83 | not-applicable | — | — | Renovate's regex versioning single-version classifier is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 104 | not-applicable | — | — | Renovate's regex versioning stability classifier is not implemented as a Rust versioning API. |
| isVersion("$version") === $expected | 115 | not-applicable | — | — | Renovate's regex versioning classifier is not implemented as a Rust versioning API. |
| getMajor, getMinor, getPatch for "$version" | 135 | not-applicable | — | — | Renovate's regex versioning component parser is not implemented as a Rust versioning API. |
| equals($a, $b) === $expected | 149 | not-applicable | — | — | Renovate's regex versioning comparator is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 171 | not-applicable | — | — | Renovate's regex versioning comparator is not implemented as a Rust versioning API. |
| isLessThanRange($version, $range) === $expected | 204 | not-applicable | — | — | Renovate's regex versioning range comparison helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 253 | not-applicable | — | — | Renovate's regex versioning satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === "$expected" | 267 | not-applicable | — | — | Renovate's regex versioning satisfying-version helper is not implemented as a Rust versioning API. |
| returns newVersion | 282 | not-applicable | — | — | Renovate's regex versioning update-value helper is not implemented as a Rust versioning API. |
| sorts versions in an ascending order | 295 | not-applicable | — | — | Renovate's regex versioning sorting comparator is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 304 | not-applicable | — | — | Renovate's regex versioning matcher is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 365 | not-applicable | — | — | Renovate's regex versioning build/revision validation is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 373 | not-applicable | — | — | Renovate's regex versioning build/revision compatibility helper is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 384 | not-applicable | — | — | Renovate's regex versioning build/revision comparator is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 392 | not-applicable | — | — | Renovate's regex versioning build/revision matcher is not implemented as a Rust versioning API. |
| getSatisfyingVersion | 403 | not-applicable | — | — | Renovate's regex versioning build/revision satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion | 412 | not-applicable | — | — | Renovate's regex versioning build/revision satisfying-version helper is not implemented as a Rust versioning API. |

---

