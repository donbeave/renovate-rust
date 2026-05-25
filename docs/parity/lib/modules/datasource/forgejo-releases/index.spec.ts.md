# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/forgejo-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/forgejo-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** done

### `modules/datasource/forgejo-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from forgejo.com | 9 | ported | `crates/renovate-core/src/datasources/forgejo_releases.rs` | `returns_tags_from_forgejo_com` | fetch_releases with default forgejo registry |
| returns tags from codeberg.org | 106 | ported | `crates/renovate-core/src/datasources/forgejo_releases.rs` | `returns_tags_from_codeberg_org` | fetch_releases with custom registry URL |
| returns commits from codeberg.org | 236 | ported | `crates/renovate-core/src/datasources/forgejo_releases.rs` | `returns_commits_from_codeberg_org` | get_digest → latest commit SHA |
| returns commits from forgejo.com | 283 | ported | `crates/renovate-core/src/datasources/forgejo_releases.rs` | `returns_commits_from_forgejo_com_empty` | get_digest → None when empty |
| returns tags commit hash from forgejo.com | 299 | ported | `crates/renovate-core/src/datasources/forgejo_releases.rs` | `returns_tags_commit_hash_from_forgejo_com` | get_digest with newValue → tag commit SHA |

---
