# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/composer/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/composer/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/composer/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getMajor("$version") === $expected | 4 | not-applicable | — | — | Renovate's Composer versioning scheme is not implemented as a Rust versioning API; Rust Composer support is extractor/pipeline oriented. |
| getMinor("$version") === $expected | 12 | not-applicable | — | — | Renovate's Composer versioning scheme is not implemented as a Rust versioning API; Rust Composer support is extractor/pipeline oriented. |
| getPatch("$version") === $expected | 20 | not-applicable | — | — | Renovate's Composer versioning scheme is not implemented as a Rust versioning API; Rust Composer support is extractor/pipeline oriented. |
| equals("$a", "$b") === $expected | 28 | not-applicable | — | — | Renovate's Composer comparator is not implemented as a Rust versioning API; Rust Composer support is extractor/pipeline oriented. |
| isGreaterThan("$a", "$b") === $expected | 40 | not-applicable | — | — | Renovate's Composer comparator is not implemented as a Rust versioning API; Rust Composer support is extractor/pipeline oriented. |
| isSingleVersion("$version") === $expected | 55 | not-applicable | — | — | Renovate's Composer single-version classifier is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 63 | not-applicable | — | — | Renovate's Composer stability classifier is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 75 | not-applicable | — | — | Renovate's Composer version/range validation is not implemented as a Rust versioning API. |
| isLessThanRange("$a", "$b") === $expected | 108 | not-applicable | — | — | Renovate's Composer range comparison helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 116 | not-applicable | — | — | Renovate's Composer satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 131 | not-applicable | — | — | Renovate's Composer satisfying-version helper is not implemented as a Rust versioning API. |
| matches("$a", "$b") === $expected | 147 | not-applicable | — | — | Renovate's Composer range matcher is not implemented as a Rust versioning API. |
| subset("$a", "$b") === $expected | 155 | not-applicable | — | — | Renovate's Composer range subset helper is not implemented as a Rust versioning API. |
| intersects("$a", "$b") === $expected | 177 | not-applicable | — | — | Renovate's Composer range intersection helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 199 | not-applicable | — | — | Renovate's Composer update-value helper is not implemented as a Rust versioning API. |
| $versions -> sortVersions -> $expected | 256 | not-applicable | — | — | Renovate's Composer sorting comparator is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 266 | not-applicable | — | — | Renovate's Composer compatibility helper is not implemented as a Rust versioning API. |
| isBreaking("$currentVersion", "$newVersion") === $expected | 275 | not-applicable | — | — | Renovate's Composer breaking-change helper is not implemented as a Rust versioning API. |

---

