# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**12/14 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:900`](../../../../../../../crates/renovate-core/src/cache/package.rs#L900) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:929`](../../../../../../../crates/renovate-core/src/cache/package.rs#L929) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:960`](../../../../../../../crates/renovate-core/src/cache/package.rs#L960) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1077`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1077) |
| 140 | does not cache values rejected by cacheresult predicate | pending | — |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1107`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1107) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:992`](../../../../../../../crates/renovate-core/src/cache/package.rs#L992) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1151`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1151) |
| 256 | updates cached result after soft ttl expires | pending | — |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1304`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1304) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1035`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1035) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1176`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1176) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1224`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1224) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1263`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1263) |

