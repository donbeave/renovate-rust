# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/config.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/config.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses "close" events, ignores "exit" events from child processes | 9 | not-applicable | — | — | Tests `simple-git` npm library config (`completion` options); Rust uses git2/subprocess |
| uses timeout value from GlobalConfig | 16 | not-applicable | — | — | Tests `simple-git` timeout config; Rust uses git2/subprocess |
| throws | 27 | not-applicable | — | — | Tests `setNoVerify` for `simple-git` string validation; no equivalent in Rust |

---

