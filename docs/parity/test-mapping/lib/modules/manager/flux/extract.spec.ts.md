# `lib/modules/manager/flux/extract.spec.ts`

[← `manager/flux`](../../../../_by-module/manager/flux.md) · [all modules](../../../../README.md)

**59/59 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 27 | extracts multiple resources | ported | `crates/renovate-core/src/extractors/flux.rs:821` |
| 72 | _(it.each / template — verify manually)_ | ? | — |
| 102 | considers components optional in system manifests | ported | `crates/renovate-core/src/extractors/flux.rs:846` |
| 111 | ignores system manifests without a version | ported | `crates/renovate-core/src/extractors/flux.rs:863` |
| 119 | extracts releases without repositories | ported | `crates/renovate-core/src/extractors/flux.rs:870` |
| 136 | falls back to unknown-registry when registryaliases has no matching helmrelease sourceref name | ported | `crates/renovate-core/src/extractors/flux.rs:887` |
| 158 | uses registryaliases to resolve helmrelease sourceref name when repository is missing | ported | `crates/renovate-core/src/extractors/flux.rs:898` |
| 180 | uses registryaliases with an oci url for helmrelease sourceref name | ported | `crates/renovate-core/src/extractors/flux.rs:910` |
| 202 | ignores helmrelease resources without an apiversion | ported | `crates/renovate-core/src/extractors/flux.rs:925` |
| 207 | ignores helmrepository resources without an apiversion | ported | `crates/renovate-core/src/extractors/flux.rs:931` |
| 212 | ignores helmrepository resources without metadata | ported | `crates/renovate-core/src/extractors/flux.rs:937` |
| 234 | ignores helmrelease resources without any chart reference | ported | `crates/renovate-core/src/extractors/flux.rs:948` |
| 250 | ignores helmrelease resources without a chart name | ported | `crates/renovate-core/src/extractors/flux.rs:963` |
| 271 | skip helmrelease with local chart | ported | `crates/renovate-core/src/extractors/flux.rs:983` |
| 299 | does not match helmrelease resources without a namespace to helmrepository resources without a namespace | ported | `crates/renovate-core/src/extractors/flux.rs:1014` |
| 325 | does not match helmrelease resources without a sourceref | ported | `crates/renovate-core/src/extractors/flux.rs:1039` |
| 355 | does not match helmrelease resources without a namespace | ported | `crates/renovate-core/src/extractors/flux.rs:1050` |
| 376 | ignores helmrepository resources without a namespace | ported | `crates/renovate-core/src/extractors/flux.rs:1059` |
| 400 | ignores helmrepository resources without a url | ported | `crates/renovate-core/src/extractors/flux.rs:1070` |
| 425 | ignores helmrelease resources using an invalid chartref | ported | `crates/renovate-core/src/extractors/flux.rs:1081` |
| 433 | ignores helmrelease resources using a chartref targetting a helmchart | ported | `crates/renovate-core/src/extractors/flux.rs:1100` |
| 457 | ignores helmrelease resources using a chartref targetting an ocirepository | ported | `crates/renovate-core/src/extractors/flux.rs:1119` |
| 492 | extracts helmchart version | ported | `crates/renovate-core/src/extractors/flux.rs:1131` |
| 513 | does not match helmchart resources without a namespace | ported | `crates/renovate-core/src/extractors/flux.rs:1145` |
| 544 | falls back to unknown-registry when registryaliases has no matching helmchart sourceref name | ported | `crates/renovate-core/src/extractors/flux.rs:1156` |
| 566 | uses registryaliases to resolve helmchart sourceref name when repository is missing | ported | `crates/renovate-core/src/extractors/flux.rs:1167` |
| 588 | ignores helmchart resources using git sources | ported | `crates/renovate-core/src/extractors/flux.rs:1179` |
| 608 | ignores helmchart resources using bucket sources | ported | `crates/renovate-core/src/extractors/flux.rs:1198` |
| 645 | ignores gitrepository without a tag nor a commit | ported | `crates/renovate-core/src/extractors/flux.rs:1238` |
| 665 | extracts gitrepository with a commit | ported | `crates/renovate-core/src/extractors/flux.rs:1259` |
| 694 | extracts gitrepository with a tag from github with ssh | ported | `crates/renovate-core/src/extractors/flux.rs:1275` |
| 722 | extracts gitrepository with a tag from github | ported | `crates/renovate-core/src/extractors/flux.rs:1293` |
| 750 | extracts gitrepository with a tag from gitlab | ported | `crates/renovate-core/src/extractors/flux.rs:1306` |
| 778 | extracts gitrepository with a tag from bitbucket | ported | `crates/renovate-core/src/extractors/flux.rs:1320` |
| 806 | extracts gitrepository with a tag from an unkown domain | ported | `crates/renovate-core/src/extractors/flux.rs:1334` |
| 834 | ignores ocirepository with no tag and no digest | ported | `crates/renovate-core/src/extractors/flux.rs:1348` |
| 861 | extracts ocirepository with a tag | ported | `crates/renovate-core/src/extractors/flux.rs:1368` |
| 897 | extracts ocirepository with a digest | ported | `crates/renovate-core/src/extractors/flux.rs:1386` |
| 925 | extracts ocirepository with a tag that contains a digest | ported | `crates/renovate-core/src/extractors/flux.rs:1400` |
| 958 | extracts ocirepository with a digest and tag | ported | `crates/renovate-core/src/extractors/flux.rs:1418` |
| 994 | extracts ocirepository with quoted digest and tag | ported | `crates/renovate-core/src/extractors/flux.rs:1435` |
| 1030 | extracts ocirepository with quoted keys | ported | `crates/renovate-core/src/extractors/flux.rs:1449` |
| 1063 | extracts ocirepository when ref key is quoted | ported | `crates/renovate-core/src/extractors/flux.rs:1463` |
| 1098 | skips ocirepository when tag value is a yaml alias | ported | `crates/renovate-core/src/extractors/flux.rs:1477` |
| 1129 | extracts ocirepository with tag and digest preceded by other document types | ported | `crates/renovate-core/src/extractors/flux.rs:1492` |
| 1195 | extracts ocirepository with tag and digest when preceded by same-named resource with scalar ref | ported | `crates/renovate-core/src/extractors/flux.rs:1510` |
| 1241 | extracts ocirepository with tag and digest when preceded by same-named resource with scalar spec | ported | `crates/renovate-core/src/extractors/flux.rs:1525` |
| 1285 | extracts ocirepository with tag and digest when ref contains a non-scalar key | ported | `crates/renovate-core/src/extractors/flux.rs:1540` |
| 1323 | extracts kustomization | ported | `crates/renovate-core/src/extractors/flux.rs:1554` |
| 1389 | ignores resources of an unknown kind | ported | `crates/renovate-core/src/extractors/flux.rs:1605` |
| 1400 | ignores resources without a kind | ported | `crates/renovate-core/src/extractors/flux.rs:1615` |
| 1408 | ignores bad manifests | ported | `crates/renovate-core/src/extractors/flux.rs:1625` |
| 1413 | ignores null resources | ported | `crates/renovate-core/src/extractors/flux.rs:1635` |
| 1420 | extracts multiple files | ported | `crates/renovate-core/src/extractors/flux.rs:1645` |
| 1486 | should handle helmrepository with type oci | ported | `crates/renovate-core/src/extractors/flux.rs:1705` |
| 1514 | should handle helmrepository w/o type oci and url starts with oci | ported | `crates/renovate-core/src/extractors/flux.rs:1734` |
| 1535 | ignores files that do not exist | ported | `crates/renovate-core/src/extractors/flux.rs:1688` |
| 1542 | ignores system manifest files without valid flux version header | ported | `crates/renovate-core/src/extractors/flux.rs:1695` |
| 1549 | should pick correct package file when using helmrepository with chartref | ported | `crates/renovate-core/src/extractors/flux.rs:1760` |

