# `lib/util/http/github.spec.ts`

[← `util/http`](../../../_by-module/util/http.md) · [all modules](../../../README.md)

**0/54 in-scope tests ported** (54 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 67 | supports app mode | pending | — |
| 81 | supports different datasources | pending | — |
| 95 | paginates | pending | — |
| 113 | uses paginationfield | pending | — |
| 142 | paginates with auth and repo | pending | — |
| 178 | paginates with auth and repo on ghe | pending | — |
| 219 | attempts to paginate | pending | — |
| 234 | rebases ghe server pagination links | pending | — |
| 263 | preserves pagination links by default | pending | — |
| 285 | preserves pagination links for github.com | pending | — |
| 310 | should log a once warning for github.com 401 | pending | — |
| 350 | should throw not found | pending | — |
| 356 | should throw 410 | pending | — |
| 364 | should throw rate limit exceeded | pending | — |
| 373 | when the rate limit is exceeded, and host rules are set for github.com, a warn is logged | pending | — |
| 391 | when the rate limit is exceeded, but no host rules are set for github.com, a warn is logged | pending | — |
| 410 | when the rate limit is exceeded to github enterprise, but no host rules are set, a warn is logged | pending | — |
| 449 | should throw secondary rate limit exceeded | pending | — |
| 458 | should throw bad credentials | pending | — |
| 464 | should throw platform failure | pending | — |
| 476 | should throw platform failure for enotfound, etimedout or eai_again | pending | — |
| 485 | should throw platform failure for 500 | pending | — |
| 489 | should throw platform failure parseerror | pending | — |
| 493 | should throw for unauthorized integration | pending | — |
| 501 | should throw for unauthorized integration2 | pending | — |
| 507 | should throw on abuse | pending | — |
| 515 | should throw on repository change | pending | — |
| 524 | should throw platform failure on 422 response | pending | — |
| 532 | should throw original error when failed to add reviewers | pending | — |
| 542 | should throw original error when pull requests aleady existed | pending | — |
| 551 | should throw original error of unknown type | pending | — |
| 559 | should throw original error when milestone not found | pending | — |
| 645 | strips path from baseurl | pending | — |
| 658 | supports app mode | pending | — |
| 672 | returns empty array for undefined data | pending | — |
| 688 | returns empty array for undefined data. | pending | — |
| 702 | throws errors for invalid responses | pending | — |
| 713 | halves node count and retries request | pending | — |
| 728 | queryrepo | pending | — |
| 742 | queryrepofield | pending | — |
| 756 | limit result size | pending | — |
| 770 | shrinks items count on 50x | pending | — |
| 799 | expands items count on timeout | pending | — |
| 827 | continues to iterate with a lower page size on error 502 | pending | — |
| 843 | removes cache record once expanded to the maximum | pending | — |
| 871 | throws on 50x if count < 10 | pending | — |
| 882 | add header and return | pending | — |
| 900 | support relative path | pending | — |
| 918 | support default to api.github.com if no baseurl has been supplied | pending | — |
| 934 | support custom host if a baseurl has been supplied | pending | — |
| 953 | support default to api.github.com if no baseurl, but repository has been supplied | pending | — |
| 971 | support custom host if a baseurl and repository has been supplied | pending | — |
| 991 | support default to api.github.com if content path is used | pending | — |
| 1007 | support custom host if content path is used | pending | — |

