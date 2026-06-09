# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9819`](../../../../../../../../crates/renovate-core/src/util.rs#L9819) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9825`](../../../../../../../../crates/renovate-core/src/util.rs#L9825) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9832`](../../../../../../../../crates/renovate-core/src/util.rs#L9832) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9839`](../../../../../../../../crates/renovate-core/src/util.rs#L9839) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9846`](../../../../../../../../crates/renovate-core/src/util.rs#L9846) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9853`](../../../../../../../../crates/renovate-core/src/util.rs#L9853) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9862`](../../../../../../../../crates/renovate-core/src/util.rs#L9862) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9870`](../../../../../../../../crates/renovate-core/src/util.rs#L9870) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9878`](../../../../../../../../crates/renovate-core/src/util.rs#L9878) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9884`](../../../../../../../../crates/renovate-core/src/util.rs#L9884) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9878`](../../../../../../../../crates/renovate-core/src/util.rs#L9878) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9916`](../../../../../../../../crates/renovate-core/src/util.rs#L9916) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9926`](../../../../../../../../crates/renovate-core/src/util.rs#L9926) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9937`](../../../../../../../../crates/renovate-core/src/util.rs#L9937) |

