# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/string.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/string.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `util/string › replaceAt`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaceAt inserts newString which is one char longer than oldString | 11 | ported | `util.rs` | `test_replace_at_longer` | — |
| replaceAt inserts newString which is significantly longer than oldString | 22 | ported | `util.rs` | `test_replace_at_much_longer` | — |

### `util/string › looseEquals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reverts to literal match if either is falsey | 35 | ported | `util.rs` | `test_loose_equals_falsey` | — |

### `util/string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| coerceString | 42 | ported | `util.rs` | `test_coerce_string` | — |

### `util/string › stripTemplates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| "$input" -> "$expected" | 51 | ported | `util.rs` | `test_strip_templates` | — |

### `util/string › capitalize`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| capitalizes | 81 | ported | `util.rs` | `test_capitalize` | — |

---

