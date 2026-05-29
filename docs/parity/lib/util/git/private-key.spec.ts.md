# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/private-key.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/private-key.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `util/git/private-key › writePrivateKey()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no private key | 39 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| throws error if failing | 45 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| imports the private GPG key | 59 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| does not import the key again | 89 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| throws error if SSH key passphrase decryption fails | 94 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| imports SSH key with passphrase successfully | 118 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| warns about GPG key passphrase being ignored | 157 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| accepts SSH key constructor with passphrase | 165 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| imports the private SSH key without passphrase | 177 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| handles SSH key with process.exit spy | 224 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|

### `util/git/private-key › base64 key encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decodes base64-encoded GPG key | 259 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| decodes base64-encoded SSH key (treated as GPG due to format detection) | 293 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| handles non-base64 encoded key unchanged | 332 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| handles invalid base64 that does not round-trip | 357 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| decodes base64-encoded SSH key with passphrase (treated as GPG) | 382 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| properly handles actual SSH key format with base64 content | 418 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| sanitizes both base64 and decoded keys for secret protection | 454 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|
| sanitizes passphrase for base64 keys | 471 | not-applicable | — | — | mocking framework internals — vi.mock on exec/child_process; TypeScript git private key SSH wrapper|

---

