# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9822`](../../../../../../../../crates/renovate-core/src/util.rs#L9822) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9828`](../../../../../../../../crates/renovate-core/src/util.rs#L9828) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9835`](../../../../../../../../crates/renovate-core/src/util.rs#L9835) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9842`](../../../../../../../../crates/renovate-core/src/util.rs#L9842) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9849`](../../../../../../../../crates/renovate-core/src/util.rs#L9849) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9856`](../../../../../../../../crates/renovate-core/src/util.rs#L9856) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:9865`](../../../../../../../../crates/renovate-core/src/util.rs#L9865) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:9873`](../../../../../../../../crates/renovate-core/src/util.rs#L9873) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:9881`](../../../../../../../../crates/renovate-core/src/util.rs#L9881) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:9887`](../../../../../../../../crates/renovate-core/src/util.rs#L9887) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:9881`](../../../../../../../../crates/renovate-core/src/util.rs#L9881) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:9919`](../../../../../../../../crates/renovate-core/src/util.rs#L9919) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:9929`](../../../../../../../../crates/renovate-core/src/util.rs#L9929) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:9940`](../../../../../../../../crates/renovate-core/src/util.rs#L9940) |

