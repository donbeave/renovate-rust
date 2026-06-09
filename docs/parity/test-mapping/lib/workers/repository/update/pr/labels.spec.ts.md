# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9816`](../../../../../../../../crates/renovate-core/src/util.rs#L9816) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9822`](../../../../../../../../crates/renovate-core/src/util.rs#L9822) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9829`](../../../../../../../../crates/renovate-core/src/util.rs#L9829) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9836`](../../../../../../../../crates/renovate-core/src/util.rs#L9836) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9843`](../../../../../../../../crates/renovate-core/src/util.rs#L9843) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9850`](../../../../../../../../crates/renovate-core/src/util.rs#L9850) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9859`](../../../../../../../../crates/renovate-core/src/util.rs#L9859) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9867`](../../../../../../../../crates/renovate-core/src/util.rs#L9867) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9875`](../../../../../../../../crates/renovate-core/src/util.rs#L9875) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9881`](../../../../../../../../crates/renovate-core/src/util.rs#L9881) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9875`](../../../../../../../../crates/renovate-core/src/util.rs#L9875) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9913`](../../../../../../../../crates/renovate-core/src/util.rs#L9913) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9923`](../../../../../../../../crates/renovate-core/src/util.rs#L9923) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9934`](../../../../../../../../crates/renovate-core/src/util.rs#L9934) |

