# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rust-release-channel/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rust-release-channel/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/rust-release-channel/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | not-applicable | — | — | Renovate's Rust release-channel versioning scheme is not implemented as a Rust versioning API. |
| isVersion("$input") === $expected | 23 | not-applicable | — | — | Renovate's Rust release-channel versioning scheme is not implemented as a Rust versioning API. |
| isSingleVersion("$input") === $expected | 40 | not-applicable | — | — | Renovate's Rust release-channel versioning scheme is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 54 | not-applicable | — | — | Renovate's Rust release-channel stability classifier is not implemented as a Rust versioning API. |
| equals("$a", "$b") === $expected | 69 | not-applicable | — | — | Renovate's Rust release-channel comparator is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 85 | not-applicable | — | — | Renovate's Rust release-channel comparator is not implemented as a Rust versioning API. |
| sortVersions("$a", "$b") === $expected | 113 | not-applicable | — | — | Renovate's Rust release-channel comparator is not implemented as a Rust versioning API. |
| getMajor("$version") === $expected | 137 | not-applicable | — | — | Renovate's Rust release-channel component parser is not implemented as a Rust versioning API. |
| getMinor("$version") === $expected | 151 | not-applicable | — | — | Renovate's Rust release-channel component parser is not implemented as a Rust versioning API. |
| getPatch("$version") === $expected | 163 | not-applicable | — | — | Renovate's Rust release-channel component parser is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 176 | not-applicable | — | — | Renovate's Rust release-channel matcher is not implemented as a Rust versioning API. |
| isCompatible("$version", "$current") === $expected | 204 | not-applicable | — | — | Renovate's Rust release-channel host/current compatibility helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 229 | not-applicable | — | — | Renovate's Rust release-channel satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 248 | not-applicable | — | — | Renovate's Rust release-channel satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue({ currentValue: "$currentValue", rangeStrategy: "$rangeStrategy", newVersion: "$newVersion" }) === $expected | 267 | not-applicable | — | — | Renovate's Rust release-channel update-value helper is not implemented as a Rust versioning API. |

---

