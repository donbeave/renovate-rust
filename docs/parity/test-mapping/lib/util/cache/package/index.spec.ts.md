# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | [`crates/renovate-core/src/cache/package.rs:823`](../../../../../../../crates/renovate-core/src/cache/package.rs#L823) |
| 33 | delegates init to backend | ported | [`crates/renovate-core/src/cache/package.rs:831`](../../../../../../../crates/renovate-core/src/cache/package.rs#L831) |
| 41 | delegates get to backend | ported | [`crates/renovate-core/src/cache/package.rs:556`](../../../../../../../crates/renovate-core/src/cache/package.rs#L556) |
| 51 | delegates set to backend | ported | [`crates/renovate-core/src/cache/package.rs:565`](../../../../../../../crates/renovate-core/src/cache/package.rs#L565) |
| 64 | delegates setwithrawttl to backend | ported | [`crates/renovate-core/src/cache/package.rs:582`](../../../../../../../crates/renovate-core/src/cache/package.rs#L582) |
| 77 | deduplicates get via memcache | ported | [`crates/renovate-core/src/cache/package.rs:878`](../../../../../../../crates/renovate-core/src/cache/package.rs#L878) |
| 89 | setwithrawttl updates memcache | ported | [`crates/renovate-core/src/cache/package.rs:894`](../../../../../../../crates/renovate-core/src/cache/package.rs#L894) |
| 99 | delegates cleanup to backend.destroy | ported | [`crates/renovate-core/src/cache/package.rs:853`](../../../../../../../crates/renovate-core/src/cache/package.rs#L853) |
| 105 | delegates getcachetype to backend | ported | [`crates/renovate-core/src/cache/package.rs:842`](../../../../../../../crates/renovate-core/src/cache/package.rs#L842) |

