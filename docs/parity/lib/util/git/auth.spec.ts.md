# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/auth.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/auth.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/auth › getGitAuthenticatedEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns url with token | 13 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with username and password | 31 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| prefers token over username and password | 53 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for different protocols | 73 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns correct url if token already contains GitHub App username | 91 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter | 112 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token and already existing GIT_CONFIG_COUNT from parameter over environment | 134 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token and already existing GIT_CONFIG_COUNT from environment | 157 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token and passthrough existing variables | 176 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| return url with token with invalid GIT_CONFIG_COUNT from environment | 199 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token containing username for GitLab token | 218 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token containing username for GitLab token without hostType | 239 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns original environment variables when no token is set | 259 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for http hosts | 274 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for orgs | 292 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for orgs and projects | 310 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for orgs and projects and ports | 330 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns url with token for bitbucket-server | 354 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |

### `util/git/auth › getGitEnvironmentVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty object if no environment variables exist | 381 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables with token if hostRule for api.github.com exists | 385 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables with token if hostRule for multiple hostsRules | 402 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables with token if hostRule is for Gitlab | 446 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables with username and password | 466 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables with URL encoded username and password | 487 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns no environment variables when hostType is not supported | 508 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns no environment variables when only username is set | 517 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns no environment variables when only password is set | 526 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables when hostType is explicitly set | 535 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns empty environment variables when matchHost contains invalid protocol | 554 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |
| returns environment variables for bitbucket-server | 563 | not-applicable | — | — | tests git URL auth variable generation using git-url-parse npm library; Rust uses own URL parsing |

---

