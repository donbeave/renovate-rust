# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2099`](../../../../../../../crates/renovate-core/src/branch.rs#L2099) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2105`](../../../../../../../crates/renovate-core/src/branch.rs#L2105) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2114`](../../../../../../../crates/renovate-core/src/branch.rs#L2114) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2123`](../../../../../../../crates/renovate-core/src/branch.rs#L2123) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2132`](../../../../../../../crates/renovate-core/src/branch.rs#L2132) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2141`](../../../../../../../crates/renovate-core/src/branch.rs#L2141) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2150`](../../../../../../../crates/renovate-core/src/branch.rs#L2150) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2159`](../../../../../../../crates/renovate-core/src/branch.rs#L2159) |

