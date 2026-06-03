# `lib/util/cache/package/impl/file.spec.ts`

[← `util/cache`](../../../../../_by-module/util/cache.md) · [all modules](../../../../../README.md)

**9/17 ported** (8 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | sets and gets | pending | — |
| 34 | stores payload with value and expiry | pending | — |
| 47 | returns undefined on cache miss | pending | — |
| 53 | expires cached entries | pending | — |
| 65 | returns undefined for null cached value | ported | `crates/renovate-core/src/cache/package.rs:604` |
| 73 | returns undefined for invalid json | ported | `crates/renovate-core/src/cache/package.rs:625` |
| 81 | returns undefined for corrupted cache payload | ported | `crates/renovate-core/src/cache/package.rs:640` |
| 93 | returns undefined for missing expiry | ported | `crates/renovate-core/src/cache/package.rs:656` |
| 102 | returns undefined for invalid expiry | ported | `crates/renovate-core/src/cache/package.rs:672` |
| 114 | retrieves value from cache payload | pending | — |
| 127 | removes expired and invalid entries | ported | `crates/renovate-core/src/cache/package.rs:693` |
| 148 | keeps entries with valid non-expired expiry read from disk | pending | — |
| 159 | keeps entries without expiry field | ported | `crates/renovate-core/src/cache/package.rs:761` |
| 169 | removes entries with invalid expiry | ported | `crates/renovate-core/src/cache/package.rs:738` |
| 182 | continues on cleanup errors | ported | `crates/renovate-core/src/cache/package.rs:779` |
| 194 | skips disk read for entry written this run | pending | — |
| 208 | skips disk read for expired entry written this run | pending | — |

