# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/hermit/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/hermit/update.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should append a new marking line at the end to trigger the artifact update | 6 | ported | `hermit.rs` | `hermit_update_appends_marker` | — |
| should not update again if the new line has been appended | 19 | ported | `hermit.rs` | `hermit_update_no_op_if_already_marked` | — |

---

