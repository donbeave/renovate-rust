# `lib/modules/manager/renovate-config/extract.spec.ts`

[← `manager/renovate-config`](../../../../_by-module/manager/renovate-config.md) · [all modules](../../../../README.md)

**18/20 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns null for empty file | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:776`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L776) |
| 11 | returns null for invalid file | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:782`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L782) |
| 18 | returns null for a config file without presets | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:788`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L788) |
| 34 | returns null for a config file only contains built-in presets | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:794`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L794) |
| 50 | provides skipreason for unsupported preset sources | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:801`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L801) |
| 88 | provides skipreason for presets without versions | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:827`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L827) |
| 120 | extracts from a config file with github hosted presets | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:847`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L847) |
| 161 | extracts from a config file with gitlab hosted presets | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:871`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L871) |
| 202 | extracts from a config file with gitea hosted presets | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:894`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L894) |
| 243 | supports json5 | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:917`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L917) |
| 269 | returns null for a config file without constraints | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:933`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L933) |
| 282 | returns null for a config file has an empty constraints | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:939`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L939) |
| 295 | extracts known `toolname`s with explicit versions | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:945`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L945) |
| 332 | extracts known `toolname`s with ranges versions | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:971`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L971) |
| 369 | extracts `toolname`s from packagerules | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:981`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L981) |
| 421 | handles no `constraints` in packagerules | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:1023`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L1023) |
| 451 | sets skipreason=unsupported for a constraint that is not a tool | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:1035`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L1035) |
| 476 | extracts known `toolname`s with ranges versions | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:971`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L971) |
| 513 | supports json5 | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:917`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L917) |
| 543 | extracts all types of configuration | ported | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:1083`](../../../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L1083) |

