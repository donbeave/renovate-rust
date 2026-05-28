# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/pretty-stdout.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/pretty-stdout.spec.ts
**Total tests:** 15 | **Ported:** 4 | **Actionable:** 15 | **Status:** partial

### `logger/pretty-stdout › getMeta(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 9 | ported | `util.rs` | `test_get_meta_null_rec` | — |
| returns empty string if empty rec | 13 | ported | `util.rs` | `test_get_meta_empty_rec` | — |
| returns empty string if no meta fields | 17 | ported | `util.rs` | `test_get_meta_no_meta_fields` | — |
| supports single meta | 24 | pending | — | — | — |
| supports multi meta | 34 | pending | — | — | — |
| returns plain text when colorize is false | 46 | ported | `util.rs` | `test_get_meta_plain_text` | — |

### `logger/pretty-stdout › getDetails(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 57 | pending | — | — | — |
| returns empty string if empty rec | 61 | pending | — | — | — |
| returns empty string if all are meta fields | 67 | pending | — | — | — |
| supports a config | 75 | pending | — | — | — |
| formats err.stack as readable multi-line output | 88 | pending | — | — | — |
| formats err.stack without other err fields | 108 | pending | — | — | — |

### `logger/pretty-stdout › formatRecord(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| formats record | 136 | pending | — | — | — |
| formats record without colors | 155 | pending | — | — | — |

### `logger/pretty-stdout › PrettyStdoutStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes formatted data to stdout | 175 | pending | — | — | — |

---
