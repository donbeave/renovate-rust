# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2113`](../../../../../../../crates/renovate-core/src/branch.rs#L2113) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2119`](../../../../../../../crates/renovate-core/src/branch.rs#L2119) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2128`](../../../../../../../crates/renovate-core/src/branch.rs#L2128) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2137`](../../../../../../../crates/renovate-core/src/branch.rs#L2137) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2146`](../../../../../../../crates/renovate-core/src/branch.rs#L2146) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2155`](../../../../../../../crates/renovate-core/src/branch.rs#L2155) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2164`](../../../../../../../crates/renovate-core/src/branch.rs#L2164) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2173`](../../../../../../../crates/renovate-core/src/branch.rs#L2173) |

