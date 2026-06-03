# `lib/modules/manager/kubernetes/extract.spec.ts`

[← `manager/kubernetes`](../../../../_by-module/manager/kubernetes.md) · [all modules](../../../../README.md)

**15/15 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | returns null for empty | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:326`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L326) |
| 18 | does not return unknown kind | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:346`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L346) |
| 23 | extracts multiple kubernetes configurations | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:274`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L274) |
| 71 | extracts image line in a yaml array | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:602`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L602) |
| 98 | extracts image tag when it contains underscores | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:354`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L354) |
| 121 | ignores non-kubernetes yaml files | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:481`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L481) |
| 125 | handles invalid yaml files | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:338`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L338) |
| 133 | extracts images and replaces registries | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:378`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L378) |
| 155 | extracts images but does no replacement | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:403`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L403) |
| 177 | extracts images and does no double replacements | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:427`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L427) |
| 200 | extracts from complex templates | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:450`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L450) |
| 223 | _(it.each / template — verify manually)_ | ? | — |
| 265 | extracts image volumes from pod and cronjob | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:525`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L525) |
| 326 | does not extract image volumes for unsupported kind | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:561`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L561) |
| 349 | skips malformed volume entries and extracts valid ones | ported | [`crates/renovate-core/src/extractors/kubernetes.rs:578`](../../../../../../../crates/renovate-core/src/extractors/kubernetes.rs#L578) |

