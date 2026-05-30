# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/rust-toolchain/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/rust-toolchain/schema.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `modules/manager/rust-toolchain/schema › RustToolchain`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid TOML with channel | 6 | ported | `rust_toolchain.rs` | `schema_parses_valid_toml_with_channel` | — |
| parses TOML with additional fields | 21 | ported | `rust_toolchain.rs` | `schema_parses_toml_with_additional_fields` | — |
| throws error for invalid TOML | 38 | ported | `rust_toolchain.rs` | `schema_rejects_invalid_toml` | — |
| throws error for missing toolchain section | 44 | ported | `rust_toolchain.rs` | `schema_rejects_missing_toolchain_section` | — |
| throws error for missing channel field | 53 | ported | `rust_toolchain.rs` | `schema_rejects_missing_channel_field` | — |
| throws error for non-string channel | 62 | ported | `rust_toolchain.rs` | `schema_rejects_non_string_channel` | — |
| throws error for empty channel | 71 | ported | `rust_toolchain.rs` | `schema_rejects_empty_channel` | — |
| parses nightly channel | 80 | ported | `rust_toolchain.rs` | `schema_parses_nightly_channel` | — |
| parses stable keyword | 95 | ported | `rust_toolchain.rs` | `schema_parses_stable_keyword` | — |
