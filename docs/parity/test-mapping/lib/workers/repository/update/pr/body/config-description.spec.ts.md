# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12682`](../../../../../../../../../crates/renovate-core/src/util.rs#L12682) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12690`](../../../../../../../../../crates/renovate-core/src/util.rs#L12690) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12708`](../../../../../../../../../crates/renovate-core/src/util.rs#L12708) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12726`](../../../../../../../../../crates/renovate-core/src/util.rs#L12726) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12745`](../../../../../../../../../crates/renovate-core/src/util.rs#L12745) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12765`](../../../../../../../../../crates/renovate-core/src/util.rs#L12765) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12787`](../../../../../../../../../crates/renovate-core/src/util.rs#L12787) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12795`](../../../../../../../../../crates/renovate-core/src/util.rs#L12795) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12813`](../../../../../../../../../crates/renovate-core/src/util.rs#L12813) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12821`](../../../../../../../../../crates/renovate-core/src/util.rs#L12821) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12829`](../../../../../../../../../crates/renovate-core/src/util.rs#L12829) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12838`](../../../../../../../../../crates/renovate-core/src/util.rs#L12838) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12846`](../../../../../../../../../crates/renovate-core/src/util.rs#L12846) |

