# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/unicode.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/unicode.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `util/unicode › logWarningIfUnicodeHiddenCharactersInPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning for hidden Unicode characters in text files | 146 | ported | `util.rs` | `hidden_unicode_chars_detected_in_text` | — |
| logs a trace message for BOM character only | 156 | ported | `util.rs` | `bom_character_detected` | — |
| does not log a warning for binary files with null bytes but no hidden unicode | 170 | ported | `util.rs` | `binary_content_with_null_bytes_detected` | — |
| logs a trace message (not warning) for binary files with hidden unicode characters | 183 | ported | `util.rs` | `binary_content_with_hidden_unicode_detected` | — |
| does not log a warning when no hidden characters are present | 203 | ported | `util.rs` | `no_hidden_unicode_in_normal_text` | — |

---
