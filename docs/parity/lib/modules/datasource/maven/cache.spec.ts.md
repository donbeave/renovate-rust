# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/cache.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/maven/cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| persists trimmed metadata and pom bodies | 41 | not-applicable | — | — | Renovate's Maven persistent HTTP cache trimming, ETag revalidation, and POM body cache layer are not implemented in Rust; Rust has only in-memory latest-version batch summaries. |
| serves cached trimmed XML without refetching | 87 | not-applicable | — | — | Renovate's Maven persistent HTTP cache trimming, ETag revalidation, and POM body cache layer are not implemented in Rust; Rust has only in-memory latest-version batch summaries. |
| preserves empty relocation markers on cache hits | 125 | not-applicable | — | — | Renovate's Maven persistent HTTP cache trimming, ETag revalidation, and POM body cache layer are not implemented in Rust; Rust has only in-memory latest-version batch summaries. |
| revalidates trimmed cached XML after 304 responses | 166 | not-applicable | — | — | Renovate's Maven persistent HTTP cache trimming, ETag revalidation, and POM body cache layer are not implemented in Rust; Rust has only in-memory latest-version batch summaries. |
| serves cached trimmed snapshot XML without refetching | 217 | not-applicable | — | — | Renovate's Maven persistent HTTP cache trimming, ETag revalidation, and POM body cache layer are not implemented in Rust; Rust has only in-memory latest-version batch summaries. |

---

