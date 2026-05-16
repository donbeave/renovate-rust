# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/cdnjs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cdnjs/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/cdnjs/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for empty result | 18 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list, empty-response, and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| throws for error | 28 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list, empty-response, and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| returns null for 404 | 38 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and null-on-404 contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| returns null for empty 200 OK | 48 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and empty-body contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| throws for 401 | 61 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| throws for 429 | 71 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| throws for 5xx | 81 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| throws for unknown error | 91 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list and external-host-error contract are not implemented in Rust; Rust only exposes a latest-version lookup. |
| processes real data | 101 | not-applicable | — | — | Renovate's CDNJS `getReleases` release-list snapshot mapping is not implemented in Rust; Rust only exposes a latest-version lookup. |
| returs null for no result | 115 | not-applicable | — | — | Renovate's CDNJS `getDigest` file/SRI lookup and null contract are not implemented in Rust. |
| returs null for empty sri object | 131 | not-applicable | — | — | Renovate's CDNJS `getDigest` file/SRI lookup and null contract are not implemented in Rust. |
| returs null if file not found | 147 | not-applicable | — | — | Renovate's CDNJS `getDigest` file/SRI lookup and null contract are not implemented in Rust. |
| returns null for 404 | 163 | not-applicable | — | — | Renovate's CDNJS `getDigest` file/SRI lookup and null contract are not implemented in Rust. |
| returns digest | 176 | not-applicable | — | — | Renovate's CDNJS `getDigest` file/SRI lookup and digest extraction are not implemented in Rust. |

---

