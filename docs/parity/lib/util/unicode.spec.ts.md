# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/unicode.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/unicode.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** partial

### `util/unicode › logWarningIfUnicodeHiddenCharactersInPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning for hidden Unicode characters in text files | 146 | not-applicable | — | — | Tests logWarningIfUnicodeHiddenCharacters which uses logger spy; not portable without tracing test infrastructure |
| logs a trace message for BOM character only | 156 | not-applicable | — | — | Tests logWarningIfUnicodeHiddenCharacters which uses logger spy; not portable without tracing test infrastructure |
| does not log a warning for binary files with null bytes but no hidden unicode | 170 | not-applicable | — | — | Tests logWarningIfUnicodeHiddenCharacters which uses logger spy; not portable without tracing test infrastructure |
| logs a trace message (not warning) for binary files with hidden unicode characters | 183 | not-applicable | — | — | Tests logWarningIfUnicodeHiddenCharacters which uses logger spy; not portable without tracing test infrastructure |
| does not log a warning when no hidden characters are present | 203 | not-applicable | — | — | Tests logWarningIfUnicodeHiddenCharacters which uses logger spy; not portable without tracing test infrastructure |

---

