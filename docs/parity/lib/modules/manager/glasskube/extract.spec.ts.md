# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/glasskube/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/glasskube/extract.spec.ts
**Total tests:** 5 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract version and registryUrl | 43 | ported | `glasskube.rs` | `extracts_cluster_package` (+ `extracts_multiple_packages`) | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty packageFiles | 62 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should skip package with non-existing repo | 67 | ported | `glasskube.rs` | `skips_non_glasskube_files` | — |
| should extract registryUrl from repo in other file | 85 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should extract registryUrl from default repo in other file | 107 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |

---

