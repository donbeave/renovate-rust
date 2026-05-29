# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/github/graphql/query-adapters/releases-query-adapter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/graphql/query-adapters/releases-query-adapter.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/github/graphql/query-adapters/releases-query-adapter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms items | 17 | ported | `util.rs` | `test_transform_github_release_basic` | — |
| filters out drafts | 28 | ported | `util.rs` | `test_transform_github_release_draft_filtered` | — |
| handles invalid items | 32 | ported | `util.rs` | `test_transform_github_release_invalid_returns_none` | — |
| marks prereleases as unstable | 36 | ported | `util.rs` | `test_transform_github_release_prerelease_unstable` | — |

---
