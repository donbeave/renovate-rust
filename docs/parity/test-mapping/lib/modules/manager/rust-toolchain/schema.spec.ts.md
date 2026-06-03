# `lib/modules/manager/rust-toolchain/schema.spec.ts`

[← `manager/rust-toolchain`](../../../../_by-module/manager/rust-toolchain.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | parses valid toml with channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:98`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L98) |
| 21 | parses toml with additional fields | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:107`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L107) |
| 38 | throws error for invalid toml | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:114`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L114) |
| 44 | throws error for missing toolchain section | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:120`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L120) |
| 53 | throws error for missing channel field | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:126`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L126) |
| 62 | throws error for non-string channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:132`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L132) |
| 71 | throws error for empty channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:138`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L138) |
| 80 | parses nightly channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:144`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L144) |
| 95 | parses stable keyword | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:153`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L153) |

