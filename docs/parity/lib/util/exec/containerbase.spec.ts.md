# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/containerbase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/containerbase.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** done

### `util/exec/containerbase › isDynamicInstall()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if binarySource is not install | 22 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns false if not containerbase | 26 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns false if any unsupported tools | 31 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns true if supported tools | 42 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/containerbase › getToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config for a known tool | 51 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns undefined for an unknown tool | 60 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/containerbase › resolveConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from config | 67 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns highest stable | 73 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| returns highest unstable | 87 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| respects latest | 96 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| supports rust docker tags | 113 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| throws for unknown tools | 127 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| throws no releases | 133 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| falls back to latest version if no compatible release | 142 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| falls back to latest version if invalid constraint | 151 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| supports python ranges "$version" => "$expected" | 160 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| removes pep440 == | 184 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| supports flutter ranges "$version" => "$expected" | 193 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| supports dart ranges "$version" => "$expected" | 223 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/containerbase › generateInstallCommands()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns install commands | 269 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

---

