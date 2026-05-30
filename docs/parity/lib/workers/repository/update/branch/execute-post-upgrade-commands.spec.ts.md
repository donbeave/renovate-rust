# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles an artifact which is a directory | 34 | pending | — | — | — |
| executes commands on update package files | 92 | pending | — | — | — |
| does not execute command with shell mode by default | 140 | pending | — | — | — |
| executes command with shell mode when allowShellExecutorForPostUpgradeCommands=true | 197 | pending | — | — | — |
| does not execute command with shell mode when allowShellExecutorForPostUpgradeCommands=false | 255 | pending | — | — | — |
| creates data file for commands | 313 | pending | — | — | — |
| should not create data file if no commands given | 375 | pending | — | — | — |
| logs files which do not match fileFilters | 426 | pending | — | — | — |
| excludes .npmrc files when npmrc config is present | 480 | pending | — | — | — |
| handles previously-deleted files which are re-added | 528 | pending | — | — | — |
| does not add back files that are renamed | 576 | pending | — | — | — |
| retains previously deleted files too | 666 | pending | — | — | — |
| passes git environment variables to exec | 754 | pending | — | — | — |
| uses workingDirTemplate when provided | 817 | pending | — | — | — |
| uses localDir when workingDirTemplate is not provided | 869 | pending | — | — | — |
| installed tool constraints that match package constraints are filtered out before templating | 919 | pending | — | — | — |

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor › when using installTools`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs when skipping a constraint that isn't a known tool | 1036 | pending | — | — | — |
| logs when skipping a value that isn't a known constraint | 1109 | pending | — | — | — |

---
