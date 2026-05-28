# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/extract/catalog.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract/catalog.spec.ts
**Total tests:** 6 | **Ported:** 3 | **Actionable:** 6 | **Status:** partial

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports versions declared as single string | 5 | ported | `extractors/gradle.rs` | `catalog_inline_string_form` (+ 5 variants) | — |
| deletes commit message for plugins with version reference | 134 | pending | — | — | — |
| ignores empty TOML file | 180 | ported | `extractors/gradle.rs` | `catalog_empty_toml_returns_empty` | — |
| skips version entries with no resolvable literal value | 185 | ported | `extractors/gradle.rs` | `catalog_skips_non_literal_versions` | — |
| changes the dependency version, not the comment version | 203 | pending | — | — | — |
| supports templated toml | 254 | pending | — | — | — |

---

