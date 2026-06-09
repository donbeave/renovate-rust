# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**12/14 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:919`](../../../../../../../crates/renovate-core/src/cache/package.rs#L919) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:948`](../../../../../../../crates/renovate-core/src/cache/package.rs#L948) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:979`](../../../../../../../crates/renovate-core/src/cache/package.rs#L979) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1096`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1096) |
| 140 | does not cache values rejected by cacheresult predicate | pending | — |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1126`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1126) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1011`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1011) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1170`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1170) |
| 256 | updates cached result after soft ttl expires | pending | — |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1323`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1323) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1054`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1054) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1195`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1195) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1243`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1243) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1282`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1282) |

