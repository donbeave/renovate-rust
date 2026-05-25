# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/check-token.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/check-token.spec.ts
**Total tests:** 34 | **Ported:** 0 | **Actionable:** 34 | **Status:** pending

### `util/check-token › checkGithubToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if data is empty | 26 | pending | — | — | — |
| returns early if GitHub token is found | 33 | pending | — | — | — |
| returns early if token warnings are disabled | 45 | pending | — | — | — |
| does not warn if there is dependencies with GitHub sourceUrl | 60 | pending | — | — | — |
| logs warning for github-tags datasource | 68 | pending | — | — | — |
| logs warning for github-releases datasource | 85 | pending | — | — | — |
| logs warning once | 102 | pending | — | — | — |

### `util/check-token › isGithubPersonalAccessToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github personnal access token | 132 | pending | — | — | — |
| returns false when string is a github application token | 136 | pending | — | — | — |
| returns false when string is a github fine grained personal access token | 140 | pending | — | — | — |
| returns false when string is not a token at all | 144 | pending | — | — | — |

### `util/check-token › isGithubServerToServerToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github server to server token | 150 | pending | — | — | — |
| returns true when string is a 2026-style GitHub Installation Access Token | 155 | pending | — | — | — |
| returns false when string is a github personal access token token | 161 | pending | — | — | — |
| returns false when string is a github fine grained personal access token | 165 | pending | — | — | — |
| returns false when string is not a token at all | 169 | pending | — | — | — |

### `util/check-token › isGithubFineGrainedPersonalAccessToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when string is a github fine grained personal access token | 175 | pending | — | — | — |
| returns false when string is a github personnal access token | 181 | pending | — | — | — |
| returns false when string is a github application token | 185 | pending | — | — | — |
| returns false when string is not a token at all | 189 | pending | — | — | — |

### `util/check-token › findGithubToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the token string when hostRule match search with a valid personal access token | 195 | pending | — | — | — |
| returns undefined when no token is defined | 201 | pending | — | — | — |
| remove x-access-token token prefix | 205 | pending | — | — | — |

### `util/check-token › takePersonalAccessTokenIfPossible`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when both token are undefined | 216 | pending | — | — | — |
| returns gitTagsToken when both token are PAT | 224 | pending | — | — | — |
| returns githubToken is PAT and gitTagsGithubToken is not a PAT | 232 | pending | — | — | — |
| returns gitTagsToken when both token are set but not PAT | 240 | pending | — | — | — |
| returns gitTagsToken when gitTagsToken not PAT and gitTagsGithubToken is not set | 248 | pending | — | — | — |
| returns githubToken when githubToken not PAT and gitTagsGithubToken is not set | 256 | pending | — | — | — |
| take personal access token over fine grained token | 264 | pending | — | — | — |
| take fine grained token over server to server token | 272 | pending | — | — | — |
| take git-tags fine grained token | 280 | pending | — | — | — |
| take git-tags unknown token type when no other token is set | 288 | pending | — | — | — |
| take github unknown token type when no other token is set | 296 | pending | — | — | — |

---

