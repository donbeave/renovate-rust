# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/github.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/github.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/github › HTTP`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports app mode | 67 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| supports different datasources | 81 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| paginates | 95 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| uses paginationField | 113 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| paginates with auth and repo | 142 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| paginates with auth and repo on GHE | 178 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| attempts to paginate | 219 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| rebases GHE Server pagination links | 234 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| preserves pagination links by default | 263 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| preserves pagination links for github.com | 285 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |

### `util/http/github › HTTP › handleGotError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log a once warning for github.com 401 | 310 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw Not found | 350 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw 410 | 356 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw rate limit exceeded | 364 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| when the rate limit is exceeded, and host rules are set for GitHub.com, a warn is logged | 373 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| when the rate limit is exceeded, but no host rules are set for GitHub.com, a warn is logged | 391 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| when the rate limit is exceeded to GitHub Enterprise, but no host rules are set, a warn is logged | 410 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw secondary rate limit exceeded | 449 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw Bad credentials | 458 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw platform failure | 464 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw platform failure for ENOTFOUND, ETIMEDOUT or EAI_AGAIN | 476 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw platform failure for 500 | 485 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw platform failure ParseError | 489 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw for unauthorized integration | 493 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw for unauthorized integration2 | 501 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw on abuse | 507 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw on repository change | 515 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw platform failure on 422 response | 524 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw original error when failed to add reviewers | 532 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw original error when pull requests aleady existed | 542 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw original error of unknown type | 551 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| should throw original error when milestone not found | 559 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |

### `util/http/github › GraphQL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips path from baseUrl | 645 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| supports app mode | 658 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| returns empty array for undefined data | 672 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| returns empty array for undefined data. | 688 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| throws errors for invalid responses | 702 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| halves node count and retries request | 713 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| queryRepo | 728 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| queryRepoField | 742 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| limit result size | 756 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| shrinks items count on 50x | 770 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| expands items count on timeout | 799 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| continues to iterate with a lower page size on error 502 | 827 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| removes cache record once expanded to the maximum | 843 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| throws on 50x if count < 10 | 871 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |

### `util/http/github › getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add header and return | 882 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support relative path | 900 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support default to api.github.com if no baseURL has been supplied | 918 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support custom host if a baseURL has been supplied | 934 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support default to api.github.com if no baseURL, but repository has been supplied | 953 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support custom host if a baseURL and repository has been supplied | 971 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support default to api.github.com if content path is used | 991 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |
| support custom host if content path is used | 1007 | not-applicable | — | — | tests GithubHttp (got-based) with httpMock; Rust uses reqwest |

---

