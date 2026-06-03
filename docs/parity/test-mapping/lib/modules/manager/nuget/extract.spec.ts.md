# `lib/modules/manager/nuget/extract.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**35/35 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 28 | returns null for invalid csproj | ported | `crates/renovate-core/src/extractors/nuget.rs:1845` |
| 43 | returns null if not xml | ported | `crates/renovate-core/src/extractors/nuget.rs:1853` |
| 61 | extracts package version dependency | ported | `crates/renovate-core/src/extractors/nuget.rs:1637` |
| 70 | extracts package file version | ported | `crates/renovate-core/src/extractors/nuget.rs:1653` |
| 79 | does not fail on package file without version | ported | `crates/renovate-core/src/extractors/cpanfile.rs:296` |
| 86 | extracts all dependencies | ported | `crates/renovate-core/src/extractors/nuget.rs:1553` |
| 94 | extracts msbuild sdk from the sdk attr of project element | ported | `crates/renovate-core/src/extractors/nuget.rs:1785` |
| 117 | does not extract msbuild sdk from the sdk attr of project element if version is missing | ported | `crates/renovate-core/src/extractors/nuget.rs:1801` |
| 132 | extracts msbuild sdk from the sdk element | ported | `crates/renovate-core/src/extractors/nuget.rs:1813` |
| 156 | does not extract msbuild sdk from the sdk element if version is missing | ported | `crates/renovate-core/src/extractors/nuget.rs:1863` |
| 172 | extracts msbuild sdk from the import element | ported | `crates/renovate-core/src/extractors/nuget.rs:1829` |
| 196 | does not extract msbuild sdk from the import element if version is missing | ported | `crates/renovate-core/src/extractors/nuget.rs:1876` |
| 212 | extracts dependency with lower-case version attribute | ported | `crates/renovate-core/src/extractors/nuget.rs:1770` |
| 226 | extracts all dependencies from global packages file | ported | `crates/renovate-core/src/extractors/nuget.rs:1722` |
| 234 | extracts containerbaseimage | ported | `crates/renovate-core/src/extractors/nuget.rs:1889` |
| 260 | extracts containerbaseimage with pinned digest | ported | `crates/renovate-core/src/extractors/nuget.rs:1906` |
| 289 | considers nuget.config | ported | `crates/renovate-core/src/extractors/nuget.rs:1949` |
| 309 | considers lower-case nuget.config | ported | `crates/renovate-core/src/extractors/nuget.rs:1981` |
| 330 | considers pascal-case nuget.config | ported | `crates/renovate-core/src/extractors/nuget.rs:2015` |
| 351 | handles malformed nuget.config | ported | `crates/renovate-core/src/extractors/nuget.rs:2049` |
| 368 | handles nuget.config without package sources | ported | `crates/renovate-core/src/extractors/nuget.rs:2073` |
| 385 | handles nuget.config with whitespaces in package source keys | ported | `crates/renovate-core/src/extractors/nuget.rs:2097` |
| 404 | ignores local feed in nuget.config | ported | `crates/renovate-core/src/extractors/nuget.rs:2133` |
| 422 | extracts registry urls independently | ported | `crates/renovate-core/src/extractors/nuget.rs:2162` |
| 461 | extracts msbuild-sdks from global.json | ported | `crates/renovate-core/src/extractors/nuget.rs:2340` |
| 483 | extracts dotnet-sdk from global.json | ported | `crates/renovate-core/src/extractors/nuget.rs:2364` |
| 501 | handles malformed global.json | ported | `crates/renovate-core/src/extractors/nuget.rs:2381` |
| 509 | handles not-a-nuget global.json | ported | `crates/renovate-core/src/extractors/nuget.rs:2387` |
| 521 | works | ported | `crates/renovate-core/src/extractors/nuget.rs:2212` |
| 537 | with-config | ported | `crates/renovate-core/src/extractors/nuget.rs:2233` |
| 561 | wrong version | ported | `crates/renovate-core/src/extractors/nuget.rs:2269` |
| 571 | returns null for no deps | ported | `crates/renovate-core/src/extractors/nuget.rs:2278` |
| 577 | does not throw | ported | `crates/renovate-core/src/extractors/nuget.rs:2285` |
| 583 | reads sdk and package directives | ported | `crates/renovate-core/src/extractors/nuget.rs:2292` |
| 615 | calls applyregistries to honor nuget.config files if present | ported | `crates/renovate-core/src/extractors/nuget.rs:2313` |

