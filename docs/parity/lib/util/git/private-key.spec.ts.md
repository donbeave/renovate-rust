# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/private-key.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/private-key.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/private-key › writePrivateKey()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no private key | 39 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| throws error if failing | 45 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| imports the private GPG key | 59 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| does not import the key again | 89 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| throws error if SSH key passphrase decryption fails | 94 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| imports SSH key with passphrase successfully | 118 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| warns about GPG key passphrase being ignored | 157 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| accepts SSH key constructor with passphrase | 165 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| imports the private SSH key without passphrase | 177 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| handles SSH key with process.exit spy | 224 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |

### `util/git/private-key › base64 key encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decodes base64-encoded GPG key | 259 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| decodes base64-encoded SSH key (treated as GPG due to format detection) | 293 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| handles non-base64 encoded key unchanged | 332 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| handles invalid base64 that does not round-trip | 357 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| decodes base64-encoded SSH key with passphrase (treated as GPG) | 382 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| properly handles actual SSH key format with base64 content | 418 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| sanitizes both base64 and decoded keys for secret protection | 454 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |
| sanitizes passphrase for base64 keys | 471 | not-applicable | — | — | tests GPG key signing via exec/fs infrastructure; Rust would use gpg subprocess differently |

---

