# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mint/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mint/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `mint.rs` | `empty_returns_empty` | — |
| Mintfile With Version Description | 10 | ported | `mint.rs` | `extracts_deps_with_version` | — |
| Mintfile Without Version Description | 41 | ported | `mint.rs` | `extracts_deps_without_version_as_skipped` | — |
| Complex Mintfile | 61 | ported | `mint.rs` | `complex_mintfile_mixed` | — |
| Mintfile Includes Commented Out | 86 | ported | `mint.rs` | `comment_lines_skipped` | — |

---

