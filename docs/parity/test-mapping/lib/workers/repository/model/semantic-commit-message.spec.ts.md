# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2115`](../../../../../../../crates/renovate-core/src/branch.rs#L2115) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2121`](../../../../../../../crates/renovate-core/src/branch.rs#L2121) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2130`](../../../../../../../crates/renovate-core/src/branch.rs#L2130) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2139`](../../../../../../../crates/renovate-core/src/branch.rs#L2139) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2148`](../../../../../../../crates/renovate-core/src/branch.rs#L2148) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2157`](../../../../../../../crates/renovate-core/src/branch.rs#L2157) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2166`](../../../../../../../crates/renovate-core/src/branch.rs#L2166) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2175`](../../../../../../../crates/renovate-core/src/branch.rs#L2175) |

