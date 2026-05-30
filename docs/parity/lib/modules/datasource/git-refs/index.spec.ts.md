# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/git-refs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/git-refs/index.spec.ts
**Total tests:** 11 | **Ported:** 8 | **Actionable:** 0 | **Status:** done

### `modules/datasource/git-refs/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns nil if response is wrong | 38 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_nil_if_response_is_wrong` | empty string → None (JS falsy) |
| returns nil if response is malformed | 48 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_nil_if_response_is_malformed` | non-empty unparseable → Some{releases:[]} |
| returns nil if remote call throws exception | 58 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_nil_if_remote_call_throws_exception` | None ls_remote → None |
| returns versions filtered from tags | 68 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_versions_filtered_from_tags` | raw datasource includes tags+heads; TS count of 6 is after getPkgReleases version filtering |
| returns null if not found | 82 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_null_if_not_found` | v2.0.0 absent → None |
| returns digest for tag | 92 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_digest_for_tag` | annotated tag → ^{} dereferenced hash |
| ignores refs/for/ | 104 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `ignores_refs_for` | master returns heads hash not for hash |
| returns digest for HEAD | 114 | ported | `crates/renovate-core/src/datasources/git_refs.rs` | `returns_digest_for_head` | None newValue → HEAD hash |
| calls simpleGit with emptyEnv if no hostrules exist | 124 | not-applicable | — | — | Rust datasource receives `ls_remote` as a string; git subprocess execution and env vars are handled by the caller layer, tested in `util.rs` |
| calls simpleGit with git envs if hostrules exist | 135 | not-applicable | — | — | Same reason as line 124 |
| calls simpleGit with git envs if hostrules exist for datasource type git-refs | 162 | not-applicable | — | — | Same reason as line 124 |

---
