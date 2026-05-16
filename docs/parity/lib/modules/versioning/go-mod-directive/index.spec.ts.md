# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/go-mod-directive/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/go-mod-directive/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/go-mod-directive/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === "$expected" | 4 | not-applicable | — | — | Renovate's Go module directive versioning API is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| getSatisfyingVersion($versions, "$range") === "$expected" | 19 | not-applicable | — | — | Renovate's Go module directive satisfying-version helper is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| isValid("$version") === $expected | 29 | not-applicable | — | — | Renovate's Go module directive validation helper is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| isVersion("$version") === $expected | 38 | not-applicable | — | — | Renovate's Go module directive version classifier is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| isLessThanRange("$version", "$range") === "$expected" | 47 | not-applicable | — | — | Renovate's Go module directive range comparison helper is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| minSatisfyingVersion($versions, "$range") === "$expected" | 58 | not-applicable | — | — | Renovate's Go module directive satisfying-version helper is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 69 | not-applicable | — | — | Renovate's Go module directive update-value helper is not implemented in Rust; Rust Go module support is extractor/datasource oriented. |

---

