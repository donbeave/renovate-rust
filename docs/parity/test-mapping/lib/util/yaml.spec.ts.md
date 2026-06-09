# `lib/util/yaml.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/19 in-scope tests ported** (9 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return empty array for empty string | ported | [`crates/renovate-core/src/util.rs:10324`](../../../../../crates/renovate-core/src/util.rs#L10324) |
| 11 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10333`](../../../../../crates/renovate-core/src/util.rs#L10333) |
| 26 | should parse content with single document with schema | pending | — |
| 50 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10343`](../../../../../crates/renovate-core/src/util.rs#L10343) |
| 70 | should parse content with multiple documents with schema | pending | — |
| 102 | should throw if schema does not match | pending | — |
| 122 | should throw if schema does not match and failurebehaviour "throw" | pending | — |
| 143 | should still return valid elements if schema does not match with "filter" behaviour | pending | — |
| 170 | should parse content with templates | ported | [`crates/renovate-core/src/util.rs:10354`](../../../../../crates/renovate-core/src/util.rs#L10354) |
| 193 | should parse content with templates without quotes | ported | [`crates/renovate-core/src/util.rs:10424`](../../../../../crates/renovate-core/src/util.rs#L10424) |
| 222 | should return undefined | ported | [`crates/renovate-core/src/util.rs:10438`](../../../../../crates/renovate-core/src/util.rs#L10438) |
| 226 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10333`](../../../../../crates/renovate-core/src/util.rs#L10333) |
| 239 | should parse invalid content using strict=false | ported | [`crates/renovate-core/src/util.rs:10414`](../../../../../crates/renovate-core/src/util.rs#L10414) |
| 253 | should parse content with single document with schema | pending | — |
| 275 | should throw with single document with schema if parsing fails | pending | — |
| 292 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10343`](../../../../../crates/renovate-core/src/util.rs#L10343) |
| 303 | should parse content with template | ported | [`crates/renovate-core/src/util.rs:10453`](../../../../../crates/renovate-core/src/util.rs#L10453) |
| 326 | should parse content with template without quotes | ported | [`crates/renovate-core/src/util.rs:10373`](../../../../../crates/renovate-core/src/util.rs#L10373) |
| 353 | should parse content with yaml tags | ported | [`crates/renovate-core/src/util.rs:10394`](../../../../../crates/renovate-core/src/util.rs#L10394) |

