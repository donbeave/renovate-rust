# `lib/workers/repository/update/pr/labels.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**13/17 in-scope tests ported** (4 pending, 3 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty array if no labels are configured | ported | [`crates/renovate-core/src/util.rs:9819`](../../../../../../../../crates/renovate-core/src/util.rs#L9819) |
| 16 | only labels | ported | [`crates/renovate-core/src/util.rs:9825`](../../../../../../../../crates/renovate-core/src/util.rs#L9825) |
| 22 | only addlabels | ported | [`crates/renovate-core/src/util.rs:9832`](../../../../../../../../crates/renovate-core/src/util.rs#L9832) |
| 30 | merge labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9839`](../../../../../../../../crates/renovate-core/src/util.rs#L9839) |
| 39 | deduplicate merged labels and addlabels | ported | [`crates/renovate-core/src/util.rs:9846`](../../../../../../../../crates/renovate-core/src/util.rs#L9846) |
| 48 | empty labels ignored | ported | [`crates/renovate-core/src/util.rs:9853`](../../../../../../../../crates/renovate-core/src/util.rs#L9853) |
| 57 | null labels ignored | opt-out | TS array inputs to prepareLabels can contain null/undefined (cast as never, with comment on empty slots as null); the prepare_labels in util.rs takes &[&str] (no nulls possible), and empty string filtering is already ported via 'empty labels ignored' (L48). Pure TypeScript runtime null/undef array handling with no direct Rust Vec analogue or need. |
| 68 | template labels | opt-out | tests templating in prepareLabels for labels using {{{datasource}}} syntax (rendered with provided datasource value); the core prepare_labels in util.rs is plain string filter/merge (ported), templating/render is separate runtime (likely using label template util or in pr layer); no direct Rust equivalent in the prepare path (labels passed pre-rendered or without this syntax). |
| 77 | template labels with empty datasource | opt-out | tests templating with undefined/empty datasource (results in empty after render/filter); same as sibling 'template labels' — templating runtime behavior with no direct in Rust prepare_labels (which doesn't do {{{ }}} render); empty filter covered by ported 'empty labels ignored'. |
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

