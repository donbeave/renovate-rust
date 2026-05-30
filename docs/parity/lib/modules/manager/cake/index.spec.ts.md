# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cake/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cake/index.spec.ts
**Total tests:** 5 | **Ported:** 3 | **Actionable:** 5 | **Status:** partial

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts | 21 | ported | `cake.rs` | `extracts_build_cake_fixture` | — |
| extracts dotnet tools from single sdk style build file | 45 | ported | `cake.rs` | `extracts_install_tools_dotnet` | — |
| skips invalid entries in InstallTools | 101 | ported | `cake.rs` | `skips_invalid_install_tools_entries` | — |
| calls applyRegistries to honor nuget.config files if present for .cake files | 124 | pending | — | — | — |
| calls applyRegistries to honor nuget.config files if present for InstallTools | 141 | pending | — | — | — |

---
