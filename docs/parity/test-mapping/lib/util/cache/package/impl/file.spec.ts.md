# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**9/17 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | pending | — |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:621`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L621) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:642`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L642) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:657`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L657) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:673`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L673) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:689`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L689) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:710`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L710) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:778`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L778) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:755`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L755) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:796`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L796) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

