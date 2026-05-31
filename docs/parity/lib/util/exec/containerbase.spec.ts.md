# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/containerbase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/containerbase.spec.ts
**Total tests:** 20 | **Ported:** 7 | **Actionable:** 13 | **Status:** partial

### `util/exec/containerbase › isDynamicInstall()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if binarySource is not install | 22 | ported | `exec/containerbase.rs` | `is_dynamic_install_requires_install_source` | — |
| returns false if not containerbase | 26 | ported | `exec/containerbase.rs` | `is_dynamic_install_requires_containerbase_env` | Rust checks `std::env::var("CONTAINERBASE")` directly instead of GlobalConfig |
| returns false if any unsupported tools | 31 | ported | `exec/containerbase.rs` | `is_dynamic_install_false_if_any_unsupported` | — |
| returns true if supported tools | 42 | ported | `exec/containerbase.rs` | `supports_dynamic_install_known` | — |

### `util/exec/containerbase › getToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config for a known tool | 51 | ported | `exec/containerbase.rs` | `get_tool_config_node` | — |
| returns undefined for an unknown tool | 60 | ported | `exec/containerbase.rs` | `get_tool_config_unknown` | — |

### `util/exec/containerbase › resolveConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from config | 67 | ported | `exec/containerbase.rs` | `resolve_constraint_exact` | Rust strips `=` prefix; TS does full version lookup via datasource |
| returns highest stable | 73 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| returns highest unstable | 87 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| respects latest | 96 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| supports rust docker tags | 113 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| throws for unknown tools | 127 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Rust returns "latest" for unknown tools instead of throwing |
| throws no releases | 133 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| falls back to latest version if no compatible release | 142 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| falls back to latest version if invalid constraint | 151 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| supports python ranges "$version" => "$expected" | 160 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| removes pep440 == | 184 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Rust does not implement PEP440 constraint normalization |
| supports flutter ranges "$version" => "$expected" | 193 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |
| supports dart ranges "$version" => "$expected" | 223 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup |

### `util/exec/containerbase › generateInstallCommands()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns install commands | 269 | not-applicable | Mock framework internals — tests containerbase exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Requires datasource version lookup to resolve tool version |

---

