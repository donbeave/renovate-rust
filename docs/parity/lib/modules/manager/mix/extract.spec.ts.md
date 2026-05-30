# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mix/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mix/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty for invalid dependency file | 11 | ported | `mix.rs` | `no_deps_function_returns_empty` (+ deps_without_do_end_block) | — |
| extracts all dependencies when no lockfile | 16 | ported | `mix.rs` | `simple_hex_dep` (+ real_world_mix_exs, dep_with_only_option, git_dep_skipped, github_dep_skipped, path_dep_skipped, dep_without_version_skipped) | — |
| extracts all dependencies and adds the locked version if lockfile present | 139 | ported | `mix.rs` | `applies_locked_versions_from_mix_lock` | — |

---

