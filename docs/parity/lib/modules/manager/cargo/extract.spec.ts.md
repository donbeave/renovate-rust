# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/extract.spec.ts
**Total tests:** 32 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid toml | 46 | ported | `cargo.rs` | `invalid_toml_returns_error` | — |
| returns null for empty dependencies | 52 | ported | `cargo.rs` | `empty_dependencies_section_returns_empty` | — |
| returns null for empty dev-dependencies | 59 | ported | `cargo.rs` | `empty_dev_dependencies_returns_empty` | — |
| returns null for empty custom target | 66 | ported | `cargo.rs` | `empty_custom_target_returns_empty` | — |
| extracts multiple dependencies simple | 73 | ported | `cargo.rs` | `extracts_simple_string_deps` | — |
| extracts multiple dependencies advanced | 79 | ported | `cargo.rs` | `version_constraint_forms_are_preserved` | — |
| handles inline tables | 85 | ported | `cargo.rs` | `handles_inline_tables` | — |
| handles standard tables | 91 | ported | `cargo.rs` | `extracts_table_deps_with_version` | — |
| extracts platform specific dependencies | 97 | ported | `cargo.rs` | `target_cfg_dependencies_extracted` | — |
| extracts registry urls from .cargo/config.toml | 103 | not-applicable | — | — | Requires async mock filesystem for .cargo/config.toml |
| extracts registry urls from .cargo/config (legacy path) | 112 | not-applicable | — | — | Requires async mock filesystem |
| extracts overridden registry indexes from .cargo/config.toml | 121 | not-applicable | — | — | Requires async mock filesystem |
| extracts overridden source registry indexes from .cargo/config.toml | 180 | not-applicable | — | — | Requires async mock filesystem |
| extracts registries overridden to the default | 205 | not-applicable | — | — | Requires async mock filesystem |
| extracts registries with an empty config.toml | 249 | not-applicable | — | — | Requires async mock filesystem |
| extracts registry urls from environment | 299 | not-applicable | — | — | Requires environment variable injection |
| extracts workspace dependencies | 345 | ported | `cargo.rs` | `workspace_dependencies_extracted` | — |
| skips workspace dependency | 390 | ported | `cargo.rs` | `workspace_dep_is_skipped` | — |
| skips unknown registries | 407 | not-applicable | — | — | Requires async mock filesystem |
| fails to parse cargo config with invalid TOML | 415 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config registries with missing index | 424 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config source replaced registries with missing index | 433 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config with circular registry source replacements | 481 | not-applicable | — | — | Requires async mock filesystem |
| extracts original package name of renamed dependencies | 539 | ported | `cargo.rs` | `renamed_dep_extracts_original_package_name` | — |
| extracts locked versions | 549 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| does not extract locked versions for git dependencies | 567 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| extracts locked versions for renamed packages | 585 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles missing locked versions | 601 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles invalid versions in the toml file | 617 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles invalid lock file | 635 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| should extract project version | 650 | ported | `cargo.rs` | `extracts_project_version` | — |
| should extract project version from workspace | 664 | ported | `cargo.rs` | `extracts_project_version_from_workspace` | — |

---

