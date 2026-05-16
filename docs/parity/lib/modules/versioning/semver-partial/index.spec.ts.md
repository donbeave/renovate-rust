# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver-partial/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver-partial/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/semver-partial/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 5 | not-applicable | — | — | Renovate's semver-partial `VersioningApi` validation contract is not implemented as a Rust API; Rust currently exposes a narrower generic semver update-summary helper. |
| isVersion("$version") === $expected | 24 | not-applicable | — | — | Renovate's semver-partial version classifier is not implemented as a Rust API; Rust currently exposes a narrower generic semver update-summary helper. |
| isStable("$version") === $expected | 47 | not-applicable | — | — | Renovate's semver-partial stability classifier is not implemented as a Rust API; Rust currently exposes a narrower generic semver update-summary helper. |
| isSingleVersion("$version") === $expected | 70 | not-applicable | — | — | Renovate's semver-partial single-version classifier is not implemented as a Rust API; Rust currently exposes a narrower generic semver update-summary helper. |
| matches("$version", "$range") === $expected | 87 | not-applicable | — | — | Renovate's semver-partial range matcher is not implemented as a Rust API; Rust currently exposes a narrower generic semver update-summary helper. |
| should handle invalid range that is not ~latest or valid version | 141 | not-applicable | — | — | Renovate's semver-partial invalid-range matcher behavior is not implemented as a Rust API. |
| getSatisfyingVersion($versions, "$range") === $expected | 149 | not-applicable | — | — | Renovate's semver-partial satisfying-version helper is not implemented as a Rust API. |
| minSatisfyingVersion($versions, "$range") === $expected | 185 | not-applicable | — | — | Renovate's semver-partial satisfying-version helper is not implemented as a Rust API. |
| isLessThanRange("$version", "$range") === $expected | 209 | not-applicable | — | — | Renovate's semver-partial range comparison helper is not implemented as a Rust API. |
| equals("$version", "$other") === $expected | 240 | not-applicable | — | — | Renovate's semver-partial comparator is not implemented as a Rust API. |
| getMajor("$version") === $expected | 262 | not-applicable | — | — | Renovate's semver-partial version component parser is not implemented as a Rust API. |
| getMinor("$version") === $expected | 275 | not-applicable | — | — | Renovate's semver-partial version component parser is not implemented as a Rust API. |
| getPatch("$version") === $expected | 288 | not-applicable | — | — | Renovate's semver-partial version component parser is not implemented as a Rust API. |
| isGreaterThan("$version", "$other") === $expected | 301 | not-applicable | — | — | Renovate's semver-partial comparator is not implemented as a Rust API. |
| sortVersions("$a", "$b") === $expected | 326 | not-applicable | — | — | Renovate's semver-partial sorting comparator is not implemented as a Rust API. |
| isBreaking("$version", "$current") === $expected | 348 | not-applicable | — | — | Renovate's semver-partial breaking-change helper is not implemented as a Rust API. |
| isCompatible("$version") === $expected | 376 | not-applicable | — | — | Renovate's semver-partial compatibility helper is not implemented as a Rust API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 390 | not-applicable | — | — | Renovate's semver-partial update-value helper is not implemented as a Rust API. |

---

