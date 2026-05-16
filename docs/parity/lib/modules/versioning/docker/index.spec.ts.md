# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/docker/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/docker/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 5 | not-applicable | — | — | Renovate's Docker tag versioning scheme is not implemented as a Rust versioning API; Rust Docker support is currently extractor/datasource oriented. |
| getMajor, getMinor, getPatch for "$version" | 27 | not-applicable | — | — | Renovate's Docker tag versioning scheme is not implemented as a Rust versioning API. |
| isGreaterThan($a, $b) === $expected | 43 | not-applicable | — | — | Renovate's Docker tag versioning comparator is not implemented as a Rust versioning API. |
| isLessThanRange($version, $range) === $expected | 54 | not-applicable | — | — | Renovate's Docker tag versioning range helper is not implemented as a Rust versioning API. |
| equals($a, $b) === $expected | 68 | not-applicable | — | — | Renovate's Docker tag versioning comparator is not implemented as a Rust versioning API. |
| satisfying for $version -> $expected | 92 | not-applicable | — | — | Renovate's Docker tag satisfying-version helper is not implemented as a Rust versioning API. |
| docker.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b") | 108 | not-applicable | — | — | Renovate's Docker tag versioning comparator is not implemented as a Rust versioning API. |
| sorts unstable | 123 | not-applicable | — | — | Renovate's Docker unstable-tag ordering behavior is not implemented as a Rust versioning API. |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 148 | not-applicable | — | — | Renovate's Docker tag update-value helper is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 164 | not-applicable | — | — | Renovate's Docker tag stability classifier is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 177 | not-applicable | — | — | Renovate's Docker tag compatibility helper is not implemented as a Rust versioning API. |
| valueToVersion("$value") === $expected | 199 | not-applicable | — | — | Renovate's Docker tag value-to-version helper is not implemented as a Rust versioning API. |

---

