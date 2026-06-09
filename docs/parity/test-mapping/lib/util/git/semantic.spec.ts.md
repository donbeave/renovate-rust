# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2216`](../../../../../../crates/renovate-core/src/branch.rs#L2216) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2224`](../../../../../../crates/renovate-core/src/branch.rs#L2224) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2230`](../../../../../../crates/renovate-core/src/branch.rs#L2230) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2240`](../../../../../../crates/renovate-core/src/branch.rs#L2240) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2246`](../../../../../../crates/renovate-core/src/branch.rs#L2246) |

