# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/github/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/url.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/github/url › getSourceUrlBase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensures trailing slash | 5 | ported | `platform/github.rs` | `github_get_source_url_base_trailing_slash` | — |
| defaults to github.com | 10 | ported | `platform/github.rs` | `github_get_source_url_base_default` | — |

### `util/github/url › getApiBaseUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps to api.github.com | 17 | ported | `platform/github.rs` | `github_get_api_base_url_maps_to_api` | — |
| supports local github installations | 22 | ported | `platform/github.rs` | `github_get_api_base_url_local_install` | — |

---

