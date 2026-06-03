# `lib/modules/platform/github/branch.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should return true if the branch exists | ported | [`crates/renovate-core/src/platform/github.rs:4584`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4584) |
| 16 | should return false if the branch does not exist | ported | [`crates/renovate-core/src/platform/github.rs:4606`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4606) |
| 27 | should throw an error for nested branches | ported | [`crates/renovate-core/src/platform/github.rs:4626`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4626) |
| 44 | should throw an error if the request fails for any other reason | ported | [`crates/renovate-core/src/platform/github.rs:4650`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4650) |

