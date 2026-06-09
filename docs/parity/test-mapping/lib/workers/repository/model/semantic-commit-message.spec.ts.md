# `lib/workers/repository/model/semantic-commit-message.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should format message without prefix | ported | [`crates/renovate-core/src/branch.rs:2096`](../../../../../../../crates/renovate-core/src/branch.rs#L2096) |
| 11 | should format sematic type | ported | [`crates/renovate-core/src/branch.rs:2102`](../../../../../../../crates/renovate-core/src/branch.rs#L2102) |
| 19 | should format sematic prefix with scope | ported | [`crates/renovate-core/src/branch.rs:2111`](../../../../../../../crates/renovate-core/src/branch.rs#L2111) |
| 28 | should transform to lowercase only first letter | ported | [`crates/renovate-core/src/branch.rs:2120`](../../../../../../../crates/renovate-core/src/branch.rs#L2120) |
| 37 | should create instance from string without scope | ported | [`crates/renovate-core/src/branch.rs:2129`](../../../../../../../crates/renovate-core/src/branch.rs#L2129) |
| 50 | should create instance from string with scope | ported | [`crates/renovate-core/src/branch.rs:2138`](../../../../../../../crates/renovate-core/src/branch.rs#L2138) |
| 65 | should create instance from string with empty description | ported | [`crates/renovate-core/src/branch.rs:2147`](../../../../../../../crates/renovate-core/src/branch.rs#L2147) |
| 78 | should return undefined for invalid string | ported | [`crates/renovate-core/src/branch.rs:2156`](../../../../../../../crates/renovate-core/src/branch.rs#L2156) |

