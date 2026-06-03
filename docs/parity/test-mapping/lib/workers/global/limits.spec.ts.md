# `lib/workers/global/limits.spec.ts`

[← `worker/global`](../../../_by-module/worker/global.md) · [all modules](../../../README.md)

**19/19 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 23 | increments limited value | ported | `crates/renovate-core/src/limits.rs:233` |
| 38 | defaults to unlimited | ported | `crates/renovate-core/src/limits.rs:248` |
| 42 | increments undefined | ported | `crates/renovate-core/src/limits.rs:256` |
| 47 | resets counter | ported | `crates/renovate-core/src/limits.rs:265` |
| 55 | resets limit | ported | `crates/renovate-core/src/limits.rs:277` |
| 63 | sets non-positive limit as reached | ported | `crates/renovate-core/src/limits.rs:289` |
| 71 | handles single upgrade | ported | `crates/renovate-core/src/limits.rs:313` |
| 85 | inherits prconcurrentlimit if branchconcurrentlimit is null | ported | `crates/renovate-core/src/limits.rs:322` |
| 99 | returns 0 if at least one upgrade has no limit in the branch | ported | `crates/renovate-core/src/limits.rs:331` |
| 123 | computes the lowest limit if multiple limits are present | ported | `crates/renovate-core/src/limits.rs:359` |
| 165 | de-duplicates upgrades by depname from debug log | ported | `crates/renovate-core/src/limits.rs:408` |
| 195 | handles single limit | ported | `crates/renovate-core/src/limits.rs:432` |
| 208 | returns false if there are multiple limits with value | ported | `crates/renovate-core/src/limits.rs:447` |
| 226 | handles multiple limits | ported | `crates/renovate-core/src/limits.rs:468` |
| 251 | returns false based on concurrent limits | ported | `crates/renovate-core/src/limits.rs:526` |
| 280 | returns true when pr hourly limit is reached | ported | `crates/renovate-core/src/limits.rs:539` |
| 309 | returns true when concurrent limit is reached | ported | `crates/renovate-core/src/limits.rs:572` |
| 338 | commit hourly limit only affects hourlycommits check | ported | `crates/renovate-core/src/limits.rs:605` |
| 362 | commit hourly limit does not block branch or pr checks | ported | `crates/renovate-core/src/limits.rs:629` |

