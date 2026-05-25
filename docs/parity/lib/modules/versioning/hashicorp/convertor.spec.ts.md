# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hashicorp/convertor.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hashicorp/convertor.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/versioning/hashicorp/convertor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| hashicorp2npm("$hashicorp") === $npm && npm2hashicorp("$npm") === $hashicorp | 4 | ported | crates/renovate-core/src/versioning/hashicorp.rs | hashicorp2npm_and_npm2hashicorp_roundtrip_matches_renovate_hashicorp_convertor_spec | — |
| hashicorp2npm("$version") === $version && npm2hashicorp("$version") === $version | 32 | ported | crates/renovate-core/src/versioning/hashicorp.rs | hashicorp2npm_and_npm2hashicorp_identity_matches_renovate_hashicorp_convertor_spec | — |
| hashicorp2npm("$hashicorp") === $npm | 57 | ported | crates/renovate-core/src/versioning/hashicorp.rs | hashicorp2npm_nonreflective_matches_renovate_hashicorp_convertor_spec | — |
| npm2hashicorp("$npm") === $hashicorp | 71 | ported | crates/renovate-core/src/versioning/hashicorp.rs | npm2hashicorp_nonreflective_matches_renovate_hashicorp_convertor_spec | — |
| hashicorp2npm doesnt support != | 85 | ported | crates/renovate-core/src/versioning/hashicorp.rs | hashicorp2npm_doesnt_support_neq_matches_renovate_hashicorp_convertor_spec | — |
| hashicorp2npm throws on invalid | 89 | ported | crates/renovate-core/src/versioning/hashicorp.rs | hashicorp2npm_throws_on_invalid_matches_renovate_hashicorp_convertor_spec | — |
| npm2hashicorp throws on unsupported | 93 | ported | crates/renovate-core/src/versioning/hashicorp.rs | npm2hashicorp_throws_on_unsupported_matches_renovate_hashicorp_convertor_spec | — |

---

