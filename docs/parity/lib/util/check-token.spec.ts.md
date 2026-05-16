# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/check-token.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/check-token.spec.ts
**Total tests:** 34 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/check-token › checkGithubToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if data is empty | 26 | not-applicable | — | — | Renovate's GitHub dependency warning helper is not implemented as a Rust API; Rust validates configured platform tokens through platform clients. |
| returns early if GitHub token is found | 33 | not-applicable | — | — | Renovate's host-rules-backed GitHub warning helper is not implemented as a Rust API. |
| returns early if token warnings are disabled | 45 | not-applicable | — | — | Renovate's `githubTokenWarn` warning helper is not implemented as a Rust API. |
| does not warn if there is dependencies with GitHub sourceUrl | 60 | not-applicable | — | — | Renovate's GitHub dependency warning helper is not implemented as a Rust API. |
| logs warning for github-tags datasource | 68 | not-applicable | — | — | Renovate's GitHub dependency warning helper is not implemented as a Rust API. |
| logs warning for github-releases datasource | 85 | not-applicable | — | — | Renovate's GitHub dependency warning helper is not implemented as a Rust API. |
| logs warning once | 102 | not-applicable | — | — | Renovate's GitHub dependency warning helper and memory-cache side effect are not implemented as a Rust API. |

### `util/check-token › isGithubPersonalAccessToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github personnal access token | 132 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github application token | 136 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github fine grained personal access token | 140 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is not a token at all | 144 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |

### `util/check-token › isGithubServerToServerToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github server to server token | 150 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns true when string is a 2026-style GitHub Installation Access Token | 155 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github personal access token token | 161 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github fine grained personal access token | 165 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is not a token at all | 169 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |

### `util/check-token › isGithubFineGrainedPersonalAccessToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github fine grained personal access token | 175 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github personnal access token | 181 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is a github application token | 185 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |
| returns false when string is not a token at all | 189 | not-applicable | — | — | Renovate's GitHub token-prefix classifier is not implemented as a Rust API. |

### `util/check-token › findGithubToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the token string when hostRule match search with a valid personal access token | 195 | not-applicable | — | — | Renovate's host-rule token extraction helper is not implemented as a Rust API. |
| returns undefined when no token is defined | 201 | not-applicable | — | — | Renovate's host-rule token extraction helper is not implemented as a Rust API. |
| remove x-access-token token prefix | 205 | not-applicable | — | — | Renovate's GitHub token-prefix stripping helper is not implemented as a Rust API. |

### `util/check-token › takePersonalAccessTokenIfPossible`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when both token are undefined | 216 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| returns gitTagsToken when both token are PAT | 224 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| returns githubToken is PAT and gitTagsGithubToken is not a PAT | 232 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| returns gitTagsToken when both token are set but not PAT | 240 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| returns gitTagsToken when gitTagsToken not PAT and gitTagsGithubToken is not set | 248 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| returns githubToken when githubToken not PAT and gitTagsGithubToken is not set | 256 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| take personal access token over fine grained token | 264 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| take fine grained token over server to server token | 272 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| take git-tags fine grained token | 280 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| take git-tags unknown token type when no other token is set | 288 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |
| take github unknown token type when no other token is set | 296 | not-applicable | — | — | Renovate's GitHub token preference helper is not implemented as a Rust API. |

---

