# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gleam/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gleam/extract.spec.ts
**Total tests:** 9 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `modules/manager/gleam/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract dev and prod dependencies | 8 | ported | `gleam.rs` | `both_sections` | — |
| should extract dev only dependencies | 41 | ported | `gleam.rs` | `extracts_dev_dependencies` | — |
| should return null when no dependencies are found | 65 | ported | `gleam.rs` | `no_deps_section_returns_empty` | — |
| should return null when gleam.toml is invalid | 82 | ported | `gleam.rs` | `invalid_toml_returns_empty` | — |
| should return locked versions | 91 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should fail to extract locked version | 119 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should fail to find locked version in range | 138 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should handle invalid versions in lock file | 166 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should handle lock file parsing and extracting errors | 193 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |

---

