# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/utils/read-only-issue-body.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/utils/read-only-issue-body.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `.readOnlyIssueBody`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes all checkbox formatting | 8 | ported | `platform/pr_body.rs` | `read_only_removes_checkbox_formatting` | — |
| removes all checkbox-related instructions | 14 | ported | `platform/pr_body.rs` | `read_only_removes_checkbox_instructions` | — |
| removes all approval-all-pending-prs | 20 | ported | `platform/pr_body.rs` | `read_only_removes_approve_all_pending_prs` | — |
| removes the create-all-rate-limited-prs | 26 | ported | `platform/pr_body.rs` | `read_only_removes_create_all_rate_limited_prs` | — |
| removes create-config-migration-pr | 33 | ported | `platform/pr_body.rs` | `read_only_removes_create_config_migration_pr` | — |
| removes the create-all-awaiting-schedule-prs | 40 | ported | `platform/pr_body.rs` | `read_only_removes_create_all_awaiting_schedule_prs` | — |

---

