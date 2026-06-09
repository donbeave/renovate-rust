# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**10/17 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | sets and gets | ported | [`crates/renovate-core/src/cache/package.rs:581`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L581) |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | [`crates/renovate-core/src/cache/package.rs:638`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L638) |
| 73 | returns undefined for invalid json | ported | [`crates/renovate-core/src/cache/package.rs:659`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L659) |
| 81 | returns undefined for corrupted cache payload | ported | [`crates/renovate-core/src/cache/package.rs:674`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L674) |
| 93 | returns undefined for missing expiry | ported | [`crates/renovate-core/src/cache/package.rs:690`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L690) |
| 102 | returns undefined for invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:706`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L706) |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | [`crates/renovate-core/src/cache/package.rs:727`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L727) |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | [`crates/renovate-core/src/cache/package.rs:795`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L795) |
| 169 | removes entries with invalid expiry | ported | [`crates/renovate-core/src/cache/package.rs:772`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L772) |
| 182 | continues on cleanup errors | ported | [`crates/renovate-core/src/cache/package.rs:813`](../../../../../../../../crates/renovate-core/src/cache/package.rs#L813) |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

