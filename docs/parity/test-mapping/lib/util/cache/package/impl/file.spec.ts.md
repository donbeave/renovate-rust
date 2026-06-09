# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**12/16 in-scope tests ported** (4 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | ported | [`crates/renovate-core/src/cache/package.rs:582`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L582) |
| 34 | stores payload with value and expiry | opt-out | asserts the exact internal cacache envelope keys (['expiry','value']) and that they are JSON strings after set (using cacache.get + JSON.parse); Rust FilePackageCache uses its own FileEntry {value, expiry} + direct file serde (not cacache/npm cacache); the value+expiry roundtrip persistence and get/set for file backend are covered by multiple ported tests (file_cache_set_and_get_roundtrip, file_cache_returns_*, cleanup tests); this is a TS-specific storage adapter detail with no Rust analogue. |
| 47 | returns undefined on cache miss | ported | [`crates/renovate-core/src/cache/package.rs:556`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L556) |
| 53 | expires cached entries | ported | [`crates/renovate-core/src/cache/package.rs:617`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L617) |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:641`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L641) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:662`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L662) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:677`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L677) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:693`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L693) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:709`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L709) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:730`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L730) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:798`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L798) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:775`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L775) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:816`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L816) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

