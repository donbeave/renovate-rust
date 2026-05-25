# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-tags/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** pending

### `modules/datasource/github-tags/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commit digest | 25 | pending | — | — | — |
| returns null for missing commit | 36 | pending | — | — | — |
| returns untagged commit digest | 45 | pending | — | — | — |
| returns tagged commit digest | 54 | pending | — | — | — |
| returns null for missing hash | 73 | pending | — | — | — |
| returns null for missing tagged commit digest | 91 | pending | — | — | — |
| returns null for error | 110 | pending | — | — | — |

### `modules/datasource/github-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags | 120 | pending | — | — | — |

| if it is newer than tag timestamp | 183 | pending | — | — | — |
| keeps tag timestamp when release timestamp is older | 212 | pending | — | — | — |
| keeps tag timestamp when release timestamp is equal | 241 | pending | — | — | — |
| keeps tag timestamp when no corresponding release exists | 270 | pending | — | — | — |
---

