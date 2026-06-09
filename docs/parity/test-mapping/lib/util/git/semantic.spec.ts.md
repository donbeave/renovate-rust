# `lib/util/git/semantic.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | detects false if unknown | ported | [`crates/renovate-core/src/branch.rs:2351`](../../../../../../crates/renovate-core/src/branch.rs#L2351) |
| 31 | detects true if known | ported | [`crates/renovate-core/src/branch.rs:2359`](../../../../../../crates/renovate-core/src/branch.rs#L2359) |
| 38 | detects false on malformed commits | ported | [`crates/renovate-core/src/branch.rs:2365`](../../../../../../crates/renovate-core/src/branch.rs#L2365) |
| 49 | detects true on breaking changes | ported | [`crates/renovate-core/src/branch.rs:2375`](../../../../../../crates/renovate-core/src/branch.rs#L2375) |
| 56 | detects true on breaking changes with scope | ported | [`crates/renovate-core/src/branch.rs:2381`](../../../../../../crates/renovate-core/src/branch.rs#L2381) |

