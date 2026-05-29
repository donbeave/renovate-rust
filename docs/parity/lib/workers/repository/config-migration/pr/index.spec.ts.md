# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/pr/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `workers/repository/config-migration/pr/index › ensureConfigMigrationPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates PR | 52 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR with default PR title | 59 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| Founds an open PR and as it is up to date and returns | 66 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| Founds an open PR and updates it | 76 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| updates an open PR with unexpected PR title | 85 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| Dry runs and does not update out of date PR | 96 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| Creates PR in dry run mode | 116 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR with labels | 128 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR with empty footer and header | 144 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR for JSON5 config file | 157 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR with footer and header with trailing and leading newlines | 167 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates non-semantic PR title | 181 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates semantic PR title | 197 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| creates PR with footer and header using templating | 215 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|

### `workers/repository/config-migration/pr/index › ensureConfigMigrationPr() throws`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when trying to create a new PR | 250 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|
| deletes branch when PR already exists but cannot find it | 256 | not-applicable | — | — | mocking framework internals — platform/git mock utilities; TypeScript config migration PR creation|

---

