# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:928`](../../../../../../../crates/renovate-core/src/cache/package.rs#L928) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:957`](../../../../../../../crates/renovate-core/src/cache/package.rs#L957) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:988`](../../../../../../../crates/renovate-core/src/cache/package.rs#L988) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1105`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1105) |
| 140 | does not cache values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1179`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1179) |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1135`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1135) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1020`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1020) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1254`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1254) |
| 256 | updates cached result after soft ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1205`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1205) |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1407`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1407) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1063`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1063) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1279`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1279) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1327`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1327) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1366`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1366) |

