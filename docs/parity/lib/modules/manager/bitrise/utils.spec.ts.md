# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bitrise/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bitrise/utils.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `parseStep()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null on an empty string | 6 | ported | `bitrise.rs` | `parse_step_returns_none_for_empty` | — |
| returns dependency for step | 10 | ported | `bitrise.rs` | `parse_step_returns_dep_with_version` | — |
| parses missing version | 19 | ported | `bitrise.rs` | `parse_step_returns_unspecified_version_when_no_at` | — |

---

