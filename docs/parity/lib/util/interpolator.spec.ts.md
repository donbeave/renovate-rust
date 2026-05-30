# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/interpolator.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/interpolator.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `util/interpolator › validateInterpolatedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if not input | 13 | ported | `util.rs` | `test_validate_interpolated_none` | — |
| does not throw error when keys and values are valid | 19 | ported | `util.rs` | `test_validate_interpolated_valid` | — |
| throws when input is not a valid object | 25 | ported | `util.rs` | `test_validate_interpolated_not_object` | — |
| throws when keys do not follow specified regex patterns | 31 | ported | `util.rs` | `test_validate_interpolated_bad_key` | — |
| throws when values are not of type string | 40 | ported | `util.rs` | `test_validate_interpolated_non_string_value` | — |

### `util/interpolator › replaceInterpolatedValuesInObject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces values and deletes secrets | 48 | ported | `config/secrets.rs` | `replaces_values_and_deletes_secrets` | — |
| replaces values and keeps secrets | 97 | ported | `config/secrets.rs` | `replaces_values_and_keeps_secrets` | — |
| does not resolve secrets in onboaringConfig | 115 | ported | `config/secrets.rs` | `does_not_resolve_secrets_in_onboarding_config` | — |
| throws error if secrets are used in disallowed options | 155 | ported | `config/secrets.rs` | `errors_if_secrets_in_disallowed_options` | — |
| throws error if secret key is not present in config | 175 | ported | `config/secrets.rs` | `errors_if_secret_key_is_not_present_in_config` | — |

---

