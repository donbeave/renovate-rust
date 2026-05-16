# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazelisk/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazelisk/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `bazelisk_returns_dep_for_version` | — |
| supports ranges | 17 | ported | `version_file.rs` | `bazelisk_supports_partial_version` | — |
| skips non ranges | 29 | ported | `version_file.rs` | `bazelisk_passes_through_non_version_string` | — |
| ignores comments past the first line | 41 | ported | `version_file.rs` | `bazelisk_ignores_comments_past_first_line` | — |

---

