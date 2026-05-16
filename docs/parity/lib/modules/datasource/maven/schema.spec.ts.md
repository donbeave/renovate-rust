# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/schema.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/maven/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| trims release metadata to the fields used by Renovate | 6 | not-applicable | — | — | Renovate's Maven XML trimming/cache schema is not exposed as a Rust datasource API; Rust parses latest version directly from metadata XML. |
| trims snapshot metadata to the fields used by Renovate | 30 | not-applicable | — | — | Renovate's Maven XML trimming/cache schema is not exposed as a Rust datasource API; Rust parses latest version directly from metadata XML. |
| trims pom files to the fields used by Renovate | 47 | not-applicable | — | — | Renovate's Maven POM XML trimming/cache schema is not implemented in Rust. |
| preserves empty relocation tags | 99 | not-applicable | — | — | Renovate's Maven POM XML trimming/cache schema is not implemented in Rust. |
| passes through unknown XML unchanged | 120 | not-applicable | — | — | Renovate's Maven XML trimming/cache schema is not exposed as a Rust datasource API. |
| passes through prefixed pom XML unchanged | 125 | not-applicable | — | — | Renovate's Maven POM XML trimming/cache schema is not implemented in Rust. |
| passes through pom XML when no retained fields are present | 131 | not-applicable | — | — | Renovate's Maven POM XML trimming/cache schema is not implemented in Rust. |
| passes through metadata XML when no retained fields are present | 136 | not-applicable | — | — | Renovate's Maven XML trimming/cache schema is not exposed as a Rust datasource API. |
| passes through invalid XML unchanged | 141 | not-applicable | — | — | Renovate's Maven XML trimming/cache schema is not exposed as a Rust datasource API. |

---

