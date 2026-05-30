# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kotlin-script/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kotlin-script/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts dependencies in a generic case | 12 | ported | `kotlin_script.rs` | `extracts_generic_case_fixture_three_deps` (+ extracts_single_dep, extracts_multiple_deps) | — |
| detects custom repository definitions | 43 | ported | `kotlin_script.rs` | `extracts_custom_repositories` | — |
| no dependencies | 71 | ported | `kotlin_script.rs` | `no_annotations_returns_empty` | — |
| skips dependencies with missing parts | 81 | ported | `kotlin_script.rs` | `skips_missing_parts` | — |

---

