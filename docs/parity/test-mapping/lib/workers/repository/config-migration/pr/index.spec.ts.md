# `lib/workers/repository/config-migration/pr/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/16 in-scope tests ported** (15 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 52 | creates pr | ported | [`crates/renovate-core/src/branch.rs:2908`](../../../../../../../../crates/renovate-core/src/branch.rs#L2908) |
| 59 | creates pr with default pr title | pending | — |
| 66 | founds an open pr and as it is up to date and returns | pending | — |
| 76 | founds an open pr and updates it | pending | — |
| 85 | updates an open pr with unexpected pr title | pending | — |
| 96 | dry runs and does not update out of date pr | pending | — |
| 116 | creates pr in dry run mode | pending | — |
| 128 | creates pr with labels | pending | — |
| 144 | creates pr with empty footer and header | pending | — |
| 157 | creates pr for json5 config file | pending | — |
| 167 | creates pr with footer and header with trailing and leading newlines | pending | — |
| 181 | creates non-semantic pr title | pending | — |
| 197 | creates semantic pr title | pending | — |
| 215 | creates pr with footer and header using templating | pending | — |
| 250 | throws when trying to create a new pr | pending | — |
| 256 | deletes branch when pr already exists but cannot find it | pending | — |

