# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/conda/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conda/index.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/conda/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 14 | not-applicable | — | — | Renovate's Conda `getReleases` release-list, registry fallback, prefix.dev pagination, and external-host-error contract are not implemented in Rust; Rust only exposes an Anaconda latest-version summary. |
| returns null for 404 | 24 | not-applicable | — | — | Renovate's Conda `getReleases` release-list, registry fallback, prefix.dev pagination, and null-on-404 contract are not implemented in Rust; Rust only exposes an Anaconda latest-version summary. |
| returns null for empty result | 34 | not-applicable | — | — | Renovate's Conda `getReleases` release-list, registry fallback, prefix.dev pagination, and empty-result contract are not implemented in Rust; Rust only exposes an Anaconda latest-version summary. |
| throws for 5xx | 47 | not-applicable | — | — | Renovate's Conda `getReleases` release-list, registry fallback, prefix.dev pagination, and external-host-error contract are not implemented in Rust; Rust only exposes an Anaconda latest-version summary. |
| processes real data | 57 | not-applicable | — | — | Renovate's Conda `getReleases` release-list response mapping is not implemented in Rust; Rust only exposes an Anaconda latest-version summary. |
| returns null without registryUrl | 70 | not-applicable | — | — | Renovate's Conda configurable registry URL handling is not implemented in Rust; Rust uses a fixed Anaconda registry. |
| supports multiple custom datasource urls | 79 | not-applicable | — | — | Renovate's Conda configurable registry fallback handling is not implemented in Rust; Rust uses a fixed Anaconda registry. |
| supports channel from prefix.dev with null response | 118 | not-applicable | — | — | Renovate's prefix.dev channel endpoint support is not implemented in Rust. |
| supports channel from prefix.dev with multiple page responses | 135 | not-applicable | — | — | Renovate's prefix.dev channel pagination support is not implemented in Rust. |

---

