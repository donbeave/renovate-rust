# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitlab-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-releases/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `modules/datasource/gitlab-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases from custom registry | 18 | ported | `crates/renovate-core/src/datasources/gitlab_releases.rs` | `parse_releases_from_api_response` | Parses tag_name/released_at JSON array; URL-encodes project path |
| returns releases from default registry | 32 | ported | `crates/renovate-core/src/datasources/gitlab_releases.rs` | `parse_releases_from_api_response` | Same logic with https://gitlab.com default |
| return null if not found | 45 | ported | `crates/renovate-core/src/datasources/gitlab_releases.rs` | `client_errors_return_none` | 404 client error → Ok(None) via is_fatal_status() |

---

