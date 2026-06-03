# `lib/util/cache/package/impl/redis.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**0/18 in-scope tests ported** (18 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | _(it.each / template — verify manually)_ | ? | — |
| 40 | initializes single-node client and connects | pending | — |
| 56 | initializes single-node client with secure url | pending | — |
| 64 | initializes cluster client | pending | — |
| 78 | initializes cluster client with username and password | pending | — |
| 91 | initializes cluster client with username only | pending | — |
| 104 | initializes cluster client with password only | pending | — |
| 119 | returns value from cache payload | pending | — |
| 133 | removes expired cached entry | pending | — |
| 145 | returns undefined for missing expiry | pending | — |
| 155 | returns undefined for invalid expiry | pending | — |
| 168 | returns undefined on cache miss | pending | — |
| 176 | returns undefined on error | pending | — |
| 186 | stores payload with value and expiry | pending | — |
| 204 | deletes entry with negative ttl | pending | — |
| 213 | handles set error gracefully | pending | — |
| 225 | destroys the client | pending | — |
| 233 | handles destroy error gracefully | pending | — |

