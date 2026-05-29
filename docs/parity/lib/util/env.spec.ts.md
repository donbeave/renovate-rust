# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/env.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `util/env › getEnv`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return combined env | 11 | ported | `util.rs` | `test_get_combined_env_return_combined` | Maps to `get_combined_env` with explicit params instead of module globals |
| maintains precendence | 26 | ported | `util.rs` | `test_get_combined_env_maintains_precedence` | Maps to `get_combined_env` with explicit params |

---
