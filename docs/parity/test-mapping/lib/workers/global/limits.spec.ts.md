# `lib/workers/global/limits.spec.ts`

[← `worker/global`](../../../_by-module/worker/global.md) · [all modules](../../../README.md)

**19/19 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | increments limited value | ported | [`crates/renovate-core/src/limits.rs:234`](../../../../../../crates/renovate-core/src/limits.rs#L234) |
| 38 | defaults to unlimited | ported | [`crates/renovate-core/src/limits.rs:249`](../../../../../../crates/renovate-core/src/limits.rs#L249) |
| 42 | increments undefined | ported | [`crates/renovate-core/src/limits.rs:257`](../../../../../../crates/renovate-core/src/limits.rs#L257) |
| 47 | resets counter | ported | [`crates/renovate-core/src/limits.rs:266`](../../../../../../crates/renovate-core/src/limits.rs#L266) |
| 55 | resets limit | ported | [`crates/renovate-core/src/limits.rs:278`](../../../../../../crates/renovate-core/src/limits.rs#L278) |
| 63 | sets non-positive limit as reached | ported | [`crates/renovate-core/src/limits.rs:290`](../../../../../../crates/renovate-core/src/limits.rs#L290) |
| 71 | handles single upgrade | ported | [`crates/renovate-core/src/limits.rs:314`](../../../../../../crates/renovate-core/src/limits.rs#L314) |
| 85 | inherits prconcurrentlimit if branchconcurrentlimit is null | ported | [`crates/renovate-core/src/limits.rs:323`](../../../../../../crates/renovate-core/src/limits.rs#L323) |
| 99 | returns 0 if at least one upgrade has no limit in the branch | ported | [`crates/renovate-core/src/limits.rs:332`](../../../../../../crates/renovate-core/src/limits.rs#L332) |
| 123 | computes the lowest limit if multiple limits are present | ported | [`crates/renovate-core/src/limits.rs:360`](../../../../../../crates/renovate-core/src/limits.rs#L360) |
| 165 | de-duplicates upgrades by depname from debug log | ported | [`crates/renovate-core/src/limits.rs:409`](../../../../../../crates/renovate-core/src/limits.rs#L409) |
| 195 | handles single limit | ported | [`crates/renovate-core/src/limits.rs:433`](../../../../../../crates/renovate-core/src/limits.rs#L433) |
| 208 | returns false if there are multiple limits with value | ported | [`crates/renovate-core/src/limits.rs:448`](../../../../../../crates/renovate-core/src/limits.rs#L448) |
| 226 | handles multiple limits | ported | [`crates/renovate-core/src/limits.rs:469`](../../../../../../crates/renovate-core/src/limits.rs#L469) |
| 251 | returns false based on concurrent limits | ported | [`crates/renovate-core/src/limits.rs:527`](../../../../../../crates/renovate-core/src/limits.rs#L527) |
| 280 | returns true when pr hourly limit is reached | ported | [`crates/renovate-core/src/limits.rs:540`](../../../../../../crates/renovate-core/src/limits.rs#L540) |
| 309 | returns true when concurrent limit is reached | ported | [`crates/renovate-core/src/limits.rs:573`](../../../../../../crates/renovate-core/src/limits.rs#L573) |
| 338 | commit hourly limit only affects hourlycommits check | ported | [`crates/renovate-core/src/limits.rs:606`](../../../../../../crates/renovate-core/src/limits.rs#L606) |
| 362 | commit hourly limit does not block branch or pr checks | ported | [`crates/renovate-core/src/limits.rs:630`](../../../../../../crates/renovate-core/src/limits.rs#L630) |

