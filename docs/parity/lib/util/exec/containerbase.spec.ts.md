# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/containerbase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/containerbase.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `util/exec/containerbase › isDynamicInstall()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if binarySource is not install | 22 | pending | — | — | —|
| returns false if not containerbase | 26 | pending | — | — | —|
| returns false if any unsupported tools | 31 | pending | — | — | —|
| returns true if supported tools | 42 | pending | — | — | —|

### `util/exec/containerbase › getToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config for a known tool | 51 | pending | — | — | —|
| returns undefined for an unknown tool | 60 | pending | — | — | —|

### `util/exec/containerbase › resolveConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from config | 67 | pending | — | — | —|
| returns highest stable | 73 | pending | — | — | —|
| returns highest unstable | 87 | pending | — | — | —|
| respects latest | 96 | pending | — | — | —|
| supports rust docker tags | 113 | pending | — | — | —|
| throws for unknown tools | 127 | pending | — | — | —|
| throws no releases | 133 | pending | — | — | —|
| falls back to latest version if no compatible release | 142 | pending | — | — | —|
| falls back to latest version if invalid constraint | 151 | pending | — | — | —|
| supports python ranges "$version" => "$expected" | 160 | pending | — | — | —|
| removes pep440 == | 184 | pending | — | — | —|
| supports flutter ranges "$version" => "$expected" | 193 | pending | — | — | —|
| supports dart ranges "$version" => "$expected" | 223 | pending | — | — | —|

### `util/exec/containerbase › generateInstallCommands()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns install commands | 269 | pending | — | — | —|

---

