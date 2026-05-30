# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/utils/pr-body.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/utils/pr-body.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 0 | **Status:** done

### `.smartTruncate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| truncates to 1000 | 9 | ported | `platform/pr_body.rs` | `smart_truncate_to_1000` | — |
| truncates to 300 not smart | 18 | ported | `platform/pr_body.rs` | `smart_truncate_to_300_not_smart` | — |
| includes truncation notice at end of truncated content (when "not smart") | 27 | ported | `platform/pr_body.rs` | `smart_truncate_notice_at_end_not_smart` | — |
| includes truncation notice before Configuration section (when "smart") | 33 | ported | `platform/pr_body.rs` | `smart_truncate_notice_before_configuration_smart` | — |
| truncates content without release notes structure when notice fits | 43 | ported | `platform/pr_body.rs` | `smart_truncate_no_release_notes_structure_notice_fits` | — |
| truncates to below notice length with release notes structure | 49 | ported | `platform/pr_body.rs` | `smart_truncate_below_notice_length` | — |
| truncates to 10 | 55 | ported | `platform/pr_body.rs` | `smart_truncate_to_10` | — |
| does not truncate | 63 | ported | `platform/pr_body.rs` | `smart_truncate_no_truncation` | — |

---

