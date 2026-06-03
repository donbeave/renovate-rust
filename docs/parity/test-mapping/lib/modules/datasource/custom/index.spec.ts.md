# `lib/modules/datasource/custom/index.spec.ts`

[← `datasource/custom`](../../../../_by-module/datasource/custom.md) · [all modules](../../../../README.md)

**1/30 ported** (29 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | return null if only the prefix is supplied | pending | — |
| 22 | return null if no registryurl is provided as well no defaultregistrytemplate is defined | pending | — |
| 33 | return null if no custom datasource could be found | pending | — |
| 42 | return null on http error | pending | — |
| 56 | return null if schema validation fails | pending | — |
| 72 | return releases for api directly exposing in renovate format | pending | — |
| 93 | return releases with digests for api directly exposing in renovate format | pending | — |
| 123 | return releases with tags and other optional fields for api directly exposing in renovate format | pending | — |
| 166 | return releases for plain text api directly exposing in renovate format | pending | — |
| 199 | return releases for plain text api and trim the content | pending | — |
| 232 | returns null if transformation compilation using jsonata fails | pending | — |
| 258 | returns null if jsonata expression evaluation fails | pending | — |
| 284 | return releases for plain text api when only returns a single version | pending | — |
| 308 | return releases for yaml api directly exposing in renovate format | pending | — |
| 348 | return releases for yaml file directly exposing in renovate format | pending | — |
| 384 | returns releases for toml api directly exposing in renovate format | pending | — |
| 426 | return releases for toml file directly exposing in renovate format | pending | — |
| 464 | return releases for json file directly exposing in renovate format | pending | — |
| 501 | return null for plain text file if the body is not what is expected | pending | — |
| 518 | return releases for plain text file directly exposing in renovate format | pending | — |
| 553 | return release when templating registryurl | pending | — |
| 578 | return release with templated path | pending | — |
| 613 | return release with templated path with multiple layers | pending | — |
| 650 | return releases from html links | pending | — |
| 688 | return releases from html links - local file | pending | — |
| 721 | return null for local file read error - html format | pending | — |
| 738 | return releases from nginx directory listing | ported | [`crates/renovate-core/src/datasources/artifactory.rs:378`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L378) |
| 778 | return releases for malformed html | pending | — |
| 815 | return releases for incomplete html | pending | — |
| 854 | returns null as digest should be provided in releases | pending | — |

