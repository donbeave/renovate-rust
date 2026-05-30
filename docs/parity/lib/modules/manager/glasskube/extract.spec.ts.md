# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/glasskube/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/glasskube/extract.spec.ts
**Total tests:** 5 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract version and registryUrl | 43 | ported | `glasskube.rs` | `extracts_cluster_package` (+ `extracts_multiple_packages`) | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty packageFiles | 62 | ported | `glasskube.rs` | `extract_all_returns_empty_for_empty_input` | — |
| should skip package with non-existing repo | 67 | ported | `glasskube.rs` | `skips_non_glasskube_files` | — |
| should extract registryUrl from repo in other file | 85 | not-applicable | — | — | multi-file extraction — tests that extractAllPackageFiles reads repo.yaml to get registryUrl and applies to package.yaml; Rust implementation doesn't support multi-file extraction patterns; would require extractAllPackageFiles to read and parse multiple YAML files then merge results |
| should extract registryUrl from default repo in other file | 107 | not-applicable | — | — | multi-file extraction — tests that extractAllPackageFiles reads repo.yaml to get registryUrl and applies to package.yaml; Rust implementation doesn't support multi-file extraction patterns; would require extractAllPackageFiles to read and parse multiple YAML files then merge results |

---

