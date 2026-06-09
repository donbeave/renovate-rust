# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2216`](../../../../../../../crates/renovate-core/src/branch.rs#L2216) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2222`](../../../../../../../crates/renovate-core/src/branch.rs#L2222) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2231`](../../../../../../../crates/renovate-core/src/branch.rs#L2231) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2240`](../../../../../../../crates/renovate-core/src/branch.rs#L2240) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2249`](../../../../../../../crates/renovate-core/src/branch.rs#L2249) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2258`](../../../../../../../crates/renovate-core/src/branch.rs#L2258) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2267`](../../../../../../../crates/renovate-core/src/branch.rs#L2267) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2276`](../../../../../../../crates/renovate-core/src/branch.rs#L2276) |

