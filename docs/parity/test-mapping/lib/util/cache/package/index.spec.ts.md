# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | [`crates/renovate-core/src/cache/package.rs:829`](../../../../../../../crates/renovate-core/src/cache/package.rs#L829) |
| 33 | delegates init to backend | ported | [`crates/renovate-core/src/cache/package.rs:837`](../../../../../../../crates/renovate-core/src/cache/package.rs#L837) |
| 41 | delegates get to backend | ported | [`crates/renovate-core/src/cache/package.rs:555`](../../../../../../../crates/renovate-core/src/cache/package.rs#L555) |
| 51 | delegates set to backend | ported | [`crates/renovate-core/src/cache/package.rs:564`](../../../../../../../crates/renovate-core/src/cache/package.rs#L564) |
| 64 | delegates setwithrawttl to backend | ported | [`crates/renovate-core/src/cache/package.rs:581`](../../../../../../../crates/renovate-core/src/cache/package.rs#L581) |
| 77 | deduplicates get via memcache | ported | [`crates/renovate-core/src/cache/package.rs:884`](../../../../../../../crates/renovate-core/src/cache/package.rs#L884) |
| 89 | setwithrawttl updates memcache | ported | [`crates/renovate-core/src/cache/package.rs:900`](../../../../../../../crates/renovate-core/src/cache/package.rs#L900) |
| 99 | delegates cleanup to backend.destroy | ported | [`crates/renovate-core/src/cache/package.rs:859`](../../../../../../../crates/renovate-core/src/cache/package.rs#L859) |
| 105 | delegates getcachetype to backend | ported | [`crates/renovate-core/src/cache/package.rs:848`](../../../../../../../crates/renovate-core/src/cache/package.rs#L848) |

