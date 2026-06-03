# `lib/modules/manager/cargo/extract.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**32/32 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 46 | returns null for invalid toml | ported | `crates/renovate-core/src/extractors/cargo.rs:825` |
| 52 | returns null for empty dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:679` |
| 59 | returns null for empty dev-dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:801` |
| 66 | returns null for empty custom target | ported | `crates/renovate-core/src/extractors/cargo.rs:817` |
| 73 | extracts multiple dependencies simple | ported | `crates/renovate-core/src/extractors/cargo.rs:584` |
| 79 | extracts multiple dependencies advanced | ported | `crates/renovate-core/src/extractors/cargo.rs:691` |
| 85 | handles inline tables | ported | `crates/renovate-core/src/extractors/cargo.rs:841` |
| 91 | handles standard tables | ported | `crates/renovate-core/src/extractors/cargo.rs:599` |
| 97 | extracts platform specific dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:766` |
| 103 | extracts registry urls from .cargo/config.toml | ported | `crates/renovate-core/src/extractors/cargo.rs:920` |
| 112 | extracts registry urls from .cargo/config (legacy path) | ported | `crates/renovate-core/src/extractors/cargo.rs:950` |
| 121 | extracts overridden registry indexes from .cargo/config.toml | ported | `crates/renovate-core/src/extractors/cargo.rs:970` |
| 180 | extracts overridden source registry indexes from .cargo/config.toml | ported | `crates/renovate-core/src/extractors/cargo.rs:1016` |
| 205 | extracts registries overridden to the default | ported | `crates/renovate-core/src/extractors/cargo.rs:1051` |
| 249 | extracts registries with an empty config.toml | ported | `crates/renovate-core/src/extractors/cargo.rs:1082` |
| 299 | extracts registry urls from environment | ported | `crates/renovate-core/src/extractors/cargo.rs:1117` |
| 345 | extracts workspace dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:725` |
| 390 | skips workspace dependency | ported | `crates/renovate-core/src/extractors/cargo.rs:638` |
| 407 | skips unknown registries | ported | `crates/renovate-core/src/extractors/cargo.rs:1155` |
| 415 | fails to parse cargo config with invalid toml | ported | `crates/renovate-core/src/extractors/cargo.rs:1164` |
| 424 | ignore cargo config registries with missing index | ported | `crates/renovate-core/src/extractors/cargo.rs:1182` |
| 433 | ignore cargo config source replaced registries with missing index | ported | `crates/renovate-core/src/extractors/cargo.rs:1200` |
| 481 | ignore cargo config with circular registry source replacements | ported | `crates/renovate-core/src/extractors/cargo.rs:1224` |
| 539 | extracts original package name of renamed dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:612` |
| 549 | extracts locked versions | ported | `crates/renovate-core/src/extractors/cargo.rs:1346` |
| 567 | does not extract locked versions for git dependencies | ported | `crates/renovate-core/src/extractors/cargo.rs:650` |
| 585 | extracts locked versions for renamed packages | ported | `crates/renovate-core/src/extractors/cargo.rs:1386` |
| 601 | handles missing locked versions | ported | `crates/renovate-core/src/extractors/cargo.rs:1400` |
| 617 | handles invalid versions in the toml file | ported | `crates/renovate-core/src/extractors/cargo.rs:1415` |
| 635 | handles invalid lock file | ported | `crates/renovate-core/src/extractors/cargo.rs:1436` |
| 650 | should extract project version | ported | `crates/renovate-core/src/extractors/cargo.rs:883` |
| 664 | should extract project version from workspace | ported | `crates/renovate-core/src/extractors/cargo.rs:890` |

