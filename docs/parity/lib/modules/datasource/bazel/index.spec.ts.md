# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bazel/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bazel/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/datasource/bazel/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 26 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `throws_for_network_error` | network error → Err |
| returns null for 404 | 33 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `returns_null_for_404` | 404 → None |
| returns null for empty result | 38 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `returns_null_for_empty_result` | `{}` → None |
| returns null for empty 200 OK | 43 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `returns_null_for_empty_versions` | empty versions array → None |
| throws for 5xx | 51 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `throws_for_5xx` | 502 → Err |
| metadata without yanked versions | 58 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `metadata_without_yanked_versions` | releases + sourceUrl from homepage |
| metadata with yanked versions | 77 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `metadata_with_yanked_versions` | yanked → isDeprecated true |
| should handle local file correctly | 106 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `local_file_handles_correctly` | file:// URL → local fs read |
| should return null for invalid file path | 135 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `local_file_returns_null_for_invalid_path` | nonexistent path → None |
| should return null for empty file content | 146 | ported | `crates/renovate-core/src/datasources/bazel.rs` | `local_file_returns_null_for_empty_content` | empty file → None |

---
