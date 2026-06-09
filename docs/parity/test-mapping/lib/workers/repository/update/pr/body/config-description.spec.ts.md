# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12674`](../../../../../../../../../crates/renovate-core/src/util.rs#L12674) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12682`](../../../../../../../../../crates/renovate-core/src/util.rs#L12682) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12700`](../../../../../../../../../crates/renovate-core/src/util.rs#L12700) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12718`](../../../../../../../../../crates/renovate-core/src/util.rs#L12718) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12737`](../../../../../../../../../crates/renovate-core/src/util.rs#L12737) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12757`](../../../../../../../../../crates/renovate-core/src/util.rs#L12757) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12779`](../../../../../../../../../crates/renovate-core/src/util.rs#L12779) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12787`](../../../../../../../../../crates/renovate-core/src/util.rs#L12787) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12805`](../../../../../../../../../crates/renovate-core/src/util.rs#L12805) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12813`](../../../../../../../../../crates/renovate-core/src/util.rs#L12813) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12821`](../../../../../../../../../crates/renovate-core/src/util.rs#L12821) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12830`](../../../../../../../../../crates/renovate-core/src/util.rs#L12830) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12838`](../../../../../../../../../crates/renovate-core/src/util.rs#L12838) |

