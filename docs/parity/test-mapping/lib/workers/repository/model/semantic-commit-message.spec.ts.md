# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2193`](../../../../../../../crates/renovate-core/src/branch.rs#L2193) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2199`](../../../../../../../crates/renovate-core/src/branch.rs#L2199) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2208`](../../../../../../../crates/renovate-core/src/branch.rs#L2208) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2217`](../../../../../../../crates/renovate-core/src/branch.rs#L2217) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2226`](../../../../../../../crates/renovate-core/src/branch.rs#L2226) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2235`](../../../../../../../crates/renovate-core/src/branch.rs#L2235) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2244`](../../../../../../../crates/renovate-core/src/branch.rs#L2244) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2253`](../../../../../../../crates/renovate-core/src/branch.rs#L2253) |

