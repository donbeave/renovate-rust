# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/jsonnet-bundler/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/jsonnet-bundler/extract.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid jsonnetfile | 24 | ported | `jsonnet_bundler.rs` | `invalid_json_returns_empty` | — |
| returns null for jsonnetfile with no dependencies | 30 | ported | `jsonnet_bundler.rs` | `empty_returns_empty` | — |
| returns null for local dependencies | 36 | ported | `jsonnet_bundler.rs` | `local_deps_returns_empty` | — |
| returns null for vendored dependencies | 42 | ported | `jsonnet_bundler.rs` | `vendored_dependencies_return_empty` | — |
| returns null for dependencies with empty Git source | 48 | ported | `jsonnet_bundler.rs` | `empty_git_source_returns_empty` | — |
| extracts dependency | 57 | ported | `jsonnet_bundler.rs` | `extracts_github_deps` (+ extracts_main_fixture_two_deps) | — |
| extracts dependency with custom name | 79 | ported | `jsonnet_bundler.rs` | `extracts_dep_with_optional_name_field_uses_path_name` | — |

---

