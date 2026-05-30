# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/homeassistant-manifest/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homeassistant-manifest/extract.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid JSON | 9 | ported | `homeassistant.rs` | `invalid_json_returns_empty` | — |
| returns null for non-Home Assistant manifest (missing domain) | 14 | ported | `homeassistant.rs` | `missing_domain_returns_empty` | — |
| returns null for non-Home Assistant manifest (missing name) | 24 | ported | `homeassistant.rs` | `missing_name_returns_empty` | — |
| returns null for chrome extension manifest | 34 | ported | `homeassistant.rs` | `chrome_extension_manifest_returns_empty` | — |
| returns null for empty requirements | 45 | ported | `homeassistant.rs` | `empty_requirements_returns_empty` | — |
| returns null when no requirements field | 55 | ported | `homeassistant.rs` | `no_requirements_field_returns_empty` | — |
| extracts single requirement with exact version | 64 | ported | `homeassistant.rs` | `extracts_single_requirement_exact_version` | — |
| extracts multiple requirements | 84 | ported | `homeassistant.rs` | `extracts_multiple_requirements` (+ extracts_requirements) | — |
| handles requirements with extras | 118 | ported | `homeassistant.rs` | `handles_requirements_with_extras` | — |
| extracts git+https requirements | 138 | ported | `homeassistant.rs` | `extracts_git_https_requirements` | — |
| supports requirements with other operators | 168 | ported | `homeassistant.rs` | `extracts_range_version` | — |
| handles requirements without version | 211 | ported | `homeassistant.rs` | `handles_requirements_without_version` | — |
| extracts from real-world ASUSWRT manifest | 237 | ported | `homeassistant.rs` | `extracts_asuswrt_manifest` | — |
| handles invalid requirement types in array | 272 | ported | `homeassistant.rs` | `skips_non_string_entries_in_requirements_array` | — |
| returns null when requirements is not an array | 299 | ported | `homeassistant.rs` | `requirements_not_an_array_returns_empty` | — |
| handles unparseable requirement strings with skipReason | 313 | ported | `homeassistant.rs` | `unparseable_requirement_has_skip_reason` | — |

---

