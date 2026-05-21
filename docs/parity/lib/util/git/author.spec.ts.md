# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/author.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/author.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `util/git/author › parseGitAuthor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if empty email given | 8 | ported | `git/author.rs` | `parse_git_author_returns_none_for_empty` | — |
| catches errors | 12 | ported | `git/author.rs` | `parse_git_author_returns_none_for_unparseable` | — |
| handles a normal address | 19 | ported | `git/author.rs` | `parse_git_author_handles_normal_address` | — |
| parses bot email | 23 | ported | `git/author.rs` | `parse_git_author_parses_bot_email` | — |
| parses bot name and email | 30 | ported | `git/author.rs` | `parse_git_author_parses_bot_name_and_email` | — |
| escapes names | 41 | ported | `git/author.rs` | `parse_git_author_handles_name_with_brackets` | — |
| tries again and fails | 47 | ported | `git/author.rs` | `parse_git_author_returns_none_for_invalid_email_in_brackets` | — |
| gives up | 51 | ported | `git/author.rs` | `parse_git_author_gives_up_on_non_email` | — |

---

