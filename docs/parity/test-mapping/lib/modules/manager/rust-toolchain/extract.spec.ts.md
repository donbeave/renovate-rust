# `lib/modules/manager/rust-toolchain/extract.spec.ts`

[← `manager/rust-toolchain`](../../../../_by-module/manager/rust-toolchain.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | extracts major.minor.patch versions | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:164`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L164) |
| 27 | extracts major.minor ranges | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:173`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L173) |
| 47 | extracts beta channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:182`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L182) |
| 67 | extracts nightly channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:191`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L191) |
| 87 | extracts dated nightly channel | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:203`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L203) |
| 107 | returns null for invalid toml | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:215`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L215) |
| 115 | returns null when [toolchain] section is absent | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:224`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L224) |
| 123 | returns null when channel is absent | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:233`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L233) |
| 134 | returns null for unparseable channel value | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:245`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L245) |
| 145 | can handle additional fields | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:257`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L257) |
| 167 | can read from legacy filename | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:264`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L264) |
| 187 | returns null for empty legacy file | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:271`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L271) |
| 192 | extracts from legacy format | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:277`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L277) |
| 206 | returns null for multiline legacy files | ported | [`crates/renovate-core/src/extractors/rust_toolchain.rs:283`](../../../../../../../crates/renovate-core/src/extractors/rust_toolchain.rs#L283) |

