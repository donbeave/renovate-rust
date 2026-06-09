# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:917`](../../../../../../../crates/renovate-core/src/cache/package.rs#L917) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:946`](../../../../../../../crates/renovate-core/src/cache/package.rs#L946) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:977`](../../../../../../../crates/renovate-core/src/cache/package.rs#L977) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1094`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1094) |
| 140 | does not cache values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1168`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1168) |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1124`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1124) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1009`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1009) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1260`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1260) |
| 256 | updates cached result after soft ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1199`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1199) |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1413`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1413) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1052`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1052) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1285`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1285) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1333`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1333) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1372`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1372) |

