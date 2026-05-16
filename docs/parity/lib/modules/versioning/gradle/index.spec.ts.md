# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/gradle/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/gradle/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/gradle/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compare("$a", "$b") === $expected | 6 | not-applicable | — | — | Renovate's Gradle versioning comparator is not implemented as a Rust versioning API; Rust Gradle support is extractor/wrapper oriented. |
| parsePrefixRange("$rangeStr") is null | 89 | not-applicable | — | — | Renovate's Gradle prefix-range parser is not implemented as a Rust versioning API. |
| parseMavenBasedRange("$rangeStr") is null | 102 | not-applicable | — | — | Renovate's Gradle Maven-style range parser is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 127 | not-applicable | — | — | Renovate's Gradle version/range validation is not implemented as a Rust versioning API. |
| isVersion("$input") === $expected | 140 | not-applicable | — | — | Renovate's Gradle version classifier is not implemented as a Rust versioning API. |
| isStable("$input") === $expected | 180 | not-applicable | — | — | Renovate's Gradle stability classifier is not implemented as a Rust versioning API. |
| "$input" is represented as [$major, $minor, $patch] | 216 | not-applicable | — | — | Renovate's Gradle component parser is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 239 | not-applicable | — | — | Renovate's Gradle range matcher is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 271 | not-applicable | — | — | Renovate's Gradle comparator wrapper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 280 | not-applicable | — | — | Renovate's Gradle satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 292 | not-applicable | — | — | Renovate's Gradle satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 304 | not-applicable | — | — | Renovate's Gradle update-value helper is not implemented as a Rust versioning API. |

---

