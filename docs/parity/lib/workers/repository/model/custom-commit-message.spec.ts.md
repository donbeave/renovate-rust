# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/model/custom-commit-message.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/model/custom-commit-message.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `workers/repository/model/custom-commit-message › CustomCommitMessage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| given subject $subject and prefix $prefix as arguments, returns $result | 5 | ported | `branch.rs` | `custom_commit_message_formats_correctly` | — |
| should provide ability to set body and footer | 31 | ported | `branch.rs` | `custom_commit_message_body_footer` | Tests title formatting; body/footer not a Rust concern |
| should remove empty subject by default | 46 | ported | `branch.rs` | `custom_commit_message_empty_subject` | — |

---

