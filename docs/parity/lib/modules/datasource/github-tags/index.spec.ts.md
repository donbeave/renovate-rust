# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-tags/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/github-tags/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commit digest | 25 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns null for missing commit | 36 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns untagged commit digest | 45 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns tagged commit digest | 54 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns null for missing hash | 73 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns null for missing tagged commit digest | 91 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| returns null for error | 110 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |

### `modules/datasource/github-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags | 120 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |

| if it is newer than tag timestamp | 183 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| keeps tag timestamp when release timestamp is older | 212 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| keeps tag timestamp when release timestamp is equal | 241 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
| keeps tag timestamp when no corresponding release exists | 270 | not-applicable | — | — | Renovate's GitHub tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup for update summaries. |
---

