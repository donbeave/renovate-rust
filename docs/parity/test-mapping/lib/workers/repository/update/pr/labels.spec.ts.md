# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/20 ported** (7 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:8274`](../../../../../../../../crates/renovate-core/src/util.rs#L8274) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:8280`](../../../../../../../../crates/renovate-core/src/util.rs#L8280) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:8287`](../../../../../../../../crates/renovate-core/src/util.rs#L8287) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:8294`](../../../../../../../../crates/renovate-core/src/util.rs#L8294) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:8301`](../../../../../../../../crates/renovate-core/src/util.rs#L8301) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:8308`](../../../../../../../../crates/renovate-core/src/util.rs#L8308) |
| 57 | null labels ignored | pending | — |
| 68 | template labels | pending | — |
| 77 | template labels with empty datasource | pending | — |
| 94 | github | pending | — |
| 102 | gitlab | pending | — |
| 115 | gitea | pending | — |
| 126 | adds new labels | ported | [`crates/renovate-core/src/util.rs:8317`](../../../../../../../../crates/renovate-core/src/util.rs#L8317) |
| 133 | removes old labels | ported | [`crates/renovate-core/src/util.rs:8325`](../../../../../../../../crates/renovate-core/src/util.rs#L8325) |
| 142 | returns true | ported | [`crates/renovate-core/src/util.rs:8333`](../../../../../../../../crates/renovate-core/src/util.rs#L8333) |
| 146 | returns false | ported | [`crates/renovate-core/src/util.rs:8339`](../../../../../../../../crates/renovate-core/src/util.rs#L8339) |
| 153 | returns true | ported | [`crates/renovate-core/src/util.rs:8333`](../../../../../../../../crates/renovate-core/src/util.rs#L8333) |
| 163 | returns false if no labels found in debugdata | ported | [`crates/renovate-core/src/util.rs:8371`](../../../../../../../../crates/renovate-core/src/util.rs#L8371) |
| 169 | returns false if labels have been modified by user | ported | [`crates/renovate-core/src/util.rs:8381`](../../../../../../../../crates/renovate-core/src/util.rs#L8381) |
| 173 | returns false if labels are not changed | ported | [`crates/renovate-core/src/util.rs:8392`](../../../../../../../../crates/renovate-core/src/util.rs#L8392) |

