# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2214`](../../../../../../crates/renovate-core/src/branch.rs#L2214) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2222`](../../../../../../crates/renovate-core/src/branch.rs#L2222) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2228`](../../../../../../crates/renovate-core/src/branch.rs#L2228) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2238`](../../../../../../crates/renovate-core/src/branch.rs#L2238) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2244`](../../../../../../crates/renovate-core/src/branch.rs#L2244) |

