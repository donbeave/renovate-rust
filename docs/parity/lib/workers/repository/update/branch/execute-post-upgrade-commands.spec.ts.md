# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** done

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles an artifact which is a directory | 34 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| executes commands on update package files | 92 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| does not execute command with shell mode by default | 140 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| executes command with shell mode when allowShellExecutorForPostUpgradeCommands=true | 197 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| does not execute command with shell mode when allowShellExecutorForPostUpgradeCommands=false | 255 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| creates data file for commands | 313 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| should not create data file if no commands given | 375 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| logs files which do not match fileFilters | 426 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| excludes .npmrc files when npmrc config is present | 480 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| handles previously-deleted files which are re-added | 528 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| does not add back files that are renamed | 576 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| retains previously deleted files too | 666 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| passes git environment variables to exec | 754 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| uses workingDirTemplate when provided | 817 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| uses localDir when workingDirTemplate is not provided | 869 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| installed tool constraints that match package constraints are filtered out before templating | 919 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |

### `workers/repository/update/branch/execute-post-upgrade-commands › postUpgradeCommandsExecutor › when using installTools`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs when skipping a constraint that isn't a known tool | 1036 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |
| logs when skipping a value that isn't a known constraint | 1109 | not-applicable | — | — | Requires vi.mock exec/fs/scm mock infrastructure |

---
