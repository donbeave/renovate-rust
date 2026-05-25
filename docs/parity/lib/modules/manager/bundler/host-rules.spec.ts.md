# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/host-rules.spec.ts
**Total tests:** 10 | **Ported:** 3 | **Actionable:** 10 | **Status:** partial

### `getAuthenticationHeaderValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the authentication header with the password | 15 | ported | `bundler.rs` | `bundler_auth_header_with_password` | — |
| returns the authentication header with the token | 24 | ported | `bundler.rs` | `bundler_auth_header_with_token` | — |
| escapes special characters in the username but not the password | 32 | ported | `bundler.rs` | `bundler_auth_header_encodes_username_at_sign` | — |

### `findAllAuthenticatable()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array if matchHost is missing | 55 | pending | — | — | — |
| returns an empty array if username is missing and password is present | 63 | pending | — | — | — |
| returns an empty array if password and token are missing | 73 | pending | — | — | — |
| returns the hostRule if using matchHost and password | 83 | pending | — | — | — |
| returns the hostRule if using matchHost and token | 92 | pending | — | — | — |
| returns the hostRule if using baseUrl and password | 101 | pending | — | — | — |
| returns the hostRule if using baseUrl and token | 110 | pending | — | — | — |

---

