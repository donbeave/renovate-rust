# `lib/modules/manager/cargo/locked-version.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 19 | returns null for missing lock file | ported | `crates/renovate-core/src/versioning/cargo.rs:1476` |
| 23 | returns null for invalid lock file | ported | `crates/renovate-core/src/versioning/cargo.rs:1482` |
| 28 | returns empty map for lock file without packages | ported | `crates/renovate-core/src/versioning/cargo.rs:1488` |
| 33 | returns a map of package versions | ported | `crates/renovate-core/src/versioning/cargo.rs:1495` |
| 51 | parses v1 lockfile string into an object | ported | `crates/renovate-core/src/versioning/cargo.rs:1421` |
| 70 | parses v2 lockfile string into an object | ported | `crates/renovate-core/src/versioning/cargo.rs:1441` |
| 88 | parses v3 lockfile string into an object | ported | `crates/renovate-core/src/versioning/cargo.rs:1458` |
| 106 | can deal with invalid lockfiles | ported | `crates/renovate-core/src/versioning/cargo.rs:1470` |

