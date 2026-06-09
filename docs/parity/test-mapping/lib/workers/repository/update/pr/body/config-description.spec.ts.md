# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12685`](../../../../../../../../../crates/renovate-core/src/util.rs#L12685) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12693`](../../../../../../../../../crates/renovate-core/src/util.rs#L12693) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12711`](../../../../../../../../../crates/renovate-core/src/util.rs#L12711) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12729`](../../../../../../../../../crates/renovate-core/src/util.rs#L12729) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12748`](../../../../../../../../../crates/renovate-core/src/util.rs#L12748) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12768`](../../../../../../../../../crates/renovate-core/src/util.rs#L12768) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12790`](../../../../../../../../../crates/renovate-core/src/util.rs#L12790) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12798`](../../../../../../../../../crates/renovate-core/src/util.rs#L12798) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12816`](../../../../../../../../../crates/renovate-core/src/util.rs#L12816) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12824`](../../../../../../../../../crates/renovate-core/src/util.rs#L12824) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12832`](../../../../../../../../../crates/renovate-core/src/util.rs#L12832) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12841`](../../../../../../../../../crates/renovate-core/src/util.rs#L12841) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12849`](../../../../../../../../../crates/renovate-core/src/util.rs#L12849) |

