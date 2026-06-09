# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2213`](../../../../../../../crates/renovate-core/src/branch.rs#L2213) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2219`](../../../../../../../crates/renovate-core/src/branch.rs#L2219) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2228`](../../../../../../../crates/renovate-core/src/branch.rs#L2228) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2237`](../../../../../../../crates/renovate-core/src/branch.rs#L2237) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2246`](../../../../../../../crates/renovate-core/src/branch.rs#L2246) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2255`](../../../../../../../crates/renovate-core/src/branch.rs#L2255) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2264`](../../../../../../../crates/renovate-core/src/branch.rs#L2264) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2273`](../../../../../../../crates/renovate-core/src/branch.rs#L2273) |

