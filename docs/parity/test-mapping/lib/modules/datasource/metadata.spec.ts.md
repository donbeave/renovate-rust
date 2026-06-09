# `lib/modules/datasource/metadata.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**29/32 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | should handle manualchangelogurls | ported | [`crates/renovate-core/src/datasources.rs:959`](../../../../../../crates/renovate-core/src/datasources.rs#L959) |
| 51 | should handle manualsourceurls | ported | [`crates/renovate-core/src/datasources.rs:982`](../../../../../../crates/renovate-core/src/datasources.rs#L982) |
| 82 | should handle parsing of sourceurls correctly | ported | [`crates/renovate-core/src/datasources.rs:999`](../../../../../../crates/renovate-core/src/datasources.rs#L999) |
| 113 | _(it.each / template — verify manually)_ | ? | — |
| 134 | _(it.each / template — verify manually)_ | ? | — |
| 158 | _(it.each / template — verify manually)_ | ? | — |
| 180 | should not overwrite any existing sourcedirectory | ported | [`crates/renovate-core/src/datasources.rs:1051`](../../../../../../crates/renovate-core/src/datasources.rs#L1051) |
| 197 | should massage github sourceurls | ported | [`crates/renovate-core/src/datasources.rs:1660`](../../../../../../crates/renovate-core/src/datasources.rs#L1660) |
| 228 | should handle parsing of sourceurls correctly for gitlab also | ported | [`crates/renovate-core/src/datasources.rs:1092`](../../../../../../crates/renovate-core/src/datasources.rs#L1092) |
| 251 | should handle failed parsing of sourceurls for gitlab | ported | [`crates/renovate-core/src/datasources.rs:1203`](../../../../../../crates/renovate-core/src/datasources.rs#L1203) |
| 274 | should handle failed parsing of sourceurls for other | ported | [`crates/renovate-core/src/datasources.rs:1237`](../../../../../../crates/renovate-core/src/datasources.rs#L1237) |
| 297 | should handle non-url | ported | [`crates/renovate-core/src/datasources.rs:1224`](../../../../../../crates/renovate-core/src/datasources.rs#L1224) |
| 319 | should handle parsing/converting of github sourceurls with http and www correctly | ported | [`crates/renovate-core/src/datasources.rs:1703`](../../../../../../crates/renovate-core/src/datasources.rs#L1703) |
| 331 | should move github homepage to sourceurl | ported | [`crates/renovate-core/src/datasources.rs:1072`](../../../../../../crates/renovate-core/src/datasources.rs#L1072) |
| 345 | should handle parsing/converting of gitlab sourceurls with http and www correctly | ported | [`crates/renovate-core/src/datasources.rs:1118`](../../../../../../crates/renovate-core/src/datasources.rs#L1118) |
| 357 | should normalize releasetimestamp | ported | [`crates/renovate-core/src/datasources.rs:1261`](../../../../../../crates/renovate-core/src/datasources.rs#L1261) |
| 385 | should return an empty string when massaging an invalid url | ported | [`crates/renovate-core/src/util.rs:12339`](../../../../../../crates/renovate-core/src/util.rs#L12339) |
| 389 | _(it.each / template — verify manually)_ | ? | — |
| 403 | _(it.each / template — verify manually)_ | ? | — |
| 415 | _(it.each / template — verify manually)_ | ? | — |
| 428 | should massage github git@ url to valid https url | ported | [`crates/renovate-core/src/util.rs:12443`](../../../../../../crates/renovate-core/src/util.rs#L12443) |
| 434 | should massage github http url to valid https url | ported | [`crates/renovate-core/src/util.rs:12451`](../../../../../../crates/renovate-core/src/util.rs#L12451) |
| 440 | should massage github http and git url to valid https url | ported | [`crates/renovate-core/src/util.rs:12460`](../../../../../../crates/renovate-core/src/util.rs#L12460) |
| 446 | should massage github ssh git@ url to valid https url | ported | [`crates/renovate-core/src/util.rs:12469`](../../../../../../crates/renovate-core/src/util.rs#L12469) |
| 452 | should massage github git url to valid https url | ported | [`crates/renovate-core/src/util.rs:12478`](../../../../../../crates/renovate-core/src/util.rs#L12478) |
| 458 | should massage gitlab git url to valid https url | ported | [`crates/renovate-core/src/util.rs:12486`](../../../../../../crates/renovate-core/src/util.rs#L12486) |
| 464 | should remove homepage when homepage and sourceurl are same | ported | [`crates/renovate-core/src/datasources.rs:1137`](../../../../../../crates/renovate-core/src/datasources.rs#L1137) |
| 503 | should delete gitlab homepage if its same as sourceurl | ported | [`crates/renovate-core/src/datasources.rs:1170`](../../../../../../crates/renovate-core/src/datasources.rs#L1170) |
| 542 | does not set homepage to sourceurl when undefined | ported | [`crates/renovate-core/src/datasources.rs:1310`](../../../../../../crates/renovate-core/src/datasources.rs#L1310) |
| 580 | does not set homepage to sourceurl when not github or gitlab | ported | [`crates/renovate-core/src/datasources.rs:1328`](../../../../../../crates/renovate-core/src/datasources.rs#L1328) |
| 618 | _(it.each / template — verify manually)_ | ? | — |
| 638 | should handle dep with no releases | ported | [`crates/renovate-core/src/datasources.rs:1343`](../../../../../../crates/renovate-core/src/datasources.rs#L1343) |

