# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/config.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/config.spec.ts
**Total tests:** 3 | **Ported:** 2 | **Actionable:** 3 | **Status:** partial

### `util/git/config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses "close" events, ignores "exit" events from child processes | 9 | ported | `git.rs` | `simple_git_config_defaults` | — |
| uses timeout value from GlobalConfig | 16 | pending | — | — | — |
| throws | 27 | ported | `git.rs` | `set_no_verify_rejects_non_array` | — |

---
