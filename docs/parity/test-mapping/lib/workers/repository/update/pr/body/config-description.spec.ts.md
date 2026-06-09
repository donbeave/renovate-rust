# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12676`](../../../../../../../../../crates/renovate-core/src/util.rs#L12676) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12684`](../../../../../../../../../crates/renovate-core/src/util.rs#L12684) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12702`](../../../../../../../../../crates/renovate-core/src/util.rs#L12702) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12720`](../../../../../../../../../crates/renovate-core/src/util.rs#L12720) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12739`](../../../../../../../../../crates/renovate-core/src/util.rs#L12739) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12759`](../../../../../../../../../crates/renovate-core/src/util.rs#L12759) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12781`](../../../../../../../../../crates/renovate-core/src/util.rs#L12781) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12789`](../../../../../../../../../crates/renovate-core/src/util.rs#L12789) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12807`](../../../../../../../../../crates/renovate-core/src/util.rs#L12807) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12815`](../../../../../../../../../crates/renovate-core/src/util.rs#L12815) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12823`](../../../../../../../../../crates/renovate-core/src/util.rs#L12823) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12832`](../../../../../../../../../crates/renovate-core/src/util.rs#L12832) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12840`](../../../../../../../../../crates/renovate-core/src/util.rs#L12840) |

