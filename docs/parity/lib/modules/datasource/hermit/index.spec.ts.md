# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hermit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hermit/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/hermit/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result from hermit list | 14 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should fail on no result found | 79 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should fail on network error | 106 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null result on non github url given | 133 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null result on missing repo or owner | 142 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null for extra path provided in registry url | 157 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null result on empty registryUrl | 166 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should fail on missing index.json asset | 174 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null on invalid index.json asset | 195 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |
| should get null on invalid registry url | 221 | not-applicable | — | — | Renovate's Hermit `getReleases` release-list and null/error translation behavior are not implemented in Rust; Rust returns a latest/update summary from the index asset. |

---

