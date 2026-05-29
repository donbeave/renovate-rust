# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bitbucket-server-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-server-tags/index.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `modules/datasource/bitbucket-server-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags | 12 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `returns_tags` | 3 tags; hash/null/missing |
| returns null on empty result | 66 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `returns_null_on_empty_result` | `{}` → None |
| returns null on missing registryUrl | 80 | ported | `bitbucket_server_tags.rs` | `returns_null_on_missing_registry_url` | — |
| handles not found | 88 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `handles_not_found` | 404 → None |
| returns commit hash of provided tag | 104 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `returns_commit_hash_of_provided_tag` | tag hash → Some(hash) |
| missing hash | 124 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `missing_hash` | null hash → None |
| returns most recent commit hash | 146 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `returns_most_recent_commit_hash` | commits[0].id |
| no commits | 173 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `no_commits` | empty values → None |
| returns null on empty result | 195 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `get_digest_returns_null_on_empty_result` | `{}` → None |
| returns null on missing registryUrl | 211 | ported | `bitbucket_server_tags.rs` | `get_digest_returns_null_on_missing_registry_url` | — |
| handles not found | 219 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `get_digest_handles_not_found` | 404 → Err |

---
