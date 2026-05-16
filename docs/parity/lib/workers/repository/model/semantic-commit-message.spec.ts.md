# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/model/semantic-commit-message.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/model/semantic-commit-message.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `workers/repository/model/semantic-commit-message`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should format message without prefix | 4 | ported | `branch.rs` | `semantic_commit_no_type_capitalizes` | — |
| should format sematic type | 11 | ported | `branch.rs` | `semantic_commit_type_only` | — |
| should format sematic prefix with scope | 19 | ported | `branch.rs` | `semantic_commit_type_and_scope` | — |
| should transform to lowercase only first letter | 28 | ported | `branch.rs` | `semantic_commit_lowercase_first_letter_only` | — |
| should create instance from string without scope | 37 | ported | `branch.rs` | `parse_semantic_commit_without_scope` | — |
| should create instance from string with scope | 50 | ported | `branch.rs` | `parse_semantic_commit_with_scope` | — |
| should create instance from string with empty description | 65 | ported | `branch.rs` | `parse_semantic_commit_empty_description` | — |
| should return undefined for invalid string | 78 | ported | `branch.rs` | `parse_semantic_commit_invalid_returns_none` | — |

---

