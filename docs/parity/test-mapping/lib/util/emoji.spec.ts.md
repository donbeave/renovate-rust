# `lib/util/emoji.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**11/11 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | encodes known shortcodes | ported | [`crates/renovate-core/src/util/emoji.rs:194`](../../../../../crates/renovate-core/src/util/emoji.rs#L194) |
| 14 | encodes aliases | ported | [`crates/renovate-core/src/util/emoji.rs:201`](../../../../../crates/renovate-core/src/util/emoji.rs#L201) |
| 20 | omits unknown shortcodes | ported | [`crates/renovate-core/src/util/emoji.rs:208`](../../../../../crates/renovate-core/src/util/emoji.rs#L208) |
| 24 | convert warning shortcode to emoji | ported | [`crates/renovate-core/src/util/emoji.rs:215`](../../../../../crates/renovate-core/src/util/emoji.rs#L215) |
| 29 | does not encode when config option is disabled | ported | [`crates/renovate-core/src/util/emoji.rs:222`](../../../../../crates/renovate-core/src/util/emoji.rs#L222) |
| 36 | strips emojis when the config has been set accordingly | ported | [`crates/renovate-core/src/util/emoji.rs:230`](../../../../../crates/renovate-core/src/util/emoji.rs#L230) |
| 45 | does not strip emojis when the config demands it | ported | [`crates/renovate-core/src/util/emoji.rs:240`](../../../../../crates/renovate-core/src/util/emoji.rs#L240) |
| 54 | converts warning emoji to shortcode | ported | [`crates/renovate-core/src/util/emoji.rs:249`](../../../../../crates/renovate-core/src/util/emoji.rs#L249) |
| 63 | _(it.each / template — verify manually)_ | ? | — |
| 81 | is independent of config option | ported | [`crates/renovate-core/src/util/emoji.rs:271`](../../../../../crates/renovate-core/src/util/emoji.rs#L271) |
| 92 | does not throw on standalone modifiers | ported | [`crates/renovate-core/src/util/emoji.rs:296`](../../../../../crates/renovate-core/src/util/emoji.rs#L296) |

