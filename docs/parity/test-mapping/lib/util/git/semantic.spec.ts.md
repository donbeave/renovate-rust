# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2360`](../../../../../../crates/renovate-core/src/branch.rs#L2360) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2368`](../../../../../../crates/renovate-core/src/branch.rs#L2368) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2374`](../../../../../../crates/renovate-core/src/branch.rs#L2374) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2384`](../../../../../../crates/renovate-core/src/branch.rs#L2384) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2390`](../../../../../../crates/renovate-core/src/branch.rs#L2390) |

