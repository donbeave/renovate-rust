# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2212`](../../../../../../../crates/renovate-core/src/branch.rs#L2212) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2218`](../../../../../../../crates/renovate-core/src/branch.rs#L2218) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2227`](../../../../../../../crates/renovate-core/src/branch.rs#L2227) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2236`](../../../../../../../crates/renovate-core/src/branch.rs#L2236) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2245`](../../../../../../../crates/renovate-core/src/branch.rs#L2245) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2254`](../../../../../../../crates/renovate-core/src/branch.rs#L2254) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2263`](../../../../../../../crates/renovate-core/src/branch.rs#L2263) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2272`](../../../../../../../crates/renovate-core/src/branch.rs#L2272) |

