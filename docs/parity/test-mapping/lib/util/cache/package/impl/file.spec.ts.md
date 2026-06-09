# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**9/17 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | pending | — |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:616`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L616) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:637`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L637) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:652`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L652) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:668`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L668) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:684`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L684) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:705`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L705) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:773`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L773) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:750`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L750) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:791`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L791) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

