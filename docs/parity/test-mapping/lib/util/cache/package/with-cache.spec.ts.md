# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:927`](../../../../../../../crates/renovate-core/src/cache/package.rs#L927) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:956`](../../../../../../../crates/renovate-core/src/cache/package.rs#L956) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:987`](../../../../../../../crates/renovate-core/src/cache/package.rs#L987) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1104`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1104) |
| 140 | does not cache values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1178`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1178) |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1134`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1134) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1019`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1019) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1253`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1253) |
| 256 | updates cached result after soft ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1204`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1204) |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1406`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1406) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1062`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1062) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1278`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1278) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1326`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1326) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1365`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1365) |

