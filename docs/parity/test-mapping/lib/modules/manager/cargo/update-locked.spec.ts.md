# `lib/modules/manager/cargo/update-locked.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | detects already updated | ported | `crates/renovate-core/src/versioning/cargo.rs:1514` |
| 21 | returns unsupported for empty lockfile | ported | `crates/renovate-core/src/versioning/cargo.rs:1530` |
| 32 | returns unsupported for empty depname | ported | `crates/renovate-core/src/versioning/cargo.rs:1541` |
| 44 | returns unsupported | ported | `crates/renovate-core/src/versioning/cargo.rs:1554` |
| 56 | returns update-failed in case of errors | ported | `crates/renovate-core/src/versioning/cargo.rs:1567` |

