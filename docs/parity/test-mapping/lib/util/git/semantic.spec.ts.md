# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2235`](../../../../../../crates/renovate-core/src/branch.rs#L2235) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2243`](../../../../../../crates/renovate-core/src/branch.rs#L2243) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2249`](../../../../../../crates/renovate-core/src/branch.rs#L2249) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2259`](../../../../../../crates/renovate-core/src/branch.rs#L2259) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2265`](../../../../../../crates/renovate-core/src/branch.rs#L2265) |

