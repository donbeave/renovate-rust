# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/rust-toolchain/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/rust-toolchain/extract.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `modules/manager/rust-toolchain/extract › extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts major.minor.patch versions | 7 | ported | `rust_toolchain.rs` | `extract_major_minor_patch_version` | — |
| extracts major.minor ranges | 27 | ported | `rust_toolchain.rs` | `extract_major_minor_range` | — |
| extracts beta channel | 47 | ported | `rust_toolchain.rs` | `extract_beta_channel` | — |
| extracts nightly channel | 67 | ported | `rust_toolchain.rs` | `extract_nightly_channel` | — |
| extracts dated nightly channel | 87 | ported | `rust_toolchain.rs` | `extract_dated_nightly_channel` | — |
| returns null for invalid TOML | 107 | ported | `rust_toolchain.rs` | `extract_returns_none_for_invalid_toml` | — |
| returns null when [toolchain] section is absent | 115 | ported | `rust_toolchain.rs` | `extract_returns_none_when_no_toolchain_section` | — |
| returns null when channel is absent | 123 | ported | `rust_toolchain.rs` | `extract_returns_none_when_channel_absent` | — |
| returns null for unparseable channel value | 134 | ported | `rust_toolchain.rs` | `extract_returns_none_for_invalid_channel` | — |
| can handle additional fields | 145 | ported | `rust_toolchain.rs` | `extract_handles_additional_fields` | — |
| can read from legacy filename | 167 | ported | `rust_toolchain.rs` | `extract_reads_from_legacy_filename` | — |
| returns null for empty legacy file | 187 | ported | `rust_toolchain.rs` | `extract_returns_none_for_empty_legacy_file` | — |
| extracts from legacy format | 192 | ported | `rust_toolchain.rs` | `extract_from_legacy_format` | — |
| returns null for multiline legacy files | 206 | ported | `rust_toolchain.rs` | `extract_returns_none_for_multiline_legacy` | — |
