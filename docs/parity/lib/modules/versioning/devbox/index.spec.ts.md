# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/devbox/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/devbox/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/devbox/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$version") === $expected | 4 | not-applicable | — | — | Renovate's Devbox versioning scheme is not implemented as a Rust versioning API; Rust Devbox support is extractor/datasource oriented. |
| isValid("$version") === $isValid | 34 | not-applicable | — | — | Renovate's Devbox version/range validation is not implemented as a Rust versioning API; Rust Devbox support is extractor/datasource oriented. |
| matches("$version", "$range") === $expected | 64 | not-applicable | — | — | Renovate's Devbox range matcher is not implemented as a Rust versioning API; Rust Devbox support is extractor/datasource oriented. |
| equals("$version", "$range") === $expected | 84 | not-applicable | — | — | Renovate's Devbox comparator is not implemented as a Rust versioning API; Rust Devbox support is extractor/datasource oriented. |

---

