# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/github.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/github.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 54 | **Status:** not-applicable

### `util/http/github › HTTP`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports app mode | 67 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| supports different datasources | 81 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| paginates | 95 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| uses paginationField | 113 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| paginates with auth and repo | 142 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| paginates with auth and repo on GHE | 178 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| attempts to paginate | 219 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| rebases GHE Server pagination links | 234 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| preserves pagination links by default | 263 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| preserves pagination links for github.com | 285 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `util/http/github › HTTP › handleGotError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log a once warning for github.com 401 | 310 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw Not found | 350 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw 410 | 356 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw rate limit exceeded | 364 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| when the rate limit is exceeded, and host rules are set for GitHub.com, a warn is logged | 373 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| when the rate limit is exceeded, but no host rules are set for GitHub.com, a warn is logged | 391 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| when the rate limit is exceeded to GitHub Enterprise, but no host rules are set, a warn is logged | 410 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw secondary rate limit exceeded | 449 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw Bad credentials | 458 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw platform failure | 464 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw platform failure for ENOTFOUND, ETIMEDOUT or EAI_AGAIN | 476 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw platform failure for 500 | 485 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw platform failure ParseError | 489 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw for unauthorized integration | 493 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw for unauthorized integration2 | 501 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw on abuse | 507 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw on repository change | 515 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw platform failure on 422 response | 524 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw original error when failed to add reviewers | 532 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw original error when pull requests aleady existed | 542 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw original error of unknown type | 551 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw original error when milestone not found | 559 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `util/http/github › GraphQL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips path from baseUrl | 645 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| supports app mode | 658 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty array for undefined data | 672 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty array for undefined data. | 688 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws errors for invalid responses | 702 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| halves node count and retries request | 713 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| queryRepo | 728 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| queryRepoField | 742 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| limit result size | 756 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| shrinks items count on 50x | 770 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| expands items count on timeout | 799 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| continues to iterate with a lower page size on error 502 | 827 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| removes cache record once expanded to the maximum | 843 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on 50x if count < 10 | 871 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `util/http/github › getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add header and return | 882 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support relative path | 900 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support default to api.github.com if no baseURL has been supplied | 918 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support custom host if a baseURL has been supplied | 934 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support default to api.github.com if no baseURL, but repository has been supplied | 953 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support custom host if a baseURL and repository has been supplied | 971 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support default to api.github.com if content path is used | 991 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| support custom host if content path is used | 1007 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

---
