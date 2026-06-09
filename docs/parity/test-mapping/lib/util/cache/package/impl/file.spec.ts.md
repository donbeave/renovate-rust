# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**9/17 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | pending | — |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:623`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L623) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:644`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L644) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:659`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L659) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:675`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L675) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:691`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L691) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:712`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L712) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:780`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L780) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:757`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L757) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:798`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L798) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

