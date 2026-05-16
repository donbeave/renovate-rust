# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/www-authenticate.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/www-authenticate.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `util/http/www-authenticate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bearer | 4 | ported | `http.rs` | `www_auth_parses_bearer` | it.each; bearer sub-case verified |
| parses empty string | 135 | ported | `http.rs` | `www_auth_parses_empty_string` | — |
| throws on invalid input | 139 | ported | `http.rs` | `www_auth_throws_on_invalid_input` | — |

---

