# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hashicorp/convertor.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hashicorp/convertor.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/hashicorp/convertor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| hashicorp2npm("$hashicorp") === $npm && npm2hashicorp("$npm") === $hashicorp | 4 | not-applicable | — | — | Renovate's HashiCorp-to-npm range convertor helpers are not implemented as a Rust API. |
| hashicorp2npm("$version") === $version && npm2hashicorp("$version") === $version | 32 | not-applicable | — | — | Renovate's HashiCorp-to-npm range convertor helpers are not implemented as a Rust API. |
| hashicorp2npm("$hashicorp") === $npm | 57 | not-applicable | — | — | Renovate's HashiCorp-to-npm range convertor helper is not implemented as a Rust API. |
| npm2hashicorp("$npm") === $hashicorp | 71 | not-applicable | — | — | Renovate's npm-to-HashiCorp range convertor helper is not implemented as a Rust API. |
| hashicorp2npm doesnt support != | 85 | not-applicable | — | — | Renovate's HashiCorp-to-npm range convertor error contract is not implemented as a Rust API. |
| hashicorp2npm throws on invalid | 89 | not-applicable | — | — | Renovate's HashiCorp-to-npm range convertor error contract is not implemented as a Rust API. |
| npm2hashicorp throws on unsupported | 93 | not-applicable | — | — | Renovate's npm-to-HashiCorp range convertor error contract is not implemented as a Rust API. |

---

