# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/extract/consistent-versions-plugin.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract/consistent-versions-plugin.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works for sub folders | 10 | ported | `extractors/gradle.rs` | `gcv_uses_gcv_sub_folders` | — |
| detects lock file header introduced with gradle-consistent-versions version 2.20.0 | 24 | ported | `extractors/gradle.rs` | `gcv_uses_gcv_header_2_20` | — |
| detects lock file header introduced with gradle-consistent-versions version 2.23.0 | 36 | ported | `extractors/gradle.rs` | `gcv_uses_gcv_header_2_23` | — |
| correct position for CRLF and LF | 48 | ported | `extractors/gradle.rs` | `gcv_parse_props_file_positions` | — |
| test bogus input lines | 60 | ported | `extractors/gradle.rs` | `gcv_parse_bogus_input` | — |
| supports multiple levels of glob | 97 | ported | `extractors/gradle.rs` | `gcv_supports_multiple_glob_levels` | — |

---
