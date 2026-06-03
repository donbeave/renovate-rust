# `lib/modules/manager/crow/extract.spec.ts`

[← `manager/crow`](../../../../_by-module/manager/crow.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null for empty | ported | [`crates/renovate-core/src/extractors/crow.rs:219`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L219) |
| 10 | returns null for non-object yaml | ported | [`crates/renovate-core/src/extractors/crow.rs:238`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L238) |
| 15 | returns null for malformed yaml | ported | [`crates/renovate-core/src/extractors/crow.rs:259`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L259) |
| 19 | extracts multiple image lines | ported | [`crates/renovate-core/src/extractors/crow.rs:148`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L148) |
| 164 | extracts image and replaces registry | ported | [`crates/renovate-core/src/extractors/crow.rs:276`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L276) |
| 194 | extracts image but no replacement | ported | [`crates/renovate-core/src/extractors/crow.rs:298`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L298) |
| 224 | extracts image and no double replacement | ported | [`crates/renovate-core/src/extractors/crow.rs:321`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L321) |
| 255 | extracts the 1.0.0 version | ported | [`crates/renovate-core/src/extractors/crow.rs:265`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L265) |
| 281 | should parse multiple sources of dependencies together | ported | [`crates/renovate-core/src/extractors/crow.rs:346`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L346) |
| 321 | return dependency when a plugin-git is cloned | ported | [`crates/renovate-core/src/extractors/crow.rs:192`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L192) |
| 348 | return null when no dependencies are provided | ported | [`crates/renovate-core/src/extractors/crow.rs:245`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L245) |
| 362 | handles empty pipeline section gracefully | ported | [`crates/renovate-core/src/extractors/crow.rs:363`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L363) |
| 390 | returns null when pipeline keys exist but contain no valid images | ported | [`crates/renovate-core/src/extractors/crow.rs:252`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L252) |
| 408 | extracts images from array-based steps format | ported | [`crates/renovate-core/src/extractors/crow.rs:178`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L178) |
| 447 | extracts images from mixed array and object formats | ported | [`crates/renovate-core/src/extractors/crow.rs:373`](../../../../../../../crates/renovate-core/src/extractors/crow.rs#L373) |

