# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitea-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitea-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/datasource/gitea-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from gitea.com | 9 | ported | `crates/renovate-core/src/datasources/gitea_releases.rs` | `returns_tags_from_gitea_com` | fetch_releases with default gitea.com registry |
| returns tags from codeberg.org | 100 | ported | `crates/renovate-core/src/datasources/gitea_releases.rs` | `returns_tags_from_codeberg_org` | fetch_releases with custom registry URL |
| returns commits from codeberg.org | 230 | ported | `crates/renovate-core/src/datasources/gitea_releases.rs` | `returns_commits_from_codeberg_org` | get_digest → latest commit SHA |
| returns commits from gitea.com | 277 | ported | `crates/renovate-core/src/datasources/gitea_releases.rs` | `returns_commits_from_gitea_com_empty` | get_digest → None when empty |
| returns tags commit hash from gitea.com | 293 | ported | `crates/renovate-core/src/datasources/gitea_releases.rs` | `returns_tags_commit_hash_from_gitea_com` | get_digest with newValue → tag commit SHA |

---
