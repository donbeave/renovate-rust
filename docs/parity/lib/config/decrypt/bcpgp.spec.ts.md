# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/decrypt/bcpgp.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt/bcpgp.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `config/decrypt/bcpgp › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid key | 40 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| works broken PGP message | 54 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| fails with ECC and AEAD (wasm-dotnet | 72 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| works with ECC and AEAD (wasm-java) | 92 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| rejects invalid PGP message | 108 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| handles PGP org constraint | 149 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| handles PGP multi-org constraint | 163 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| handles PGP org/repo constraint | 180 | not-applicable | — | — | TS-specific PGP library (bcpgp) |
| handles PGP multi-org/repo constraint | 194 | not-applicable | — | — | TS-specific PGP library (bcpgp) |

---

