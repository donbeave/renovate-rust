# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2078`](../../../../../../../crates/renovate-core/src/branch.rs#L2078) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2084`](../../../../../../../crates/renovate-core/src/branch.rs#L2084) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2093`](../../../../../../../crates/renovate-core/src/branch.rs#L2093) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2102`](../../../../../../../crates/renovate-core/src/branch.rs#L2102) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2111`](../../../../../../../crates/renovate-core/src/branch.rs#L2111) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2120`](../../../../../../../crates/renovate-core/src/branch.rs#L2120) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2129`](../../../../../../../crates/renovate-core/src/branch.rs#L2129) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2138`](../../../../../../../crates/renovate-core/src/branch.rs#L2138) |

