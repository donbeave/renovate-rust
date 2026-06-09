# `lib/workers/repository/update/pr/body/config-description.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**13/18 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | renders stopupdating=true | ported | [`crates/renovate-core/src/util.rs:12771`](../../../../../../../../../crates/renovate-core/src/util.rs#L12771) |
| 25 | renders rebasewhen="never" | ported | [`crates/renovate-core/src/util.rs:12779`](../../../../../../../../../crates/renovate-core/src/util.rs#L12779) |
| 36 | renders rebasewhen="behind-base-branch" | ported | [`crates/renovate-core/src/util.rs:12797`](../../../../../../../../../crates/renovate-core/src/util.rs#L12797) |
| 45 | renders timezone | ported | [`crates/renovate-core/src/util.rs:12815`](../../../../../../../../../crates/renovate-core/src/util.rs#L12815) |
| 54 | renders utc as the default timezone | ported | [`crates/renovate-core/src/util.rs:12834`](../../../../../../../../../crates/renovate-core/src/util.rs#L12834) |
| 62 | summarizes cron schedules | pending | — |
| 73 | displays later schedules | ported | [`crates/renovate-core/src/util.rs:12854`](../../../../../../../../../crates/renovate-core/src/util.rs#L12854) |
| 81 | renders undefined schedule | ported | [`crates/renovate-core/src/util.rs:12876`](../../../../../../../../../crates/renovate-core/src/util.rs#L12876) |
| 94 | renders empty schedule | pending | — |
| 112 | does not take into account `force` | pending | — |
| 131 | summarizes cron schedules (for automergeschedule) | pending | — |
| 142 | summarizes both branch creation and automerge schedules | pending | — |
| 161 | renders recreateclosed=true | ported | [`crates/renovate-core/src/util.rs:12884`](../../../../../../../../../crates/renovate-core/src/util.rs#L12884) |
| 169 | does not render recreateclosed=false | ported | [`crates/renovate-core/src/util.rs:12902`](../../../../../../../../../crates/renovate-core/src/util.rs#L12902) |
| 177 | does not render recreateclosed=undefined | ported | [`crates/renovate-core/src/util.rs:12910`](../../../../../../../../../crates/renovate-core/src/util.rs#L12910) |
| 182 | renders singular | ported | [`crates/renovate-core/src/util.rs:12918`](../../../../../../../../../crates/renovate-core/src/util.rs#L12918) |
| 190 | renders automerge | ported | [`crates/renovate-core/src/util.rs:12927`](../../../../../../../../../crates/renovate-core/src/util.rs#L12927) |
| 195 | renders blocked automerge | ported | [`crates/renovate-core/src/util.rs:12935`](../../../../../../../../../crates/renovate-core/src/util.rs#L12935) |

