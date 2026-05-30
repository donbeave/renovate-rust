# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/config-formatter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/config-formatter.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `createNuGetConfigXml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns xml with registries | 12 | ported | `extractors/nuget.rs` | `nuget_config_xml_basic_registries` | — |
| returns xml with authenticated registries | 58 | ported | `extractors/nuget.rs` | `nuget_config_xml_with_credentials` | — |
| escapes registry credential names containing special characters | 138 | ported | `extractors/nuget.rs` | `nuget_config_xml_escapes_special_chars_in_names` | — |
| strips protocol version from feed url | 181 | ported | `extractors/nuget.rs` | `nuget_config_xml_strips_protocol_version_from_hash` | — |
| includes packageSourceMapping when defined | 202 | ported | `extractors/nuget.rs` | `nuget_config_xml_package_source_mapping` | — |
| excludes packageSourceMapping when undefined | 245 | ported | `extractors/nuget.rs` | `nuget_config_xml_no_package_source_mapping` | — |
| skips duplicate registry URLs | 265 | ported | `extractors/nuget.rs` | `nuget_config_xml_skips_duplicates` | — |

---
