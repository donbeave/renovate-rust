# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/decrypt.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/decrypt › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty with no privateKey | 23 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| warns if no privateKey found | 29 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| throws exception if encrypted found but no privateKey | 41 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| throws exception if encrypted found but no privateKey- Mend Hosted | 51 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |

### `config/decrypt › validateDecryptedValue() › platforms non azure`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 68 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform › general tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 93 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform › tests self hosted - ignore "tfs/" before collection name`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 129 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| endpoint URL invalid | 164 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| endpoint URL without collection | 196 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › getAzureCollection()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no pathname and url ends with slash | 235 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| no pathname and no slash at end of URL | 243 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname no slash at end | 251 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname with slash at end | 259 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname 2 levels no slash at end | 267 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname 2 levels with slash at end | 275 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

---

