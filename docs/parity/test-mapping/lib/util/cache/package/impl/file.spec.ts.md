# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**9/17 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | pending | — |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:604`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L604) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:625`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L625) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:640`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L640) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:656`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L656) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:672`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L672) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:693`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L693) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:761`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L761) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:738`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L738) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:779`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L779) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

