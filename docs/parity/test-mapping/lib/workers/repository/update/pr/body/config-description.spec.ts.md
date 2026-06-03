# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 ported** (5 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:10950`](../../../../../../../../../crates/renovate-core/src/util.rs#L10950) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:10958`](../../../../../../../../../crates/renovate-core/src/util.rs#L10958) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:10976`](../../../../../../../../../crates/renovate-core/src/util.rs#L10976) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:10994`](../../../../../../../../../crates/renovate-core/src/util.rs#L10994) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:11013`](../../../../../../../../../crates/renovate-core/src/util.rs#L11013) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:11033`](../../../../../../../../../crates/renovate-core/src/util.rs#L11033) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:11055`](../../../../../../../../../crates/renovate-core/src/util.rs#L11055) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:11063`](../../../../../../../../../crates/renovate-core/src/util.rs#L11063) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:11081`](../../../../../../../../../crates/renovate-core/src/util.rs#L11081) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:11089`](../../../../../../../../../crates/renovate-core/src/util.rs#L11089) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:11097`](../../../../../../../../../crates/renovate-core/src/util.rs#L11097) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:11106`](../../../../../../../../../crates/renovate-core/src/util.rs#L11106) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:11114`](../../../../../../../../../crates/renovate-core/src/util.rs#L11114) |

