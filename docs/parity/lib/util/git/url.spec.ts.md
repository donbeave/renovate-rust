# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/url.spec.ts
**Total tests:** 23 | **Ported:** 8 | **Actionable:** 23 | **Status:** partial

### `util/git/url › parseGitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports ports | 9 | pending | — | — | —|

### `util/git/url › getHttpUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns https url for git url | 40 | ported | `util.rs` | `test_get_http_url_git` | — |
| returns https url for https url | 44 | ported | `util.rs` | `test_get_http_url_https` | — |
| returns http url for http url | 48 | ported | `util.rs` | `test_get_http_url_http` | — |
| returns http url for ssh url with port | 52 | ported | `util.rs` | `test_get_http_url_ssh_with_port` | — |
| returns gitlab url with token | 60 | ported | `util.rs` | `test_get_http_url_gitlab_token` | — |
| returns github url with token | 75 | ported | `util.rs` | `test_get_http_url_github_token` | — |
| returns bitbucket-server url | 90 | pending | — | — | —|
| removes username/password from URL | 100 | ported | `util.rs` | `test_get_http_url_removes_credentials` | — |
| replaces username/password with given token | 106 | ported | `util.rs` | `test_get_http_url_replaces_credentials` | — |

### `util/git/url › getRemoteUrlWithToken()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns original url if no host rule is found | 117 | pending | — | — | —|
| transforms an ssh git url to https for the purpose of finding hostRules | 123 | pending | — | — | —|
| does not transform urls that are not parseable as git urls | 132 | pending | — | — | —|
| returns http url with token | 141 | pending | — | — | —|
| returns https url with token | 148 | pending | — | — | —|
| returns https url with token for non-http protocols | 155 | pending | — | — | —|
| returns https url with encoded token | 162 | pending | — | — | —|
| returns http url with username and password | 169 | pending | — | — | —|
| returns https url with username and password | 179 | pending | — | — | —|
| returns https url with username and password for non-http protocols | 189 | pending | — | — | —|
| returns https url with encoded username and password | 199 | pending | — | — | —|
| returns https url with encoded gitlab token | 209 | pending | — | — | —|
| returns https url for ssh url with encoded github token | 218 | pending | — | — | —|

---

