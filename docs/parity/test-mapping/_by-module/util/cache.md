# Module: `util/cache`

[← all modules](../../README.md)

**Coverage:** 59/151 in-scope tests ported (opt-out=0) across 13 spec files.

| Spec file | it() | ported | pending | opt-out | Rust test file(s) | Status |
|---|--:|--:|--:|--:|---|---|
| [`lib/util/cache/memory/index.spec.ts`](../../lib/util/cache/memory/index.spec.ts.md) | 7 | 7 | 0 | 0 | [`crates/renovate-core/src/cache/memory.rs:54`](../../../../../crates/renovate-core/src/cache/memory.rs#L54) | ported |
| [`lib/util/cache/package/backend.spec.ts`](../../lib/util/cache/package/backend.spec.ts.md) | 10 | 0 | 10 | 0 | — | pending |
| [`lib/util/cache/package/impl/file.spec.ts`](../../lib/util/cache/package/impl/file.spec.ts.md) | 17 | 9 | 8 | 0 | [`crates/renovate-core/src/cache/package.rs:616`](../../../../../crates/renovate-core/src/cache/package.rs#L616) | partial |
| [`lib/util/cache/package/impl/redis.spec.ts`](../../lib/util/cache/package/impl/redis.spec.ts.md) | 18 | 0 | 18 | 0 | — | pending |
| [`lib/util/cache/package/impl/sqlite.spec.ts`](../../lib/util/cache/package/impl/sqlite.spec.ts.md) | 12 | 0 | 12 | 0 | — | pending |
| [`lib/util/cache/package/index.spec.ts`](../../lib/util/cache/package/index.spec.ts.md) | 9 | 9 | 0 | 0 | [`crates/renovate-core/src/cache/package.rs:555`](../../../../../crates/renovate-core/src/cache/package.rs#L555) | ported |
| [`lib/util/cache/package/key.spec.ts`](../../lib/util/cache/package/key.spec.ts.md) | 1 | 1 | 0 | 0 | [`crates/renovate-core/src/branch.rs:2519`](../../../../../crates/renovate-core/src/branch.rs#L2519) | ported |
| [`lib/util/cache/package/ttl.spec.ts`](../../lib/util/cache/package/ttl.spec.ts.md) | 31 | 18 | 13 | 0 | [`crates/renovate-core/src/cache/package.rs:1340`](../../../../../crates/renovate-core/src/cache/package.rs#L1340) | partial |
| [`lib/util/cache/package/with-cache.spec.ts`](../../lib/util/cache/package/with-cache.spec.ts.md) | 14 | 12 | 2 | 0 | [`crates/renovate-core/src/cache/package.rs:912`](../../../../../crates/renovate-core/src/cache/package.rs#L912) | partial |
| [`lib/util/cache/repository/http-cache.spec.ts`](../../lib/util/cache/repository/http-cache.spec.ts.md) | 3 | 3 | 0 | 0 | [`crates/renovate-core/src/http.rs:1540`](../../../../../crates/renovate-core/src/http.rs#L1540) | ported |
| [`lib/util/cache/repository/impl/local.spec.ts`](../../lib/util/cache/repository/impl/local.spec.ts.md) | 13 | 0 | 13 | 0 | — | pending |
| [`lib/util/cache/repository/impl/s3.spec.ts`](../../lib/util/cache/repository/impl/s3.spec.ts.md) | 11 | 0 | 11 | 0 | — | pending |
| [`lib/util/cache/repository/index.spec.ts`](../../lib/util/cache/repository/index.spec.ts.md) | 5 | 0 | 5 | 0 | — | pending |

