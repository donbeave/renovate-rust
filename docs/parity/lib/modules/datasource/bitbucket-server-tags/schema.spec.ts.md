# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bitbucket-server-tags/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-server-tags/schema.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** done

### `modules/datasource/bitbucket-server-tags/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses BitbucketServerTags | 4 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `parses_bitbucket_server_tags` | displayId + null/missing hash |
| parses BitbucketServerCommits | 39 | ported | `crates/renovate-core/src/datasources/bitbucket_server_tags.rs` | `parses_bitbucket_server_commits` | id field |

---
