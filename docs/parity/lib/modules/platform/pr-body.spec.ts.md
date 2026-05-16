# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/pr-body.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/pr-body.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/platform/pr-body › getPrBodyStruct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns hash for empty inputs | 6 | not-applicable | — | — | Renovate's TypeScript PR body metadata parser is not implemented as a Rust API; Rust currently has no PR body update workflow. |
| checks if we reach warning | 29 | not-applicable | — | — | Renovate's TypeScript PR body debug-data parsing and warning side effect are not implemented as a Rust API. |
| hashes ignoring debug info | 39 | not-applicable | — | — | Renovate's TypeScript PR body hashing helper is not implemented as a Rust API. |
| hashes ignoring reviewable section | 45 | not-applicable | — | — | Renovate's TypeScript PR body hashing helper is not implemented as a Rust API. |
| hashes an undefined body | 51 | not-applicable | — | — | Renovate's TypeScript PR body hashing helper is not implemented as a Rust API. |
| returns rebaseRequested=true flag | 58 | not-applicable | — | — | Renovate's TypeScript PR rebase checkbox parser is not implemented as a Rust API. |
| returns rebaseRequested=false flag | 67 | not-applicable | — | — | Renovate's TypeScript PR rebase checkbox parser is not implemented as a Rust API. |
| returns rebaseRequested=undefined flag | 76 | not-applicable | — | — | Renovate's TypeScript PR rebase checkbox parser is not implemented as a Rust API. |
| returns raw config hash | 84 | not-applicable | — | — | Renovate's TypeScript PR config-hash marker parser is not implemented as a Rust API. |
| strips reviewable section | 95 | not-applicable | — | — | Renovate's TypeScript PR body reviewable-section stripping helper is not implemented as a Rust API. |

---

