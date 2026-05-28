# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/env.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `util/exec/env`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default environment variables | 35 | ported | `util.rs` | `test_get_child_process_env_defaults` | — |
| returns environment variable only if defined | 57 | ported | `util.rs` | `test_get_child_process_env_only_defined` | — |
| returns custom environment variables if passed and defined | 62 | ported | `util.rs` | `test_get_child_process_env_custom_vars` | — |

### `util/exec/env › getChildProcessEnv when trustlevel set to high`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns process.env if trustlevel set to high | 79 | ported | `util.rs` | `test_get_child_process_env_expose_all` | — |

---
