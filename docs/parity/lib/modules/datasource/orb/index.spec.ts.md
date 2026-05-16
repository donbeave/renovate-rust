# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/orb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/orb/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/orb/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 32 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| returns null for missing orb | 42 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| returns null for 404 | 55 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| returns null for unknown error | 65 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| processes real data | 75 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| processes homeUrl | 85 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |
| supports other registries | 96 | not-applicable | — | — | Renovate's Orb `getReleases` release-list, homepage, registry URL, and null-on-error contract are not implemented in Rust; Rust only exposes latest-version update summaries. |

---

