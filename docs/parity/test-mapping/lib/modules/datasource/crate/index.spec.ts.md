# `lib/modules/datasource/crate/index.spec.ts`

[← `datasource/crate`](../../../../_by-module/datasource/crate.md) · [all modules](../../../../README.md)

**22/27 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 98 | returns correct suffixes | ported | [`crates/renovate-core/src/datasources/crates_io.rs:725`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L725) |
| 148 | returns null for missing registry url | ported | [`crates/renovate-core/src/datasources/crates_io.rs:930`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L930) |
| 163 | returns null for invalid registry url | ported | [`crates/renovate-core/src/datasources/crates_io.rs:946`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L946) |
| 173 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/crates_io.rs:953`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L953) |
| 189 | returns null for missing fields | ported | [`crates/renovate-core/src/datasources/crates_io.rs:969`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L969) |
| 205 | returns null for empty list | ported | [`crates/renovate-core/src/datasources/crates_io.rs:985`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L985) |
| 221 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1001`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1001) |
| 235 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1017`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1017) |
| 249 | returns null for unknown error | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1033`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1033) |
| 263 | processes real data: libc | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1045`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1045) |
| 281 | processes real data: amethyst | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1109`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1109) |
| 299 | uses cached registry config for subsequent packages | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1155`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1155) |
| 329 | refuses to clone if allowcustomcrateregistries is not true | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1234`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1234) |
| 342 | clones cloudsmith private registry | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1270`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1270) |
| 357 | clones other private registry with explicit gittimeout | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1298`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1298) |
| 374 | clones other private registry | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1289`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1289) |
| 389 | clones once then reuses the cache | pending | — |
| 406 | reads config.json from cloned registry | pending | — |
| 419 | guards against race conditions while cloning | pending | — |
| 446 | returns null when git clone fails | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1308`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1308) |
| 466 | does not clone for sparse registries | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1199`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1199) |
| 484 | retries if shallow fails because of dumb http git repo | pending | — |
| 530 | retries if shallow fails but retry can also fail | pending | — |
| 569 | no-op for registries without cached config | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1325`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1325) |
| 583 | no-op when registryurl is null | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1334`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1334) |
| 597 | no-op for release with timestamp | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1342`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1342) |
| 614 | fetches releasetimestamp | ported | [`crates/renovate-core/src/datasources/crates_io.rs:1353`](../../../../../../../crates/renovate-core/src/datasources/crates_io.rs#L1353) |

