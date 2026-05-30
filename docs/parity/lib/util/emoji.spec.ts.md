# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/emoji.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/emoji.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** done

### `util/emoji › emojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes known shortcodes | 53 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| encodes aliases | 57 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| omits unknown shortcodes | 63 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| convert warning shortcode to emoji | 67 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| does not encode when config option is disabled | 72 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |

### `util/emoji › unemojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips emojis when the config has been set accordingly | 79 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| does not strip emojis when the config demands it | 88 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| converts warning emoji to shortcode | 97 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |

### `util/emoji › problematic characters`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts %s forth and back | 106 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |

### `util/emoji › stripEmojis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is independent of config option | 124 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |
| does not throw on standalone modifiers | 135 | not-applicable | — | — |Tests emojibase npm package (emoji shortcode library); TS-specific library integration |

---

