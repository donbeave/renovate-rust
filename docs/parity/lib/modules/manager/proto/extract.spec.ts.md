# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/proto/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/proto/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty content | 10 | ported | `proto.rs` | `returns_null_for_empty_content` | — |
| returns null for invalid TOML | 14 | ported | `proto.rs` | `returns_null_for_invalid_toml` | — |
| returns null when only config sections exist | 18 | ported | `proto.rs` | `returns_null_when_only_config_sections` | — |
| extracts a single tool version | 29 | ported | `proto.rs` | `extracts_single_tool_version` | — |
| extracts multiple tool versions | 46 | ported | `proto.rs` | `extracts_multiple_tool_versions` | — |
| skips non-version sections | 76 | ported | `proto.rs` | `skips_non_version_sections` | — |
| handles proto self-versioning | 105 | ported | `proto.rs` | `handles_proto_self_versioning` | — |
| handles moon tool | 122 | ported | `proto.rs` | `handles_moon_tool` | — |
| handles uv tool | 139 | ported | `proto.rs` | `handles_uv_tool` | — |
| marks unknown tools as unsupported-datasource | 156 | ported | `proto.rs` | `marks_unknown_tools_as_unsupported_datasource` | — |
| skips alias values like latest | 172 | ported | `proto.rs` | `skips_alias_values_like_latest` | — |
| skips alias value stable | 188 | ported | `proto.rs` | `skips_alias_value_stable` | — |
| handles partial versions | 204 | ported | `proto.rs` | `handles_partial_versions` | — |
| extracts all supported tools from fixture | 221 | ported | `proto.rs` | `extracts_all_supported_tools_from_fixture` | — |
| extracts all supported built-in tools | 278 | ported | `proto.rs` | `extracts_all_supported_builtin_tools` | — |

---

