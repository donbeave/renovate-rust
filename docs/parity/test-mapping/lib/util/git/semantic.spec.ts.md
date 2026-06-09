# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2331`](../../../../../../crates/renovate-core/src/branch.rs#L2331) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2339`](../../../../../../crates/renovate-core/src/branch.rs#L2339) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2345`](../../../../../../crates/renovate-core/src/branch.rs#L2345) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2355`](../../../../../../crates/renovate-core/src/branch.rs#L2355) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2361`](../../../../../../crates/renovate-core/src/branch.rs#L2361) |

