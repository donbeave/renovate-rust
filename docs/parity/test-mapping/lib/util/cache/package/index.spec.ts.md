# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | [`crates/renovate-core/src/cache/package.rs:837`](../../../../../../../crates/renovate-core/src/cache/package.rs#L837) |
| 33 | delegates init to backend | ported | [`crates/renovate-core/src/cache/package.rs:845`](../../../../../../../crates/renovate-core/src/cache/package.rs#L845) |
| 41 | delegates get to backend | ported | [`crates/renovate-core/src/cache/package.rs:555`](../../../../../../../crates/renovate-core/src/cache/package.rs#L555) |
| 51 | delegates set to backend | ported | [`crates/renovate-core/src/cache/package.rs:565`](../../../../../../../crates/renovate-core/src/cache/package.rs#L565) |
| 64 | delegates setwithrawttl to backend | ported | [`crates/renovate-core/src/cache/package.rs:594`](../../../../../../../crates/renovate-core/src/cache/package.rs#L594) |
| 77 | deduplicates get via memcache | ported | [`crates/renovate-core/src/cache/package.rs:892`](../../../../../../../crates/renovate-core/src/cache/package.rs#L892) |
| 89 | setwithrawttl updates memcache | ported | [`crates/renovate-core/src/cache/package.rs:908`](../../../../../../../crates/renovate-core/src/cache/package.rs#L908) |
| 99 | delegates cleanup to backend.destroy | ported | [`crates/renovate-core/src/cache/package.rs:867`](../../../../../../../crates/renovate-core/src/cache/package.rs#L867) |
| 105 | delegates getcachetype to backend | ported | [`crates/renovate-core/src/cache/package.rs:856`](../../../../../../../crates/renovate-core/src/cache/package.rs#L856) |

