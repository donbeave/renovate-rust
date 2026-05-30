# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/updates/branchify.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/updates/branchify.spec.ts
**Total tests:** 6 | **Ported:** 5 | **Actionable:** 1 | **Status:** partial

### `workers/repository/updates/branchify › branchifyUpgrades()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 24 | ported | `crates/renovate-cli/src/report_builders.rs` | `collect_branch_updates_empty` | Core grouping logic ported |
| returns one branch if one input | 30 | ported | `crates/renovate-cli/src/report_builders.rs` | `collect_branch_updates_single` | Core grouping logic ported |
| deduplicates | 48 | ported | `crates/renovate-cli/src/report_builders.rs` | `collect_branch_updates_deduplicates_same_branch` | Core grouping logic ported |
| groups if same compiled branch names | 76 | ported | `crates/renovate-cli/src/report_builders.rs` | `collect_branch_updates_groups_by_branch_name` | Core grouping logic ported |
| groups if same compiled group name | 103 | ported | `crates/renovate-cli/src/report_builders.rs` | `collect_branch_updates_groups_by_branch_name` | groupName → branch_name is handled by `pipeline_utils::apply_update_blocking_to_report` |
| no fetch changelogs  | 134 | pending | — | — | Changelog fetching is a separate workers layer not yet in scope for branchify parity |

---

