# `lib/modules/manager/bundler/host-rules.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 15 | returns the authentication header with the password | ported | `crates/renovate-core/src/extractors/bundler.rs:1013` |
| 24 | returns the authentication header with the token | ported | `crates/renovate-core/src/extractors/bundler.rs:1020` |
| 32 | escapes special characters in the username but not the password | ported | `crates/renovate-core/src/extractors/bundler.rs:1027` |
| 55 | returns an empty array if matchhost is missing | ported | `crates/renovate-core/src/extractors/bundler.rs:1394` |
| 63 | returns an empty array if username is missing and password is present | ported | `crates/renovate-core/src/extractors/bundler.rs:1410` |
| 73 | returns an empty array if password and token are missing | ported | `crates/renovate-core/src/extractors/bundler.rs:1426` |
| 83 | returns the hostrule if using matchhost and password | ported | `crates/renovate-core/src/extractors/bundler.rs:1442` |
| 92 | returns the hostrule if using matchhost and token | ported | `crates/renovate-core/src/extractors/bundler.rs:1460` |
| 101 | returns the hostrule if using baseurl and password | ported | `crates/renovate-core/src/extractors/bundler.rs:1477` |
| 110 | returns the hostrule if using baseurl and token | ported | `crates/renovate-core/src/extractors/bundler.rs:1494` |

