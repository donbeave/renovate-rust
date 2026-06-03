# `lib/workers/repository/update/branch/execute-post-upgrade-commands.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/17 in-scope tests ported** (17 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 34 | handles an artifact which is a directory | pending | — |
| 92 | executes commands on update package files | pending | — |
| 140 | does not execute command with shell mode by default | pending | — |
| 197 | executes command with shell mode when allowshellexecutorforpostupgradecommands=true | pending | — |
| 255 | does not execute command with shell mode when allowshellexecutorforpostupgradecommands=false | pending | — |
| 313 | creates data file for commands | pending | — |
| 375 | should not create data file if no commands given | pending | — |
| 426 | logs files which do not match filefilters | pending | — |
| 480 | excludes .npmrc files when npmrc config is present | pending | — |
| 528 | handles previously-deleted files which are re-added | pending | — |
| 576 | does not add back files that are renamed | pending | — |
| 666 | retains previously deleted files too | pending | — |
| 754 | passes git environment variables to exec | pending | — |
| 817 | uses workingdirtemplate when provided | pending | — |
| 869 | uses localdir when workingdirtemplate is not provided | pending | — |
| 1036 | logs when skipping a constraint that isn't a known tool | pending | — |
| 1109 | logs when skipping a value that isn't a known constraint | pending | — |

