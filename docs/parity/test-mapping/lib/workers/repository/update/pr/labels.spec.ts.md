# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9818`](../../../../../../../../crates/renovate-core/src/util.rs#L9818) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9824`](../../../../../../../../crates/renovate-core/src/util.rs#L9824) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9831`](../../../../../../../../crates/renovate-core/src/util.rs#L9831) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9838`](../../../../../../../../crates/renovate-core/src/util.rs#L9838) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9845`](../../../../../../../../crates/renovate-core/src/util.rs#L9845) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9852`](../../../../../../../../crates/renovate-core/src/util.rs#L9852) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9861`](../../../../../../../../crates/renovate-core/src/util.rs#L9861) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9869`](../../../../../../../../crates/renovate-core/src/util.rs#L9869) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9877`](../../../../../../../../crates/renovate-core/src/util.rs#L9877) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9883`](../../../../../../../../crates/renovate-core/src/util.rs#L9883) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9877`](../../../../../../../../crates/renovate-core/src/util.rs#L9877) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9915`](../../../../../../../../crates/renovate-core/src/util.rs#L9915) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9925`](../../../../../../../../crates/renovate-core/src/util.rs#L9925) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9936`](../../../../../../../../crates/renovate-core/src/util.rs#L9936) |

