# `lib/util/cache/repository/http-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | should not throw if cache is not a valid httpcache | ported | [`crates/renovate-core/src/http.rs:1540`](../../../../../../../crates/renovate-core/src/http.rs#L1540) |
| 16 | should remove expired items from the cache | ported | [`crates/renovate-core/src/http.rs:1548`](../../../../../../../crates/renovate-core/src/http.rs#L1548) |
| 50 | should remove all items if ttldays is not configured | ported | [`crates/renovate-core/src/http.rs:1575`](../../../../../../../crates/renovate-core/src/http.rs#L1575) |

