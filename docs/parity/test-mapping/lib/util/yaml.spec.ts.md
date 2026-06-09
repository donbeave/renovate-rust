# `lib/util/yaml.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/19 in-scope tests ported** (9 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return empty array for empty string | ported | [`crates/renovate-core/src/util.rs:10325`](../../../../../crates/renovate-core/src/util.rs#L10325) |
| 11 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10334`](../../../../../crates/renovate-core/src/util.rs#L10334) |
| 26 | should parse content with single document with schema | pending | — |
| 50 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10344`](../../../../../crates/renovate-core/src/util.rs#L10344) |
| 70 | should parse content with multiple documents with schema | pending | — |
| 102 | should throw if schema does not match | pending | — |
| 122 | should throw if schema does not match and failurebehaviour "throw" | pending | — |
| 143 | should still return valid elements if schema does not match with "filter" behaviour | pending | — |
| 170 | should parse content with templates | ported | [`crates/renovate-core/src/util.rs:10355`](../../../../../crates/renovate-core/src/util.rs#L10355) |
| 193 | should parse content with templates without quotes | ported | [`crates/renovate-core/src/util.rs:10425`](../../../../../crates/renovate-core/src/util.rs#L10425) |
| 222 | should return undefined | ported | [`crates/renovate-core/src/util.rs:10439`](../../../../../crates/renovate-core/src/util.rs#L10439) |
| 226 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:10334`](../../../../../crates/renovate-core/src/util.rs#L10334) |
| 239 | should parse invalid content using strict=false | ported | [`crates/renovate-core/src/util.rs:10415`](../../../../../crates/renovate-core/src/util.rs#L10415) |
| 253 | should parse content with single document with schema | pending | — |
| 275 | should throw with single document with schema if parsing fails | pending | — |
| 292 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:10344`](../../../../../crates/renovate-core/src/util.rs#L10344) |
| 303 | should parse content with template | ported | [`crates/renovate-core/src/util.rs:10454`](../../../../../crates/renovate-core/src/util.rs#L10454) |
| 326 | should parse content with template without quotes | ported | [`crates/renovate-core/src/util.rs:10374`](../../../../../crates/renovate-core/src/util.rs#L10374) |
| 353 | should parse content with yaml tags | ported | [`crates/renovate-core/src/util.rs:10395`](../../../../../crates/renovate-core/src/util.rs#L10395) |

