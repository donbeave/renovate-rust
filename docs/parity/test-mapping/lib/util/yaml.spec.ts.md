# `lib/util/yaml.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/19 in-scope tests ported** (9 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return empty array for empty string | ported | [`crates/renovate-core/src/util.rs:10327`](../../../../../crates/renovate-core/src/util.rs#L10327) |
| 11 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10336`](../../../../../crates/renovate-core/src/util.rs#L10336) |
| 26 | should parse content with single document with schema | pending | — |
| 50 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10346`](../../../../../crates/renovate-core/src/util.rs#L10346) |
| 70 | should parse content with multiple documents with schema | pending | — |
| 102 | should throw if schema does not match | pending | — |
| 122 | should throw if schema does not match and failurebehaviour "throw" | pending | — |
| 143 | should still return valid elements if schema does not match with "filter" behaviour | pending | — |
| 170 | should parse content with templates | ported | [`crates/renovate-core/src/util.rs:10357`](../../../../../crates/renovate-core/src/util.rs#L10357) |
| 193 | should parse content with templates without quotes | ported | [`crates/renovate-core/src/util.rs:10427`](../../../../../crates/renovate-core/src/util.rs#L10427) |
| 222 | should return undefined | ported | [`crates/renovate-core/src/util.rs:10441`](../../../../../crates/renovate-core/src/util.rs#L10441) |
| 226 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10336`](../../../../../crates/renovate-core/src/util.rs#L10336) |
| 239 | should parse invalid content using strict=false | ported | [`crates/renovate-core/src/util.rs:10417`](../../../../../crates/renovate-core/src/util.rs#L10417) |
| 253 | should parse content with single document with schema | pending | — |
| 275 | should throw with single document with schema if parsing fails | pending | — |
| 292 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10346`](../../../../../crates/renovate-core/src/util.rs#L10346) |
| 303 | should parse content with template | ported | [`crates/renovate-core/src/util.rs:10456`](../../../../../crates/renovate-core/src/util.rs#L10456) |
| 326 | should parse content with template without quotes | ported | [`crates/renovate-core/src/util.rs:10376`](../../../../../crates/renovate-core/src/util.rs#L10376) |
| 353 | should parse content with yaml tags | ported | [`crates/renovate-core/src/util.rs:10397`](../../../../../crates/renovate-core/src/util.rs#L10397) |

