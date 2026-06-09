# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9914`](../../../../../../../../crates/renovate-core/src/util.rs#L9914) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9920`](../../../../../../../../crates/renovate-core/src/util.rs#L9920) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9927`](../../../../../../../../crates/renovate-core/src/util.rs#L9927) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9934`](../../../../../../../../crates/renovate-core/src/util.rs#L9934) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9941`](../../../../../../../../crates/renovate-core/src/util.rs#L9941) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9948`](../../../../../../../../crates/renovate-core/src/util.rs#L9948) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9957`](../../../../../../../../crates/renovate-core/src/util.rs#L9957) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9965`](../../../../../../../../crates/renovate-core/src/util.rs#L9965) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9973`](../../../../../../../../crates/renovate-core/src/util.rs#L9973) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9979`](../../../../../../../../crates/renovate-core/src/util.rs#L9979) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9973`](../../../../../../../../crates/renovate-core/src/util.rs#L9973) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:10011`](../../../../../../../../crates/renovate-core/src/util.rs#L10011) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:10021`](../../../../../../../../crates/renovate-core/src/util.rs#L10021) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:10032`](../../../../../../../../crates/renovate-core/src/util.rs#L10032) |

