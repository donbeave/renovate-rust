# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bitbucket-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** done

### `modules/datasource/bitbucket-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from bitbucket cloud | 9 | ported | `crates/renovate-core/src/datasources/bitbucket_tags.rs` | `returns_tags_from_bitbucket_cloud` | Deserialization + timestamp conversion with optional target fields |
| returns commits from bitbucket cloud | 43 | ported | `crates/renovate-core/src/datasources/bitbucket_tags.rs` | `returns_commits_from_bitbucket_cloud` | get_digest via mainbranch lookup then commits endpoint |
| returns commits from bitbucket cloud | 85 | ported | `crates/renovate-core/src/datasources/bitbucket_tags.rs` | `returns_null_when_no_commits` | get_digest → None when commits list empty |
| returns tags commit hash from bitbucket cloud | 112 | ported | `crates/renovate-core/src/datasources/bitbucket_tags.rs` | `returns_tags_commit_hash` | get_digest with newValue → target.hash |
| returns null for missing hash | 136 | ported | `crates/renovate-core/src/datasources/bitbucket_tags.rs` | `returns_null_for_missing_hash` | Tag with no target → None |

---
