# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/private-key.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/private-key.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `util/git/private-key › writePrivateKey()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no private key | 39 | pending | — | — | — |
| throws error if failing | 45 | pending | — | — | — |
| imports the private GPG key | 59 | pending | — | — | — |
| does not import the key again | 89 | pending | — | — | — |
| throws error if SSH key passphrase decryption fails | 94 | pending | — | — | — |
| imports SSH key with passphrase successfully | 118 | pending | — | — | — |
| warns about GPG key passphrase being ignored | 157 | pending | — | — | — |
| accepts SSH key constructor with passphrase | 165 | pending | — | — | — |
| imports the private SSH key without passphrase | 177 | pending | — | — | — |
| handles SSH key with process.exit spy | 224 | pending | — | — | — |

### `util/git/private-key › base64 key encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decodes base64-encoded GPG key | 259 | pending | — | — | — |
| decodes base64-encoded SSH key (treated as GPG due to format detection) | 293 | pending | — | — | — |
| handles non-base64 encoded key unchanged | 332 | pending | — | — | — |
| handles invalid base64 that does not round-trip | 357 | pending | — | — | — |
| decodes base64-encoded SSH key with passphrase (treated as GPG) | 382 | pending | — | — | — |
| properly handles actual SSH key format with base64 content | 418 | pending | — | — | — |
| sanitizes both base64 and decoded keys for secret protection | 454 | pending | — | — | — |
| sanitizes passphrase for base64 keys | 471 | pending | — | — | — |

---

