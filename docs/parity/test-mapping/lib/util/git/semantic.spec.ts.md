# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2332`](../../../../../../crates/renovate-core/src/branch.rs#L2332) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2340`](../../../../../../crates/renovate-core/src/branch.rs#L2340) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2346`](../../../../../../crates/renovate-core/src/branch.rs#L2346) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2356`](../../../../../../crates/renovate-core/src/branch.rs#L2356) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2362`](../../../../../../crates/renovate-core/src/branch.rs#L2362) |

