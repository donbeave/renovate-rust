# `lib/modules/datasource/metadata.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**10/32 in-scope tests ported** (22 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | should handle manualchangelogurls | ported | [`crates/renovate-core/src/datasources.rs:959`](../../../../../../crates/renovate-core/src/datasources.rs#L959) |
| 51 | should handle manualsourceurls | ported | [`crates/renovate-core/src/datasources.rs:982`](../../../../../../crates/renovate-core/src/datasources.rs#L982) |
| 82 | should handle parsing of sourceurls correctly | ported | [`crates/renovate-core/src/datasources.rs:999`](../../../../../../crates/renovate-core/src/datasources.rs#L999) |
| 113 | _(it.each / template — verify manually)_ | ? | — |
| 134 | _(it.each / template — verify manually)_ | ? | — |
| 158 | _(it.each / template — verify manually)_ | ? | — |
| 180 | should not overwrite any existing sourcedirectory | pending | — |
| 197 | should massage github sourceurls | ported | [`crates/renovate-core/src/datasources.rs:1350`](../../../../../../crates/renovate-core/src/datasources.rs#L1350) |
| 228 | should handle parsing of sourceurls correctly for gitlab also | pending | — |
| 251 | should handle failed parsing of sourceurls for gitlab | pending | — |
| 274 | should handle failed parsing of sourceurls for other | pending | — |
| 297 | should handle non-url | pending | — |
| 319 | should handle parsing/converting of github sourceurls with http and www correctly | ported | [`crates/renovate-core/src/datasources.rs:1393`](../../../../../../crates/renovate-core/src/datasources.rs#L1393) |
| 331 | should move github homepage to sourceurl | pending | — |
| 345 | should handle parsing/converting of gitlab sourceurls with http and www correctly | pending | — |
| 357 | should normalize releasetimestamp | pending | — |
| 385 | should return an empty string when massaging an invalid url | ported | [`crates/renovate-core/src/util.rs:12336`](../../../../../../crates/renovate-core/src/util.rs#L12336) |
| 389 | _(it.each / template — verify manually)_ | ? | — |
| 403 | _(it.each / template — verify manually)_ | ? | — |
| 415 | _(it.each / template — verify manually)_ | ? | — |
| 428 | should massage github git@ url to valid https url | pending | — |
| 434 | should massage github http url to valid https url | pending | — |
| 440 | should massage github http and git url to valid https url | pending | — |
| 446 | should massage github ssh git@ url to valid https url | pending | — |
| 452 | should massage github git url to valid https url | pending | — |
| 458 | should massage gitlab git url to valid https url | pending | — |
| 464 | should remove homepage when homepage and sourceurl are same | pending | — |
| 503 | should delete gitlab homepage if its same as sourceurl | pending | — |
| 542 | does not set homepage to sourceurl when undefined | pending | — |
| 580 | does not set homepage to sourceurl when not github or gitlab | pending | — |
| 618 | _(it.each / template — verify manually)_ | ? | — |
| 638 | should handle dep with no releases | pending | — |

