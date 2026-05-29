# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/emoji.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/emoji.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `util/emoji › emojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes known shortcodes | 53 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| encodes aliases | 57 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| omits unknown shortcodes | 63 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| convert warning shortcode to emoji | 67 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| does not encode when config option is disabled | 72 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|

### `util/emoji › unemojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips emojis when the config has been set accordingly | 79 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| does not strip emojis when the config demands it | 88 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| converts warning emoji to shortcode | 97 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|

### `util/emoji › problematic characters`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts %s forth and back | 106 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|

### `util/emoji › stripEmojis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is independent of config option | 124 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|
| does not throw on standalone modifiers | 135 | not-applicable | — | — | TS-library-specific; tests emojibase npm package for emoji shortcode conversion; Rust would use different emoji data|

---

