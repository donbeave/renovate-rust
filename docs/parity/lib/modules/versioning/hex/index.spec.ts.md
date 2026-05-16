# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hex/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/hex/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 4 | not-applicable | — | — | Renovate's Hex versioning scheme is not implemented as a Rust versioning API; Rust Hex support is datasource/extractor oriented. |
| getSatisfyingVersion($versions, "$range") === $expected | 19 | not-applicable | — | — | Renovate's Hex range satisfying-version helper is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 30 | not-applicable | — | — | Renovate's Hex versioning validation is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 41 | not-applicable | — | — | Renovate's Hex single-version classifier is not implemented as a Rust versioning API. |
| getPinnedValue returns == prefixed version | 52 | not-applicable | — | — | Renovate's Hex pinned-value helper is not implemented as a Rust versioning API. |
| isLessThanRange($version, $range) === $expected | 56 | not-applicable | — | — | Renovate's Hex range comparison helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 69 | not-applicable | — | — | Renovate's Hex range satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 80 | not-applicable | — | — | Renovate's Hex update-value helper is not implemented as a Rust versioning API. |

---

