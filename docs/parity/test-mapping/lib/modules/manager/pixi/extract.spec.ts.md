# `lib/modules/manager/pixi/extract.spec.ts`

[← `manager/pixi`](../../../../_by-module/manager/pixi.md) · [all modules](../../../../README.md)

**16/16 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 145 | returns null for empty pyproject.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:426` |
| 151 | returns null for empty pixi.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:396` |
| 155 | returns null for parsed file without pixi section | ported | `crates/renovate-core/src/extractors/pixi.rs:403` |
| 161 | returns parse pixi.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:308` |
| 297 | returns parse pixi section from pyproject.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:376` |
| 316 | returns package of pyproject.toml tool.pixi section | ported | `crates/renovate-core/src/extractors/pixi.rs:433` |
| 335 | returns parse pixi.toml with features | ported | `crates/renovate-core/src/extractors/pixi.rs:357` |
| 481 | returns parse non-known config file as pyproject.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:447` |
| 509 | returns parse non-known config file as pixi.toml | ported | `crates/renovate-core/src/extractors/pixi.rs:465` |
| 538 | extract feature with channels | ported | `crates/renovate-core/src/extractors/pixi.rs:483` |
| 571 | skip package without channels | ported | `crates/renovate-core/src/extractors/pixi.rs:597` |
| 601 | extract package from with workspace | ported | `crates/renovate-core/src/extractors/pixi.rs:504` |
| 630 | extract package with channel priority | ported | `crates/renovate-core/src/extractors/pixi.rs:521` |
| 681 | returns null for non-known config file | ported | `crates/renovate-core/src/extractors/pixi.rs:590` |
| 685 | set registrystrategy='merge' for channel-priority='disabled'" | ported | `crates/renovate-core/src/extractors/pixi.rs:555` |
| 706 | use default registrystrategy for channel-priority='strict'" | ported | `crates/renovate-core/src/extractors/pixi.rs:573` |

