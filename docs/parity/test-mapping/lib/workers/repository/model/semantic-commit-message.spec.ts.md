# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2075`](../../../../../../../crates/renovate-core/src/branch.rs#L2075) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2081`](../../../../../../../crates/renovate-core/src/branch.rs#L2081) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2090`](../../../../../../../crates/renovate-core/src/branch.rs#L2090) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2099`](../../../../../../../crates/renovate-core/src/branch.rs#L2099) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2108`](../../../../../../../crates/renovate-core/src/branch.rs#L2108) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2117`](../../../../../../../crates/renovate-core/src/branch.rs#L2117) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2126`](../../../../../../../crates/renovate-core/src/branch.rs#L2126) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2135`](../../../../../../../crates/renovate-core/src/branch.rs#L2135) |

