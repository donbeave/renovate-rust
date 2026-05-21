# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/url.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/url › parseGitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports ports | 9 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |

### `util/git/url › getHttpUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns https url for git url | 40 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url for https url | 44 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns http url for http url | 48 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns http url for ssh url with port | 52 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns gitlab url with token | 60 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns github url with token | 75 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns bitbucket-server url | 90 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| removes username/password from URL | 100 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| replaces username/password with given token | 106 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |

### `util/git/url › getRemoteUrlWithToken()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns original url if no host rule is found | 117 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| transforms an ssh git url to https for the purpose of finding hostRules | 123 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| does not transform urls that are not parseable as git urls | 132 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns http url with token | 141 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with token | 148 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with token for non-http protocols | 155 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with encoded token | 162 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns http url with username and password | 169 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with username and password | 179 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with username and password for non-http protocols | 189 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with encoded username and password | 199 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url with encoded gitlab token | 209 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |
| returns https url for ssh url with encoded github token | 218 | not-applicable | — | — | tests git URL parsing/manipulation via git-url-parse npm library; no direct Rust equivalent |

---

