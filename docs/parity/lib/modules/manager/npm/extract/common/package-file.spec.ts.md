# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/common/package-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/common/package-file.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for a valid packageManager with name@version(e.g. pnpm@8.15.4) | 20 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns true for a valid range like npm@^9 | 31 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns true for yarn classic pin yarn@1.22.19 | 38 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns false when packageManager does not contain '@' (e.g. 'npm') | 45 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns false when packageManager is missing | 52 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns false when package.json is invalid | 57 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |
| returns false if packageManager is an empty string | 62 | not-applicable | — | — | tests npm package.json extraction utilities; Rust npm extractor uses own extraction approach |

---

