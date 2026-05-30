# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmsman/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmsman/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if empty | 9 | ported | `helmsman.rs` | `empty_file_returns_empty` | — |
| returns null if extracting non helmsman yaml file | 16 | ported | `helmsman.rs` | `app_with_no_chart_or_version_has_skip_reason` | — |
| returns null if apps not defined | 23 | ported | `helmsman.rs` | `invalid_yaml_returns_empty` | — |
| extract deps | 29 | ported | `helmsman.rs` | `extract_deps_validhelmsfile` (+ `extracts_helm_deps`, `skips_missing_version`, `skips_unknown_repo`) | — |

---

