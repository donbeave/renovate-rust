# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rust-release-channel/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rust-release-channel/index.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `modules/versioning/rust-release-channel/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_valid_matches_renovate_rust_release_channel_index_spec | — |
| isVersion("$input") === $expected | 23 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_version_matches_renovate_rust_release_channel_index_spec | — |
| isSingleVersion("$input") === $expected | 40 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_single_version_matches_renovate_rust_release_channel_index_spec | — |
| isStable("$version") === $expected | 54 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_stable_matches_renovate_rust_release_channel_index_spec | — |
| equals("$a", "$b") === $expected | 69 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | equals_matches_renovate_rust_release_channel_index_spec | — |
| isGreaterThan("$a", "$b") === $expected | 85 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_greater_than_matches_renovate_rust_release_channel_index_spec | — |
| sortVersions("$a", "$b") === $expected | 113 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | sort_versions_matches_renovate_rust_release_channel_index_spec | — |
| getMajor("$version") === $expected | 137 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | get_major_matches_renovate_rust_release_channel_index_spec | — |
| getMinor("$version") === $expected | 151 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | get_minor_matches_renovate_rust_release_channel_index_spec | — |
| getPatch("$version") === $expected | 163 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | get_patch_matches_renovate_rust_release_channel_index_spec | — |
| matches("$version", "$range") === $expected | 176 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | matches_matches_renovate_rust_release_channel_index_spec | — |
| isCompatible("$version", "$current") === $expected | 204 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | is_compatible_matches_renovate_rust_release_channel_index_spec | — |
| getSatisfyingVersion($versions, "$range") === $expected | 229 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | get_satisfying_version_matches_renovate_rust_release_channel_index_spec | — |
| minSatisfyingVersion($versions, "$range") === $expected | 248 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | min_satisfying_version_matches_renovate_rust_release_channel_index_spec | — |
| getNewValue({ currentValue: "$currentValue", rangeStrategy: "$rangeStrategy", newVersion: "$newVersion" }) === $expected | 267 | ported | crates/renovate-core/src/versioning/rust_release_channel.rs | get_new_value_matches_renovate_rust_release_channel_index_spec | — |

---
