# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nodenv/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nodenv/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `nodenv_returns_dep_for_version` | — |
| supports ranges | 16 | ported | `version_file.rs` | `nodenv_supports_partial_version` | — |
| skips non ranges | 27 | ported | `version_file.rs` | `nodenv_passes_through_non_version_string` | — |

---

