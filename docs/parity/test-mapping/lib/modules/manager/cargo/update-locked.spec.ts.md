# `lib/modules/manager/cargo/update-locked.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | detects already updated | ported | [`crates/renovate-core/src/versioning/cargo.rs:1515`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1515) |
| 21 | returns unsupported for empty lockfile | ported | [`crates/renovate-core/src/versioning/cargo.rs:1531`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1531) |
| 32 | returns unsupported for empty depname | ported | [`crates/renovate-core/src/versioning/cargo.rs:1542`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1542) |
| 44 | returns unsupported | ported | [`crates/renovate-core/src/versioning/cargo.rs:1555`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1555) |
| 56 | returns update-failed in case of errors | ported | [`crates/renovate-core/src/versioning/cargo.rs:1568`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1568) |

