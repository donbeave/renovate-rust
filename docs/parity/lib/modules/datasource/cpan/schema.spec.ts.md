# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/cpan/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cpan/schema.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `modules/datasource/cpan/schema › MetaCpanApiFileSearchResponse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters out entries with empty module arrays | 5 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `schema_filters_empty_module_array` | Empty module vec → None |
| filters out entries where module has no version | 29 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `schema_filters_empty_version` | Empty version string → None |
| includes valid entries | 53 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `schema_includes_valid_entries` | Valid entry → release with correct fields |

---
