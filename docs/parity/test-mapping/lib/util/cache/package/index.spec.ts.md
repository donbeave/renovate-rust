# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**8/9 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | [`crates/renovate-core/src/cache/package.rs:810`](../../../../../../../crates/renovate-core/src/cache/package.rs#L810) |
| 33 | delegates init to backend | ported | [`crates/renovate-core/src/cache/package.rs:818`](../../../../../../../crates/renovate-core/src/cache/package.rs#L818) |
| 41 | delegates get to backend | ported | [`crates/renovate-core/src/cache/package.rs:555`](../../../../../../../crates/renovate-core/src/cache/package.rs#L555) |
| 51 | delegates set to backend | ported | [`crates/renovate-core/src/cache/package.rs:564`](../../../../../../../crates/renovate-core/src/cache/package.rs#L564) |
| 64 | delegates setwithrawttl to backend | pending | — |
| 77 | deduplicates get via memcache | ported | [`crates/renovate-core/src/cache/package.rs:865`](../../../../../../../crates/renovate-core/src/cache/package.rs#L865) |
| 89 | setwithrawttl updates memcache | ported | [`crates/renovate-core/src/cache/package.rs:881`](../../../../../../../crates/renovate-core/src/cache/package.rs#L881) |
| 99 | delegates cleanup to backend.destroy | ported | [`crates/renovate-core/src/cache/package.rs:840`](../../../../../../../crates/renovate-core/src/cache/package.rs#L840) |
| 105 | delegates getcachetype to backend | ported | [`crates/renovate-core/src/cache/package.rs:829`](../../../../../../../crates/renovate-core/src/cache/package.rs#L829) |

