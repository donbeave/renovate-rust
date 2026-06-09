# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2226`](../../../../../../../crates/renovate-core/src/branch.rs#L2226) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2232`](../../../../../../../crates/renovate-core/src/branch.rs#L2232) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2241`](../../../../../../../crates/renovate-core/src/branch.rs#L2241) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2250`](../../../../../../../crates/renovate-core/src/branch.rs#L2250) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2259`](../../../../../../../crates/renovate-core/src/branch.rs#L2259) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2268`](../../../../../../../crates/renovate-core/src/branch.rs#L2268) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2277`](../../../../../../../crates/renovate-core/src/branch.rs#L2277) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2286`](../../../../../../../crates/renovate-core/src/branch.rs#L2286) |

