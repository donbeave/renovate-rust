# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/locked-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/locked-version.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Parse Rails Gem Lock File | 13 | ported | `bundler.rs` | `bundler_locked_version_parse_rails` | — |
| Parse WebPacker Gem Lock File | 19 | ported | `bundler.rs` | `bundler_locked_version_parse_webpacker` | — |
| Parse Mastodon Gem Lock File | 25 | ported | `bundler.rs` | `bundler_locked_version_parse_mastodon` | — |
| Parse Ruby CI Gem Lock File | 31 | ported | `bundler.rs` | `bundler_locked_version_parse_rubyci` | — |
| Parse Gitlab Foss Gem Lock File | 37 | ported | `bundler.rs` | `bundler_locked_version_parse_gitlab_foss` | — |
| returns empty map for empty string | 43 | ported | `bundler.rs` | `bundler_locked_version_empty_string` | — |
| returns empty map when errors occur | 48 | ported | `bundler.rs` | `bundler_locked_version_invalid_input_empty` | Rust has no undefined; tests garbage input |
| strips platform suffixes from dependencies | 54 | ported | `bundler.rs` | `bundler_locked_version_strips_platform_suffix` | — |

### `version extraction regex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts simple versions from parentheses | 84 | ported | `bundler.rs` | `bundler_locked_version_simple_versions` | — |
| extracts complex version formats from parentheses | 98 | ported | `bundler.rs` | `bundler_locked_version_complex_versions` | — |
| correctly extracts gem names when versions contain special characters | 114 | ported | `bundler.rs` | `bundler_locked_version_gem_names_with_special_chars` | — |
| handles gems with platform-specific versions | 130 | ported | `bundler.rs` | `bundler_locked_version_platform_specific_versions` | — |

---

