# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/emoji.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/emoji.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/emoji › emojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes known shortcodes | 53 | not-applicable | — | — | Renovate's emoji shortcode conversion helper is not implemented as a Rust API. |
| encodes aliases | 57 | not-applicable | — | — | Renovate's emoji shortcode alias table is not implemented as a Rust API. |
| omits unknown shortcodes | 63 | not-applicable | — | — | Renovate's emoji shortcode conversion helper is not implemented as a Rust API. |
| convert warning shortcode to emoji | 67 | not-applicable | — | — | Renovate's emoji shortcode conversion helper is not implemented as a Rust API. |
| does not encode when config option is disabled | 72 | not-applicable | — | — | Renovate's process-global emoji config helper is not implemented as a Rust API. |

### `util/emoji › unemojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips emojis when the config has been set accordingly | 79 | not-applicable | — | — | Renovate's emoji stripping/config helper is not implemented as a Rust API. |
| does not strip emojis when the config demands it | 88 | not-applicable | — | — | Renovate's process-global emoji config helper is not implemented as a Rust API. |
| converts warning emoji to shortcode | 97 | not-applicable | — | — | Renovate's emoji-to-shortcode conversion helper is not implemented as a Rust API. |

### `util/emoji › problematic characters`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts %s forth and back | 106 | not-applicable | — | — | Renovate's emoji round-trip conversion helper is not implemented as a Rust API. |

### `util/emoji › stripEmojis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is independent of config option | 124 | not-applicable | — | — | Renovate's emoji stripping helper is not implemented as a Rust API. |
| does not throw on standalone modifiers | 135 | not-applicable | — | — | Renovate's emoji stripping helper is not implemented as a Rust API. |

---

