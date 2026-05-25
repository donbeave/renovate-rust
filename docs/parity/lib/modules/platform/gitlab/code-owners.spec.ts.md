# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/code-owners.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/code-owners.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `CodeOwnersParser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract an owner rule from a line | 5 | ported | `gitlab.rs` | `code_owners_parses_pattern_with_usernames` | — |
| should extract an owner rule from a line with no usernames | 20 | ported | `gitlab.rs` | `code_owners_parses_pattern_without_usernames` | — |
| should extract an owner rule from a line after a section header | 33 | ported | `gitlab.rs` | `code_owners_section_header_default_users` | — |
| should extract an owner rule from a line after a section header with no usernames | 47 | ported | `gitlab.rs` | `code_owners_section_header_no_users` | — |
| should extract an owner rule from a line after a section header with spaces | 61 | ported | `gitlab.rs` | `code_owners_section_header_with_spaces` | — |
| should extract an owner rule from a line after a section header with spaces and no usernames | 75 | ported | `gitlab.rs` | `code_owners_section_header_with_spaces_no_users` | — |
| should extract an owner rule from a line after a section header with spaces and multiple usernames | 89 | ported | `gitlab.rs` | `code_owners_section_header_multiple_users` | — |
| should extract an owner rule from a line after an optional section header with spaces | 103 | ported | `gitlab.rs` | `code_owners_optional_section_header` | — |
| should extract an owner rule from a line after a section header with approval count and spaces | 117 | ported | `gitlab.rs` | `code_owners_section_header_with_approval_count` | — |

---

