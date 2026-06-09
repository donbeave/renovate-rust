# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9832`](../../../../../../../../crates/renovate-core/src/util.rs#L9832) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9838`](../../../../../../../../crates/renovate-core/src/util.rs#L9838) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9845`](../../../../../../../../crates/renovate-core/src/util.rs#L9845) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9852`](../../../../../../../../crates/renovate-core/src/util.rs#L9852) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9859`](../../../../../../../../crates/renovate-core/src/util.rs#L9859) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9866`](../../../../../../../../crates/renovate-core/src/util.rs#L9866) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9875`](../../../../../../../../crates/renovate-core/src/util.rs#L9875) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9883`](../../../../../../../../crates/renovate-core/src/util.rs#L9883) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9891`](../../../../../../../../crates/renovate-core/src/util.rs#L9891) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9897`](../../../../../../../../crates/renovate-core/src/util.rs#L9897) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9891`](../../../../../../../../crates/renovate-core/src/util.rs#L9891) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9929`](../../../../../../../../crates/renovate-core/src/util.rs#L9929) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9939`](../../../../../../../../crates/renovate-core/src/util.rs#L9939) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9950`](../../../../../../../../crates/renovate-core/src/util.rs#L9950) |

