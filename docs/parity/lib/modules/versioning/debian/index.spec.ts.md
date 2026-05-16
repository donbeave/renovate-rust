# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/debian/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/debian/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/debian/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test | 18 | not-applicable | — | — | Renovate's Debian versioning scheme and rolling release data helpers are not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 22 | not-applicable | — | — | Renovate's Debian versioning validation is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 82 | not-applicable | — | — | Renovate's Debian versioning compatibility helper is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 104 | not-applicable | — | — | Renovate's Debian single-version classifier is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 115 | not-applicable | — | — | Renovate's Debian stability classifier is not implemented as a Rust versioning API. |
| ensures that rolling release is not refreshed within frame time window: $version | 169 | not-applicable | — | — | Renovate's Debian rolling release data cache is not implemented as a Rust versioning API. |
| isVersion("$version") === $expected | 188 | not-applicable | — | — | Renovate's Debian versioning scheme is not implemented as a Rust versioning API. |
| getMajor, getMinor, getPatch for "$version" | 248 | not-applicable | — | — | Renovate's Debian version component parser is not implemented as a Rust versioning API. |
| equals($a, $b) === $expected | 273 | not-applicable | — | — | Renovate's Debian version comparator is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 297 | not-applicable | — | — | Renovate's Debian version comparator is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === "$expected" | 340 | not-applicable | — | — | Renovate's Debian range satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === "$expected" | 361 | not-applicable | — | — | Renovate's Debian range satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 383 | not-applicable | — | — | Renovate's Debian update-value helper is not implemented as a Rust versioning API. |
| debian.sortVersions($a, $b) === $expected | 409 | not-applicable | — | — | Renovate's Debian version comparator is not implemented as a Rust versioning API. |
| matches("$version", "$range") === "$expected" | 429 | not-applicable | — | — | Renovate's Debian range matcher is not implemented as a Rust versioning API. |
| checks runtime date handling & refresh rolling release data | 441 | not-applicable | — | — | Renovate's Debian rolling release data refresh behavior is not implemented as a Rust versioning API. |

---

