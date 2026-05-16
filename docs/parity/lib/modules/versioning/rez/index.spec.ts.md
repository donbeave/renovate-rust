# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rez/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rez/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/rez/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$version", "$equal") === $expected | 5 | not-applicable | — | — | Renovate's Rez versioning scheme is not implemented as a Rust versioning API. |
| getMajor("$version") === $expected | 21 | not-applicable | — | — | Renovate's Rez versioning scheme is not implemented as a Rust versioning API. |
| getMinor("$version") === $expected | 30 | not-applicable | — | — | Renovate's Rez versioning scheme is not implemented as a Rust versioning API. |
| getPatch("$version") === $expected | 39 | not-applicable | — | — | Renovate's Rez versioning scheme is not implemented as a Rust versioning API. |
| isGreaterThan("$version", "$other") === $expected | 49 | not-applicable | — | — | Renovate's Rez versioning comparator is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 67 | not-applicable | — | — | Renovate's Rez versioning stability classifier is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 78 | not-applicable | — | — | Renovate's Rez versioning validation is not implemented as a Rust versioning API. |
| isVersion("$input") === $expected | 100 | not-applicable | — | — | Renovate's Rez single-version classifier is not implemented as a Rust versioning API. |
| isSingleVersion("$input") === $expected | 108 | not-applicable | — | — | Renovate's Rez single-version classifier is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 119 | not-applicable | — | — | Renovate's Rez range satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 135 | not-applicable | — | — | Renovate's Rez range satisfying-version helper is not implemented as a Rust versioning API. |
| isLessThanRange($version, "$range") === $expected | 145 | not-applicable | — | — | Renovate's Rez range comparison helper is not implemented as a Rust versioning API. |
| matches($version, "$range") === $expected | 158 | not-applicable | — | — | Renovate's Rez range matcher is not implemented as a Rust versioning API. |
| rez.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b") | 178 | not-applicable | — | — | Renovate's Rez versioning comparator is not implemented as a Rust versioning API. |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 193 | not-applicable | — | — | Renovate's Rez range update-value helper is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 443 | not-applicable | — | — | Renovate's Rez compatibility helper is not implemented as a Rust versioning API. |

---

