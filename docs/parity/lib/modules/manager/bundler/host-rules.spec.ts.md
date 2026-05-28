# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/host-rules.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `getAuthenticationHeaderValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the authentication header with the password | 15 | ported | `bundler.rs` | `bundler_auth_header_with_password` | — |
| returns the authentication header with the token | 24 | ported | `bundler.rs` | `bundler_auth_header_with_token` | — |
| escapes special characters in the username but not the password | 32 | ported | `bundler.rs` | `bundler_auth_header_encodes_username_at_sign` | — |

### `findAllAuthenticatable()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array if matchHost is missing | 55 | ported | `bundler.rs` | `find_all_authenticatable_empty_if_no_match_host` | — |
| returns an empty array if username is missing and password is present | 63 | ported | `bundler.rs` | `find_all_authenticatable_empty_if_no_username` | — |
| returns an empty array if password and token are missing | 73 | ported | `bundler.rs` | `find_all_authenticatable_empty_if_no_credentials` | — |
| returns the hostRule if using matchHost and password | 83 | ported | `bundler.rs` | `find_all_authenticatable_returns_rule_with_match_host_and_password` | — |
| returns the hostRule if using matchHost and token | 92 | ported | `bundler.rs` | `find_all_authenticatable_returns_rule_with_match_host_and_token` | — |
| returns the hostRule if using baseUrl and password | 101 | ported | `bundler.rs` | `find_all_authenticatable_returns_rule_with_base_url_and_password` | — |
| returns the hostRule if using baseUrl and token | 110 | ported | `bundler.rs` | `find_all_authenticatable_returns_rule_with_base_url_and_token` | — |

---

