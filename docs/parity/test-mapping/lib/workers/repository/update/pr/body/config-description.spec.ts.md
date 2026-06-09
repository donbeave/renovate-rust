# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12675`](../../../../../../../../../crates/renovate-core/src/util.rs#L12675) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12683`](../../../../../../../../../crates/renovate-core/src/util.rs#L12683) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12701`](../../../../../../../../../crates/renovate-core/src/util.rs#L12701) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12719`](../../../../../../../../../crates/renovate-core/src/util.rs#L12719) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12738`](../../../../../../../../../crates/renovate-core/src/util.rs#L12738) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12758`](../../../../../../../../../crates/renovate-core/src/util.rs#L12758) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12780`](../../../../../../../../../crates/renovate-core/src/util.rs#L12780) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12788`](../../../../../../../../../crates/renovate-core/src/util.rs#L12788) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12806`](../../../../../../../../../crates/renovate-core/src/util.rs#L12806) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12814`](../../../../../../../../../crates/renovate-core/src/util.rs#L12814) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12822`](../../../../../../../../../crates/renovate-core/src/util.rs#L12822) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12831`](../../../../../../../../../crates/renovate-core/src/util.rs#L12831) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12839`](../../../../../../../../../crates/renovate-core/src/util.rs#L12839) |

