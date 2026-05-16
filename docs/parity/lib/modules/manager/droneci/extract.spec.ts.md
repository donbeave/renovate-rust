# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/droneci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/droneci/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 8 | ported | `droneci.rs` | `empty_returns_empty` | — |
| extracts multiple image lines | 12 | ported | `droneci.rs` | `extracts_drone_fixture_six_deps` | — |

### `modules/manager/droneci/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts image and replaces registry | 19 | ported | `droneci.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 42 | ported | `droneci.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 65 | ported | `droneci.rs` | `extracts_image_without_double_registry_replacement` | — |

---

