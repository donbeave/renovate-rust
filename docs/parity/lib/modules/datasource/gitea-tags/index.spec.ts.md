# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitea-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitea-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/datasource/gitea-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from gitea.com | 9 | ported | `crates/renovate-core/src/datasources/gitea_tags.rs` | `returns_tags_from_gitea_com` | fetch_releases maps name/commit.sha/commit.created |
| returns tags from codeberg.org | 124 | ported | `crates/renovate-core/src/datasources/gitea_tags.rs` | `returns_tags_from_codeberg_org` | fetch_releases with custom registry URL |
| returns commits from codeberg.org | 209 | ported | `crates/renovate-core/src/datasources/gitea_tags.rs` | `returns_commits_from_codeberg_org` | get_digest → latest commit SHA |
| returns commits from gitea.com | 256 | ported | `crates/renovate-core/src/datasources/gitea_tags.rs` | `returns_commits_from_gitea_com_empty` | get_digest → None when empty |
| returns tags commit hash from gitea.com | 272 | ported | `crates/renovate-core/src/datasources/gitea_tags.rs` | `returns_tags_commit_hash_from_gitea_com` | get_digest with newValue → tag commit SHA |

---
