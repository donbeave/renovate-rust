# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2254`](../../../../../../crates/renovate-core/src/branch.rs#L2254) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2262`](../../../../../../crates/renovate-core/src/branch.rs#L2262) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2268`](../../../../../../crates/renovate-core/src/branch.rs#L2268) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2278`](../../../../../../crates/renovate-core/src/branch.rs#L2278) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2284`](../../../../../../crates/renovate-core/src/branch.rs#L2284) |

