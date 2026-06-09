# `lib/util/cache/package/with-cache.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**12/14 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 35 | caches string result | ported | [`crates/renovate-core/src/cache/package.rs:913`](../../../../../../../crates/renovate-core/src/cache/package.rs#L913) |
| 57 | disables cache if cacheable is false | ported | [`crates/renovate-core/src/cache/package.rs:942`](../../../../../../../crates/renovate-core/src/cache/package.rs#L942) |
| 83 | forces cache if cacheprivatepackages=true | ported | [`crates/renovate-core/src/cache/package.rs:973`](../../../../../../../crates/renovate-core/src/cache/package.rs#L973) |
| 115 | caches null values | ported | [`crates/renovate-core/src/cache/package.rs:1090`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1090) |
| 140 | does not cache values rejected by cacheresult predicate | pending | — |
| 170 | ignores cached values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1120`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1120) |
| 212 | does not cache undefined | ported | [`crates/renovate-core/src/cache/package.rs:1005`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1005) |
| 232 | uses custom ttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1164`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1164) |
| 256 | updates cached result after soft ttl expires | pending | — |
| 313 | overrides soft ttl and updates result | ported | [`crates/renovate-core/src/cache/package.rs:1317`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1317) |
| 375 | returns stale result on error | ported | [`crates/renovate-core/src/cache/package.rs:1048`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1048) |
| 414 | does not return stale values rejected by cacheresult predicate | ported | [`crates/renovate-core/src/cache/package.rs:1189`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1189) |
| 454 | drops stale value after hard ttl expires | ported | [`crates/renovate-core/src/cache/package.rs:1237`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1237) |
| 505 | does not use fallback when fallback=false | ported | [`crates/renovate-core/src/cache/package.rs:1276`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1276) |

