# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/swift/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/swift/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/swift/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$version") === $expected | 14 | not-applicable | — | — | Renovate's Swift version classifier is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| isValid("$version") === $expected | 24 | not-applicable | — | — | Renovate's Swift version/range validation is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| minSatisfyingVersion($versions, "$range") === "$expected" | 62 | not-applicable | — | — | Renovate's Swift satisfying-version helper is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| getSatisfyingVersion($versions, "$range") === "$expected" | 74 | not-applicable | — | — | Renovate's Swift satisfying-version helper is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| isLessThanRange("$version", "$range") === "$expected" | 87 | not-applicable | — | — | Renovate's Swift range comparison helper is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| matches("$version", "$range") === "$expected" | 101 | not-applicable | — | — | Renovate's Swift range matcher is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 117 | not-applicable | — | — | Renovate's Swift update-value helper is not implemented as a Rust versioning API; Rust Swift support is extractor oriented. |

---

