# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/auth.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/auth.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `util/http/auth › applyAuthorization`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing | 6 | ported | `http.rs` | `auth_does_nothing_with_existing_header` | — |
| gitea password | 24 | ported | `http.rs` | `auth_gitea_password_basic` | — |
| gittea token | 44 | ported | `http.rs` | `auth_gitea_token_bearer` | — |
| github token | 64 | ported | `http.rs` | `auth_github_token_prefix` | — |
| github token for datasource using github api | 82 | ported | `http.rs` | `auth_github_releases_token_prefix` | — |
| github app token with hostType not in GITHUB_API_USING_HOST_TYPES | 101 | ported | `http.rs` | `auth_github_app_token_bearer` | — |
| gitlab personal access token | 115 | ported | `http.rs` | `auth_gitlab_personal_access_token` | — |
| gitlab oauth token | 136 | ported | `http.rs` | `auth_gitlab_oauth_token_bearer` | — |
| npm basic token | 157 | ported | `http.rs` | `auth_npm_basic_auth_type` | — |
| bare token | 181 | ported | `http.rs` | `auth_token_only_auth_type` | — |
| honors authType | 203 | ported | `http.rs` | `auth_honors_auth_type` | — |

---

