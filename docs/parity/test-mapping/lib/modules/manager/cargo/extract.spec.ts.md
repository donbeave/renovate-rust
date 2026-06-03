# `lib/modules/manager/cargo/extract.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**32/32 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | returns null for invalid toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:825`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L825) |
| 52 | returns null for empty dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:679`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L679) |
| 59 | returns null for empty dev-dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:801`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L801) |
| 66 | returns null for empty custom target | ported | [`crates/renovate-core/src/extractors/cargo.rs:817`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L817) |
| 73 | extracts multiple dependencies simple | ported | [`crates/renovate-core/src/extractors/cargo.rs:584`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L584) |
| 79 | extracts multiple dependencies advanced | ported | [`crates/renovate-core/src/extractors/cargo.rs:691`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L691) |
| 85 | handles inline tables | ported | [`crates/renovate-core/src/extractors/cargo.rs:841`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L841) |
| 91 | handles standard tables | ported | [`crates/renovate-core/src/extractors/cargo.rs:599`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L599) |
| 97 | extracts platform specific dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:766`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L766) |
| 103 | extracts registry urls from .cargo/config.toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:920`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L920) |
| 112 | extracts registry urls from .cargo/config (legacy path) | ported | [`crates/renovate-core/src/extractors/cargo.rs:950`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L950) |
| 121 | extracts overridden registry indexes from .cargo/config.toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:970`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L970) |
| 180 | extracts overridden source registry indexes from .cargo/config.toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:1016`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1016) |
| 205 | extracts registries overridden to the default | ported | [`crates/renovate-core/src/extractors/cargo.rs:1051`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1051) |
| 249 | extracts registries with an empty config.toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:1082`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1082) |
| 299 | extracts registry urls from environment | ported | [`crates/renovate-core/src/extractors/cargo.rs:1117`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1117) |
| 345 | extracts workspace dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:725`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L725) |
| 390 | skips workspace dependency | ported | [`crates/renovate-core/src/extractors/cargo.rs:638`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L638) |
| 407 | skips unknown registries | ported | [`crates/renovate-core/src/extractors/cargo.rs:1155`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1155) |
| 415 | fails to parse cargo config with invalid toml | ported | [`crates/renovate-core/src/extractors/cargo.rs:1164`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1164) |
| 424 | ignore cargo config registries with missing index | ported | [`crates/renovate-core/src/extractors/cargo.rs:1182`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1182) |
| 433 | ignore cargo config source replaced registries with missing index | ported | [`crates/renovate-core/src/extractors/cargo.rs:1200`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1200) |
| 481 | ignore cargo config with circular registry source replacements | ported | [`crates/renovate-core/src/extractors/cargo.rs:1224`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1224) |
| 539 | extracts original package name of renamed dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:612`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L612) |
| 549 | extracts locked versions | ported | [`crates/renovate-core/src/extractors/cargo.rs:1346`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1346) |
| 567 | does not extract locked versions for git dependencies | ported | [`crates/renovate-core/src/extractors/cargo.rs:650`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L650) |
| 585 | extracts locked versions for renamed packages | ported | [`crates/renovate-core/src/extractors/cargo.rs:1386`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1386) |
| 601 | handles missing locked versions | ported | [`crates/renovate-core/src/extractors/cargo.rs:1400`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1400) |
| 617 | handles invalid versions in the toml file | ported | [`crates/renovate-core/src/extractors/cargo.rs:1415`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1415) |
| 635 | handles invalid lock file | ported | [`crates/renovate-core/src/extractors/cargo.rs:1436`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1436) |
| 650 | should extract project version | ported | [`crates/renovate-core/src/extractors/cargo.rs:883`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L883) |
| 664 | should extract project version from workspace | ported | [`crates/renovate-core/src/extractors/cargo.rs:890`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L890) |

