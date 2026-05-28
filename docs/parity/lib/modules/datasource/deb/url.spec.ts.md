# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deb/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/url.spec.ts
**Total tests:** 6 | **Ported:** 3 | **Actionable:** 6 | **Status:** partial

### `modules/datasource/deb/url`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| constructs URLs correctly from registry URL with suite | 11 | ported | `datasources/deb.rs` | `construct_component_urls_with_suite` | — |
| constructs URLs correctly from registry URL with deprecated release | 22 | ported | `datasources/deb.rs` | `construct_component_urls_with_release` | — |
| throws an error if required parameters are missing | 33 | ported | `datasources/deb.rs` | `construct_component_urls_missing_params` | — |
| should return true for different status code | 45 | pending | — | — | — |
| returns empty array for invalid registry URL | 41 | pending | — | — | — |
| should return true if request failed | 60 | pending | — | — | — |

---

