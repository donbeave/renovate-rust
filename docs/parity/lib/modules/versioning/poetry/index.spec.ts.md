# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/poetry/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/poetry/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/poetry/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$a", "$b") === $expected | 5 | not-applicable | — | — | Renovate's Poetry versioning scheme is not implemented as a Rust versioning API; Rust Poetry support is extractor oriented. |
| getMajor, getMinor, getPatch for "$version" | 28 | not-applicable | — | — | Renovate's Poetry component parser is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 47 | not-applicable | — | — | Renovate's Poetry comparator is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 82 | not-applicable | — | — | Renovate's Poetry stability classifier is not implemented as a Rust versioning API. |
| isVersion("$version") === $expected | 95 | not-applicable | — | — | Renovate's Poetry version classifier is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 107 | not-applicable | — | — | Renovate's Poetry version/range validation is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 134 | not-applicable | — | — | Renovate's Poetry single-version classifier is not implemented as a Rust versioning API. |
| matches("$version", "$range") === "$expected" | 145 | not-applicable | — | — | Renovate's Poetry range matcher is not implemented as a Rust versioning API. |
| isLessThanRange("$version", "$range") === "$expected" | 167 | not-applicable | — | — | Renovate's Poetry range comparison helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 178 | not-applicable | — | — | Renovate's Poetry satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 194 | not-applicable | — | — | Renovate's Poetry satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 207 | not-applicable | — | — | Renovate's Poetry update-value helper is not implemented as a Rust versioning API. |
| sortVersions("$a", "$b") === $expected | 269 | not-applicable | — | — | Renovate's Poetry sorting comparator is not implemented as a Rust versioning API. |
| subset("$a", "$b") === $expected | 287 | not-applicable | — | — | Renovate's Poetry range subset helper is not implemented as a Rust versioning API. |

---

