# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-tags/index.spec.ts
**Total tests:** 12 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `modules/datasource/github-tags/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commit digest | 25 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_commit_digest` | GET /commits?per_page=1 → first sha |
| returns null for missing commit | 36 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_null_for_missing_commit` | empty commits list → None |
| returns untagged commit digest | 45 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_untagged_commit_digest` | GET /commits → sha |
| returns tagged commit digest | 54 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_tagged_commit_digest` | GET /tags → find tag → sha |
| returns null for missing hash | 73 | not-applicable | — | — | TypeScript test uses vi.spyOn(githubGraphql, 'queryTags') — GraphQL path specific; Rust implementation uses REST /tags which always includes commit.sha |
| returns null for missing tagged commit digest | 91 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_null_for_missing_tagged_commit_digest` | tag not found → None |
| returns null for error | 110 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_null_for_error` | 500 → None |

### `modules/datasource/github-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags | 120 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `returns_tags` | tags + commits + releases merged; isStable from !prerelease |
| if it is newer than tag timestamp | 183 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `release_timestamp_newer_than_tag_timestamp` | release published_at strictly newer → use release timestamp |
| keeps tag timestamp when release timestamp is older | 212 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `keeps_tag_timestamp_when_release_timestamp_is_older` | older release published_at → keep commit timestamp |
| keeps tag timestamp when release timestamp is equal | 241 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `keeps_tag_timestamp_when_release_timestamp_is_equal` | equal timestamps → keep commit timestamp |
| keeps tag timestamp when no corresponding release exists | 270 | ported | `crates/renovate-core/src/datasources/github_tags.rs` | `keeps_tag_timestamp_when_no_corresponding_release_exists` | no release entry → keep commit timestamp |

---
