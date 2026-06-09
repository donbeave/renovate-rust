# `lib/workers/global/limits.spec.ts`

[← `worker/global`](../../../_by-module/worker/global.md) · [all modules](../../../README.md)

**19/19 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | increments limited value | ported | [`crates/renovate-core/src/limits.rs:237`](../../../../../../crates/renovate-core/src/limits.rs#L237) |
| 38 | defaults to unlimited | ported | [`crates/renovate-core/src/limits.rs:252`](../../../../../../crates/renovate-core/src/limits.rs#L252) |
| 42 | increments undefined | ported | [`crates/renovate-core/src/limits.rs:260`](../../../../../../crates/renovate-core/src/limits.rs#L260) |
| 47 | resets counter | ported | [`crates/renovate-core/src/limits.rs:269`](../../../../../../crates/renovate-core/src/limits.rs#L269) |
| 55 | resets limit | ported | [`crates/renovate-core/src/limits.rs:281`](../../../../../../crates/renovate-core/src/limits.rs#L281) |
| 63 | sets non-positive limit as reached | ported | [`crates/renovate-core/src/limits.rs:293`](../../../../../../crates/renovate-core/src/limits.rs#L293) |
| 71 | handles single upgrade | ported | [`crates/renovate-core/src/limits.rs:317`](../../../../../../crates/renovate-core/src/limits.rs#L317) |
| 85 | inherits prconcurrentlimit if branchconcurrentlimit is null | ported | [`crates/renovate-core/src/limits.rs:326`](../../../../../../crates/renovate-core/src/limits.rs#L326) |
| 99 | returns 0 if at least one upgrade has no limit in the branch | ported | [`crates/renovate-core/src/limits.rs:335`](../../../../../../crates/renovate-core/src/limits.rs#L335) |
| 123 | computes the lowest limit if multiple limits are present | ported | [`crates/renovate-core/src/limits.rs:363`](../../../../../../crates/renovate-core/src/limits.rs#L363) |
| 165 | de-duplicates upgrades by depname from debug log | ported | [`crates/renovate-core/src/limits.rs:412`](../../../../../../crates/renovate-core/src/limits.rs#L412) |
| 195 | handles single limit | ported | [`crates/renovate-core/src/limits.rs:436`](../../../../../../crates/renovate-core/src/limits.rs#L436) |
| 208 | returns false if there are multiple limits with value | ported | [`crates/renovate-core/src/limits.rs:451`](../../../../../../crates/renovate-core/src/limits.rs#L451) |
| 226 | handles multiple limits | ported | [`crates/renovate-core/src/limits.rs:472`](../../../../../../crates/renovate-core/src/limits.rs#L472) |
| 251 | returns false based on concurrent limits | ported | [`crates/renovate-core/src/limits.rs:530`](../../../../../../crates/renovate-core/src/limits.rs#L530) |
| 280 | returns true when pr hourly limit is reached | ported | [`crates/renovate-core/src/limits.rs:543`](../../../../../../crates/renovate-core/src/limits.rs#L543) |
| 309 | returns true when concurrent limit is reached | ported | [`crates/renovate-core/src/limits.rs:576`](../../../../../../crates/renovate-core/src/limits.rs#L576) |
| 338 | commit hourly limit only affects hourlycommits check | ported | [`crates/renovate-core/src/limits.rs:609`](../../../../../../crates/renovate-core/src/limits.rs#L609) |
| 362 | commit hourly limit does not block branch or pr checks | ported | [`crates/renovate-core/src/limits.rs:633`](../../../../../../crates/renovate-core/src/limits.rs#L633) |

