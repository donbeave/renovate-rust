# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deps-edn/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deps-edn/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid file | 6 | ported | `deps_edn.rs` | `invalid_edn_returns_empty` | — |
| extractPackageFile | 10 | ported | `deps_edn.rs` | `extracts_deps` (+ `skips_git_deps`, `skips_local_deps`, `extracts_alias_deps`) | — |

---

