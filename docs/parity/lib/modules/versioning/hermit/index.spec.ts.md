# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hermit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hermit/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/hermit/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$version") === $expected | 6 | not-applicable | — | — | Renovate's Hermit versioning scheme is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| isValid("$version") === $expected | 19 | not-applicable | — | — | Renovate's Hermit versioning validation is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| getMajor, getMinor, getPatch for "$version" | 46 | not-applicable | — | — | Renovate's Hermit version component parser is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| equals("$version", "$other") === $expected | 65 | not-applicable | — | — | Renovate's Hermit comparator is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| matches("$version", "$range") === $expected | 83 | not-applicable | — | — | Renovate's Hermit range matcher is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| isGreaterThan("$version", "$other") === $expected | 110 | not-applicable | — | — | Renovate's Hermit comparator is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| isLessThanRange("$version", "$other") === $expected | 139 | not-applicable | — | — | Renovate's Hermit range comparison helper is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| getSatisfyingVersion | 166 | not-applicable | — | — | Renovate's Hermit satisfying-version helper is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| minSatisfyingVersion | 184 | not-applicable | — | — | Renovate's Hermit satisfying-version helper is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |
| sorts versions in an ascending order | 203 | not-applicable | — | — | Renovate's Hermit sorting comparator is not implemented as a Rust versioning API; Rust Hermit support is extractor/datasource oriented. |

---

