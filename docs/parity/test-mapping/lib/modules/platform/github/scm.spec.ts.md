# `lib/modules/platform/github/scm.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | platformcommit = disabled => delegate to git | ported | `crates/renovate-core/src/platform/scm.rs:161` |
| 39 | platformcommit = enabled => delegate to github | ported | `crates/renovate-core/src/platform/scm.rs:171` |
| 52 | platformcommit = auto => delegate to git | ported | `crates/renovate-core/src/platform/scm.rs:178` |
| 65 | platformcommit = auto and is a github app => delegate to github | ported | `crates/renovate-core/src/platform/scm.rs:184` |

