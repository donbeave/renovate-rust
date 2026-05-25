# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/github.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/github.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 54 | **Status:** pending

### `util/http/github › HTTP`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports app mode | 67 | pending | — | — | — |
| supports different datasources | 81 | pending | — | — | — |
| paginates | 95 | pending | — | — | — |
| uses paginationField | 113 | pending | — | — | — |
| paginates with auth and repo | 142 | pending | — | — | — |
| paginates with auth and repo on GHE | 178 | pending | — | — | — |
| attempts to paginate | 219 | pending | — | — | — |
| rebases GHE Server pagination links | 234 | pending | — | — | — |
| preserves pagination links by default | 263 | pending | — | — | — |
| preserves pagination links for github.com | 285 | pending | — | — | — |

### `util/http/github › HTTP › handleGotError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log a once warning for github.com 401 | 310 | pending | — | — | — |
| should throw Not found | 350 | pending | — | — | — |
| should throw 410 | 356 | pending | — | — | — |
| should throw rate limit exceeded | 364 | pending | — | — | — |
| when the rate limit is exceeded, and host rules are set for GitHub.com, a warn is logged | 373 | pending | — | — | — |
| when the rate limit is exceeded, but no host rules are set for GitHub.com, a warn is logged | 391 | pending | — | — | — |
| when the rate limit is exceeded to GitHub Enterprise, but no host rules are set, a warn is logged | 410 | pending | — | — | — |
| should throw secondary rate limit exceeded | 449 | pending | — | — | — |
| should throw Bad credentials | 458 | pending | — | — | — |
| should throw platform failure | 464 | pending | — | — | — |
| should throw platform failure for ENOTFOUND, ETIMEDOUT or EAI_AGAIN | 476 | pending | — | — | — |
| should throw platform failure for 500 | 485 | pending | — | — | — |
| should throw platform failure ParseError | 489 | pending | — | — | — |
| should throw for unauthorized integration | 493 | pending | — | — | — |
| should throw for unauthorized integration2 | 501 | pending | — | — | — |
| should throw on abuse | 507 | pending | — | — | — |
| should throw on repository change | 515 | pending | — | — | — |
| should throw platform failure on 422 response | 524 | pending | — | — | — |
| should throw original error when failed to add reviewers | 532 | pending | — | — | — |
| should throw original error when pull requests aleady existed | 542 | pending | — | — | — |
| should throw original error of unknown type | 551 | pending | — | — | — |
| should throw original error when milestone not found | 559 | pending | — | — | — |

### `util/http/github › GraphQL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips path from baseUrl | 645 | pending | — | — | — |
| supports app mode | 658 | pending | — | — | — |
| returns empty array for undefined data | 672 | pending | — | — | — |
| returns empty array for undefined data. | 688 | pending | — | — | — |
| throws errors for invalid responses | 702 | pending | — | — | — |
| halves node count and retries request | 713 | pending | — | — | — |
| queryRepo | 728 | pending | — | — | — |
| queryRepoField | 742 | pending | — | — | — |
| limit result size | 756 | pending | — | — | — |
| shrinks items count on 50x | 770 | pending | — | — | — |
| expands items count on timeout | 799 | pending | — | — | — |
| continues to iterate with a lower page size on error 502 | 827 | pending | — | — | — |
| removes cache record once expanded to the maximum | 843 | pending | — | — | — |
| throws on 50x if count < 10 | 871 | pending | — | — | — |

### `util/http/github › getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add header and return | 882 | pending | — | — | — |
| support relative path | 900 | pending | — | — | — |
| support default to api.github.com if no baseURL has been supplied | 918 | pending | — | — | — |
| support custom host if a baseURL has been supplied | 934 | pending | — | — | — |
| support default to api.github.com if no baseURL, but repository has been supplied | 953 | pending | — | — | — |
| support custom host if a baseURL and repository has been supplied | 971 | pending | — | — | — |
| support default to api.github.com if content path is used | 991 | pending | — | — | — |
| support custom host if content path is used | 1007 | pending | — | — | — |

---

