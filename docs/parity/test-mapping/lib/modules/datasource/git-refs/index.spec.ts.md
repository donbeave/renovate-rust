# `lib/modules/datasource/git-refs/index.spec.ts`

[← `datasource/git-refs`](../../../../_by-module/datasource/git-refs.md) · [all modules](../../../../README.md)

**8/11 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 38 | returns nil if response is wrong | ported | `crates/renovate-core/src/datasources/git_refs.rs:189` |
| 48 | returns nil if response is malformed | ported | `crates/renovate-core/src/datasources/git_refs.rs:197` |
| 58 | returns nil if remote call throws exception | ported | `crates/renovate-core/src/datasources/git_refs.rs:209` |
| 68 | returns versions filtered from tags | ported | `crates/renovate-core/src/datasources/git_refs.rs:217` |
| 82 | returns null if not found | ported | `crates/renovate-core/src/datasources/git_refs.rs:260` |
| 92 | returns digest for tag | ported | `crates/renovate-core/src/datasources/git_refs.rs:267` |
| 104 | ignores refs/for/ | ported | `crates/renovate-core/src/datasources/git_refs.rs:275` |
| 114 | returns digest for head | ported | `crates/renovate-core/src/datasources/git_refs.rs:284` |
| 124 | calls simplegit with emptyenv if no hostrules exist | pending | — |
| 135 | calls simplegit with git envs if hostrules exist | pending | — |
| 162 | calls simplegit with git envs if hostrules exist for datasource type git-refs | pending | — |

