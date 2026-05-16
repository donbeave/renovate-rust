# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/glasskube/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/glasskube/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/glasskube/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$version") === $expected | 6 | not-applicable | — | — | Renovate's Glasskube versioning scheme is not implemented as a Rust versioning API; Rust Glasskube support is extractor/datasource oriented. |
| isValid("$version") === $expected | 16 | not-applicable | — | — | Renovate's Glasskube versioning validation is not implemented as a Rust versioning API; Rust Glasskube support is extractor/datasource oriented. |
| getMajor, getMinor, getPatch for "$version" | 30 | not-applicable | — | — | Renovate's Glasskube version component parser is not implemented as a Rust versioning API; Rust Glasskube support is extractor/datasource oriented. |
| getMajor, getMinor, getPatch for "$version" | 44 | not-applicable | — | — | Renovate's Glasskube comparator is not implemented as a Rust versioning API; Rust Glasskube support is extractor/datasource oriented. |

---

