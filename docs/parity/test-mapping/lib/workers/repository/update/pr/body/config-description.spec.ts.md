# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12673`](../../../../../../../../../crates/renovate-core/src/util.rs#L12673) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12681`](../../../../../../../../../crates/renovate-core/src/util.rs#L12681) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12699`](../../../../../../../../../crates/renovate-core/src/util.rs#L12699) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12717`](../../../../../../../../../crates/renovate-core/src/util.rs#L12717) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12736`](../../../../../../../../../crates/renovate-core/src/util.rs#L12736) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12756`](../../../../../../../../../crates/renovate-core/src/util.rs#L12756) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12778`](../../../../../../../../../crates/renovate-core/src/util.rs#L12778) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12786`](../../../../../../../../../crates/renovate-core/src/util.rs#L12786) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12804`](../../../../../../../../../crates/renovate-core/src/util.rs#L12804) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12812`](../../../../../../../../../crates/renovate-core/src/util.rs#L12812) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12820`](../../../../../../../../../crates/renovate-core/src/util.rs#L12820) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12829`](../../../../../../../../../crates/renovate-core/src/util.rs#L12829) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12837`](../../../../../../../../../crates/renovate-core/src/util.rs#L12837) |

