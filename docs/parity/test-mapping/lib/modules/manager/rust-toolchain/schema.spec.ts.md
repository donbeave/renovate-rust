# `lib/modules/manager/rust-toolchain/schema.spec.ts`

[← `manager/rust-toolchain`](../../../../_by-module/manager/rust-toolchain.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | parses valid toml with channel | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:98` |
| 21 | parses toml with additional fields | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:107` |
| 38 | throws error for invalid toml | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:114` |
| 44 | throws error for missing toolchain section | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:120` |
| 53 | throws error for missing channel field | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:126` |
| 62 | throws error for non-string channel | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:132` |
| 71 | throws error for empty channel | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:138` |
| 80 | parses nightly channel | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:144` |
| 95 | parses stable keyword | ported | `crates/renovate-core/src/extractors/rust_toolchain.rs:153` |

