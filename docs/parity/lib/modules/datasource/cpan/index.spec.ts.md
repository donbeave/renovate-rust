# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/cpan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cpan/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/cpan/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 11 | not-applicable | — | — | Renovate's CPAN file-search `getReleases` response mapping is not implemented in Rust; Rust uses a latest-only MetaCPAN module endpoint. |
| returns null for 404 | 27 | not-applicable | — | — | Renovate's CPAN file-search `getReleases` response mapping is not implemented in Rust; Rust uses a latest-only MetaCPAN module endpoint. |
| throws for 5xx | 37 | not-applicable | — | — | Renovate's CPAN file-search `getReleases` response mapping is not implemented in Rust; Rust uses a latest-only MetaCPAN module endpoint. |
| returns null for unknown error | 47 | not-applicable | — | — | Renovate's CPAN file-search `getReleases` response mapping is not implemented in Rust; Rust uses a latest-only MetaCPAN module endpoint. |
| processes real data | 57 | not-applicable | — | — | Renovate's CPAN file-search `getReleases` response mapping is not implemented in Rust; Rust uses a latest-only MetaCPAN module endpoint. |

---

