# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/private-key.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/private-key.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/private-key › writePrivateKey()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no private key | 39 | not-applicable | — | — | `writePrivateKey` and GPG/SSH key import not implemented in Rust |
| throws error if failing | 45 | not-applicable | — | — | `writePrivateKey` and GPG/SSH key import not implemented in Rust |
| imports the private GPG key | 59 | not-applicable | — | — | GPG key import not implemented in Rust |
| does not import the key again | 89 | not-applicable | — | — | GPG key import not implemented in Rust |
| throws error if SSH key passphrase decryption fails | 94 | not-applicable | — | — | SSH key passphrase decryption not implemented in Rust |
| imports SSH key with passphrase successfully | 118 | not-applicable | — | — | SSH key import not implemented in Rust |
| warns about GPG key passphrase being ignored | 157 | not-applicable | — | — | GPG key import not implemented in Rust |
| accepts SSH key constructor with passphrase | 165 | not-applicable | — | — | SSH key import not implemented in Rust |
| imports the private SSH key without passphrase | 177 | not-applicable | — | — | SSH key import not implemented in Rust |
| handles SSH key with process.exit spy | 224 | not-applicable | — | — | SSH key import not implemented in Rust |

### `util/git/private-key › base64 key encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decodes base64-encoded GPG key | 259 | not-applicable | — | — | GPG key base64 decoding not implemented in Rust |
| decodes base64-encoded SSH key (treated as GPG due to format detection) | 293 | not-applicable | — | — | SSH/GPG key format detection not implemented in Rust |
| handles non-base64 encoded key unchanged | 332 | not-applicable | — | — | Key base64 handling not implemented in Rust |
| handles invalid base64 that does not round-trip | 357 | not-applicable | — | — | Key base64 handling not implemented in Rust |
| decodes base64-encoded SSH key with passphrase (treated as GPG) | 382 | not-applicable | — | — | SSH/GPG key format detection not implemented in Rust |
| properly handles actual SSH key format with base64 content | 418 | not-applicable | — | — | SSH key format detection not implemented in Rust |
| sanitizes both base64 and decoded keys for secret protection | 454 | not-applicable | — | — | Key sanitization for secrets not implemented in Rust |
| sanitizes passphrase for base64 keys | 471 | not-applicable | — | — | Key sanitization for secrets not implemented in Rust |

---
