# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/redis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/redis.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `util/cache/package/impl/redis › normalizeRedisUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rewrites $url to $expected | 10 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|

### `util/cache/package/impl/redis › PackageCacheRedis › create`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initializes single-node client and connects | 40 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| initializes single-node client with secure url | 56 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| initializes cluster client | 64 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| initializes cluster client with username and password | 78 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| initializes cluster client with username only | 91 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| initializes cluster client with password only | 104 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|

### `util/cache/package/impl/redis › PackageCacheRedis › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns value from cache payload | 119 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| removes expired cached entry | 133 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| returns undefined for missing expiry | 145 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| returns undefined for invalid expiry | 155 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| returns undefined on cache miss | 168 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| returns undefined on error | 176 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|

### `util/cache/package/impl/redis › PackageCacheRedis › set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores payload with value and expiry | 186 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| deletes entry with negative TTL | 204 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| handles set error gracefully | 213 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|

### `util/cache/package/impl/redis › PackageCacheRedis › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| destroys the client | 225 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|
| handles destroy error gracefully | 233 | not-applicable | — | — | TS-library-specific; uses ioredis mock client; TypeScript Redis cache implementation pipeline|

---

