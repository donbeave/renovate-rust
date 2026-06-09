# `lib/modules/manager/cargo/locked-version.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns null for missing lock file | ported | [`crates/renovate-core/src/versioning/cargo.rs:1477`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1477) |
| 23 | returns null for invalid lock file | ported | [`crates/renovate-core/src/versioning/cargo.rs:1483`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1483) |
| 28 | returns empty map for lock file without packages | ported | [`crates/renovate-core/src/versioning/cargo.rs:1489`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1489) |
| 33 | returns a map of package versions | ported | [`crates/renovate-core/src/versioning/cargo.rs:1496`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1496) |
| 51 | parses v1 lockfile string into an object | ported | [`crates/renovate-core/src/versioning/cargo.rs:1422`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1422) |
| 70 | parses v2 lockfile string into an object | ported | [`crates/renovate-core/src/versioning/cargo.rs:1442`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1442) |
| 88 | parses v3 lockfile string into an object | ported | [`crates/renovate-core/src/versioning/cargo.rs:1459`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1459) |
| 106 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/versioning/cargo.rs:1471`](../../../../../../../crates/renovate-core/src/versioning/cargo.rs#L1471) |

