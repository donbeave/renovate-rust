# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12689`](../../../../../../../../../crates/renovate-core/src/util.rs#L12689) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12697`](../../../../../../../../../crates/renovate-core/src/util.rs#L12697) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12715`](../../../../../../../../../crates/renovate-core/src/util.rs#L12715) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12733`](../../../../../../../../../crates/renovate-core/src/util.rs#L12733) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12752`](../../../../../../../../../crates/renovate-core/src/util.rs#L12752) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12772`](../../../../../../../../../crates/renovate-core/src/util.rs#L12772) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12794`](../../../../../../../../../crates/renovate-core/src/util.rs#L12794) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12802`](../../../../../../../../../crates/renovate-core/src/util.rs#L12802) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12820`](../../../../../../../../../crates/renovate-core/src/util.rs#L12820) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12828`](../../../../../../../../../crates/renovate-core/src/util.rs#L12828) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12836`](../../../../../../../../../crates/renovate-core/src/util.rs#L12836) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12845`](../../../../../../../../../crates/renovate-core/src/util.rs#L12845) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12853`](../../../../../../../../../crates/renovate-core/src/util.rs#L12853) |

