# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2192`](../../../../../../../crates/renovate-core/src/branch.rs#L2192) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2198`](../../../../../../../crates/renovate-core/src/branch.rs#L2198) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2207`](../../../../../../../crates/renovate-core/src/branch.rs#L2207) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2216`](../../../../../../../crates/renovate-core/src/branch.rs#L2216) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2225`](../../../../../../../crates/renovate-core/src/branch.rs#L2225) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2234`](../../../../../../../crates/renovate-core/src/branch.rs#L2234) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2243`](../../../../../../../crates/renovate-core/src/branch.rs#L2243) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2252`](../../../../../../../crates/renovate-core/src/branch.rs#L2252) |

