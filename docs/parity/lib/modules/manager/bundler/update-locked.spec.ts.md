# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/update-locked.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/update-locked.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects already updated | 9 | ported | `bundler.rs` | `bundler_update_locked_detects_already_updated` | — |
| returns unsupported for empty lockfile | 21 | ported | `bundler.rs` | `bundler_update_locked_unsupported_for_no_content` | — |
| returns unsupported for empty depName | 32 | ported | `bundler.rs` | `bundler_update_locked_unsupported_for_no_dep_name` | — |
| returns unsupported | 44 | ported | `bundler.rs` | `bundler_update_locked_unsupported_version_not_in_lock` | — |
| returns update-failed in case of errors | 56 | ported | `bundler.rs` | `bundler_update_locked_update_failed_on_invalid_lock` | — |

---

