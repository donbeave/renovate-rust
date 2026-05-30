# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/url.spec.ts
**Total tests:** 23 | **Ported:** 23 | **Actionable:** 0 | **Status:** done

### `util/git/url › parseGitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports ports | 9 | ported | `util.rs` | `git_url_parse_supports_ports` | — |

### `util/git/url › getHttpUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns https url for git url | 40 | ported | `util.rs` | `test_get_http_url_git` | — |
| returns https url for https url | 44 | ported | `util.rs` | `test_get_http_url_https` | — |
| returns http url for http url | 48 | ported | `util.rs` | `test_get_http_url_http` | — |
| returns http url for ssh url with port | 52 | ported | `util.rs` | `test_get_http_url_ssh_with_port` | — |
| returns gitlab url with token | 60 | ported | `util.rs` | `test_get_http_url_gitlab_token` | — |
| returns github url with token | 75 | ported | `util.rs` | `test_get_http_url_github_token` | — |
| returns bitbucket-server url | 90 | ported | `util.rs` | `test_get_http_url_bitbucket_server` | — |
| removes username/password from URL | 100 | ported | `util.rs` | `test_get_http_url_removes_credentials` | — |
| replaces username/password with given token | 106 | ported | `util.rs` | `test_get_http_url_replaces_credentials` | — |

### `util/git/url › getRemoteUrlWithToken()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns original url if no host rule is found | 117 | ported | `util.rs` | `git_remote_url_with_token_returns_original_without_host_rule` | — |
| transforms an ssh git url to https for the purpose of finding hostRules | 123 | ported | `util.rs` | `git_remote_url_with_token_finds_host_rule_using_coerced_ssh_url` | — |
| does not transform urls that are not parseable as git urls | 132 | ported | `util.rs` | `git_remote_url_with_token_keeps_unparseable_lookup_url` | — |
| returns http url with token | 141 | ported | `util.rs` | `git_remote_url_with_token_returns_http_url_with_token` | — |
| returns https url with token | 148 | ported | `util.rs` | `git_remote_url_with_token_returns_https_url_with_token` | — |
| returns https url with token for non-http protocols | 155 | ported | `util.rs` | `git_remote_url_with_token_returns_https_url_for_non_http_protocols` | — |
| returns https url with encoded token | 162 | ported | `util.rs` | `git_remote_url_with_token_encodes_token` | — |
| returns http url with username and password | 169 | ported | `util.rs` | `git_remote_url_with_token_returns_http_url_with_username_password` | — |
| returns https url with username and password | 179 | ported | `util.rs` | `git_remote_url_with_token_returns_https_url_with_username_password` | — |
| returns https url with username and password for non-http protocols | 189 | ported | `util.rs` | `git_remote_url_with_token_returns_https_url_with_username_password_for_non_http` | — |
| returns https url with encoded username and password | 199 | ported | `util.rs` | `git_remote_url_with_token_encodes_username_password` | — |
| returns https url with encoded gitlab token | 209 | ported | `util.rs` | `git_remote_url_with_token_returns_gitlab_credentials` | — |
| returns https url for ssh url with encoded github token | 218 | ported | `util.rs` | `git_remote_url_with_token_returns_github_credentials` | — |

---
