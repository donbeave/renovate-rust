# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/decrypt.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** pending

### `config/decrypt › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty with no privateKey | 23 | pending | — | — | — |
| warns if no privateKey found | 29 | pending | — | — | — |
| throws exception if encrypted found but no privateKey | 41 | pending | — | — | — |
| throws exception if encrypted found but no privateKey- Mend Hosted | 51 | pending | — | — | — |

### `config/decrypt › validateDecryptedValue() › platforms non azure`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 68 | pending | — | — | — |

### `config/decrypt › validateDecryptedValue() › azure only platform › general tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 93 | pending | — | — | — |

### `config/decrypt › validateDecryptedValue() › azure only platform › tests self hosted - ignore "tfs/" before collection name`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 129 | pending | — | — | — |

### `config/decrypt › validateDecryptedValue() › azure only platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| endpoint URL invalid | 164 | pending | — | — | — |
| endpoint URL without collection | 196 | pending | — | — | — |

### `config/decrypt › getAzureCollection()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no pathname and url ends with slash | 235 | pending | — | — | — |
| no pathname and no slash at end of URL | 243 | pending | — | — | — |
| pathname no slash at end | 251 | pending | — | — | — |
| pathname with slash at end | 259 | pending | — | — | — |
| pathname 2 levels no slash at end | 267 | pending | — | — | — |
| pathname 2 levels with slash at end | 275 | pending | — | — | — |

---

