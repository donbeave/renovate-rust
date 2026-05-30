# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/puppet/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/puppet/common.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `RE_REPOSITORY_GENERIC_GIT_SSH_FORMAT`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| access by index | 8 | ported | `puppet.rs` | `puppet_git_ssh_regex_captures_repository` | — |
| access by named group | 18 | ported | `puppet.rs` | `puppet_git_ssh_regex_captures_named_group` | — |

### `parseGitOwnerRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unable to parse url | 32 | ported | `puppet.rs` | `puppet_parse_git_owner_repo_returns_none_for_invalid` | — |
| parseable url | 36 | ported | `puppet.rs` | `puppet_parse_git_owner_repo_parses_https_url` | — |

---

