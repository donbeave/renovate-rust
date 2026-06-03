# `lib/util/git/private-key.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**0/18 ported** (18 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 39 | returns if no private key | pending | — |
| 45 | throws error if failing | pending | — |
| 59 | imports the private gpg key | pending | — |
| 89 | does not import the key again | pending | — |
| 94 | throws error if ssh key passphrase decryption fails | pending | — |
| 118 | imports ssh key with passphrase successfully | pending | — |
| 157 | warns about gpg key passphrase being ignored | pending | — |
| 165 | accepts ssh key constructor with passphrase | pending | — |
| 177 | imports the private ssh key without passphrase | pending | — |
| 224 | handles ssh key with process.exit spy | pending | — |
| 259 | decodes base64-encoded gpg key | pending | — |
| 293 | decodes base64-encoded ssh key (treated as gpg due to format detection) | pending | — |
| 332 | handles non-base64 encoded key unchanged | pending | — |
| 357 | handles invalid base64 that does not round-trip | pending | — |
| 382 | decodes base64-encoded ssh key with passphrase (treated as gpg) | pending | — |
| 418 | properly handles actual ssh key format with base64 content | pending | — |
| 454 | sanitizes both base64 and decoded keys for secret protection | pending | — |
| 471 | sanitizes passphrase for base64 keys | pending | — |

