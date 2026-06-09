# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9817`](../../../../../../../../crates/renovate-core/src/util.rs#L9817) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9823`](../../../../../../../../crates/renovate-core/src/util.rs#L9823) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9830`](../../../../../../../../crates/renovate-core/src/util.rs#L9830) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9837`](../../../../../../../../crates/renovate-core/src/util.rs#L9837) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9844`](../../../../../../../../crates/renovate-core/src/util.rs#L9844) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9851`](../../../../../../../../crates/renovate-core/src/util.rs#L9851) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9860`](../../../../../../../../crates/renovate-core/src/util.rs#L9860) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9868`](../../../../../../../../crates/renovate-core/src/util.rs#L9868) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9876`](../../../../../../../../crates/renovate-core/src/util.rs#L9876) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9882`](../../../../../../../../crates/renovate-core/src/util.rs#L9882) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9876`](../../../../../../../../crates/renovate-core/src/util.rs#L9876) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9914`](../../../../../../../../crates/renovate-core/src/util.rs#L9914) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9924`](../../../../../../../../crates/renovate-core/src/util.rs#L9924) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9935`](../../../../../../../../crates/renovate-core/src/util.rs#L9935) |

