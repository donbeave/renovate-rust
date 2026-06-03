# `lib/modules/manager/kustomize/extract.spec.ts`

[← `manager/kustomize`](../../../../_by-module/manager/kustomize.md) · [all modules](../../../../README.md)

**43/45 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | should successfully parse a valid kustomize file | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1427`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1427) |
| 33 | return null on an invalid file | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1387`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1387) |
| 38 | should return null when header has invalid resource kind | ported | [`crates/renovate-core/src/extractors/kustomize.rs:669`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L669) |
| 47 | should fall back to default resource kind when header is missing | ported | [`crates/renovate-core/src/extractors/kustomize.rs:682`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L682) |
| 56 | should extract charthome | ported | [`crates/renovate-core/src/extractors/kustomize.rs:695`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L695) |
| 66 | should return null for a local base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1074`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1074) |
| 71 | should return null for an http base without ref/version | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1080`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1080) |
| 77 | should extract out the version of an http base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1086`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1086) |
| 90 | should extract the version of a non http base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1095`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1095) |
| 102 | should extract the depname if the url includes a port number | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1108`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1108) |
| 114 | should extract the version of a non http base with subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1120`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1120) |
| 126 | should extract out the version of an github base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1133`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1133) |
| 139 | should extract out the version of a git base | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1142`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1142) |
| 152 | should extract out the version of a git base with subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1151`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1151) |
| 165 | should extract out the version of an http base with additional params | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1159`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1159) |
| 180 | should extract out the version of an http base from first version param | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1170`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1170) |
| 193 | should extract out the version of an http base from first ref param | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1180`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1180) |
| 208 | should return null on a null input | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1446`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1446) |
| 217 | should correctly extract a chart | ported | [`crates/renovate-core/src/extractors/kustomize.rs:877`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L877) |
| 233 | should correctly extract an oci chart | ported | [`crates/renovate-core/src/extractors/kustomize.rs:913`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L913) |
| 249 | should correctly extract an oci chart with registryaliases | ported | [`crates/renovate-core/src/extractors/kustomize.rs:935`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L935) |
| 270 | should return null for image with name only (no newtag/newname/digest) | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1013`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1013) |
| 275 | should return null on a null input | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1446`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1446) |
| 283 | should return null on invalid input | pending | — |
| 292 | should correctly extract a default image | ported | [`crates/renovate-core/src/extractors/kustomize.rs:630`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L630) |
| 310 | should correctly extract an image in a repo | ported | [`crates/renovate-core/src/extractors/kustomize.rs:708`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L708) |
| 328 | should correctly extract from a different registry | ported | [`crates/renovate-core/src/extractors/kustomize.rs:727`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L727) |
| 346 | should correctly extract from a different port | ported | [`crates/renovate-core/src/extractors/kustomize.rs:746`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L746) |
| 364 | should correctly extract from a multi-depth registry | ported | [`crates/renovate-core/src/extractors/kustomize.rs:765`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L765) |
| 382 | should correctly extract with registryaliases | ported | [`crates/renovate-core/src/extractors/kustomize.rs:784`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L784) |
| 405 | returns null for non kustomize kubernetes files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1409`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1409) |
| 421 | extracts multiple image lines | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1189`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1189) |
| 449 | extracts ssh dependency | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1221`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1221) |
| 467 | extracts ssh dependency with a subdir | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1241`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1241) |
| 486 | extracts http dependency | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1260`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1260) |
| 511 | should extract out image versions | ported | [`crates/renovate-core/src/extractors/kustomize.rs:831`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L831) |
| 591 | ignores non-kubernetes empty files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1393`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1393) |
| 595 | does nothing with kustomize empty kustomize files | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1399`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1399) |
| 603 | should extract bases resources and components from their respective blocks | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1289`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1289) |
| 637 | should extract dependencies when kind is component | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1321`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1321) |
| 680 | extracts from newtag | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1035`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1035) |
| 715 | extracts from digest | ported | [`crates/renovate-core/src/extractors/kustomize.rs:961`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L961) |
| 762 | extracts newname | ported | [`crates/renovate-core/src/extractors/kustomize.rs:811`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L811) |
| 804 | parses helmchart field | ported | [`crates/renovate-core/src/extractors/kustomize.rs:1352`](../../../../../../../crates/renovate-core/src/extractors/kustomize.rs#L1352) |
| 1109 | _(it.each / template — verify manually)_ | ? | — |

