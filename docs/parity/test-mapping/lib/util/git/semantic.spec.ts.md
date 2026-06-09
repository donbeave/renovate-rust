# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2365`](../../../../../../crates/renovate-core/src/branch.rs#L2365) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2373`](../../../../../../crates/renovate-core/src/branch.rs#L2373) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2379`](../../../../../../crates/renovate-core/src/branch.rs#L2379) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2389`](../../../../../../crates/renovate-core/src/branch.rs#L2389) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2395`](../../../../../../crates/renovate-core/src/branch.rs#L2395) |

