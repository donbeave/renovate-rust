# `lib/workers/global/limits.spec.ts`

[← `worker/global`](../../../_by-module/worker/global.md) · [all modules](../../../README.md)

**19/19 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 23 | increments limited value | ported | [`crates/renovate-core/src/limits.rs:233`](../../../../../../crates/renovate-core/src/limits.rs#L233) |
| 38 | defaults to unlimited | ported | [`crates/renovate-core/src/limits.rs:248`](../../../../../../crates/renovate-core/src/limits.rs#L248) |
| 42 | increments undefined | ported | [`crates/renovate-core/src/limits.rs:256`](../../../../../../crates/renovate-core/src/limits.rs#L256) |
| 47 | resets counter | ported | [`crates/renovate-core/src/limits.rs:265`](../../../../../../crates/renovate-core/src/limits.rs#L265) |
| 55 | resets limit | ported | [`crates/renovate-core/src/limits.rs:277`](../../../../../../crates/renovate-core/src/limits.rs#L277) |
| 63 | sets non-positive limit as reached | ported | [`crates/renovate-core/src/limits.rs:289`](../../../../../../crates/renovate-core/src/limits.rs#L289) |
| 71 | handles single upgrade | ported | [`crates/renovate-core/src/limits.rs:313`](../../../../../../crates/renovate-core/src/limits.rs#L313) |
| 85 | inherits prconcurrentlimit if branchconcurrentlimit is null | ported | [`crates/renovate-core/src/limits.rs:322`](../../../../../../crates/renovate-core/src/limits.rs#L322) |
| 99 | returns 0 if at least one upgrade has no limit in the branch | ported | [`crates/renovate-core/src/limits.rs:331`](../../../../../../crates/renovate-core/src/limits.rs#L331) |
| 123 | computes the lowest limit if multiple limits are present | ported | [`crates/renovate-core/src/limits.rs:359`](../../../../../../crates/renovate-core/src/limits.rs#L359) |
| 165 | de-duplicates upgrades by depname from debug log | ported | [`crates/renovate-core/src/limits.rs:408`](../../../../../../crates/renovate-core/src/limits.rs#L408) |
| 195 | handles single limit | ported | [`crates/renovate-core/src/limits.rs:432`](../../../../../../crates/renovate-core/src/limits.rs#L432) |
| 208 | returns false if there are multiple limits with value | ported | [`crates/renovate-core/src/limits.rs:447`](../../../../../../crates/renovate-core/src/limits.rs#L447) |
| 226 | handles multiple limits | ported | [`crates/renovate-core/src/limits.rs:468`](../../../../../../crates/renovate-core/src/limits.rs#L468) |
| 251 | returns false based on concurrent limits | ported | [`crates/renovate-core/src/limits.rs:526`](../../../../../../crates/renovate-core/src/limits.rs#L526) |
| 280 | returns true when pr hourly limit is reached | ported | [`crates/renovate-core/src/limits.rs:539`](../../../../../../crates/renovate-core/src/limits.rs#L539) |
| 309 | returns true when concurrent limit is reached | ported | [`crates/renovate-core/src/limits.rs:572`](../../../../../../crates/renovate-core/src/limits.rs#L572) |
| 338 | commit hourly limit only affects hourlycommits check | ported | [`crates/renovate-core/src/limits.rs:605`](../../../../../../crates/renovate-core/src/limits.rs#L605) |
| 362 | commit hourly limit does not block branch or pr checks | ported | [`crates/renovate-core/src/limits.rs:629`](../../../../../../crates/renovate-core/src/limits.rs#L629) |

