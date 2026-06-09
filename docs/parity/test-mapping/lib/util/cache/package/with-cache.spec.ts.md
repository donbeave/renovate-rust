# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**12/14 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:912`](../../../../../../../crates/renovate-core/src/cache/package.rs#L912) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:941`](../../../../../../../crates/renovate-core/src/cache/package.rs#L941) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:972`](../../../../../../../crates/renovate-core/src/cache/package.rs#L972) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1089`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1089) |
| 140 | does not cache values rejected by cacheresult predicate | pending | — |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1119`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1119) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1004`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1004) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1163`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1163) |
| 256 | updates cached result after soft ttl expires | pending | — |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1316`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1316) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1047`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1047) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1188`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1188) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1236`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1236) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1275`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1275) |

