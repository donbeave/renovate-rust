# `lib/modules/manager/nuget/extract.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**35/35 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 28 | returns null for invalid csproj | ported | [`crates/renovate-core/src/extractors/nuget.rs:1845`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1845) |
| 43 | returns null if not xml | ported | [`crates/renovate-core/src/extractors/nuget.rs:1853`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1853) |
| 61 | extracts package version dependency | ported | [`crates/renovate-core/src/extractors/nuget.rs:1637`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1637) |
| 70 | extracts package file version | ported | [`crates/renovate-core/src/extractors/nuget.rs:1653`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1653) |
| 79 | does not fail on package file without version | ported | [`crates/renovate-core/src/extractors/cpanfile.rs:296`](../../../../../../../crates/renovate-core/src/extractors/cpanfile.rs#L296) |
| 86 | extracts all dependencies | ported | [`crates/renovate-core/src/extractors/nuget.rs:1553`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1553) |
| 94 | extracts msbuild sdk from the sdk attr of project element | ported | [`crates/renovate-core/src/extractors/nuget.rs:1785`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1785) |
| 117 | does not extract msbuild sdk from the sdk attr of project element if version is missing | ported | [`crates/renovate-core/src/extractors/nuget.rs:1801`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1801) |
| 132 | extracts msbuild sdk from the sdk element | ported | [`crates/renovate-core/src/extractors/nuget.rs:1813`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1813) |
| 156 | does not extract msbuild sdk from the sdk element if version is missing | ported | [`crates/renovate-core/src/extractors/nuget.rs:1863`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1863) |
| 172 | extracts msbuild sdk from the import element | ported | [`crates/renovate-core/src/extractors/nuget.rs:1829`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1829) |
| 196 | does not extract msbuild sdk from the import element if version is missing | ported | [`crates/renovate-core/src/extractors/nuget.rs:1876`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1876) |
| 212 | extracts dependency with lower-case version attribute | ported | [`crates/renovate-core/src/extractors/nuget.rs:1770`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1770) |
| 226 | extracts all dependencies from global packages file | ported | [`crates/renovate-core/src/extractors/nuget.rs:1722`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1722) |
| 234 | extracts containerbaseimage | ported | [`crates/renovate-core/src/extractors/nuget.rs:1889`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1889) |
| 260 | extracts containerbaseimage with pinned digest | ported | [`crates/renovate-core/src/extractors/nuget.rs:1906`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1906) |
| 289 | considers nuget.config | ported | [`crates/renovate-core/src/extractors/nuget.rs:1949`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1949) |
| 309 | considers lower-case nuget.config | ported | [`crates/renovate-core/src/extractors/nuget.rs:1981`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L1981) |
| 330 | considers pascal-case nuget.config | ported | [`crates/renovate-core/src/extractors/nuget.rs:2015`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2015) |
| 351 | handles malformed nuget.config | ported | [`crates/renovate-core/src/extractors/nuget.rs:2049`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2049) |
| 368 | handles nuget.config without package sources | ported | [`crates/renovate-core/src/extractors/nuget.rs:2073`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2073) |
| 385 | handles nuget.config with whitespaces in package source keys | ported | [`crates/renovate-core/src/extractors/nuget.rs:2097`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2097) |
| 404 | ignores local feed in nuget.config | ported | [`crates/renovate-core/src/extractors/nuget.rs:2133`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2133) |
| 422 | extracts registry urls independently | ported | [`crates/renovate-core/src/extractors/nuget.rs:2162`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2162) |
| 461 | extracts msbuild-sdks from global.json | ported | [`crates/renovate-core/src/extractors/nuget.rs:2340`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2340) |
| 483 | extracts dotnet-sdk from global.json | ported | [`crates/renovate-core/src/extractors/nuget.rs:2364`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2364) |
| 501 | handles malformed global.json | ported | [`crates/renovate-core/src/extractors/nuget.rs:2381`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2381) |
| 509 | handles not-a-nuget global.json | ported | [`crates/renovate-core/src/extractors/nuget.rs:2387`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2387) |
| 521 | works | ported | [`crates/renovate-core/src/extractors/nuget.rs:2212`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2212) |
| 537 | with-config | ported | [`crates/renovate-core/src/extractors/nuget.rs:2233`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2233) |
| 561 | wrong version | ported | [`crates/renovate-core/src/extractors/nuget.rs:2269`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2269) |
| 571 | returns null for no deps | ported | [`crates/renovate-core/src/extractors/nuget.rs:2278`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2278) |
| 577 | does not throw | ported | [`crates/renovate-core/src/extractors/nuget.rs:2285`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2285) |
| 583 | reads sdk and package directives | ported | [`crates/renovate-core/src/extractors/nuget.rs:2292`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2292) |
| 615 | calls applyregistries to honor nuget.config files if present | ported | [`crates/renovate-core/src/extractors/nuget.rs:2313`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2313) |

