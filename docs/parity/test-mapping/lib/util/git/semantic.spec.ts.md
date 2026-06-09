# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2352`](../../../../../../crates/renovate-core/src/branch.rs#L2352) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2360`](../../../../../../crates/renovate-core/src/branch.rs#L2360) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2366`](../../../../../../crates/renovate-core/src/branch.rs#L2366) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2376`](../../../../../../crates/renovate-core/src/branch.rs#L2376) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2382`](../../../../../../crates/renovate-core/src/branch.rs#L2382) |

