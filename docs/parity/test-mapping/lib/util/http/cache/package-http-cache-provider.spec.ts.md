# `lib/util/http/cache/package-http-cache-provider.spec.ts`

[← `util/http`](../../../../_by-module/util/http.md) · [all modules](../../../../README.md)

**0/20 in-scope tests ported** (20 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 74 | skips persisting null cache values | pending | — |
| 83 | loads cache correctly | pending | — |
| 100 | loads cache bypassing server | pending | — |
| 123 | handles cache miss | pending | — |
| 147 | applies writeschema before persisting cache | pending | — |
| 175 | skips cache write when writeschema validation fails | pending | — |
| 189 | prevents caching when cache-control is private | pending | — |
| 206 | prevents caching when the request contains authorization header | pending | — |
| 224 | allows caching when cache-control is private but cacheprivatepackages=true | pending | — |
| 242 | allows caching when cache-control is private but checkcachecontrolheader=false | pending | — |
| 258 | serves stale response during revalidation error | pending | — |
| 274 | stores a trimmed body when refreshing cache after 304 | pending | — |
| 309 | handles cache miss for head request | pending | — |
| 330 | loads cache correctly for head request | pending | — |
| 347 | loads cache bypassing server for head request | pending | — |
| 363 | serves stale head response during revalidation error | pending | — |
| 379 | prevents caching head request when cache-control is private | pending | — |
| 396 | caches head and get requests separately | pending | — |
| 445 | _(it.each / template — verify manually)_ | ? | — |
| 519 | handles case-insensitive cache-control values | pending | — |

