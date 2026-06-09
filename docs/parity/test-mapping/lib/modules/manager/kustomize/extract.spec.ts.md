# `lib/modules/manager/kustomize/extract.spec.ts`

[← `manager/kustomize`](../../../../_by-module/manager/kustomize.md) · [all modules](../../../../README.md)

**44/45 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | should successfully parse a valid kustomize file | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1433`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1433) |
| 33 | return null on an invalid file | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1393`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1393) |
| 38 | should return null when header has invalid resource kind | ported | [`crates/renovate-core/src/extractors/kustomize.rs:675`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L675) |
| 47 | should fall back to default resource kind when header is missing | ported | [`crates/renovate-core/src/extractors/kustomize.rs:688`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L688) |
| 56 | should extract charthome | ported | [`crates/renovate-core/src/extractors/kustomize.rs:701`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L701) |
| 66 | should return null for a local base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1080`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1080) |
| 71 | should return null for an http base without ref/version | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1086`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1086) |
| 77 | should extract out the version of an http base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1092`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1092) |
| 90 | should extract the version of a non http base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1101`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1101) |
| 102 | should extract the depname if the url includes a port number | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1114`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1114) |
| 114 | should extract the version of a non http base with subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1126`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1126) |
| 126 | should extract out the version of an github base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1139`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1139) |
| 139 | should extract out the version of a git base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1148`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1148) |
| 152 | should extract out the version of a git base with subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1157`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1157) |
| 165 | should extract out the version of an http base with additional params | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1165`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1165) |
| 180 | should extract out the version of an http base from first version param | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1176`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1176) |
| 193 | should extract out the version of an http base from first ref param | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1186`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1186) |
| 208 | should return null on a null input | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1452`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1452) |
| 217 | should correctly extract a chart | ported | [`crates/renovate-core/src/extractors/kustomize.rs:883`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L883) |
| 233 | should correctly extract an oci chart | ported | [`crates/renovate-core/src/extractors/kustomize.rs:919`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L919) |
| 249 | should correctly extract an oci chart with registryaliases | ported | [`crates/renovate-core/src/extractors/kustomize.rs:941`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L941) |
| 270 | should return null for image with name only (no newtag/newname/digest) | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1019`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1019) |
| 275 | should return null on a null input | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1452`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1452) |
| 283 | should return null on invalid input | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1466`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1466) |
| 292 | should correctly extract a default image | ported | [`crates/renovate-core/src/extractors/kustomize.rs:636`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L636) |
| 310 | should correctly extract an image in a repo | ported | [`crates/renovate-core/src/extractors/kustomize.rs:714`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L714) |
| 328 | should correctly extract from a different registry | ported | [`crates/renovate-core/src/extractors/kustomize.rs:733`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L733) |
| 346 | should correctly extract from a different port | ported | [`crates/renovate-core/src/extractors/kustomize.rs:752`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L752) |
| 364 | should correctly extract from a multi-depth registry | ported | [`crates/renovate-core/src/extractors/kustomize.rs:771`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L771) |
| 382 | should correctly extract with registryaliases | ported | [`crates/renovate-core/src/extractors/kustomize.rs:790`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L790) |
| 405 | returns null for non kustomize kubernetes files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1415`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1415) |
| 421 | extracts multiple image lines | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1195`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1195) |
| 449 | extracts ssh dependency | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1227`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1227) |
| 467 | extracts ssh dependency with a subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1247`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1247) |
| 486 | extracts http dependency | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1266`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1266) |
| 511 | should extract out image versions | ported | [`crates/renovate-core/src/extractors/kustomize.rs:837`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L837) |
| 591 | ignores non-kubernetes empty files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1399`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1399) |
| 595 | does nothing with kustomize empty kustomize files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1405`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1405) |
| 603 | should extract bases resources and components from their respective blocks | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1295`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1295) |
| 637 | should extract dependencies when kind is component | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1327`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1327) |
| 680 | extracts from newtag | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1041`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1041) |
| 715 | extracts from digest | ported | [`crates/renovate-core/src/extractors/kustomize.rs:967`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L967) |
| 762 | extracts newname | ported | [`crates/renovate-core/src/extractors/kustomize.rs:817`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L817) |
| 804 | parses helmchart field | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1358`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1358) |
| 1109 | _(it.each / template — verify manually)_ | ? | — |

