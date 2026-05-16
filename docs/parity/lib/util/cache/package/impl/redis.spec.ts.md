# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/redis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/redis.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `util/cache/package/impl/redis › normalizeRedisUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rewrites $url to $expected | 10 | pending | — | — | — |

### `util/cache/package/impl/redis › PackageCacheRedis › create`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initializes single-node client and connects | 40 | pending | — | — | — |
| initializes single-node client with secure url | 56 | pending | — | — | — |
| initializes cluster client | 64 | pending | — | — | — |
| initializes cluster client with username and password | 78 | pending | — | — | — |
| initializes cluster client with username only | 91 | pending | — | — | — |
| initializes cluster client with password only | 104 | pending | — | — | — |

### `util/cache/package/impl/redis › PackageCacheRedis › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns value from cache payload | 119 | pending | — | — | — |
| removes expired cached entry | 133 | pending | — | — | — |
| returns undefined for missing expiry | 145 | pending | — | — | — |
| returns undefined for invalid expiry | 155 | pending | — | — | — |
| returns undefined on cache miss | 168 | pending | — | — | — |
| returns undefined on error | 176 | pending | — | — | — |

### `util/cache/package/impl/redis › PackageCacheRedis › set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores payload with value and expiry | 186 | pending | — | — | — |
| deletes entry with negative TTL | 204 | pending | — | — | — |
| handles set error gracefully | 213 | pending | — | — | — |

### `util/cache/package/impl/redis › PackageCacheRedis › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| destroys the client | 225 | pending | — | — | — |
| handles destroy error gracefully | 233 | pending | — | — | — |

---

