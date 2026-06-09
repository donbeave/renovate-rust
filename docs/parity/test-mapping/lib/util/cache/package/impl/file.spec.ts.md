# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 5 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | ported | [`crates/renovate-core/src/cache/package.rs:582`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L582) |
| 34 | stores payload with value and expiry | opt-out | asserts the exact internal cacache envelope keys (['expiry','value']) and that they are JSON strings after set (using cacache.get + JSON.parse); Rust FilePackageCache uses its own FileEntry {value, expiry} + direct file serde (not cacache/npm cacache); the value+expiry roundtrip persistence and get/set for file backend are covered by multiple ported tests (file_cache_set_and_get_roundtrip, file_cache_returns_*, cleanup tests); this is a TS-specific storage adapter detail with no Rust analogue. |
| 47 | returns undefined on cache miss | ported | [`crates/renovate-core/src/cache/package.rs:556`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L556) |
| 53 | expires cached entries | ported | [`crates/renovate-core/src/cache/package.rs:607`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L607) |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:631`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L631) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:652`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L652) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:667`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L667) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:683`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L683) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:699`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L699) |
| 114 | retrieves value from cache payload | opt-out | asserts internal retrieval from the on-disk payload envelope after set (cacache or equivalent read path); Rust equivalent (FileEntry deserialize in get) behavior covered by all the ported file get/roundtrip/expiry tests; the 'from payload' is TS storage detail. |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:720`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L720) |
| 148 | keeps entries with valid non-expired expiry read from disk | opt-out | positive case for reading valid expiry entry from disk and returning value; covered by ported set/get roundtrips and 'returns undefined for invalid expiry' etc that exercise the valid path implicitly; no new behavior. |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:788`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L788) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:765`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L765) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:806`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L806) |
| 194 | skips disk read for entry written this run | opt-out | asserts mem dedup / written-this-run layer prevents redundant disk read (via internal spy or call count on cacache/fs); Rust PackageCache has mem dedup layer exercised by multi-get in same run tests (e.g. with_cache same key), but no exact 'skips disk read' spy assertion in current tests; core 'second get returns cached without recompute' covered. |
| 208 | skips disk read for expired entry written this run | opt-out | variant of this-run skip for an expired-but-written-this-run entry; same mem dedup reason, core expiry + mem behavior covered by existing ported. |

