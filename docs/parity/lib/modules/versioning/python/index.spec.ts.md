# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/python/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/python/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/python/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | not-applicable | — | — | Renovate's Python legacy versioning scheme is not implemented as a Rust versioning API; Rust only has a separate PEP 440 summary helper. |
| matches("$version", "$range") === "$expected" | 28 | not-applicable | — | — | Renovate's Python legacy range matcher is not implemented as a Rust versioning API. |
| isLessThanRange("$version", "$range") === "$expected" | 54 | not-applicable | — | — | Renovate's Python legacy range comparison helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === $expected | 66 | not-applicable | — | — | Renovate's Python legacy satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 83 | not-applicable | — | — | Renovate's Python legacy satisfying-version helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 97 | not-applicable | — | — | Renovate's Python legacy update-value helper is not implemented as a Rust versioning API. |
| subset("$a", "$b") === $expected | 160 | not-applicable | — | — | Renovate's Python legacy range subset helper is not implemented as a Rust versioning API. |
| isBreaking("$currentVersion", "$newVersion") === $expected | 182 | not-applicable | — | — | Renovate's Python legacy breaking-change helper is not implemented as a Rust versioning API. |

---

