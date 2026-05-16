# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/supersedes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/supersedes.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `workers/repository/extract/supersedes › processSupersedesManagers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty input | 6 | ported | `managers.rs` | `supersedes_handles_empty_input` | — |
| ignores extracts without superseding managers | 12 | ported | `managers.rs` | `supersedes_ignores_non_superseding_managers` | — |
| removes superseded package files without lock files | 28 | ported | `managers.rs` | `supersedes_removes_superseded_files_without_lock` | — |
| keeps superseded package files with lock files | 52 | ported | `managers.rs` | `supersedes_keeps_files_with_lock_files` | — |
| keeps non-superseded package files | 88 | ported | `managers.rs` | `supersedes_keeps_non_superseded_files` | — |
| handles primary extract with undefined packageFiles | 115 | ported | `managers.rs` | `supersedes_handles_primary_with_no_package_files` | None = undefined |
| handles missing secondary extract manager | 137 | ported | `managers.rs` | `supersedes_handles_missing_secondary_manager` | — |
| handles secondary extract with undefined packageFiles | 153 | ported | `managers.rs` | `supersedes_handles_secondary_with_no_package_files` | None = undefined |

---

