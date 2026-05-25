# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/forgejo-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/forgejo-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** done

### `modules/datasource/forgejo-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from code.forgejo.org | 9 | ported | `crates/renovate-core/src/datasources/forgejo_tags.rs` | `returns_tags_from_forgejo_org` | fetch_releases with forgejo default registry |
| returns tags from codeberg.org | 129 | ported | `crates/renovate-core/src/datasources/forgejo_tags.rs` | `returns_tags_from_codeberg_org` | fetch_releases with custom registry URL |
| returns commits from codeberg.org | 214 | ported | `crates/renovate-core/src/datasources/forgejo_tags.rs` | `returns_commits_from_codeberg_org` | get_digest → latest commit SHA |
| returns null from code.forgejo.org when no commits found | 261 | ported | `crates/renovate-core/src/datasources/forgejo_tags.rs` | `returns_null_when_no_commits` | get_digest → None when empty |
| returns tags commit hash from code.forgejo.org | 277 | ported | `crates/renovate-core/src/datasources/forgejo_tags.rs` | `returns_tags_commit_hash` | get_digest with newValue → tag commit SHA |

---
