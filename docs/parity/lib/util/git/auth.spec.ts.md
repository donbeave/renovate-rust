# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/auth.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/auth.spec.ts
**Total tests:** 30 | **Ported:** 30 | **Actionable:** 0 | **Status:** done

### `util/git/auth › getGitAuthenticatedEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns url with token | 13 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token` | `GIT_CONFIG_*` insteadOf triplet |
| returns url with username and password | 31 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_username_and_password` | URL-encoded username/password auth triplet |
| prefers token over username and password | 53 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_prefers_token_over_username_and_password` | Token auth wins over username/password |
| returns url with token for different protocols | 73 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token_for_different_protocols` | Non-HTTP git URL coerced to HTTPS auth URL |
| returns correct url if token already contains GitHub App username | 91 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_keeps_github_app_username` | Preserves `x-access-token:` token username |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter | 112 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_honors_existing_count_parameter` | Starts new keys at provided count |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter over environment | 134 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_prefers_parameter_count_over_process_env` | Existing env map overrides process env count |
| returns url with token and already existing GIT_CONFIG_COUNT from environment | 157 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_uses_process_env_count` | Process env count used when no explicit env map count |
| returns url with token and passthrough existing variables | 176 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_passthrough_existing_variables` | Existing env entries preserved |
| return url with token with invalid GIT_CONFIG_COUNT from environment | 199 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_ignores_invalid_process_env_count` | Invalid count falls back to zero |
| returns url with token containing username for GitLab token | 218 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_uses_gitlab_token_username` | Adds `gitlab-ci-token:` username |
| returns url with token containing username for GitLab token without hostType | 239 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_detects_gitlab_token_without_host_type` | Detects gitlab.com from matchHost |
| returns original environment variables when no token is set | 259 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_original_env_without_credentials` | No credentials returns original env map |
| returns url with token for http hosts | 274 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token_for_http_hosts` | Preserves HTTP protocol |
| returns url with token for orgs | 292 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token_for_orgs` | Org path auth rewrite |
| returns url with token for orgs and projects | 310 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token_for_orgs_and_projects` | Org/repo path auth rewrite |
| returns url with token for orgs and projects and ports | 330 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_url_with_token_for_orgs_projects_and_ports` | Port and `.git` suffix auth rewrite |
| returns url with token for bitbucket-server | 354 | ported | `crates/renovate-core/src/util.rs` | `git_authenticated_env_returns_bitbucket_server_urls` | Adds `/scm/` HTTPS rewrite and default SSH port 7999 |

### `util/git/auth › getGitEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty object if no environment variables exist | 381 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_empty_without_host_rules` | Empty host-rule registry |
| returns environment variables with token if hostRule for api.github.com exists | 385 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_uses_github_api_rule_for_github_dot_com` | `api.github.com` token rewrites `github.com` git URLs |
| returns environment variables with token if hostRule for multiple hostsRules | 402 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_multiple_host_rules` | GitHub API, GitLab, and GitHub Enterprise rules concatenate config entries |
| returns environment variables with token if hostRule is for Gitlab | 446 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_gitlab_token` | GitLab host rule uses `gitlab-ci-token:` username |
| returns environment variables with username and password | 466 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_username_password` | Username/password credential triplet |
| returns environment variables with URL encoded username and password | 487 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_url_encoded_username_password` | `encodeURIComponent`-style credential encoding |
| returns no environment variables when hostType is not supported | 508 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_ignores_unsupported_host_type` | Unsupported host type ignored unless explicitly allowed |
| returns no environment variables when only username is set | 517 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_ignores_username_without_password` | Partial credentials ignored |
| returns no environment variables when only password is set | 526 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_ignores_password_without_username` | Partial credentials ignored |
| returns environment variables when hostType is explicitly set | 535 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_allows_explicit_datasource_host_type` | Additional host type allow-list |
| returns empty environment variables when matchHost contains invalid protocol | 554 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_ignores_invalid_protocol_match_host` | Non-HTTP matchHost ignored |
| returns environment variables for bitbucket-server | 563 | ported | `crates/renovate-core/src/util.rs` | `git_environment_variables_bitbucket_server` | Bitbucket Server `/scm/` and SSH port handling |

---
