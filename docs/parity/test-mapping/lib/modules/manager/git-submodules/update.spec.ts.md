# `lib/modules/manager/git-submodules/update.spec.ts`

[← `manager/git-submodules`](../../../../_by-module/manager/git-submodules.md) · [all modules](../../../../README.md)

**3/6 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 49 | returns null on error | ported | `crates/renovate-core/src/extractors/git_submodules.rs:607` |
| 60 | returns content on update | pending | — |
| 72 | returns content on update and uses git environment variables | pending | — |
| 107 | update gitmodule branch value if value changed | ported | `crates/renovate-core/src/extractors/git_submodules.rs:552` |
| 136 | do not update gitmodule branch value if value not changed | ported | `crates/renovate-core/src/extractors/git_submodules.rs:573` |
| 154 | returns content on update and uses git environment variables for git-tags/git-refs | pending | — |

