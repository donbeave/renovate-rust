# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | [`crates/renovate-core/src/cache/package.rs:838`](../../../../../../../crates/renovate-core/src/cache/package.rs#L838) |
| 33 | delegates init to backend | ported | [`crates/renovate-core/src/cache/package.rs:846`](../../../../../../../crates/renovate-core/src/cache/package.rs#L846) |
| 41 | delegates get to backend | ported | [`crates/renovate-core/src/cache/package.rs:556`](../../../../../../../crates/renovate-core/src/cache/package.rs#L556) |
| 51 | delegates set to backend | ported | [`crates/renovate-core/src/cache/package.rs:566`](../../../../../../../crates/renovate-core/src/cache/package.rs#L566) |
| 64 | delegates setwithrawttl to backend | ported | [`crates/renovate-core/src/cache/package.rs:595`](../../../../../../../crates/renovate-core/src/cache/package.rs#L595) |
| 77 | deduplicates get via memcache | ported | [`crates/renovate-core/src/cache/package.rs:893`](../../../../../../../crates/renovate-core/src/cache/package.rs#L893) |
| 89 | setwithrawttl updates memcache | ported | [`crates/renovate-core/src/cache/package.rs:909`](../../../../../../../crates/renovate-core/src/cache/package.rs#L909) |
| 99 | delegates cleanup to backend.destroy | ported | [`crates/renovate-core/src/cache/package.rs:868`](../../../../../../../crates/renovate-core/src/cache/package.rs#L868) |
| 105 | delegates getcachetype to backend | ported | [`crates/renovate-core/src/cache/package.rs:857`](../../../../../../../crates/renovate-core/src/cache/package.rs#L857) |

