# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2252`](../../../../../../crates/renovate-core/src/branch.rs#L2252) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2260`](../../../../../../crates/renovate-core/src/branch.rs#L2260) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2266`](../../../../../../crates/renovate-core/src/branch.rs#L2266) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2276`](../../../../../../crates/renovate-core/src/branch.rs#L2276) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2282`](../../../../../../crates/renovate-core/src/branch.rs#L2282) |

