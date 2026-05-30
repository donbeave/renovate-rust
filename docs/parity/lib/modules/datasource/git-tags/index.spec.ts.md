# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/git-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/git-tags/index.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** done

### `modules/datasource/git-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns nil if response is wrong | 38 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_nil_if_response_is_wrong` | empty string → None |
| returns nil if remote call throws exception | 45 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_nil_if_remote_call_throws_exception` | None → None |
| returns versions filtered from tags | 52 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_versions_filtered_from_tags` | only tags type; 6 releases with deref hashes |
| returns null if not found | 64 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_null_if_not_found` | notfound → None |
| returns digest for tag | 74 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_digest_for_tag` | v1.0.2 → deref hash |
| returns digest for HEAD | 84 | ported | `crates/renovate-core/src/datasources/git_tags.rs` | `returns_digest_for_head` | None → HEAD hash |
| returns digest for HEAD with authentication environment variables | 94 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_uses_github_api_rule_for_github_dot_com` | Host-rule token builds Git `insteadOf` env |
| returns digest for HEAD with authentication environment variables for datasource type git-tags | 121 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_allows_git_tags_datasource_host_type` | Additional datasource host type accepted |

---
