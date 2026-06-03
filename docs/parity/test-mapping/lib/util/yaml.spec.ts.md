# `lib/util/yaml.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/19 ported** (9 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | should return empty array for empty string | ported | [`crates/renovate-core/src/util.rs:8771`](../../../../../crates/renovate-core/src/util.rs#L8771) |
| 11 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:8780`](../../../../../crates/renovate-core/src/util.rs#L8780) |
| 26 | should parse content with single document with schema | pending | — |
| 50 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:8790`](../../../../../crates/renovate-core/src/util.rs#L8790) |
| 70 | should parse content with multiple documents with schema | pending | — |
| 102 | should throw if schema does not match | pending | — |
| 122 | should throw if schema does not match and failurebehaviour "throw" | pending | — |
| 143 | should still return valid elements if schema does not match with "filter" behaviour | pending | — |
| 170 | should parse content with templates | ported | [`crates/renovate-core/src/util.rs:8801`](../../../../../crates/renovate-core/src/util.rs#L8801) |
| 193 | should parse content with templates without quotes | ported | [`crates/renovate-core/src/util.rs:8871`](../../../../../crates/renovate-core/src/util.rs#L8871) |
| 222 | should return undefined | ported | [`crates/renovate-core/src/util.rs:8885`](../../../../../crates/renovate-core/src/util.rs#L8885) |
| 226 | should parse content with single document | ported | [`crates/renovate-core/src/util.rs:8780`](../../../../../crates/renovate-core/src/util.rs#L8780) |
| 239 | should parse invalid content using strict=false | ported | [`crates/renovate-core/src/util.rs:8861`](../../../../../crates/renovate-core/src/util.rs#L8861) |
| 253 | should parse content with single document with schema | pending | — |
| 275 | should throw with single document with schema if parsing fails | pending | — |
| 292 | should parse content with multiple documents | ported | [`crates/renovate-core/src/util.rs:8790`](../../../../../crates/renovate-core/src/util.rs#L8790) |
| 303 | should parse content with template | ported | [`crates/renovate-core/src/util.rs:8900`](../../../../../crates/renovate-core/src/util.rs#L8900) |
| 326 | should parse content with template without quotes | ported | [`crates/renovate-core/src/util.rs:8820`](../../../../../crates/renovate-core/src/util.rs#L8820) |
| 353 | should parse content with yaml tags | ported | [`crates/renovate-core/src/util.rs:8841`](../../../../../crates/renovate-core/src/util.rs#L8841) |

