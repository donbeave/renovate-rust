# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/auth.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/auth.spec.ts
**Total tests:** 30 | **Ported:** 8 | **Actionable:** 30 | **Status:** partial

### `util/git/auth › getGitAuthenticatedEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns url with token | 13 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token` | `GIT_CONFIG_*` insteadOf triplet |
| returns url with username and password | 31 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_username_and_password` | URL-encoded username/password auth triplet |
| prefers token over username and password | 53 | pending | — | — | — |
| returns url with token for different protocols | 73 | pending | — | — | — |
| returns correct url if token already contains GitHub App username | 91 | pending | — | — | — |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter | 112 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_honors_existing_count_parameter` | Starts new keys at provided count |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter over environment | 134 | pending | — | — | — |
| returns url with token and already existing GIT_CONFIG_COUNT from environment | 157 | pending | — | — | — |
| returns url with token and passthrough existing variables | 176 | pending | — | — | — |
| return url with token with invalid GIT_CONFIG_COUNT from environment | 199 | pending | — | — | — |
| returns url with token containing username for GitLab token | 218 | pending | — | — | — |
| returns url with token containing username for GitLab token without hostType | 239 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_detects_gitlab_token_without_host_type` | Detects gitlab.com from matchHost |
| returns original environment variables when no token is set | 259 | pending | — | — | — |
| returns url with token for http hosts | 274 | pending | — | — | — |
| returns url with token for orgs | 292 | pending | — | — | — |
| returns url with token for orgs and projects | 310 | pending | — | — | — |
| returns url with token for orgs and projects and ports | 330 | pending | — | — | — |
| returns url with token for bitbucket-server | 354 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_bitbucket_server_urls` | Adds `/scm/` HTTPS rewrite and default SSH port 7999 |

### `util/git/auth › getGitEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty object if no environment variables exist | 381 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_empty_without_host_rules` | Empty host-rule registry |
| returns environment variables with token if hostRule for api.github.com exists | 385 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_uses_github_api_rule_for_github_dot_com` | `api.github.com` token rewrites `github.com` git URLs |
| returns environment variables with token if hostRule for multiple hostsRules | 402 | pending | — | — | — |
| returns environment variables with token if hostRule is for Gitlab | 446 | pending | — | — | — |
| returns environment variables with username and password | 466 | pending | — | — | — |
| returns environment variables with URL encoded username and password | 487 | pending | — | — | — |
| returns no environment variables when hostType is not supported | 508 | pending | — | — | — |
| returns no environment variables when only username is set | 517 | pending | — | — | — |
| returns no environment variables when only password is set | 526 | pending | — | — | — |
| returns environment variables when hostType is explicitly set | 535 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_allows_explicit_datasource_host_type` | Additional host type allow-list |
| returns empty environment variables when matchHost contains invalid protocol | 554 | pending | — | — | — |
| returns environment variables for bitbucket-server | 563 | pending | — | — | — |

---
