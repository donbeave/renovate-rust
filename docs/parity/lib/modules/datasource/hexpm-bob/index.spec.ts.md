# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hexpm-bob/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hexpm-bob/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** done

### `modules/datasource/hexpm-bob/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 9 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `throws_for_network_error` | 5xx propagates as Err |
| returns null for 404 | 22 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `returns_null_for_404` | 404 → Ok(None) |
| returns null for empty result | 35 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `returns_null_for_empty_result` | Empty body → Ok(None) |
| returns empty list for empty 200 OK | 48 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `returns_null_for_empty_200_ok` | Empty body → Ok(None) |
| throws for 5xx | 61 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `throws_for_5xx` | 502 propagates as Err |
| processes real data | 74 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `processes_real_data_elixir` | Elixir builds.txt: version cleaning, stability flag, timestamps |
| processes real data (erlang / ubuntu 20.04) | 122 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `processes_real_data_erlang_ubuntu` | OTP builds.txt: OTP-prefixed stable, branch builds unstable |
| can override registry url | 155 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `can_override_registry_url` | Custom registryUrl used for HTTP request |
| returns empty list for invalid package name | 172 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs` | `returns_null_for_invalid_package_name` | Unknown package → Ok(None) without HTTP |

---
