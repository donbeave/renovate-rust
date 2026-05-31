# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles an artifact which is a directory  | 34 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| executes commands on update package files  | 92 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| does not execute command with shell mode by default  | 140 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| executes command with shell mode when allowShellExecutorForPostUpgradeCommands=true  | 197 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| does not execute command with shell mode when allowShellExecutorForPostUpgradeCommands=false  | 255 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| creates data file for commands  | 313 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| should not create data file if no commands given  | 375 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| logs files which do not match fileFilters  | 426 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| excludes .npmrc files when npmrc config is present  | 480 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| handles previously-deleted files which are re-added  | 528 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| does not add back files that are renamed  | 576 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| retains previously deleted files too  | 666 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| passes git environment variables to exec  | 754 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| uses workingDirTemplate when provided  | 817 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| uses localDir when workingDirTemplate is not provided  | 869 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| installed tool constraints that match package constraints are filtered out before templating  | 919 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor › when using installTools`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs when skipping a constraint that isn't a known tool  | 1036 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |
| logs when skipping a value that isn't a known constraint  | 1109 | not-applicable | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer | — | Mock framework internals — tests post-upgrade commands via vitest-mocked exec/fs; Rust tests this at different layer |

---
