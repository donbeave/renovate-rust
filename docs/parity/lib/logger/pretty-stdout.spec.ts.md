# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/pretty-stdout.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/pretty-stdout.spec.ts
**Total tests:** 15 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `logger/pretty-stdout › getMeta(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 9 | ported | `util.rs` | `test_get_meta_null_rec` | — |
| returns empty string if empty rec | 13 | ported | `util.rs` | `test_get_meta_empty_rec` | — |
| returns empty string if no meta fields | 17 | ported | `util.rs` | `test_get_meta_no_meta_fields` | — |
| supports single meta | 24 | ported | `util.rs` | `test_get_meta_single_meta` | — |
| supports multi meta | 34 | ported | `util.rs` | `test_get_meta_multi_meta` | — |
| returns plain text when colorize is false | 46 | ported | `util.rs` | `test_get_meta_plain_text` | — |

### `logger/pretty-stdout › getDetails(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 57 | ported | `util.rs` | `test_get_details_null_rec` | — |
| returns empty string if empty rec | 61 | ported | `util.rs` | `test_get_details_empty_rec` | — |
| returns empty string if all are meta fields | 67 | ported | `util.rs` | `test_get_details_all_meta_fields` | — |
| supports a config | 75 | ported | `util.rs` | `test_get_details_config` | — |
| formats err.stack as readable multi-line output | 88 | ported | `util.rs` | `test_get_details_err_with_stack` | — |
| formats err.stack without other err fields | 108 | ported | `util.rs` | `test_get_details_err_stack_only` | — |

### `logger/pretty-stdout › formatRecord(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| formats record | 136 | ported | `util.rs` | `test_format_record` | — |
| formats record without colors | 155 | ported | `util.rs` | `test_format_record_no_colors` | — |

### `logger/pretty-stdout › PrettyStdoutStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes formatted data to stdout | 175 | not-applicable | — | — | mocking framework internals — tests Node.js stream write/chunk mechanics via vi.spyOn(process.stdout.write); behavior covered by format_record tests |

---
