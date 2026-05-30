# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rust-release-channel/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rust-release-channel/parse.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 0 | **Status:** done

### `modules/versioning/rust-release-channel/parse › channel names`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 6 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_channel_names_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › full versions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 17 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_full_versions_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › partial versions (ranges)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 28 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_partial_versions_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › beta versions with number`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 39 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_beta_versions_with_number_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › beta ranges (without number)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 50 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_beta_ranges_without_number_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › dated channels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 60 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_dated_channels_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › with host triples`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses "$input" correctly | 74 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_host_triples_matches_renovate_rust_release_channel_parse_spec | — |

### `modules/versioning/rust-release-channel/parse › invalid inputs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for "$input" ($reason) | 87 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | parse_invalid_inputs_matches_renovate_rust_release_channel_parse_spec | — |

---
