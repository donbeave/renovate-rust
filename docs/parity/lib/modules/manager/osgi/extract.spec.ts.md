# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/osgi/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/osgi/extract.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty file | 143 | ported | `osgi.rs` | `empty_returns_empty` | — |
| returns null for invalid file | 147 | ported | `osgi.rs` | `invalid_json_returns_empty` | — |
| returns null for unsupported version of feature model definition | 151 | ported | `osgi.rs` | `unsupported_version_skipped` | — |
| returns null for an invalid version of feature model definition | 157 | ported | `osgi.rs` | `invalid_feature_version_returns_empty` | — |
| returns null for a null string passed in as a feature model definition | 163 | ported | `osgi.rs` | `null_string_returns_empty` | — |
| returns null for a valid file with no artifact definitions | 167 | ported | `osgi.rs` | `no_bundles_returns_empty` | — |
| extracts the bundles from a file with object bundles definitions | 171 | ported | `osgi.rs` | `extracts_object_bundle` | — |
| extracts the bundles from a file with string bundles defintions | 193 | ported | `osgi.rs` | `extracts_string_bundle` (+ slash_separator_normalized) | — |
| extracts the bundles from a file with comments | 215 | ported | `osgi.rs` | `json_with_comments` | — |
| extracts the artifacts from an extension section | 228 | ported | `osgi.rs` | `extracts_from_extension_section` | — |
| extracts the artifacts a file with a double slash | 241 | ported | `osgi.rs` | `double_slash_in_value_not_treated_as_comment` | — |
| extracts the artifacts from the framework artifact section | 263 | ported | `osgi.rs` | `extracts_from_framework_artifact_section` | — |
| skips depedencies with with malformed definitions | 276 | ported | `osgi.rs` | `malformed_definitions_skipped_with_valid_kept` | — |
| skips artifacts with variables in version | 297 | ported | `osgi.rs` | `variable_version_skipped` | — |

---

