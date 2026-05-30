# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/setup-cfg/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/setup-cfg/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `setup_cfg.rs` | `empty_content_returns_no_deps` | — |
| extracts dependencies | 10 | ported | `setup_cfg.rs` | `extracts_install_requires` (+ extracts_setup_requires, extracts_tests_require, extracts_extras_require, skips_git_source, normalizes_package_name, strips_env_markers, ignores_unrelated_sections) | — |

---

