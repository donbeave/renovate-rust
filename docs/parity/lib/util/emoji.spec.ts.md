# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/emoji.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/emoji.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** pending-applicable

### `util/emoji › emojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes known shortcodes | 53 | pending | — | — | — |
| encodes aliases | 57 | pending | — | — | — |
| omits unknown shortcodes | 63 | pending | — | — | — |
| convert warning shortcode to emoji | 67 | pending | — | — | — |
| does not encode when config option is disabled | 72 | pending | — | — | — |

### `util/emoji › unemojify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips emojis when the config has been set accordingly | 79 | pending | — | — | — |
| does not strip emojis when the config demands it | 88 | pending | — | — | — |
| converts warning emoji to shortcode | 97 | pending | — | — | — |

### `util/emoji › problematic characters`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts %s forth and back | 106 | pending | — | — | — |

### `util/emoji › stripEmojis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is independent of config option | 124 | pending | — | — | — |
| does not throw on standalone modifiers | 135 | pending | — | — | — |

---

