# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/decrypt.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt.spec.ts
**Total tests:** 15 | **Ported:** 11 | **Actionable:** 15 | **Status:** done

### `config/decrypt › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty with no privateKey | 23 | not-applicable | — | — | Requires logger spy (logger.logger.once.warn) |
| warns if no privateKey found | 29 | not-applicable | — | — | Requires logger spy (logger.logger.once.warn) |
| throws exception if encrypted found but no privateKey | 41 | not-applicable | — | — | Requires logger spy + process.env mutation |
| throws exception if encrypted found but no privateKey- Mend Hosted | 51 | not-applicable | — | — | Requires logger spy + process.env mutation |

### `config/decrypt › validateDecryptedValue() › platforms non azure`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 68 | ported | `config/decrypt.rs` | `validate_decrypted_value_platforms_non_azure` | — |

### `config/decrypt › validateDecryptedValue() › azure only platform › general tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 93 | ported | `config/decrypt.rs` | `validate_decrypted_value_azure_dev` | — |

### `config/decrypt › validateDecryptedValue() › azure only platform › tests self hosted - ignore "tfs/" before collection name`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 129 | ported | `config/decrypt.rs` | `validate_decrypted_value_azure_tfs` | — |

### `config/decrypt › validateDecryptedValue() › azure only platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| endpoint URL invalid | 164 | ported | `config/decrypt.rs` | `validate_decrypted_value_azure_invalid_endpoint` | — |
| endpoint URL without collection | 196 | ported | `config/decrypt.rs` | `validate_decrypted_value_azure_no_collection` | — |

### `config/decrypt › getAzureCollection()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no pathname and url ends with slash | 235 | ported | `config/decrypt.rs` | `get_azure_collection_no_pathname_slash` | — |
| no pathname and no slash at end of URL | 243 | ported | `config/decrypt.rs` | `get_azure_collection_no_pathname_no_slash` | — |
| pathname no slash at end | 251 | ported | `config/decrypt.rs` | `get_azure_collection_pathname_no_slash` | — |
| pathname with slash at end | 259 | ported | `config/decrypt.rs` | `get_azure_collection_pathname_with_slash` | — |
| pathname 2 levels no slash at end | 267 | ported | `config/decrypt.rs` | `get_azure_collection_pathname_2_levels_no_slash` | — |
| pathname 2 levels with slash at end | 275 | ported | `config/decrypt.rs` | `get_azure_collection_pathname_2_levels_with_slash` | — |

---

